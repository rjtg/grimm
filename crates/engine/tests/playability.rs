use grimm_engine::{Engine, PlayabilityLimits};
use grimm_semantic::{
    Ending, GameAction, NarrativeContent,
    story::{Choice, Condition, Effect, Node, Story, Value},
};

fn node(id: &str, target: &str) -> Node {
    Node {
        continue_to: None,
        presentation_required: false,
        decision: None,
        id: id.into(),
        content: vec![NarrativeContent::Narration { text: id.into() }],
        choices: vec![Choice {
            advance_minutes: 0,
            id: "next".into(),
            label: "Next".into(),
            target: target.into(),
            condition: None,
            effects: vec![],
        }],
        ending: None,
    }
}
fn story() -> Story {
    let mut end = node("end", "end");
    end.choices.clear();
    end.ending = Some(Ending {
        id: "finished".into(),
        title: "Finished".into(),
    });
    let mut definition = Story {
        time: None,
        schema_version: 1,
        id: "playability".into(),
        version: 1,
        title: "Playability fixture".into(),
        start: "a".into(),
        variables: [("ready".into(), Value::Bool(true))].into(),
        nodes: vec![
            node("a", "b"),
            node("b", "c"),
            node("c", "d"),
            node("d", "end"),
            end,
        ],
    };
    definition.nodes[1].choices[0].effects = vec![Effect::Set {
        variable: "ready".into(),
        value: Value::Bool(false),
    }];
    definition.nodes[3].choices[0].condition = Some(Condition::Equals {
        variable: "ready".into(),
        value: Value::Bool(true),
    });
    definition
}
fn next(engine: &mut Engine) {
    engine
        .apply(&GameAction {
            revision: engine.output().revision,
            choice_id: "next".into(),
        })
        .unwrap();
}

#[test]
fn finds_a_two_transition_state_dependent_deadlock_without_mutating_the_session() {
    let engine = Engine::new(story()).unwrap();
    let before = engine.save();
    let report = engine.check_playability(PlayabilityLimits::default());
    assert!(!report.truncated);
    assert_eq!(report.deadlocks.len(), 1);
    let issue = &report.deadlocks[0];
    assert_eq!(issue.node_id, "c");
    assert_eq!(
        issue
            .path
            .iter()
            .map(|step| (step.node_id.as_str(), step.choice_id.as_str()))
            .collect::<Vec<_>>(),
        [("a", "next"), ("b", "next")]
    );
    assert!(issue.reason.contains("choice 'next'"));
    assert!(issue.reason.contains("node 'd'"));
    assert_eq!(engine.save(), before);
    assert_eq!(
        report,
        engine.check_playability(PlayabilityLimits::default())
    );
    let mut played = Engine::new(story()).unwrap();
    next(&mut played);
    next(&mut played);
    assert_eq!(played.save().node_id, issue.node_id);
    assert!(played.output().offers.is_empty());
    assert!(played.output().ending.is_none());
    assert!(
        played
            .validate_current_state()
            .unwrap_err()
            .to_string()
            .contains("node 'd'")
    );
}

#[test]
fn explicit_bad_outcomes_are_endings_not_deadlocks() {
    let mut definition = story();
    definition.nodes.truncate(3);
    definition.nodes[2].choices.clear();
    definition.nodes[2].ending = Some(Ending {
        id: "bad".into(),
        title: "The warning ignored".into(),
    });
    let mut engine = Engine::new(definition).unwrap();
    let report = engine.check_playability(PlayabilityLimits::default());
    assert!(!report.truncated);
    assert!(report.deadlocks.is_empty());
    assert_eq!(report.endings.into_iter().collect::<Vec<_>>(), ["bad"]);
    next(&mut engine);
    next(&mut engine);
    assert_eq!(engine.output().ending.unwrap().id, "bad");
    assert_eq!(engine.save().variables["ready"], Value::Bool(false));
    engine.validate_current_state().unwrap();
}

fn cycle() -> Story {
    let mut definition = story();
    let end = definition.nodes.pop().unwrap();
    definition.nodes.truncate(2);
    definition.nodes.push(end);
    definition.nodes[1].choices[0].target = "a".into();
    definition.nodes[1].choices[0].effects.clear();
    let mut leave = definition.nodes[0].choices[0].clone();
    leave.id = "leave".into();
    leave.target = "end".into();
    definition.nodes[0].choices.push(leave);
    definition
}

