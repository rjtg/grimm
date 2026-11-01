//! Validate production references against the real story index and original text.
use grimm_cli::screenplay::{Screenplay, validate};
use grimm_semantic::authoring::StoryIndex;
const STORY: &str = include_str!("../../../examples/whiteout/story.yaml");
const TEXT: &str = include_str!("../../../examples/whiteout/text.yaml");
const SCREENPLAY: &str =
    include_str!("../../../examples/whiteout/production/blackout.screenplay.yaml");
fn index() -> StoryIndex {
    grimm_cli::authoring::inspect("story.yaml", STORY)
        .index
        .unwrap()
}
fn config() -> grimm_semantic::lifecycle::SceneConfiguration {
    serde_yaml_ng::from_str(include_str!("../../../examples/whiteout/scenes.yaml")).unwrap()
}
fn document() -> Screenplay {
    serde_yaml_ng::from_str(SCREENPLAY).unwrap()
}
#[test]
fn blackout_conversion_preserves_text_and_covers_its_actual_context() {
    let index = index();
    let doc = document();
    validate(&doc, &index, TEXT, &config()).unwrap();
    let context = index.contexts.iter().find(|c| c.id == "blackout").unwrap();
    let scene = &doc.scenes[0];
    assert_eq!(
        scene.beats.len(),
        context
            .cues
            .iter()
            .filter(|c| c.narrative_id.is_some())
            .count()
    );
    assert_eq!(
        scene.decision_actions,
        context
            .actions
            .iter()
            .map(|a| a.id.clone())
            .collect::<Vec<_>>()
    );
    let roundtrip: Screenplay =
        serde_json::from_str(&serde_json::to_string(&doc).unwrap()).unwrap();
    validate(&roundtrip, &index, TEXT, &config()).unwrap();
}
#[test]
fn invalid_ids_references_versions_content_and_timing_are_diagnosed() {
    let index = index();
    let cases: [(&str, &str); 9] = [
        ("whiteout.blackout.arrival", "whiteout.blackout"),
        ("scene_id: whiteout.blackout", "scene_id: Whiteout/blackout"),
        ("context_id: blackout", "context_id: missing"),
        ("cue_id: blackout.arrival", "cue_id: generator.arrival"),
        (
            "narrative_id: blackout.arrival",
            "narrative_id: generator.arrival",
        ),
        ("blackout.begin", "blackout.missing"),
        ("story_version: 2", "story_version: 99"),
        ("AURORA / 03:17", "AURORA / 04:17"),
        ("duration_ms: 12000", "duration_ms: 18446744073709551615"),
    ];
    for (from, to) in cases {
        assert!(SCREENPLAY.contains(from), "{from}");
        let doc: Screenplay = serde_yaml_ng::from_str(&SCREENPLAY.replace(from, to)).unwrap();
        assert!(
            validate(&doc, &index, TEXT, &config()).is_err(),
            "accepted {from} -> {to}"
        );
    }
    assert!(
        validate(
            &document(),
            &index,
            "narratives: {}\nactions: {}\n",
            &config()
        )
        .is_err()
    );
    let mut doc = document();
    doc.scenes.push(doc.scenes[0].clone());
    assert!(validate(&doc, &index, TEXT, &config()).is_err());
    let mut doc = document();
    let repeated = doc.scenes[0].beats[0].clone();
    doc.scenes[0].beats.push(repeated);
    assert!(validate(&doc, &index, TEXT, &config()).is_err());
}
#[test]
fn codec_is_strict_and_cli_runs_the_real_fragment() {
    assert!(
        serde_yaml_ng::from_str::<Screenplay>(
            &SCREENPLAY.replace("kind: shot", "kind: execute_rust")
        )
        .is_err()
    );
    assert!(
        serde_yaml_ng::from_str::<Screenplay>(&format!("{SCREENPLAY}\nunknown_field: true"))
            .is_err()
    );
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .args([
            "check-screenplay",
            "examples/whiteout/story.yaml",
            "examples/whiteout/production/blackout.screenplay.yaml",
            "examples/whiteout/text.yaml",
            "examples/whiteout/scenes.yaml",
        ])
        .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("valid screenplay fragment (1 scene(s), 1 beat(s))")
    );
}

#[test]
fn lifecycle_scope_and_duplicate_binding_mismatches_are_diagnosed() {
    let index = index();
    let mut wrong = config();
    wrong
        .scenes
        .insert("blackout".into(), "whiteout.somewhere_else".into());
    let error = validate(&document(), &index, TEXT, &wrong)
        .unwrap_err()
        .to_string();
    assert!(error.contains("scene scope differs"), "{error}");
    wrong = config();
    wrong.story_version += 1;
    assert!(validate(&document(), &index, TEXT, &wrong).is_err());
    let duplicate = format!("{TEXT}\n  blackout.begin: Conflicting action label\n");
    assert!(validate(&document(), &index, &duplicate, &config()).is_err());
}
