//! M4.6 headless lifecycle tests: deterministic protocol inputs, no sleeps.
use grimm_engine::scene::lifecycle::LifecycleEngine;
use grimm_semantic::{lifecycle::*, scene::*, story::Value};
const STORY: &str = r#"
schema_version: 2
id: lifecycle_demo
version: 1
title: Lifecycle demo
start: room
globals: {count: 0}
contexts:
  - id: room
    locals: {marker: 1}
    cues:
      - {cue_id: intro, kind: narrative, narrative_id: intro, duration_ms: 10}
      - {cue_id: clock, kind: timer, set_deadline_ms: 1000, visible: false}
      - {cue_id: choices, kind: actions, available_actions: eligible}
      - {cue_id: warning, at_ms: 100, kind: narrative, narrative_id: warning, duration_ms: 20}
    actions:
      - id: travel
        duration_ms: 30
        effects: [{op: add, var: global.count, value: 1}]
        outcomes:
          - goto: hall
            cues: [{cue_id: farewell, kind: narrative, narrative_id: farewell, duration_ms: 5000}]
    timeout:
      effects: [{op: set, var: global.count, value: -1}]
      cues: [{cue_id: timeout, kind: narrative, narrative_id: timeout, duration_ms: 50}]
      goto: end
  - id: hall
    locals: {marker: 7}
    cues:
      - {cue_id: clock, at_ms: 0, kind: timer, set_deadline_ms: 500, visible: true}
      - {cue_id: arrival, kind: narrative, narrative_id: arrival, duration_ms: 100}
      - {cue_id: choices, kind: actions, available_actions: eligible}
    actions:
      - id: return
        outcomes: [{goto: room}]
      - id: finish
        outcomes: [{goto: end}]
    timeout: {goto: end}
  - id: end
    terminal: true
    cues: []
