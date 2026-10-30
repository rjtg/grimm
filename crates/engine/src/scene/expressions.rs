use super::{SceneError, SceneResult};
use crate::expressions::same_type;
use grimm_semantic::{
    scene::*,
    story::{Value, Variables},
};

#[derive(Clone, Copy, Default)]
pub(super) struct Times {
    pub selection: u64,
    pub end: u64,
    pub duration: u64,
    pub effective: u64,
    pub deadline: Option<u64>,
    pub interrupted: bool,
}

fn integer(value: Value) -> SceneResult<i64> {
    match value {
        Value::Int(n) => Ok(n),
        _ => Err(SceneError("integer expression required".into())),
    }
}

pub(super) fn eval(
    expr: &Expr,
    globals: &Variables,
    locals: &Variables,
    time: Times,
    depth: usize,
) -> SceneResult<Value> {
    if depth > 64 {
        return Err(SceneError("expression depth exceeds 64".into()));
    }
    Ok(match expr {
        Expr::Literal(value) => value.clone(),
        Expr::Variable { var } => lookup(globals, locals, var)?.clone(),
        Expr::Builtin { clock } => match clock {
            ClockValue::Interrupted => Value::Bool(time.interrupted),
            _ => Value::Int(
                i64::try_from(match clock {
                    ClockValue::SelectionMs => time.selection,
                    ClockValue::TimePassed => time.end,
                    ClockValue::DurationMs => time.duration,
                    ClockValue::EffectiveMs => time.effective,
                    ClockValue::DeadlineMs => time.deadline.unwrap_or(i64::MAX as u64),
                    ClockValue::Interrupted => unreachable!(),
                })
                .map_err(|_| SceneError("time expression exceeds i64".into()))?,
            ),
        },
        Expr::Arithmetic { calc, args } => {
            let a = integer(eval(&args[0], globals, locals, time, depth + 1)?)?;
            let b = integer(eval(&args[1], globals, locals, time, depth + 1)?)?;
            Value::Int(
                match calc {
                    Arithmetic::Add => a.checked_add(b),
                    Arithmetic::Sub => a.checked_sub(b),
                    Arithmetic::Mul => a.checked_mul(b),
                    Arithmetic::Div => a.checked_div(b),
                }
                .ok_or_else(|| SceneError("arithmetic overflow or division by zero".into()))?,
            )
        }
    })
}

pub(super) fn lookup<'a>(
    globals: &'a Variables,
    locals: &'a Variables,
    var: &str,
) -> SceneResult<&'a Value> {
    let (scope, name) = var
        .split_once('.')
        .ok_or_else(|| SceneError(format!("variable '{var}' requires global. or local. scope")))?;
    match scope {
        "global" => globals.get(name),
        "local" => locals.get(name),
        _ => None,
    }
    .ok_or_else(|| SceneError(format!("undeclared variable '{var}'")))
}

pub(super) fn check(
    test: &Test,
    globals: &Variables,
    locals: &Variables,
    time: Times,
    depth: usize,
) -> SceneResult<bool> {
    if depth > 64 {
        return Err(SceneError("condition depth exceeds 64".into()));
    }
    let pair = |left, right| -> SceneResult<(Value, Value)> {
        let a = eval(left, globals, locals, time, depth + 1)?;
        let b = eval(right, globals, locals, time, depth + 1)?;
        if !same_type(&a, &b) {
            return Err(SceneError(
                "comparison operands have different types".into(),
            ));
        }
        Ok((a, b))
    };
    Ok(match test {
        Test::Eq { left, right } => {
            let (a, b) = pair(left, right)?;
            a == b
        }
        Test::Ne { left, right } => {
            let (a, b) = pair(left, right)?;
            a != b
        }
        Test::Lt { left, right }
        | Test::Le { left, right }
        | Test::Gt { left, right }
        | Test::Ge { left, right } => {
            let (a, b) = pair(left, right)?;
            let (a, b) = (integer(a)?, integer(b)?);
            match test {
                Test::Lt { .. } => a < b,
                Test::Le { .. } => a <= b,
                Test::Gt { .. } => a > b,
                _ => a >= b,
            }
        }
        Test::All { tests } => {
            for test in tests {
                if !check(test, globals, locals, time, depth + 1)? {
                    return Ok(false);
                }
            }
            true
        }
        Test::Any { tests } => {
            for test in tests {
                if check(test, globals, locals, time, depth + 1)? {
                    return Ok(true);
                }
            }
            false
        }
        Test::Not { test } => !check(test, globals, locals, time, depth + 1)?,
    })
}

pub(super) fn update(
    updates: &[Update],
    globals: &mut Variables,
    locals: &mut Variables,
    time: Times,
) -> SceneResult<()> {
    for update in updates {
        let (var, expr) = match update {
            Update::Set { var, value } | Update::Add { var, value } => (var, value),
        };
        let before = lookup(globals, locals, var)?;
        let value = eval(expr, globals, locals, time, 0)?;
        if !same_type(before, &value) {
            return Err(SceneError(format!("effect on '{var}' changes its type")));
        }
        let value = match update {
            Update::Set { .. } => value,
            Update::Add { .. } => Value::Int(
                integer(before.clone())?
                    .checked_add(integer(value)?)
                    .ok_or_else(|| SceneError(format!("effect overflows '{var}'")))?,
            ),
        };
        let (scope, name) = var.split_once('.').expect("validated variable");
        if scope == "global" {
            globals.insert(name.into(), value);
        } else {
            locals.insert(name.into(), value);
        }
    }
    Ok(())
}

/// Infer types without evaluating arithmetic (zero defaults must not falsely
/// reject a legal expression whose divisor is positive only when offered).
pub(super) fn expr_type(
    expr: &Expr,
    globals: &Variables,
    locals: &Variables,
    depth: usize,
) -> SceneResult<Value> {
    if depth > 64 {
        return Err(SceneError("expression depth exceeds 64".into()));
    }
    Ok(match expr {
        Expr::Literal(value) => value.clone(),
        Expr::Variable { var } => lookup(globals, locals, var)?.clone(),
        Expr::Builtin {
            clock: ClockValue::Interrupted,
        } => Value::Bool(false),
        Expr::Builtin { .. } => Value::Int(0),
        Expr::Arithmetic { args, .. } => {
            integer(expr_type(&args[0], globals, locals, depth + 1)?)?;
            integer(expr_type(&args[1], globals, locals, depth + 1)?)?;
            Value::Int(0)
        }
    })
}

pub(super) fn validate_test(
    test: &Test,
    globals: &Variables,
    locals: &Variables,
    depth: usize,
) -> SceneResult<()> {
    if depth > 64 {
        return Err(SceneError("condition depth exceeds 64".into()));
    }
    match test {
        Test::All { tests } | Test::Any { tests } => {
            for t in tests {
                validate_test(t, globals, locals, depth + 1)?;
            }
        }
        Test::Not { test } => validate_test(test, globals, locals, depth + 1)?,
        Test::Eq { left, right }
        | Test::Ne { left, right }
        | Test::Lt { left, right }
        | Test::Le { left, right }
        | Test::Gt { left, right }
        | Test::Ge { left, right } => {
            let a = expr_type(left, globals, locals, depth + 1)?;
            let b = expr_type(right, globals, locals, depth + 1)?;
            if !same_type(&a, &b) {
                return Err(SceneError(
                    "comparison operands have different types".into(),
                ));
            }
            if !matches!(test, Test::Eq { .. } | Test::Ne { .. }) {
                integer(a)?;
            }
        }
    }
    Ok(())
}
