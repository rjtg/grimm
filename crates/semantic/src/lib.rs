//! Renderer-independent proposals and player-visible engine output.
//!
//! These M0 types are provisional; they are not a stable persistence format.

use serde::{Deserialize, Serialize};

/// A proposal referring to an offer from a particular engine revision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameAction {
    pub revision: u64,
    pub choice_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionOffer {
    pub id: String,
    pub label: String,
}

/// A player-visible projection, never an unfiltered event stream or world state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticOutput {
    pub revision: u64,
    pub text: String,
    pub offers: Vec<InteractionOffer>,
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
            revision: 7,
            text: "Exact authored text.\nSecond line.".into(),
            offers: vec![InteractionOffer {
                id: "continue".into(),
                label: "Continue".into(),
            }],
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
