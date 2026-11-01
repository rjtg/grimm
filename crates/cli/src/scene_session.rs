//! Synchronous host coordination shared by piped playback and the TUI.
//! The renderer supplies time; this host forwards chronological semantic inputs.
mod backend;
use backend::Backend;
use grimm_adapters::cues as adapter;
use grimm_cli_renderer::cues::CuePlayer;
use grimm_engine::scene::lifecycle::LifecycleEngine;
use grimm_engine::scene::{SceneEngine, SceneError, SceneResult};
use grimm_presentation::cues::{Cue, Instruction};
use grimm_presentation::lifecycle::{SceneEvent, ScenePhase, ScenePresentation};
use grimm_semantic::scene::{SceneAction, SceneSave};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct StartedNarrative {
    pub epoch: u64,
    pub revision: u64,
    pub cue: Cue,
}

pub struct SceneSession {
    engine: Backend,
    player: CuePlayer,
    automatic: bool,
    started_text: Vec<StartedNarrative>,
    presentation_ms: u64,
    media_started: BTreeMap<String, u64>,
}
impl SceneSession {
    pub fn new(engine: SceneEngine, automatic: bool) -> SceneResult<Self> {
        Self::from_backend(Backend::Legacy(Box::new(engine)), automatic)
    }
    pub fn new_lifecycle(engine: LifecycleEngine, automatic: bool) -> SceneResult<Self> {
        Self::from_backend(Backend::Lifecycle(Box::new(engine)), automatic)
    }
    fn from_backend(engine: Backend, automatic: bool) -> SceneResult<Self> {
        let mut session = Self {
            engine,
            player: CuePlayer::default(),
            automatic,
            started_text: vec![],
            presentation_ms: 0,
            media_started: BTreeMap::new(),
        };
        session.replace();
        session.seed_running_text();
        session.pump_now()?;
        Ok(session)
    }
    pub fn engine(&self) -> &SceneEngine {
        self.engine.core()
    }
    pub fn scene(&self) -> Option<ScenePresentation> {
        self.engine.scene()
    }
    pub fn is_complete(&self) -> bool {
        self.engine.is_complete()
    }
    pub fn provider_revision(&self) -> Option<u64> {
        self.engine.provider_revision()
    }
    fn ending(&self) -> bool {
        self.scene()
            .is_some_and(|scene| scene.phase == ScenePhase::Ending)
    }
    pub fn preparation_scene(&mut self) -> SceneResult<String> {
        if self
            .scene()
            .is_some_and(|scene| scene.phase == ScenePhase::Stable && scene.next_scene_id.is_some())
        {
            self.engine.event(SceneEvent::Prepare)?;
            self.replace();
        }
        let scene = self
            .scene()
            .ok_or_else(|| SceneError("not a lifecycle session".into()))?;
        if scene.phase != ScenePhase::Prepare {
            return Err(SceneError("scene is not preparing".into()));
        }
        Ok(scene.scene_id)
    }
    /// Call only after the concrete renderer successfully resolves its resources.
    pub fn begin_prepared_scene(&mut self) -> SceneResult<()> {
        self.engine.event(SceneEvent::Ready)?;
        self.engine.event(SceneEvent::Begin)?;
        self.presentation_ms = 0;
        self.media_started.clear();
        self.replace();
        self.pump_now()?;
        Ok(())
    }
    pub fn preparation_failed(&mut self) -> SceneResult<()> {
        self.engine.event(SceneEvent::PreparationFailed)?;
        self.replace();
        Ok(())
    }
    pub fn save_json(&self) -> SceneResult<String> {
        self.engine.save_json()
    }
    pub fn restore_json(&mut self, json: &str) -> SceneResult<()> {
        self.engine.restore_json(json)?;
        self.reset_presentation()
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
        if self.ending() && self.player.clock().is_some_and(|clock| clock.running) {
            self.presentation_ms = 0;
            self.media_started.clear();
        }
        self.player.replace(adapter::present(&self.engine.output()));
    }
    fn accept_completion(&mut self, id: &str) -> SceneResult<bool> {
        let mut candidate = self.player.clone();
        let input = candidate.completed(id).map_err(|e| SceneError(e.into()))?;
        let epoch = self.engine.output().clock.epoch;
        let closing_terminal = self
            .scene()
            .is_some_and(|scene| scene.phase == ScenePhase::Active)
            && self.engine.core().is_terminal();
        let sheet = self.engine.cue_input(&adapter::feedback(input))?;
        if sheet.revision != self.player.revision() {
            if closing_terminal {
                self.player = candidate;
            }
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
            if self.ending() && self.engine.output().cues.is_empty() {
                self.engine.event(SceneEvent::End)?;
                self.replace();
                continue;
            }
            if let Some(id) = self.player.controls_pending_completion().first().cloned() {
                self.accept_completion(&id)?;
                if epoch != self.engine.output().clock.epoch {
                    return Ok(true);
                }
                continue;
            }
            if self.automatic
                && let Some(id) = self.completions_due().first().cloned()
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
                if self.ending() {
                    self.media_started
                        .entry(cue.cue_id.clone())
                        .or_insert(self.presentation_ms);
                }
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
        if self
            .scene()
            .is_some_and(|scene| scene.phase != ScenePhase::Active)
        {
            if target != self.player.clock().expect("sheet").elapsed_ms {
                return Err(SceneError("scene clock is frozen".into()));
            }
            self.pump_now()?;
            return Ok(());
        }
        if target < self.player.clock().expect("sheet").elapsed_ms {
            return Err(SceneError("renderer clock regression".into()));
        }
        for _ in 0..4096 {
            if self.pump_now()? {
                self.pump_now()?;
                return Ok(());
            }
            if self
                .scene()
                .is_some_and(|scene| scene.phase != ScenePhase::Active)
            {
                return Ok(());
            }
            let clock = self.player.clock().expect("sheet");
            let now = clock.elapsed_ms;
            let presentation = if self.automatic {
                self.player.next_event_ms()
            } else {
                self.player.next_start_ms()
            };
            let deadline = (!self.engine.core().is_terminal())
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
            if sheet.clock.epoch != origin_epoch
                || self
                    .scene()
                    .is_some_and(|scene| scene.phase != ScenePhase::Active)
            {
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
        self.reset_presentation()
    }
    fn reset_presentation(&mut self) -> SceneResult<()> {
        self.presentation_ms = 0;
        self.media_started.clear();
        self.player = CuePlayer::default();
        self.started_text.clear();
        self.replace();
        self.seed_running_text();
        self.pump_now()?;
        Ok(())
    }
    fn completions_due(&self) -> Vec<String> {
        if !self.ending() {
            return self.player.completions_due();
        }
        self.player
            .sheet()
            .into_iter()
            .flat_map(|sheet| &sheet.cues)
            .filter_map(|cue| {
                let Instruction::Narrative {
                    duration_ms: Some(duration),
                    ..
                } = cue.instruction
                else {
                    return None;
                };
                let end = self.media_started.get(&cue.cue_id)?.checked_add(duration)?;
                (self.presentation_ms >= end
                    && self.player.running_narratives().contains(&cue.cue_id))
                .then(|| cue.cue_id.clone())
            })
            .collect()
    }
    /// Renderer-only elapsed media time while Ending freezes gameplay time.
    /// Pause callers simply do not submit elapsed presentation time.
    pub fn presentation_pending(&self) -> bool {
        self.ending()
            && self.automatic
            && self.player.sheet().is_some_and(|sheet| {
                sheet.cues.iter().any(|cue| {
                    matches!(
                        cue.instruction,
                        Instruction::Narrative {
                            duration_ms: Some(_),
                            ..
                        }
                    ) && self.player.running_narratives().contains(&cue.cue_id)
                })
            })
    }
    pub fn advance_presentation_by(&mut self, elapsed: u64) -> SceneResult<()> {
        if !self.ending() {
            return Ok(());
        }
        let target = self
            .presentation_ms
            .checked_add(elapsed)
            .ok_or_else(|| SceneError("presentation time overflow".into()))?;
        for _ in 0..4096 {
            self.pump_now()?;
            if !self.ending() {
                return Ok(());
            }
            let next = self
                .player
                .sheet()
                .into_iter()
                .flat_map(|sheet| &sheet.cues)
                .filter_map(|cue| {
                    let Instruction::Narrative {
                        duration_ms: Some(duration),
                        ..
                    } = cue.instruction
                    else {
                        return None;
                    };
                    if !self.automatic || !self.player.running_narratives().contains(&cue.cue_id) {
                        return None;
                    }
                    self.media_started.get(&cue.cue_id)?.checked_add(duration)
                })
                .min()
                .unwrap_or(target)
                .min(target);
            self.presentation_ms = next.max(self.presentation_ms);
            self.pump_now()?;
            if self.presentation_ms >= target || !self.ending() {
                return Ok(());
            }
        }
        Err(SceneError("ending presentation budget exhausted".into()))
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
