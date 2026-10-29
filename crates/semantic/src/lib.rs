//! Renderer-independent proposals and player-visible engine output.
//!
//! Authoring definitions and checkpoints are host/engine DTOs, not projections.

pub mod flow;
pub mod save;
pub mod story;
pub mod time;

use serde::{Deserialize, Serialize};

/// A proposal referring to an offer from a particular engine revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameAction {
    pub revision: u64,
    pub choice_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InteractionOffer {
    pub id: String,
    pub label: String,
}

/// Authored meaning and exact prose. Formatting belongs to output adapters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum NarrativeContent {
    Narration { text: String },
    Dialogue { speaker: String, text: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ending {
    pub id: String,
    pub title: String,
}

/// A player-visible projection, never an unfiltered event stream or world state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticOutput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flow: Option<flow::FlowView>,
    pub revision: u64,
    pub content: Vec<NarrativeContent>,
    pub offers: Vec<InteractionOffer>,
    pub ending: Option<Ending>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision: Option<time::DecisionView>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_round_trips() {
        let action = GameAction {
            revision: 7,
            choice_id: "continue".into(),
        };
        let output = SemanticOutput {
            flow: None,
            decision: None,
            revision: 7,
            content: vec![
                NarrativeContent::Narration {
                    text: "Exact authored text.\nSecond line.".into(),
                },
                NarrativeContent::Dialogue {
                    speaker: "Witness".into(),
                    text: "I saw it.".into(),
                },
            ],
            offers: vec![InteractionOffer {
                id: "continue".into(),
                label: "Continue".into(),
            }],
            ending: None,
        };
        assert_eq!(
            action,
            serde_json::from_str(&serde_json::to_string(&action).unwrap()).unwrap()
        );
        assert_eq!(
            output,
            serde_json::from_str(&serde_json::to_string(&output).unwrap()).unwrap()
        );
    }
}
