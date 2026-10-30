//! M2 deterministic renderer-clock integration. No sleeps or wall clocks.
use grimm_adapters::cues as adapter;
use grimm_cli_renderer::cues::CuePlayer;
use grimm_engine::scene::SceneEngine;
use grimm_presentation::cues::{CueEvent, Instruction};
use grimm_semantic::{scene::SceneStory, story::Value};

const OVERLAP: &str = r#"
schema_version: 1
id: overlap
version: 1
title: Drawer and radio
start: room
contexts:
  - id: room
    locals: {drawer_open: false}
    cues:
      - {cue_id: intro, kind: narrative, narrative_id: search_room_start}
      - {cue_id: start_clock, kind: timer, start_ms: 30000, visible: false}
      - {cue_id: initial_choices, kind: actions, available_actions: [open_drawer, search_desk]}
      - {cue_id: radio_warning, at_ms: 10000, kind: narrative, narrative_id: radio_johnny_returns}
      - {cue_id: reveal_clock, at_ms: 10000, kind: timer, visible: true}
      - {cue_id: warning_choices, at_ms: 10000, kind: actions, available_actions: eligible}
    actions:
      - id: open_drawer
        when: {op: eq, left: {var: local.drawer_open}, right: false}
        duration_ms: 3000
        outcomes:
          - effects: [{op: set, var: local.drawer_open, value: true}]
            cues:
              - {cue_id: drawer_opening, kind: narrative, narrative_id: drawer_opened}
              - {cue_id: drawer_new_choices, at_ms: 0, kind: actions, available_actions: eligible}
      - id: search_desk
        outcomes:
          - cues:
              - {cue_id: desk_searched, kind: narrative, narrative_id: desk_empty}
              - {cue_id: desk_choices, at_ms: 0, kind: actions, available_actions: eligible}
      - id: hide
        outcomes:
          - goto: escaped
            cues: [{cue_id: hidden, kind: narrative, narrative_id: hid_successfully}]
    timeout:
      goto: caught
      cues: [{cue_id: timeout, kind: narrative, narrative_id: caught_at_desk}]
  - id: escaped
    terminal: true
    cues: [{cue_id: escape_text, kind: narrative, narrative_id: escaped}]
  - id: caught
    terminal: true
    cues: [{cue_id: caught_text, kind: narrative, narrative_id: caught}]
"#;

fn parse(yaml: &str) -> SceneEngine {
    SceneEngine::new(serde_yaml_ng::from_str::<SceneStory>(yaml).unwrap()).unwrap()
}

fn pump(engine: &mut SceneEngine, player: &mut CuePlayer) -> Vec<String> {
    let mut started = vec![];
    for _ in 0..128 {
        let Some(cue) = player.due().into_iter().next() else {
            return started;
        };
        let feedback = player.started(&cue.cue_id).unwrap();
        engine
            .cue_input(&adapter::feedback(feedback))
            .unwrap_or_else(|error| {
                panic!("starting {} at {:?}: {error}", cue.cue_id, player.clock())
            });
        started.push(cue.cue_id.clone());
        if !matches!(cue.instruction, Instruction::Narrative { .. }) {
            let feedback = player.completed(&cue.cue_id).unwrap();
            engine.cue_input(&adapter::feedback(feedback)).unwrap();
        }
    }
    panic!("unbounded cue execution");
}
fn complete(engine: &mut SceneEngine, player: &mut CuePlayer, id: &str) {
    let feedback = player.completed(id).unwrap();
    assert_eq!(feedback.event, CueEvent::Completed);
    engine.cue_input(&adapter::feedback(feedback)).unwrap();
}
fn at(engine: &mut SceneEngine, player: &mut CuePlayer, time: u64) {
    let sheet = engine.observe(player.revision(), time).unwrap();
    if sheet.revision != player.revision() {
        player.replace(adapter::present(&sheet));
    } else {
        player.set_time(time).unwrap();
    }
}
fn select(engine: &mut SceneEngine, player: &mut CuePlayer, id: &str) {
    let index = player
        .actions()
        .iter()
        .position(|action| action == id)
        .unwrap();
    let input = adapter::action(player.select(index).unwrap());
    let sheet = engine.apply(&input).unwrap();
    player.replace(adapter::present(&sheet));
}

