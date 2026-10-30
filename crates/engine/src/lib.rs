//! Headless authored CYOA runtime. Only the engine commits canonical state.
mod demo;
mod expressions;
mod flow;
mod playability;
pub mod scene;
mod temporal;
mod validation;

pub use flow::FlowError;
pub use playability::{Deadlock, PlayabilityLimits, PlayabilityReport, PlayabilityStep};
pub use temporal::TimeError;

use expressions::{apply_effects, evaluate, same_type};
use grimm_semantic::{
    GameAction, InteractionOffer, SemanticOutput,
    flow::{FlowPhase, FlowView},
    save::{FLOW_SAVE_SCHEMA_VERSION, SAVE_SCHEMA_VERSION, SaveGame, TEMPORAL_SAVE_SCHEMA_VERSION},
    story::{Choice, FLOW_STORY_SCHEMA_VERSION, Story, Variables},
    time::{DecisionPhase, TemporalSave},
};
use std::fmt;
use temporal::ActiveDecision;

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
    world_minutes: Option<u64>,
    decision: Option<ActiveDecision>,
    flow: Option<FlowView>,
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
    WorldTimeOverflow,
    DecisionNotActive,
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
            Self::WorldTimeOverflow => f.write_str("fictional world-time cost overflows"),
            Self::DecisionNotActive => {
                f.write_str("decision requires presentation readiness and must not be paused")
            }
        }
    }
}
impl std::error::Error for ActionError {}

impl Default for Engine {
    fn default() -> Self {
        Self::new(demo::story()).expect("valid built-in M0 demo")
    }
}

