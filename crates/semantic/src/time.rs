//! Explicit host time/lifecycle inputs and the intentionally visible deadline view.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionPhase {
    AwaitingReady,
    Active,
    Paused,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionView {
    pub remaining_ms: u64,
    pub phase: DecisionPhase,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimeInput {
    pub revision: u64,
    pub event: TimeEvent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TimeEvent {
    Ready,
    Elapsed { total_ms: u64 },
    Pause,
    Resume,
    AdvanceWorld { minutes: u64 },
}

/// Private host checkpoint data, never part of a player projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TemporalSave {
    pub world_minutes: u64,
    pub decision: Option<DecisionView>,
}