#[test]
fn mandatory_drawer_radio_overlap_uses_one_clock_and_full_replacement() {
    let mut engine = parse(OVERLAP);
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    assert_eq!(pump(&mut engine, &mut player), ["room.intro"]);
    assert!(!player.clock().unwrap().running);
    assert!(player.actions().is_empty());
    complete(&mut engine, &mut player, "room.intro");
    pump(&mut engine, &mut player);
    assert!(player.clock().unwrap().running);
    assert!(!player.clock().unwrap().visible);
    assert_eq!(player.clock().unwrap().deadline_ms, Some(30000));
    at(&mut engine, &mut player, 8000);
    select(&mut engine, &mut player, "room.open_drawer");
    assert_eq!(engine.locals()["drawer_open"], Value::Bool(true));
    assert_eq!(engine.action_history()[0].logical_end_ms, 11000);
    assert_eq!(player.clock().unwrap().elapsed_ms, 8000); // never jump to 11
    assert!(player.actions().is_empty());
    let started = pump(&mut engine, &mut player);
    assert!(started.contains(&"room.drawer_opening__run_1".into()));
    assert!(
        player
            .running_narratives()
            .contains(&"room.drawer_opening__run_1".into())
    );
    at(&mut engine, &mut player, 10000);
    let started = pump(&mut engine, &mut player);
    assert!(started.contains(&"room.radio_warning".into()));
    assert!(player.clock().unwrap().visible);
    assert_eq!(
        player.clock().unwrap().deadline_ms.unwrap() - player.clock().unwrap().elapsed_ms,
        20000
    );
    assert!(player.actions().is_empty()); // logical duration not finished
    complete(&mut engine, &mut player, "room.radio_warning");
    at(&mut engine, &mut player, 11000);
    pump(&mut engine, &mut player);
    assert!(player.actions().contains(&"room.search_desk".into()));
    // Availability overlaps the still-running drawer cue, by explicit at.
    assert!(
        player
            .running_narratives()
            .contains(&"room.drawer_opening__run_1".into())
    );
    at(&mut engine, &mut player, 12000);
    select(&mut engine, &mut player, "room.search_desk");
    let sheet = engine.output();
    assert!(
        !sheet
            .cues
            .iter()
            .any(|cue| cue.cue_id == "room.start_clock" || cue.cue_id == "room.radio_warning")
    );
    assert!(
        sheet
            .cues
            .iter()
            .any(|cue| cue.cue_id == "room.drawer_opening__run_1")
    );
    assert_eq!(sheet.clock.deadline_ms, Some(30000));
    let started = pump(&mut engine, &mut player);
    assert!(!started.contains(&"room.drawer_opening__run_1".into()));
    select(&mut engine, &mut player, "room.hide");
    assert!(engine.is_terminal());
    assert_eq!(engine.output().clock.epoch, 1);
    assert!(!engine.output().cues.iter().any(
        |cue| cue.cue_id == "room.drawer_opening__run_1" || cue.cue_id == "room.radio_warning"
    ));
    assert!(player.actions().is_empty());
}

#[test]
fn locked_room_is_a_real_authored_dsl_with_complete_text_bindings() {
    let yaml = include_str!("../../../examples/locked-room/story.yaml");
    let engine = parse(yaml);
    assert_eq!(engine.globals()["character_strength"], Value::Int(1));
    assert_eq!(engine.locals()["door_damage"], Value::Int(0));
    let text: serde_yaml_ng::Value =
        serde_yaml_ng::from_str(include_str!("../../../examples/locked-room/text.yaml")).unwrap();
    let story: SceneStory = serde_yaml_ng::from_str(yaml).unwrap();
    for context in &story.contexts {
        let namespace = context.namespace.as_deref().unwrap_or(&context.id);
        for action in &context.actions {
            let id = format!("{namespace}.{}", action.id);
            assert!(text["actions"][&id].as_str().is_some(), "missing {id}");
        }
        let result_cues = context.actions.iter().flat_map(|action| {
            action
                .outcomes
                .iter()
                .chain(&action.interrupted_outcomes)
                .flat_map(|branch| &branch.outcome.cues)
        });
        for cue in context
            .cues
            .iter()
            .chain(result_cues)
            .chain(context.timeout.iter().flat_map(|outcome| &outcome.cues))
        {
            if let grimm_semantic::scene::AuthoredInstruction::Narrative { narrative_id } =
                &cue.instruction
            {
                let id = if narrative_id.contains('.') {
                    narrative_id.clone()
                } else {
                    format!("{namespace}.{narrative_id}")
                };
                assert!(text["narratives"][&id].as_str().is_some(), "missing {id}");
            }
        }
    }
    assert!(
        engine
            .output()
            .cues
            .iter()
            .any(|cue| cue.cue_id == "locked_room.scene_deadline")
    );
}

const LOCKED_ROOM: &str = include_str!("../../../examples/locked-room/story.yaml");

fn settle(engine: &mut SceneEngine, player: &mut CuePlayer) -> Vec<String> {
    let mut seen = vec![];
    for _ in 0..128 {
        seen.extend(pump(engine, player));
        if let Some(id) = player.running_narratives().first() {
            complete(engine, player, id);
        } else {
            return seen;
        }
    }
    panic!("presentation did not settle");
}

fn room(strength: i64, dex: i64) -> (SceneEngine, CuePlayer) {
    let mut story: SceneStory = serde_yaml_ng::from_str(LOCKED_ROOM).unwrap();
    story
        .globals
        .insert("character_strength".into(), Value::Int(strength));
    story
        .globals
        .insert("character_dex".into(), Value::Int(dex));
    let mut engine = SceneEngine::new(story).unwrap();
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    (engine, player)
}

