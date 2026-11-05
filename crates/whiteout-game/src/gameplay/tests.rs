use super::*;
use crate::{
    game,
    scheduler::{self, CueInstance, HostRequest, Runtime, SceneView},
};
use grimm_graph::{GraphEngine, StoryGraph};

fn authored() -> StoryGraph {
    serde_yaml_ng::from_str(include_str!("../../assets/bootstrap.yaml")).unwrap()
}
fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, bevy::input::InputPlugin, game::install));
    scheduler::tests::advance(&mut app, 0);
    app
}
fn select(app: &mut App, edge: &str) {
    let r = app.world().resource::<Runtime>();
    let request = HostRequest::Selection {
        edge_id: edge.into(),
        epoch: r.sheet.epoch,
        observed_ms: r.elapsed_ms,
    };
    app.world_mut().write_message(request);
    scheduler::tests::advance(app, 0);
}
fn warning(app: &mut App) {
    select(app, "inspect_panel");
    scheduler::tests::advance(app, 5000);
}
fn pickup(app: &mut App) {
    select(app, "take_spare_part");
    scheduler::tests::advance(app, 900);
    assert!(app.world().resource::<Generator>().has_spare_part);
}

#[test]
fn missing_part_locks_repair_and_live_ecs_changes_recalculate_it() {
    let mut app = app();
    warning(&mut app);
    let repair = &app.world().resource::<Runtime>().sheet.outgoing_edges["repair_generator"];
    assert_eq!(
        blocked_reason(repair, app.world().resource::<Generator>()).unwrap(),
        Some("requires a spare part")
    );
    select(&mut app, "repair_generator");
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_WARNING"
    );
    app.world_mut().resource_mut::<Generator>().has_spare_part = true;
    let repair = &app.world().resource::<Runtime>().sheet.outgoing_edges["repair_generator"];
    assert!(
        blocked_reason(repair, app.world().resource::<Generator>())
            .unwrap()
            .is_none()
    );
    select(&mut app, "repair_generator");
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_REPAIR"
    );
    // Accepting an edge does not apply its future effect.
    let state = app.world().resource::<Generator>();
    assert_eq!(state.status, GeneratorStatus::Unstable);
    assert!(state.has_spare_part);
}

#[test]
fn pickup_then_timed_repair_consumes_the_part_exactly_once_and_reaches_powered_outcome() {
    let mut app = app();
    select(&mut app, "take_spare_part");
    scheduler::tests::advance(&mut app, 599);
    assert!(!app.world().resource::<Generator>().has_spare_part);
    scheduler::tests::advance(&mut app, 1);
    assert!(app.world().resource::<Generator>().has_spare_part);
    let take = CueInstance {
        epoch: app.world().resource::<Runtime>().sheet.epoch,
        cue_id: "take_part".into(),
    };
    app.world_mut().write_message(HostRequest::WakeCue(take));
    scheduler::tests::advance(&mut app, 300);
    assert_eq!(app.world().resource::<Generator>().applications, 1);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_ROOM"
    );
    warning(&mut app);
    select(&mut app, "repair_generator");
    let repair = CueInstance {
        epoch: app.world().resource::<Runtime>().sheet.epoch,
        cue_id: "repair".into(),
    };
    scheduler::tests::advance(&mut app, 1199);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Unstable
    );
    assert!(app.world().resource::<Generator>().has_spare_part);
    scheduler::tests::advance(&mut app, 1);
    let state = app.world().resource::<Generator>();
    assert_eq!(state.status, GeneratorStatus::Repaired);
    assert!(!state.has_spare_part);
    assert_eq!(state.applications, 3); // take, warning lamp, repair
    app.world_mut()
        .write_message(HostRequest::WakeCue(repair.clone()));
    scheduler::tests::advance(&mut app, 300);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_REPAIRED"
    );
    app.world_mut().write_message(HostRequest::WakeCue(repair));
    scheduler::tests::advance(&mut app, 30000);
    assert_eq!(app.world().resource::<Generator>().applications, 3);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_REPAIRED"
    );
    assert!(app.world().resource::<SceneView>().failure.is_none());
    select(&mut app, "restart_demo");
    scheduler::tests::advance(&mut app, 1);
    let state = app.world().resource::<Generator>();
    assert_eq!(state.status, GeneratorStatus::Running);
    assert!(!state.has_spare_part);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_ROOM"
    );
}

