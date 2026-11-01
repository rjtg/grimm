use super::{SceneError, SceneResult, expressions::*};
use crate::expressions::same_type;
use grimm_semantic::authoring::{Diagnostic, Severity, Source};
use grimm_semantic::{
    scene::*,
    story::{Value, Variables},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Trace {
    path: String,
    related: Vec<Source>,
    declarations: BTreeMap<String, String>,
    schema_version: u32,
}
impl Trace {
    fn at(&mut self, path: impl Into<String>) {
        self.path = path.into();
        self.related.clear();
    }
    fn declaration(&mut self, key: String) {
        if let Some(first) = self.declarations.get(&key) {
            self.related.push(Source::at(first));
        } else {
            self.declarations.insert(key, self.path.clone());
        }
    }
}

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
    trace: &mut Trace,
) -> SceneResult<()> {
    let base = trace.path.clone();
    for (index, cue) in cues.iter_mut().enumerate() {
        trace.at(format!("{base}/{index}"));
        if cue.at_ms.is_some_and(|time| time > i64::MAX as u64) {
            return Err(SceneError("cue time exceeds i64 milliseconds".into()));
        }
        if let Some(test) = &cue.when {
            validate_test(test, globals, locals, 0)?;
        }
        if let Some(after) = &mut cue.after_cue {
            if trace.schema_version < 2 {
                return Err(SceneError(
                    "after_cue requires scene schema_version 2".into(),
                ));
            }
            *after = qualify(ns, after)?;
        }
        match &mut cue.instruction {
            AuthoredInstruction::Narrative {
                narrative_id,
                duration_ms,
            } => {
                if duration_ms.is_some() && trace.schema_version < 2 {
                    return Err(SceneError(
                        "narrative duration_ms requires scene schema_version 2".into(),
                    ));
                }
                if duration_ms.is_some_and(|duration| {
                    duration > i64::MAX as u64
                        || cue
                            .at_ms
                            .unwrap_or(0)
                            .checked_add(duration)
                            .is_none_or(|end| end > i64::MAX as u64)
                }) {
                    return Err(SceneError("narrative duration/time overflow".into()));
                }
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
                deadline_at_ms,
                unset_deadline,
                extend_ms,
                visible,
            } => {
                if trace.schema_version < 2 && (deadline_at_ms.is_some() || *unset_deadline) {
                    return Err(SceneError(
                        "deadline lifecycle directives require scene schema_version 2".into(),
                    ));
                }
                let modes = usize::from(start_ms.is_some())
                    + usize::from(deadline_at_ms.is_some())
                    + usize::from(*unset_deadline)
                    + usize::from(extend_ms.is_some());
                if modes > 1 {
                    return Err(SceneError(
                        "timer cannot combine set/absolute/unset/extend deadline operations".into(),
                    ));
                }
                if *unset_deadline && *visible == Some(true) {
                    return Err(SceneError(
                        "unset_deadline cannot also show the removed deadline".into(),
                    ));
                }
                if deadline_at_ms.is_some_and(|at| at > i64::MAX as u64) {
                    return Err(SceneError(
                        "absolute deadline exceeds i64 milliseconds".into(),
                    ));
                }
                if start_ms.is_none()
                    && deadline_at_ms.is_none()
                    && !*unset_deadline
                    && extend_ms.is_none()
                    && visible.is_none()
                {
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
            // Preserve M2/M4 content-derived identities for legacy definitions.
            // Only new duration/dependency/directive content uses the v2 seed.
            let legacy = match &cue.instruction {
                AuthoredInstruction::Narrative {
                    narrative_id,
                    duration_ms: None,
                } => Some(format!("Narrative {{ narrative_id: {narrative_id:?} }}")),
                AuthoredInstruction::Actions { .. } => Some(format!("{:?}", cue.instruction)),
                AuthoredInstruction::Timer {
                    start_ms,
                    deadline_at_ms: None,
                    unset_deadline: false,
                    extend_ms,
                    visible,
                } => Some(format!(
                    "Timer {{ start_ms: {start_ms:?}, extend_ms: {extend_ms:?}, visible: {visible:?} }}"
                )),
                _ => None,
            };
            let content = if let Some(legacy) = legacy.filter(|_| cue.after_cue.is_none()) {
                format!("cue-v1:{:?}:{:?}:{legacy}", cue.at_ms, cue.when)
            } else {
                format!(
                    "cue-v2:{:?}:{:?}:{:?}:{:?}",
                    cue.at_ms, cue.when, cue.after_cue, cue.instruction
                )
            };
            for byte in content.bytes() {
                hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
            }
            format!("{ns}.cue_{hash:016x}")
        };
        if cue.cue_id.is_some() {
            trace.at(format!("{base}/{index}/cue_id"));
        }
        trace.declaration(format!("cue:{id}"));
        if !ids.insert(id.clone()) {
            return Err(SceneError(format!(
                "duplicate/colliding cue identifier '{id}'; give repeated content distinct cue_id values"
            )));
        }
        cue.cue_id = Some(id);
    }
    Ok(())
}

pub(super) fn compile(story: SceneStory) -> SceneResult<SceneStory> {
    compile_located(story).map_err(|diagnostic| SceneError(diagnostic.message))
}

pub(super) fn compile_located(story: SceneStory) -> Result<SceneStory, Box<Diagnostic>> {
    let mut trace = Trace::default();
    compile_inner(story, &mut trace).map_err(|error| Box::new(Diagnostic {
        severity: Severity::Error,
        code: "invalid_scene".into(),
        suggestion: if error.0.contains("duplicate") || error.0.contains("collid") {
            "Use distinct normalized IDs; give repeated cue content explicit, distinct cue_id values."
        } else if error.0.contains("unknown") || error.0.contains("undeclared") {
            "Declare the referenced name or correct its spelling and scope."
        } else if error.0.contains("type") || error.0.contains("integer") {
            "Use operands and effects matching the declared variable type."
        } else if error.0.contains("unreachable") {
            "Connect this context from the start graph, or remove the unused declaration."
        } else {
            "Correct the declaration at this source path according to the scene-format reference."
        }.into(),
        message: error.0,
        source: Source::at(trace.path),
        related: trace.related,
    }))
}

fn compile_inner(mut story: SceneStory, trace: &mut Trace) -> SceneResult<SceneStory> {
    trace.schema_version = story.schema_version;
    if !matches!(story.schema_version, 1 | 2) || story.version == 0 || story.title.trim().is_empty()
    {
        return Err(SceneError(
            "scene DSL requires schema_version 1 or 2, positive story version and title".into(),
        ));
    }
    trace.at("/id");
    story.id = normalize(&story.id)?;
    trace.at("/start");
    story.start = normalize(&story.start)?;
    trace.at("/globals");
    declarations(&story.globals)?;
    let mut contexts = BTreeSet::new();
    let mut namespaces = BTreeSet::new();
    for (ci, ctx) in story.contexts.iter_mut().enumerate() {
        let cp = format!("/contexts/{ci}");
        trace.at(format!("{cp}/id"));
        ctx.id = normalize(&ctx.id)?;
        trace.declaration(format!("context:{}", ctx.id));
        if !contexts.insert(ctx.id.clone()) {
            return Err(SceneError(format!(
                "duplicate normalized context '{}'",
                ctx.id
            )));
        }
        trace.at(if ctx.namespace.is_some() {
            format!("{cp}/namespace")
        } else {
            format!("{cp}/id")
        });
        let ns = normalize(ctx.namespace.as_deref().unwrap_or(&ctx.id))?;
        trace.declaration(format!("namespace:{ns}"));
        if !namespaces.insert(ns.clone()) {
            return Err(SceneError(format!(
                "duplicate normalized context/namespace '{ns}' (context '{}')",
                ctx.id
            )));
        }
        ctx.namespace = Some(ns.clone());
        trace.at(format!("{cp}/locals"));
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
        for (ai, action) in ctx.actions.iter_mut().enumerate() {
            trace.at(format!("{cp}/actions/{ai}/id"));
            action.id = qualify(&ns, &action.id)?;
            trace.declaration(format!("action:{}", action.id));
            if action.id.ends_with(".timeout") || !action_ids.insert(action.id.clone()) {
                return Err(SceneError(format!(
                    "duplicate or reserved action '{}'",
                    action.id
                )));
            }
        }
        let mut cue_ids = BTreeSet::new();
        let verify = (|| -> SceneResult<()> {
            for (ai, action) in ctx.actions.iter_mut().enumerate() {
                let ap = format!("{cp}/actions/{ai}");
                trace.at(&ap);
                if action.duration_ms > i64::MAX as u64 {
                    return Err(SceneError(format!(
                        "action '{}' duration exceeds i64 milliseconds",
                        action.id
                    )));
                }
                if let Some(test) = &action.when {
                    trace.at(format!("{ap}/when"));
                    validate_test(test, &story.globals, &ctx.locals, 0)?;
                }
                trace.at(format!("{ap}/effects"));
                updates(&action.effects, &story.globals, &ctx.locals)?;
                trace.at(format!("{ap}/outcomes"));
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
                let normal_count = action.outcomes.len();
                for (bi, branch) in action
                    .outcomes
                    .iter_mut()
                    .chain(&mut action.interrupted_outcomes)
                    .enumerate()
                {
                    let bp = if bi < normal_count {
                        format!("{ap}/outcomes/{bi}")
                    } else {
                        format!("{ap}/interrupted_outcomes/{}", bi - normal_count)
                    };
                    trace.at(&bp);
                    if let Some(test) = &branch.when {
                        validate_test(test, &story.globals, &ctx.locals, 0)?;
                    }
                    updates(&branch.outcome.effects, &story.globals, &ctx.locals)?;
                    trace.at(format!("{bp}/cues"));
                    cues(
                        &mut branch.outcome.cues,
                        &ns,
                        &action_ids,
                        &story.globals,
                        &ctx.locals,
                        &mut cue_ids,
                        trace,
                    )?;
                    trace.at(&bp);
                    if let Some(target) = &mut branch.outcome.goto {
                        *target = normalize(target)?;
                        if branch.outcome.cues.iter().any(|cue| cue.at_ms.is_some()) {
                            return Err(SceneError("transition outcome cues cannot specify at_ms; author timed cues in the destination context's entry cue plan".into()));
                        }
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
                for (kind, branches) in [
                    ("outcomes", &action.outcomes),
                    ("interrupted_outcomes", &action.interrupted_outcomes),
                ] {
                    let mut guards: Vec<(&Test, String)> = vec![];
                    for (bi, branch) in branches.iter().enumerate() {
                        if let Some(guard) = &branch.when {
                            let path = format!("{ap}/{kind}/{bi}/when");
                            if let Some((_, first)) =
                                guards.iter().find(|(previous, _)| *previous == guard)
                            {
                                trace.at(&path);
                                trace.related.push(Source::at(first));
                                return Err(SceneError(format!(
                                    "action '{}': duplicate IF guard is unreachable",
                                    action.id
                                )));
                            }
                            guards.push((guard, path));
                        }
                    }
                }
            }
            if let Some(outcome) = &mut ctx.timeout {
                trace.at(format!("{cp}/timeout"));
                updates(&outcome.effects, &story.globals, &ctx.locals)?;
                trace.at(format!("{cp}/timeout/cues"));
                cues(
                    &mut outcome.cues,
                    &ns,
                    &action_ids,
                    &story.globals,
                    &ctx.locals,
                    &mut cue_ids,
                    trace,
                )?;
                trace.at(format!("{cp}/timeout"));
                if let Some(target) = &mut outcome.goto {
                    *target = normalize(target)?;
                    if outcome.cues.iter().any(|cue| cue.at_ms.is_some()) {
                        return Err(SceneError("timeout transition outcome cues cannot specify at_ms; author timed cues in the destination context's entry cue plan".into()));
                    }
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
            trace.at(format!("{cp}/cues"));
            cues(
                &mut ctx.cues,
                &ns,
                &action_ids,
                &story.globals,
                &ctx.locals,
                &mut cue_ids,
                trace,
            )?;
            if story.schema_version == 1
                && let Some(index) = ctx.cues.iter().position(|cue| {
                    matches!(
                        cue.instruction,
                        AuthoredInstruction::Timer {
                            start_ms: Some(_),
                            ..
                        }
                    )
                })
            {
                trace.at(format!("{cp}/cues/{index}"));
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
                    } | AuthoredInstruction::Timer {
                        deadline_at_ms: Some(_),
                        ..
                    }
                ) {
                    starts += 1;
                }
            }
            let all_cues: Vec<_> = ctx
                .cues
                .iter()
                .chain(ctx.actions.iter().flat_map(|a| {
                    a.outcomes
                        .iter()
                        .chain(&a.interrupted_outcomes)
                        .flat_map(|b| &b.outcome.cues)
                }))
                .chain(ctx.timeout.iter().flat_map(|o| &o.cues))
                .collect();
            let dependencies: BTreeMap<_, _> = all_cues
                .iter()
                .filter_map(|cue| {
                    cue.after_cue
                        .as_ref()
                        .map(|after| (cue.cue_id.as_ref().expect("compiled cue ID"), after))
                })
                .collect();
            for (id, after) in &dependencies {
                trace.at(trace.declarations[&format!("cue:{id}")].clone());
                if !cue_ids.contains(*after) {
                    return Err(SceneError(format!(
                        "cue '{id}' has unknown after_cue '{after}'"
                    )));
                }
                let mut path = BTreeSet::new();
                let mut cursor = *id;
                while let Some(next) = dependencies.get(cursor) {
                    if !path.insert(cursor) {
                        return Err(SceneError(format!(
                            "cyclic after_cue dependency at '{cursor}'"
                        )));
                    }
                    cursor = next;
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
    trace.at("/start");
    if !contexts.contains(&story.start) {
        return Err(SceneError(format!(
            "unknown start context '{}'",
            story.start
        )));
    }
    let mut edges = BTreeMap::new();
    for (ci, ctx) in story.contexts.iter().enumerate() {
        let mut targets = vec![];
        let outcomes =
            ctx.actions
                .iter()
                .enumerate()
                .flat_map(|(ai, action)| {
                    action
                        .outcomes
                        .iter()
                        .enumerate()
                        .map(move |(bi, b)| {
                            (
                                format!("/contexts/{ci}/actions/{ai}/outcomes/{bi}/goto"),
                                &b.outcome,
                            )
                        })
                        .chain(action.interrupted_outcomes.iter().enumerate().map(
                            move |(bi, b)| {
                                (
                                    format!(
                                        "/contexts/{ci}/actions/{ai}/interrupted_outcomes/{bi}/goto"
                                    ),
                                    &b.outcome,
                                )
                            },
                        ))
                })
                .chain(
                    ctx.timeout
                        .iter()
                        .map(|o| (format!("/contexts/{ci}/timeout/goto"), o)),
                );
        for (path, outcome) in outcomes {
            if let Some(target) = &outcome.goto {
                trace.at(path);
                if !contexts.contains(target) {
                    return Err(SceneError(format!(
                        "context '{}': unknown transition target '{target}'",
                        ctx.id
                    )));
                }
                targets.push(target.clone());
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
    for (ci, ctx) in story.contexts.iter().enumerate() {
        trace.at(format!("/contexts/{ci}/id"));
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
