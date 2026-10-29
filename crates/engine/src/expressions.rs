use crate::{ActionError, ValidationError};
use grimm_semantic::story::{Condition, Effect, Value, Variables};

pub(crate) fn same_type(a: &Value, b: &Value) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn variable<'a>(variables: &'a Variables, name: &str) -> Result<&'a Value, ValidationError> {
    variables
        .get(name)
        .ok_or_else(|| ValidationError(format!("undeclared variable '{name}'")))
}

pub(crate) fn validate_condition(
    condition: &Condition,
    variables: &Variables,
    depth: usize,
    temporal: bool,
) -> Result<(), ValidationError> {
    if depth > 64 {
        return Err(ValidationError(
            "conditions may nest at most 64 levels".into(),
        ));
    }
    match condition {
        Condition::WorldAtLeast { .. } | Condition::WorldAtMost { .. } => {
            if !temporal {
                return Err(ValidationError(
                    "world-time predicates require schema 2 and time configuration".into(),
                ));
            }
        }
        Condition::Equals {
            variable: name,
            value,
        } => {
            if !same_type(variable(variables, name)?, value) {
                return Err(ValidationError(format!(
                    "variable '{name}': equality value has the wrong type"
                )));
            }
        }
        Condition::AtLeast { variable: name, .. } | Condition::AtMost { variable: name, .. } => {
            if !matches!(variable(variables, name)?, Value::Int(_)) {
                return Err(ValidationError(format!(
                    "variable '{name}': comparison requires an integer"
                )));
            }
        }
        Condition::All { conditions } | Condition::Any { conditions } => {
            for child in conditions {
                validate_condition(child, variables, depth + 1, temporal)?;
            }
        }
        Condition::Not { condition } => {
            validate_condition(condition, variables, depth + 1, temporal)?
        }
    }
    Ok(())
}

pub(crate) fn validate_effect(
    effect: &Effect,
    variables: &Variables,
) -> Result<(), ValidationError> {
    let (name, expected) = match effect {
        Effect::Set { variable, value } => (variable, value.clone()),
        Effect::Add { variable, .. } => (variable, Value::Int(0)),
    };
    if !same_type(variable(variables, name)?, &expected) {
        return Err(ValidationError(format!(
            "variable '{name}': effect has the wrong type"
        )));
    }
    Ok(())
}

pub(crate) fn evaluate(condition: &Condition, variables: &Variables, world_minutes: u64) -> bool {
    match condition {
        Condition::WorldAtLeast { minutes } => world_minutes >= *minutes,
        Condition::WorldAtMost { minutes } => world_minutes <= *minutes,
        Condition::Equals { variable, value } => variables.get(variable) == Some(value),
        Condition::AtLeast { variable, value } => {
            matches!(variables.get(variable), Some(Value::Int(current)) if current >= value)
        }
        Condition::AtMost { variable, value } => {
            matches!(variables.get(variable), Some(Value::Int(current)) if current <= value)
        }
        Condition::All { conditions } => conditions
            .iter()
            .all(|child| evaluate(child, variables, world_minutes)),
        Condition::Any { conditions } => conditions
            .iter()
            .any(|child| evaluate(child, variables, world_minutes)),
        Condition::Not { condition } => !evaluate(condition, variables, world_minutes),
    }
}

pub(crate) fn apply_effects(
    effects: &[Effect],
    variables: &mut Variables,
) -> Result<(), ActionError> {
    for effect in effects {
        match effect {
            Effect::Set { variable, value } => {
                variables.insert(variable.clone(), value.clone());
            }
            Effect::Add { variable, amount } => {
                let Some(Value::Int(current)) = variables.get(variable) else {
                    unreachable!("validated integer effect")
                };
                let next =
                    current
                        .checked_add(*amount)
                        .ok_or_else(|| ActionError::EffectOverflow {
                            variable: variable.clone(),
                        })?;
                variables.insert(variable.clone(), Value::Int(next));
            }
        }
    }
    Ok(())
}