fn eligible(choice: &Choice, variables: &Variables, world_minutes: Option<u64>) -> bool {
    choice
        .condition
        .as_ref()
        .is_none_or(|condition| evaluate(condition, variables, world_minutes.unwrap_or(0)))
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
        let world_minutes = story.time.as_ref().map(|time| time.start_minutes);
        let decision = ActiveDecision::for_node(&story.nodes[node]);
        let flow = flow::for_node(&story.nodes[node]);
        let engine = Self {
            story,
            node,
            variables,
            revision: 0,
            world_minutes,
            decision,
            flow,
        };
        engine.ensure_playable(engine.node, &engine.variables, world_minutes)?;
        Ok(engine)
    }

    fn ensure_playable(
        &self,
        node: usize,
        variables: &Variables,
        world: Option<u64>,
    ) -> Result<(), ValidationError> {
        let node = &self.story.nodes[node];
        // Validated timeouts provide an executable fallback, even if all ordinary
        // choices are unavailable. Readiness is a lifecycle input, not a deadlock.
        if node.ending.is_some() || node.decision.is_some() {
            return Ok(());
        }
        if let Some(choice) = flow::continuation(node) {
            return self
                .preview(&choice, variables, world)
                .map(|_| ())
                .map_err(|error| {
                    ValidationError(format!("node '{}' continuation: {error}", node.id))
                });
        }
        let mut failures = Vec::new();
        for choice in node
            .choices
            .iter()
            .filter(|choice| eligible(choice, variables, world))
        {
            match self.preview(choice, variables, world) {
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

    /// Pure one-transition preview shared by offers, actions and authoring checks.
    fn preview(
        &self,
        choice: &Choice,
        variables: &Variables,
        world: Option<u64>,
    ) -> Result<(usize, Variables, Option<u64>), ActionError> {
        if !eligible(choice, variables, world) {
            return Err(ActionError::InvalidChoice);
        }
        let world = match world {
            Some(current) => Some(
                current
                    .checked_add(choice.advance_minutes)
                    .ok_or(ActionError::WorldTimeOverflow)?,
            ),
            None if choice.advance_minutes == 0 => None,
            None => return Err(ActionError::WorldTimeOverflow), // prevented by validation
        };
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
            && node.decision.is_none()
            && node.continue_to.is_none()
            && !node
                .choices
                .iter()
                .any(|choice| eligible(choice, &variables, world))
        {
            return Err(ActionError::NoOffers {
                node: node.id.clone(),
            });
        }
        Ok((target, variables, world))
    }

    /// Visible authored content and deadline only; private chronology stays inside.
    pub fn output(&self) -> SemanticOutput {
        let node = &self.story.nodes[self.node];
        SemanticOutput {
            flow: self.flow.clone(),
            revision: self.revision,
            content: node.content.clone(),
            offers: node
                .choices
                .iter()
                .filter(|choice| {
                    self.revision < u64::MAX
                        && self
                            .flow
                            .as_ref()
                            .is_none_or(|flow| !flow.paused && flow.phase != FlowPhase::Presenting)
                        && self
                            .decision
                            .as_ref()
                            .is_none_or(|decision| decision.view.phase != DecisionPhase::Paused)
                        && self
                            .preview(choice, &self.variables, self.world_minutes)
                            .is_ok_and(|(target, _, _)| {
                                self.revision != u64::MAX - 1
                                    || (self.story.nodes[target].decision.is_none()
                                        && flow::for_node(&self.story.nodes[target]).is_none())
                            })
                })
                .map(|choice| InteractionOffer {
                    id: choice.id.clone(),
                    label: choice.label.clone(),
                })
                .collect(),
            ending: node.ending.clone(),
            decision: self.decision.as_ref().map(|decision| decision.view.clone()),
        }
    }

    pub fn apply(&mut self, action: &GameAction) -> Result<SemanticOutput, ActionError> {
        if action.revision != self.revision {
            return Err(ActionError::StaleRevision);
        }
        if self
            .decision
            .as_ref()
            .is_some_and(|decision| decision.view.phase != DecisionPhase::Active)
        {
            return Err(ActionError::DecisionNotActive);
        }
        if self
            .flow
            .as_ref()
            .is_some_and(|flow| flow.paused || flow.phase != FlowPhase::Interactive)
        {
            return Err(ActionError::DecisionNotActive);
        }
        let choice = self.story.nodes[self.node]
            .choices
            .iter()
            .find(|choice| choice.id == action.choice_id)
            .ok_or(ActionError::InvalidChoice)?;
        let (target, variables, world) =
            self.preview(choice, &self.variables, self.world_minutes)?;
        self.commit(target, variables, world)
    }

    fn commit(
        &mut self,
        target: usize,
        variables: Variables,
        world: Option<u64>,
    ) -> Result<SemanticOutput, ActionError> {
        let revision = self
            .revision
            .checked_add(1)
            .ok_or(ActionError::RevisionOverflow)?;
        if revision == u64::MAX
            && (self.story.nodes[target].decision.is_some()
                || flow::for_node(&self.story.nodes[target]).is_some())
        {
            return Err(ActionError::RevisionOverflow);
        }
        self.variables = variables;
        self.node = target;
        self.world_minutes = world;
        self.revision = revision;
        self.decision = ActiveDecision::for_node(&self.story.nodes[target]);
        self.flow = flow::for_node(&self.story.nodes[target]);
        Ok(self.output())
    }

    /// Host-only checkpoint. Adapters/renderers never receive this private DTO.
    pub fn save(&self) -> SaveGame {
        SaveGame {
            flow: self.flow.clone(),
            schema_version: if self.story.schema_version == FLOW_STORY_SCHEMA_VERSION {
                FLOW_SAVE_SCHEMA_VERSION
            } else if self.world_minutes.is_some() {
                TEMPORAL_SAVE_SCHEMA_VERSION
            } else {
                SAVE_SCHEMA_VERSION
            },
            story_id: self.story.id.clone(),
            story_version: self.story.version,
            node_id: self.story.nodes[self.node].id.clone(),
            variables: self.variables.clone(),
            revision: self.revision,
            temporal: self.world_minutes.map(|world_minutes| TemporalSave {
                world_minutes,
                decision: self.decision.as_ref().map(|decision| decision.view.clone()),
            }),
        }
    }

    /// Validate before replacement. Offline time is frozen; old inputs stay stale.
    pub fn restore(&mut self, save: SaveGame) -> Result<SemanticOutput, ValidationError> {
        if save.schema_version != SAVE_SCHEMA_VERSION
            && save.schema_version != TEMPORAL_SAVE_SCHEMA_VERSION
            && save.schema_version != FLOW_SAVE_SCHEMA_VERSION
        {
            return Err(ValidationError(format!(
                "unsupported save schema version {}; expected 1, 2 or 3",
                save.schema_version
            )));
        }
        if (self.story.schema_version == FLOW_STORY_SCHEMA_VERSION)
            != (save.schema_version == FLOW_SAVE_SCHEMA_VERSION)
        {
            return Err(ValidationError(
                "story schema 3 requires save schema 3; legacy stories require legacy saves".into(),
            ));
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
        let (world, decision) = self.restore_temporal(&save, node)?;
        let flow = self.restore_flow(&save, node)?;
        self.ensure_playable(node, &save.variables, world)
            .map_err(|error| ValidationError(format!("save {error}")))?;
        let revision = if world.is_some() || self.story.schema_version == FLOW_STORY_SCHEMA_VERSION
        {
            // A fresh process must not reactivate a checkpoint's pending action
            // or cumulative clock observations when it acknowledges readiness.
            self.revision.max(save.revision).checked_add(1)
        } else {
            // Preserve the existing untimed v1 restoration behavior.
            self.revision
                .checked_add(1)
                .map(|revision| revision.max(save.revision))
        }
        .ok_or_else(|| ValidationError("cannot restore: session revision exhausted".into()))?;
        if revision == u64::MAX && (decision.is_some() || flow.is_some()) {
            return Err(ValidationError(
                "cannot restore an exhausted interaction revision".into(),
            ));
        }
        self.node = node;
        self.variables = save.variables;
        self.revision = revision;
        self.world_minutes = world;
        self.decision = decision;
        self.flow = flow;
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
