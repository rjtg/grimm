use grimm_engine::{ActionError, Engine, TimeError};
use grimm_semantic::{
    Ending, GameAction,
    story::{Choice, Condition, Decision, Effect, Node, Story, Timeout, Value, WorldTime},
    time::{DecisionPhase, TimeEvent, TimeInput},
};

fn choice(id: &str, target: &str, cost: u64) -> Choice {
    Choice {
        id: id.into(),
        label: id.into(),
        target: target.into(),
        condition: None,
        effects: vec![],
        advance_minutes: cost,
    }
}
fn node(id: &str, choices: Vec<Choice>) -> Node {
    Node {
        id: id.into(),
        content: vec![grimm_semantic::NarrativeContent::Narration { text: id.into() }],
        choices,
        ending: None,
        decision: None,
    }
}
fn story() -> Story {
    let mut fast = choice("fast", "alarm", 2);
    fast.effects.push(Effect::Set {
        variable: "evidence".into(),
        value: Value::Bool(true),
    });
    let mut slow = fast.clone();
    slow.id = "slow".into();
    slow.advance_minutes = 15;
    let mut finish = choice("finish", "win", 0);
    finish.condition = Some(Condition::All {
        conditions: vec![
            Condition::Equals {
                variable: "evidence".into(),
                value: Value::Bool(true),
            },
            Condition::WorldAtMost { minutes: 15 },
        ],
    });
    let mut alarm = node("alarm", vec![finish, choice("evacuate", "fail", 0)]);
    alarm.decision = Some(Decision {
        limit_ms: 1000,
        timeout: Timeout {
            target: "fail".into(),
            effects: vec![Effect::Set {
                variable: "evidence".into(),
                value: Value::Bool(false),
            }],
        },
    });
    let mut win = node("win", vec![]);
    win.ending = Some(Ending {
        id: "win".into(),
        title: "Win".into(),
    });
    let mut fail = node("fail", vec![]);
    fail.ending = Some(Ending {
        id: "fail".into(),
        title: "Failure is authored".into(),
    });
    Story {
        schema_version: 2,
        id: "temporal-fixture".into(),
        version: 1,
        title: "Temporal fixture".into(),
        start: "lobby".into(),
        variables: [
            ("evidence".into(), Value::Bool(false)),
            ("count".into(), Value::Int(0)),
        ]
        .into(),
        nodes: vec![
            node("lobby", vec![fast, slow, choice("quit", "fail", 0)]),
            alarm,
            win,
            fail,
        ],
        time: Some(WorldTime { start_minutes: 10 }),
    }
}
fn action(engine: &mut Engine, id: &str) -> Result<grimm_semantic::SemanticOutput, ActionError> {
    engine.apply(&GameAction {
        revision: engine.output().revision,
        choice_id: id.into(),
    })
}
fn event(
    engine: &mut Engine,
    event: TimeEvent,
) -> Result<grimm_semantic::SemanticOutput, TimeError> {
    engine.time_input(&TimeInput {
        revision: engine.output().revision,
        event,
    })
}
fn alarm() -> Engine {
    let mut engine = Engine::new(story()).unwrap();
    action(&mut engine, "fast").unwrap();
    engine
}

#[test]
fn authored_world_costs_and_nested_temporal_conditions_are_separate_from_real_time() {
    let mut fast = alarm();
    assert_eq!(fast.save().temporal.unwrap().world_minutes, 12);
    assert!(
        fast.output()
            .offers
            .iter()
            .any(|offer| offer.id == "finish")
    );
    let mut slow = Engine::new(story()).unwrap();
    action(&mut slow, "slow").unwrap();
    assert_eq!(slow.save().temporal.unwrap().world_minutes, 25);
    assert_eq!(
        slow.output()
            .offers
            .iter()
            .map(|offer| offer.id.as_str())
            .collect::<Vec<_>>(),
        ["evacuate"]
    );
    event(&mut fast, TimeEvent::Ready).unwrap();
    event(&mut fast, TimeEvent::Elapsed { total_ms: 900 }).unwrap();
    assert_eq!(fast.save().temporal.unwrap().world_minutes, 12);
    let mut definition = story();
    definition.nodes[0].choices[0].condition = Some(Condition::Not {
        condition: Box::new(Condition::WorldAtLeast { minutes: 11 }),
    });
    let mut engine = Engine::new(definition).unwrap();
    event(&mut engine, TimeEvent::AdvanceWorld { minutes: 1 }).unwrap();
    assert!(
        !engine
            .output()
            .offers
            .iter()
            .any(|offer| offer.id == "fast")
    );
}