"#;
const CONFIG: &str = r#"
schema_version: 1
id: lifecycle_test
version: 1
story_id: lifecycle_demo
story_version: 1
scenes: {room: demo.room, hall: demo.hall, end: demo.end}
"#;
fn new() -> LifecycleEngine {
    LifecycleEngine::new(
        serde_yaml_ng::from_str(STORY).unwrap(),
        serde_yaml_ng::from_str(CONFIG).unwrap(),
    )
    .unwrap()
}
fn signal(engine: &mut LifecycleEngine, event: SceneEvent) -> SceneCueSheet {
    let sheet = engine.output();
    engine
        .scene_input(&SceneFeedback {
            revision: sheet.plan.revision,
            scene_id: sheet.scene.scene_id,
            epoch: sheet.scene.epoch,
            event,
        })
        .unwrap()
}
fn cue(engine: &mut LifecycleEngine, id: &str, event: CueEvent, time: u64) -> SceneCueSheet {
    let sheet = engine.output();
    engine
        .cue_input(&CueInput {
            revision: sheet.plan.revision,
            cue_id: id.into(),
            event,
            scene_time_ms: time,
        })
        .unwrap()
}
fn begin(engine: &mut LifecycleEngine) {
    signal(engine, SceneEvent::Ready);
    signal(engine, SceneEvent::Begin);
}
fn room(engine: &mut LifecycleEngine, time: u64) {
    begin(engine);
    cue(engine, "room.intro", CueEvent::Started, 0);
    cue(engine, "room.intro", CueEvent::Completed, time);
    cue(engine, "room.clock", CueEvent::Started, time);
    cue(engine, "room.choices", CueEvent::Started, time);
}
fn travel(engine: &mut LifecycleEngine, time: u64) {
    engine
        .apply(&SceneAction {
            revision: engine.output().plan.revision,
            action_id: "room.travel".into(),
            scene_time_ms: time,
        })
        .unwrap();
}
fn finish_outgoing(engine: &mut LifecycleEngine) {
    for _ in 0..16 {
        let sheet = engine.output();
        let Some(c) = sheet.plan.cues.first() else {
            break;
        };
        let id = c.cue_id.clone();
        let time = sheet.plan.clock.elapsed_ms;
        cue(engine, &id, CueEvent::Started, time);
        cue(engine, &id, CueEvent::Completed, time);
    }
    assert!(engine.output().plan.cues.is_empty());
    signal(engine, SceneEvent::End);
}
#[test]
fn preparation_readiness_and_duplicate_lifecycle_feedback_cannot_start_early() {
    let mut e = new();
    let initial = e.output();
    assert_eq!(initial.scene.phase, ScenePhase::Prepare);
    assert!(!initial.plan.clock.running);
    assert!(initial.plan.cues.is_empty());
    assert!(e.checkpoint().is_err());
    assert!(e.observe(initial.plan.revision, 10000).is_err());
    assert!(
        e.scene_input(&SceneFeedback {
            revision: 0,
            scene_id: "demo.room".into(),
            epoch: 0,
            event: SceneEvent::Begin
        })
        .is_err()
    );
    signal(&mut e, SceneEvent::PreparationFailed);
    assert!(!e.output().scene.ready);
    let sheet = e.output();
    let ready = SceneFeedback {
        revision: sheet.plan.revision,
        scene_id: sheet.scene.scene_id,
        epoch: 0,
        event: SceneEvent::Ready,
    };
    e.scene_input(&ready).unwrap();
    signal(&mut e, SceneEvent::Begin);
    let active = e.output();
    assert!(active.plan.clock.running);
    assert_eq!(active.plan.clock.elapsed_ms, 0);
    assert_eq!(e.scene_input(&ready).unwrap(), active);
    assert!(
        e.scene_input(&SceneFeedback {
            event: SceneEvent::Begin,
            ..ready
        })
        .is_err()
    );
    let json = serde_json::to_string(&active).unwrap();
    assert!(!json.contains("globals"));
    assert!(!json.contains("locals"));
    assert!(!json.contains("context_id"));
}
#[test]
fn relative_deadline_uses_actual_start_and_outgoing_media_never_charges_scene_time() {
    let mut e = new();
    room(&mut e, 50);
    assert_eq!(e.output().plan.clock.deadline_ms, Some(1050));
    travel(&mut e, 50);
    let ending = e.output();
    assert_eq!(ending.scene.phase, ScenePhase::Ending);
    assert_eq!(ending.scene.scene_id, "demo.room");
    assert_eq!(ending.scene.next_scene_id, Some("demo.hall".into()));
    assert_eq!(ending.plan.clock.elapsed_ms, 50);
    assert!(!ending.plan.clock.running);
    assert_eq!(ending.plan.clock.deadline_ms, None);
    assert_eq!(e.world_snapshot().globals["count"], Value::Int(1));
    assert_eq!(e.world_snapshot().locals["marker"], Value::Int(7));
    assert!(e.observe(ending.plan.revision, 5050).is_err());
    assert!(e.checkpoint().is_err());
    let blocked = SceneFeedback {
        revision: ending.plan.revision,
        scene_id: ending.scene.scene_id.clone(),
        epoch: ending.scene.epoch,
        event: SceneEvent::End,
    };
    assert!(e.scene_input(&blocked).is_err());
    finish_outgoing(&mut e);
    assert_eq!(e.phase(), ScenePhase::Stable);
    let checkpoint = e.checkpoint().unwrap();
    assert_eq!(e.output().scene.next_scene_id, Some("demo.hall".into()));
    assert!(e.observe(e.output().plan.revision, 99999).is_err());
    assert_eq!(e.checkpoint().unwrap(), checkpoint);
    signal(&mut e, SceneEvent::Prepare);
    begin(&mut e);
    assert_eq!(e.output().plan.clock.elapsed_ms, 0);
    assert_eq!(e.output().plan.clock.epoch, 1);
    cue(&mut e, "hall.clock", CueEvent::Started, 0);
    assert_eq!(e.output().plan.clock.deadline_ms, Some(500));
    assert_eq!(e.world_snapshot().globals["count"], Value::Int(1));
}
#[test]
fn boundary_restore_replays_no_outgoing_graphics_and_rejects_inconsistent_world() {
    let mut e = new();
    room(&mut e, 10);
    travel(&mut e, 10);
    finish_outgoing(&mut e);
    let saved = e.checkpoint().unwrap();
    let old_revision = e.output().plan.revision;
    let mut corrupt = saved.clone();
    corrupt.world.globals.insert("count".into(), Value::Int(99));
    assert!(e.restore(corrupt).is_err());
    assert_eq!(e.checkpoint().unwrap(), saved);
    e.restore(saved.clone()).unwrap();
    assert!(e.output().plan.revision > old_revision);
    assert!(e.output().plan.cues.is_empty());
    assert_eq!(e.output().scene.next_scene_id, Some("demo.hall".into()));
    let saved_again = e.checkpoint().unwrap();
    e.restore(saved_again).unwrap();
    signal(&mut e, SceneEvent::Prepare);
    begin(&mut e);
    assert!(
        !e.output()
            .plan
            .cues
            .iter()
            .any(|c| c.cue_id.contains("farewell"))
    );
    assert_eq!(e.world_snapshot().globals["count"], Value::Int(1));
    assert!(e.restore(saved).is_err());
}
#[test]
fn timeout_and_cutoff_close_once_and_terminal_scene_also_requires_end() {
    for selected in [None, Some(1000)] {
        let mut e = new();
        room(&mut e, 10);
        if let Some(time) = selected {
            travel(&mut e, time);
        } else {
            e.observe(e.output().plan.revision, 1500).unwrap();
        }
        assert_eq!(e.phase(), ScenePhase::Ending);
        assert_eq!(e.world_snapshot().globals["count"], Value::Int(-1));
        if selected.is_some() {
            assert_eq!(e.engine().action_history().last().unwrap().effective_ms, 10);
        }
        finish_outgoing(&mut e);
        assert!(!e.is_complete());
        signal(&mut e, SceneEvent::Prepare);
        begin(&mut e);
        assert_eq!(e.phase(), ScenePhase::Ending);
        assert!(!e.is_complete());
        signal(&mut e, SceneEvent::End);
        assert!(e.is_complete());
        assert!(e.checkpoint().is_ok());
        assert!(
            e.scene_input(&SceneFeedback {
                revision: e.output().plan.revision,
                scene_id: "demo.end".into(),
                epoch: e.output().scene.epoch,
                event: SceneEvent::Prepare
            })
            .is_err()
        );
    }
}
#[test]
fn bad_configuration_never_silently_infers_scenes_from_filenames() {
    for source in [
        CONFIG.replace("demo.room", "Demo Room"),
        CONFIG.replace("room: demo.room, ", ""),
        CONFIG.replace("story_version: 1", "story_version: 2"),
    ] {
        assert!(
            LifecycleEngine::new(
                serde_yaml_ng::from_str(STORY).unwrap(),
                serde_yaml_ng::from_str(&source).unwrap()
            )
            .is_err()
        );
    }
}

