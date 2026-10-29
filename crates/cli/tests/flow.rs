//! Deterministic engine and renderer-neutral host tests. No clock or sleeping.
use grimm_adapters::{OutputAdapter, TextAdapter, lifecycle_input};
use grimm_cli::parse_story;
use grimm_engine::{ActionError, Engine, FlowError, PlayabilityLimits};
use grimm_presentation::{PresentationCommand, PresentationEvent, PresentationFeedback};
use grimm_semantic::{
    GameAction,
    flow::{FlowEvent, FlowInput, FlowPhase},
    story::Story,
    time::{DecisionPhase, TimeEvent, TimeInput},
};

const SIGNAL: &str = include_str!("../../../stories/the-silent-signal.yaml");

// Keep a gated timed fixture to test the reusable lifecycle independently of
// the playtest-driven narrative-only buildup + immediate decision in the game.
fn gated_story() -> Story {
    let mut story: Story = serde_yaml_ng::from_str(SIGNAL).unwrap();
    let relay = story.nodes.remove(3);
    let emergency = &mut story.nodes[2];
    emergency.continue_to = None;
    emergency.presentation_required = true;
    emergency.choices = relay.choices;
    emergency.decision = relay.decision;
    story
}
fn engine() -> Engine {
    Engine::new(gated_story()).unwrap()
}
fn flow(engine: &mut Engine, event: FlowEvent) {
    engine
        .flow_input(&FlowInput {
            revision: engine.output().revision,
            event,
        })
        .unwrap();
}
fn time(engine: &mut Engine, event: TimeEvent) {
    engine
        .time_input(&TimeInput {
            revision: engine.output().revision,
            event,
        })
        .unwrap();
}
fn choose(engine: &mut Engine, id: &str) {
    engine
        .apply(&GameAction {
            revision: engine.output().revision,
            choice_id: id.into(),
        })
        .unwrap();
}
fn emergency() -> Engine {
    let mut engine = engine();
    flow(&mut engine, FlowEvent::PresentationComplete);
    assert_eq!(engine.save().node_id, "carrier");
    flow(&mut engine, FlowEvent::PresentationComplete);
    assert_eq!(engine.save().node_id, "emergency");
    engine
}

#[test]
fn consecutive_presentations_advance_once_without_actions_or_offers() {
    let mut engine = engine();
    for expected in ["carrier", "emergency"] {
        let output = engine.output();
        assert!(output.offers.is_empty());
        let input = FlowInput {
            revision: output.revision,
            event: FlowEvent::PresentationComplete,
        };
        for revision in [output.revision + 1, u64::MAX] {
            let before = engine.save();
            assert_eq!(
                engine.flow_input(&FlowInput {
                    revision,
                    event: FlowEvent::PresentationComplete
                }),
                Err(FlowError::StaleRevision)
            );
            assert_eq!(engine.save(), before);
        }
        engine.flow_input(&input).unwrap();
        assert_eq!(engine.save().node_id, expected);
        let after = engine.save();
        assert_eq!(engine.flow_input(&input), Err(FlowError::StaleRevision));
        assert_eq!(after, engine.save());
    }
}

#[test]
fn gated_deadline_requires_completion_then_actual_readiness() {
    let mut engine = emergency();
    let before = engine.save();
    assert!(
        engine
            .flow_input(&FlowInput {
                revision: before.revision,
                event: FlowEvent::InteractionReady,
            })
            .is_err()
    );
    for event in [TimeEvent::Ready, TimeEvent::Elapsed { total_ms: u64::MAX }] {
        assert!(
            engine
                .time_input(&TimeInput {
                    revision: before.revision,
                    event
                })
                .is_err()
        );
    }
    assert_eq!(engine.save(), before);
    assert!(engine.output().offers.is_empty());
    assert!(
        engine
            .apply(&GameAction {
                revision: before.revision,
                choice_id: "answer".into()
            })
            .is_err()
    );
    flow(&mut engine, FlowEvent::PresentationComplete);
    assert_eq!(engine.output().offers.len(), 2);
    assert_eq!(engine.output().decision.unwrap().remaining_ms, 12000);
    assert!(
        engine
            .apply(&GameAction {
                revision: engine.output().revision,
                choice_id: "answer".into()
            })
            .is_err()
    );
    assert!(
        engine
            .time_input(&TimeInput {
                revision: engine.output().revision,
                event: TimeEvent::Elapsed { total_ms: 999999 }
            })
            .is_err()
    );
    let stale_ready = FlowInput {
        revision: before.revision,
        event: FlowEvent::InteractionReady,
    };
    assert_eq!(
        engine.flow_input(&stale_ready),
        Err(FlowError::StaleRevision)
    );
    flow(&mut engine, FlowEvent::InteractionReady);
    let ready = engine.save();
    assert!(
        engine
            .flow_input(&FlowInput {
                revision: ready.revision,
                event: FlowEvent::InteractionReady
            })
            .is_err()
    );
    assert!(
        engine
            .flow_input(&FlowInput {
                revision: ready.revision,
                event: FlowEvent::PresentationComplete
            })
            .is_err()
    );
    assert_eq!(engine.save(), ready);
    time(&mut engine, TimeEvent::Elapsed { total_ms: 11999 });
    choose(&mut engine, "answer");
    assert_eq!(engine.save().node_id, "investigation");
}

