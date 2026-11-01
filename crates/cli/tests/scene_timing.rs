//! M4.5 protocol and synchronous host scheduler: all clocks are deterministic.
use grimm_cli::scene_session::SceneSession;
use grimm_engine::scene::SceneEngine;
use grimm_semantic::{scene::*, story::Value};

const TIMED: &str = r#"
schema_version: 2
id: clock_test
version: 1
title: Clock test
start: room
globals: {counter: 0}
contexts:
  - id: room
    locals: {marker: local}
    cues:
      - {cue_id: deadline, at_ms: 0, kind: timer, set_deadline_ms: 10000, visible: false}
      - {cue_id: intro, at_ms: 0, kind: narrative, narrative_id: arrival, duration_ms: 1000}
      - {cue_id: choices, at_ms: 0, after_cue: intro, kind: actions, available_actions: eligible}
      - {cue_id: radio, at_ms: 2500, kind: narrative, narrative_id: radio, duration_ms: 2000}
    actions:
      - id: unrelated
        duration_ms: 100
        effects: [{op: add, var: global.counter, value: 1}]
        outcomes:
          - cues: [{cue_id: result, kind: narrative, narrative_id: result, duration_ms: 200}]
      - id: cancel
        duration_ms: 200
        outcomes:
          - cues:
              - {cue_id: stop, kind: timer, unset_deadline: true}
              - {cue_id: cancelled, kind: narrative, narrative_id: cancelled, duration_ms: 500}
      - id: show
        outcomes: [{cues: [{cue_id: shown, kind: timer, visible: true}]}]
      - id: hide
        outcomes: [{cues: [{cue_id: hidden, kind: timer, show_deadline: false}]}]
      - id: extend
        outcomes: [{cues: [{cue_id: extended, kind: timer, extend_ms: 6000}]}]
      - id: absolute
        outcomes: [{cues: [{cue_id: absolute, kind: timer, deadline_at_ms: 9000}]}]
      - id: relative
        outcomes: [{cues: [{cue_id: relative, kind: timer, set_deadline_ms: 2000}]}]
    timeout:
      effects: [{op: set, var: global.counter, value: -1}]
      goto: end
  - id: end
    terminal: true
    cues: [{cue_id: end, kind: narrative, narrative_id: end, duration_ms: 100}]
"#;
fn engine(yaml: &str) -> SceneEngine {
    SceneEngine::new(serde_yaml_ng::from_str(yaml).unwrap()).unwrap()
}
fn session(yaml: &str) -> SceneSession {
    SceneSession::new(engine(yaml), true).unwrap()
}
fn choose(s: &mut SceneSession, id: &str, at: u64) {
    s.choose(&format!("room.{id}"), s.player().revision(), at)
        .unwrap();
}

#[test]
fn scene_clock_runs_without_deadline_and_auto_text_unblocks_its_dependency() {
    let yaml=TIMED.replace("      - {cue_id: deadline, at_ms: 0, kind: timer, set_deadline_ms: 10000, visible: false}\n","");
    let mut s = session(&yaml);
    assert!(s.player().clock().unwrap().running);
    assert_eq!(s.player().clock().unwrap().deadline_ms, None);
    assert!(s.player().actions().is_empty());
    s.advance_to(999).unwrap();
    assert!(s.player().actions().is_empty());
    s.advance_to(1000).unwrap();
    assert!(!s.player().actions().is_empty());
    assert_eq!(s.player().clock().unwrap().elapsed_ms, 1000);
    assert!(s.engine().save().history.iter().any(|event| matches!(event,SceneInput::Cue(c) if c.cue_id=="room.intro" && c.event==CueEvent::Completed && c.scene_time_ms==1000)));
    let wire = serde_json::to_value(s.engine().output()).unwrap();
    assert_eq!(wire["protocol_version"], 2);
    assert!(
        wire["cues"]
            .as_array()
            .unwrap()
            .iter()
            .all(|c| c["at_ms"].is_u64())
    );
}

#[test]
fn authored_cancel_removes_deadline_but_clock_and_replay_continue() {
    let mut s = session(TIMED);
    s.advance_to(1000).unwrap();
    choose(&mut s, "cancel", 2000);
    assert_eq!(s.player().clock().unwrap().deadline_ms, None);
    assert!(!s.player().clock().unwrap().visible);
    assert_eq!(s.player().clock().unwrap().elapsed_ms, 2000);
    s.advance_to(50000).unwrap();
    assert!(!s.engine().is_terminal());
    assert!(s.player().clock().unwrap().running);
    let saved = s.engine().save();
    assert_eq!(saved.schema_version, 2);
    let world = s.engine().inspect_world();
    let mut loaded = session(TIMED);
    loaded.restore(saved).unwrap();
    loaded.advance_to(60000).unwrap();
    assert_eq!(loaded.engine().inspect_world().globals, world.globals);
    assert_eq!(loaded.player().clock().unwrap().deadline_ms, None);
    assert!(!loaded.engine().is_terminal());
}

