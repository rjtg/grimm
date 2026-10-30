//! Renderer-only plan and feedback. No engine, graph or world-state dependency.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CueSheet {
    pub revision: u64,
    pub clock: Clock,
    pub cues: Vec<Cue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clock {
    pub epoch: u64,
    pub elapsed_ms: u64,
    pub running: bool,
    pub deadline_ms: Option<u64>,
    pub visible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cue {
    pub cue_id: String,
    pub at_ms: Option<u64>,
    pub after_cue: Option<String>,
    #[serde(flatten)]
    pub instruction: Instruction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Instruction {
    Narrative {
        narrative_id: String,
    },
    Actions {
        available_actions: Vec<String>,
    },
    Timer {
        start_ms: Option<u64>,
        extend_ms: Option<u64>,
        visible: Option<bool>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CueEvent {
    Started,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CueFeedback {
    pub revision: u64,
    pub cue_id: String,
    pub scene_time_ms: u64,
    pub event: CueEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionInput {
    pub revision: u64,
    pub action_id: String,
    pub scene_time_ms: u64,
}