fn play(engine: &mut SceneEngine, player: &mut CuePlayer, action: &str) -> Vec<String> {
    let epoch = player.clock().unwrap().epoch;
    select(engine, player, &format!("locked_room.{action}"));
    let mut cues = settle(engine, player);
    if player.clock().unwrap().epoch == epoch {
        let end = engine.action_history().last().unwrap().logical_end_ms;
        at(engine, player, end);
        cues.extend(settle(engine, player));
    }
    cues
}

#[test]
fn complete_locked_room_keys_trace_books_fourteen_seconds_once() {
    let (mut engine, mut player) = room(1, 3);
    at(&mut engine, &mut player, 2000);
    play(&mut engine, &mut player, "open_drawer");
    assert_eq!(engine.locals()["drawer_open"], Value::Bool(true));
    assert_eq!(player.clock().unwrap().elapsed_ms, 3000);
    at(&mut engine, &mut player, 5000);
    play(&mut engine, &mut player, "take_keychain");
    assert_eq!(engine.globals()["has_keychain"], Value::Bool(true));
    assert_eq!(player.clock().unwrap().elapsed_ms, 6000);
    at(&mut engine, &mut player, 8000);
    let cues = play(&mut engine, &mut player, "unlock_door_key1");
    assert!(cues.iter().any(|id| id.contains("wrong_key")));
    assert_eq!(engine.globals()["door_open"], Value::Bool(false));
    assert_eq!(player.clock().unwrap().elapsed_ms, 10000);
    at(&mut engine, &mut player, 12000);
    play(&mut engine, &mut player, "unlock_door_key2");
    assert!(engine.is_terminal());
    assert_eq!(engine.globals()["door_open"], Value::Bool(true));
    assert_eq!(
        engine.action_history().last().unwrap().logical_end_ms,
        14000
    );
    assert_eq!(engine.action_history().last().unwrap().effective_ms, 2000);
    assert!(engine.locals().is_empty()); // Context-local facts are discarded on exit.
}

#[test]
fn repeated_door_attempts_have_distinct_cues_strength_and_dexterity_outcomes() {
    for (strength, attempts) in [(1, 4), (2, 2), (4, 1)] {
        let (mut engine, mut player) = room(strength, 3);
        let mut cues = vec![];
        for attempt in 1..=attempts {
            cues.extend(play(&mut engine, &mut player, "break_door"));
            if attempt < attempts {
                assert!(!engine.is_terminal());
                assert_eq!(
                    engine.locals()["door_damage"],
                    Value::Int(strength * attempt)
                );
                assert!(
                    player
                        .actions()
                        .contains(&"locked_room.squeeze_through".into())
                );
            }
        }
        assert!(engine.is_terminal());
        assert_eq!(engine.action_history().len(), attempts as usize);
        assert_eq!(
            engine.action_history().last().unwrap().logical_end_ms,
            attempts as u64 * 2000
        );
        assert_eq!(engine.globals()["door_open"], Value::Bool(true));
        if strength == 1 {
            for cue in [
                "door_barely",
                "door_progress",
                "door_almost",
                "door_success",
            ] {
                assert!(
                    cues.iter().any(|id| id.contains(cue)),
                    "missing {cue}: {cues:?}"
                );
            }
        }
    }
    let (mut engine, mut player) = room(1, 3);
    play(&mut engine, &mut player, "break_door");
    play(&mut engine, &mut player, "break_door");
    let checkpoint = engine.save();
    play(&mut engine, &mut player, "squeeze_through");
    assert!(engine.is_terminal());
    assert_eq!(engine.action_history().last().unwrap().logical_end_ms, 6000);
    engine.restore(checkpoint).unwrap();
    player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    play(&mut engine, &mut player, "break_door"); // damage 3
    let cues = play(&mut engine, &mut player, "squeeze_through");
    assert!(!engine.is_terminal());
    assert!(cues.iter().any(|id| id.contains("squeeze_failure")));
    assert_eq!(engine.locals()["door_damage"], Value::Int(3));
    play(&mut engine, &mut player, "break_door");
    assert!(engine.is_terminal());
}

#[test]
fn deadline_cutoff_and_authored_boundary_policy_commit_failure_immediately() {
    let (mut engine, mut player) = room(4, 3);
    at(&mut engine, &mut player, 29000);
    select(&mut engine, &mut player, "locked_room.break_door");
    assert!(engine.is_terminal()); // No playback/progress callback decides this.
    assert_eq!(engine.globals()["door_open"], Value::Bool(false));
    let record = engine.action_history().last().unwrap();
    assert_eq!(
        (
            record.effective_ms,
            record.logical_end_ms,
            record.interrupted
        ),
        (1000, 30000, true)
    );
    assert!(
        engine
            .output()
            .cues
            .iter()
            .any(|cue| cue.cue_id.contains("partial_door"))
    );
    let (mut engine, mut player) = room(1, 3);
    play(&mut engine, &mut player, "open_drawer");
    play(&mut engine, &mut player, "take_keychain");
    at(&mut engine, &mut player, 28000);
    play(&mut engine, &mut player, "unlock_door_key2"); // exact 30s is authored failure
    assert_eq!(engine.globals()["door_open"], Value::Bool(false));
    assert!(engine.is_terminal());
    assert!(!engine.action_history().last().unwrap().interrupted);
}

