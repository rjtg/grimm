//! Version-1 authoring data. The engine validates these before use.

use crate::{Ending, NarrativeContent};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, Visitor},
};
use std::collections::BTreeMap;
use std::fmt;

pub const STORY_SCHEMA_VERSION: u32 = 1;
pub const TEMPORAL_STORY_SCHEMA_VERSION: u32 = 2;
pub const FLOW_STORY_SCHEMA_VERSION: u32 = 3;
pub type Variables = BTreeMap<String, Value>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Story {
    pub schema_version: u32,
    pub id: String,
    pub version: u32,
    pub title: String,
    pub start: String,
    #[serde(default, deserialize_with = "unique_variables")]
    pub variables: Variables,
    pub nodes: Vec<Node>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<WorldTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorldTime {
    #[serde(default)]
    pub start_minutes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub limit_ms: u64,
    pub timeout: Timeout,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Timeout {
    pub target: String,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continue_to: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub presentation_required: bool,
    pub id: String,
    pub content: Vec<NarrativeContent>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub ending: Option<Ending>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<Decision>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub id: String,
    pub label: String,
    pub target: String,
    #[serde(default)]
    pub condition: Option<Condition>,
    #[serde(default)]
    pub effects: Vec<Effect>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub advance_minutes: u64,
}

fn is_zero(value: &u64) -> bool {
    *value == 0
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Condition {
    WorldAtLeast { minutes: u64 },
    WorldAtMost { minutes: u64 },
    Equals { variable: String, value: Value },
    AtLeast { variable: String, value: i64 },
    AtMost { variable: String, value: i64 },
    All { conditions: Vec<Condition> },
    Any { conditions: Vec<Condition> },
    Not { condition: Box<Condition> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Effect {
    Set { variable: String, value: Value },
    Add { variable: String, amount: i64 },
}

/// Reject duplicate variable names instead of silently overwriting author input.
pub(crate) fn unique_variables<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Variables, D::Error> {
    struct Unique;
    impl<'de> Visitor<'de> for Unique {
        type Value = Variables;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a mapping with unique variable names")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Variables, M::Error> {
            let mut variables = Variables::new();
            while let Some((name, value)) = map.next_entry::<String, Value>()? {
                if variables.insert(name.clone(), value).is_some() {
                    return Err(de::Error::custom(format!("duplicate variable '{name}'")));
                }
            }
            Ok(variables)
        }
    }
    deserializer.deserialize_map(Unique)
}