#[test]
fn presentation_adapter_keeps_scope_and_feedback_out_of_world_state() {
    use grimm_presentation::lifecycle as p;
    let mut engine = new();
    let initial = engine.output();
    let shown = grimm_adapters::lifecycle::present(&initial);
    assert_eq!(shown.protocol_version, 3);
    assert_eq!(shown.scene.phase, p::ScenePhase::Prepare);
    assert_eq!(
        serde_json::to_value(&shown).unwrap(),
        serde_json::to_value(&initial).unwrap()
    );
    let ready = p::SceneFeedback {
        revision: shown.plan.revision,
        scene_id: shown.scene.scene_id,
        epoch: shown.scene.epoch,
        event: p::SceneEvent::Ready,
    };
    engine
        .scene_input(&grimm_adapters::lifecycle::feedback(ready))
        .unwrap();
    signal(&mut engine, SceneEvent::Begin);
    let shown = grimm_adapters::lifecycle::present(&engine.output());
    assert_eq!(shown.scene.phase, p::ScenePhase::Active);
    assert_eq!(shown.plan.clock.epoch, 0);
    assert!(
        serde_yaml_ng::from_str::<SceneConfiguration>(&CONFIG.replace(
            "room: demo.room, hall: demo.hall",
            "room: demo.room, room: demo.hall"
        ))
        .is_err()
    );
}

#[test]
fn fresh_start_can_restore_a_verified_boundary_before_any_scene_begins() {
    let mut original = new();
    room(&mut original, 10);
    travel(&mut original, 10);
    finish_outgoing(&mut original);
    let saved = original.checkpoint().unwrap();
    let mut fresh = new();
    fresh.restore(saved).unwrap();
    assert_eq!(fresh.phase(), ScenePhase::Stable);
    assert!(fresh.output().plan.cues.is_empty());
    assert_eq!(fresh.world_snapshot().globals["count"], Value::Int(1));
    signal(&mut fresh, SceneEvent::Prepare);
    begin(&mut fresh);
    assert_eq!(fresh.output().scene.scene_id, "demo.hall");
    assert_eq!(fresh.output().plan.clock.elapsed_ms, 0);
}