#[test]
fn alternate_tool_window_failure_and_timer_extension_are_authored() {
    let (mut engine, mut player) = room(1, 3);
    play(&mut engine, &mut player, "carefully_look_around");
    play(&mut engine, &mut player, "search_fireplace");
    play(&mut engine, &mut player, "use_iron_bar");
    assert!(engine.is_terminal());
    assert_eq!(engine.globals()["has_iron_bar"], Value::Bool(true));
    for dex in [1, 3] {
        let (mut engine, mut player) = room(1, dex);
        play(&mut engine, &mut player, "open_wardrobe");
        let cues = play(&mut engine, &mut player, "escape_window");
        assert!(engine.is_terminal());
        assert!(cues.iter().any(|id| id.contains(if dex == 1 {
            "window_failure"
        } else {
            "window_success"
        })));
    }
    let (mut engine, mut player) = room(1, 3);
    play(&mut engine, &mut player, "open_wardrobe");
    play(&mut engine, &mut player, "buy_time");
    assert_eq!(player.clock().unwrap().deadline_ms, Some(35000));
    assert_eq!(player.clock().unwrap().elapsed_ms, 3000);
    assert_eq!(engine.output().clock.deadline_ms, Some(35000));
    assert!(!player.actions().contains(&"locked_room.buy_time".into()));
}

