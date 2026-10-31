//! Synchronous host coordination shared by piped playback and the TUI.
//! The renderer supplies time; this host forwards chronological semantic inputs.
use grimm_adapters::cues as adapter;
use grimm_cli_renderer::cues::CuePlayer;
use grimm_engine::scene::{SceneEngine, SceneError, SceneResult};
use grimm_presentation::cues::{Cue, Instruction};
use grimm_semantic::scene::{SceneAction, SceneSave};

#[derive(Debug, Clone)]
pub struct StartedNarrative {
    pub epoch: u64,
    pub revision: u64,
    pub cue: Cue,
}

pub struct SceneSession {
    engine: SceneEngine,
    player: CuePlayer,
    automatic: bool,
    started_text: Vec<StartedNarrative>,
}
impl SceneSession {
    pub fn new(engine: SceneEngine, automatic: bool) -> SceneResult<Self> {
        let mut session = Self {
            engine,
            player: CuePlayer::default(),
            automatic,
            started_text: vec![],
        };
        session.replace();
        session.seed_running_text();
        session.pump_now()?;
        Ok(session)
    }
    pub fn engine(&self) -> &SceneEngine {
        &self.engine
    }
    pub fn player(&self) -> &CuePlayer {
        &self.player
    }
    pub fn automatic(&self) -> bool {
        self.automatic
    }
    pub fn take_started_text(&mut self) -> Vec<StartedNarrative> {
        std::mem::take(&mut self.started_text)
    }
    fn replace(&mut self) {
        self.player.replace(adapter::present(&self.engine.output()));
    }
    fn accept_completion(&mut self, id: &str) -> SceneResult<bool> {
        let mut candidate = self.player.clone();
        let input = candidate.completed(id).map_err(|e| SceneError(e.into()))?;
        let epoch = self.engine.output().clock.epoch;
        let sheet = self.engine.cue_input(&adapter::feedback(input))?;
        if sheet.revision != self.player.revision() {
            self.replace();
        } else {
            self.player = candidate;
        }
        Ok(epoch != sheet.clock.epoch)
    }
    /// Complete due text, then start newly eligible cues, until this timestamp
    /// is stable. Expiry is processed by advance_to before this function.
    fn pump_now(&mut self) -> SceneResult<bool> {
        let epoch = self.engine.output().clock.epoch;
        for _ in 0..2048 {
            if let Some(id) = self.player.controls_pending_completion().first().cloned() {
                self.accept_completion(&id)?;
                if epoch != self.engine.output().clock.epoch {
                    return Ok(true);
                }
                continue;
            }
            if self.automatic
                && let Some(id) = self.player.completions_due().first().cloned()
            {
                self.accept_completion(&id)?;
                if epoch != self.engine.output().clock.epoch {
                    return Ok(true);
                }
                continue;
            }
            let Some(cue) = self.player.due().into_iter().next() else {
                return Ok(false);
            };
            let mut candidate = self.player.clone();
            let input = candidate
                .started(&cue.cue_id)
                .map_err(|e| SceneError(e.into()))?;
            let sheet = self.engine.cue_input(&adapter::feedback(input))?;
            if sheet.revision != self.player.revision() {
                self.replace();
                if epoch != sheet.clock.epoch {
                    return Ok(true);
                }
                continue;
            }
            self.player = candidate;
            if matches!(cue.instruction, Instruction::Narrative { .. }) {
                let mut cue = cue;
                cue.started_at_ms = Some(sheet.clock.elapsed_ms);
                self.started_text.push(StartedNarrative {
                    epoch: sheet.clock.epoch,
                    revision: sheet.revision,
                    cue,
                });
            }
        }
        Err(SceneError("cue scheduling budget exhausted".into()))
    }
    /// Deadline first at equal timestamps, then auto completions, then starts.
    /// No unobserved time carries into a newly entered epoch.
    pub fn advance_to(&mut self, target: u64) -> SceneResult<()> {
        let origin_epoch = self.engine.output().clock.epoch;
        if target < self.player.clock().expect("sheet").elapsed_ms {
            return Err(SceneError("renderer clock regression".into()));
        }
        for _ in 0..4096 {
            if self.pump_now()? {
                self.pump_now()?;
                return Ok(());
            }
            let clock = self.player.clock().expect("sheet");
            let now = clock.elapsed_ms;
            let presentation = if self.automatic {
                self.player.next_event_ms()
            } else {
                self.player.next_start_ms()
            };
            let deadline = (!self.engine.is_terminal())
                .then_some(clock.deadline_ms)
                .flatten();
            let milestone = presentation
                .into_iter()
                .chain(deadline)
                .min()
                .unwrap_or(target)
                .min(target)
                .max(now);
            let sheet = self.engine.observe(self.player.revision(), milestone)?;
            if sheet.revision != self.player.revision() {
                self.replace();
            } else {
                self.player
                    .set_time(milestone)
                    .map_err(|e| SceneError(e.into()))?;
            }
            if sheet.clock.epoch != origin_epoch {
                self.pump_now()?;
                return Ok(());
            }
            if milestone == target {
                self.pump_now()?;
                return Ok(());
            }
            if milestone == now {
                return Err(SceneError("cue scheduler failed to advance".into()));
            }
        }
        Err(SceneError("cue scheduling budget exhausted".into()))
    }
    /// Pass the action ID captured from the displayed offer, not a number
    /// remapped after a scheduled update. This also protects same-revision lists.
    pub fn choose(&mut self, id: &str, revision: u64, at: u64) -> SceneResult<()> {
        self.advance_to(at)?;
        if revision != self.player.revision() {
            return Err(SceneError("stale input ignored".into()));
        }
        self.engine.apply(&SceneAction {
            revision,
            action_id: id.into(),
            scene_time_ms: at,
        })?;
        self.replace();
        self.pump_now()?;
        Ok(())
    }
    pub fn complete_text(&mut self, id: &str, revision: u64, at: u64) -> SceneResult<()> {
        self.advance_to(at)?;
        if revision != self.player.revision() {
            return Err(SceneError("stale input ignored".into()));
        }
        self.accept_completion(id)?;
        self.pump_now()?;
        Ok(())
    }
    pub fn restore(&mut self, save: SceneSave) -> SceneResult<()> {
        self.engine.restore(save)?;
        self.player = CuePlayer::default();
        self.started_text.clear();
        self.replace();
        self.seed_running_text();
        self.pump_now()?;
        Ok(())
    }
    // Reconstruct presentation only: never emit another semantic start event.
    fn seed_running_text(&mut self) {
        let sheet = adapter::present(&self.engine.output());
        self.started_text.extend(
            sheet
                .cues
                .into_iter()
                .filter(|cue| {
                    cue.started_at_ms.is_some()
                        && matches!(cue.instruction, Instruction::Narrative { .. })
                })
                .map(|cue| StartedNarrative {
                    epoch: sheet.clock.epoch,
                    revision: sheet.revision,
                    cue,
                }),
        );
    }
}
