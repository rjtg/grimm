use grimm_cli::text_resources::TextResources;
use grimm_semantic::lifecycle::SceneConfiguration;
const STORY: &str = include_str!("../../../examples/whiteout/story.yaml");
const TEXT: &str = include_str!("../../../examples/whiteout/text.yaml");
const CONFIG: &str = include_str!("../../../examples/whiteout/scenes.yaml");
fn config() -> SceneConfiguration {
    serde_yaml_ng::from_str(CONFIG).unwrap()
}
#[test]
fn actual_whiteout_text_registry_exports_and_prepares_explicit_scopes() {
    let mut resources = TextResources::from_sources(STORY, &config(), TEXT, None).unwrap();
    assert_eq!(resources.catalog.scenes.len(), 9);
    assert_eq!(resources.prepared_scene(), None);
    assert_eq!(resources.narrative("blackout.arrival"), None);
    resources.prepare("whiteout.blackout").unwrap();
    assert!(
        resources
            .narrative("blackout.arrival")
            .unwrap()
            .contains("AURORA")
    );
    assert!(resources.action("blackout.begin").is_some());
    assert!(resources.narrative("generator.arrival").is_none());
    resources.prepare("whiteout.generator_annex").unwrap();
    assert!(resources.narrative("generator.arrival").is_some());
    assert!(resources.narrative("whiteout.latch_crack").is_some()); // alternate authored outcomes
    assert!(resources.narrative("blackout.arrival").is_none());
    assert!(resources.action("generator.start_generator").is_some());
    let source: grimm_semantic::scene::SceneStory = serde_yaml_ng::from_str(STORY).unwrap();
    let index = grimm_cli::authoring::inspect("story", STORY).index.unwrap();
    assert!(
        grimm_cli::catalog::compare(&index, &source, &resources.catalog)
            .errors
            .is_empty()
    );
}
#[test]
fn missing_actual_resources_and_failed_preparation_never_silently_fall_back() {
    let missing = TEXT.replace("  generator.arrival:", "  generator.absent:");
    assert!(TextResources::from_sources(STORY, &config(), &missing, None).is_err());
    let duplicate = format!("{TEXT}\n  blackout.begin: Duplicate\n");
    assert!(TextResources::from_sources(STORY, &config(), &duplicate, None).is_err());
    let mut resources = TextResources::from_sources(STORY, &config(), TEXT, None).unwrap();
    resources.prepare("whiteout.blackout").unwrap();
    resources
        .catalog
        .scenes
        .get_mut("whiteout.generator_annex")
        .unwrap()
        .resource = "file:///missing.asset".into();
    assert!(resources.prepare("whiteout.generator_annex").is_err());
    assert_eq!(resources.prepared_scene(), Some("whiteout.blackout"));
    assert!(resources.narrative("blackout.arrival").is_some());
    resources
        .catalog
        .scenes
        .get_mut("whiteout.generator_annex")
        .unwrap()
        .resource = "text://whiteout.generator_annex".into();
    resources.prepare("whiteout.generator_annex").unwrap();
    assert_eq!(resources.prepared_scene(), Some("whiteout.generator_annex"));
}
#[test]
fn actual_export_command_produces_a_usable_catalog() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .args([
            "export-text-catalog",
            "examples/whiteout/story.yaml",
            "examples/whiteout/scenes.yaml",
            "examples/whiteout/text.yaml",
        ])
        .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let catalog: grimm_cli::catalog::Catalog = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        catalog
            .presentations
            .contains_key("generator.startup_prime")
    );
    let mut resources = TextResources::from_sources(STORY, &config(), TEXT, Some(catalog)).unwrap();
    resources.prepare("whiteout.generator_annex").unwrap();
}
