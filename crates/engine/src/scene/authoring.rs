//! Renderer-independent inspection of the same compiler used by SceneEngine.
use super::{
    expressions::{Times, check},
    validation,
};
use grimm_semantic::{authoring::*, scene::*, story::Variables};
use std::collections::{BTreeMap, BTreeSet};

/// Compile once using M2 identity/type/reference rules, then inspect its graph.
/// Additional structural lint does not change runtime loading or save semantics.
pub fn analyze(authored: SceneStory) -> Report {
    let compiled = match validation::compile_located(authored.clone()) {
        Ok(story) => story,
        Err(error) => {
            return Report {
                schema_version: 1,
                diagnostics: vec![*error],
                index: None,
            };
        }
    };
    let mut index = StoryIndex {
        schema_version: 1,
        story_id: compiled.id.clone(),
        story_version: compiled.version,
        contexts: vec![],
    };
    let mut diagnostics = vec![];
    let mut edges = BTreeMap::<String, BTreeSet<String>>::new();
    for (ci, (ctx, original)) in compiled.contexts.iter().zip(&authored.contexts).enumerate() {
        let cp = format!("/contexts/{ci}");
        let mut entry = ContextIndex {
            id: ctx.id.clone(),
            namespace: ctx.namespace.clone().expect("compiled namespace"),
            terminal: ctx.terminal,
            source: Source::at(format!("{cp}/id")),
            actions: vec![],
            cues: vec![],
            narratives: vec![],
            transitions: vec![],
        };
        let mut targets = BTreeSet::new();
        let mut uncertain = ctx
            .cues
            .iter()
            .any(|cue| truth(cue.when.as_ref()).is_none());
        if !ctx.terminal && !entry_can_progress(ctx) {
            diagnostics.push(diagnostic(Severity::Error, "no_entry_progress", format!("context '{}' has neither a possible entry action offer nor an entry timer that can invoke its timeout", ctx.id), &cp, "Offer an action in the entry cue plan, or start an authored timer with a timeout handler."));
        }
        index_cues(&ctx.cues, &original.cues, &format!("{cp}/cues"), &mut entry);
        for (ai, (action, old)) in ctx.actions.iter().zip(&original.actions).enumerate() {
            let ap = format!("{cp}/actions/{ai}");
            entry.actions.push(Declaration {
                id: action.id.clone(),
                source: Source::at(format!("{ap}/id")),
                uses: vec![],
            });
            let available = truth(action.when.as_ref());
            uncertain |= available.is_none();
            if available == Some(false) {
                diagnostics.push(diagnostic(
                    Severity::Warning,
                    "constant_false_action",
                    format!(
                        "action '{}' is disabled by a constant-false condition",
                        action.id
                    ),
                    &ap,
                    "Remove the action or correct its eligibility condition.",
                ));
            }
            for (kind, branches, originals) in [
                ("outcomes", &action.outcomes, &old.outcomes),
                (
                    "interrupted_outcomes",
                    &action.interrupted_outcomes,
                    &old.interrupted_outcomes,
                ),
            ] {
                let mut covered = false;
                for (bi, (branch, old_branch)) in branches.iter().zip(originals).enumerate() {
                    let bp = format!("{ap}/{kind}/{bi}");
                    index_cues(
                        &branch.outcome.cues,
                        &old_branch.outcome.cues,
                        &format!("{bp}/cues"),
                        &mut entry,
                    );
                    let guard = truth(branch.when.as_ref());
                    uncertain |= guard.is_none();
                    let possible = available != Some(false) && guard != Some(false) && !covered;
                    if !possible && available != Some(false) {
                        diagnostics.push(diagnostic(Severity::Warning, "unreachable_branch", format!("branch of action '{}' is constant-false or follows an unconditional case", action.id), &bp, "Correct the guard or remove the unreachable branch."));
                    }
                    if let Some(target) = &branch.outcome.goto {
                        entry.transitions.push(Transition {
                            target: target.clone(),
                            action_id: Some(action.id.clone()),
                            kind: if kind == "outcomes" {
                                "action"
                            } else {
                                "interrupted_action"
                            }
                            .into(),
                            source: Source::at(format!("{bp}/goto")),
                        });
                        if possible {
                            targets.insert(target.clone());
                        }
                    }
                    covered |= guard == Some(true);
                }
            }
        }
        if let Some(timeout) = &ctx.timeout {
            index_cues(
                &timeout.cues,
                &original.timeout.as_ref().expect("same story").cues,
                &format!("{cp}/timeout/cues"),
                &mut entry,
            );
            if let Some(target) = &timeout.goto {
                entry.transitions.push(Transition {
                    target: target.clone(),
                    action_id: None,
                    kind: "timeout".into(),
                    source: Source::at(format!("{cp}/timeout/goto")),
                });
                targets.insert(target.clone());
            }
        }
        if !ctx.terminal && uncertain {
            diagnostics.push(diagnostic(Severity::Info, "state_dependent", format!("context '{}': conditional availability/outcomes remain state-dependent; structural exits do not prove playability", ctx.id), &cp, "Playtest state-dependent routes; this check does not enumerate world states."));
        }
        let instructions: BTreeMap<_, _> = ctx
            .cues
            .iter()
            .chain(ctx.actions.iter().flat_map(|a| {
                a.outcomes
                    .iter()
                    .chain(&a.interrupted_outcomes)
                    .flat_map(|b| &b.outcome.cues)
            }))
            .chain(ctx.timeout.iter().flat_map(|o| &o.cues))
            .map(|cue| {
                (
                    cue.cue_id.as_ref().expect("compiled cue ID"),
                    &cue.instruction,
                )
            })
            .collect();
        for cue in &entry.cues {
            if let AuthoredInstruction::Actions { available_actions } = instructions[&cue.cue_id] {
                for action in &mut entry.actions {
                    let usage = match available_actions {
                        ActionSet::Eligible(_) => {
                            Some(format!("{}/available_actions", cue.source.path))
                        }
                        ActionSet::Only(ids) => ids
                            .iter()
                            .position(|id| id == &action.id)
                            .map(|i| format!("{}/available_actions/{i}", cue.source.path)),
                    };
                    if let Some(path) = usage {
                        action.uses.push(Source::at(path));
                    }
                }
            }
        }
        for action in &mut entry.actions {
            action.uses.sort_by(|a, b| a.path.cmp(&b.path));
        }
        let mut content = BTreeMap::<String, Vec<Source>>::new();
        for cue in &entry.cues {
            if let Some(id) = &cue.narrative_id {
                let mut source = cue.source.clone();
                source.path.push_str("/narrative_id");
                content.entry(id.clone()).or_default().push(source);
            }
        }
        entry.narratives = content
            .into_iter()
            .map(|(narrative_id, mut uses)| {
                uses.sort_by(|a, b| a.path.cmp(&b.path));
                ContentIndex { narrative_id, uses }
            })
            .collect();
        entry.actions.sort_by(|a, b| a.id.cmp(&b.id));
        entry.cues.sort_by(|a, b| a.cue_id.cmp(&b.cue_id));
        entry.transitions.sort_by(|a, b| {
            (&a.target, &a.action_id, &a.kind, &a.source.path).cmp(&(
                &b.target,
                &b.action_id,
                &b.kind,
                &b.source.path,
            ))
        });
        edges.insert(ctx.id.clone(), targets);
        index.contexts.push(entry);
    }
    index.contexts.sort_by(|a, b| a.id.cmp(&b.id));
    let reachable = reachable(&compiled.start, &edges);
    let mut can_finish: BTreeSet<_> = compiled
        .contexts
        .iter()
        .filter(|c| c.terminal)
        .map(|c| c.id.clone())
        .collect();
    loop {
        let before = can_finish.len();
        for (from, targets) in &edges {
            if targets.iter().any(|to| can_finish.contains(to)) {
                can_finish.insert(from.clone());
            }
        }
        if before == can_finish.len() {
            break;
        }
    }
    for context in &index.contexts {
        if !reachable.contains(&context.id) {
            diagnostics.push(diagnostic(
                Severity::Error,
                "unreachable_context",
                format!(
                    "context '{}' is unreachable after excluding constant-false routes",
                    context.id
                ),
                &context.source.path,
                "Connect this context through a possible route or remove it.",
            ));
        }
        if !context.terminal && edges[&context.id].is_empty() {
            diagnostics.push(diagnostic(
                Severity::Error,
                "no_exit",
                format!(
                    "nonterminal context '{}' has no possible authored transition",
                    context.id
                ),
                &context.source.path,
                "Add a goto to a reachable context or mark an intentional ending terminal.",
            ));
        } else if !can_finish.contains(&context.id) {
            diagnostics.push(diagnostic(Severity::Error, "no_terminal_path", format!("context '{}' belongs to a region with no structural path to an authored terminal ending", context.id), &context.source.path, "Add an exit from this region; a failed ending is a valid terminal exit."));
        }
    }
    diagnostics.sort_by(|a, b| {
        (&a.source.path, &a.code, &a.message).cmp(&(&b.source.path, &b.code, &b.message))
    });
    Report {
        schema_version: 1,
        diagnostics,
        index: Some(index),
    }
}

