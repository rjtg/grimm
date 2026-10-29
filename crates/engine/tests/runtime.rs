use grimm_engine::{ActionError, Engine};
use grimm_semantic::{
    Ending, GameAction, NarrativeContent,
    story::{Choice, Condition, Effect, Node, Story, Value},
};

fn narration(text: &str) -> Vec<NarrativeContent> {
    vec![NarrativeContent::Narration { text: text.into() }]
}

fn story() -> Story {
    Story {
        time: None,
        schema_version: 1,
        id: "fixture".into(),
        version: 1,
        title: "Fixture".into(),
        start: "start".into(),
        variables: [
            ("count".into(), Value::Int(0)),
            ("flag".into(), Value::Bool(false)),
            ("word".into(), Value::Text("unmade".into())),
        ]
        .into(),
        nodes: vec![
            Node {
                decision: None,
                id: "start".into(),
                content: narration("Start"),
                ending: None,
                choices: vec![
                    Choice {
                        advance_minutes: 0,
                        id: "step".into(),
                        label: "Step".into(),
                        target: "gate".into(),
                        condition: None,
                        effects: vec![
                            Effect::Set {
                                variable: "flag".into(),
                                value: Value::Bool(true),
                            },
                            Effect::Add {
                                variable: "count".into(),
                                amount: 1,
                            },
                            Effect::Set {
                                variable: "word".into(),
                                value: Value::Text("kept".into()),
                            },
                        ],
                    },
                    Choice {
                        advance_minutes: 0,
                        id: "direct".into(),
                        label: "Direct".into(),
                        target: "end".into(),
                        condition: Some(Condition::AtMost {
                            variable: "count".into(),
                            value: 0,
                        }),
                        effects: vec![],
                    },
                ],
            },
            Node {
                decision: None,
                id: "gate".into(),
                content: narration("Gate"),
                ending: None,
                choices: vec![Choice {
                    advance_minutes: 0,
                    id: "finish".into(),
                    label: "Finish".into(),
                    target: "end".into(),
                    condition: Some(Condition::All {
                        conditions: vec![
                            Condition::Equals {
                                variable: "flag".into(),
                                value: Value::Bool(true),
                            },
                            Condition::AtLeast {
                                variable: "count".into(),
                                value: 1,
                            },
                            Condition::Equals {
                                variable: "word".into(),
                                value: Value::Text("kept".into()),
                            },
                        ],
                    }),
                    effects: vec![],
                }],
            },
            Node {
                decision: None,
                id: "end".into(),
                content: narration("End"),
                ending: Some(Ending {
                    id: "finished".into(),
                    title: "Finished".into(),
                }),
                choices: vec![],
            },
        ],
    }
}

fn choose(engine: &mut Engine, id: &str) -> Result<grimm_semantic::SemanticOutput, ActionError> {
    engine.apply(&GameAction {
        revision: engine.output().revision,
        choice_id: id.into(),
    })
}

#[test]
fn reusable_runtime_commits_typed_effects_and_explicit_endings() {
    let mut engine = Engine::new(story()).unwrap();
    assert_eq!(engine.output().offers.len(), 2);
    choose(&mut engine, "step").unwrap();
    let save = engine.save();
    assert_eq!(save.node_id, "gate");
    assert_eq!(save.variables["count"], Value::Int(1));
    assert_eq!(save.variables["flag"], Value::Bool(true));
    assert_eq!(save.variables["word"], Value::Text("kept".into()));
    assert_eq!(engine.output().offers[0].id, "finish");
    assert_eq!(
        choose(&mut engine, "direct"),
        Err(ActionError::InvalidChoice)
    );
    let end = choose(&mut engine, "finish").unwrap();
    assert_eq!(end.ending.unwrap().id, "finished");
    assert!(end.offers.is_empty());
    assert_eq!(
        choose(&mut engine, "finish"),
        Err(ActionError::InvalidChoice)
    );
}

#[test]
fn ordered_effects_and_negative_addition_are_deterministic() {
    let mut definition = story();
    definition.nodes[0].choices[0].effects = vec![
        Effect::Set {
            variable: "count".into(),
            value: Value::Int(10),
        },
        Effect::Add {
            variable: "count".into(),
            amount: -2,
        },
        Effect::Add {
            variable: "count".into(),
            amount: 3,
        },
    ];
    definition.nodes[1].choices[0].condition = None;
    let mut a = Engine::new(definition.clone()).unwrap();
    let mut b = Engine::new(definition).unwrap();
    assert_eq!(choose(&mut a, "step"), choose(&mut b, "step"));
    assert_eq!(a.save(), b.save());
    assert_eq!(a.save().variables["count"], Value::Int(11));
}