#[test]
fn no_part_allows_isolation_or_authored_timeout_as_distinct_outcomes() {
    let mut safe = app();
    warning(&mut safe);
    select(&mut safe, "isolate_generator");
    scheduler::tests::advance(&mut safe, 1500);
    assert_eq!(
        safe.world().resource::<Generator>().status,
        GeneratorStatus::Isolated
    );
    assert_eq!(
        safe.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_DECISION"
    );
    let mut expired = app();
    warning(&mut expired);
    let sheet = &expired.world().resource::<Runtime>().sheet;
    assert!(!sheet.outgoing_edges.contains_key("generator_exploded"));
    scheduler::tests::advance(&mut expired, 25000);
    assert_eq!(
        expired.world().resource::<Generator>().status,
        GeneratorStatus::Stopped
    );
    assert_eq!(
        expired.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_FAILED"
    );
}

#[test]
fn stale_visible_offer_is_checked_again_before_traversal() {
    let mut app = app();
    pickup(&mut app);
    warning(&mut app);
    let runtime = app.world().resource::<Runtime>();
    assert!(
        blocked_reason(
            &runtime.sheet.outgoing_edges["repair_generator"],
            app.world().resource::<Generator>()
        )
        .unwrap()
        .is_none()
    );
    let selection = HostRequest::Selection {
        edge_id: "repair_generator".into(),
        epoch: runtime.sheet.epoch,
        observed_ms: runtime.elapsed_ms,
    };
    app.world_mut().write_message(selection);
    // A legitimate game-system change after display/selection but before arbitration.
    app.world_mut().resource_mut::<Generator>().has_spare_part = false;
    scheduler::tests::advance(&mut app, 0);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_WARNING"
    );
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Unstable
    );
}

#[test]
fn unknown_conditions_and_bad_bindings_fail_validation_and_runtime_explicitly() {
    for (id, args) in [
        ("unknown_condition", Parameters::new()),
        (
            "can_repair_generator",
            [("required_item".into(), Value::Int(1))].into(),
        ),
        (
            "can_repair_generator",
            [("required_item".into(), Value::Text("unknown_item".into()))].into(),
        ),
        (
            "can_take_spare_part",
            [("extra".into(), Value::Bool(true))].into(),
        ),
    ] {
        let mut graph = authored();
        graph.nodes[1]
            .edges
            .get_mut("repair_generator")
            .unwrap()
            .condition = Some(ConditionCall {
            condition_id: id.into(),
            args,
        });
        assert!(game::validate_bindings(&graph).is_err());
        let mut app = app();
        app.insert_resource(Runtime::new(GraphEngine::new(graph).unwrap()));
        // The fixture bypasses startup validation to exercise the runtime guard.
        scheduler::tests::advance(&mut app, 0);
        warning(&mut app);
        select(&mut app, "repair_generator");
        assert_eq!(
            app.world().resource::<Runtime>().sheet.node_id,
            "GENERATOR_WARNING"
        );
        assert!(app.world().resource::<SceneView>().failure.is_some());
    }
}

#[test]
fn repair_failure_preflights_all_fields_and_blocks_dependent_cues() {
    let mut app = app();
    pickup(&mut app);
    warning(&mut app);
    select(&mut app, "repair_generator");
    app.world_mut().resource_mut::<Generator>().has_spare_part = false;
    let before = app.world().resource::<Generator>().applications;
    scheduler::tests::advance(&mut app, 1500);
    let state = app.world().resource::<Generator>();
    assert_eq!(state.status, GeneratorStatus::Unstable);
    assert!(!state.has_spare_part);
    assert_eq!(state.applications, before);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_REPAIR"
    );
    assert!(
        app.world()
            .resource::<SceneView>()
            .failure
            .as_ref()
            .unwrap()
            .contains("requires a spare part")
    );
}

#[test]
fn typed_effects_reject_bad_arguments_and_overflow_without_partial_writes() {
    let mut state = Generator {
        status: GeneratorStatus::Unstable,
        has_spare_part: true,
        applications: 4,
    };
    for (id, args) in [
        ("unknown", Parameters::new()),
        (
            "repair_generator",
            [("required_item".into(), Value::Bool(true))].into(),
        ),
        (
            "take_item",
            [
                ("item".into(), Value::Text("spare_part".into())),
                ("extra".into(), Value::Int(1)),
            ]
            .into(),
        ),
        ("reset_demo", [("x".into(), Value::Int(1))].into()),
    ] {
        assert!(state.apply(id, &args).is_err());
        assert_eq!(state.status, GeneratorStatus::Unstable);
        assert!(state.has_spare_part);
        assert_eq!(state.applications, 4);
    }
    state.applications = u64::MAX;
    assert!(
        state
            .apply(
                "repair_generator",
                &[("required_item".into(), Value::Text("spare_part".into()))].into()
            )
            .is_err()
    );
    assert_eq!(state.status, GeneratorStatus::Unstable);
    assert!(state.has_spare_part);
    assert_eq!(state.applications, u64::MAX);
}