#[test]
fn gated_exact_boundary_expires_once_and_old_action_cannot_commit() {
    let mut engine = emergency();
    flow(&mut engine, FlowEvent::PresentationComplete);
    // Existing temporal Ready is safely extended; it cannot bypass presentation.
    time(&mut engine, TimeEvent::Ready);
    let old = engine.output().revision;
    time(&mut engine, TimeEvent::Elapsed { total_ms: 12000 });
    assert_eq!(engine.save().node_id, "lost");
    let after = engine.save();
    assert_eq!(
        engine.apply(&GameAction {
            revision: old,
            choice_id: "answer".into()
        }),
        Err(ActionError::StaleRevision)
    );
    assert!(
        engine
            .time_input(&TimeInput {
                revision: old,
                event: TimeEvent::Elapsed { total_ms: 12000 }
            })
            .is_err()
    );
    assert_eq!(after, engine.save());
}

#[test]
fn pause_and_resume_never_complete_a_presentation_or_revive_old_callbacks() {
    let mut engine = emergency();
    for presenting in [true, false] {
        let old = engine.output().revision;
        flow(&mut engine, FlowEvent::Pause);
        let paused = engine.save();
        assert!(engine.output().offers.is_empty());
        assert!(
            engine
                .flow_input(&FlowInput {
                    revision: paused.revision,
                    event: FlowEvent::PresentationComplete
                })
                .is_err()
        );
        assert_eq!(engine.save(), paused);
        flow(&mut engine, FlowEvent::Resume);
        assert_eq!(
            engine.output().flow.unwrap().phase,
            if presenting {
                FlowPhase::Presenting
            } else {
                FlowPhase::AwaitingInteraction
            }
        );
        assert!(
            engine
                .flow_input(&FlowInput {
                    revision: old,
                    event: FlowEvent::PresentationComplete
                })
                .is_err()
        );
        if presenting {
            flow(&mut engine, FlowEvent::PresentationComplete);
        }
    }
    flow(&mut engine, FlowEvent::InteractionReady);
    time(&mut engine, TimeEvent::Elapsed { total_ms: 2000 });
    time(&mut engine, TimeEvent::Pause);
    time(&mut engine, TimeEvent::Resume);
    assert_eq!(engine.output().decision.unwrap().remaining_ms, 10000);
    flow(&mut engine, FlowEvent::InteractionReady);
    time(&mut engine, TimeEvent::Elapsed { total_ms: 1 });
    assert_eq!(engine.output().decision.unwrap().remaining_ms, 9999);
}

