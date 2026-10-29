//! Version-1 authoring data. The engine validates these before use.

use crate::{Ending, NarrativeContent};
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, MapAccess, Visitor},
};
use std::collections::BTreeMap;
use std::fmt;

pub const STORY_SCHEMA_VERSION: u32 = 1;
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    pub id: String,
    pub content: Vec<NarrativeContent>,
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub ending: Option<Ending>,
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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Condition {
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