#[test]
fn manifest_signatures_and_domain_values_match_the_rust_bindings_and_authored_references() {
    let manifest: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(include_str!("../../assets/handlers.yaml")).unwrap();
    assert_eq!(manifest["schema_version"].as_u64(), Some(1));
    fn parses(group: &str, id: &str, args: Parameters) -> bool {
        match group {
            "conditions" => Condition::parse(&ConditionCall {
                condition_id: id.into(),
                args,
            })
            .is_ok(),
            "effects" => Effect::parse(id, &args).is_ok(),
            _ => panic!("unknown manifest group"),
        }
    }
    for group in ["conditions", "effects"] {
        let bindings = manifest[group].as_mapping().unwrap();
        let mut ids: Vec<_> = bindings.keys().map(|id| id.as_str().unwrap()).collect();
        ids.sort();
        assert_eq!(
            ids,
            if group == "conditions" {
                vec!["can_repair_generator", "can_take_spare_part"]
            } else {
                vec![
                    "repair_generator",
                    "reset_demo",
                    "set_generator_status",
                    "take_item",
                ]
            }
        );
        for (id, spec) in bindings {
            let id = id.as_str().unwrap();
            let argspec = spec["args"].as_mapping().unwrap();
            let mut args = Parameters::new();
            for (name, ty) in argspec {
                let domain = manifest["types"][ty.as_str().unwrap()]
                    .as_sequence()
                    .unwrap();
                args.insert(
                    name.as_str().unwrap().into(),
                    Value::Text(domain[0].as_str().unwrap().into()),
                );
            }
            assert!(parses(group, id, args.clone()), "{group}/{id}");
            let mut extra = args.clone();
            extra.insert("extra".into(), Value::Bool(true));
            assert!(!parses(group, id, extra));
            for (name, ty) in argspec {
                let name = name.as_str().unwrap();
                for value in manifest["types"][ty.as_str().unwrap()]
                    .as_sequence()
                    .unwrap()
                {
                    let mut valid = args.clone();
                    valid.insert(name.into(), Value::Text(value.as_str().unwrap().into()));
                    assert!(parses(group, id, valid));
                }
                let mut missing = args.clone();
                missing.remove(name);
                assert!(!parses(group, id, missing));
                for wrong in [
                    Value::Bool(true),
                    Value::Int(7),
                    Value::Text("unknown_domain_value".into()),
                ] {
                    let mut invalid = args.clone();
                    invalid.insert(name.into(), wrong);
                    assert!(!parses(group, id, invalid));
                }
            }
        }
    }
    let graph = authored();
    game::validate_bindings(&graph).unwrap();
    for node in graph.nodes {
        for edge in node.edges.values() {
            if let Some(call) = &edge.condition {
                assert!(
                    manifest["conditions"]
                        .as_mapping()
                        .unwrap()
                        .contains_key(call.condition_id.as_str())
                );
            }
        }
        for cue in node.cues {
            if let grimm_graph::Instruction::Effect { effect_id, .. } = cue.instruction {
                assert!(
                    manifest["effects"]
                        .as_mapping()
                        .unwrap()
                        .contains_key(effect_id.as_str())
                );
            }
        }
    }
}

#[test]
fn scheduled_world_change_is_seen_by_a_selection_in_the_same_arbitration_pass() {
    let mut graph = authored();
    graph.nodes[1].cues.push(grimm_graph::Cue {
        cue_id: "part_delivery".into(),
        at_ms: 5000,
        instruction: grimm_graph::Instruction::Effect {
            effect_id: "take_item".into(),
            args: [("item".into(), Value::Text("spare_part".into()))].into(),
        },
    });
    let mut app = app();
    app.insert_resource(Runtime::new(GraphEngine::new(graph).unwrap()));
    scheduler::tests::advance(&mut app, 0);
    select(&mut app, "inspect_panel");
    let epoch = app.world().resource::<Runtime>().sheet.epoch;
    app.world_mut().write_message(HostRequest::Selection {
        edge_id: "repair_generator".into(),
        epoch,
        observed_ms: 5000,
    });
    scheduler::tests::advance(&mut app, 5000);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_REPAIR"
    );
    assert!(app.world().resource::<Generator>().has_spare_part); // future repair has not run
}
