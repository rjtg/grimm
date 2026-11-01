//! Deterministic cue player. The host supplies its renderer clock; this object
//! never sees story rules, variables, graph nodes or engine implementations.
use grimm_presentation::cues::*;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub const HISTORY_LIMIT: usize = 32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifecycle {
    Pending,
    Running { at_ms: u64 },
    Completed { at_ms: u64 },
    Cancelled,
}
#[derive(Debug, Clone)]
pub struct CueTrace {
    pub cue: Cue,
    pub lifecycle: Lifecycle,
}
#[derive(Debug, Clone)]
pub struct RevisionTrace {
    pub revision: u64,
    pub epoch: u64,
    pub clock: Clock,
    pub cues: Vec<CueTrace>,
}

#[derive(Debug, Clone, Default)]
pub struct CuePlayer {
    sheet: Option<CueSheet>,
    started: BTreeMap<String, u64>,
    completed: BTreeSet<String>,
    actions: Vec<String>,
    history: VecDeque<RevisionTrace>,
}

impl CuePlayer {
    /// Replace the whole pending plan. Retain execution identity, not old queued
    /// instructions. Same-ID running media continue instead of starting again.
    pub fn replace(&mut self, sheet: CueSheet) {
        let same_epoch = self
            .sheet
            .as_ref()
            .is_some_and(|old| old.clock.epoch == sheet.clock.epoch);
        let same_revision = same_epoch
            && self
                .sheet
                .as_ref()
                .is_some_and(|old| old.revision == sheet.revision);
        let ids: BTreeSet<_> = sheet.cues.iter().map(|cue| cue.cue_id.clone()).collect();
        if let Some(previous) = self.history.back_mut()
            && let Some(old) = &self.sheet
        {
            previous.clock = old.clock.clone();
        }
        for previous in &mut self.history {
            for row in &mut previous.cues {
                if (previous.epoch != sheet.clock.epoch || !ids.contains(&row.cue.cue_id))
                    && !matches!(
                        row.lifecycle,
                        Lifecycle::Completed { .. } | Lifecycle::Cancelled
                    )
                {
                    row.lifecycle = Lifecycle::Cancelled;
                }
            }
        }
        if !same_epoch {
            self.started.clear();
            self.completed.clear();
        }
        if !same_revision {
            self.actions.clear();
        }
        for cue in &sheet.cues {
            if self.completed.contains(&cue.cue_id)
                && let Instruction::Actions { available_actions } = &cue.instruction
            {
                self.actions = available_actions.clone();
            }
            if let Some(at) = cue.started_at_ms {
                self.started.entry(cue.cue_id.clone()).or_insert(at);
                if let Instruction::Actions { available_actions } = &cue.instruction {
                    self.actions = available_actions.clone();
                }
            }
        }
        let needed: BTreeSet<_> = ids
            .into_iter()
            .chain(sheet.cues.iter().filter_map(|cue| cue.after_cue.clone()))
            .collect();
        self.started.retain(|id, _| needed.contains(id));
        self.completed.retain(|id| needed.contains(id));
        if !same_revision {
            let rows = sheet
                .cues
                .iter()
                .map(|cue| CueTrace {
                    cue: cue.clone(),
                    lifecycle: if self.completed.contains(&cue.cue_id) {
                        Lifecycle::Completed {
                            at_ms: sheet.clock.elapsed_ms,
                        }
                    } else if let Some(at) = self.started.get(&cue.cue_id) {
                        Lifecycle::Running { at_ms: *at }
                    } else {
                        Lifecycle::Pending
                    },
                })
                .collect();
            self.history.push_back(RevisionTrace {
                revision: sheet.revision,
                epoch: sheet.clock.epoch,
                clock: sheet.clock.clone(),
                cues: rows,
            });
            while self.history.len() > HISTORY_LIMIT {
                self.history.pop_front();
            }
        }
        self.sheet = Some(sheet);
    }

