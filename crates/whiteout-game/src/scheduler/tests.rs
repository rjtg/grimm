use super::*;
use bevy::{
    input::{
        ButtonState, InputPlugin,
        keyboard::{Key, KeyboardInput},
    },
    time::TimeUpdateStrategy,
};
use grimm_graph::{Cue, StoryGraph};

fn authored() -> StoryGraph {
    serde_yaml_ng::from_str(include_str!("fixtures/scheduler-v2.yaml")).unwrap()
}

pub(crate) fn app() -> App {
    app_with(authored())
}
fn app_with(graph: StoryGraph) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, InputPlugin, crate::game::install))
        .insert_resource(Runtime::new(GraphEngine::new(graph).unwrap()));
    advance(&mut app, 0);
    app
}

pub(crate) fn advance(app: &mut App, millis: u64) {
    app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
        millis,
    )));
    app.update();
}
pub(crate) fn key(app: &mut App, state: ButtonState, repeat: bool, key_code: KeyCode) {
    app.world_mut().write_message(KeyboardInput {
        key_code,
        logical_key: Key::Character("1".into()),
        state,
        text: None,
        repeat,
        window: Entity::PLACEHOLDER,
    });
    advance(app, 0);
}
fn select(app: &mut App, edge: &str, observed_ms: u64) {
    let epoch = app.world().resource::<Runtime>().sheet.epoch;
    app.world_mut().write_message(HostRequest::Selection {
        edge_id: edge.into(),
        epoch,
        observed_ms,
    });
}
fn enter_warning(app: &mut App) {
    key(app, ButtonState::Pressed, false, KeyCode::Digit1);
    key(app, ButtonState::Released, false, KeyCode::Digit1);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_WARNING"
    );
}
fn warning() -> App {
    let mut app = app();
    enter_warning(&mut app);
    app
}
fn instance(app: &App, cue: &str) -> CueInstance {
    CueInstance {
        epoch: app.world().resource::<Runtime>().sheet.epoch,
        cue_id: cue.into(),
    }
}
fn wake(app: &mut App, cue: CueInstance) {
    app.world_mut().write_message(HostRequest::WakeCue(cue));
}
fn effect(id: &str, at_ms: u64, status: &str) -> Cue {
    Cue {
        cue_id: id.into(),
        at_ms,
        instruction: Instruction::Effect {
            effect_id: "set_generator_status".into(),
            args: [("status".into(), Value::Text(status.into()))].into(),
        },
    }
}

#[test]
fn keyboard_uses_sheet_discovery_and_explicit_readiness_not_held_keys() {
    let mut app = warning();
    assert_eq!(
        app.world().resource::<Runtime>().sheet.outgoing_edges.len(),
        1
    );
    assert!(
        !app.world()
            .resource::<Runtime>()
            .sheet
            .outgoing_edges
            .contains_key("generator_exploded")
    );
    advance(&mut app, 4999);
    key(&mut app, ButtonState::Pressed, false, KeyCode::Digit1);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_WARNING"
    );
    advance(&mut app, 1);
    assert!(app.world().resource::<SceneView>().actions_open());
    key(&mut app, ButtonState::Pressed, true, KeyCode::Digit1);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_WARNING"
    );
    key(&mut app, ButtonState::Released, false, KeyCode::Digit1);
    key(&mut app, ButtonState::Pressed, false, KeyCode::Digit1);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_TRANSITION"
    );
    assert_eq!(app.world().resource::<Runtime>().elapsed_ms, 0);
    assert_eq!(
        app.world()
            .resource::<SceneView>()
            .presentation
            .as_ref()
            .unwrap()
            .id,
        "generator_transition"
    );
}

#[test]
fn effects_are_never_early_and_duplicate_wakes_do_not_repeat_them() {
    let mut app = warning();
    let lamp = instance(&app, "warning_lamp");
    wake(&mut app, lamp.clone());
    advance(&mut app, 1999);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Running
    );
    assert_eq!(app.world().resource::<Generator>().applications, 1); // room reset
    assert!(!app.world().resource::<Runtime>().consumed.contains(&lamp));
    advance(&mut app, 1);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Unstable
    );
    assert_eq!(app.world().resource::<Generator>().applications, 2);
    wake(&mut app, lamp.clone());
    wake(&mut app, lamp.clone());
    advance(&mut app, 0);
    assert_eq!(app.world().resource::<Generator>().applications, 2);
    assert!(app.world().resource::<Runtime>().consumed.contains(&lamp));
}

