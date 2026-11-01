//! Opt-in M4.6 host configuration, lifecycle feedback and boundary checkpoints.
//! Core story rules remain the same schema-2 SceneStory used by text games.
use crate::{
    scene::{CueInput, CueSheet, SceneAction},
    world::WorldSnapshot,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneConfiguration {
    pub schema_version: u32,
    pub id: String,
    pub version: u32,
    pub story_id: String,
    pub story_version: u32,
    /// Explicit context -> presentation scene mapping. Host-only configuration.
    pub scenes: BTreeMap<String, String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenePhase {
    Prepare,
    Active,
    Ending,
    Stable,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenePresentation {
    pub phase: ScenePhase,
    pub scene_id: String,
    pub epoch: u64,
    pub next_scene_id: Option<String>,
    pub ready: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneCueSheet {
    pub protocol_version: u32,
    pub scene: ScenePresentation,
    /// One full replacement ordered list, using the existing cue contract.
    pub plan: CueSheet,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneEvent {
    Ready,
    Begin,
    End,
    Prepare,
    PreparationFailed,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SceneFeedback {
    pub revision: u64,
    pub scene_id: String,
    pub epoch: u64,
    pub event: SceneEvent,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum LifecycleInput {
    Action(SceneAction),
    Cue(CueInput),
    Observe {
        revision: u64,
        scene_time_ms: u64,
    },
    Scene(SceneFeedback),
    /// Engine-generated restore reservation, never accepted from a player.
    RestoreEpoch {
        revision: u64,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryCheckpoint {
    pub schema_version: u32,
    pub story_id: String,
    pub story_version: u32,
    pub configuration_id: String,
    pub configuration_version: u32,
    pub revision: u64,
    pub history: Vec<LifecycleInput>,
    pub world: WorldSnapshot,
}