#[test]
fn failed_world_cost_effect_and_destination_are_transactional() {
    for failure in [0, 1, 2] {
        let mut definition = story();
        match failure {
            0 => definition.nodes[0].choices[0].advance_minutes = u64::MAX,
            1 => {
                definition
                    .variables
                    .insert("count".into(), Value::Int(i64::MAX));
                definition.nodes[0].choices[0].effects.push(Effect::Add {
                    variable: "count".into(),
                    amount: 1,
                });
            }
            _ => {
                definition.nodes[1].decision = None;
                definition.nodes[1].choices[0].condition =
                    Some(Condition::WorldAtMost { minutes: 0 });
                definition.nodes[1].choices[1].condition =
                    Some(Condition::WorldAtMost { minutes: 0 });
            }
        }
        let mut engine = Engine::new(definition).unwrap();
        let before = engine.save();
        assert!(action(&mut engine, "fast").is_err());
        assert_eq!(engine.save(), before);
    }
}

#[test]
fn failed_timed_action_preserves_remaining_time_and_all_canonical_state() {
    for world_overflow in [false, true] {
        let mut definition = story();
        if world_overflow {
            definition.nodes[1].choices[0].advance_minutes = u64::MAX;
        } else {
            definition
                .variables
                .insert("count".into(), Value::Int(i64::MAX));
            definition.nodes[1].choices[0].effects.push(Effect::Add {
                variable: "count".into(),
                amount: 1,
            });
        }
        let mut engine = Engine::new(definition).unwrap();
        action(&mut engine, "fast").unwrap();
        event(&mut engine, TimeEvent::Ready).unwrap();
        event(&mut engine, TimeEvent::Elapsed { total_ms: 200 }).unwrap();
        let before = engine.save();
        assert!(action(&mut engine, "finish").is_err());
        assert_eq!(engine.save(), before);
        assert_eq!(engine.output().decision.unwrap().remaining_ms, 800);
    }
}

#[test]
fn host_world_advance_is_explicit_revision_scoped_and_checked() {
    let mut engine = Engine::new(story()).unwrap();
    let initial = engine.save();
    assert_eq!(
        event(&mut engine, TimeEvent::AdvanceWorld { minutes: 0 }),
        Err(TimeError::InvalidDuration)
    );
    assert_eq!(engine.save(), initial);
    event(&mut engine, TimeEvent::AdvanceWorld { minutes: 3 }).unwrap();
    assert_eq!(engine.save().temporal.unwrap().world_minutes, 13);
    assert_eq!(
        engine.apply(&GameAction {
            revision: 0,
            choice_id: "fast".into()
        }),
        Err(ActionError::StaleRevision)
    );
    action(&mut engine, "fast").unwrap();
    let before = engine.save();
    assert_eq!(
        event(&mut engine, TimeEvent::AdvanceWorld { minutes: 1 }),
        Err(TimeError::InvalidDuration)
    );
    assert_eq!(engine.save(), before);
    let mut engine = Engine::new(story()).unwrap();
    let mut save = engine.save();
    save.temporal.as_mut().unwrap().world_minutes = u64::MAX;
    engine.restore(save).unwrap();
    let before = engine.save();
    assert_eq!(
        event(&mut engine, TimeEvent::AdvanceWorld { minutes: 1 }),
        Err(TimeError::Transition(ActionError::WorldTimeOverflow))
    );
    assert_eq!(engine.save(), before);
    let mut definition = story();
    for choice in &mut definition.nodes[0].choices {
        choice.condition = Some(Condition::WorldAtMost { minutes: 10 });
    }
    let mut engine = Engine::new(definition).unwrap();
    let before = engine.save();
    assert!(event(&mut engine, TimeEvent::AdvanceWorld { minutes: 1 }).is_err());
    assert_eq!(engine.save(), before);
    let mut engine = Engine::new(story()).unwrap();
    let mut near_exhaustion = engine.save();
    near_exhaustion.revision = u64::MAX - 2;
    engine.restore(near_exhaustion).unwrap();
    assert_eq!(
        engine
            .output()
            .offers
            .iter()
            .map(|offer| offer.id.as_str())
            .collect::<Vec<_>>(),
        ["quit"]
    );
    action(&mut engine, "quit").unwrap();
    let terminal = engine.save();
    assert_eq!(
        event(&mut engine, TimeEvent::AdvanceWorld { minutes: 1 }),
        Err(TimeError::InvalidPhase)
    );
    assert_eq!(engine.save(), terminal);
}

