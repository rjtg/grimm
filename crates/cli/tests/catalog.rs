use grimm_cli::catalog::{Catalog, Completion, DurationKind, compare};
use grimm_semantic::scene::SceneStory;
const STORY: &str = include_str!("fixtures/catalog/story.yaml");
const CATALOG: &str = include_str!("fixtures/catalog/catalog.yaml");
fn check(catalog: &Catalog) -> grimm_cli::catalog::Report {
    let story: SceneStory = serde_yaml_ng::from_str(STORY).unwrap();
    let report = grimm_cli::authoring::inspect("story.yaml", STORY);
    assert!(!report.has_errors(), "{report:?}");
    compare(&report.index.unwrap(), &story, catalog)
}
fn catalog() -> Catalog {
    serde_yaml_ng::from_str(CATALOG).unwrap()
}
#[test]
fn all_paths_and_optional_completion_metadata_are_validated() {
    assert!(check(&catalog()).errors.is_empty());
    let mut doc = catalog();
    doc.presentations.remove("room.hidden");
    assert!(
        check(&doc)
            .errors
            .iter()
            .any(|e| e.contains("missing presentation 'room.hidden'"))
    );
    let mut doc = catalog();
    doc.presentations.get_mut("room.intro").unwrap().duration_ms = Some(101);
    assert!(
        check(&doc)
            .errors
            .iter()
            .any(|e| e.contains("conflicts with authored duration 100ms"))
    );
    let mut doc = catalog();
    doc.presentations
        .get_mut("room.hidden")
        .unwrap()
        .duration_kind = Some(DurationKind::Exact);
    assert!(
        check(&doc)
            .errors
            .iter()
            .any(|e| e.contains("incompatible completion/duration"))
    );
    let mut doc = catalog();
    doc.presentations.get_mut("room.intro").unwrap().completion = Completion::Callback;
    assert!(!check(&doc).errors.is_empty());
}
#[test]
fn scope_missing_references_and_unused_resources_are_explicit() {
    let mut doc = catalog();
    doc.presentations.get_mut("room.normal").unwrap().scene_ids = vec!["demo.end".into()];
    assert!(
        check(&doc)
            .errors
            .iter()
            .any(|e| e.contains("does not support scene 'demo.room'"))
    );
    let mut doc = catalog();
    doc.contexts.remove("room");
    assert!(
        check(&doc)
            .errors
            .iter()
            .any(|e| e.contains("has no scene mapping"))
    );
    let mut doc = catalog();
    doc.scenes.remove("demo.room");
    assert!(
        check(&doc)
            .errors
            .iter()
            .any(|e| e.contains("unknown scene"))
    );
    let mut doc = catalog();
    doc.presentations.insert(
        "unused.notice".into(),
        doc.presentations["end.ending"].clone(),
    );
    assert!(check(&doc).errors.is_empty());
    assert!(
        check(&doc)
            .warnings
            .iter()
            .any(|e| e.contains("unused presentation"))
    );
}
#[test]
fn strict_catalog_cli_has_machine_readable_success_and_usage_failures() {
    assert!(
        serde_yaml_ng::from_str::<Catalog>(
            &CATALOG.replace("completion: callback", "completion: arbitrary_code")
        )
        .is_err()
    );
    assert!(serde_yaml_ng::from_str::<Catalog>(&format!("{CATALOG}\nunknown: true")).is_err());
    assert!(
        serde_yaml_ng::from_str::<Catalog>(&CATALOG.replace(
            "contexts: {room: demo.room, end: demo.end}",
            "contexts: {room: demo.room, room: demo.end}"
        ))
        .is_err()
    );
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .args([
            "check-catalog",
            "crates/cli/tests/fixtures/catalog/story.yaml",
            "crates/cli/tests/fixtures/catalog/catalog.yaml",
        ])
        .current_dir(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["errors"], serde_json::json!([]));
    assert!(
        !std::process::Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
            .arg("check-catalog")
            .output()
            .unwrap()
            .status
            .success()
    );
}