#[test]
fn skipped_frames_catch_up_all_due_cues_using_real_time_not_virtual_delta() {
    let mut app = warning();
    advance(&mut app, 31000);
    let runtime = app.world().resource::<Runtime>();
    assert_eq!(runtime.sheet.node_id, "GENERATOR_FAILED");
    assert_eq!(runtime.elapsed_ms, 0);
    assert_eq!(runtime.sheet, runtime.graph.current());
    assert_eq!(app.world().resource::<Generator>().applications, 3); // reset, lamp, stop
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Stopped
    );
    assert_eq!(
        app.world().resource::<Time<Real>>().elapsed(),
        Duration::from_secs(31)
    );
    assert!(
        app.world()
            .resource::<SceneView>()
            .presentation
            .as_ref()
            .unwrap()
            .id
            == "generator_failed"
    );
}

#[test]
fn timestamp_then_authored_order_control_simultaneous_cues() {
    let mut graph = authored();
    graph.nodes[0].cues.extend([
        effect("first_at_ten", 10, "stopped"),
        effect("second_at_ten", 10, "isolated"),
        effect("last_in_list_but_earlier", 5, "unstable"),
        effect("not_yet", 11, "running"),
    ]);
    let mut app = app_with(graph);
    advance(&mut app, 10);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Isolated
    );
    assert_eq!(app.world().resource::<Generator>().applications, 4);
    advance(&mut app, 0);
    assert_eq!(app.world().resource::<Generator>().applications, 4);
    advance(&mut app, 1);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Running
    );
}

#[test]
fn early_selection_replaces_timeout_and_transition_effects_traverse_on_the_new_clock() {
    let mut app = warning();
    let obsolete_timeout = instance(&app, "deadline");
    advance(&mut app, 5000);
    select(&mut app, "isolate_generator", 5000);
    advance(&mut app, 0);
    assert_eq!(app.world().resource::<Runtime>().elapsed_ms, 0);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Unstable
    );
    advance(&mut app, 1199);
    assert_eq!(app.world().resource::<Generator>().applications, 2);
    advance(&mut app, 1);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Isolated
    );
    advance(&mut app, 300);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_DECISION"
    );
    assert_eq!(app.world().resource::<Runtime>().elapsed_ms, 0);
    wake(&mut app, obsolete_timeout);
    advance(&mut app, 30000);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_DECISION"
    );
    assert_eq!(app.world().resource::<Generator>().applications, 3);
}

#[test]
fn timeout_wins_at_equality_independently_of_message_arrival_order() {
    for selected_ms in [29999, 30000, 30001] {
        for reverse in [false, true] {
            let mut app = warning();
            let timeout = HostRequest::WakeCue(instance(&app, "deadline"));
            let selection = HostRequest::Selection {
                edge_id: "isolate_generator".into(),
                epoch: 1,
                observed_ms: selected_ms,
            };
            let events = if reverse {
                [selection, timeout]
            } else {
                [timeout, selection]
            };
            app.world_mut().write_message_batch(events);
            advance(&mut app, 30001); // both events are due in one skipped frame
            assert_eq!(
                app.world().resource::<Runtime>().sheet.node_id,
                if selected_ms < 30000 {
                    "GENERATOR_TRANSITION"
                } else {
                    "GENERATOR_FAILED"
                }
            );
            assert_eq!(app.world().resource::<Runtime>().elapsed_ms, 0);
        }
    }
}

#[test]
fn replace_cancels_not_started_effects_and_old_callbacks_cannot_apply_them() {
    let mut graph = authored();
    graph.nodes[1]
        .cues
        .push(effect("obsolete_effect", 25000, "stopped"));
    let mut app = app_with(graph);
    enter_warning(&mut app);
    let old_effect = instance(&app, "obsolete_effect");
    advance(&mut app, 5000);
    select(&mut app, "isolate_generator", 5000);
    advance(&mut app, 0);
    advance(&mut app, 25000);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_DECISION"
    );
    wake(&mut app, old_effect);
    advance(&mut app, 0);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Isolated
    );
    assert_eq!(app.world().resource::<Generator>().applications, 3);
}

