use grimm_cli::{parse_story, read_save, write_save};
use grimm_engine::Engine;
use grimm_semantic::{
    GameAction, NarrativeContent,
    story::{Story, Value},
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicUsize, Ordering},
};

const YAML: &str = include_str!("../../../stories/the-last-witness.yaml");
const STORY: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../stories/the-last-witness.yaml"
);

fn engine() -> Engine {
    parse_story("reference.yaml", YAML).unwrap().1
}
fn choose(engine: &mut Engine, id: &str) {
    engine
        .apply(&GameAction {
            revision: engine.output().revision,
            choice_id: id.into(),
        })
        .unwrap();
}
fn offered(engine: &Engine) -> Vec<String> {
    engine
        .output()
        .offers
        .into_iter()
        .map(|offer| offer.id)
        .collect()
}

struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "grimm-m1-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn play(args: &[&str], input: &str) -> String {
    let output = run(args, input);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn full_playthroughs_reach_every_reference_ending() {
    for (route, ending) in [
        (
            vec!["investigate", "take-ledger", "reassure", "testify"],
            "justice",
        ),
        (
            vec!["investigate", "take-ledger", "press", "publish"],
            "exile",
        ),
        (vec!["leave"], "silence"),
        (vec!["investigate", "hurry", "press", "withdraw"], "silence"),
        (vec!["investigate", "take-ledger", "walk-away"], "silence"),
    ] {
        let mut engine = engine();
        for id in route {
            assert!(offered(&engine).contains(&id.to_string()));
            choose(&mut engine, id);
        }
        let output = engine.output();
        assert_eq!(output.ending.unwrap().id, ending);
        assert!(output.offers.is_empty());
    }
}

#[test]
fn evidence_and_trust_change_offers_without_exposing_variables() {
    let mut no_evidence = engine();
    choose(&mut no_evidence, "investigate");
    choose(&mut no_evidence, "hurry");
    assert_eq!(offered(&no_evidence), ["press", "walk-away"]);
    let before = no_evidence.save();
    assert!(
        no_evidence
            .apply(&GameAction {
                revision: no_evidence.output().revision,
                choice_id: "reassure".into()
            })
            .is_err()
    );
    assert_eq!(no_evidence.save(), before);
    choose(&mut no_evidence, "press");
    assert_eq!(offered(&no_evidence), ["withdraw"]);

    let mut supported = engine();
    choose(&mut supported, "investigate");
    choose(&mut supported, "take-ledger");
    assert_eq!(offered(&supported), ["reassure", "press", "walk-away"]);
    choose(&mut supported, "reassure");
    assert_eq!(offered(&supported), ["testify", "withdraw"]);
    assert_eq!(supported.save().variables["trust"], Value::Int(2));
    assert_eq!(
        supported.save().variables["promise"],
        Value::Text("kept".into())
    );
    let projection = serde_json::to_value(supported.output()).unwrap();
    let object = projection.as_object().unwrap();
    assert_eq!(object.len(), 4);
    for key in ["variables", "nodes", "story", "effects", "conditions"] {
        assert!(!object.contains_key(key));
    }
    assert_eq!(object["offers"][0].as_object().unwrap().len(), 2);
}

#[test]
fn yaml_and_checkpoint_serialization_round_trip() {
    let story: Story = serde_yaml_ng::from_str(YAML).unwrap();
    assert_eq!(
        story,
        serde_yaml_ng::from_str(&serde_yaml_ng::to_string(&story).unwrap()).unwrap()
    );
    let mut original = engine();
    choose(&mut original, "investigate");
    choose(&mut original, "take-ledger");
    let temp = Temp::new();
    let path = temp.path("checkpoint.json");
    write_save(&path, &original).unwrap();
    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["story_id"], "the-last-witness");
    let mut restored = engine();
    assert_eq!(
        restored.restore(read_save(&path).unwrap()).unwrap(),
        original.output()
    );
    for id in ["reassure", "testify"] {
        choose(&mut original, id);
        choose(&mut restored, id);
    }
    assert_eq!(restored.output(), original.output());
    write_save(&path, &restored).unwrap();
    assert_eq!(
        engine().restore(read_save(&path).unwrap()).unwrap(),
        restored.output()
    );
}