#[test]
fn checkpoints_preserve_each_phase_and_reserve_new_epochs_in_fresh_hosts() {
    let mut pending = emergency();
    let mut cases = vec![engine().save(), pending.save()];
    flow(&mut pending, FlowEvent::Pause);
    cases.push(pending.save());
    let mut untimed = emergency();
    flow(&mut untimed, FlowEvent::PresentationComplete);
    flow(&mut untimed, FlowEvent::InteractionReady);
    choose(&mut untimed, "answer");
    choose(&mut untimed, "spectrum");
    choose(&mut untimed, "mark");
    flow(&mut untimed, FlowEvent::PresentationComplete);
    cases.push(untimed.save()); // Untimed gated presentation.
    flow(&mut untimed, FlowEvent::PresentationComplete);
    cases.push(untimed.save()); // Completed, controls not yet ready.
    flow(&mut untimed, FlowEvent::InteractionReady);
    cases.push(untimed.save()); // Untimed interactive gate.
    flow(&mut untimed, FlowEvent::Pause);
    cases.push(untimed.save());
    flow(&mut pending, FlowEvent::Resume);
    flow(&mut pending, FlowEvent::PresentationComplete);
    cases.push(pending.save());
    flow(&mut pending, FlowEvent::InteractionReady);
    time(&mut pending, TimeEvent::Elapsed { total_ms: 3456 });
    cases.push(pending.save());
    flow(&mut pending, FlowEvent::Pause);
    cases.push(pending.save());
    for original in cases {
        let json = serde_json::to_string(&original).unwrap();
        let mut loaded = engine();
        loaded
            .restore(serde_json::from_str(&json).unwrap())
            .unwrap();
        let restored = loaded.save();
        assert!(restored.revision > original.revision);
        assert_eq!(restored.node_id, original.node_id);
        assert_eq!(restored.variables, original.variables);
        let mut expected = original.flow.clone().unwrap();
        if expected.phase == FlowPhase::Interactive {
            expected.phase = FlowPhase::AwaitingInteraction;
        }
        assert_eq!(restored.flow, Some(expected));
        assert_eq!(
            restored
                .temporal
                .as_ref()
                .unwrap()
                .decision
                .as_ref()
                .map(|view| view.remaining_ms),
            original
                .temporal
                .as_ref()
                .unwrap()
                .decision
                .as_ref()
                .map(|view| view.remaining_ms)
        );
        for event in [
            FlowEvent::PresentationComplete,
            FlowEvent::InteractionReady,
            FlowEvent::Resume,
        ] {
            assert_eq!(
                loaded.flow_input(&FlowInput {
                    revision: original.revision,
                    event
                }),
                Err(FlowError::StaleRevision)
            );
        }
        assert_eq!(loaded.save(), restored);
    }
}

#[test]
fn invalid_checkpoints_are_transactional_including_phase_deadline_mismatches() {
    let engine_at_gate = emergency();
    let original = engine_at_gate.save();
    let mut bad = vec![];
    let mut save = original.clone();
    save.flow = None;
    bad.push(save);
    let mut save = original.clone();
    save.flow.as_mut().unwrap().phase = FlowPhase::Interactive;
    bad.push(save);
    let mut save = original.clone();
    save.temporal
        .as_mut()
        .unwrap()
        .decision
        .as_mut()
        .unwrap()
        .remaining_ms -= 1;
    bad.push(save);
    let mut save = original.clone();
    save.schema_version = 2;
    bad.push(save);
    let mut save = original.clone();
    save.revision = u64::MAX;
    bad.push(save);
    let mut save = engine().save();
    save.flow.as_mut().unwrap().phase = FlowPhase::AwaitingInteraction;
    bad.push(save);
    for save in bad {
        let mut current = emergency();
        let before = current.save();
        assert!(current.restore(save).is_err());
        assert_eq!(before, current.save());
    }
}

#[test]
fn visual_style_renderer_reports_completion_through_host_without_consuming_time() {
    struct MockVisual {
        pending: Option<u64>,
        work_completed: u64,
    }
    impl MockVisual {
        fn render(&mut self, commands: &[PresentationCommand]) {
            for command in commands {
                if let PresentationCommand::AwaitCompletion { revision, .. } = command {
                    self.pending = Some(*revision);
                }
            }
        }
        fn cinematic_finished(&self) -> PresentationFeedback {
            PresentationFeedback {
                revision: self.pending.unwrap(),
                event: PresentationEvent::Complete,
            }
        }
    }
    let mut engine = emergency();
    let before = engine.save();
    let mut adapter = TextAdapter::default();
    let mut renderer = MockVisual {
        pending: None,
        work_completed: 0,
    };
    renderer.render(&adapter.present(&engine.output()));
    renderer.work_completed += 1_000_000; // Arbitrary external animation/voice work.
    assert_eq!(engine.save(), before);
    assert!(engine.output().offers.is_empty());
    let callback = renderer.cinematic_finished();
    engine
        .flow_input(&lifecycle_input(callback.clone()))
        .unwrap();
    let commands = adapter.present(&engine.output());
    assert!(
        commands
            .iter()
            .any(|command| matches!(command, PresentationCommand::Choices(_)))
    );
    let revision = commands
        .iter()
        .find_map(|command| match command {
            PresentationCommand::InteractionAvailable { revision } => Some(*revision),
            _ => None,
        })
        .unwrap();
    engine
        .flow_input(&lifecycle_input(PresentationFeedback {
            revision,
            event: PresentationEvent::InteractionReady,
        }))
        .unwrap();
    assert_eq!(
        engine.output().decision.as_ref().unwrap().remaining_ms,
        12000
    );
    assert_eq!(
        engine.output().decision.unwrap().phase,
        DecisionPhase::Active
    );
    time(&mut engine, TimeEvent::Elapsed { total_ms: 12000 });
    assert_eq!(engine.save().node_id, "lost");
    assert!(
        engine
            .flow_input(&lifecycle_input(callback.clone()))
            .is_err()
    );
    engine.restore(before).unwrap();
    assert!(engine.flow_input(&lifecycle_input(callback)).is_err());
}