// A context cannot bootstrap via effects of actions it never offers. This
// only rejects literal absence/constant-false eligibility, not schedule timing.
fn entry_can_progress(context: &Context) -> bool {
    context.cues.iter().any(|cue| {
        if truth(cue.when.as_ref()) == Some(false) {
            return false;
        }
        match &cue.instruction {
            AuthoredInstruction::Timer {
                start_ms: Some(_), ..
            } => context.timeout.is_some(),
            AuthoredInstruction::Actions { available_actions } => {
                context.actions.iter().any(|action| {
                    truth(action.when.as_ref()) != Some(false)
                        && match available_actions {
                            ActionSet::Eligible(_) => true,
                            ActionSet::Only(ids) => ids.contains(&action.id),
                        }
                })
            }
            _ => false,
        }
    })
}

fn index_cues(
    cues: &[AuthoredCue],
    originals: &[AuthoredCue],
    path: &str,
    index: &mut ContextIndex,
) {
    for (i, (cue, original)) in cues.iter().zip(originals).enumerate() {
        index.cues.push(CueIndex {
            cue_id: cue.cue_id.clone().expect("compiled cue ID"),
            generated: original.cue_id.is_none(),
            narrative_id: match &cue.instruction {
                AuthoredInstruction::Narrative { narrative_id } => Some(narrative_id.clone()),
                _ => None,
            },
            source: Source::at(format!("{path}/{i}")),
        });
    }
}