#[test]
fn reentry_reserves_new_cue_identity_and_rejects_old_valid_edge_and_effect() {
    let mut app = warning();
    let old = instance(&app, "warning_lamp");
    advance(&mut app, 5000);
    select(&mut app, "isolate_generator", 5000);
    advance(&mut app, 0);
    advance(&mut app, 1500);
    select(&mut app, "restart_demo", 0);
    advance(&mut app, 0);
    select(&mut app, "inspect_panel", 0);
    advance(&mut app, 0);
    let fresh = instance(&app, "warning_lamp");
    assert_eq!(fresh.cue_id, old.cue_id);
    assert_ne!(fresh.epoch, old.epoch);
    wake(&mut app, old.clone());
    app.world_mut().write_message(HostRequest::Selection {
        edge_id: "isolate_generator".into(),
        epoch: old.epoch,
        observed_ms: 5000,
    });
    advance(&mut app, 2000);
    let runtime = app.world().resource::<Runtime>();
    assert_eq!(runtime.sheet.epoch, fresh.epoch);
    assert!(runtime.consumed.contains(&fresh));
    assert!(!runtime.consumed.contains(&old));
    assert_eq!(app.world().resource::<Generator>().applications, 5);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Unstable
    );
}

#[test]
fn failed_effects_are_visible_not_consumed_not_retried_and_block_followup_traversal() {
    for args in [
        Parameters::new(),
        [("status".into(), Value::Int(42))].into(),
        [("status".into(), Value::Text("typo".into()))].into(),
    ] {
        let mut graph = authored();
        graph.nodes[1].cues[1].instruction = Instruction::Effect {
            effect_id: "set_generator_status".into(),
            args,
        };
        let mut app = app_with(graph);
        enter_warning(&mut app);
        let failed = instance(&app, "warning_lamp");
        select(&mut app, "isolate_generator", 5000);
        advance(&mut app, 30000);
        assert_eq!(
            app.world().resource::<Runtime>().sheet.node_id,
            "GENERATOR_WARNING"
        );
        assert_eq!(app.world().resource::<Generator>().applications, 1);
        assert_eq!(
            app.world().resource::<Generator>().status,
            GeneratorStatus::Running
        );
        assert!(!app.world().resource::<Runtime>().consumed.contains(&failed));
        assert!(
            app.world()
                .resource::<SceneView>()
                .failure
                .as_ref()
                .unwrap()
                .contains("warning_lamp")
        );
        assert!(!app.world().resource::<SceneView>().actions_open());
        wake(&mut app, failed);
        advance(&mut app, 30000);
        assert_eq!(app.world().resource::<Generator>().applications, 1);
        assert_eq!(app.world().resource::<Runtime>().sheet.epoch, 1);
    }
}

#[test]
fn effect_overflow_preflights_every_field_and_stops_the_entry() {
    let mut app = warning();
    app.world_mut().resource_mut::<Generator>().applications = u64::MAX;
    advance(&mut app, 2000);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Running
    );
    assert_eq!(app.world().resource::<Generator>().applications, u64::MAX);
    assert!(
        app.world()
            .resource::<SceneView>()
            .failure
            .as_ref()
            .unwrap()
            .contains("overflow")
    );
}

#[test]
fn presentation_duration_is_metadata_not_completion_or_an_implicit_traversal() {
    let mut graph = authored();
    let Instruction::Presentation { duration_ms, .. } = &mut graph.nodes[2].cues[0].instruction
    else {
        panic!()
    };
    *duration_ms = Some(60000); // no actual animation or ACK in this placeholder host
    let mut app = app_with(graph);
    enter_warning(&mut app);
    advance(&mut app, 5000);
    select(&mut app, "isolate_generator", 5000);
    advance(&mut app, 0);
    assert_eq!(
        app.world()
            .resource::<SceneView>()
            .presentation
            .as_ref()
            .unwrap()
            .duration_ms,
        Some(60000)
    );
    advance(&mut app, 1500);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_DECISION"
    );
}