#[test]
fn overflow_and_deadlock_leave_the_whole_state_unchanged() {
    let mut definition = story();
    definition.nodes[0].choices[0].effects.extend([
        Effect::Add {
            variable: "count".into(),
            amount: i64::MAX,
        },
        Effect::Set {
            variable: "word".into(),
            value: Value::Text("never".into()),
        },
    ]);
    let mut engine = Engine::new(definition).unwrap();
    let before = engine.save();
    assert_eq!(
        engine
            .output()
            .offers
            .iter()
            .map(|offer| offer.id.as_str())
            .collect::<Vec<_>>(),
        ["direct"]
    );
    assert_eq!(
        choose(&mut engine, "step"),
        Err(ActionError::EffectOverflow {
            variable: "count".into()
        })
    );
    assert_eq!(engine.save(), before);
    let mut definition = story();
    definition.nodes[0].choices[0].effects.clear();
    let mut engine = Engine::new(definition).unwrap();
    let before = engine.save();
    assert_eq!(
        engine
            .output()
            .offers
            .iter()
            .map(|offer| offer.id.as_str())
            .collect::<Vec<_>>(),
        ["direct"]
    );
    assert_eq!(
        choose(&mut engine, "step"),
        Err(ActionError::NoOffers {
            node: "gate".into()
        })
    );
    assert_eq!(engine.save(), before);
}

#[test]
fn every_offer_can_be_applied_in_its_unchanged_state() {
    let mut deadlock = story();
    // Ordered effects close the destination's gate after initially opening it.
    deadlock.nodes[0].choices[0].effects.push(Effect::Set {
        variable: "flag".into(),
        value: Value::Bool(false),
    });
    let mut overflow = story();
    overflow.nodes[0].choices[0].effects.push(Effect::Add {
        variable: "count".into(),
        amount: i64::MAX,
    });
    for definition in [story(), deadlock, overflow] {
        let engine = Engine::new(definition.clone()).unwrap();
        let before = engine.save();
        let output = engine.output();
        assert!(!output.offers.is_empty());
        assert_eq!(engine.output(), output);
        assert_eq!(engine.save(), before, "previews must not apply effects");
        for offer in output.offers {
            let mut candidate = Engine::new(definition.clone()).unwrap();
            let result = candidate.apply(&GameAction {
                revision: output.revision,
                choice_id: offer.id,
            });
            assert!(result.is_ok(), "offered transition rejected: {result:?}");
        }
    }
}

#[test]
fn initialization_and_restore_reject_states_with_only_unexecutable_choices() {
    for overflow in [false, true] {
        let mut definition = story();
        if overflow {
            definition.nodes[0].choices[0].effects.push(Effect::Add {
                variable: "count".into(),
                amount: i64::MAX,
            });
        } else {
            definition.nodes[0].choices[0].effects.clear();
        }
        let mut engine = Engine::new(definition.clone()).unwrap();
        let before = engine.save();
        let mut save = before.clone();
        save.variables.insert("count".into(), Value::Int(1));
        let error = engine.restore(save).unwrap_err().to_string();
        assert!(
            error.contains("node 'start': no executable choices"),
            "{error}"
        );
        assert!(error.contains("choice 'step'"), "{error}");
        assert!(
            error.contains(if overflow {
                "overflows variable 'count'"
            } else {
                "node 'gate'"
            }),
            "{error}"
        );
        assert_eq!(engine.save(), before);
        definition.variables.insert("count".into(), Value::Int(1));
        assert!(
            Engine::new(definition)
                .unwrap_err()
                .to_string()
                .contains("no executable choices")
        );
    }
}

#[test]
fn conditions_hide_choices_and_engine_rejects_forged_hidden_ids() {
    let mut definition = story();
    definition.nodes[0].choices[1].condition = Some(Condition::Any {
        conditions: vec![
            Condition::AtLeast {
                variable: "count".into(),
                value: 1,
            },
            Condition::Not {
                condition: Box::new(Condition::AtMost {
                    variable: "count".into(),
                    value: 0,
                }),
            },
        ],
    });
    let mut engine = Engine::new(definition).unwrap();
    assert_eq!(
        engine
            .output()
            .offers
            .iter()
            .map(|offer| offer.id.as_str())
            .collect::<Vec<_>>(),
        ["step"]
    );
    let before = engine.save();
    assert_eq!(
        choose(&mut engine, "direct"),
        Err(ActionError::InvalidChoice)
    );
    assert_eq!(engine.save(), before);
    assert_eq!(
        engine.apply(&GameAction {
            revision: 8,
            choice_id: "step".into()
        }),
        Err(ActionError::StaleRevision)
    );
    assert_eq!(engine.save(), before);
}