#[test]
fn exact_state_deduplication_handles_cycles_without_hiding_changing_state() {
    let engine = Engine::new(cycle()).unwrap();
    let report = engine.check_playability(PlayabilityLimits::default());
    assert!(!report.truncated);
    assert_eq!(report.states_checked, 3);
    assert!(report.deadlocks.is_empty());
    assert!(report.endings.contains("finished"));
    let mut changing = cycle();
    changing.variables.insert("count".into(), Value::Int(0));
    changing.nodes[1].choices[0].effects.push(Effect::Add {
        variable: "count".into(),
        amount: 1,
    });
    let report = Engine::new(changing)
        .unwrap()
        .check_playability(PlayabilityLimits {
            max_states: 8,
            max_transitions: 20,
        });
    assert!(report.truncated);
    assert!(report.states_checked <= 8);
    assert!(report.transitions_checked <= 20);
    assert!(report.deadlocks.is_empty());
}

#[test]
fn executable_cycles_are_not_claimed_to_eventually_finish() {
    let mut definition = cycle();
    definition.nodes[0].choices[1].condition = Some(Condition::Equals {
        variable: "ready".into(),
        value: Value::Bool(false),
    });
    let report = Engine::new(definition)
        .unwrap()
        .check_playability(PlayabilityLimits::default());
    assert!(!report.truncated);
    assert_eq!(report.states_checked, 2);
    assert!(report.deadlocks.is_empty());
    assert!(report.endings.is_empty());
}

#[test]
fn cyclic_routes_can_still_lead_to_state_dependent_deadlocks() {
    let mut definition = story();
    let mut back = definition.nodes[0].choices[0].clone();
    back.id = "back".into();
    back.target = "a".into();
    definition.nodes[1].choices.push(back);
    let report = Engine::new(definition)
        .unwrap()
        .check_playability(PlayabilityLimits::default());
    assert!(!report.truncated);
    assert!(report.deadlocks.iter().any(|issue| issue.node_id == "c"));
}

#[test]
fn cutoffs_are_inconclusive_and_do_not_invent_deadlocks_in_partial_nodes() {
    let mut definition = story();
    definition
        .variables
        .insert("ready".into(), Value::Bool(false));
    let mut blocked = definition.nodes[0].choices[0].clone();
    blocked.id = "blocked".into();
    blocked.target = "d".into();
    definition.nodes[0].choices.insert(0, blocked);
    let engine = Engine::new(definition).unwrap();
    for limits in [
        PlayabilityLimits {
            max_states: 0,
            max_transitions: 100,
        },
        PlayabilityLimits {
            max_states: 1,
            max_transitions: 100,
        },
        PlayabilityLimits {
            max_states: 100,
            max_transitions: 1,
        },
    ] {
        let report = engine.check_playability(limits);
        assert!(report.truncated);
        assert!(report.deadlocks.is_empty());
        assert!(report.states_checked <= limits.max_states);
        assert!(report.transitions_checked <= limits.max_transitions);
    }
    let mut terminal = story();
    terminal.nodes = vec![terminal.nodes.pop().unwrap()];
    terminal.start = "end".into();
    let report = Engine::new(terminal)
        .unwrap()
        .check_playability(PlayabilityLimits {
            max_states: 1,
            max_transitions: 0,
        });
    assert!(!report.truncated);
    assert!(report.deadlocks.is_empty());
    assert!(report.endings.contains("finished"));
}

#[test]
fn checkpoints_preserve_findings_and_invalid_deadlock_restore_is_transactional() {
    let mut original = Engine::new(story()).unwrap();
    next(&mut original);
    let mut restored = Engine::new(story()).unwrap();
    restored.restore(original.save()).unwrap();
    assert_eq!(
        original.check_playability(PlayabilityLimits::default()),
        restored.check_playability(PlayabilityLimits::default())
    );
    let report = restored.check_playability(PlayabilityLimits::default());
    assert_eq!(report.deadlocks[0].path.len(), 1);
    assert_eq!(report.deadlocks[0].path[0].node_id, "b");
    next(&mut original);
    let before = restored.save();
    assert!(
        restored
            .restore(original.save())
            .unwrap_err()
            .to_string()
            .contains("no executable choices")
    );
    assert_eq!(restored.save(), before);
}

#[test]
fn detects_overflow_blocked_states_and_current_revision_exhaustion() {
    let mut definition = story();
    definition
        .variables
        .insert("count".into(), Value::Int(i64::MAX));
    definition.nodes[1].choices[0].effects = vec![Effect::Add {
        variable: "count".into(),
        amount: 1,
    }];
    let mut engine = Engine::new(definition).unwrap();
    let report = engine.check_playability(PlayabilityLimits::default());
    assert_eq!(report.deadlocks[0].node_id, "b");
    assert!(
        report.deadlocks[0]
            .reason
            .contains("overflows variable 'count'")
    );
    let mut save = engine.save();
    save.revision = u64::MAX;
    engine.restore(save).unwrap();
    let report = engine.check_playability(PlayabilityLimits::default());
    assert_eq!(report.deadlocks[0].node_id, "a");
    assert!(report.deadlocks[0].reason.contains("revision exhausted"));
}