#[test]
fn shared_host_finishes_outgoing_media_without_advancing_frozen_gameplay() {
    use grimm_cli::scene_session::SceneSession;
    use grimm_presentation::lifecycle::ScenePhase as Phase;
    let mut host = SceneSession::new_lifecycle(new(), true).unwrap();
    assert_eq!(host.preparation_scene().unwrap(), "demo.room");
    assert!(host.advance_to(10000).is_err());
    host.begin_prepared_scene().unwrap();
    host.advance_to(10).unwrap();
    assert_eq!(host.player().clock().unwrap().deadline_ms, Some(1010));
    host.choose("room.travel", host.player().revision(), 10)
        .unwrap();
    assert_eq!(host.scene().unwrap().phase, Phase::Ending);
    assert!(!host.player().clock().unwrap().running);
    assert_eq!(host.player().clock().unwrap().elapsed_ms, 10);
    assert_eq!(host.player().running_narratives().len(), 1);
    let revision = host.provider_revision();
    assert!(host.save_json().is_err());
    host.advance_presentation_by(4999).unwrap();
    assert_eq!(host.scene().unwrap().phase, Phase::Ending);
    assert_eq!(host.provider_revision(), revision);
    assert_eq!(host.player().clock().unwrap().elapsed_ms, 10);
    host.advance_presentation_by(1).unwrap();
    assert_eq!(host.scene().unwrap().phase, Phase::Stable);
    assert_eq!(host.provider_revision(), revision);
    let checkpoint = host.save_json().unwrap();
    let mut fresh = SceneSession::new_lifecycle(new(), true).unwrap();
    fresh.restore_json(&checkpoint).unwrap();
    assert!(fresh.take_started_text().is_empty());
    assert_eq!(fresh.preparation_scene().unwrap(), "demo.hall");
    fresh.begin_prepared_scene().unwrap();
    assert_eq!(fresh.player().clock().unwrap().elapsed_ms, 0);
    assert_eq!(fresh.player().clock().unwrap().deadline_ms, Some(500));
    fresh.advance_to(100).unwrap();
    assert!(fresh.player().actions().contains(&"hall.finish".into()));
    assert_eq!(fresh.engine().globals()["count"], Value::Int(1));
    fresh
        .choose("hall.finish", fresh.player().revision(), 100)
        .unwrap();
    assert!(!fresh.is_complete());
    fresh.preparation_scene().unwrap();
    fresh.begin_prepared_scene().unwrap();
    assert!(fresh.is_complete());
    assert!(fresh.save_json().is_ok());
}

#[test]
fn automatic_terminal_text_finishes_before_scene_closure_and_does_not_replay_on_load() {
    use grimm_cli::scene_session::SceneSession;
    use grimm_cli_renderer::cues::Lifecycle;
    let story=STORY.replace("    terminal: true\n    cues: []","    terminal: true\n    cues: [{cue_id: end_text, kind: narrative, narrative_id: done, duration_ms: 100}]");
    let engine = LifecycleEngine::new(
        serde_yaml_ng::from_str(&story).unwrap(),
        serde_yaml_ng::from_str(CONFIG).unwrap(),
    )
    .unwrap();
    let mut host = SceneSession::new_lifecycle(engine, true).unwrap();
    host.begin_prepared_scene().unwrap();
    host.advance_to(10).unwrap();
    host.choose("room.travel", host.player().revision(), 10)
        .unwrap();
    host.advance_presentation_by(5000).unwrap();
    host.preparation_scene().unwrap();
    host.begin_prepared_scene().unwrap();
    host.advance_to(100).unwrap();
    host.choose("hall.finish", host.player().revision(), 100)
        .unwrap();
    host.preparation_scene().unwrap();
    host.begin_prepared_scene().unwrap();
    assert!(!host.is_complete());
    host.advance_to(500).unwrap();
    assert!(host.is_complete());
    assert_eq!(host.player().clock().unwrap().elapsed_ms, 100);
    assert!(
        host.player()
            .history()
            .iter()
            .flat_map(|sheet| &sheet.cues)
            .any(|row| row.cue.cue_id == "end.end_text"
                && matches!(row.lifecycle, Lifecycle::Completed { .. }))
    );
    let saved = host.save_json().unwrap();
    host.restore_json(&saved).unwrap();
    assert!(host.is_complete());
    assert!(host.take_started_text().is_empty());
}