#[test]
fn authored_routes_cover_all_endings_and_state_dependent_offers() {
    let story: Story = serde_yaml_ng::from_str(SIGNAL).unwrap();
    assert!((10..=15).contains(&story.nodes.len()));
    assert!(
        story
            .nodes
            .iter()
            .filter(|node| node.continue_to.is_some())
            .count()
            >= 2
    );
    assert!(
        story
            .nodes
            .iter()
            .filter(|node| node.presentation_required)
            .count()
            >= 1
    );
    for (answer, lead, end, expected) in [
        ("answer", "spectrum", "guide", "rescue"),
        ("isolate", "spectrum", "independent", "rescue"),
        ("answer", "imani", "guide", "rescue"),
        ("isolate", "power", "sever", "quarantine"),
        ("answer", "power", "beam", "trap"),
    ] {
        let mut engine = parse_story("signal", SIGNAL).unwrap().1;
        for _ in 0..3 {
            flow(&mut engine, FlowEvent::PresentationComplete);
        }
        assert_eq!(engine.save().node_id, "relay");
        assert!(engine.output().flow.is_none());
        assert!(engine.output().content.len() == 1);
        time(&mut engine, TimeEvent::Ready);
        choose(&mut engine, answer);
        choose(&mut engine, lead);
        let action = match lead {
            "spectrum" => "mark",
            "imani" => "reconstruct",
            _ => "connect",
        };
        choose(&mut engine, action);
        flow(&mut engine, FlowEvent::PresentationComplete);
        assert_eq!(engine.save().node_id, "final_gate");
        assert!(engine.output().offers.is_empty());
        flow(&mut engine, FlowEvent::PresentationComplete);
        assert!(engine.output().decision.is_none());
        assert!(
            engine
                .apply(&GameAction {
                    revision: engine.output().revision,
                    choice_id: end.into()
                })
                .is_err()
        );
        flow(&mut engine, FlowEvent::InteractionReady);
        assert_eq!(
            engine
                .output()
                .offers
                .iter()
                .any(|offer| offer.id == "guide"),
            engine.save().variables["trust"] == grimm_semantic::story::Value::Bool(true)
                && engine.save().variables["decoded"] == grimm_semantic::story::Value::Bool(true)
        );
        choose(&mut engine, end);
        assert_eq!(engine.output().ending.unwrap().id, expected);
    }
    let engine = parse_story("signal", SIGNAL).unwrap().1;
    let report = engine.check_playability(PlayabilityLimits::default());
    assert!(!report.truncated);
    assert!(report.deadlocks.is_empty(), "{report:?}");
    assert_eq!(report.endings.len(), 3);
    assert!(
        engine
            .check_playability(PlayabilityLimits {
                max_states: 2,
                max_transitions: 2
            })
            .truncated
    );
}

#[test]
fn legacy_stories_omit_flow_and_schema_three_lifecycle_codecs_round_trip() {
    for yaml in [
        include_str!("../../../stories/the-last-witness.yaml"),
        include_str!("../../../stories/the-last-lighthouse.yaml"),
        include_str!("../../../stories/the-last-train.yaml"),
    ] {
        let engine = parse_story("legacy", yaml).unwrap().1;
        assert!(engine.output().flow.is_none());
        assert!(
            serde_json::to_value(engine.output())
                .unwrap()
                .get("flow")
                .is_none()
        );
        assert!(
            serde_json::to_value(engine.save())
                .unwrap()
                .get("flow")
                .is_none()
        );
        let story: Story = serde_yaml_ng::from_str(yaml).unwrap();
        let node = serde_json::to_value(&story.nodes[0]).unwrap();
        assert!(node.get("continue_to").is_none());
        assert!(node.get("presentation_required").is_none());
        let mut bad = engine.save();
        bad.schema_version = 3;
        let mut current = parse_story("legacy", yaml).unwrap().1;
        assert!(current.restore(bad).is_err());
    }
    for event in [
        FlowEvent::PresentationComplete,
        FlowEvent::InteractionReady,
        FlowEvent::Pause,
        FlowEvent::Resume,
    ] {
        let input = FlowInput {
            revision: 17,
            event,
        };
        assert_eq!(
            input,
            serde_json::from_str(&serde_json::to_string(&input).unwrap()).unwrap()
        );
    }
}

