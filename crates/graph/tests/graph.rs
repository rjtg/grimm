//! The only engine imported is grimm-graph. No world provider, rules interpreter,
//! presentation player, timer, or ACK coordination is required by these tests.
use grimm_graph::*;
const SOURCE: &str = include_str!("../../../examples/graph-foundation/locked-in.yaml");
fn story() -> StoryGraph {
    serde_yaml_ng::from_str(SOURCE).unwrap()
}
fn new() -> GraphEngine {
    GraphEngine::new(story()).unwrap()
}

#[test]
fn default_exposure_discovers_condition_data_without_evaluation() {
    let mut graph = new();
    let edges: Vec<_> = graph.exposed_edges().collect();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].0, "break_door");
    assert!(edges[0].1.expose);
    let condition = edges[0].1.condition.as_ref().unwrap();
    assert_eq!(condition.condition_id, "can_break_door");
    assert_eq!(
        condition.args["minimum_strength"],
        grimm_semantic::story::Value::Int(3)
    );
    let mut authored = story();
    authored.nodes[0]
        .edges
        .get_mut("break_door")
        .unwrap()
        .condition = Some(ConditionCall {
        condition_id: "unregistered_and_false".into(),
        args: [("result".into(), grimm_semantic::story::Value::Bool(false))].into(),
    });
    let mut unfiltered = GraphEngine::new(authored).unwrap();
    assert_eq!(unfiltered.exposed_edges().count(), 1);
    // Host-owned conditions do not authorize/refuse traversal inside GRIMM.
    unfiltered.traverse_edge("break_door", 0).unwrap();
    graph.traverse_edge("break_door", 0).unwrap();
    assert_eq!(graph.exposed_edges().count(), 0);
    graph.traverse_edge("continue", 1).unwrap(); // hidden is still traversable
    assert_eq!(graph.current().node_id, "CORRIDOR_SCENE");
}

#[test]
fn traversal_is_current_node_scoped_atomic_and_immediately_returns_full_replace() {
    let mut graph = new();
    let initial = graph.current();
    assert!(matches!(
        graph.traverse_edge("continue", 0),
        Err(StoryError::UnknownEdge { .. })
    ));
    assert_eq!(graph.current(), initial);
    let result = graph.traverse_edge("break_door", 0).unwrap();
    assert_eq!(result.cue_sheet, graph.current());
    assert_eq!(result.cue_sheet.node_id, "BREAK_OUT_SCENE");
    assert_eq!(result.cue_sheet.epoch, 1);
    assert_eq!(result.cue_sheet.cues, story().nodes[1].cues);
    assert_eq!(
        result.path_event,
        PathEvent {
            sequence: 1,
            from_node: "LOCKED_IN_SCENE".into(),
            from_epoch: 0,
            edge_id: "break_door".into(),
            to_node: "BREAK_OUT_SCENE".into(),
            to_epoch: 1
        }
    );
    let current = graph.snapshot();
    assert!(matches!(
        graph.traverse_edge("break_door", 0),
        Err(StoryError::StaleEpoch { .. })
    ));
    assert!(matches!(
        graph.traverse_edge("break_door", 1),
        Err(StoryError::UnknownEdge { .. })
    ));
    assert_eq!(graph.snapshot(), current);
    let next = graph.traverse_edge("continue", 1).unwrap();
    assert_eq!(next.cue_sheet.cues, story().nodes[2].cues);
    assert!(!next.cue_sheet.cues.iter().any(|cue| cue.cue_id == "onward"));
    let end = graph.current();
    assert!(graph.traverse_edge("continue", 1).is_err());
    assert_eq!(graph.current(), end);
}

#[test]
fn transition_staging_and_effects_are_host_executed_on_a_fake_timeline() {
    let mut graph = new();
    let middle = graph.traverse_edge("break_door", 0).unwrap().cue_sheet;
    let mut host_door_broken = false;
    assert_eq!(middle.node_id, "BREAK_OUT_SCENE");
    assert!(!host_door_broken); // traversal applies no effect
    for at_ms in [0, 1200, 1500] {
        for cue in middle.cues.iter().filter(|c| c.at_ms == at_ms) {
            match &cue.instruction {
                Instruction::Effect { effect_id, args } => {
                    assert_eq!(effect_id, "set_door_state");
                    assert_eq!(
                        args["door"],
                        grimm_semantic::story::Value::Text("north_door".into())
                    );
                    host_door_broken = true;
                }
                Instruction::TraverseEdge { edge_id } => {
                    assert!(host_door_broken);
                    assert_eq!(at_ms, 1500);
                    graph.traverse_edge(edge_id, middle.epoch).unwrap();
                }
                Instruction::Presentation { duration_ms, .. } => {
                    assert_eq!(*duration_ms, Some(1200))
                }
            }
        }
    }
    assert_eq!(graph.current().node_id, "CORRIDOR_SCENE");
    assert_eq!(graph.current().epoch, 2);
}