#[test]
fn unrelated_replace_visibility_and_extension_never_restart_the_clock() {
    let mut s = session(TIMED);
    s.advance_to(1000).unwrap();
    choose(&mut s, "unrelated", 1500);
    assert_eq!(s.player().clock().unwrap().elapsed_ms, 1500);
    assert_eq!(s.engine().action_history()[0].logical_end_ms, 1600);
    s.advance_to(2000).unwrap();
    choose(&mut s, "show", 2000);
    assert!(s.player().clock().unwrap().visible);
    assert_eq!(s.player().clock().unwrap().deadline_ms, Some(10000));
    choose(&mut s, "hide", 2000);
    assert!(!s.player().clock().unwrap().visible);
    choose(&mut s, "extend", 2000);
    assert_eq!(s.player().clock().unwrap().deadline_ms, Some(16000));
    choose(&mut s, "absolute", 2000);
    assert_eq!(s.player().clock().unwrap().deadline_ms, Some(9000));
    choose(&mut s, "relative", 2000);
    assert_eq!(s.player().clock().unwrap().deadline_ms, Some(4000));
    assert_eq!(s.player().clock().unwrap().epoch, 0);
}

#[test]
fn retained_running_narrative_does_not_restart_or_extend_after_replace_or_restore() {
    let mut s = session(TIMED);
    s.advance_to(2500).unwrap();
    assert!(
        s.player()
            .running_narratives()
            .contains(&"room.radio".into())
    );
    choose(&mut s, "unrelated", 3000);
    let radio = s
        .engine()
        .output()
        .cues
        .into_iter()
        .find(|c| c.cue_id == "room.radio")
        .unwrap();
    assert_eq!(radio.started_at_ms, Some(2500));
    let save = s.engine().save();
    let mut restored = session(TIMED);
    restored.restore(save).unwrap();
    restored.advance_to(4499).unwrap();
    assert!(
        restored
            .player()
            .running_narratives()
            .contains(&"room.radio".into())
    );
    restored.advance_to(4500).unwrap();
    assert!(
        !restored
            .player()
            .running_narratives()
            .contains(&"room.radio".into())
    );
    let count=restored.engine().save().history.iter().filter(|e|matches!(e,SceneInput::Cue(c) if c.cue_id=="room.radio" && c.event==CueEvent::Started)).count();
    assert_eq!(count, 1);
}

#[test]
fn deadline_wins_equal_time_auto_completion_and_stale_input_cannot_choose_successor() {
    let yaml = TIMED.replace("duration_ms: 1000", "duration_ms: 10000");
    let mut s = session(&yaml);
    let rev = s.player().revision();
    assert!(s.choose("room.cancel", rev, 10000).is_err());
    assert!(s.engine().is_terminal());
    assert_eq!(s.engine().inspect_world().scene_epoch, 1);
    assert_eq!(
        s.engine().inspect_world().globals["counter"],
        Value::Int(-1)
    );
    assert!(!s.engine().save().history.iter().any(
        |e| matches!(e,SceneInput::Cue(c) if c.cue_id=="room.intro" && c.event==CueEvent::Completed)
    ));
}

#[test]
fn completion_at_boundary_can_cancel_but_a_crossing_action_cannot() {
    for (selected, success) in [(9800, true), (9900, false)] {
        let mut s = session(TIMED);
        s.advance_to(1000).unwrap();
        choose(&mut s, "cancel", selected);
        assert_eq!(!s.engine().is_terminal(), success);
        let record = &s.engine().action_history()[0];
        assert_eq!(record.logical_end_ms, 10000);
        assert_eq!(record.interrupted, !success);
        assert_eq!(record.effective_ms, if success { 200 } else { 100 });
        if success {
            s.advance_to(20000).unwrap();
            assert!(!s.engine().is_terminal());
        }
    }
}

#[test]
fn inspection_is_pure_typed_and_separate_from_player_output() {
    let s = session(TIMED);
    let before = s.engine().save();
    let mut world = s.engine().inspect_world();
    assert_eq!(world.context_id, "room");
    assert_eq!(world.scene_epoch, 0);
    assert_eq!(world.locals["marker"], Value::Text("local".into()));
    world.globals.insert("counter".into(), Value::Int(99));
    assert_eq!(s.engine().inspect_world().globals["counter"], Value::Int(0));
    assert_eq!(s.engine().save(), before);
    let json = serde_json::to_value(s.engine().output()).unwrap();
    for hidden in ["globals", "locals", "context_id"] {
        assert!(json.get(hidden).is_none());
    }
}