#[test]
fn slow_presentation_cannot_expire_an_unready_interaction_and_ready_is_not_resettable() {
    let mut engine = alarm();
    let pending = engine.save();
    assert_eq!(
        engine.output().decision.unwrap().phase,
        DecisionPhase::AwaitingReady
    );
    assert_eq!(
        event(&mut engine, TimeEvent::Elapsed { total_ms: u64::MAX }),
        Err(TimeError::InvalidPhase)
    );
    assert_eq!(
        action(&mut engine, "finish"),
        Err(ActionError::DecisionNotActive)
    );
    assert_eq!(engine.save(), pending);
    event(&mut engine, TimeEvent::Ready).unwrap();
    event(&mut engine, TimeEvent::Elapsed { total_ms: 200 }).unwrap();
    let before = engine.save();
    assert_eq!(
        event(&mut engine, TimeEvent::Ready),
        Err(TimeError::InvalidPhase)
    );
    for total_ms in [0, 199, 200] {
        assert_eq!(
            event(&mut engine, TimeEvent::Elapsed { total_ms }),
            Err(TimeError::NonIncreasingElapsed)
        );
    }
    assert_eq!(engine.save(), before);
}

#[test]
fn exact_boundary_timeout_wins_and_old_actions_or_expirations_cannot_commit_twice() {
    let mut engine = alarm();
    event(&mut engine, TimeEvent::Ready).unwrap();
    let old_revision = engine.output().revision;
    let expiration = TimeInput {
        revision: old_revision,
        event: TimeEvent::Elapsed { total_ms: 1000 },
    };
    let result = engine.time_input(&expiration).unwrap();
    assert_eq!(result.ending.unwrap().id, "fail");
    assert_eq!(engine.save().variables["evidence"], Value::Bool(false));
    let after = engine.save();
    assert_eq!(
        engine.apply(&GameAction {
            revision: old_revision,
            choice_id: "finish".into()
        }),
        Err(ActionError::StaleRevision)
    );
    assert_eq!(
        engine.time_input(&expiration),
        Err(TimeError::StaleRevision)
    );
    assert_eq!(engine.save(), after);
    let mut timely = alarm();
    event(&mut timely, TimeEvent::Ready).unwrap();
    event(&mut timely, TimeEvent::Elapsed { total_ms: 999 }).unwrap();
    assert_eq!(
        action(&mut timely, "finish").unwrap().ending.unwrap().id,
        "win"
    );
    assert_eq!(
        timely.time_input(&expiration),
        Err(TimeError::StaleRevision)
    );
}

#[test]
fn timeout_can_enter_an_authored_scene_and_never_carries_over_unseen_time() {
    let mut definition = story();
    let mut lost = node("lost", vec![choice("safe-exit", "fail", 0)]);
    lost.decision = Some(Decision {
        limit_ms: 500,
        timeout: Timeout {
            target: "fail".into(),
            effects: vec![],
        },
    });
    definition.nodes[1]
        .decision
        .as_mut()
        .unwrap()
        .timeout
        .target = "lost".into();
    definition.nodes.push(lost);
    let mut engine = Engine::new(definition).unwrap();
    action(&mut engine, "fast").unwrap();
    event(&mut engine, TimeEvent::Ready).unwrap();
    event(&mut engine, TimeEvent::Elapsed { total_ms: u64::MAX }).unwrap();
    assert_eq!(engine.save().node_id, "lost");
    let decision = engine.output().decision.unwrap();
    assert_eq!(decision.remaining_ms, 500);
    assert_eq!(decision.phase, DecisionPhase::AwaitingReady);
    event(&mut engine, TimeEvent::Ready).unwrap();
    assert_eq!(
        action(&mut engine, "safe-exit").unwrap().ending.unwrap().id,
        "fail"
    );
}

