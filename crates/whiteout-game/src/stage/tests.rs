use super::*;
use crate::{
    game, gameplay,
    scheduler::{self, HostRequest},
};
use bevy::{audio::Decodable, input::InputPlugin};
use std::time::Duration;

fn graph() -> grimm_graph::StoryGraph {
    serde_yaml_ng::from_str(include_str!("../../assets/bootstrap.yaml")).unwrap()
}
fn app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        InputPlugin,
        game::install,
        crate::screen::install,
        install,
    ));
    scheduler::tests::advance(&mut app, 0);
    app
}
fn advance(app: &mut App, ms: u64) {
    scheduler::tests::advance(app, ms);
}
fn select(app: &mut App, edge: &str) {
    let r = app.world().resource::<Runtime>();
    let request = HostRequest::Selection {
        edge_id: edge.into(),
        epoch: r.sheet.epoch,
        observed_ms: r.elapsed_ms,
    };
    app.world_mut().write_message(request);
    advance(app, 0);
}
fn annex(app: &mut App) {
    advance(app, 18000);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_ANNEX"
    );
}
fn entity_count<T: Component>(app: &mut App) -> usize {
    let world = app.world_mut();
    world
        .query_filtered::<Entity, With<T>>()
        .iter(world)
        .count()
}

#[test]
fn canonical_success_is_a_complete_32_second_path_with_timed_effect_and_normal_transition() {
    let mut app = app();
    annex(&mut app);
    assert!(!app.world().resource::<Generator>().deicer_isolated);
    advance(&mut app, 1999);
    assert!(!app.world().resource::<Generator>().deicer_isolated);
    advance(&mut app, 1);
    assert!(app.world().resource::<Generator>().deicer_isolated);
    advance(&mut app, 2000);
    select(&mut app, "start_generator");
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_STARTUP"
    );
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Stopped
    );
    advance(&mut app, 3999);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Stopped
    );
    advance(&mut app, 1);
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Powered
    );
    advance(&mut app, 500);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "DISPATCH_WARM"
    );
    assert!(!app.world().resource::<SceneView>().actions_open());
    advance(&mut app, 6000);
    assert!(app.world().resource::<SceneView>().actions_open());
    assert_eq!(
        app.world().resource::<Time<Real>>().elapsed(),
        Duration::from_millis(32500)
    );
    assert!(!app.world().resource::<Generator>().engineering_lost);
}
#[test]
fn abandonment_and_timeout_have_explicit_cold_consequences() {
    for timeout in [false, true] {
        let mut app = app();
        annex(&mut app);
        advance(&mut app, 4000);
        if timeout {
            advance(&mut app, 26000);
        } else {
            select(&mut app, "abandon");
        }
        assert_eq!(
            app.world().resource::<Runtime>().sheet.node_id,
            if timeout {
                "ANNEX_SEALED"
            } else {
                "DISPATCH_COLD"
            }
        );
        let g = app.world().resource::<Generator>();
        assert!(g.engineering_lost);
        assert_eq!(g.status, GeneratorStatus::Stopped);
        advance(&mut app, 9000);
        assert!(app.world().resource::<SceneView>().actions_open());
        let total = app.world().resource::<Time<Real>>().elapsed().as_millis();
        assert!((30000..=60000).contains(&total));
    }
}
#[test]
fn simultaneous_radio_audio_and_camera_do_not_overwrite_the_caption_or_restart_completed_cues() {
    let mut app = app();
    annex(&mut app);
    advance(&mut app, 10000);
    let view = app.world().resource::<SceneView>();
    assert_eq!(view.presentation.as_ref().unwrap().id, "canon.radio");
    assert!(view.active.iter().any(|p| p.id == "sound.alarm"));
    assert!(view.active.iter().any(|p| p.id == "shot.controls"));
    assert!(entity_count::<AudioPlayer>(&mut app) >= 2);
    let count = entity_count::<CueEntity>(&mut app);
    let epoch = app.world().resource::<Runtime>().sheet.epoch;
    app.world_mut()
        .write_message(HostRequest::WakeCue(CueInstance {
            epoch,
            cue_id: "alarm".into(),
        }));
    advance(&mut app, 500);
    assert_eq!(entity_count::<CueEntity>(&mut app), count);
    assert_eq!(
        app.world()
            .resource::<SceneView>()
            .presentation
            .as_ref()
            .unwrap()
            .id,
        "canon.radio"
    );
}
#[test]
fn replace_despawns_old_audio_motion_and_rejects_stale_wakes_with_bounded_assets() {
    let mut app = app();
    annex(&mut app);
    advance(&mut app, 10000);
    let epoch = app.world().resource::<Runtime>().sheet.epoch;
    let old: Vec<_> = {
        let w = app.world_mut();
        w.query_filtered::<Entity, With<CueEntity>>()
            .iter(w)
            .collect()
    };
    select(&mut app, "start_generator");
    for entity in old {
        assert!(app.world().get_entity(entity).is_err());
    }
    app.world_mut()
        .write_message(HostRequest::WakeCue(CueInstance {
            epoch,
            cue_id: "timeout".into(),
        }));
    advance(&mut app, 4500);
    advance(&mut app, 6000);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "DISPATCH_WARM"
    );
    for _ in 0..3 {
        select(&mut app, "restart_slice");
        annex(&mut app);
        advance(&mut app, 4000);
        select(&mut app, "abandon");
        advance(&mut app, 9000);
    }
    assert_eq!(entity_count::<Object>(&mut app), 8);
    assert_eq!(app.world().resource::<Assets<AudioSource>>().len(), 3);
    assert!(entity_count::<CueEntity>(&mut app) < 10);
}
#[test]
fn conditions_are_rechecked_and_deadline_still_wins_at_equality() {
    for (selected, lose_prereq) in [(29999, false), (30000, false), (4000, true)] {
        let mut app = app();
        annex(&mut app);
        advance(&mut app, 4000);
        if lose_prereq {
            app.world_mut().resource_mut::<Generator>().deicer_isolated = false;
        }
        let epoch = app.world().resource::<Runtime>().sheet.epoch;
        app.world_mut().write_message(HostRequest::Selection {
            edge_id: "start_generator".into(),
            epoch,
            observed_ms: selected,
        });
        advance(&mut app, if selected == 4000 { 0 } else { 26000 });
        assert_eq!(
            app.world().resource::<Runtime>().sheet.node_id,
            if lose_prereq {
                "GENERATOR_ANNEX"
            } else if selected == 30000 {
                "ANNEX_SEALED"
            } else {
                "GENERATOR_STARTUP"
            }
        );
    }
}
#[test]
fn missing_assets_and_handlers_have_specific_preflight_and_runtime_errors() {
    let graph = graph();
    let mut catalog = Catalog::default();
    catalog.sounds.remove(&Sound::Wind);
    assert!(
        game::validate_with_catalog(&graph, &catalog)
            .unwrap_err()
            .contains("wind.wav")
    );
    let mut catalog = Catalog::default();
    catalog.objects.remove(&Object::Starter);
    assert!(
        game::validate_with_catalog(&graph, &catalog)
            .unwrap_err()
            .contains("Starter")
    );
    let mut bad = graph.clone();
    bad.nodes[0].cues[1].instruction = grimm_graph::Instruction::Presentation {
        presentation_id: "missing.handler".into(),
        duration_ms: None,
        args: Default::default(),
    };
    assert!(
        game::validate_bindings(&bad)
            .unwrap_err()
            .contains("missing.handler")
    );
    let mut app = app();
    let handle = app.world().resource::<AudioBank>().0[&Sound::Wind].clone();
    app.world_mut()
        .resource_mut::<Assets<AudioSource>>()
        .remove(handle.id());
    advance(&mut app, 18000);
    assert!(
        app.world()
            .resource::<SceneView>()
            .failure
            .as_ref()
            .unwrap()
            .contains("audio data")
    );
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "DISPATCH_APPROACH"
    );
    advance(&mut app, 0);
    assert_eq!(entity_count::<CueEntity>(&mut app), 0);
}
#[test]
fn audio_placeholders_decode_to_non_silent_pcm_without_an_audio_device() {
    for sound in Sound::ALL {
        let source = AudioSource {
            bytes: sound.bytes().into(),
        };
        assert!(source.decoder().take(1024).any(|sample| sample != 0.));
    }
}
#[test]
fn closing_an_annex_does_not_stop_an_already_running_canonical_generator() {
    let mut state = gameplay::Generator {
        status: GeneratorStatus::Powered,
        ..default()
    };
    state.apply("seal_annex", &Default::default()).unwrap();
    assert!(state.engineering_lost);
    assert_eq!(state.status, GeneratorStatus::Powered);
}
#[test]
fn canonical_dialogue_and_current_manifest_are_bound_not_independent_demo_fiction() {
    let source: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(include_str!("../../../../examples/whiteout/text.yaml")).unwrap();
    let normalize = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    for (id, original) in [
        ("canon.isolated", "generator.isolated"),
        ("canon.prime", "generator.startup_prime"),
        ("canon.ignition", "generator.startup_ignition"),
        ("canon.heat", "generator.startup_heat"),
    ] {
        assert_eq!(
            normalize(crate::content::presentation(id).unwrap().body),
            normalize(source["narratives"][original].as_str().unwrap())
        );
    }
    let manifest: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(include_str!("../../assets/handlers.yaml")).unwrap();
    for node in graph().nodes {
        for edge in node.edges.values() {
            if let Some(c) = &edge.condition {
                assert!(
                    manifest["conditions"]
                        .as_mapping()
                        .unwrap()
                        .contains_key(c.condition_id.as_str())
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
fn actual_motion_advances_with_scene_time_without_committing_a_gameplay_effect() {
    let mut app = app();
    let jonas_x = |app: &mut App| {
        let w = app.world_mut();
        w.query::<(&Object, &Transform)>()
            .iter(w)
            .find(|(object, _)| **object == Object::Jonas)
            .unwrap()
            .1
            .translation
            .x
    };
    let start = jonas_x(&mut app);
    advance(&mut app, 4000);
    advance(&mut app, 3000);
    let midpoint = jonas_x(&mut app);
    assert!(midpoint > start && midpoint < -210.);
    advance(&mut app, 3000);
    assert_eq!(jonas_x(&mut app), -210.);
    assert_eq!(app.world().resource::<Generator>().applications, 1);
    assert!(!app.world().resource::<Generator>().deicer_isolated);
}

#[test]
fn losing_a_live_stage_object_stops_progress_and_cleans_up_instances() {
    let mut app = app();
    annex(&mut app);
    advance(&mut app, 4000);
    select(&mut app, "start_generator");
    let starter = {
        let w = app.world_mut();
        w.query::<(Entity, &Object)>()
            .iter(w)
            .find(|(_, object)| **object == Object::Starter)
            .unwrap()
            .0
    };
    app.world_mut().despawn(starter);
    advance(&mut app, 4500);
    assert!(
        app.world()
            .resource::<SceneView>()
            .failure
            .as_ref()
            .unwrap()
            .contains("Starter")
    );
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "GENERATOR_STARTUP"
    );
    assert_eq!(
        app.world().resource::<Generator>().status,
        GeneratorStatus::Stopped
    );
    advance(&mut app, 0);
    assert_eq!(entity_count::<CueEntity>(&mut app), 0);
}

#[test]
fn graphic_mode_switch_preserves_epoch_timer_effects_and_active_cues() {
    let mut app = app();
    annex(&mut app);
    advance(&mut app, 11000);
    let sheet = app.world().resource::<Runtime>().sheet.clone();
    let time = app.world().resource::<Runtime>().elapsed_ms;
    let active = app.world().resource::<SceneView>().active.clone();
    let status = app.world().resource::<Generator>().status;
    scheduler::tests::key(
        &mut app,
        bevy::input::ButtonState::Pressed,
        false,
        KeyCode::F2,
    );
    advance(&mut app, 0);
    assert_eq!(
        *app.world().resource::<crate::art::GraphicMode>(),
        crate::art::GraphicMode::Layered
    );
    let runtime = app.world().resource::<Runtime>();
    assert_eq!(runtime.sheet, sheet);
    assert_eq!(runtime.elapsed_ms, time);
    assert_eq!(app.world().resource::<SceneView>().active, active);
    assert_eq!(app.world().resource::<Generator>().status, status);
    assert!(app.world().resource::<Generator>().deicer_isolated);
}

#[test]
fn pending_artwork_does_not_start_or_spend_the_authored_scene_clock() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        InputPlugin,
        game::install,
        crate::screen::install,
        install,
    ));
    app.add_systems(
        PostStartup,
        |mut paintings: ResMut<crate::art::Paintings>| paintings.ready = false,
    );
    advance(&mut app, 0);
    advance(&mut app, 60000);
    assert_eq!(
        app.world().resource::<Runtime>().sheet.node_id,
        "DISPATCH_APPROACH"
    );
    assert_eq!(app.world().resource::<Runtime>().elapsed_ms, 0);
    app.world_mut()
        .resource_mut::<crate::art::Paintings>()
        .ready = true;
    advance(&mut app, 0);
    assert_eq!(app.world().resource::<Runtime>().elapsed_ms, 0);
    advance(&mut app, 1000);
    assert_eq!(app.world().resource::<Runtime>().elapsed_ms, 1000);
}
