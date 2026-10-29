use grimm_cli::{parse_story, read_save, write_save};
use grimm_engine::{Engine, PlayabilityLimits};
use grimm_semantic::{
    GameAction,
    save::SaveGame,
    story::{Story, Value},
    time::{DecisionPhase, TimeEvent, TimeInput},
};
use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, Receiver},
    },
    thread,
    time::{Duration, Instant},
};

const YAML: &str = include_str!("../../../stories/the-last-train.yaml");
const PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../stories/the-last-train.yaml"
);

fn game() -> Engine {
    parse_story("train.yaml", YAML).unwrap().1
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
    if engine
        .output()
        .decision
        .as_ref()
        .is_some_and(|decision| decision.phase == DecisionPhase::AwaitingReady)
    {
        time(engine, TimeEvent::Ready);
        time(engine, TimeEvent::Elapsed { total_ms: 1 });
    }
    let output = engine.output();
    assert!(
        output.offers.iter().any(|offer| offer.id == id),
        "{id} not offered at {}",
        engine.save().node_id
    );
    engine
        .apply(&GameAction {
            revision: output.revision,
            choice_id: id.into(),
        })
        .unwrap();
}
fn route(engine: &mut Engine, ids: &[&str]) {
    for id in ids {
        choose(engine, id);
    }
}
fn expire(engine: &mut Engine) {
    let remaining = engine.output().decision.unwrap().remaining_ms;
    time(engine, TimeEvent::Ready);
    time(
        engine,
        TimeEvent::Elapsed {
            total_ms: remaining,
        },
    );
}

#[test]
fn full_routes_cover_every_ending_and_both_fictional_time_windows() {
    let routes = [
        (
            vec![
                "answer-phone",
                "reassure-rowan",
                "search-tower",
                "hurry-to-fuse",
                "fit-fuse",
                "controlled-stop",
            ],
            "stopped",
            17,
        ),
        (
            vec![
                "answer-phone",
                "reassure-rowan",
                "take-tunnel",
                "restore-circuit",
                "verify-log",
                "controlled-stop",
            ],
            "stopped",
            9,
        ),
        (
            vec![
                "answer-phone",
                "doubt-call",
                "investigate-alone",
                "inspect-siding",
                "follow-wire",
                "isolate-transmitter",
                "controlled-stop",
            ],
            "stopped",
            9,
        ),
        (
            vec![
                "answer-phone",
                "reassure-rowan",
                "search-tower",
                "document-every-wire",
                "fit-fuse",
                "clear-platform",
            ],
            "evacuation",
            21,
        ),
        (
            vec![
                "answer-phone",
                "reassure-rowan",
                "run-to-platform",
                "trust-green",
            ],
            "disaster",
            2,
        ),
    ];
    for (ids, ending, minutes) in routes {
        let mut engine = game();
        route(&mut engine, &ids);
        assert_eq!(engine.output().ending.unwrap().id, ending);
        assert!(engine.output().decision.is_none());
        assert_eq!(engine.save().temporal.unwrap().world_minutes, minutes);
    }
    for (investigation, stop_available) in [("hurry-to-fuse", true), ("document-every-wire", false)]
    {
        let mut engine = game();
        route(
            &mut engine,
            &[
                "answer-phone",
                "reassure-rowan",
                "search-tower",
                investigation,
                "fit-fuse",
            ],
        );
        assert_eq!(engine.save().variables["evidence"], Value::Bool(true));
        assert_eq!(engine.save().variables["prepared"], Value::Bool(true));
        assert_eq!(
            engine
                .output()
                .offers
                .iter()
                .any(|offer| offer.id == "controlled-stop"),
            stop_available
        );
    }
}

#[test]
fn unanswered_call_and_platform_have_authored_consequences_without_player_input() {
    let mut engine = game();
    choose(&mut engine, "answer-phone");
    expire(&mut engine);
    assert_eq!(engine.save().node_id, "call_lost");
    assert_eq!(engine.save().variables["rowan_trust"], Value::Bool(false));
    route(
        &mut engine,
        &["investigate-alone", "take-tunnel", "restore-circuit"],
    );
    assert!(
        !engine
            .output()
            .offers
            .iter()
            .any(|offer| offer.id == "verify-log")
    );
    choose(&mut engine, "proceed-without-log");
    expire(&mut engine);
    assert_eq!(engine.output().ending.unwrap().id, "disaster");
    let mut engine = game();
    choose(&mut engine, "answer-phone");
    expire(&mut engine);
    choose(&mut engine, "evacuate-without-proof");
    assert_eq!(engine.output().ending.unwrap().id, "evacuation");
}