#[test]
fn same_context_expiry_consumes_deadline_once_and_can_expire_at_current_time() {
    let yaml=TIMED.replace("      goto: end"," ").replace("      effects: [{op: set, var: global.counter, value: -1}]","      effects: [{op: add, var: global.counter, value: 1}]")
        .replace("  - id: end\n    terminal: true\n    cues: [{cue_id: end, kind: narrative, narrative_id: end, duration_ms: 100}]","");
    // Keep an authored terminal reachable through a normal exit.
    let yaml = yaml.replace(
        "      - id: relative",
        "      - id: finish\n        outcomes: [{goto: end}]\n      - id: relative",
    ) + "\n  - id: end\n    terminal: true\n    cues: []\n";
    let mut s = session(&yaml);
    s.advance_to(10000).unwrap();
    s.advance_to(20000).unwrap();
    assert_eq!(s.engine().inspect_world().globals["counter"], Value::Int(1));
    assert_eq!(s.player().clock().unwrap().deadline_ms, None);
    let instant = TIMED.replace("set_deadline_ms: 10000", "deadline_at_ms: 0");
    let mut immediate = session(&instant);
    immediate.advance_to(0).unwrap();
    assert!(immediate.engine().is_terminal());
}

#[test]
fn legacy_generated_ids_are_unchanged_by_protocol_lowering() {
    let story: SceneStory =
        serde_yaml_ng::from_str(include_str!("fixtures/authoring/valid-endings.yaml")).unwrap();
    let index = grimm_engine::scene::authoring::analyze(story)
        .index
        .unwrap();
    let mut ids: Vec<_> = index
        .contexts
        .into_iter()
        .flat_map(|c| c.cues)
        .map(|c| c.cue_id)
        .collect();
    ids.sort();
    assert_eq!(
        ids,
        [
            "failure.cue_ccaa1b10b4e83135",
            "room.cue_4ac25ef38c0e9f77",
            "room.welcome",
            "success.cue_ccaa1b10b4e83135"
        ]
    );
}

#[test]
fn pending_replacement_and_bounded_inspector_history_do_not_create_engine_events() {
    let yaml=TIMED.replace("      - {cue_id: deadline, at_ms: 0, kind: timer, set_deadline_ms: 10000, visible: false}\n", "");
    let mut s = session(&yaml);
    s.advance_to(1000).unwrap();
    let before = s.engine().save();
    let _ = s.player().history();
    let _ = s.engine().inspect_world();
    assert_eq!(s.engine().save(), before);
    for at in (1100..=11100).step_by(250) {
        choose(&mut s, "unrelated", at);
        s.advance_to(at + 200).unwrap();
        if s.engine().is_terminal() {
            break;
        }
    }
    assert!(s.player().history().len() <= grimm_cli_renderer::cues::HISTORY_LIMIT);
    let early = TIMED
        .replace("duration_ms: 1000", "duration_ms: 20000")
        .replace("set_deadline_ms: 10000", "set_deadline_ms: 2000");
    let mut cancelled = session(&early);
    // A new context discards the old pending radio and records presentation-only cancellation.
    cancelled.advance_to(2000).unwrap();
    assert!(
        cancelled
            .player()
            .history()
            .iter()
            .flat_map(|r| &r.cues)
            .any(|c| c.cue.cue_id == "room.radio"
                && matches!(c.lifecycle, grimm_cli_renderer::cues::Lifecycle::Cancelled))
    );
}

#[test]
fn constructing_host_from_restored_engine_recovers_running_text_without_restarting_it() {
    let mut original = session(TIMED);
    original.advance_to(3000).unwrap();
    let saved = original.engine().save();
    let mut restored = engine(TIMED);
    restored.restore(saved).unwrap();
    let restored_save = restored.save();
    let mut host = SceneSession::new(restored, true).unwrap();
    let text = host.take_started_text();
    assert_eq!(text.len(), 1);
    assert_eq!(text[0].cue.cue_id, "room.radio");
    assert_eq!(text[0].cue.started_at_ms, Some(2500));
    assert_eq!(host.engine().save(), restored_save);
    host.advance_to(4500).unwrap();
    assert!(host.player().running_narratives().is_empty());
    assert!(host.take_started_text().is_empty());
}

#[test]
fn absolute_deadline_can_be_the_only_authored_exit() {
    let yaml = r#"
schema_version: 2
id: absolute_exit
version: 1
title: Absolute exit
start: room
contexts:
  - id: room
    cues: [{kind: timer, deadline_at_ms: 100}]
    timeout: {goto: end}
  - id: end
    terminal: true
    cues: []
"#;
    let report = grimm_engine::scene::authoring::analyze(serde_yaml_ng::from_str(yaml).unwrap());
    assert!(report.index.is_some(), "{report:?}");
    let mut host = session(yaml);
    host.advance_to(100).unwrap();
    assert!(host.engine().is_terminal());
}