#[test]
fn reveal_extend_and_unset_preserve_absolute_and_relative_deadlines() {
    let source = STORY.replace(
        "      - {cue_id: warning, at_ms: 100, kind: narrative, narrative_id: warning, duration_ms: 20}",
        "      - {cue_id: reveal, at_ms: 100, kind: timer, visible: true}\n      - {cue_id: extend, at_ms: 200, kind: timer, extend_ms: 500}\n      - {cue_id: stop, at_ms: 300, kind: timer, unset_deadline: true}",
    );
    for absolute in [false, true] {
        let source = if absolute {
            source.replace("set_deadline_ms: 1000", "deadline_at_ms: 1000")
        } else {
            source.clone()
        };
        let mut e = LifecycleEngine::new(
            serde_yaml_ng::from_str(&source).unwrap(),
            serde_yaml_ng::from_str(CONFIG).unwrap(),
        )
        .unwrap();
        room(&mut e, 50);
        let end = if absolute { 1000 } else { 1050 };
        assert_eq!(e.output().plan.clock.deadline_ms, Some(end));
        cue(&mut e, "room.reveal", CueEvent::Started, 100);
        assert!(e.output().plan.clock.visible);
        assert_eq!(e.output().plan.clock.deadline_ms, Some(end));
        cue(&mut e, "room.extend", CueEvent::Started, 200);
        assert_eq!(e.output().plan.clock.deadline_ms, Some(end + 500));
        cue(&mut e, "room.stop", CueEvent::Started, 300);
        e.observe(e.output().plan.revision, 5000).unwrap();
        assert_eq!(e.phase(), ScenePhase::Active);
        assert!(e.output().plan.clock.running);
        assert_eq!(e.output().plan.clock.deadline_ms, None);
        assert_eq!(e.world_snapshot().globals["count"], Value::Int(0));
    }
}

#[test]
fn revisiting_same_scene_rejects_prior_epoch_cue_and_lifecycle_acknowledgements() {
    let mut e = new();
    room(&mut e, 10);
    let old = e.output();
    let old_cue = CueInput {
        revision: old.plan.revision,
        cue_id: "room.warning".into(),
        event: CueEvent::Started,
        scene_time_ms: 100,
    };
    travel(&mut e, 10);
    finish_outgoing(&mut e);
    signal(&mut e, SceneEvent::Prepare);
    begin(&mut e);
    cue(&mut e, "hall.clock", CueEvent::Started, 0);
    cue(&mut e, "hall.arrival", CueEvent::Started, 0);
    cue(&mut e, "hall.arrival", CueEvent::Completed, 100);
    cue(&mut e, "hall.choices", CueEvent::Started, 100);
    e.apply(&SceneAction {
        revision: e.output().plan.revision,
        action_id: "hall.return".into(),
        scene_time_ms: 100,
    })
    .unwrap();
    finish_outgoing(&mut e);
    signal(&mut e, SceneEvent::Prepare);
    room(&mut e, 10);
    let current = e.output();
    assert_eq!(current.scene.scene_id, old.scene.scene_id);
    assert_eq!(current.scene.epoch, old.scene.epoch + 2);
    assert!(e.cue_input(&old_cue).is_err());
    assert!(
        e.scene_input(&SceneFeedback {
            revision: old.plan.revision,
            scene_id: old.scene.scene_id,
            epoch: old.scene.epoch,
            event: SceneEvent::End
        })
        .is_err()
    );
    assert_eq!(e.output(), current);
    assert_eq!(e.world_snapshot().locals["marker"], Value::Int(1));
    assert_eq!(e.world_snapshot().globals["count"], Value::Int(1));
}