fn diagnostic(
    severity: Severity,
    code: &str,
    message: String,
    path: &str,
    suggestion: &str,
) -> Diagnostic {
    Diagnostic {
        severity,
        code: code.into(),
        message,
        source: Source::at(path),
        related: vec![],
        suggestion: suggestion.into(),
    }
}

fn reachable(start: &str, edges: &BTreeMap<String, BTreeSet<String>>) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut pending = vec![start.to_string()];
    while let Some(id) = pending.pop() {
        if seen.insert(id.clone()) {
            pending.extend(edges[&id].iter().cloned());
        }
    }
    seen
}

/// Only constant expressions are evaluated. Initial variable values are never
/// treated as invariants; action effects can change them before a branch check.
fn truth(test: Option<&Test>) -> Option<bool> {
    let Some(test) = test else {
        return Some(true);
    };
    fn constant(expr: &Expr) -> bool {
        match expr {
            Expr::Literal(_) => true,
            Expr::Arithmetic { args, .. } => args.iter().all(|e| constant(e)),
            _ => false,
        }
    }
    fn independent(test: &Test) -> bool {
        match test {
            Test::Eq { left, right }
            | Test::Ne { left, right }
            | Test::Lt { left, right }
            | Test::Le { left, right }
            | Test::Gt { left, right }
            | Test::Ge { left, right } => constant(left) && constant(right),
            Test::All { tests } | Test::Any { tests } => tests.iter().all(independent),
            Test::Not { test } => independent(test),
        }
    }
    if independent(test) {
        check(
            test,
            &Variables::new(),
            &Variables::new(),
            Times::default(),
            0,
        )
        .ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use grimm_semantic::story::Value;

    #[test]
    fn static_guards_do_not_treat_initial_world_values_or_clock_as_constants() {
        let dynamic = Test::Eq {
            left: Expr::Variable {
                var: "global.ready".into(),
            },
            right: Expr::Literal(Value::Bool(false)),
        };
        assert_eq!(truth(Some(&dynamic)), None);
        let timed = Test::Lt {
            left: Expr::Builtin {
                clock: ClockValue::SelectionMs,
            },
            right: Expr::Literal(Value::Int(5000)),
        };
        assert_eq!(truth(Some(&timed)), None);
        let constant = Test::Eq {
            left: Expr::Arithmetic {
                calc: Arithmetic::Add,
                args: [
                    Box::new(Expr::Literal(Value::Int(2))),
                    Box::new(Expr::Literal(Value::Int(3))),
                ],
            },
            right: Expr::Literal(Value::Int(5)),
        };
        assert_eq!(truth(Some(&constant)), Some(true));
        assert_eq!(
            truth(Some(&Test::Not {
                test: Box::new(constant)
            })),
            Some(false)
        );
    }
}