fn run_cli(script: &str) -> std::process::Output {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let mut child = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .args([
            "play-scene",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../examples/locked-room/story.yaml"
            ),
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../examples/locked-room/text.yaml"
            ),
            "--manual-clock",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn actual_cli_plays_keys_escape_and_deadline_failure_without_real_sleeps() {
    let success = run_cli(
        "\n:time 2000\n@locked_room.open_drawer\n\n:time 5000\n@locked_room.take_keychain\n\n:time 8000\n@locked_room.unlock_door_key1\n\n:time 12000\n@locked_room.unlock_door_key2\n\n\n",
    );
    assert!(
        success.status.success(),
        "{}",
        String::from_utf8_lossy(&success.stderr)
    );
    let text = String::from_utf8(success.stdout).unwrap();
    assert!(text.contains("ESCAPED: You reach the road"), "{text}");
    let failure = run_cli("\n:time 29500\n@locked_room.open_drawer\n\n\n");
    assert!(
        failure.status.success(),
        "{}",
        String::from_utf8_lossy(&failure.stderr)
    );
    let text = String::from_utf8(failure.stdout).unwrap();
    assert!(text.contains("CAUGHT: Johnny"), "{text}");
    assert!(!text.contains("ESCAPED:"));
    assert!(!text.contains("door_damage"));
}

#[test]
fn untimed_action_order_and_explicit_reentry_preserve_globals_reset_locals() {
    let yaml = include_str!("../../../examples/locked-room/detective.yaml");
    for examine_first in [false, true] {
        let mut engine = parse(yaml);
        let mut player = CuePlayer::default();
        player.replace(adapter::present(&engine.output()));
        settle(&mut engine, &mut player);
        assert!(engine.output().clock.deadline_ms.is_none());
        if examine_first {
            select(&mut engine, &mut player, "crime_scene.examine_body");
            settle(&mut engine, &mut player);
        }
        select(&mut engine, &mut player, "crime_scene.search_crime_scene");
        let cues = settle(&mut engine, &mut player);
        assert_eq!(
            player
                .actions()
                .contains(&"crime_scene.confront_suspect".into()),
            !examine_first
        );
        assert_eq!(
            player
                .actions()
                .contains(&"crime_scene.find_suspect_footprints".into()),
            examine_first
        );
        assert!(cues.iter().any(|id| id.contains(if examine_first {
            "footprints_found"
        } else {
            "suspect_found"
        })));
        if !examine_first {
            select(&mut engine, &mut player, "crime_scene.examine_body");
            settle(&mut engine, &mut player);
        }
        select(&mut engine, &mut player, "crime_scene.leave");
        settle(&mut engine, &mut player);
        select(&mut engine, &mut player, "hall.return_to_scene");
        settle(&mut engine, &mut player);
        assert_eq!(engine.globals()["suspect_present"], Value::Bool(false));
        assert_eq!(engine.locals()["confront"], Value::Bool(false));
        assert_eq!(engine.locals()["footprints"], Value::Bool(false));
        assert!(
            !player
                .actions()
                .contains(&"crime_scene.confront_suspect".into())
        );
    }
}

#[test]
fn cue_acks_are_idempotent_and_old_callbacks_only_complete_retained_running_media() {
    use grimm_semantic::scene::{CueEvent as Event, CueInput, SceneAction};
    let mut engine = parse(OVERLAP);
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    at(&mut engine, &mut player, 8000);
    select(&mut engine, &mut player, "room.open_drawer");
    pump(&mut engine, &mut player);
    let old_revision = player.revision();
    let started = CueInput {
        revision: old_revision,
        cue_id: "room.drawer_opening__run_1".into(),
        scene_time_ms: 8000,
        event: Event::Started,
    };
    let before = engine.save();
    engine.cue_input(&started).unwrap();
    assert_eq!(engine.save(), before);
    at(&mut engine, &mut player, 11000);
    pump(&mut engine, &mut player);
    select(&mut engine, &mut player, "room.search_desk");
    let before = engine.save();
    assert!(
        engine
            .apply(&SceneAction {
                revision: old_revision,
                action_id: "room.hide".into(),
                scene_time_ms: 11000
            })
            .is_err()
    );
    assert_eq!(engine.save(), before);
    let completion = CueInput {
        scene_time_ms: 11000,
        event: Event::Completed,
        ..started.clone()
    };
    engine.cue_input(&completion).unwrap(); // Old sheet, same running identity.
    let completed = engine.save();
    engine.cue_input(&completion).unwrap();
    assert_eq!(completed, engine.save());
    // An old scheduled offer instruction cannot reactivate the prior set.
    assert!(
        engine
            .cue_input(&CueInput {
                revision: old_revision,
                cue_id: "room.warning_choices".into(),
                scene_time_ms: 11000,
                event: Event::Started
            })
            .is_err()
    );
    let snapshot = engine.save();
    let expected_globals = engine.globals().clone();
    let expected_locals = engine.locals().clone();
    let expected_records = engine.action_history().to_vec();
    let mut restored = parse(OVERLAP);
    restored
        .restore(serde_json::from_str(&serde_json::to_string(&snapshot).unwrap()).unwrap())
        .unwrap();
    assert_eq!(restored.globals(), &expected_globals);
    assert_eq!(restored.locals(), &expected_locals);
    assert_eq!(restored.action_history(), expected_records);
    assert_eq!(restored.output().clock, engine.output().clock);
    assert!(restored.output().revision > engine.output().revision);
    assert!(restored.cue_input(&completion).is_err());
    let saved_again = restored.save();
    restored.restore(saved_again).unwrap(); // Restore epochs themselves replay.
    assert_eq!(restored.locals(), &expected_locals);
}

#[test]
fn generated_cue_identity_depends_on_content_and_timing_not_list_position() {
    use grimm_semantic::scene::{AuthoredCue, AuthoredInstruction};
    let mut story: SceneStory = serde_yaml_ng::from_str(OVERLAP).unwrap();
    story.contexts[0].cues[0].cue_id = None;
    let original = SceneEngine::new(story.clone()).unwrap().output().cues[0]
        .cue_id
        .clone();
    story.contexts[0].cues.insert(
        0,
        AuthoredCue {
            cue_id: Some("another_intro".into()),
            at_ms: None,
            when: None,
            instruction: AuthoredInstruction::Narrative {
                narrative_id: "another_intro".into(),
            },
        },
    );
    let moved = SceneEngine::new(story.clone()).unwrap().output().cues[1]
        .cue_id
        .clone();
    assert_eq!(original, moved);
    assert!(original.starts_with("room.cue_"));
    story.contexts[0].cues[1].at_ms = Some(1000);
    assert_ne!(
        original,
        SceneEngine::new(story).unwrap().output().cues[1].cue_id
    );
    let mut repeated: SceneStory = serde_yaml_ng::from_str(OVERLAP).unwrap();
    repeated.contexts[0].cues[0].cue_id = None;
    let duplicate = repeated.contexts[0].cues[0].clone();
    repeated.contexts[0].cues.insert(1, duplicate);
    assert!(
        SceneEngine::new(repeated)
            .unwrap_err()
            .to_string()
            .contains("colliding cue")
    );
    let mut same_content: SceneStory = serde_yaml_ng::from_str(OVERLAP).unwrap();
    let mut repeated = same_content.contexts[0].cues[0].clone();
    repeated.cue_id = Some("second_intro".into());
    same_content.contexts[0].cues.insert(1, repeated);
    assert!(SceneEngine::new(same_content).is_ok()); // Shared narrative_id is legal.
}

#[test]
fn sequential_release_waits_for_both_logical_time_and_presentation_completion() {
    let yaml = OVERLAP.replace(
        "cue_id: drawer_new_choices, at_ms: 0",
        "cue_id: drawer_new_choices",
    );
    let mut engine = parse(&yaml);
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    at(&mut engine, &mut player, 1000);
    select(&mut engine, &mut player, "room.open_drawer");
    pump(&mut engine, &mut player);
    // Finishing short media early does not consume the remaining authored cost.
    complete(&mut engine, &mut player, "room.drawer_opening__run_1");
    pump(&mut engine, &mut player);
    assert!(player.actions().is_empty());
    at(&mut engine, &mut player, 4000);
    pump(&mut engine, &mut player);
    assert!(!player.actions().is_empty());
    let mut engine = parse(&yaml);
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    at(&mut engine, &mut player, 1000);
    select(&mut engine, &mut player, "room.open_drawer");
    pump(&mut engine, &mut player);
    at(&mut engine, &mut player, 4000);
    pump(&mut engine, &mut player);
    assert!(player.actions().is_empty()); // Conversely media can finish late.
    complete(&mut engine, &mut player, "room.drawer_opening__run_1");
    pump(&mut engine, &mut player);
    assert!(!player.actions().is_empty());
}

#[test]
fn five_second_action_at_six_is_resolved_as_four_seconds_at_ten_without_more_input() {
    let mut story: SceneStory = serde_yaml_ng::from_str(OVERLAP).unwrap();
    story.globals.insert("attempted_ms".into(), Value::Int(0));
    if let grimm_semantic::scene::AuthoredInstruction::Timer { start_ms, .. } =
        &mut story.contexts[0].cues[1].instruction
    {
        *start_ms = Some(10000);
    }
    story.contexts[0].actions[0].duration_ms = 5000;
    story.contexts[0].actions[0].interrupted_outcomes = serde_yaml_ng::from_str(
        r#"
- effects: [{op: set, var: global.attempted_ms, value: {clock: effective_ms}}]
  goto: caught
  cues: [{cue_id: attempted, kind: narrative, narrative_id: four_second_attempt_cut_short}]
"#,
    )
    .unwrap();
    let mut engine = SceneEngine::new(story).unwrap();
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    at(&mut engine, &mut player, 6000);
    select(&mut engine, &mut player, "room.open_drawer");
    assert!(engine.is_terminal());
    assert_eq!(engine.globals()["attempted_ms"], Value::Int(4000));
    let result = engine.action_history().last().unwrap();
    assert_eq!(
        (
            result.selected_ms,
            result.effective_ms,
            result.logical_end_ms
        ),
        (6000, 4000, 10000)
    );
    assert!(result.interrupted);
    assert!(engine.output().cues.iter().any(|cue| matches!(&cue.instruction, grimm_semantic::scene::Instruction::Narrative { narrative_id } if narrative_id == "room.four_second_attempt_cut_short")));
}

#[test]
fn parsing_and_basic_compilation_reject_bad_rules_ids_timers_and_references() {
    use grimm_semantic::scene::{Expr, Test, Update};
    let story: SceneStory = serde_yaml_ng::from_str(OVERLAP).unwrap();
    let mut bad = vec![];
    let mut s = story.clone();
    s.start = "missing".into();
    bad.push((s, "start"));
    let mut s = story.clone();
    s.contexts[0].actions[2].outcomes[0].outcome.goto = Some("missing".into());
    bad.push((s, "unknown transition"));
    let mut s = story.clone();
    s.contexts[0].actions[0].outcomes.clear();
    bad.push((s, "final ELSE"));
    let mut s = story.clone();
    s.contexts[0].actions[0].outcomes[0].when = Some(Test::Eq {
        left: Expr::Literal(Value::Int(1)),
        right: Expr::Literal(Value::Int(1)),
    });
    bad.push((s, "final ELSE"));
    let mut s = story.clone();
    s.contexts[0].actions[1].id = "Open Drawer".into();
    bad.push((s, "duplicate"));
    let mut s = story.clone();
    s.contexts[0].actions[1].id = "äpple".into();
    bad.push((s, "ASCII"));
    let mut s = story.clone();
    s.contexts[0].cues[1].cue_id = Some("INTRO".into());
    bad.push((s, "colliding cue"));
    let mut s = story.clone();
    s.contexts[0].cues[0].cue_id = Some("123bad".into());
    bad.push((s, "invalid"));
    let mut s = story.clone();
    s.contexts[0].cues[0].cue_id = Some("__grimm_reserved".into());
    bad.push((s, "reserved"));
    let mut s = story.clone();
    s.contexts[0].cues[1].at_ms = Some(1000);
    bad.push((s, "initial timer"));
    let mut s = story.clone();
    s.contexts[0].timeout = None;
    bad.push((s, "requires an authored timeout"));
    let mut s = story.clone();
    s.contexts[0].actions[0].effects.push(Update::Set {
        var: "local.drawer_open".into(),
        value: Expr::Literal(Value::Int(1)),
    });
    bad.push((s, "wrong type"));
    let mut s = story.clone();
    s.contexts[0].actions[0].when = Some(Test::Eq {
        left: Expr::Variable {
            var: "local.missing".into(),
        },
        right: Expr::Literal(Value::Bool(true)),
    });
    bad.push((s, "undeclared"));
    let mut s = story.clone();
    if let grimm_semantic::scene::AuthoredInstruction::Actions { available_actions } =
        &mut s.contexts[0].cues[2].instruction
    {
        *available_actions = grimm_semantic::scene::ActionSet::Only(vec!["missing".into()]);
    }
    bad.push((s, "unknown offered"));
    for (story, message) in bad {
        let error = SceneEngine::new(story).unwrap_err().to_string();
        assert!(error.contains(message), "expected {message}: {error}");
    }
    assert!(
        grimm_cli::scene_runtime::parse(
            "bad.scene.yaml",
            &OVERLAP.replace("duration_ms: 3000", "duration_ms: -1")
        )
        .unwrap_err()
        .to_string()
        .contains("bad.scene.yaml")
    );
    assert!(
        serde_yaml_ng::from_str::<SceneStory>(&OVERLAP.replace(
            "narrative_id: search_room_start",
            "narrative_id: search_room_start, typo: true"
        ))
        .is_err()
    );
    assert!(
        serde_yaml_ng::from_str::<SceneStory>(&OVERLAP.replace(
            "locals: {drawer_open: false}",
            "locals: {drawer_open: false, drawer_open: true}"
        ))
        .is_err()
    );
    assert!(
        serde_yaml_ng::from_str::<SceneStory>(&OVERLAP.replace(
            "{var: local.drawer_open}",
            "{var: local.drawer_open, typo: 1}"
        ))
        .is_err()
    );
}

#[test]
fn rule_overflow_invalid_time_and_invalid_checkpoint_leave_entire_state_unchanged() {
    use grimm_semantic::scene::{Expr, SceneAction, Update};
    let mut story: SceneStory = serde_yaml_ng::from_str(OVERLAP).unwrap();
    story.globals.insert("counter".into(), Value::Int(i64::MAX));
    story.contexts[0].actions[0].effects.push(Update::Add {
        var: "global.counter".into(),
        value: Expr::Literal(Value::Int(1)),
    });
    let mut engine = SceneEngine::new(story).unwrap();
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    let before = engine.save();
    let sheet = engine.output();
    let error = engine
        .apply(&SceneAction {
            revision: player.revision(),
            action_id: "room.open_drawer".into(),
            scene_time_ms: 0,
        })
        .unwrap_err();
    assert!(error.to_string().contains("overflow"));
    assert_eq!(engine.save(), before);
    assert_eq!(engine.output(), sheet);
    assert_eq!(engine.locals()["drawer_open"], Value::Bool(false));
    let mut bad = before.clone();
    bad.story_version += 1;
    assert!(engine.restore(bad).is_err());
    let mut bad = before.clone();
    bad.revision += 1;
    assert!(engine.restore(bad).is_err());
    assert_eq!(engine.save(), before);
    at(&mut engine, &mut player, 1000);
    let before = engine.save();
    assert!(engine.observe(player.revision(), 999).is_err());
    assert_eq!(engine.save(), before);
    // At idle deadline, the host time observation chooses the authored timeout.
    at(&mut engine, &mut player, 30000);
    assert!(engine.is_terminal());
    assert_eq!(engine.action_history().last().unwrap().action_id, "timeout");
}

#[test]
fn cue_sheets_and_replay_checkpoints_round_trip_without_private_projections() {
    use grimm_semantic::scene::CueSheet;
    let (mut a, mut pa) = room(1, 3);
    let (mut b, mut pb) = room(1, 3);
    for id in ["open_drawer", "take_keychain", "unlock_door_key1"] {
        play(&mut a, &mut pa, id);
        play(&mut b, &mut pb, id);
        assert_eq!(a.save(), b.save());
        assert_eq!(a.output(), b.output());
    }
    let sheet = a.output();
    let json = serde_json::to_string(&sheet).unwrap();
    assert_eq!(sheet, serde_json::from_str::<CueSheet>(&json).unwrap());
    let value = serde_json::to_value(&sheet).unwrap();
    assert_eq!(
        value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["clock", "cues", "revision"]
    );
    for private in [
        "door_damage",
        "drawer_open",
        "has_keychain",
        "world_state",
        "contexts",
        "terminal",
    ] {
        assert!(!json.contains(private), "leaked {private}: {json}");
    }
    let mut loaded = parse(LOCKED_ROOM);
    loaded
        .restore(serde_json::from_str(&serde_json::to_string(&a.save()).unwrap()).unwrap())
        .unwrap();
    assert_eq!(loaded.globals(), a.globals());
    assert_eq!(loaded.locals(), a.locals());
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&loaded.output()));
    settle(&mut loaded, &mut player);
    play(&mut loaded, &mut player, "unlock_door_key2");
    assert!(loaded.is_terminal());
    assert_eq!(loaded.globals()["door_open"], Value::Bool(true));
}

