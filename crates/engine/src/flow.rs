use crate::{ActionError, Engine, ValidationError};
use grimm_semantic::{
    SemanticOutput,
    flow::{FlowEvent, FlowInput, FlowPhase, FlowView},
    save::SaveGame,
    story::{Choice, Node},
    time::DecisionPhase,
};
use std::fmt;

pub(crate) fn for_node(node: &Node) -> Option<FlowView> {
    (node.continue_to.is_some() || node.presentation_required).then_some(FlowView {
        phase: FlowPhase::Presenting,
        paused: false,
    })
}

/// Diagnostic/preview edge only; never exposed as an offer or GameAction.
pub(crate) fn continuation(node: &Node) -> Option<Choice> {
    node.continue_to.as_ref().map(|target| Choice {
        id: "<continue>".into(),
        label: "Acknowledged presentation".into(),
        target: target.clone(),
        condition: None,
        effects: vec![],
        advance_minutes: 0,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowError {
    StaleRevision,
    InvalidPhase,
    Transition(ActionError),
}
impl fmt::Display for FlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleRevision => f.write_str("presentation input refers to a stale revision"),
            Self::InvalidPhase => f.write_str("presentation input is invalid in the current phase"),
            Self::Transition(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for FlowError {}
impl From<ActionError> for FlowError {
    fn from(error: ActionError) -> Self {
        Self::Transition(error)
    }
}

impl Engine {
    pub(crate) fn preview_continuation(
        &self,
        choice: &Choice,
        variables: &grimm_semantic::story::Variables,
        world: Option<u64>,
    ) -> Result<(usize, grimm_semantic::story::Variables, Option<u64>), ActionError> {
        let (target, variables, world) = self.preview(choice, variables, world)?;
        self.ensure_playable(target, &variables, world)
            .map_err(|_| ActionError::NoOffers {
                node: self.story.nodes[target].id.clone(),
            })?;
        Ok((target, variables, world))
    }

    /// One explicit lifecycle input, at most one continuation, no renderer wait.
    pub fn flow_input(&mut self, input: &FlowInput) -> Result<SemanticOutput, FlowError> {
        if input.revision != self.revision {
            return Err(FlowError::StaleRevision);
        }
        let mut flow = self.flow.clone().ok_or(FlowError::InvalidPhase)?;
        let mut decision = self.decision.clone();
        let mut revision = self.revision;
        match input.event {
            FlowEvent::PresentationComplete
                if !flow.paused && flow.phase == FlowPhase::Presenting =>
            {
                if let Some(choice) = continuation(&self.story.nodes[self.node]) {
                    let (target, variables, world) =
                        self.preview_continuation(&choice, &self.variables, self.world_minutes)?;
                    return Ok(self.commit(target, variables, world)?);
                }
                flow.phase = FlowPhase::AwaitingInteraction;
                revision = self.next_flow_revision()?;
            }
            FlowEvent::InteractionReady
                if !flow.paused && flow.phase == FlowPhase::AwaitingInteraction =>
            {
                flow.phase = FlowPhase::Interactive;
                if let Some(decision) = &mut decision {
                    decision.view.phase = DecisionPhase::Active;
                    decision.last_elapsed_ms = 0;
                }
            }
            FlowEvent::Pause if !flow.paused => {
                flow.paused = true;
                revision = self.next_flow_revision()?;
                if let Some(decision) = &mut decision {
                    decision.view.phase = DecisionPhase::Paused;
                    decision.last_elapsed_ms = 0;
                }
            }
            FlowEvent::Resume if flow.paused => {
                flow.paused = false;
                if flow.phase == FlowPhase::Interactive {
                    flow.phase = FlowPhase::AwaitingInteraction;
                }
                revision = self.next_flow_revision()?;
                if let Some(decision) = &mut decision {
                    decision.view.phase = DecisionPhase::AwaitingReady;
                    decision.last_elapsed_ms = 0;
                }
            }
            _ => return Err(FlowError::InvalidPhase),
        }
        self.flow = Some(flow);
        self.decision = decision;
        self.revision = revision;
        Ok(self.output())
    }

    fn next_flow_revision(&self) -> Result<u64, ActionError> {
        self.revision
            .checked_add(1)
            .filter(|value| *value < u64::MAX)
            .ok_or(ActionError::RevisionOverflow)
    }

    pub(crate) fn restore_flow(
        &self,
        save: &SaveGame,
        node: usize,
    ) -> Result<Option<FlowView>, ValidationError> {
        let expected = for_node(&self.story.nodes[node]);
        let invalid =
            || ValidationError("save presentation phase does not match the node/deadline".into());
        match (&expected, &save.flow) {
            (None, None) => Ok(None),
            (Some(_), Some(saved)) => {
                if self.story.nodes[node].continue_to.is_some()
                    && saved.phase != FlowPhase::Presenting
                {
                    return Err(invalid());
                }
                if let Some(decision) = save
                    .temporal
                    .as_ref()
                    .and_then(|time| time.decision.as_ref())
                {
                    let phase = if saved.paused {
                        DecisionPhase::Paused
                    } else if saved.phase == FlowPhase::Interactive {
                        DecisionPhase::Active
                    } else {
                        DecisionPhase::AwaitingReady
                    };
                    if phase != decision.phase {
                        return Err(invalid());
                    }
                    if saved.phase == FlowPhase::Presenting
                        && self.story.nodes[node]
                            .decision
                            .as_ref()
                            .is_some_and(|rule| rule.limit_ms != decision.remaining_ms)
                    {
                        return Err(invalid());
                    }
                }
                let mut restored = saved.clone();
                if restored.phase == FlowPhase::Interactive {
                    restored.phase = FlowPhase::AwaitingInteraction;
                }
                Ok(Some(restored))
            }
            _ => Err(invalid()),
        }
    }
}
