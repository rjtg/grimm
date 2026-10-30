//! Deterministic cue player. The host supplies its renderer clock; this object
//! never sees story rules, variables, graph nodes or engine implementations.
use grimm_presentation::cues::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Default)]
pub struct CuePlayer {
    sheet: Option<CueSheet>,
    started: BTreeMap<String, u64>,
    completed: BTreeSet<String>,
    actions: Vec<String>,
}

impl CuePlayer {
    /// Replace the whole pending plan. Retain execution identity, not old queued
    /// instructions. Same-ID running media continue instead of starting again.
    pub fn replace(&mut self, sheet: CueSheet) {
        let same_epoch = self
            .sheet
            .as_ref()
            .is_some_and(|old| old.clock.epoch == sheet.clock.epoch);
        let same_revision = self
            .sheet
            .as_ref()
            .is_some_and(|old| old.revision == sheet.revision);
        if !same_epoch {
            self.started.clear();
            self.completed.clear();
        }
        if !same_epoch || !same_revision {
            self.actions.clear();
        }
        for cue in &sheet.cues {
            if self.completed.contains(&cue.cue_id)
                && let Instruction::Actions { available_actions } = &cue.instruction
            {
                self.actions = available_actions.clone();
            }
        }
        self.sheet = Some(sheet);
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
            .enumerate()
            .filter(|(index, cue)| {
                !self.started.contains_key(&cue.cue_id)
                    && cue
                        .after_cue
                        .as_ref()
                        .is_none_or(|id| self.completed.contains(id))
                    && cue.at_ms.map_or_else(
                        || {
                            sheet.cues[..*index]
                                .iter()
                                .filter(|prior| prior.at_ms.is_none())
                                .all(|prior| self.completed.contains(&prior.cue_id))
                        },
                        |at| sheet.clock.running && sheet.clock.elapsed_ms >= at,
                    )
            })
            .map(|(_, cue)| cue.clone())
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
                extend_ms,
                visible,
            } => {
                if let Some(ms) = start_ms {
                    sheet.clock.deadline_ms = Some(at.checked_add(ms).ok_or("timer overflow")?);
                    sheet.clock.running = true;
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
        self.started.insert(id.into(), sheet.revision);
        Ok(CueFeedback {
            revision: sheet.revision,
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
        self.completed.insert(id.into());
        Ok(CueFeedback {
            revision: sheet.revision,
            cue_id: id.into(),
            scene_time_ms: sheet.clock.elapsed_ms,
            event: CueEvent::Completed,
        })
    }

    pub fn select(&self, index: usize) -> Option<ActionInput> {
        Some(ActionInput {
            revision: self.revision(),
            action_id: self.actions.get(index)?.clone(),
            scene_time_ms: self.clock()?.elapsed_ms,
        })
    }
}