#[test]
fn pause_resume_preserves_remaining_and_invalidates_old_segment_events() {
    let mut engine = alarm();
    event(&mut engine, TimeEvent::Ready).unwrap();
    event(&mut engine, TimeEvent::Elapsed { total_ms: 200 }).unwrap();
    let old = engine.output().revision;
    event(&mut engine, TimeEvent::Pause).unwrap();
    let paused = engine.save();
    assert_eq!(engine.output().decision.unwrap().remaining_ms, 800);
    assert!(engine.output().offers.is_empty());
    assert_eq!(
        event(&mut engine, TimeEvent::Elapsed { total_ms: 900000 }),
        Err(TimeError::InvalidPhase)
    );
    assert_eq!(
        action(&mut engine, "finish"),
        Err(ActionError::DecisionNotActive)
    );
    assert_eq!(
        event(&mut engine, TimeEvent::Pause),
        Err(TimeError::InvalidPhase)
    );
    assert_eq!(engine.save(), paused);
    event(&mut engine, TimeEvent::Resume).unwrap();
    assert_eq!(
        engine.time_input(&TimeInput {
            revision: old,
            event: TimeEvent::Elapsed { total_ms: 1000 }
        }),
        Err(TimeError::StaleRevision)
    );
    event(&mut engine, TimeEvent::Ready).unwrap();
    event(&mut engine, TimeEvent::Elapsed { total_ms: 799 }).unwrap();
    assert_eq!(engine.output().decision.unwrap().remaining_ms, 1);
    event(&mut engine, TimeEvent::Elapsed { total_ms: 800 }).unwrap();
    assert!(engine.output().ending.is_some());
}

#[test]
fn active_and_paused_saves_freeze_offline_and_require_readiness_after_load() {
    for paused in [false, true] {
        let mut original = alarm();
        event(&mut original, TimeEvent::Ready).unwrap();
        event(&mut original, TimeEvent::Elapsed { total_ms: 400 }).unwrap();
        if paused {
            event(&mut original, TimeEvent::Pause).unwrap();
        }
        let saved = original.save();
        assert_eq!(saved.schema_version, 2);
        let mut restored = Engine::new(story()).unwrap();
        restored.restore(saved).unwrap();
        let remaining = restored.output().decision.unwrap();
        assert_eq!(remaining.remaining_ms, 600);
        assert_eq!(
            remaining.phase,
            if paused {
                DecisionPhase::Paused
            } else {
                DecisionPhase::AwaitingReady
            }
        );
        let before = restored.save();
        assert_eq!(
            event(&mut restored, TimeEvent::Elapsed { total_ms: 9999999 }),
            Err(TimeError::InvalidPhase)
        );
        assert_eq!(restored.save(), before);
        if paused {
            event(&mut restored, TimeEvent::Resume).unwrap();
        }
        event(&mut restored, TimeEvent::Ready).unwrap();
        event(&mut restored, TimeEvent::Elapsed { total_ms: 599 }).unwrap();
        assert_eq!(restored.output().decision.unwrap().remaining_ms, 1);
        event(&mut restored, TimeEvent::Elapsed { total_ms: 600 }).unwrap();
        assert_eq!(restored.output().ending.unwrap().id, "fail");
    }
}

