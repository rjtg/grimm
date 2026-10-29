//! Private session checkpoint DTO, never sent to an output adapter or renderer.
use crate::story::{Variables, unique_variables};
use serde::{Deserialize, Serialize};

pub const SAVE_SCHEMA_VERSION: u32 = 1;
pub const TEMPORAL_SAVE_SCHEMA_VERSION: u32 = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveGame {
    pub schema_version: u32,
    pub story_id: String,
    pub story_version: u32,
    pub node_id: String,
    #[serde(deserialize_with = "unique_variables")]
    pub variables: Variables,
    pub revision: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporal: Option<crate::time::TemporalSave>,
}