#[test]
fn cli_plays_all_endings_and_prints_authored_dialogue() {
    for (input, ending) in [
        ("1\n1\n1\n1\n", "A Voice in the Light"),
        ("1\n1\n2\n1\n", "Truth Across the Water"),
        ("2\n", "The Price of Silence"),
    ] {
        let output = play(&["play", STORY], input);
        assert!(output.starts_with("The Last Witness\n"));
        assert!(output.contains(&format!("Ending: {ending}")));
        if input.starts_with("1\n") {
            assert!(output.contains("Mara: I saw the captain bar the mill doors. If I speak, who will protect my brother?"));
        }
        assert!(!output.contains("schema_version"));
    }
}

#[test]
fn cli_saves_and_restores_a_midgame_session() {
    let temp = Temp::new();
    let path = temp.path("save with spaces.json");
    let file = path.to_str().unwrap();
    let output = play(&["play", STORY], &format!("1\n1\n:save {file}\n:quit\n"));
    assert!(output.contains("Saved to"));
    assert_eq!(read_save(&path).unwrap().node_id, "witness");
    let output = play(&["play", STORY, "--load", file], "1\n1\n");
    assert!(!output.contains("At dawn, the last ferry"));
    assert!(output.contains("Ending: A Voice in the Light"));
    let output = play(&["play", STORY], &format!("1\n2\n:load {file}\n1\n1\n"));
    assert!(output.contains("Loaded"));
    assert!(output.contains("Ending: A Voice in the Light"));
}

#[test]
fn cli_load_failures_preserve_the_active_session() {
    let temp = Temp::new();
    let path = temp.path("bad.json");
    let mut save = engine().save();
    save.story_id = "wrong-story".into();
    fs::write(&path, serde_json::to_vec(&save).unwrap()).unwrap();
    let output = play(&["play", STORY], &format!(":load {}\n2\n", path.display()));
    assert!(output.contains("cannot restore"));
    assert!(output.contains("Ending: The Price of Silence"));
    fs::write(&path, "{broken").unwrap();
    let output = play(
        &["play", STORY],
        &format!(":load {}\n:save\n:unknown\n2\n", path.display()),
    );
    assert!(output.contains("invalid JSON save"));
    assert!(output.contains("Provide a file path"));
    assert!(output.contains("Unknown session command"));
    assert!(output.contains("Ending: The Price of Silence"));
    assert!(
        !run(&["play", STORY, "--load", path.to_str().unwrap()], "")
            .status
            .success()
    );
}

#[test]
fn yaml_errors_include_source_and_actionable_context() {
    for (text, expected) in [
        ("nodes: [", "line"),
        (
            &YAML.replace("schema_version: 1", "schema_version: 9"),
            "schema version",
        ),
        (
            &YAML.replacen("target: archive", "target: absent", 1),
            "choice 'investigate': target 'absent'",
        ),
        (
            &YAML.replacen(
                "title: The Last Witness",
                "title: The Last Witness\nunknown: value",
                1,
            ),
            "unknown field",
        ),
        (
            &YAML.replacen("label: Search", "lable: Search", 1),
            "unknown field",
        ),
        (
            &YAML.replacen("trust: 0", "trust: 0\n  trust: 1", 1),
            "duplicate variable",
        ),
        (
            &YAML.replacen("op: add", "op: multiply", 1),
            "unknown variant",
        ),
        (
            &YAML.replacen("value: true", "value: \"true\"", 1),
            "wrong type",
        ),
    ] {
        let error = parse_story("broken.yaml", text).unwrap_err().to_string();
        assert!(error.contains("broken.yaml"), "{error}");
        assert!(
            error.contains(expected),
            "expected {expected:?}, got {error:?}"
        );
    }
}