#[test]
fn invalid_temporal_restores_preserve_every_part_of_the_active_state() {
    let mut engine = alarm();
    event(&mut engine, TimeEvent::Ready).unwrap();
    let good = engine.save();
    let mut bad = Vec::new();
    let mut s = good.clone();
    s.schema_version = 1;
    bad.push(s);
    let mut s = good.clone();
    s.temporal = None;
    bad.push(s);
    let mut s = good.clone();
    s.temporal.as_mut().unwrap().world_minutes = 9;
    bad.push(s);
    let mut s = good.clone();
    s.temporal.as_mut().unwrap().decision = None;
    bad.push(s);
    for ms in [0, 1001] {
        let mut s = good.clone();
        s.temporal
            .as_mut()
            .unwrap()
            .decision
            .as_mut()
            .unwrap()
            .remaining_ms = ms;
        bad.push(s);
    }
    let mut s = good.clone();
    s.revision = u64::MAX;
    bad.push(s);
    for saved in bad {
        assert!(engine.restore(saved).is_err());
        assert_eq!(engine.save(), good);
    }
    let old = good.revision;
    engine.restore(good).unwrap();
    assert_eq!(
        engine.time_input(&TimeInput {
            revision: old,
            event: TimeEvent::Ready
        }),
        Err(TimeError::StaleRevision)
    );
    assert_eq!(
        engine.apply(&GameAction {
            revision: old,
            choice_id: "finish".into()
        }),
        Err(ActionError::StaleRevision)
    );
    let checkpoint = engine.save();
    let mut fresh = Engine::new(story()).unwrap();
    fresh.restore(checkpoint.clone()).unwrap();
    assert!(fresh.output().revision > checkpoint.revision);
    event(&mut fresh, TimeEvent::Ready).unwrap();
    let before = fresh.save();
    assert_eq!(
        fresh.time_input(&TimeInput {
            revision: checkpoint.revision,
            event: TimeEvent::Elapsed { total_ms: 1000 }
        }),
        Err(TimeError::StaleRevision)
    );
    assert_eq!(
        fresh.apply(&GameAction {
            revision: checkpoint.revision,
            choice_id: "finish".into()
        }),
        Err(ActionError::StaleRevision)
    );
    assert_eq!(fresh.save(), before);
}

#[test]
fn identical_action_and_time_sequences_replay_exactly_without_a_clock() {
    let mut a = alarm();
    let mut b = alarm();
    for input in [
        TimeEvent::Ready,
        TimeEvent::Elapsed { total_ms: 123 },
        TimeEvent::Pause,
        TimeEvent::Resume,
        TimeEvent::Ready,
        TimeEvent::Elapsed { total_ms: 876 },
    ] {
        assert_eq!(event(&mut a, input.clone()), event(&mut b, input));
        assert_eq!(a.save(), b.save());
    }
    assert_eq!(action(&mut a, "finish"), action(&mut b, "finish"));
    assert_eq!(a.save(), b.save());
}

#[test]
fn invalid_timeout_rules_fail_validation_instead_of_expiring_into_an_error() {
    for case in 0..7 {
        let mut s = story();
        match case {
            0 => s.nodes[1].decision.as_mut().unwrap().limit_ms = 0,
            1 => s.nodes[1].decision.as_mut().unwrap().timeout.target = "missing".into(),
            2 => s.nodes[1].decision.as_mut().unwrap().timeout.target = "lobby".into(), // no zero-cost safe exit? remove it below
            3 => s.nodes[1]
                .decision
                .as_mut()
                .unwrap()
                .timeout
                .effects
                .push(Effect::Add {
                    variable: "count".into(),
                    amount: 1,
                }),
            4 => s.schema_version = 1,
            5 => s.time = None,
            _ => s.nodes[2].decision = s.nodes[1].decision.clone(),
        }
        if case == 2 {
            s.nodes[0].choices[2].condition = Some(Condition::WorldAtLeast { minutes: 100 });
        }
        assert!(Engine::new(s).is_err(), "case {case}");
    }
    let mut legacy = Engine::default();
    let before = legacy.save();
    assert_eq!(before.schema_version, 1);
    assert!(before.temporal.is_none());
    assert_eq!(
        event(&mut legacy, TimeEvent::Ready),
        Err(TimeError::NoTemporalRules)
    );
    assert_eq!(legacy.save(), before);
}