#[test]
fn idle_clock_samples_coalesce_without_changing_checkpoint_reconstruction() {
    let (mut engine, mut player) = room(1, 3);
    let before = engine.save().history.len();
    for time in 1..=1000 {
        at(&mut engine, &mut player, time);
    }
    assert_eq!(engine.save().history.len(), before + 1);
    let mut restored = parse(LOCKED_ROOM);
    restored.restore(engine.save()).unwrap();
    assert_eq!(restored.output().clock.elapsed_ms, 1000);
    assert_eq!(restored.globals(), engine.globals());
    assert_eq!(restored.locals(), engine.locals());
}

#[test]
fn untimed_narration_after_logically_delayed_actions_keeps_authored_order() {
    let yaml = OVERLAP.replace(
        "{cue_id: drawer_new_choices, at_ms: 0, kind: actions, available_actions: eligible}",
        "{cue_id: drawer_new_choices, kind: actions, available_actions: eligible}\n              - {cue_id: after_choices, kind: narrative, narrative_id: choices_are_ready}",
    );
    let mut engine = parse(&yaml);
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    at(&mut engine, &mut player, 1000);
    select(&mut engine, &mut player, "room.open_drawer");
    pump(&mut engine, &mut player);
    complete(&mut engine, &mut player, "room.drawer_opening__run_1");
    assert!(pump(&mut engine, &mut player).is_empty());
    at(&mut engine, &mut player, 4000);
    let started = pump(&mut engine, &mut player);
    let actions = started
        .iter()
        .position(|id| id == "room.drawer_new_choices__run_1")
        .unwrap();
    let narration = started
        .iter()
        .position(|id| id == "room.after_choices__run_1")
        .unwrap();
    assert!(actions < narration);
}