#[test]
fn cli_reports_missing_invalid_files_and_bad_arguments() {
    let temp = Temp::new();
    let path = temp.path("bad.yaml");
    fs::write(&path, "nodes: [").unwrap();
    for args in [
        vec!["play", path.to_str().unwrap()],
        vec!["play", "missing-story.yaml"],
        vec!["unknown"],
    ] {
        let output = run(&args, "");
        assert!(!output.status.success());
        assert!(!output.stderr.is_empty());
    }
    assert!(play(&["--help"], "").contains("Usage:"));
    let output = play(&["play", STORY], "99\n2\n");
    assert!(output.contains("Enter a displayed choice number."));
    assert!(output.contains("Ending: The Price of Silence"));
    assert!(!play(&["play", STORY], "").contains("Ending:"));
}

#[test]
fn file_errors_and_strict_save_schema_are_reported() {
    let temp = Temp::new();
    let missing = temp.path("missing.json");
    assert!(
        read_save(&missing)
            .unwrap_err()
            .to_string()
            .contains("cannot read save")
    );
    assert!(
        write_save(&temp.path("missing-directory/save.json"), &engine())
            .unwrap_err()
            .to_string()
            .contains("cannot write save")
    );
    let mut json = serde_json::to_value(engine().save()).unwrap();
    json["extra"] = true.into();
    fs::write(&missing, json.to_string()).unwrap();
    assert!(
        read_save(&missing)
            .unwrap_err()
            .to_string()
            .contains("unknown field")
    );
    let json = serde_json::to_string(&engine().save()).unwrap();
    fs::write(
        &missing,
        json.replace("\"trust\":0", "\"trust\":0,\"trust\":1"),
    )
    .unwrap();
    assert!(
        read_save(&missing)
            .unwrap_err()
            .to_string()
            .contains("duplicate variable")
    );
    assert!(
        grimm_cli::load_story(Path::new("absent.yaml"))
            .unwrap_err()
            .to_string()
            .contains("absent.yaml")
    );
}

#[test]
fn structured_authored_content_is_preserved_without_interpolation() {
    let mut story: Story = serde_yaml_ng::from_str(YAML).unwrap();
    story.nodes[0].content = vec![NarrativeContent::Narration {
        text: "Exact\n${trust} stays literal.".into(),
    }];
    let output = Engine::new(story).unwrap().output();
    assert_eq!(
        output.content,
        vec![NarrativeContent::Narration {
            text: "Exact\n${trust} stays literal.".into()
        }]
    );
}

#[test]
fn cli_reports_later_blocked_states_instead_of_prompting_without_offers() {
    // One-transition previews do not promise that the whole route is solvable.
    let yaml = r#"
schema_version: 1
id: later-deadlock
version: 1
title: Later deadlock
start: start
variables: { ready: false }
nodes:
  - id: start
    content: [{ kind: narration, text: Start }]
    choices: [{ id: step, label: Step, target: middle }]
  - id: middle
    content: [{ kind: narration, text: Middle }]
    choices: [{ id: blocked, label: Blocked, target: locked }]
  - id: locked
    content: [{ kind: narration, text: Locked }]
    choices:
      - id: finish
        label: Finish
        target: end
        condition: { op: equals, variable: ready, value: true }
  - id: end
    content: [{ kind: narration, text: End }]
    ending: { id: end, title: End }
"#;
    let temp = Temp::new();
    let path = temp.path("later-deadlock.yaml");
    fs::write(&path, yaml).unwrap();
    let output = run(&["play", path.to_str().unwrap()], "1\n");
    assert!(!output.status.success());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .contains("no executable choices")
    );
    let screen = String::from_utf8(output.stdout).unwrap();
    assert!(screen.contains("Middle"));
    assert!(!screen.contains("Blocked"));
    assert!(!screen.contains("Ending:"));
}