#[test]
fn automatic_overlap_reveal_and_sequential_release_share_one_continuous_clock() {
    let yaml = TIMED
        .replace("set_deadline_ms: 10000", "set_deadline_ms: 30000")
        .replace("at_ms: 2500", "at_ms: 10000")
        .replace("duration_ms: 100\n", "duration_ms: 3000\n")
        .replace(
            "narrative_id: result, duration_ms: 200",
            "narrative_id: result, duration_ms: 3500",
        )
        .replace(
            "    actions:\n",
            "      - {cue_id: reveal, at_ms: 10000, kind: timer, visible: true}\n    actions:\n",
        );
    let mut s = session(&yaml);
    s.advance_to(8000).unwrap();
    choose(&mut s, "unrelated", 8000);
    assert_eq!(s.player().clock().unwrap().elapsed_ms, 8000);
    s.advance_to(10000).unwrap();
    assert_eq!(s.player().running_narratives().len(), 2);
    let clock = s.player().clock().unwrap();
    assert!(clock.visible);
    assert_eq!(clock.deadline_ms.unwrap() - clock.elapsed_ms, 20000);
    s.advance_to(11000).unwrap();
    assert!(s.player().actions().is_empty()); // actual completion is a barrier too
    s.advance_to(11500).unwrap();
    assert!(!s.player().actions().is_empty());
    choose(&mut s, "show", 11600);
    assert_eq!(s.player().clock().unwrap().deadline_ms, Some(30000));
    s.advance_to(12200).unwrap();
    assert!(
        !s.player()
            .running_narratives()
            .contains(&"room.radio".into())
    );
    let starts = s.engine().save().history.iter().filter(|input| matches!(input, SceneInput::Cue(cue) if cue.cue_id == "room.radio" && cue.event == CueEvent::Started)).count();
    assert_eq!(starts, 1);
}

#[test]
fn lifecycle_diagnostics_reject_conflicting_deadlines_bad_dependencies_and_overflow() {
    for (from, to, message) in [
        (
            "set_deadline_ms: 10000",
            "set_deadline_ms: 10000, unset_deadline: true",
            "timer",
        ),
        ("after_cue: intro", "after_cue: missing", "after_cue"),
        (
            "cue_id: intro, at_ms: 0",
            "cue_id: intro, after_cue: choices, at_ms: 0",
            "cyclic",
        ),
        (
            "duration_ms: 1000",
            "duration_ms: 18446744073709551615",
            "overflow",
        ),
        ("schema_version: 2", "schema_version: 1", "schema_version"),
    ] {
        let story = serde_yaml_ng::from_str(&TIMED.replace(from, to)).unwrap();
        let error = match SceneEngine::new(story) {
            Ok(_) => panic!("accepted {to}"),
            Err(error) => error,
        };
        assert!(error.to_string().contains(message), "{error}");
    }
}

#[test]
fn cancellation_updates_all_retained_revision_rows() {
    let yaml = TIMED.replace(
        "      - id: relative",
        "      - id: finish\n        outcomes: [{goto: end}]\n      - id: relative",
    );
    let mut host = session(&yaml);
    host.advance_to(3000).unwrap();
    choose(&mut host, "show", 3000);
    choose(&mut host, "finish", 3100);
    let rows: Vec<_> = host
        .player()
        .history()
        .iter()
        .flat_map(|sheet| &sheet.cues)
        .filter(|row| row.cue.cue_id == "room.radio")
        .collect();
    assert!(rows.len() >= 2);
    assert!(rows.iter().all(|row| matches!(
        row.lifecycle,
        grimm_cli_renderer::cues::Lifecycle::Cancelled
    )));
}

#[test]
fn archived_schema_one_story_and_checkpoint_still_restore_with_original_clock_semantics() {
    let source = include_str!("../../../examples/locked-room/legacy-v1/story.yaml");
    let mut old = session(source);
    assert!(!old.player().clock().unwrap().running);
    let id = old.player().running_narratives()[0].clone();
    old.complete_text(&id, old.player().revision(), 0).unwrap();
    old.advance_to(8000).unwrap();
    let before = old.engine().output().clock;
    let save = old.engine().save();
    assert_eq!(save.schema_version, 1);
    let mut restored = session(source);
    restored.restore(save.clone()).unwrap();
    assert_eq!(restored.engine().output().clock, before);
    assert!(
        engine(include_str!("../../../examples/locked-room/story.yaml"))
            .restore(save)
            .is_err()
    );
}