#[test]
fn zero_time_traversals_are_bounded_without_losing_pending_cues() {
    let mut graph = authored();
    graph.nodes[0].cues.push(Cue {
        cue_id: "instant".into(),
        at_ms: 0,
        instruction: Instruction::TraverseEdge {
            edge_id: "inspect_panel".into(),
        },
    });
    graph.nodes[1].cues.push(Cue {
        cue_id: "instant".into(),
        at_ms: 0,
        instruction: Instruction::TraverseEdge {
            edge_id: "generator_exploded".into(),
        },
    });
    let mut app = app_with(graph);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_WARNING"
    );
    advance(&mut app, 0);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_FAILED"
    );
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Stopped
    );
}

#[test]
fn future_backdated_and_hidden_player_selections_do_not_bypass_host_arbitration() {
    let mut app = warning();
    advance(&mut app, 5000);
    for (edge, time) in [
        ("isolate_generator", 4999),
        ("isolate_generator", 99999),
        ("generator_exploded", 5000),
        ("unknown", 5000),
    ] {
        select(&mut app, edge, time);
        advance(&mut app, 0);
        assert_eq!(
            app.world().resource::<Runtime>().sheet.node_id,
            "GENERATOR_WARNING"
        );
    }
}

#[derive(Resource)]
struct ObservedSelection(u64);
fn racing_collector(
    time: Res<Time<Real>>,
    runtime: Res<Runtime>,
    selected: Res<ObservedSelection>,
    mut messages: MessageWriter<HostRequest>,
) {
    if runtime.sheet.node_id == "GENERATOR_WARNING" && runtime.observed_ms(time.elapsed()) >= 30000
    {
        messages.write(HostRequest::Selection {
            edge_id: "isolate_generator".into(),
            epoch: runtime.sheet.epoch,
            observed_ms: selected.0,
        });
    }
}

#[test]
fn arbitration_does_not_depend_on_the_order_of_bevy_collectors() {
    use crate::game::FrameSet;
    for observed_ms in [29999, 30000] {
        for input_first in [true, false] {
            let mut app = warning();
            app.insert_resource(ObservedSelection(observed_ms));
            if input_first {
                app.add_systems(
                    Update,
                    racing_collector
                        .before(queue_cues)
                        .in_set(FrameSet::Collect),
                );
            } else {
                app.add_systems(
                    Update,
                    racing_collector.after(queue_cues).in_set(FrameSet::Collect),
                );
            }
            advance(&mut app, 30000);
            assert_eq!(
                app.world().resource::<Runtime>().sheet.node_id,
                if observed_ms == 30000 {
                    "GENERATOR_FAILED"
                } else {
                    "GENERATOR_TRANSITION"
                }
            );
        }
    }
}

#[test]
fn destination_entry_effect_failure_stays_visible_after_the_graph_has_traversed() {
    let mut graph = authored();
    let failed = graph
        .nodes
        .iter_mut()
        .find(|node| node.id == "GENERATOR_FAILED")
        .unwrap();
    failed.cues[0].instruction = Instruction::Effect {
        effect_id: "set_generator_status".into(),
        args: Parameters::new(),
    };
    let mut app = app_with(graph);
    enter_warning(&mut app);
    advance(&mut app, 30000);
    // Graph traversal is immediate and is not a transaction with future host
    // effects. The destination failure must be explicit, never a success view.
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_FAILED"
    );
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Unstable
    );
    assert_eq!(app.world().resource::<Generator>().applications, 2);
    let view = app.world().resource::<SceneView>();
    assert!(view.failure.as_ref().unwrap().contains("stop"));
    assert!(view.presentation.is_none());
    assert!(!view.actions_open());
    let stop = instance(&app, "stop");
    assert!(!app.world().resource::<Runtime>().consumed.contains(&stop));
    wake(&mut app, stop);
    advance(&mut app, 0);
    assert_eq!(app.world().resource::<Generator>().applications, 2);
    assert!(app.world().resource::<SceneView>().presentation.is_none());
}
