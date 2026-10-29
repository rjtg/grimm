//! Opt-in authoring diagnostics. These reports may contain private graph context;
//! they are never part of a player-visible semantic projection.
use crate::{Engine, ValidationError, eligible};
use grimm_semantic::story::Variables;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
pub struct PlayabilityLimits {
    pub max_states: usize,
    pub max_transitions: usize,
}

impl Default for PlayabilityLimits {
    fn default() -> Self {
        Self {
            max_states: 1024,
            max_transitions: 8192,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayabilityStep {
    pub node_id: String,
    pub choice_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deadlock {
    pub node_id: String,
    /// A shortest discovered route from the session being checked.
    pub path: Vec<PlayabilityStep>,
    pub reason: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PlayabilityReport {
    pub states_checked: usize,
    /// Counts all inspected choices, including condition-ineligible ones.
    pub transitions_checked: usize,
    /// Some states/choices were not inspected. Absence of findings is inconclusive.
    pub truncated: bool,
    pub deadlocks: Vec<Deadlock>,
    pub endings: BTreeSet<String>,
}

#[derive(Clone)]
struct State {
    node: usize,
    variables: Variables,
    world_minutes: Option<u64>,
    path: Vec<PlayabilityStep>,
}

impl Engine {
    /// Validate the active state without mutating it, for explicit host diagnostics.
    pub fn validate_current_state(&self) -> Result<(), ValidationError> {
        if self.story.nodes[self.node].ending.is_none() && self.revision == u64::MAX {
            return Err(ValidationError(format!(
                "node '{}': session revision exhausted",
                self.story.nodes[self.node].id
            )));
        }
        self.ensure_playable(self.node, &self.variables, self.world_minutes)
    }

    /// Breadth-first exploration using the runtime's exact single-step preview.
    /// Budgets bound discovered states and inspected choices. Exact node/variable
    /// equality deduplicates cycles; revision is not narrative progress. This is
    /// authoring feedback, not a proof that every playthrough eventually finishes.
    pub fn check_playability(&self, limits: PlayabilityLimits) -> PlayabilityReport {
        let mut report = PlayabilityReport::default();
        if limits.max_states == 0 {
            report.truncated = true;
            return report;
        }
        let mut states = vec![State {
            node: self.node,
            variables: self.variables.clone(),
            world_minutes: self.world_minutes,
            path: vec![],
        }];
        let mut cursor = 0;
        while cursor < states.len() {
            let state = states[cursor].clone();
            cursor += 1;
            report.states_checked += 1;
            let node = &self.story.nodes[state.node];
            if let Some(ending) = &node.ending {
                report.endings.insert(ending.id.clone());
                continue;
            }
            if cursor == 1 && self.revision == u64::MAX {
                report.deadlocks.push(Deadlock {
                    node_id: node.id.clone(),
                    path: state.path,
                    reason: self.validate_current_state().unwrap_err().to_string(),
                });
                continue;
            }
            let mut executable = false;
            let mut failures = Vec::new();
            let timeout = crate::temporal::timeout_choice(node);
            for choice in node.choices.iter().chain(timeout.iter()) {
                if report.transitions_checked == limits.max_transitions {
                    report.truncated = true;
                    // A partially inspected node must not be called a deadlock.
                    return report;
                }
                report.transitions_checked += 1;
                if !eligible(choice, &state.variables, state.world_minutes) {
                    continue;
                }
                match self.preview(choice, &state.variables, state.world_minutes) {
                    Err(error) => failures.push(format!("choice '{}': {error}", choice.id)),
                    Ok((next, variables, world_minutes)) => {
                        executable = true;
                        if states.iter().any(|known| {
                            known.node == next
                                && known.variables == variables
                                && known.world_minutes == world_minutes
                        }) {
                            continue;
                        }
                        if states.len() == limits.max_states {
                            report.truncated = true;
                            continue;
                        }
                        let mut path = state.path.clone();
                        path.push(PlayabilityStep {
                            node_id: node.id.clone(),
                            choice_id: choice.id.clone(),
                        });
                        states.push(State {
                            node: next,
                            variables,
                            world_minutes,
                            path,
                        });
                    }
                }
            }
            if !executable {
                let reason = if failures.is_empty() {
                    "no eligible choices in this state".into()
                } else {
                    format!("no executable choices ({})", failures.join("; "))
                };
                report.deadlocks.push(Deadlock {
                    node_id: node.id.clone(),
                    path: state.path,
                    reason,
                });
            }
        }
        report
    }
}
