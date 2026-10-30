use super::{SceneError, SceneResult, expressions::*};
use crate::expressions::same_type;
use grimm_semantic::{
    scene::*,
    story::{Value, Variables},
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn normalize(name: &str) -> SceneResult<String> {
    if !name.is_ascii() {
        return Err(SceneError(format!("identifier '{name}' must be ASCII")));
    }
    let mut result = String::new();
    for word in name.split('.') {
        let mut part = String::new();
        for ch in word.trim().chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                part.push(ch.to_ascii_lowercase());
            } else if matches!(ch, ' ' | '-') {
                if !part.ends_with('_') {
                    part.push('_');
                }
            } else {
                return Err(SceneError(format!("invalid identifier '{name}'")));
            }
        }
        if part.is_empty()
            || part.starts_with(|ch: char| ch.is_ascii_digit())
            || part.starts_with("__grimm")
            || part.contains("__run_")
        {
            return Err(SceneError(format!(
                "invalid or reserved normalized identifier '{name}'"
            )));
        }
        if !result.is_empty() {
            result.push('.');
        }
        result.push_str(&part);
    }
    Ok(result)
}

fn qualify(namespace: &str, name: &str) -> SceneResult<String> {
    let name = normalize(name)?;
    if name.contains('.') {
        if !name.starts_with(&format!("{namespace}.")) {
            return Err(SceneError(format!(
                "cue/action '{name}' is outside namespace '{namespace}'"
            )));
        }
        Ok(name)
    } else {
        Ok(format!("{namespace}.{name}"))
    }
}

fn declarations(variables: &Variables) -> SceneResult<()> {
    for name in variables.keys() {
        if normalize(name)? != *name || name.contains('.') {
            return Err(SceneError(format!(
                "variable '{name}' must be a normalized local name"
            )));
        }
    }
    Ok(())
}

fn updates(effects: &[Update], globals: &Variables, locals: &Variables) -> SceneResult<()> {
    for effect in effects {
        let (var, expr) = match effect {
            Update::Set { var, value } | Update::Add { var, value } => (var, value),
        };
        let expected = lookup(globals, locals, var)?;
        let actual = expr_type(expr, globals, locals, 0)?;
        if !same_type(expected, &actual)
            || (matches!(effect, Update::Add { .. }) && !matches!(expected, Value::Int(_)))
        {
            return Err(SceneError(format!("effect on '{var}' has the wrong type")));
        }
    }
    Ok(())
}

fn cues(
    cues: &mut [AuthoredCue],
    ns: &str,
    actions: &BTreeSet<String>,
    globals: &Variables,
    locals: &Variables,
    ids: &mut BTreeSet<String>,
) -> SceneResult<()> {
    for cue in cues {
        if cue.at_ms.is_some_and(|time| time > i64::MAX as u64) {
            return Err(SceneError("cue time exceeds i64 milliseconds".into()));
        }
        if let Some(test) = &cue.when {
            validate_test(test, globals, locals, 0)?;
        }
        match &mut cue.instruction {
            AuthoredInstruction::Narrative { narrative_id } => {
                // Content IDs may be shared between scenes; they are not graph references.
                *narrative_id = if narrative_id.contains('.') {
                    normalize(narrative_id)?
                } else {
                    qualify(ns, narrative_id)?
                };
            }
            AuthoredInstruction::Actions { available_actions } => match available_actions {
                ActionSet::Eligible(word) if word == "eligible" => {}
                ActionSet::Eligible(_) => {
                    return Err(SceneError(
                        "available_actions must be eligible or a list of action IDs".into(),
                    ));
                }
                ActionSet::Only(names) => {
                    let mut seen = BTreeSet::new();
                    for name in names {
                        *name = qualify(ns, name)?;
                        if !actions.contains(name) {
                            return Err(SceneError(format!("unknown offered action '{name}'")));
                        }
                        if !seen.insert(name.clone()) {
                            return Err(SceneError(format!("duplicate offered action '{name}'")));
                        }
                    }
                }
            },
            AuthoredInstruction::Timer {
                start_ms,
                extend_ms,
                visible,
            } => {
                if start_ms.is_none() && extend_ms.is_none() && visible.is_none() {
                    return Err(SceneError("empty timer instruction".into()));
                }
                if start_ms.is_some() && extend_ms.is_some() {
                    return Err(SceneError(
                        "timer cannot start and extend in one instruction".into(),
                    ));
                }
                if start_ms.is_some_and(|ms| ms == 0 || ms > i64::MAX as u64)
                    || extend_ms.is_some_and(|ms| ms == 0 || ms > i64::MAX as u64)
                {
                    return Err(SceneError(
                        "timer duration must be positive and fit i64 milliseconds".into(),
                    ));
                }
            }
        }
        let id = if let Some(id) = &cue.cue_id {
            qualify(ns, id)?
        } else {
            // Versioned FNV-1a over authored normalized content/timing/guard,
            // never an output index. Identical instructions require explicit IDs.
            let mut hash = 0xcbf29ce484222325u64;
            let content = format!(
                "cue-v1:{:?}:{:?}:{:?}",
                cue.at_ms, cue.when, cue.instruction
            );
            for byte in content.bytes() {
                hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
            }
            format!("{ns}.cue_{hash:016x}")
        };
        if !ids.insert(id.clone()) {
            return Err(SceneError(format!(
                "duplicate/colliding cue identifier '{id}'; give repeated content distinct cue_id values"
            )));
        }
        cue.cue_id = Some(id);
    }
    Ok(())
}