#[test]
fn actual_cli_save_load_continues_with_keys_without_replaying_committed_actions() {
    let path = std::env::temp_dir().join(format!("grimm-m2-save-{}.json", std::process::id()));
    let first = run_cli(&format!(
        "\n:time 2000\n@locked_room.open_drawer\n\n:time 5000\n@locked_room.take_keychain\n\n:time 6000\n:save {}\n:quit\n",
        path.display()
    ));
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(path.exists(), "{}", String::from_utf8_lossy(&first.stdout));
    let continued = run_cli(&format!(
        ":load {}\n@locked_room.unlock_door_key2\n\n\n",
        path.display()
    ));
    std::fs::remove_file(&path).unwrap();
    assert!(
        continued.status.success(),
        "{}",
        String::from_utf8_lossy(&continued.stderr)
    );
    let text = String::from_utf8(continued.stdout).unwrap();
    assert!(text.contains("ESCAPED: You reach the road"), "{text}");
    assert!(
        !text.contains("You pocket the keychain"),
        "committed media replayed: {text}"
    );
}

#[test]
fn timeout_transition_cannot_offer_source_context_actions_in_its_destination() {
    let yaml = OVERLAP.replace(
        "cues: [{cue_id: timeout, kind: narrative, narrative_id: caught_at_desk}]",
        "cues: [{cue_id: timeout, kind: actions, available_actions: eligible}]",
    );
    let story = serde_yaml_ng::from_str::<SceneStory>(&yaml).unwrap();
    let error = SceneEngine::new(story).err().unwrap().to_string();
    assert!(error.contains("timeout transition"), "{error}");
}