#[test]
fn boolean_combinations_and_integer_boundaries() {
    let cases = [
        (Condition::All { conditions: vec![] }, true),
        (Condition::Any { conditions: vec![] }, false),
        (
            Condition::AtLeast {
                variable: "count".into(),
                value: 0,
            },
            true,
        ),
        (
            Condition::AtMost {
                variable: "count".into(),
                value: -1,
            },
            false,
        ),
        (
            Condition::Not {
                condition: Box::new(Condition::Equals {
                    variable: "flag".into(),
                    value: Value::Bool(false),
                }),
            },
            false,
        ),
        (
            Condition::Any {
                conditions: vec![Condition::Equals {
                    variable: "word".into(),
                    value: Value::Text("unmade".into()),
                }],
            },
            true,
        ),
    ];
    for (condition, expected) in cases {
        let mut definition = story();
        definition.nodes[0].choices[1].condition = Some(condition);
        let engine = Engine::new(definition).unwrap();
        assert_eq!(
            engine
                .output()
                .offers
                .iter()
                .any(|offer| offer.id == "direct"),
            expected
        );
    }
}

#[test]
fn restore_reproduces_output_and_continuation_including_endings() {
    let definition = story();
    let mut uninterrupted = Engine::new(definition.clone()).unwrap();
    choose(&mut uninterrupted, "step").unwrap();
    let save = uninterrupted.save();
    let mut restored = Engine::new(definition).unwrap();
    assert_eq!(restored.restore(save).unwrap(), uninterrupted.output());
    assert_eq!(
        choose(&mut restored, "finish"),
        choose(&mut uninterrupted, "finish")
    );
    let ending = restored.save();
    let mut fresh = Engine::new(story()).unwrap();
    assert_eq!(fresh.restore(ending).unwrap(), restored.output());
    assert!(fresh.output().ending.is_some());
}

#[test]
fn invalid_checkpoints_preserve_the_active_session() {
    let mut engine = Engine::new(story()).unwrap();
    choose(&mut engine, "step").unwrap();
    let valid = engine.save();
    let mut invalid = Vec::new();
    let mut save = valid.clone();
    save.schema_version = 99;
    invalid.push((save, "schema"));
    let mut save = valid.clone();
    save.story_id = "other".into();
    invalid.push((save, "belongs"));
    let mut save = valid.clone();
    save.story_version = 2;
    invalid.push((save, "version"));
    let mut save = valid.clone();
    save.node_id = "missing".into();
    invalid.push((save, "node"));
    let mut save = valid.clone();
    save.variables.remove("word");
    invalid.push((save, "variables"));
    let mut save = valid.clone();
    save.variables.insert("extra".into(), Value::Int(0));
    invalid.push((save, "variables"));
    let mut save = valid.clone();
    save.variables
        .insert("flag".into(), Value::Text("true".into()));
    invalid.push((save, "flag"));
    let mut save = valid.clone();
    save.variables.insert("flag".into(), Value::Bool(false));
    invalid.push((save, "no eligible"));
    for (save, expected) in invalid {
        assert!(
            engine
                .restore(save)
                .unwrap_err()
                .to_string()
                .contains(expected)
        );
        assert_eq!(engine.save(), valid);
    }
}

#[test]
fn revision_overflow_does_not_commit_effects() {
    let mut engine = Engine::new(story()).unwrap();
    let mut save = engine.save();
    save.revision = u64::MAX;
    engine.restore(save.clone()).unwrap();
    assert!(engine.output().offers.is_empty());
    assert_eq!(
        choose(&mut engine, "step"),
        Err(ActionError::RevisionOverflow)
    );
    assert_eq!(engine.save(), save);
}

#[test]
fn restoring_older_checkpoints_never_reactivates_stale_actions() {
    let mut engine = Engine::new(story()).unwrap();
    let initial = engine.save();
    choose(&mut engine, "step").unwrap();
    let gate = engine.save();
    let old_action = GameAction {
        revision: engine.output().revision,
        choice_id: "finish".into(),
    };
    engine.apply(&old_action).unwrap();
    let previous_revision = engine.output().revision;
    let restored = engine.restore(gate).unwrap();
    assert!(restored.revision > previous_revision);
    assert_eq!(engine.apply(&old_action), Err(ActionError::StaleRevision));
    assert_eq!(engine.save().node_id, "gate");
    choose(&mut engine, "finish").unwrap();
    let mut exhausted = engine.save();
    exhausted.revision = u64::MAX;
    engine.restore(exhausted.clone()).unwrap();
    assert!(
        engine
            .restore(initial)
            .unwrap_err()
            .to_string()
            .contains("revision exhausted")
    );
    assert_eq!(engine.save(), exhausted);
}

