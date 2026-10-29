use grimm_cli::{parse_story, read_save, write_save};
use grimm_engine::Engine;
use grimm_semantic::{
    GameAction, NarrativeContent,
    story::{Story, Value},
};
use std::{
    collections::BTreeSet,
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

const YAML: &str = include_str!("../../../stories/the-last-lighthouse.yaml");
const PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../stories/the-last-lighthouse.yaml"
);
const PREPARE: &[&str] = &["listen-to-lena", "read-log", "prepare-wire", "take-wire"];
const INVESTIGATE: &[&str] = &[
    "record-signal",
    "share-tape",
    "ask-for-partner",
    "answer-safely",
];

fn engine() -> Engine {
    parse_story("the-last-lighthouse.yaml", YAML).unwrap().1
}
fn choose(engine: &mut Engine, ids: &[&str]) {
    for id in ids {
        let output = engine.output();
        assert!(
            output.offers.iter().any(|offer| offer.id == *id),
            "{id} not offered at {}",
            engine.save().node_id
        );
        engine
            .apply(&GameAction {
                revision: output.revision,
                choice_id: (*id).into(),
            })
            .unwrap();
    }
}
fn offers(engine: &Engine) -> Vec<String> {
    engine
        .output()
        .offers
        .into_iter()
        .map(|offer| offer.id)
        .collect()
}
fn cli(args: &[&str], input: &str) -> String {
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
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn authored_identity_spans_three_distinct_scenes_not_three_places() {
    let story: Story = serde_yaml_ng::from_str(YAML).unwrap();
    assert!((10..=15).contains(&story.nodes.len()));
    let mut scenes = BTreeSet::new();
    for (id, time) in [
        ("radio_day", "Daylight"),
        ("radio_night", "Night watch"),
        ("radio_return", "Dawn return"),
    ] {
        let scene = story.nodes.iter().find(|node| node.id == id).unwrap();
        assert!(scene.content.iter().any(|block| matches!(block, NarrativeContent::Narration { text } if text.contains("North Spit radio room") && text.starts_with(time))));
        scenes.insert(scene.id.clone());
    }
    assert_eq!(scenes.len(), 3);
    let lena_scenes: Vec<_> = story.nodes.iter().filter(|node| node.content.iter().any(|block| matches!(block, NarrativeContent::Dialogue { speaker, .. } if speaker == "Lena Voss"))).collect();
    assert!(lena_scenes.len() >= 4);
    assert!(lena_scenes.iter().any(|node| node.id == "lena_day"));
    assert!(lena_scenes.iter().any(|node| node.id == "night_confession"));
    assert!(lena_scenes.iter().any(|node| node.id == "final_watch"));
}

#[test]
fn progress_and_relationship_persist_when_returning_to_the_same_location() {
    let mut prepared = engine();
    choose(&mut prepared, PREPARE);
    assert_eq!(prepared.save().node_id, "radio_night");
    choose(&mut prepared, INVESTIGATE);
    let save = prepared.save();
    assert_eq!(save.node_id, "radio_return");
    assert_eq!(save.variables["lena_trust"], Value::Int(3));
    for flag in [
        "knows_callsign",
        "has_ground_wire",
        "signal_recorded",
        "reply_sent",
    ] {
        assert_eq!(save.variables[flag], Value::Bool(true));
    }
    assert_eq!(
        offers(&prepared),
        ["review-together", "report-safe-reply", "keep-distance"]
    );

    let mut unprepared = engine();
    choose(
        &mut unprepared,
        &["suspect-lena", "leave-wire", "talk-to-lena"],
    );
    assert_eq!(offers(&unprepared), ["decide-alone"]);
    choose(&mut unprepared, &["decide-alone"]);
    assert!(!offers(&unprepared).contains(&"answer-safely".into()));
    choose(&mut unprepared, &["wait-for-dawn"]);
    assert_eq!(unprepared.save().node_id, save.node_id);
    assert_eq!(unprepared.save().variables["lena_trust"], Value::Int(0));
    assert_eq!(offers(&unprepared), ["keep-distance"]);
    assert_ne!(offers(&prepared), offers(&unprepared));
    let projection = serde_json::to_value(prepared.output()).unwrap();
    assert_eq!(projection.as_object().unwrap().len(), 4);
    assert!(projection.get("variables").is_none());
    assert!(projection.get("nodes").is_none());
}

#[test]
fn semantic_routes_reach_all_four_explicit_endings() {
    let routes = [
        (
            vec![
                "listen-to-lena",
                "read-log",
                "prepare-wire",
                "take-wire",
                "record-signal",
                "share-tape",
                "ask-for-partner",
                "answer-safely",
                "review-together",
                "bring-investigators",
            ],
            "rescue",
        ),
        (
            vec![
                "listen-to-lena",
                "read-log",
                "prepare-wire",
                "take-wire",
                "approach-receiver",
                "answer-safely",
                "report-safe-reply",
                "release-watch",
            ],
            "release",
        ),
        (
            vec![
                "suspect-lena",
                "leave-wire",
                "approach-receiver",
                "wait-for-dawn",
                "keep-distance",
                "close-station",
            ],
            "silence",
        ),
        (
            vec![
                "suspect-lena",
                "leave-wire",
                "approach-receiver",
                "follow-lights",
                "board-orison",
            ],
            "taken",
        ),
        (
            vec![
                "listen-to-lena",
                "inspect-pier",
                "take-wire",
                "record-signal",
                "share-tape",
                "ask-for-partner",
                "follow-lights",
                "trust-shore",
                "review-together",
                "bring-investigators",
            ],
            "rescue",
        ),
    ];
    for (route, ending) in routes {
        let mut game = engine();
        choose(&mut game, &route);
        assert_eq!(game.output().ending.unwrap().id, ending);
        assert!(game.output().offers.is_empty());
    }
}

#[test]
fn actual_cli_scripts_reach_all_endings() {
    for (input, title) in [
        ("1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n", "Witnesses on the Shore"),
        ("1\n1\n1\n1\n3\n1\n1\n1\n", "The Watch Is Over"),
        ("2\n2\n3\n2\n1\n1\n", "An Unanswered Bell"),
        ("2\n2\n3\n1\n2\n", "One More Voice"),
    ] {
        let text = cli(&["play", PATH], input);
        assert!(text.contains(&format!("Ending: {title}")), "{text}");
        assert_eq!(text.matches("Ending:").count(), 1);
        assert!(!text.contains("lena_trust"));
        assert!(!text.contains("signal_recorded"));
    }
}

struct Temp(PathBuf);
impl Temp {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("grimm-lighthouse-{}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn file(&self) -> PathBuf {
        self.0.join("night watch.json")
    }
}
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn save_load_preserves_world_progress_and_cli_continuation() {
    let temp = Temp::new();
    let path = temp.file();
    let file = path.to_str().unwrap();
    let mut uninterrupted = engine();
    choose(&mut uninterrupted, PREPARE);
    write_save(&path, &uninterrupted).unwrap();
    let mut restored = engine();
    assert_eq!(
        restored.restore(read_save(&path).unwrap()).unwrap(),
        uninterrupted.output()
    );
    choose(&mut uninterrupted, INVESTIGATE);
    choose(&mut restored, INVESTIGATE);
    assert_eq!(restored.save(), uninterrupted.save());
    choose(&mut restored, &["review-together", "bring-investigators"]);
    assert_eq!(restored.output().ending.unwrap().id, "rescue");
    let output = cli(
        &["play", PATH],
        &format!("1\n1\n1\n1\n:save {file}\n:quit\n"),
    );
    assert!(output.contains("Saved to"));
    let resumed = cli(&["play", PATH, "--load", file], "1\n1\n1\n1\n1\n1\n");
    assert!(resumed.contains("Night watch in the North Spit radio room"));
    assert!(!resumed.contains("Daylight in the North Spit radio room"));
    assert!(resumed.contains("Ending: Witnesses on the Shore"));
}

#[test]
fn bounded_authoring_check_finishes_for_both_reference_stories_and_the_m0_demo() {
    for (name, yaml, expected) in [
        (
            "Lighthouse",
            YAML,
            vec!["release", "rescue", "silence", "taken"],
        ),
        (
            "Witness",
            include_str!("../../../stories/the-last-witness.yaml"),
            vec!["exile", "justice", "silence"],
        ),
    ] {
        let game = parse_story(name, yaml).unwrap().1;
        let before = game.save();
        let report = game.check_playability(grimm_engine::PlayabilityLimits::default());
        assert!(!report.truncated, "{name}");
        assert!(report.deadlocks.is_empty(), "{report:?}");
        assert_eq!(report.endings.into_iter().collect::<Vec<_>>(), expected);
        assert_eq!(game.save(), before);
    }
    let report = Engine::default().check_playability(grimm_engine::PlayabilityLimits::default());
    assert!(!report.truncated);
    assert!(report.deadlocks.is_empty());
    assert!(report.endings.contains("complete"));
    let text = cli(&["check", PATH], "");
    assert!(text.contains("release, rescue, silence, taken"));
    assert!(text.contains("No nonterminal deadlocks found"));
}
