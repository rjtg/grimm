//! Host-only transactional storage DTOs. Never included in renderer Cue Sheets.
use crate::story::{Value, Variables};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldScope {
    pub context_id: String,
    pub epoch: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldSnapshot {
    pub revision: u64,
    pub scope: WorldScope,
    pub globals: Variables,
    pub locals: Variables,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VariableScope {
    Global,
    Local,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum StateMutation {
    Set {
        scope: VariableScope,
        name: String,
        value: Value,
    },
    Add {
        scope: VariableScope,
        name: String,
        delta: i64,
    },
    Enter {
        scope: WorldScope,
        locals: Variables,
    },
}
