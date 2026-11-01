//! Renderer-owned scene correlation and feedback; no graph or world-state DTOs.
use crate::cues::CueSheet;
use serde::{Deserialize, Serialize};
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