#[test]
fn deterministic_lifecycle_replay_has_identical_checkpoints() {
    fn replay() -> Vec<grimm_semantic::save::SaveGame> {
        let mut engine = engine();
        let mut history = vec![engine.save()];
        for event in [
            FlowEvent::Pause,
            FlowEvent::Resume,
            FlowEvent::PresentationComplete,
            FlowEvent::PresentationComplete,
            FlowEvent::PresentationComplete,
            FlowEvent::InteractionReady,
        ] {
            flow(&mut engine, event);
            history.push(engine.save());
        }
        time(&mut engine, TimeEvent::Elapsed { total_ms: 4000 });
        history.push(engine.save());
        choose(&mut engine, "answer");
        history.push(engine.save());
        history
    }
    assert_eq!(replay(), replay());
}

#[test]
fn rejects_invalid_configurations_and_continuation_cycles() {
    let story = gated_story();
    let mut bad = vec![];
    let mut s = story.clone();
    s.nodes[0].continue_to = Some("missing".into());
    bad.push((s, "does not exist"));
    let mut s = story.clone();
    s.nodes[1].continue_to = Some("storm".into());
    bad.push((s, "cycle"));
    let mut s = story.clone();
    s.nodes[0].continue_to = Some("storm".into());
    bad.push((s, "cycle"));
    let mut s = story.clone();
    s.nodes[0].choices = story.nodes[2].choices.clone();
    bad.push((s, "cannot accompany"));
    let mut s = story.clone();
    s.nodes[0].decision = story.nodes[2].decision.clone();
    bad.push((s, "cannot accompany"));
    let mut s = story.clone();
    s.nodes[0].presentation_required = true;
    bad.push((s, "cannot accompany"));
    let mut s = story.clone();
    s.nodes[10].presentation_required = true;
    bad.push((s, "ending"));
    let mut s = story.clone();
    s.schema_version = 2;
    bad.push((s, "requires schema 3"));
    for (s, expected) in bad {
        assert!(Engine::new(s).unwrap_err().to_string().contains(expected));
    }
    // Continuations intentionally have no effects or world costs; reject unknown syntax.
    assert!(
        parse_story(
            "bad",
            &SIGNAL.replace(
                "continue_to: carrier",
                "continue_to: {target: carrier, effects: []}"
            )
        )
        .is_err()
    );
}

#[test]
fn untimed_flow_is_optional_and_overflow_and_bad_destinations_do_not_commit() {
    let mut story = gated_story();
    story.time = None;
    story.nodes[2].decision = None;
    // lost was reachable only by timeout; keep it reachable via a choice.
    story.nodes[2].choices[1].target = "lost".into();
    let mut engine = Engine::new(story.clone()).unwrap();
    let mut save = engine.save();
    save.revision = u64::MAX - 2;
    engine.restore(save).unwrap();
    let before = engine.save();
    assert!(
        engine
            .flow_input(&FlowInput {
                revision: before.revision,
                event: FlowEvent::PresentationComplete
            })
            .is_err()
    );
    assert_eq!(engine.save(), before);
    assert!(engine.save().temporal.is_none());
    // Inject a conditional deadlock behind a narrative segment, still structurally valid.
    story.nodes[1].continue_to = Some("investigation".into());
    story.nodes[0].continue_to = Some("emergency".into());
    story.nodes[2].choices[0].target = "carrier".into();
    for choice in &mut story.nodes[4].choices {
        choice.condition = Some(grimm_semantic::story::Condition::Equals {
            variable: "trust".into(),
            value: grimm_semantic::story::Value::Bool(false),
        });
    }
    let mut engine = Engine::new(story).unwrap();
    flow(&mut engine, FlowEvent::PresentationComplete);
    flow(&mut engine, FlowEvent::PresentationComplete);
    flow(&mut engine, FlowEvent::InteractionReady);
    choose(&mut engine, "answer"); // Enters narrative carrier with trust=true.
    let before = engine.save();
    assert!(
        engine
            .flow_input(&FlowInput {
                revision: before.revision,
                event: FlowEvent::PresentationComplete
            })
            .is_err()
    );
    assert_eq!(engine.save(), before);
    let report = engine.check_playability(PlayabilityLimits::default());
    assert!(!report.deadlocks.is_empty());
}
