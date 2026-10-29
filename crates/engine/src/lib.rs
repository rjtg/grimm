//! Headless M0 stub. Only this crate commits canonical state changes.

use grimm_semantic::{GameAction, InteractionOffer, SemanticOutput};

/// State is private; callers can only obtain owned, player-visible projections.
///
/// ```compile_fail
/// let mut engine = grimm_engine::Engine::default();
/// engine.completed = true;
/// ```
#[derive(Debug, Default)]
pub struct Engine {
    completed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionError {
    StaleRevision,
    InvalidChoice,
}

impl Engine {
    /// Return only the information this demo intentionally reveals to the player.
    pub fn output(&self) -> SemanticOutput {
        if self.completed {
            SemanticOutput {
                revision: 1,
                text: "The engine accepted your choice. Demo complete.".into(),
                offers: vec![],
            }
        } else {
            SemanticOutput {
                revision: 0,
                text: "Welcome to GRIMM's architecture demo.".into(),
                offers: vec![InteractionOffer {
                    id: "continue".into(),
                    label: "Continue".into(),
                }],
            }
        }
    }

    /// Validate a proposal before committing the stub's single transition.
    pub fn apply(&mut self, action: &GameAction) -> Result<SemanticOutput, ActionError> {
        let revision = u64::from(self.completed);
        if action.revision != revision {
            return Err(ActionError::StaleRevision);
        }
        if self.completed || action.choice_id != "continue" {
            return Err(ActionError::InvalidChoice);
        }
        self.completed = true;
        Ok(self.output())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_and_stale_proposals_do_not_commit() {
        let mut engine = Engine::default();
        let initial = engine.output();
        for (action, error) in [
            (
                GameAction {
                    revision: 9,
                    choice_id: "continue".into(),
                },
                ActionError::StaleRevision,
            ),
            (
                GameAction {
                    revision: 0,
                    choice_id: "invented".into(),
                },
                ActionError::InvalidChoice,
            ),
        ] {
            assert_eq!(engine.apply(&action), Err(error));
            assert_eq!(engine.output(), initial);
        }
        let action = GameAction {
            revision: 0,
            choice_id: "continue".into(),
        };
        let final_output = engine.apply(&action).unwrap();
        assert!(final_output.offers.is_empty());
        assert_eq!(final_output.revision, 1);
        assert_eq!(engine.apply(&action), Err(ActionError::StaleRevision));
        assert_eq!(
            engine.apply(&GameAction {
                revision: 1,
                ..action
            }),
            Err(ActionError::InvalidChoice)
        );
        assert_eq!(engine.output(), final_output);
    }

    #[test]
    fn same_actions_produce_same_outputs() {
        let action = GameAction {
            revision: 0,
            choice_id: "continue".into(),
        };
        assert_eq!(
            Engine::default().apply(&action),
            Engine::default().apply(&action)
        );
    }
}
