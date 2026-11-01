//! Offline game-production data. This is neither a Cue Sheet extension nor an
//! executable staging language. Graph references are checked through the M3 index.
use grimm_semantic::{authoring::StoryIndex, lifecycle::SceneConfiguration};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    fs,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Screenplay {
    pub schema_version: u32,
    pub story_id: String,
    pub story_version: u32,
    pub scenes: Vec<Scene>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    pub scene_id: String,
    pub context_id: String,
    pub slugline: String,
    pub beats: Vec<Beat>,
    pub decision_actions: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Beat {
    pub beat_id: String,
    pub cue_id: String,
    pub narrative_id: String,
    pub content: Vec<Content>,
    #[serde(default)]
    pub staging: Vec<Direction>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Content {
    Narration { text: String },
    Dialogue { speaker: String, text: String },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Direction {
    pub kind: DirectionKind,
    pub description: String,
    /// Editorial target, never an override of authored action/cue duration.
    pub duration_ms: Option<u64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectionKind {
    Shot,
    Blocking,
    Movement,
    Camera,
    Transition,
    Pause,
    Environment,
    Audio,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Bindings {
    #[serde(deserialize_with = "crate::catalog::unique_map")]
    narratives: BTreeMap<String, String>,
    #[serde(deserialize_with = "crate::catalog::unique_map")]
    actions: BTreeMap<String, String>,
}
fn require(ok: bool, message: impl Into<String>) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.into().into())
    }
}
fn id(value: &str) -> bool {
    value.split('.').all(|part| {
        let mut chars = part.bytes();
        chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
    })
}
fn unique(ids: &mut BTreeSet<String>, value: &str) -> Result<()> {
    require(
        id(value),
        format!("invalid production identifier '{value}' (expected lowercase ASCII components)"),
    )?;
    require(
        ids.insert(value.into()),
        format!("duplicate production identifier '{value}'"),
    )
}
/// Validate a deliberately partial production document against actual rules and
/// text, never by filename conventions or a currently offered execution path.
pub fn validate(
    screenplay: &Screenplay,
    index: &StoryIndex,
    text_yaml: &str,
    configuration: &SceneConfiguration,
) -> Result<()> {
    require(
        screenplay.schema_version == 1,
        "unsupported screenplay schema_version",
    )?;
    require(
        screenplay.story_id == index.story_id && screenplay.story_version == index.story_version,
        "screenplay story identity/version mismatch",
    )?;
    require(!screenplay.scenes.is_empty(), "screenplay has no scenes")?;
    require(
        configuration.schema_version == 1
            && configuration.version > 0
            && configuration.story_id == index.story_id
            && configuration.story_version == index.story_version,
        "screenplay scene configuration identity/version mismatch",
    )?;
    let bindings: Bindings = serde_yaml_ng::from_str(text_yaml)?;
    let mut ids = BTreeSet::new();
    let mut contexts = BTreeSet::new();
    for scene in &screenplay.scenes {
        unique(&mut ids, &scene.scene_id)?;
        require(
            contexts.insert(&scene.context_id),
            format!("duplicate context mapping '{}'", scene.context_id),
        )?;
        require(
            !scene.slugline.trim().is_empty(),
            format!("{}: empty slugline", scene.scene_id),
        )?;
        let context = index
            .contexts
            .iter()
            .find(|c| c.id == scene.context_id)
            .ok_or_else(|| format!("{}: unknown context '{}'", scene.scene_id, scene.context_id))?;
        require(
            !scene.beats.is_empty(),
            format!("{}: no beats", scene.scene_id),
        )?;
        require(
            configuration.scenes.get(&scene.context_id) == Some(&scene.scene_id),
            format!(
                "{}: scene scope differs from lifecycle configuration",
                scene.scene_id
            ),
        )?;
        let mut mapped = BTreeSet::new();
        for beat in &scene.beats {
            unique(&mut ids, &beat.beat_id)?;
            require(
                mapped.insert(&beat.cue_id),
                format!(
                    "{}: duplicate cue mapping '{}'",
                    scene.scene_id, beat.cue_id
                ),
            )?;
            let cue = context
                .cues
                .iter()
                .find(|c| c.cue_id == beat.cue_id)
                .ok_or_else(|| {
                    format!(
                        "{}: unknown cue '{}' in context '{}'",
                        beat.beat_id, beat.cue_id, scene.context_id
                    )
                })?;
            require(
                cue.narrative_id.as_deref() == Some(&beat.narrative_id),
                format!("{}: cue/narrative mapping mismatch", beat.beat_id),
            )?;
            let source = bindings.narratives.get(&beat.narrative_id).ok_or_else(|| {
                format!(
                    "{}: missing text binding '{}'",
                    beat.beat_id, beat.narrative_id
                )
            })?;
            require(
                !beat.content.is_empty(),
                format!("{}: no content", beat.beat_id),
            )?;
            let mut parts = vec![];
            for content in &beat.content {
                let text = match content {
                    Content::Narration { text } => text,
                    Content::Dialogue { speaker, text } => {
                        require(id(speaker), format!("{}: invalid speaker ID", beat.beat_id))?;
                        text
                    }
                };
                require(
                    !text.trim().is_empty(),
                    format!("{}: empty content", beat.beat_id),
                )?;
                parts.push(text.as_str());
            }
            require(
                parts.join("\n\n") == *source,
                format!(
                    "{}: content differs from original text binding '{}'",
                    beat.beat_id, beat.narrative_id
                ),
            )?;
            for direction in &beat.staging {
                require(
                    !direction.description.trim().is_empty(),
                    format!("{}: empty staging direction", beat.beat_id),
                )?;
                require(
                    direction
                        .duration_ms
                        .is_none_or(|time| time <= i64::MAX as u64),
                    format!("{}: staging duration overflow", beat.beat_id),
                )?;
            }
        }
        let mut actions = BTreeSet::new();
        for action in &scene.decision_actions {
            require(
                actions.insert(action),
                format!("{}: duplicate decision action '{action}'", scene.scene_id),
            )?;
            require(
                context.actions.iter().any(|a| a.id == *action),
                format!("{}: unknown decision action '{action}'", scene.scene_id),
            )?;
            require(
                bindings
                    .actions
                    .get(action)
                    .is_some_and(|label| !label.trim().is_empty()),
                format!("{}: missing action label '{action}'", scene.scene_id),
            )?;
        }
    }
    Ok(())
}
pub fn run(args: &[OsString]) -> Result<()> {
    let [story, screenplay, text, scenes] = args else {
        return Err(
            "Usage: grimm-cli check-screenplay STORY.yaml SCREENPLAY.yaml TEXT.yaml SCENES.yaml"
                .into(),
        );
    };
    let report = crate::authoring::inspect(&story.to_string_lossy(), &fs::read_to_string(story)?);
    require(
        !report.has_errors(),
        "story compilation/structural validation failed; run check-scene for diagnostics",
    )?;
    let index = report.index.expect("validated story index");
    let document: Screenplay = serde_yaml_ng::from_str(&fs::read_to_string(screenplay)?)?;
    let configuration: SceneConfiguration = serde_yaml_ng::from_str(&fs::read_to_string(scenes)?)?;
    validate(
        &document,
        &index,
        &fs::read_to_string(text)?,
        &configuration,
    )?;
    println!(
        "{}: valid screenplay fragment ({} scene(s), {} beat(s)); remaining story coverage and staging execution are not validated.",
        screenplay.to_string_lossy(),
        document.scenes.len(),
        document.scenes.iter().map(|s| s.beats.len()).sum::<usize>()
    );
    Ok(())
}