#[test]
fn repeated_node_and_self_entry_get_new_epochs_and_reject_old_scheduled_traversal() {
    let mut authored = story();
    authored.nodes[1].edges.insert(
        "return".into(),
        Edge {
            target: "LOCKED_IN_SCENE".into(),
            expose: false,
            condition: None,
        },
    );
    authored.nodes[0].edges.insert(
        "again".into(),
        Edge {
            target: "LOCKED_IN_SCENE".into(),
            expose: false,
            condition: None,
        },
    );
    let mut graph = GraphEngine::new(authored).unwrap();
    graph.traverse_edge("again", 0).unwrap();
    assert_eq!(graph.current().node_id, "LOCKED_IN_SCENE");
    assert_eq!(graph.current().epoch, 1);
    let old = graph.traverse_edge("break_door", 1).unwrap().cue_sheet;
    graph.traverse_edge("return", old.epoch).unwrap();
    graph.traverse_edge("break_door", 3).unwrap();
    let current = graph.current();
    assert_eq!(current.node_id, old.node_id);
    assert!(matches!(
        graph.traverse_edge("continue", old.epoch),
        Err(StoryError::StaleEpoch { .. })
    ));
    assert_eq!(graph.current(), current);
    graph.traverse_edge("continue", current.epoch).unwrap();
}

#[test]
fn continuation_round_trip_restores_position_without_traversal_or_effect_replay() {
    let mut graph = new();
    graph.traverse_edge("break_door", 0).unwrap();
    let saved = graph.snapshot();
    let json = serde_json::to_string(&saved).unwrap();
    let saved: StoryContinuation = serde_json::from_str(&json).unwrap();
    graph.traverse_edge("continue", 1).unwrap();
    let restored = graph.restore(saved.clone()).unwrap();
    assert_eq!(restored.node_id, "BREAK_OUT_SCENE");
    assert_eq!(restored.epoch, 3);
    assert_eq!(restored.cues, story().nodes[1].cues);
    assert!(graph.traverse_edge("continue", saved.epoch).is_err());
    graph.traverse_edge("continue", restored.epoch).unwrap();
    let mut fresh = new();
    assert_eq!(fresh.restore(saved).unwrap().epoch, 2);
    for field in ["schema_version", "story_version", "story_id", "node_id"] {
        let before = graph.snapshot();
        let mut invalid = serde_json::to_value(&before).unwrap();
        invalid[field] = if field.ends_with("id") {
            serde_json::json!("missing")
        } else {
            serde_json::json!(99)
        };
        assert!(
            graph
                .restore(serde_json::from_value(invalid).unwrap())
                .is_err()
        );
        assert_eq!(graph.snapshot(), before);
    }
}

#[test]
fn exhausted_epoch_never_wraps_or_partially_traverses() {
    let mut graph = new();
    let mut continuation = graph.snapshot();
    continuation.epoch = u64::MAX;
    let before = graph.current();
    assert_eq!(
        graph.restore(continuation.clone()),
        Err(StoryError::EpochExhausted)
    );
    assert_eq!(graph.current(), before);
    continuation.epoch -= 1;
    graph.restore(continuation).unwrap();
    let before = graph.current();
    assert_eq!(
        graph.traverse_edge("break_door", u64::MAX),
        Err(StoryError::EpochExhausted)
    );
    assert_eq!(graph.current(), before);
}

#[test]
fn graph_validation_rejects_ambiguous_ids_and_invalid_references() {
    let mut cases = vec![];
    let mut invalid = story();
    invalid.nodes.push(invalid.nodes[0].clone());
    cases.push(invalid);
    let mut invalid = story();
    invalid.start = "missing".into();
    cases.push(invalid);
    let mut invalid = story();
    invalid.nodes[0].edges.get_mut("break_door").unwrap().target = "missing".into();
    cases.push(invalid);
    let mut invalid = story();
    invalid.nodes[1].cues[2].instruction = Instruction::TraverseEdge {
        edge_id: "missing".into(),
    };
    cases.push(invalid);
    let mut invalid = story();
    invalid.nodes[1].cues[2].instruction = Instruction::TraverseEdge {
        edge_id: "break_door".into(),
    };
    cases.push(invalid); // exists only on another node
    let mut invalid = story();
    let duplicate = invalid.nodes[1].cues[0].clone();
    invalid.nodes[1].cues.push(duplicate);
    cases.push(invalid);
    let mut invalid = story();
    invalid.nodes[0].id = "with spaces".into();
    cases.push(invalid);
    let mut invalid = story();
    invalid.nodes[0]
        .edges
        .get_mut("break_door")
        .unwrap()
        .condition
        .as_mut()
        .unwrap()
        .condition_id
        .clear();
    cases.push(invalid);
    let mut invalid = story();
    invalid.schema_version = 99;
    cases.push(invalid);
    let mut invalid = story();
    invalid.version = 0;
    cases.push(invalid);
    for invalid in cases {
        assert!(matches!(
            GraphEngine::new(invalid),
            Err(StoryError::InvalidDefinition(_))
        ));
    }
}