#[test]
fn rejects_invalid_graphs_and_typed_expressions_with_context() {
    let invalid: Vec<(Story, &str)> = vec![
        (
            {
                let mut s = story();
                s.schema_version = 99;
                s
            },
            "schema version",
        ),
        (
            {
                let mut s = story();
                s.version = 0;
                s
            },
            "version",
        ),
        (
            {
                let mut s = story();
                s.title.clear();
                s
            },
            "title",
        ),
        (
            {
                let mut s = story();
                s.start = "missing".into();
                s
            },
            "start node 'missing'",
        ),
        (
            {
                let mut s = story();
                s.nodes.push(s.nodes[0].clone());
                s
            },
            "duplicate node 'start'",
        ),
        (
            {
                let mut s = story();
                let duplicate = s.nodes[0].choices[0].clone();
                s.nodes[0].choices.push(duplicate);
                s
            },
            "duplicate choice",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].target = "missing".into();
                s
            },
            "choice 'step': target 'missing'",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].content.clear();
                s
            },
            "content",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].content = narration("  ");
                s
            },
            "narration",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].content = vec![NarrativeContent::Dialogue {
                    speaker: "".into(),
                    text: "Text".into(),
                }];
                s
            },
            "speaker",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices.clear();
                s
            },
            "not an ending",
        ),
        (
            {
                let mut s = story();
                s.nodes[2].choices = s.nodes[0].choices.clone();
                s
            },
            "ending node",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].condition = Some(Condition::Equals {
                    variable: "missing".into(),
                    value: Value::Bool(true),
                });
                s
            },
            "node 'start', choice 'step': undeclared variable 'missing'",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].condition = Some(Condition::Equals {
                    variable: "count".into(),
                    value: Value::Bool(true),
                });
                s
            },
            "wrong type",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].condition = Some(Condition::AtLeast {
                    variable: "flag".into(),
                    value: 1,
                });
                s
            },
            "integer",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].effects = vec![Effect::Set {
                    variable: "missing".into(),
                    value: Value::Int(1),
                }];
                s
            },
            "undeclared variable",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].effects = vec![Effect::Add {
                    variable: "flag".into(),
                    amount: 1,
                }];
                s
            },
            "wrong type",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].effects = vec![Effect::Set {
                    variable: "flag".into(),
                    value: Value::Text("true".into()),
                }];
                s
            },
            "wrong type",
        ),
        (
            {
                let mut s = story();
                let mut node = s.nodes[2].clone();
                node.id = "unreachable".into();
                node.ending.as_mut().unwrap().id = "other".into();
                s.nodes.push(node);
                s
            },
            "unreachable",
        ),
        (
            {
                let mut s = story();
                s.nodes[0].choices[0].condition = Some(Condition::Any { conditions: vec![] });
                s.nodes[0].choices[1].condition = Some(Condition::Any { conditions: vec![] });
                s
            },
            "no eligible",
        ),
    ];
    for (definition, expected) in invalid {
        let error = Engine::new(definition).unwrap_err().to_string();
        assert!(
            error.contains(expected),
            "expected {expected:?}, got {error:?}"
        );
    }
}

#[test]
fn cycles_need_an_exit_but_are_otherwise_playable() {
    let mut definition = story();
    definition.nodes.remove(1);
    definition.nodes[0].choices[0].target = "start".into();
    let mut engine = Engine::new(definition.clone()).unwrap();
    choose(&mut engine, "step").unwrap();
    assert_eq!(engine.save().node_id, "start");
    assert_eq!(engine.save().revision, 1);
    definition.nodes.pop();
    definition.nodes[0].choices.pop();
    assert!(
        Engine::new(definition)
            .unwrap_err()
            .to_string()
            .contains("no structural path")
    );
}

#[test]
fn rejects_duplicate_endings_blank_labels_and_excessive_condition_nesting() {
    let mut definition = story();
    let mut extra = definition.nodes[2].clone();
    extra.id = "other".into();
    definition.nodes.push(extra);
    assert!(
        Engine::new(definition)
            .unwrap_err()
            .to_string()
            .contains("duplicate ending")
    );
    let mut definition = story();
    definition.nodes[0].choices[0].label = " ".into();
    assert!(
        Engine::new(definition)
            .unwrap_err()
            .to_string()
            .contains("label")
    );
    let mut definition = story();
    let mut condition = Condition::All { conditions: vec![] };
    for _ in 0..66 {
        condition = Condition::Not {
            condition: Box::new(condition),
        };
    }
    definition.nodes[0].choices[0].condition = Some(condition);
    assert!(
        Engine::new(definition)
            .unwrap_err()
            .to_string()
            .contains("64 levels")
    );
}
