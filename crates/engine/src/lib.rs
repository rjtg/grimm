//! Headless authored CYOA runtime. Only the engine commits canonical state.
mod demo;
mod expressions;
mod playability;
mod validation;

pub use playability::{Deadlock, PlayabilityLimits, PlayabilityReport, PlayabilityStep};

use expressions::{apply_effects, evaluate, same_type};
use grimm_semantic::{
    GameAction, InteractionOffer, SemanticOutput,
    save::{SAVE_SCHEMA_VERSION, SaveGame},
    story::{Choice, Story, Variables},
};
use std::fmt;

/// Canonical state is private; projections and checkpoints are owned copies.
///
/// ```compile_fail
/// let mut engine = grimm_engine::Engine::default();
/// engine.variables.clear();
/// ```
#[derive(Debug)]
pub struct Engine {
    story: Story,
    node: usize,
    variables: Variables,
    revision: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub String);
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for ValidationError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    StaleRevision,
    InvalidChoice,
    EffectOverflow { variable: String },
    NoOffers { node: String },
    RevisionOverflow,
}
impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleRevision => f.write_str("choice refers to a stale session revision"),
            Self::InvalidChoice => f.write_str("choice is not currently offered"),
            Self::EffectOverflow { variable } => {
                write!(f, "integer effect overflows variable '{variable}'")
            }
            Self::NoOffers { node } => write!(
                f,
                "transition would leave node '{node}' without an eligible choice"
            ),
            Self::RevisionOverflow => f.write_str("session revision overflow"),
        }
    }
}
impl std::error::Error for ActionError {}

impl Default for Engine {
    fn default() -> Self {
        Self::new(demo::story()).expect("valid built-in M0 demo")
    }
}

fn eligible(choice: &Choice, variables: &Variables) -> bool {
    choice
        .condition
        .as_ref()
        .is_none_or(|condition| evaluate(condition, variables))
}

impl Engine {
    pub fn new(story: Story) -> Result<Self, ValidationError> {
        validation::validate(&story)?;
        let node = story
            .nodes
            .iter()
            .position(|node| node.id == story.start)
            .expect("validated start");
        let variables = story.variables.clone();
        let engine = Self {
            story,
            node,
            variables,
            revision: 0,
        };
        engine.ensure_playable(engine.node, &engine.variables)?;
        Ok(engine)
    }

    fn ensure_playable(&self, node: usize, variables: &Variables) -> Result<(), ValidationError> {
        let node = &self.story.nodes[node];
        if node.ending.is_some() {
            return Ok(());
        }
        let mut failures = Vec::new();
        for choice in node
            .choices
            .iter()
            .filter(|choice| eligible(choice, variables))
        {
            match self.preview(choice, variables) {
                Ok(_) => return Ok(()),
                Err(error) => failures.push(format!("choice '{}': {error}", choice.id)),
            }
        }
        let reason = if failures.is_empty() {
            "no eligible choices in this state".into()
        } else {
            format!("no executable choices ({})", failures.join("; "))
        };
        Err(ValidationError(format!("node '{}': {reason}", node.id)))
    }

    /// Preview exactly one transition without committing. Destination conditions
    /// are checked after effects; this is not a search for a winning playthrough.
    fn preview(
        &self,
        choice: &Choice,
        variables: &Variables,
    ) -> Result<(usize, Variables), ActionError> {
        if !eligible(choice, variables) {
            return Err(ActionError::InvalidChoice);
        }
        let mut variables = variables.clone();
        apply_effects(&choice.effects, &mut variables)?;
        let target = self
            .story
            .nodes
            .iter()
            .position(|node| node.id == choice.target)
            .expect("validated target");
        let node = &self.story.nodes[target];
        if node.ending.is_none()
            && !node
                .choices
                .iter()
                .any(|choice| eligible(choice, &variables))
        {
            return Err(ActionError::NoOffers {
                node: node.id.clone(),
            });
        }
        Ok((target, variables))
    }

    /// Returns visible authored content and executable offers, never private state.
    /// Each offer passes the same one-transition preview as `apply`; without a
    /// state/revision change it cannot fail on effects or destination conditions.
    pub fn output(&self) -> SemanticOutput {
        let node = &self.story.nodes[self.node];
        SemanticOutput {
            revision: self.revision,
            content: node.content.clone(),
            offers: node
                .choices
                .iter()
                .filter(|choice| {
                    self.revision < u64::MAX && self.preview(choice, &self.variables).is_ok()
                })
                .map(|choice| InteractionOffer {
                    id: choice.id.clone(),
                    label: choice.label.clone(),
                })
                .collect(),
            ending: node.ending.clone(),
        }
    }

    pub fn apply(&mut self, action: &GameAction) -> Result<SemanticOutput, ActionError> {
        if action.revision != self.revision {
            return Err(ActionError::StaleRevision);
        }
        let choice = self.story.nodes[self.node]
            .choices
            .iter()
            .find(|choice| choice.id == action.choice_id)
            .ok_or(ActionError::InvalidChoice)?;
        let (target, variables) = self.preview(choice, &self.variables)?;
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(ActionError::RevisionOverflow)?;
        self.variables = variables;
        self.node = target;
        self.revision = revision;
        Ok(self.output())
    }

    /// Host-only checkpoint. Adapters/renderers never receive this private DTO.
    pub fn save(&self) -> SaveGame {
        SaveGame {
            schema_version: SAVE_SCHEMA_VERSION,
            story_id: self.story.id.clone(),
            story_version: self.story.version,
            node_id: self.story.nodes[self.node].id.clone(),
            variables: self.variables.clone(),
            revision: self.revision,
        }
    }

    /// Validate before replacement so failures preserve the active session.
    pub fn restore(&mut self, save: SaveGame) -> Result<SemanticOutput, ValidationError> {
        if save.schema_version != SAVE_SCHEMA_VERSION {
            return Err(ValidationError(format!(
                "unsupported save schema version {}; expected {SAVE_SCHEMA_VERSION}",
                save.schema_version
            )));
        }
        if save.story_id != self.story.id || save.story_version != self.story.version {
            return Err(ValidationError(format!(
                "save belongs to story '{}' version {}; expected '{}' version {}",
                save.story_id, save.story_version, self.story.id, self.story.version
            )));
        }
        let node = self
            .story
            .nodes
            .iter()
            .position(|node| node.id == save.node_id)
            .ok_or_else(|| {
                ValidationError(format!("save node '{}' does not exist", save.node_id))
            })?;
        if save.variables.len() != self.story.variables.len() {
            return Err(ValidationError(
                "save variables do not match the story declarations".into(),
            ));
        }
        for (name, initial) in &self.story.variables {
            if !save
                .variables
                .get(name)
                .is_some_and(|value| same_type(initial, value))
            {
                return Err(ValidationError(format!(
                    "save variable '{name}' is missing or has the wrong type"
                )));
            }
        }
        self.ensure_playable(node, &save.variables)
            .map_err(|error| ValidationError(format!("save {error}")))?;
        // Loading an older checkpoint must not reactivate an already issued
        // action. Preserve saved revisions when possible, but never reuse one
        // within this engine instance.
        let revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| ValidationError("cannot restore: session revision exhausted".into()))?
            .max(save.revision);
        self.node = node;
        self.variables = save.variables;
        self.revision = revision;
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