    pub fn history(&self) -> &VecDeque<RevisionTrace> {
        &self.history
    }
    pub fn sheet(&self) -> Option<&CueSheet> {
        self.sheet.as_ref()
    }
    fn track(&mut self, id: &str, lifecycle: Lifecycle) {
        let Some(sheet) = &self.sheet else {
            return;
        };
        for revision in &mut self.history {
            if revision.epoch != sheet.clock.epoch {
                continue;
            }
            for row in &mut revision.cues {
                if row.cue.cue_id == id
                    && !matches!(
                        row.lifecycle,
                        Lifecycle::Cancelled | Lifecycle::Completed { .. }
                    )
                {
                    row.lifecycle = lifecycle;
                }
            }
        }
    }

    pub fn clock(&self) -> Option<&Clock> {
        self.sheet.as_ref().map(|sheet| &sheet.clock)
    }
    pub fn set_time(&mut self, at: u64) -> Result<(), &'static str> {
        let clock = &mut self.sheet.as_mut().ok_or("no Cue Sheet")?.clock;
        if at < clock.elapsed_ms || (!clock.running && at != 0) {
            return Err("renderer clock regression or not started");
        }
        clock.elapsed_ms = at;
        Ok(())
    }
    pub fn actions(&self) -> &[String] {
        &self.actions
    }
    pub fn revision(&self) -> u64 {
        self.sheet.as_ref().map_or(0, |sheet| sheet.revision)
    }
    pub fn pending(&self) -> bool {
        self.sheet.as_ref().is_some_and(|sheet| {
            sheet
                .cues
                .iter()
                .any(|cue| !self.completed.contains(&cue.cue_id))
        })
    }
    pub fn running_narratives(&self) -> Vec<String> {
        self.sheet
            .iter()
            .flat_map(|sheet| &sheet.cues)
            .filter(|cue| {
                matches!(cue.instruction, Instruction::Narrative { .. })
                    && self.started.contains_key(&cue.cue_id)
                    && !self.completed.contains(&cue.cue_id)
            })
            .map(|cue| cue.cue_id.clone())
            .collect()
    }

    pub fn due(&self) -> Vec<Cue> {
        let Some(sheet) = &self.sheet else {
            return vec![];
        };
        sheet
            .cues
            .iter()
            .filter(|cue| {
                !self.started.contains_key(&cue.cue_id)
                    && cue
                        .after_cue
                        .as_ref()
                        .is_none_or(|id| self.completed.contains(id))
                    && sheet.clock.elapsed_ms >= cue.at_ms
                    && (sheet.clock.running || cue.at_ms == 0)
            })
            .cloned()
            .collect()
    }

    /// Called after the host accepts the corresponding feedback, never a verdict
    /// about gameplay success. Control cues apply only renderer coordination.
    pub fn started(&mut self, id: &str) -> Result<CueFeedback, &'static str> {
        let cue = self
            .due()
            .into_iter()
            .find(|cue| cue.cue_id == id)
            .ok_or("cue is not due")?;
        let sheet = self.sheet.as_mut().ok_or("no sheet")?;
        let at = sheet.clock.elapsed_ms;
        match cue.instruction {
            Instruction::Actions { available_actions } => self.actions = available_actions,
            Instruction::Timer {
                start_ms,
                deadline_at_ms,
                unset_deadline,
                extend_ms,
                visible,
            } => {
                if let Some(ms) = start_ms {
                    sheet.clock.deadline_ms = Some(at.checked_add(ms).ok_or("timer overflow")?);
                    sheet.clock.running = true;
                }
                if let Some(at) = deadline_at_ms {
                    sheet.clock.deadline_ms = Some(at);
                }
                if unset_deadline {
                    sheet.clock.deadline_ms = None;
                    sheet.clock.visible = false;
                }
                if let Some(ms) = extend_ms {
                    sheet.clock.deadline_ms = Some(
                        sheet
                            .clock
                            .deadline_ms
                            .ok_or("no timer to extend")?
                            .checked_add(ms)
                            .ok_or("timer overflow")?,
                    );
                }
                if let Some(visible) = visible {
                    sheet.clock.visible = visible;
                }
            }
            Instruction::Narrative { .. } => {}
        }
        let revision = sheet.revision;
        self.started.insert(id.into(), at);
        self.track(id, Lifecycle::Running { at_ms: at });
        Ok(CueFeedback {
            revision,
            cue_id: id.into(),
            scene_time_ms: at,
            event: CueEvent::Started,
        })
    }

    pub fn completed(&mut self, id: &str) -> Result<CueFeedback, &'static str> {
        let sheet = self.sheet.as_ref().ok_or("no sheet")?;
        if !self.started.contains_key(id) || !sheet.cues.iter().any(|cue| cue.cue_id == id) {
            return Err("cue has not started or was replaced");
        }
        let revision = sheet.revision;
        let at = sheet.clock.elapsed_ms;
        self.completed.insert(id.into());
        self.track(id, Lifecycle::Completed { at_ms: at });
        Ok(CueFeedback {
            revision,
            cue_id: id.into(),
            scene_time_ms: at,
            event: CueEvent::Completed,
        })
    }

    /// Next semantic auto-completion or unblocked scheduled start. The host
    /// drives these milestones before sampling a later time or selecting input.
    pub fn controls_pending_completion(&self) -> Vec<String> {
        self.sheet
            .iter()
            .flat_map(|sheet| &sheet.cues)
            .filter(|cue| {
                !matches!(cue.instruction, Instruction::Narrative { .. })
                    && self.started.contains_key(&cue.cue_id)
                    && !self.completed.contains(&cue.cue_id)
            })
            .map(|cue| cue.cue_id.clone())
            .collect()
    }

    pub fn next_start_ms(&self) -> Option<u64> {
        let sheet = self.sheet.as_ref()?;
        sheet
            .cues
            .iter()
            .filter(|cue| {
                !self.started.contains_key(&cue.cue_id)
                    && cue
                        .after_cue
                        .as_ref()
                        .is_none_or(|id| self.completed.contains(id))
            })
            .map(|cue| cue.at_ms.max(sheet.clock.elapsed_ms))
            .min()
    }

    pub fn next_event_ms(&self) -> Option<u64> {
        let sheet = self.sheet.as_ref()?;
        let completions = sheet.cues.iter().filter_map(|cue| {
            if self.completed.contains(&cue.cue_id) {
                return None;
            }
            let Instruction::Narrative {
                duration_ms: Some(duration),
                ..
            } = cue.instruction
            else {
                return None;
            };
            self.started.get(&cue.cue_id)?.checked_add(duration)
        });
        completions
            .chain(
                sheet
                    .cues
                    .iter()
                    .filter(|cue| {
                        !self.started.contains_key(&cue.cue_id)
                            && cue
                                .after_cue
                                .as_ref()
                                .is_none_or(|id| self.completed.contains(id))
                    })
                    .map(|cue| cue.at_ms),
            )
            .map(|at| at.max(sheet.clock.elapsed_ms))
            .min()
    }
    pub fn completions_due(&self) -> Vec<String> {
        let Some(sheet) = &self.sheet else {
            return vec![];
        };
        sheet
            .cues
            .iter()
            .filter(|cue| {
                !self.completed.contains(&cue.cue_id)
                    && match cue.instruction {
                        Instruction::Narrative {
                            duration_ms: Some(duration),
                            ..
                        } => self
                            .started
                            .get(&cue.cue_id)
                            .and_then(|at| at.checked_add(duration))
                            .is_some_and(|end| end <= sheet.clock.elapsed_ms),
                        _ => false,
                    }
            })
            .map(|cue| cue.cue_id.clone())
            .collect()
    }

    pub fn select(&self, index: usize) -> Option<ActionInput> {
        Some(ActionInput {
            revision: self.revision(),
            action_id: self.actions.get(index)?.clone(),
            scene_time_ms: self.clock()?.elapsed_ms,
        })
    }
}
