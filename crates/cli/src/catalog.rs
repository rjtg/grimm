//! Offline comparison with renderer-exported catalogs. Never used by the core
//! to load media, inspect private state or authorize gameplay.
use grimm_semantic::{authoring::StoryIndex, scene::SceneStory};
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
};
use std::{fmt, marker::PhantomData};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub manifest_version: u32,
    pub renderer: String,
    /// Explicit graph-to-presentation mapping for tooling, not a filename rule.
    #[serde(deserialize_with = "unique_map")]
    pub contexts: BTreeMap<String, String>,
    #[serde(deserialize_with = "unique_map")]
    pub scenes: BTreeMap<String, SceneResource>,
    #[serde(deserialize_with = "unique_map")]
    pub presentations: BTreeMap<String, Presentation>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneResource {
    pub resource: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Presentation {
    pub scene_ids: Vec<String>,
    pub completion: Completion,
    pub duration_ms: Option<u64>,
    pub duration_kind: Option<DurationKind>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Completion {
    Timed,
    Callback,
    Instant,
    Persistent,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationKind {
    Exact,
    Estimate,
}
#[derive(Debug, Default, Serialize)]
pub struct Report {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
fn unique_map<'de, D, T>(deserializer: D) -> Result<BTreeMap<String, T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Unique<T>(PhantomData<T>);
    impl<'de, T: Deserialize<'de>> Visitor<'de> for Unique<T> {
        type Value = BTreeMap<String, T>;
        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a map of unique explicit identifiers")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
            let mut result = BTreeMap::new();
            while let Some((key, value)) = map.next_entry::<String, T>()? {
                if result.insert(key.clone(), value).is_some() {
                    return Err(de::Error::custom(format!(
                        "duplicate catalog identifier '{key}'"
                    )));
                }
            }
            Ok(result)
        }
    }
    deserializer.deserialize_map(Unique(PhantomData))
}
fn id(id: &str) -> bool {
    id.split('.').all(|part| {
        let mut bytes = part.bytes();
        bytes.next().is_some_and(|b| b.is_ascii_lowercase())
            && bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
    })
}
/// Validate all authored paths, not a live engine's current offers. Resource URIs
/// remain opaque; existence/import validation belongs to the renderer exporter.
pub fn compare(index: &StoryIndex, story: &SceneStory, catalog: &Catalog) -> Report {
    let mut report = Report::default();
    if catalog.manifest_version != 1 {
        report
            .errors
            .push("unsupported renderer manifest_version".into());
    }
    if !id(&catalog.renderer) {
        report.errors.push("invalid renderer identifier".into());
    }
    for (scene, resource) in &catalog.scenes {
        if !id(scene) || resource.resource.trim().is_empty() {
            report
                .errors
                .push(format!("invalid scene/resource '{scene}'"));
        }
    }
    for (name, presentation) in &catalog.presentations {
        if !id(name) {
            report
                .errors
                .push(format!("invalid presentation identifier '{name}'"));
        }
        let mut scopes = BTreeSet::new();
        if presentation.scene_ids.is_empty() {
            report.errors.push(format!("'{name}' has no scene scope"));
        }
        for scope in &presentation.scene_ids {
            if !catalog.scenes.contains_key(scope) || !scopes.insert(scope) {
                report
                    .errors
                    .push(format!("'{name}' has missing/duplicate scene '{scope}'"));
            }
        }
        if presentation.duration_ms.is_some() != presentation.duration_kind.is_some()
            || presentation
                .duration_ms
                .is_some_and(|ms| ms > i64::MAX as u64)
        {
            report
                .errors
                .push(format!("'{name}' has invalid duration metadata"));
        }
        let valid = match presentation.completion {
            Completion::Timed => {
                presentation.duration_ms.is_some()
                    && presentation.duration_kind == Some(DurationKind::Exact)
            }
            Completion::Callback => presentation
                .duration_kind
                .is_none_or(|kind| kind == DurationKind::Estimate),
            Completion::Instant => {
                presentation.duration_ms.is_none_or(|ms| ms == 0)
                    && presentation.duration_kind != Some(DurationKind::Estimate)
            }
            Completion::Persistent => presentation.duration_ms.is_none(),
        };
        if !valid {
            report
                .errors
                .push(format!("'{name}' has incompatible completion/duration"));
        }
    }
    let source = serde_json::to_value(story).expect("story DTO serialization");
    let mut used = BTreeSet::new();
    let mut used_scenes = BTreeSet::new();
    for context in &index.contexts {
        let Some(scene) = catalog.contexts.get(&context.id) else {
            report
                .errors
                .push(format!("context '{}' has no scene mapping", context.id));
            continue;
        };
        used_scenes.insert(scene);
        if !catalog.scenes.contains_key(scene) {
            report.errors.push(format!(
                "context '{}' maps to unknown scene '{scene}'",
                context.id
            ));
        }
        for cue in &context.cues {
            let Some(narrative) = &cue.narrative_id else {
                continue;
            };
            used.insert(narrative);
            let Some(presentation) = catalog.presentations.get(narrative) else {
                report.errors.push(format!(
                    "{}: missing presentation '{narrative}' (cue '{}')",
                    cue.source.path, cue.cue_id
                ));
                continue;
            };
            if !presentation.scene_ids.contains(scene) {
                report.errors.push(format!(
                    "{}: presentation '{narrative}' does not support scene '{scene}'",
                    cue.source.path
                ));
            }
            let Some(authored) = source.pointer(&cue.source.path) else {
                report.errors.push(format!(
                    "unresolved source pointer for cue '{}'",
                    cue.cue_id
                ));
                continue;
            };
            if let Some(duration) = authored.get("duration_ms").and_then(|v| v.as_u64()) {
                let matches = if duration == 0 && presentation.completion == Completion::Instant {
                    true
                } else {
                    presentation.completion == Completion::Timed
                        && presentation.duration_kind == Some(DurationKind::Exact)
                        && presentation.duration_ms == Some(duration)
                };
                if !matches {
                    report.errors.push(format!("{}: presentation '{narrative}' conflicts with authored duration {duration}ms",cue.source.path));
                }
            }
        }
    }
    for name in catalog.presentations.keys() {
        if !used.contains(name) {
            report
                .warnings
                .push(format!("unused presentation '{name}'"));
        }
    }
    for name in catalog.scenes.keys() {
        if !used_scenes.contains(name) {
            report.warnings.push(format!("unused scene '{name}'"));
        }
    }
    for context in catalog.contexts.keys() {
        if !index.contexts.iter().any(|c| c.id == *context) {
            report
                .errors
                .push(format!("unknown mapped context '{context}'"));
        }
    }
    report
}
pub fn run(args: &[OsString]) -> Result<(), Box<dyn std::error::Error>> {
    let [story_path, catalog_path] = args else {
        return Err("Usage: grimm-cli check-catalog STORY.yaml CATALOG.yaml".into());
    };
    let yaml = fs::read_to_string(story_path)?;
    let index = crate::authoring::inspect(&story_path.to_string_lossy(), &yaml);
    if index.has_errors() {
        return Err("invalid story; run check-scene for diagnostics".into());
    }
    let story: SceneStory = serde_yaml_ng::from_str(&yaml)?;
    let catalog: Catalog = serde_yaml_ng::from_str(&fs::read_to_string(catalog_path)?)?;
    let report = compare(&index.index.expect("valid index"), &story, &catalog);
    println!("{}", serde_json::to_string_pretty(&report)?);
    if report.errors.is_empty() {
        Ok(())
    } else {
        Err("renderer catalog validation failed".into())
    }
}
