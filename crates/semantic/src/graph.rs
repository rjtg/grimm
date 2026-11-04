//! Draft-0.3 graph declarations. The host interprets condition/effect parameters
//! and executes scheduled cues; these types contain no executable rule logic.
use crate::story::Value;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const GRAPH_SCHEMA_VERSION: u32 = 1;
pub const CUE_SHEET_VERSION: u32 = 1;
pub const CONTINUATION_VERSION: u32 = 1;
pub type Parameters = BTreeMap<String, Value>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoryGraph {
    pub schema_version: u32,
    pub id: String,
    pub version: u32,
    pub start: String,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    pub id: String,
    #[serde(default)]
    pub cues: Vec<Cue>,
    #[serde(default, deserialize_with = "unique_edges")]
    pub edges: BTreeMap<String, Edge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edge {
    pub target: String,
    #[serde(default = "exposed")]
    pub expose: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConditionCall>,
}
fn exposed() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConditionCall {
    pub condition_id: String,
    #[serde(default)]
    pub args: Parameters,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cue {
    pub cue_id: String,
    pub at_ms: u64,
    pub instruction: Instruction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Instruction {
    Presentation {
        presentation_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        duration_ms: Option<u64>,
        #[serde(default)]
        args: Parameters,
    },
    Effect {
        effect_id: String,
        #[serde(default)]
        args: Parameters,
    },
    TraverseEdge {
        edge_id: String,
    },
}

/// Always a complete replacement. No pending/started/completed execution state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CueSheet {
    pub protocol_version: u32,
    pub node_id: String,
    pub epoch: u64,
    pub cues: Vec<Cue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoryContinuation {
    pub schema_version: u32,
    pub story_id: String,
    pub story_version: u32,
    pub node_id: String,
    pub epoch: u64,
}

/// Optional consumers can record accepted traversal without owning a replay log.
/// Sequence uses the destination epoch; restore can reserve gaps in the sequence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathEvent {
    pub sequence: u64,
    pub from_node: String,
    pub from_epoch: u64,
    pub edge_id: String,
    pub to_node: String,
    pub to_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraversalResult {
    pub cue_sheet: CueSheet,
    pub path_event: PathEvent,
}

fn unique_edges<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<BTreeMap<String, Edge>, D::Error> {
    struct Unique;
    impl<'de> serde::de::Visitor<'de> for Unique {
        type Value = BTreeMap<String, Edge>;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("unique named edges")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(
            self,
            mut map: M,
        ) -> Result<Self::Value, M::Error> {
            let mut edges = BTreeMap::new();
            while let Some((id, edge)) = map.next_entry::<String, Edge>()? {
                if edges.insert(id.clone(), edge).is_some() {
                    return Err(serde::de::Error::custom(format!("duplicate edge '{id}'")));
                }
            }
            Ok(edges)
        }
    }
    deserializer.deserialize_map(Unique)
}
