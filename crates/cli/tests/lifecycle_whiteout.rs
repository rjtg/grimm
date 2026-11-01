//! Canonical WHITEOUT in the opted-in lifecycle, using a fake renderer clock.
use grimm_cli::scene_session::SceneSession;
use grimm_engine::scene::{SceneEngine, lifecycle::LifecycleEngine};
use grimm_presentation::lifecycle::ScenePhase;
use grimm_semantic::story::Value;
const STORY: &str = include_str!("../../../examples/whiteout/story.yaml");
const CONFIG: &str = include_str!("../../../examples/whiteout/scenes.yaml");
fn session(lifecycle: bool) -> SceneSession {
    if lifecycle {
        SceneSession::new_lifecycle(
            LifecycleEngine::new(
                serde_yaml_ng::from_str(STORY).unwrap(),
                serde_yaml_ng::from_str(CONFIG).unwrap(),
            )
            .unwrap(),
            false,
        )
        .unwrap()
    } else {
        SceneSession::new(
            SceneEngine::new(serde_yaml_ng::from_str(STORY).unwrap()).unwrap(),
            false,
        )
        .unwrap()
    }
}
fn settle(host: &mut SceneSession) {
    for _ in 0..256 {
        match host.scene().map(|s| s.phase) {
            Some(ScenePhase::Prepare | ScenePhase::Stable) if !host.is_complete() => {
                // A real host prepares its resource cache here; registry tests cover it.
                host.preparation_scene().unwrap();
                host.begin_prepared_scene().unwrap();
            }
            _ => {}
        }
        if let Some(id) = host.player().running_narratives().first().cloned() {
            let time = host.player().clock().unwrap().elapsed_ms;
            host.complete_text(&id, host.player().revision(), time)
                .unwrap();
            continue;
        }
        if !host.player().actions().is_empty() || host.is_complete() {
            return;
        }
        let next = host
            .player()
            .next_start_ms()
            .expect("authored continuation");
        host.advance_to(next).unwrap();
    }
    panic!("settling did not finish");
}
#[test]
fn all_whiteout_endings_keep_identical_canonical_state_and_action_costs() {
    for (input, expected) in [
        (
            include_str!("../../../examples/whiteout/playthroughs/full-rescue.input"),
            "full_rescue",
        ),
        (
            include_str!("../../../examples/whiteout/playthroughs/partial-survival.input"),
            "partial_survival",
        ),
        (
            include_str!("../../../examples/whiteout/playthroughs/station-lost.input"),
            "station_lost",
        ),
    ] {
        let mut old = session(false);
        let mut new = session(true);
        settle(&mut old);
        settle(&mut new);
        for action in input.lines().filter_map(|l| l.strip_prefix('@')) {
            for host in [&mut old, &mut new] {
                let at = host.player().clock().unwrap().elapsed_ms;
                host.choose(action, host.player().revision(), at).unwrap();
                settle(host);
            }
            assert_eq!(new.engine().globals(), old.engine().globals(), "{action}");
            assert_eq!(new.engine().locals(), old.engine().locals(), "{action}");
            assert_eq!(
                new.engine().action_history(),
                old.engine().action_history(),
                "{action}"
            );
        }
        assert!(new.is_complete());
        assert_eq!(
            new.engine().globals()["ending"],
            Value::Text(expected.into())
        );
        let save = new.save_json().unwrap();
        let globals = new.engine().globals().clone();
        new.restore_json(&save).unwrap();
        assert!(new.is_complete());
        assert_eq!(new.engine().globals(), &globals);
        assert!(new.player().running_narratives().is_empty());
    }
}

#[test]
fn actual_lifecycle_cli_plays_whiteout_to_all_three_endings() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    for (script, ending) in [
        (
            include_str!("../../../examples/whiteout/playthroughs/full-rescue.input"),
            "ENDING / FOUR NAMES ON THE MANIFEST",
        ),
        (
            include_str!("../../../examples/whiteout/playthroughs/partial-survival.input"),
            "ENDING / THE EMPTY PLACE",
        ),
        (
            include_str!("../../../examples/whiteout/playthroughs/station-lost.input"),
            "ENDING / WHITEOUT",
        ),
    ] {
        // Existing authored decisions/times are unchanged. Explicitly continue
        // through the new persistent boundaries before setting each scene time.
        let script = script.replace(":time ", ":continue\n\n\n\n\n\n\n:time ");
        let mut child = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
            .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
            .args([
                "play-scene",
                "examples/whiteout/story.yaml",
                "examples/whiteout/text.yaml",
                "--lifecycle",
                "examples/whiteout/scenes.yaml",
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
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        assert!(text.lines().any(|line| line == ending), "{text}");
        for unwanted in ["Action rejected", "cold_exposure", "global."] {
            assert!(!text.contains(unwanted), "{unwanted}: {text}");
        }
    }
}

#[test]
fn lifecycle_replacement_retains_running_warning_once_and_cancels_obsolete_schedule() {
    let mut host = session(true);
    settle(&mut host);
    for action in [
        "blackout.begin",
        "stores.take_thermal_liner",
        "stores.jonas_treated",
        "stores.take_copper_braid",
        "stores.take_ceramic_mug",
        "stores.take_spanner",
        "stores.take_heat_pack",
        "stores.log_read",
        "stores.take_safety_line",
        "stores.leave",
        "dispatch.engineering",
    ] {
        let at = host.player().clock().unwrap().elapsed_ms;
        host.choose(action, host.player().revision(), at).unwrap();
        settle(&mut host);
    }
    host.take_started_text();
    host.choose(
        "generator.open_generator_latch",
        host.player().revision(),
        8000,
    )
    .unwrap();
    let latch = host.player().running_narratives().first().unwrap().clone();
    host.advance_to(10000).unwrap();
    assert!(host.player().running_narratives().contains(&latch));
    assert_eq!(host.player().clock().unwrap().deadline_ms, Some(30000));
    assert!(host.player().clock().unwrap().visible);
    let warning = host
        .player()
        .running_narratives()
        .into_iter()
        .find(|id| id.contains("radio_warning"))
        .unwrap();
    host.complete_text(&warning, host.player().revision(), 10000)
        .unwrap();
    host.advance_to(11000).unwrap();
    host.choose("generator.isolate_deicer", host.player().revision(), 12000)
        .unwrap();
    assert!(host.player().running_narratives().contains(&latch));
    assert!(
        !host
            .player()
            .sheet()
            .unwrap()
            .cues
            .iter()
            .any(|cue| cue.cue_id == warning)
    );
    assert_eq!(host.player().clock().unwrap().deadline_ms, Some(30000));
    let started = host.take_started_text();
    assert_eq!(
        started.iter().filter(|n| n.cue.cue_id == warning).count(),
        1
    );
    assert_eq!(started.iter().filter(|n| n.cue.cue_id == latch).count(), 1);
    let isolated = host
        .player()
        .running_narratives()
        .into_iter()
        .find(|id| id.contains(".isolated"))
        .unwrap();
    host.complete_text(&isolated, host.player().revision(), 12000)
        .unwrap();
    host.choose("generator.abandon", host.player().revision(), 14000)
        .unwrap();
    settle(&mut host);
    assert!(
        !host
            .player()
            .sheet()
            .unwrap()
            .cues
            .iter()
            .any(|cue| cue.cue_id.starts_with("generator."))
    );
    assert!(!host.player().running_narratives().contains(&latch));
}