pub(super) fn compile(mut story: SceneStory) -> SceneResult<SceneStory> {
    if story.schema_version != 1 || story.version == 0 || story.title.trim().is_empty() {
        return Err(SceneError(
            "scene DSL requires schema_version 1, positive story version and title".into(),
        ));
    }
    story.id = normalize(&story.id)?;
    story.start = normalize(&story.start)?;
    declarations(&story.globals)?;
    let mut contexts = BTreeSet::new();
    let mut namespaces = BTreeSet::new();
    for ctx in &mut story.contexts {
        ctx.id = normalize(&ctx.id)?;
        let ns = normalize(ctx.namespace.as_deref().unwrap_or(&ctx.id))?;
        if !contexts.insert(ctx.id.clone()) || !namespaces.insert(ns.clone()) {
            return Err(SceneError(format!(
                "duplicate normalized context/namespace '{}'",
                ctx.id
            )));
        }
        ctx.namespace = Some(ns.clone());
        declarations(&ctx.locals)?;
        if ctx.terminal && (!ctx.actions.is_empty() || ctx.timeout.is_some()) {
            return Err(SceneError(format!(
                "terminal context '{}' cannot have action/timeout handlers",
                ctx.id
            )));
        }
        if !ctx.terminal && ctx.actions.is_empty() && ctx.timeout.is_none() {
            return Err(SceneError(format!("context '{}' has no handlers", ctx.id)));
        }
        let mut action_ids = BTreeSet::new();
        for action in &mut ctx.actions {
            action.id = qualify(&ns, &action.id)?;
            if action.id.ends_with(".timeout") || !action_ids.insert(action.id.clone()) {
                return Err(SceneError(format!(
                    "duplicate or reserved action '{}'",
                    action.id
                )));
            }
        }
        let mut cue_ids = BTreeSet::new();
        let verify = (|| -> SceneResult<()> {
            for action in &mut ctx.actions {
                if action.duration_ms > i64::MAX as u64 {
                    return Err(SceneError(format!(
                        "action '{}' duration exceeds i64 milliseconds",
                        action.id
                    )));
                }
                if let Some(test) = &action.when {
                    validate_test(test, &story.globals, &ctx.locals, 0)?;
                }
                updates(&action.effects, &story.globals, &ctx.locals)?;
                if action.outcomes.is_empty()
                    || action
                        .outcomes
                        .last()
                        .is_some_and(|branch| branch.when.is_some())
                    || action.outcomes[..action.outcomes.len() - 1]
                        .iter()
                        .any(|branch| branch.when.is_none())
                {
                    return Err(SceneError(format!(
                        "action '{}': ordered IF/ELSE IF outcomes require exactly one final ELSE",
                        action.id
                    )));
                }
                if !action.interrupted_outcomes.is_empty()
                    && (action
                        .interrupted_outcomes
                        .last()
                        .is_some_and(|branch| branch.when.is_some())
                        || action.interrupted_outcomes[..action.interrupted_outcomes.len() - 1]
                            .iter()
                            .any(|branch| branch.when.is_none()))
                {
                    return Err(SceneError(format!(
                        "action '{}': interrupted outcomes require one final ELSE",
                        action.id
                    )));
                }
                for branch in action
                    .outcomes
                    .iter_mut()
                    .chain(&mut action.interrupted_outcomes)
                {
                    if let Some(test) = &branch.when {
                        validate_test(test, &story.globals, &ctx.locals, 0)?;
                    }
                    updates(&branch.outcome.effects, &story.globals, &ctx.locals)?;
                    cues(
                        &mut branch.outcome.cues,
                        &ns,
                        &action_ids,
                        &story.globals,
                        &ctx.locals,
                        &mut cue_ids,
                    )?;
                    if let Some(target) = &mut branch.outcome.goto {
                        *target = normalize(target)?;
                        if branch
                            .outcome
                            .cues
                            .iter()
                            .any(|cue| match &cue.instruction {
                                AuthoredInstruction::Timer { .. } => true,
                                AuthoredInstruction::Actions {
                                    available_actions: ActionSet::Eligible(_),
                                } => true,
                                AuthoredInstruction::Actions {
                                    available_actions: ActionSet::Only(ids),
                                } => !ids.is_empty(),
                                _ => false,
                            })
                        {
                            return Err(SceneError("a transition's result may narrate/lock input; define timers and nonempty offers in the destination entry".into()));
                        }
                    }
                }
                for branches in [&action.outcomes, &action.interrupted_outcomes] {
                    let mut guards = vec![];
                    for branch in branches {
                        if let Some(guard) = &branch.when {
                            if guards.contains(&guard) {
                                return Err(SceneError(format!(
                                    "action '{}': duplicate IF guard is unreachable",
                                    action.id
                                )));
                            }
                            guards.push(guard);
                        }
                    }
                }
            }
            if let Some(outcome) = &mut ctx.timeout {
                updates(&outcome.effects, &story.globals, &ctx.locals)?;
                cues(
                    &mut outcome.cues,
                    &ns,
                    &action_ids,
                    &story.globals,
                    &ctx.locals,
                    &mut cue_ids,
                )?;
                if let Some(target) = &mut outcome.goto {
                    *target = normalize(target)?;
                    if outcome.cues.iter().any(|cue| match &cue.instruction {
                        AuthoredInstruction::Timer { .. } => true,
                        AuthoredInstruction::Actions {
                            available_actions: ActionSet::Eligible(_),
                        } => true,
                        AuthoredInstruction::Actions {
                            available_actions: ActionSet::Only(ids),
                        } => !ids.is_empty(),
                        _ => false,
                    }) {
                        return Err(SceneError("a timeout transition's result may narrate/lock input; define timers and nonempty offers in the destination entry".into()));
                    }
                }
            }
            cues(
                &mut ctx.cues,
                &ns,
                &action_ids,
                &story.globals,
                &ctx.locals,
                &mut cue_ids,
            )?;
            if let Some(index) = ctx.cues.iter().position(|cue| {
                matches!(
                    cue.instruction,
                    AuthoredInstruction::Timer {
                        start_ms: Some(_),
                        ..
                    }
                )
            }) {
                if ctx.cues[index].at_ms.is_some() || ctx.cues[index].when.is_some() {
                    return Err(SceneError("initial timer start must be unconditional and sequential, not clock-scheduled".into()));
                }
                if ctx.cues[..index].iter().any(|cue| match &cue.instruction {
                    AuthoredInstruction::Actions {
                        available_actions: ActionSet::Eligible(_),
                    } => true,
                    AuthoredInstruction::Actions {
                        available_actions: ActionSet::Only(ids),
                    } => !ids.is_empty(),
                    AuthoredInstruction::Timer {
                        extend_ms: Some(_), ..
                    } => true,
                    _ => false,
                }) {
                    return Err(SceneError(
                        "timer start must precede nonempty offers or timer extensions".into(),
                    ));
                }
            }
            let all = ctx
                .cues
                .iter()
                .chain(ctx.actions.iter().flat_map(|a| {
                    a.outcomes
                        .iter()
                        .chain(&a.interrupted_outcomes)
                        .flat_map(|b| &b.outcome.cues)
                }))
                .chain(ctx.timeout.iter().flat_map(|o| &o.cues));
            let mut starts = 0;
            for cue in all {
                if matches!(
                    cue.instruction,
                    AuthoredInstruction::Timer {
                        start_ms: Some(_),
                        ..
                    }
                ) {
                    starts += 1;
                }
            }
            if starts > 0 && ctx.timeout.is_none() {
                return Err(SceneError(
                    "a timer requires an authored timeout outcome".into(),
                ));
            }
            Ok(())
        })();
        verify.map_err(|error| SceneError(format!("context '{}': {error}", ctx.id)))?;
    }
    if !contexts.contains(&story.start) {
        return Err(SceneError(format!(
            "unknown start context '{}'",
            story.start
        )));
    }
    let mut edges = BTreeMap::new();
    for ctx in &story.contexts {
        let targets: Vec<_> = ctx
            .actions
            .iter()
            .flat_map(|a| a.outcomes.iter().chain(&a.interrupted_outcomes))
            .map(|b| &b.outcome)
            .chain(ctx.timeout.iter())
            .filter_map(|outcome| outcome.goto.clone())
            .collect();
        for target in &targets {
            if !contexts.contains(target) {
                return Err(SceneError(format!(
                    "context '{}': unknown transition target '{target}'",
                    ctx.id
                )));
            }
        }
        edges.insert(ctx.id.clone(), targets);
    }
    let mut seen = BTreeSet::new();
    let mut pending = vec![story.start.clone()];
    while let Some(id) = pending.pop() {
        if seen.insert(id.clone()) {
            pending.extend(edges[&id].iter().cloned());
        }
    }
    for ctx in &story.contexts {
        if !seen.contains(&ctx.id) {
            return Err(SceneError(format!(
                "context '{}' is structurally unreachable",
                ctx.id
            )));
        }
    }
    if !story.contexts.iter().any(|ctx| ctx.terminal) {
        return Err(SceneError("story has no authored terminal context".into()));
    }
    Ok(story)
}