#[test]
fn transition_to_interactive_context_preserves_action_duration_on_the_new_clock() {
    let yaml = OVERLAP.replace(
        "- id: hide\n        outcomes:",
        "- id: hide\n        duration_ms: 3000\n        outcomes:",
    ).replace(
        "- id: escaped\n    terminal: true\n    cues: [{cue_id: escape_text, kind: narrative, narrative_id: escaped}]",
        "- id: escaped\n    cues:\n      - {cue_id: escape_text, kind: narrative, narrative_id: escaped}\n      - {cue_id: new_choices, kind: actions, available_actions: eligible}\n    actions:\n      - id: finish\n        outcomes: [{goto: caught}]",
    );
    let mut engine = parse(&yaml);
    let mut player = CuePlayer::default();
    player.replace(adapter::present(&engine.output()));
    settle(&mut engine, &mut player);
    at(&mut engine, &mut player, 8000);
    select(&mut engine, &mut player, "room.open_drawer");
    at(&mut engine, &mut player, 11000);
    settle(&mut engine, &mut player);
    select(&mut engine, &mut player, "room.hide");
    assert_eq!(player.clock().unwrap().elapsed_ms, 0);
    settle(&mut engine, &mut player);
    assert!(player.actions().is_empty());
    at(&mut engine, &mut player, 2999);
    settle(&mut engine, &mut player);
    assert!(player.actions().is_empty());
    at(&mut engine, &mut player, 3000);
    settle(&mut engine, &mut player);
    assert_eq!(player.actions(), ["escaped.finish"]);
}
