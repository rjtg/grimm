//! Actual text-renderer registry/export/preparation. File paths belong to the
//! CLI, semantic scene IDs to configuration, and rules remain in the engine.
use crate::{
    catalog::{self, Catalog, Completion, DurationKind, Presentation, SceneResource},
    scene_runtime::TextBindings,
};
use grimm_semantic::{authoring::StoryIndex, lifecycle::SceneConfiguration, scene::SceneStory};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct TextResources {
    pub catalog: Catalog,
    pub warnings: Vec<String>,
    index: StoryIndex,
    registry: TextBindings,
    authored_presentations: BTreeMap<String, Presentation>,
    prepared_scene: Option<String>,
    narratives: BTreeMap<String, String>,
    actions: BTreeMap<String, String>,
}
impl TextResources {
    pub fn from_sources(
        story_yaml: &str,
        configuration: &SceneConfiguration,
        text_yaml: &str,
        supplied: Option<Catalog>,
    ) -> Result<Self> {
        let story: SceneStory = serde_yaml_ng::from_str(story_yaml)?;
        // Reuse the engine's actual configuration/identity validation.
        grimm_engine::scene::lifecycle::LifecycleEngine::new(story.clone(), configuration.clone())?;
        let report = crate::authoring::inspect("story", story_yaml);
        if report.has_errors() {
            return Err("invalid story; run check-scene for diagnostics".into());
        }
        let index = report.index.expect("validated index");
        let registry: TextBindings = serde_yaml_ng::from_str(text_yaml)?;
        let source = serde_json::to_value(&story)?;
        let mut scenes = BTreeMap::new();
        let mut presentations: BTreeMap<String, Presentation> = BTreeMap::new();
        let mut used_actions = BTreeSet::new();
        for context in &index.contexts {
            let scene = &configuration.scenes[&context.id];
            scenes.insert(
                scene.clone(),
                SceneResource {
                    resource: format!("text://{scene}"),
                },
            );
            for cue in &context.cues {
                let Some(narrative) = &cue.narrative_id else {
                    continue;
                };
                if !registry
                    .narratives
                    .get(narrative)
                    .is_some_and(|text| !text.trim().is_empty())
                {
                    return Err(format!(
                        "{}: missing actual text resource '{narrative}'",
                        cue.source.path
                    )
                    .into());
                }
                let authored = source
                    .pointer(&cue.source.path)
                    .ok_or("unresolved cue source pointer")?;
                let duration = authored.get("duration_ms").and_then(|ms| ms.as_u64());
                let entry =
                    presentations
                        .entry(narrative.clone())
                        .or_insert_with(|| Presentation {
                            scene_ids: vec![],
                            completion: Completion::Callback,
                            duration_ms: None,
                            duration_kind: None,
                        });
                if !entry.scene_ids.contains(scene) {
                    entry.scene_ids.push(scene.clone());
                    entry.scene_ids.sort();
                }
                if let Some(duration) = duration {
                    if entry.duration_ms.is_some_and(|prior| prior != duration) {
                        return Err(format!(
                            "presentation '{narrative}' has conflicting authored exact durations"
                        )
                        .into());
                    }
                    entry.duration_ms = Some(duration);
                    entry.duration_kind = Some(DurationKind::Exact);
                    entry.completion = Completion::Timed;
                }
            }
            for action in &context.actions {
                used_actions.insert(action.id.clone());
                if !registry
                    .actions
                    .get(&action.id)
                    .is_some_and(|text| !text.trim().is_empty())
                {
                    return Err(format!("missing actual action label '{}'", action.id).into());
                }
            }
        }
        let exported = Catalog {
            manifest_version: 1,
            renderer: "grimm.text".into(),
            contexts: configuration.scenes.clone(),
            scenes,
            presentations,
        };
        let mut warnings: Vec<_> = registry
            .narratives
            .keys()
            .filter(|id| !exported.presentations.contains_key(*id))
            .map(|id| format!("unused actual narrative resource '{id}'"))
            .collect();
        warnings.extend(
            registry
                .actions
                .keys()
                .filter(|id| !used_actions.contains(*id))
                .map(|id| format!("unused actual action label '{id}'")),
        );
        let authored_presentations = exported.presentations.clone();
        let catalog = supplied.unwrap_or(exported);
        if catalog.renderer != "grimm.text" || catalog.contexts != configuration.scenes {
            return Err(
                "catalog renderer or scene configuration does not match the text runtime".into(),
            );
        }
        let compared = catalog::compare(&index, &story, &catalog);
        if !compared.errors.is_empty() {
            return Err(compared.errors.join("\n").into());
        }
        warnings.extend(compared.warnings);
        Ok(Self {
            catalog,
            warnings,
            index,
            registry,
            authored_presentations,
            prepared_scene: None,
            narratives: BTreeMap::new(),
            actions: BTreeMap::new(),
        })
    }
    /// Resolve all resources for the upcoming scope before the host sends Ready.
    /// Build the replacement cache off to the side: failure preserves old media.
    pub fn prepare(&mut self, scene_id: &str) -> Result<()> {
        let resource = self
            .catalog
            .scenes
            .get(scene_id)
            .ok_or_else(|| format!("missing scene resource '{scene_id}'"))?;
        if resource.resource != format!("text://{scene_id}") {
            return Err(format!(
                "cannot preload scene '{scene_id}': unsupported/missing text resource '{}'",
                resource.resource
            )
            .into());
        }
        let mut narratives = BTreeMap::new();
        let mut actions = BTreeMap::new();
        for context in &self.index.contexts {
            if self.catalog.contexts.get(&context.id).map(String::as_str) != Some(scene_id) {
                continue;
            }
            for cue in &context.cues {
                let Some(narrative) = &cue.narrative_id else {
                    continue;
                };
                let presentation = self
                    .catalog
                    .presentations
                    .get(narrative)
                    .ok_or_else(|| format!("missing presentation '{narrative}'"))?;
                // This concrete text renderer implements authored fixed-duration
                // text or explicit reading callbacks; other renderers export
                // their own supported persistent/instant behaviors.
                let authored = &self.authored_presentations[narrative];
                if (presentation.completion == Completion::Timed && authored.duration_ms.is_none())
                    || !matches!(
                        presentation.completion,
                        Completion::Timed | Completion::Callback
                    )
                {
                    return Err(format!(
                        "text renderer does not implement {:?} presentation '{narrative}'",
                        presentation.completion
                    )
                    .into());
                }
                let text = self
                    .registry
                    .narratives
                    .get(narrative)
                    .ok_or_else(|| format!("missing actual text resource '{narrative}'"))?;
                narratives.insert(narrative.clone(), text.clone());
            }
            for action in &context.actions {
                let text = self
                    .registry
                    .actions
                    .get(&action.id)
                    .ok_or_else(|| format!("missing actual action label '{}'", action.id))?;
                actions.insert(action.id.clone(), text.clone());
            }
        }
        self.narratives = narratives;
        self.actions = actions;
        self.prepared_scene = Some(scene_id.into());
        Ok(())
    }
    pub fn prepared_scene(&self) -> Option<&str> {
        self.prepared_scene.as_deref()
    }
    pub fn narrative(&self, id: &str) -> Option<&str> {
        self.narratives.get(id).map(String::as_str)
    }
    pub fn action(&self, id: &str) -> Option<&str> {
        self.actions.get(id).map(String::as_str)
    }
}
pub fn export(args: &[OsString]) -> Result<()> {
    let [story, configuration, text] = args else {
        return Err("Usage: grimm-cli export-text-catalog STORY.yaml SCENES.yaml TEXT.yaml".into());
    };
    let configuration: SceneConfiguration =
        serde_yaml_ng::from_str(&fs::read_to_string(configuration)?)?;
    let resources = TextResources::from_sources(
        &fs::read_to_string(story)?,
        &configuration,
        &fs::read_to_string(text)?,
        None,
    )?;
    for warning in &resources.warnings {
        eprintln!("warning: {warning}");
    }
    println!("{}", serde_json::to_string_pretty(&resources.catalog)?);
    Ok(())
}