#[test]
fn train_pause_save_load_continuation_and_private_projection_are_consistent() {
    let temp = Temp::new();
    for paused in [false, true] {
        let path = temp.file("pending.json");
        let mut engine = game();
        choose(&mut engine, "answer-phone");
        time(&mut engine, TimeEvent::Ready);
        time(&mut engine, TimeEvent::Elapsed { total_ms: 5000 });
        if paused {
            time(&mut engine, TimeEvent::Pause);
        }
        write_save(&path, &engine).unwrap();
        let saved = read_save(&path).unwrap();
        assert_eq!(saved.schema_version, 2);
        assert_eq!(
            saved
                .temporal
                .as_ref()
                .unwrap()
                .decision
                .as_ref()
                .unwrap()
                .remaining_ms,
            10000
        );
        let mut restored = game();
        restored.restore(saved).unwrap();
        assert_eq!(
            restored.output().decision.as_ref().unwrap().phase,
            if paused {
                DecisionPhase::Paused
            } else {
                DecisionPhase::AwaitingReady
            }
        );
        let visible = serde_json::to_value(restored.output()).unwrap();
        assert!(visible.get("temporal").is_none());
        assert!(visible.get("world_minutes").is_none());
        assert!(visible.get("variables").is_none());
        let decision = visible["decision"].as_object().unwrap();
        assert_eq!(decision.len(), 2);
        assert!(!decision.contains_key("timeout"));
        assert!(!decision.contains_key("last_elapsed_ms"));
        if paused {
            time(&mut restored, TimeEvent::Resume);
        }
        route(
            &mut restored,
            &[
                "reassure-rowan",
                "inspect-siding",
                "follow-wire",
                "isolate-transmitter",
                "controlled-stop",
            ],
        );
        assert_eq!(restored.output().ending.unwrap().id, "stopped");
    }
}

#[test]
fn temporal_story_validation_codecs_and_bounded_checker_preserve_legacy_formats() {
    let story: Story = serde_yaml_ng::from_str(YAML).unwrap();
    assert!((10..=15).contains(&story.nodes.len()));
    assert!(
        story
            .nodes
            .iter()
            .filter(|node| node.decision.is_some())
            .count()
            >= 2
    );
    assert!(story.nodes[0].decision.is_none());
    assert_eq!(
        story,
        serde_yaml_ng::from_str(&serde_yaml_ng::to_string(&story).unwrap()).unwrap()
    );
    let engine = game();
    let before = engine.save();
    let report = engine.check_playability(PlayabilityLimits::default());
    assert!(!report.truncated);
    assert!(report.deadlocks.is_empty());
    assert_eq!(
        report.endings.into_iter().collect::<Vec<_>>(),
        ["disaster", "evacuation", "stopped"]
    );
    assert_eq!(engine.save(), before);
    for changed in [
        YAML.replace("limit_ms: 15000", "limit_ms: -1"),
        YAML.replace("advance_minutes: 15", "advance_minutes: -1"),
        YAML.replace("timeout: { target: disaster }", "timeout: {}"),
        YAML.replace(
            "target: call_lost\n        effects",
            "target: missing\n        effects",
        ),
    ] {
        assert!(parse_story("invalid.yaml", &changed).is_err());
    }
    // Literal old JSON has no temporal fields, not merely a newly constructed DTO.
    let mut legacy = Engine::default();
    let json = r#"{"schema_version":1,"story_id":"m0-demo","story_version":1,"node_id":"start","variables":{},"revision":0}"#;
    let old: SaveGame = serde_json::from_str(json).unwrap();
    legacy.restore(old).unwrap();
    let serialized = serde_json::to_value(legacy.save()).unwrap();
    assert_eq!(serialized["schema_version"], 1);
    assert!(serialized.get("temporal").is_none());
    assert!(
        serde_json::to_value(legacy.output())
            .unwrap()
            .get("decision")
            .is_none()
    );
    for event in [
        TimeEvent::Ready,
        TimeEvent::Elapsed { total_ms: 1 },
        TimeEvent::Pause,
        TimeEvent::Resume,
        TimeEvent::AdvanceWorld { minutes: 2 },
    ] {
        let input = TimeInput { revision: 1, event };
        assert_eq!(
            input,
            serde_json::from_str(&serde_json::to_string(&input).unwrap()).unwrap()
        );
    }
}

struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "grimm-train-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Paced actual-CLI driver; core tests never use this clock or sleep.
struct Process {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout: Receiver<Vec<u8>>,
    text: String,
    cursor: usize,
}
impl Process {
    fn start(args: &[&str]) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take();
        let mut output = child.stdout.take().unwrap();
        let (sender, stdout) = mpsc::channel();
        thread::spawn(move || {
            let mut buffer = [0; 1024];
            while let Ok(count) = output.read(&mut buffer) {
                if count == 0 {
                    break;
                }
                if sender.send(buffer[..count].to_vec()).is_err() {
                    break;
                }
            }
        });
        Self {
            child,
            stdin,
            stdout,
            text: String::new(),
            cursor: 0,
        }
    }
    fn until(&mut self, needle: &str) {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if let Some(index) = self.text[self.cursor..].find(needle) {
                self.cursor += index + needle.len();
                return;
            }
            let remaining = deadline.saturating_duration_since(Instant::now());
            let bytes = self.stdout.recv_timeout(remaining).unwrap_or_else(|error| {
                panic!("waiting for {needle:?}: {error}; transcript: {}", self.text)
            });
            self.text.push_str(&String::from_utf8_lossy(&bytes));
        }
    }
    fn send(&mut self, line: &str) {
        let stdin = self.stdin.as_mut().unwrap();
        writeln!(stdin, "{line}").unwrap();
        stdin.flush().unwrap();
    }
    fn finish(&mut self, ending: &str) {
        self.until(ending);
        let deadline = Instant::now() + Duration::from_secs(5);
        let status = loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                break status;
            }
            assert!(Instant::now() < deadline, "CLI did not exit");
            thread::sleep(Duration::from_millis(10));
        };
        let mut error = String::new();
        self.child
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut error)
            .unwrap();
        assert!(status.success(), "{error}");
        assert!(error.is_empty(), "{error}");
    }
}
impl Drop for Process {
    fn drop(&mut self) {
        if self.child.try_wait().ok().flatten().is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[test]
fn actual_cli_completes_the_train_with_timely_choices_and_no_unseen_skip() {
    let mut process = Process::start(&["play", PATH]);
    for selected in ["1", "1", "2", "1", "1", "1"] {
        process.until("> ");
        process.send(selected);
    }
    process.finish("Ending: A Red Light in Time");
    assert!(process.text.contains("15.0s"));
    assert!(process.text.contains("20.0s"));
    assert!(!process.text.contains("world_minutes"));
    assert!(!process.text.contains("rowan_trust"));
}

fn short_story(ms: u64) -> String {
    format!(
        r#"
schema_version: 2
id: host-timer
version: 1
title: Host timer regression
time: {{}}
start: prompt
nodes:
  - id: prompt
    content: [{{kind: narration, text: Choose before expiry.}}]
    decision:
      limit_ms: {ms}
      timeout: {{target: expired}}
    choices: [{{id: timely, label: Choose in time., target: timely}}]
  - id: timely
    content: [{{kind: narration, text: Accepted}}]
    ending: {{id: timely, title: Timely}}
  - id: expired
    content: [{{kind: narration, text: "Authored timeout, not an I/O error."}}]
    ending: {{id: expired, title: Expired}}
"#
    )
}

#[test]
fn actual_host_expires_with_open_stdin_and_with_eof_without_a_line() {
    let temp = Temp::new();
    let path = temp.file("short.yaml");
    fs::write(&path, short_story(150)).unwrap();
    for close_input in [false, true] {
        let mut process = Process::start(&["play", path.to_str().unwrap()]);
        process.until("> ");
        if close_input {
            process.stdin.take();
        }
        process.finish("Ending: Expired");
        assert!(process.text.contains("Authored timeout"));
        assert!(!process.text.contains("Ending: Timely"));
    }
}

#[test]
fn actual_host_pause_save_load_and_resume_preserve_pending_time() {
    let temp = Temp::new();
    let story = temp.file("pause.yaml");
    let save = temp.file("pending.json");
    let active_file = temp.file("active.json");
    let mut active = parse_story("active", &short_story(2000)).unwrap().1;
    time(&mut active, TimeEvent::Ready);
    time(&mut active, TimeEvent::Elapsed { total_ms: 500 });
    write_save(&active_file, &active).unwrap();
    fs::write(&story, short_story(2000)).unwrap();
    let mut process = Process::start(&["play", story.to_str().unwrap()]);
    process.until("> ");
    process.send(":pause");
    process.until("Paused; time remaining:");
    process.send(&format!(":save {}", save.display()));
    process.until("Saved to");
    let snapshot = read_save(&save).unwrap();
    assert_eq!(
        snapshot
            .temporal
            .as_ref()
            .unwrap()
            .decision
            .as_ref()
            .unwrap()
            .phase,
        DecisionPhase::Paused
    );
    thread::sleep(Duration::from_millis(2100)); // integration only: longer than authored deadline
    process.send(&format!(":load {}", save.display()));
    process.until("Loaded");
    process.until("Paused; time remaining:");
    process.send(":resume");
    process.until("> ");
    process.send("1");
    process.finish("Ending: Timely");
    assert!(!process.text.contains("Ending: Expired"));
    // Start a different process after wall time elapsed; paused checkpoint is frozen.
    let mut loaded = Process::start(&[
        "play",
        story.to_str().unwrap(),
        "--load",
        save.to_str().unwrap(),
    ]);
    loaded.until("Paused; time remaining:");
    loaded.send(":resume");
    loaded.until("> ");
    loaded.send("1");
    loaded.finish("Ending: Timely");
    let mut active_load = Process::start(&[
        "play",
        story.to_str().unwrap(),
        "--load",
        active_file.to_str().unwrap(),
    ]);
    active_load.until("> ");
    assert!(active_load.text.contains("1.5s"));
    active_load.send("1");
    active_load.finish("Ending: Timely");
}

#[test]
fn in_flight_old_input_cannot_choose_after_timeout_and_new_timer_waits_for_ack() {
    let temp = Temp::new();
    let path = temp.file("two-prompts.yaml");
    let mut story: Story = serde_yaml_ng::from_str(&short_story(150)).unwrap();
    let mut second = story.nodes[0].clone();
    second.id = "second".into();
    second.decision.as_mut().unwrap().limit_ms = 1000;
    story.nodes[0].decision.as_mut().unwrap().timeout.target = "second".into();
    story.nodes.push(second);
    fs::write(&path, serde_yaml_ng::to_string(&story).unwrap()).unwrap();
    let mut process = Process::start(&["play", path.to_str().unwrap()]);
    process.until("> ");
    process.until("Press Enter to acknowledge");
    thread::sleep(Duration::from_millis(1200)); // longer than the new, unready deadline
    process.send("1"); // still belongs to the old read, not the new interaction
    process.until("Input from the previous prompt was ignored");
    process.until("> ");
    process.send("1");
    process.finish("Ending: Timely");
    assert!(!process.text.contains("Ending: Expired"));
}

#[test]
fn chained_authored_timeouts_continue_when_stdin_is_exhausted() {
    let temp = Temp::new();
    let path = temp.file("eof-chain.yaml");
    let mut story: Story = serde_yaml_ng::from_str(&short_story(150)).unwrap();
    let mut second = story.nodes[0].clone();
    second.id = "second".into();
    second.content = story.nodes[2].content.clone();
    story.nodes[0].decision.as_mut().unwrap().timeout.target = "second".into();
    story.nodes.push(second);
    fs::write(&path, serde_yaml_ng::to_string(&story).unwrap()).unwrap();
    // EOF both before expiry and while an obsolete read blocks readiness.
    for close_after_transition in [false, true] {
        let mut process = Process::start(&["play", path.to_str().unwrap()]);
        process.until("> ");
        if close_after_transition {
            process.until("Press Enter to acknowledge");
        }
        process.stdin.take();
        process.finish("Ending: Expired");
        assert!(process.text.contains("Authored timeout"));
        assert!(!process.text.contains("Ending: Timely"));
    }
}

#[test]
fn stale_lifecycle_commands_only_acknowledge_the_successor() {
    let temp = Temp::new();
    let path = temp.file("stale-commands.yaml");
    let save = temp.file("snapshot.json");
    let mut story: Story = serde_yaml_ng::from_str(&short_story(150)).unwrap();
    let mut second = story.nodes[0].clone();
    second.id = "second".into();
    second.decision.as_mut().unwrap().limit_ms = 150;
    story.nodes[0].decision.as_mut().unwrap().timeout.target = "second".into();
    story.nodes.push(second);
    fs::write(&path, serde_yaml_ng::to_string(&story).unwrap()).unwrap();
    let engine = Engine::new(story).unwrap();
    write_save(&save, &engine).unwrap();
    let before = fs::read(&save).unwrap();
    for command in [
        ":pause".to_string(),
        ":resume".to_string(),
        format!(":save {}", save.display()),
        format!(":load {}", save.display()),
        ":quit".to_string(),
    ] {
        let mut process = Process::start(&["play", path.to_str().unwrap()]);
        process.until("Press Enter to acknowledge");
        process.send(&command);
        process.until("Input from the previous prompt was ignored");
        process.stdin.take();
        process.finish("Ending: Expired");
        assert!(!process.text.contains("Paused. Use"));
        assert!(!process.text.contains("Resuming after"));
        assert!(!process.text.contains("Saved to"));
        assert!(!process.text.contains("Loaded"));
        assert_eq!(fs::read(&save).unwrap(), before);
    }
}