#[test]
fn legacy_and_boundary_save_formats_are_rejected_cross_mode_without_mutation() {
    use grimm_cli::scene_session::SceneSession;
    use grimm_engine::scene::SceneEngine;
    let mut boundary = new();
    room(&mut boundary, 10);
    travel(&mut boundary, 10);
    finish_outgoing(&mut boundary);
    let saved = boundary.checkpoint().unwrap();
    let json = serde_json::to_string(&saved).unwrap();
    let schema1 = "schema_version: 1\nid: old_game\nversion: 1\ntitle: Old game\nstart: end\ncontexts: [{id: end, terminal: true, cues: []}]\n";
    for source in [schema1, STORY] {
        let mut legacy = SceneSession::new(
            SceneEngine::new(serde_yaml_ng::from_str(source).unwrap()).unwrap(),
            false,
        )
        .unwrap();
        let old = legacy.save_json().unwrap();
        assert!(legacy.restore_json(&json).is_err());
        assert_eq!(legacy.save_json().unwrap(), old);
        let mut host = SceneSession::new_lifecycle(new(), false).unwrap();
        let before = host.engine().save();
        assert!(host.restore_json(&old).is_err());
        assert_eq!(host.engine().save(), before);
        assert_eq!(
            host.scene().unwrap().phase,
            grimm_presentation::lifecycle::ScenePhase::Prepare
        );
        legacy.restore_json(&old).unwrap();
    }
    for field in ["schema_version", "story_version", "configuration_version"] {
        let mut corrupt = serde_json::to_value(&saved).unwrap();
        corrupt[field] = serde_json::json!(999);
        assert!(
            boundary
                .restore(serde_json::from_value(corrupt).unwrap())
                .is_err()
        );
        assert_eq!(boundary.checkpoint().unwrap(), saved);
    }
}

#[test]
fn restore_reservation_keeps_duplicate_acknowledgements_safe_and_stale_ones_inert() {
    let mut e = new();
    room(&mut e, 10);
    let old = e.output();
    travel(&mut e, 10);
    finish_outgoing(&mut e);
    let saved = e.checkpoint().unwrap();
    let accepted_end = match saved.history.last().unwrap() {
        LifecycleInput::Scene(input) => input.clone(),
        other => panic!("expected end acknowledgement, got {other:?}"),
    };
    e.restore(saved).unwrap();
    let restored = e.output();
    let world = e.world_snapshot();
    assert_eq!(e.scene_input(&accepted_end).unwrap(), restored);
    assert_eq!(e.scene_input(&accepted_end).unwrap(), restored);
    assert!(
        e.scene_input(&SceneFeedback {
            event: SceneEvent::Begin,
            ..accepted_end.clone()
        })
        .is_err()
    );
    assert_eq!(e.output(), restored);
    assert_eq!(e.world_snapshot(), world);
    signal(&mut e, SceneEvent::Prepare);
    begin(&mut e);
    let current = e.output();
    assert_eq!(e.scene_input(&accepted_end).unwrap(), current);
    assert!(
        e.cue_input(&CueInput {
            revision: old.plan.revision,
            cue_id: "room.warning".into(),
            event: CueEvent::Started,
            scene_time_ms: 100
        })
        .is_err()
    );
    let clock = CueInput {
        revision: current.plan.revision,
        cue_id: "hall.clock".into(),
        event: CueEvent::Started,
        scene_time_ms: 0,
    };
    let started = e.cue_input(&clock).unwrap();
    assert_eq!(e.cue_input(&clock).unwrap(), started);
    assert_eq!(
        e.cue_input(&CueInput {
            event: CueEvent::Completed,
            ..clock
        })
        .unwrap(),
        started
    );
    assert_eq!(started.plan.clock.deadline_ms, Some(500));
    assert_eq!(e.world_snapshot(), world);
}

#[test]
fn oversized_boundary_history_is_rejected_before_replay_without_mutation() {
    let mut e = new();
    room(&mut e, 10);
    travel(&mut e, 10);
    finish_outgoing(&mut e);
    let saved = e.checkpoint().unwrap();
    let mut oversized = saved.clone();
    oversized
        .history
        .resize(65537, oversized.history[0].clone());
    assert!(
        e.restore(oversized)
            .unwrap_err()
            .to_string()
            .contains("history mismatch")
    );
    assert_eq!(e.checkpoint().unwrap(), saved);
}