#[test]
fn serialized_contract_is_strict_and_preserves_order_times_and_descriptor_values() {
    let authored = story();
    let json = serde_json::to_string(&authored).unwrap();
    assert_eq!(serde_json::from_str::<StoryGraph>(&json).unwrap(), authored);
    let yaml_duplicate = SOURCE.replace(
        "    break_door:\n",
        "    break_door: {target: BREAK_OUT_SCENE}\n    break_door:\n",
    );
    assert!(serde_yaml_ng::from_str::<StoryGraph>(&yaml_duplicate).is_err());
    let json_duplicate = r#"{"target":"A","expose":true}"#;
    let duplicate = format!(
        r#"{{"id":"N","cues":[],"edges":{{"go":{json_duplicate},"go":{json_duplicate}}}}}"#
    );
    assert!(serde_json::from_str::<Node>(&duplicate).is_err());
    assert!(
        serde_yaml_ng::from_str::<StoryGraph>(&SOURCE.replace("at_ms: 1500", "at_ms: -1")).is_err()
    );
    assert!(
        serde_yaml_ng::from_str::<StoryGraph>(
            &SOURCE.replace("kind: effect", "kind: execute_rust")
        )
        .is_err()
    );
    assert!(
        serde_yaml_ng::from_str::<StoryGraph>(&format!("{SOURCE}\nworld_state: {{}}\n")).is_err()
    );
    // Time is declaration data. No sorting, inferred readiness, or current clock.
    let mut unusual = authored;
    unusual.nodes[0].cues.push(Cue {
        cue_id: "overlap".into(),
        at_ms: 10,
        instruction: Instruction::Presentation {
            presentation_id: "sound".into(),
            duration_ms: None,
            args: Parameters::new(),
        },
    });
    unusual.nodes[0].cues.push(Cue {
        cue_id: "parallel".into(),
        at_ms: 0,
        instruction: Instruction::Effect {
            effect_id: "unregistered".into(),
            args: Parameters::new(),
        },
    });
    let expected = unusual.nodes[0].cues.clone();
    assert_eq!(GraphEngine::new(unusual).unwrap().current().cues, expected);
}

#[test]
fn host_deadline_arbitration_uses_ordinary_edges_and_cancels_old_sheet_work() {
    for selected_ms in [29999, 30000, 30001] {
        let mut authored = story();
        authored.nodes[0].edges.insert(
            "timeout".into(),
            Edge {
                target: "CORRIDOR_SCENE".into(),
                expose: false,
                condition: None,
            },
        );
        authored.nodes[0].cues.push(Cue {
            cue_id: "deadline".into(),
            at_ms: 30000,
            instruction: Instruction::TraverseEdge {
                edge_id: "timeout".into(),
            },
        });
        let mut graph = GraphEngine::new(authored).unwrap();
        let old = graph.current();
        // This tie rule belongs to the host, not graph traversal. The graph has
        // no selection timestamp or physical clock from which to infer it.
        let chosen = if selected_ms >= 30000 {
            "timeout"
        } else {
            "break_door"
        };
        let result = graph.traverse_edge(chosen, old.epoch).unwrap();
        assert_eq!(
            result.cue_sheet.node_id,
            if selected_ms >= 30000 {
                "CORRIDOR_SCENE"
            } else {
                "BREAK_OUT_SCENE"
            }
        );
        let obsolete = if chosen == "timeout" {
            "break_door"
        } else {
            "timeout"
        };
        assert!(matches!(
            graph.traverse_edge(obsolete, old.epoch),
            Err(StoryError::StaleEpoch { .. })
        ));
        assert_eq!(graph.current(), result.cue_sheet);
    }
}

#[test]
fn host_drops_delayed_effect_after_replacement_without_sending_cue_feedback() {
    let mut graph = new();
    let old = graph.traverse_edge("break_door", 0).unwrap().cue_sheet;
    let effect = old
        .cues
        .iter()
        .find(|c| matches!(c.instruction, Instruction::Effect { .. }))
        .unwrap();
    assert_eq!(effect.at_ms, 1200);
    // A host-native event leaves before that scheduled effect. Timing/effect
    // success are not checked by the graph; the host must cancel obsolete work.
    graph.traverse_edge("continue", old.epoch).unwrap();
    let mut host_door_broken = false;
    if old.epoch == graph.current().epoch {
        host_door_broken = true;
    }
    assert!(!host_door_broken);
    assert!(graph.traverse_edge("continue", old.epoch).is_err());
}
