use crate::{ActionError, Engine, ValidationError};
use grimm_semantic::{
    SemanticOutput,
    save::{FLOW_SAVE_SCHEMA_VERSION, SaveGame, TEMPORAL_SAVE_SCHEMA_VERSION},
    story::{Choice, Node},
    time::{DecisionPhase, DecisionView, TimeEvent, TimeInput},
};
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct ActiveDecision {
    pub view: DecisionView,
    pub(crate) last_elapsed_ms: u64,
}

impl ActiveDecision {
    pub(crate) fn for_node(node: &Node) -> Option<Self> {
        node.decision.as_ref().map(|decision| Self {
            view: DecisionView {
                remaining_ms: decision.limit_ms,
                phase: DecisionPhase::AwaitingReady,
            },
            last_elapsed_ms: 0,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeError {
    Flow(crate::FlowError),
    StaleRevision,
    NoTemporalRules,
    NoDecision,
    InvalidPhase,
    NonIncreasingElapsed,
    InvalidDuration,
    Transition(ActionError),
}
impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flow(error) => error.fmt(f),
            Self::StaleRevision => f.write_str("time input refers to a stale interaction revision"),
            Self::NoTemporalRules => f.write_str("story has no temporal rules"),
            Self::NoDecision => f.write_str("there is no pending decision deadline"),
            Self::InvalidPhase => {
                f.write_str("time input is invalid in the current decision phase")
            }
            Self::NonIncreasingElapsed => {
                f.write_str("elapsed observation must strictly increase since readiness")
            }
            Self::InvalidDuration => {
                f.write_str("world-time advance must be positive and occur at an untimed node")
            }
            Self::Transition(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for TimeError {}
impl From<ActionError> for TimeError {
    fn from(error: ActionError) -> Self {
        Self::Transition(error)
    }
}

pub(crate) fn timeout_choice(node: &Node) -> Option<Choice> {
    node.decision.as_ref().map(|decision| Choice {
        id: "<timeout>".into(),
        label: "Authored timeout".into(),
        target: decision.timeout.target.clone(),
        condition: None,
        effects: decision.timeout.effects.clone(),
        advance_minutes: 0,
    })
}

impl Engine {
    /// The host supplies observations, never an instruction to invent an outcome.
    /// At the deadline, this commits the authored timeout and consumes the revision.
    pub fn time_input(&mut self, input: &TimeInput) -> Result<SemanticOutput, TimeError> {
        if input.revision != self.revision {
            return Err(TimeError::StaleRevision);
        }
        if self.flow.is_some() {
            use grimm_semantic::flow::{FlowEvent, FlowInput};
            let event = match input.event {
                TimeEvent::Ready => Some(FlowEvent::InteractionReady),
                TimeEvent::Pause => Some(FlowEvent::Pause),
                TimeEvent::Resume => Some(FlowEvent::Resume),
                TimeEvent::AdvanceWorld { .. } => return Err(TimeError::InvalidPhase),
                TimeEvent::Elapsed { .. } => None,
            };
            if let Some(event) = event {
                return self
                    .flow_input(&FlowInput {
                        revision: input.revision,
                        event,
                    })
                    .map_err(TimeError::Flow);
            }
        }
        let world = self.world_minutes.ok_or(TimeError::NoTemporalRules)?;
        if self.story.nodes[self.node].ending.is_some() {
            return Err(TimeError::InvalidPhase);
        }
        if let TimeEvent::AdvanceWorld { minutes } = input.event {
            if minutes == 0 || self.decision.is_some() {
                return Err(TimeError::InvalidDuration);
            }
            let world = world
                .checked_add(minutes)
                .ok_or(ActionError::WorldTimeOverflow)?;
            self.ensure_playable(self.node, &self.variables, Some(world))
                .map_err(|_| ActionError::NoOffers {
                    node: self.story.nodes[self.node].id.clone(),
                })?;
            return Ok(self.commit(self.node, self.variables.clone(), Some(world))?);
        }
        let mut decision = self.decision.clone().ok_or(TimeError::NoDecision)?;
        match input.event {
            TimeEvent::Ready => {
                if decision.view.phase != DecisionPhase::AwaitingReady {
                    return Err(TimeError::InvalidPhase);
                }
                decision.view.phase = DecisionPhase::Active;
            }
            TimeEvent::Elapsed { total_ms } => {
                if decision.view.phase != DecisionPhase::Active {
                    return Err(TimeError::InvalidPhase);
                }
                if total_ms <= decision.last_elapsed_ms {
                    return Err(TimeError::NonIncreasingElapsed);
                }
                let delta = total_ms - decision.last_elapsed_ms;
                if delta >= decision.view.remaining_ms {
                    let choice = timeout_choice(&self.story.nodes[self.node])
                        .expect("active authored deadline");
                    let (target, variables, world) =
                        self.preview(&choice, &self.variables, self.world_minutes)?;
                    return Ok(self.commit(target, variables, world)?);
                }
                decision.view.remaining_ms -= delta;
                decision.last_elapsed_ms = total_ms;
            }
            TimeEvent::Pause => {
                if decision.view.phase == DecisionPhase::Paused {
                    return Err(TimeError::InvalidPhase);
                }
                let revision = self
                    .revision
                    .checked_add(1)
                    .filter(|revision| *revision < u64::MAX)
                    .ok_or(ActionError::RevisionOverflow)?;
                decision.view.phase = DecisionPhase::Paused;
                decision.last_elapsed_ms = 0;
                self.revision = revision;
            }
            TimeEvent::Resume => {
                if decision.view.phase != DecisionPhase::Paused {
                    return Err(TimeError::InvalidPhase);
                }
                let revision = self
                    .revision
                    .checked_add(1)
                    .filter(|revision| *revision < u64::MAX)
                    .ok_or(ActionError::RevisionOverflow)?;
                decision.view.phase = DecisionPhase::AwaitingReady;
                decision.last_elapsed_ms = 0;
                self.revision = revision;
            }
            TimeEvent::AdvanceWorld { .. } => unreachable!(),
        }
        self.decision = Some(decision);
        Ok(self.output())
    }

    pub(crate) fn restore_temporal(
        &self,
        save: &SaveGame,
        node: usize,
    ) -> Result<(Option<u64>, Option<ActiveDecision>), ValidationError> {
        let Some(config) = &self.story.time else {
            if save.temporal.is_some() {
                return Err(ValidationError(
                    "untimed story cannot restore temporal state".into(),
                ));
            }
            return Ok((None, None));
        };
        if save.schema_version != TEMPORAL_SAVE_SCHEMA_VERSION
            && save.schema_version != FLOW_SAVE_SCHEMA_VERSION
        {
            return Err(ValidationError(
                "temporal stories require save schema 2 (or 3 for flow stories)".into(),
            ));
        }
        let temporal = save
            .temporal
            .as_ref()
            .ok_or_else(|| ValidationError("save is missing temporal state".into()))?;
        if temporal.world_minutes < config.start_minutes {
            return Err(ValidationError(
                "save world time precedes the authored origin".into(),
            ));
        }
        let decision = match (&self.story.nodes[node].decision, &temporal.decision) {
            (None, None) => None,
            (Some(rule), Some(view))
                if view.remaining_ms > 0 && view.remaining_ms <= rule.limit_ms =>
            {
                let mut view = view.clone();
                if view.phase == DecisionPhase::Active {
                    view.phase = DecisionPhase::AwaitingReady;
                }
                Some(ActiveDecision {
                    view,
                    last_elapsed_ms: 0,
                })
            }
            _ => {
                return Err(ValidationError(
                    "save deadline does not match the node or remaining duration is invalid".into(),
                ));
            }
        };
        Ok((Some(temporal.world_minutes), decision))
    }
}
