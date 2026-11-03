//! Explicit M4.6 scene ownership around the existing deterministic rule engine.
//! Story rules are not copied: transitions/effects still resolve in SceneEngine.
use super::{
    SceneEngine, SceneError, SceneResult, Status,
    state::{InMemoryWorldState, ProvidedSceneEngine, WorldStateProvider},
};
use grimm_semantic::{lifecycle::*, scene::*, world::WorldSnapshot};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
struct Closing {
    scene_id: String,
    clock: SceneClock,
    outgoing: BTreeSet<String>,
    has_next: bool,
}
#[derive(Debug)]
pub struct LifecycleEngine<P: WorldStateProvider = InMemoryWorldState> {
    inner: ProvidedSceneEngine<P>,
    configuration: SceneConfiguration,
    phase: ScenePhase,
    ready: bool,
    closing: Option<Closing>,
    revision: u64,
    issued: BTreeMap<u64, (u64, u64, BTreeSet<String>)>,
    history: Vec<LifecycleInput>,
}
impl LifecycleEngine<InMemoryWorldState> {
    pub fn new(story: SceneStory, configuration: SceneConfiguration) -> SceneResult<Self> {
        Self::with_provider(story, configuration, |initial| {
            InMemoryWorldState::new(initial.scope, initial.globals, initial.locals)
        })
    }
}
impl<P: WorldStateProvider> LifecycleEngine<P> {
    pub fn with_provider(
        story: SceneStory,
        configuration: SceneConfiguration,
        make: impl FnOnce(WorldSnapshot) -> SceneResult<P>,
    ) -> SceneResult<Self> {
        let inner = ProvidedSceneEngine::new(story, make)?;
        let compiled = &inner.engine().story;
        if configuration.schema_version != 1
            || configuration.version == 0
            || configuration.story_id != compiled.id
            || configuration.story_version != compiled.version
            || super::validation::normalize(&configuration.id)? != configuration.id
        {
            return Err(SceneError(
                "scene configuration identity/version mismatch".into(),
            ));
        }
        if configuration.scenes.len() != compiled.contexts.len() {
            return Err(SceneError(
                "scene configuration must map every context exactly once".into(),
            ));
        }
        for context in &compiled.contexts {
            let scene = configuration.scenes.get(&context.id).ok_or_else(|| {
                SceneError(format!(
                    "missing presentation scene for context '{}'",
                    context.id
                ))
            })?;
            if super::validation::normalize(scene)? != *scene {
                return Err(SceneError(format!(
                    "invalid explicit scene identifier '{scene}'"
                )));
            }
        }
        let mut result = Self {
            inner,
            configuration,
            phase: ScenePhase::Prepare,
            ready: false,
            closing: None,
            revision: 0,
            issued: BTreeMap::new(),
            history: vec![],
        };
        result.register();
        Ok(result)
    }
    pub fn engine(&self) -> &SceneEngine {
        self.inner.engine()
    }
    pub fn world_snapshot(&self) -> WorldSnapshot {
        self.inner.world_snapshot()
    }
    pub fn phase(&self) -> ScenePhase {
        self.phase
    }
    pub fn is_complete(&self) -> bool {
        self.phase == ScenePhase::Stable && self.closing.as_ref().is_some_and(|c| !c.has_next)
    }
    fn scene_id(&self) -> String {
        self.configuration.scenes[&self.inner.engine().ctx().id].clone()
    }
    fn next_revision(&self) -> SceneResult<u64> {
        self.revision
            .checked_add(1)
            .ok_or_else(|| SceneError("lifecycle revision exhausted".into()))
    }
    pub fn output(&self) -> SceneCueSheet {
        let mut plan = self.inner.engine().output();
        plan.revision = self.revision;
        let mut scene_id = self.scene_id();
        let mut next = None;
        match self.phase {
            ScenePhase::Active => {}
            ScenePhase::Prepare => {
                plan.cues.clear();
                plan.clock.running = false;
                plan.clock.deadline_ms = None;
                plan.clock.visible = false;
            }
            ScenePhase::Ending | ScenePhase::Stable => {
                let closing = self.closing.as_ref().expect("closing scope");
                scene_id = closing.scene_id.clone();
                plan.clock = closing.clock.clone();
                next = closing.has_next.then(|| self.scene_id());
                if self.phase == ScenePhase::Stable {
                    plan.cues.clear();
                } else {
                    plan.cues
                        .retain(|cue| closing.outgoing.contains(&cue.cue_id));
                    for cue in &mut plan.cues {
                        cue.at_ms = plan.clock.elapsed_ms;
                        if cue.started_at_ms.is_some() {
                            cue.started_at_ms = Some(plan.clock.elapsed_ms);
                        }
                    }
                }
            }
        }
        SceneCueSheet {
            protocol_version: 3,
            scene: ScenePresentation {
                phase: self.phase,
                scene_id,
                epoch: plan.clock.epoch,
                next_scene_id: next,
                ready: self.ready,
            },
            plan,
        }
    }
    fn register(&mut self) {
        let sheet = self.output();
        let entry = self.issued.entry(self.revision).or_insert_with(|| {
            (
                sheet.scene.epoch,
                self.inner.engine().revision,
                BTreeSet::new(),
            )
        });
        // Issued IDs describe correlation history, not just the current pending
        // subset. Keep completed controls identifiable for duplicate ACKs.
        entry
            .2
            .extend(sheet.plan.cues.into_iter().map(|cue| cue.cue_id));
    }
    fn begin_closing(&mut self, old_scene: String, mut old_clock: SceneClock, has_next: bool) {
        old_clock.running = false;
        old_clock.deadline_ms = None;
        old_clock.visible = false;
        let suffix = format!("__run_{}", self.inner.engine().records.len());
        let outgoing = if has_next {
            self.inner
                .engine()
                .output()
                .cues
                .into_iter()
                .filter(|cue| {
                    cue.cue_id.ends_with(&suffix)
                        && matches!(cue.instruction, Instruction::Narrative { .. })
                })
                .map(|cue| cue.cue_id)
                .collect()
        } else {
            BTreeSet::new()
        };
        self.closing = Some(Closing {
            scene_id: old_scene,
            clock: old_clock,
            outgoing,
            has_next,
        });
        self.phase = ScenePhase::Ending;
        self.ready = false;
    }
    fn close_terminal_if_finished(&mut self) {
        if self.phase == ScenePhase::Active
            && self.inner.engine().is_terminal()
            && !self
                .inner
                .engine()
                .output()
                .cues
                .iter()
                .any(|cue| matches!(cue.instruction, Instruction::Narrative { .. }))
        {
            self.begin_closing(self.scene_id(), self.inner.engine().clock.clone(), false);
        }
    }
    // The rule transaction and the internal input-lock acknowledgement share a
    // single provider commit. No preparation/begin or incoming media runs here.
    fn operation(
        &mut self,
        operation: impl FnOnce(&mut SceneEngine) -> SceneResult<CueSheet>,
        old_scene: String,
        old_clock: SceneClock,
    ) -> SceneResult<()> {
        let epoch = self.inner.engine().clock.epoch;
        self.inner.transact(|engine| {
            operation(engine)?;
            if engine.clock.epoch != epoch {
                let lock = engine.plan.first().expect("transition lock").cue_id.clone();
                engine.cue_input(&CueInput {
                    revision: engine.revision,
                    cue_id: lock,
                    scene_time_ms: 0,
                    event: CueEvent::Started,
                })?;
            }
            Ok(engine.output())
        })?;
        if self.inner.engine().clock.epoch != epoch {
            self.begin_closing(old_scene, old_clock, true);
        }
        self.close_terminal_if_finished();
        Ok(())
    }
    pub fn apply(&mut self, input: &SceneAction) -> SceneResult<SceneCueSheet> {
        if self.phase != ScenePhase::Active || input.revision != self.revision {
            return Err(SceneError("action is stale or scene is not active".into()));
        }
        let next = self.next_revision()?;
        let mut old_clock = self.inner.engine().clock.clone();
        old_clock.elapsed_ms = input.scene_time_ms;
        let translated = SceneAction {
            revision: self.inner.engine().revision,
            ..input.clone()
        };
        self.operation(
            |engine| engine.apply(&translated),
            self.scene_id(),
            old_clock,
        )?;
        self.revision = next;
        self.history.push(LifecycleInput::Action(input.clone()));
        self.register();
        Ok(self.output())
    }
    /// Interpret a host-detected deadline. No physical clock is read or run.
    /// Invalid/early/stale/duplicate notifications leave state untouched. A
    /// duplicate after an accepted timeout is rejected, never applied twice.
    /// The existing replay checkpoint records this as an observation for now;
    /// a host-owned semantic journal/continuation is a separate planned change.
    pub fn deadline_reached(&mut self, input: &DeadlineReached) -> SceneResult<SceneCueSheet> {
        let current = self.output();
        if self.phase != ScenePhase::Active
            || self.inner.engine().is_terminal()
            || input.revision != self.revision
            || input.scene_id != current.scene.scene_id
            || input.epoch != current.scene.epoch
            || current.plan.clock.deadline_ms != Some(input.deadline_ms)
            || input.scene_time_ms < input.deadline_ms
        {
            return Err(SceneError(
                "deadline notification is early, stale or not active".into(),
            ));
        }
        self.observe(input.revision, input.scene_time_ms)
    }

    pub fn observe(&mut self, revision: u64, time: u64) -> SceneResult<SceneCueSheet> {
        if self.phase != ScenePhase::Active || revision != self.revision {
            return Err(SceneError(
                "observation is stale or scene clock is frozen".into(),
            ));
        }
        let next = self.next_revision()?;
        let mut old_clock = self.inner.engine().clock.clone();
        old_clock.elapsed_ms = old_clock
            .deadline_ms
            .filter(|end| time >= *end)
            .unwrap_or(time);
        let before = self.inner.engine().revision;
        let phase = self.phase;
        let inner_revision = before;
        self.operation(
            |engine| engine.observe(inner_revision, time),
            self.scene_id(),
            old_clock,
        )?;
        if self.inner.engine().revision != before || self.phase != phase {
            self.revision = next;
        }
        if let Some(LifecycleInput::Observe {
            revision: previous,
            scene_time_ms,
        }) = self.history.last_mut()
        {
            if *previous == revision {
                *scene_time_ms = time;
            } else {
                self.history.push(LifecycleInput::Observe {
                    revision,
                    scene_time_ms: time,
                });
            }
        } else {
            self.history.push(LifecycleInput::Observe {
                revision,
                scene_time_ms: time,
            });
        }
        self.register();
        Ok(self.output())
    }
    pub fn cue_input(&mut self, input: &CueInput) -> SceneResult<SceneCueSheet> {
        if !matches!(self.phase, ScenePhase::Active | ScenePhase::Ending) {
            return Err(SceneError(
                "cue feedback outside active/ending scene".into(),
            ));
        }
        let current = self.output();
        let Some((epoch, inner_revision, ids)) = self.issued.get(&input.revision) else {
            return Err(SceneError("cue feedback was not issued".into()));
        };
        if *epoch != current.scene.epoch || !ids.contains(&input.cue_id) {
            return Err(SceneError(
                "cue feedback belongs to another scene/offer".into(),
            ));
        }
        if self.inner.engine().statuses.get(&input.cue_id) == Some(&Status::Completed) {
            return Ok(current);
        }
        if !current
            .plan
            .cues
            .iter()
            .any(|cue| cue.cue_id == input.cue_id)
        {
            return Err(SceneError("cue was replaced".into()));
        }
        let next = self.next_revision()?;
        let phase = self.phase;
        let before = self.inner.engine().revision;
        let mut old_clock = current.plan.clock;
        let ending = self.phase == ScenePhase::Ending;
        if ending && input.scene_time_ms != old_clock.elapsed_ms {
            return Err(SceneError(
                "ending presentation cannot advance gameplay time".into(),
            ));
        }
        if !ending {
            old_clock.elapsed_ms = old_clock
                .deadline_ms
                .filter(|end| input.scene_time_ms >= *end)
                .unwrap_or(input.scene_time_ms);
        }
        let translated = CueInput {
            revision: if ending {
                self.inner.engine().revision
            } else {
                *inner_revision
            },
            scene_time_ms: if ending { 0 } else { input.scene_time_ms },
            ..input.clone()
        };
        let length = self.inner.engine().history.len();
        self.operation(
            |engine| engine.cue_input(&translated),
            current.scene.scene_id,
            old_clock,
        )?;
        if self.inner.engine().revision != before || self.phase != phase {
            self.revision = next;
        }
        if self.inner.engine().history.len() != length {
            self.history.push(LifecycleInput::Cue(input.clone()));
        }
        self.register();
        Ok(self.output())
    }
    pub fn scene_input(&mut self, input: &SceneFeedback) -> SceneResult<SceneCueSheet> {
        // Idempotent accepted feedback cannot reactivate a later interaction.
        if self
            .history
            .iter()
            .any(|event| matches!(event,LifecycleInput::Scene(accepted) if accepted==input))
        {
            return Ok(self.output());
        }
        let current = self.output();
        if input.revision != self.revision
            || input.scene_id != current.scene.scene_id
            || input.epoch != current.scene.epoch
        {
            return Err(SceneError("stale scene lifecycle feedback".into()));
        }
        let next = self.next_revision()?;
        self.inner.checked_world_snapshot()?;
        match (self.phase, input.event) {
            (ScenePhase::Prepare, SceneEvent::Ready) => self.ready = true,
            (ScenePhase::Prepare, SceneEvent::PreparationFailed) => self.ready = false,
            (ScenePhase::Prepare, SceneEvent::Begin) if self.ready => {
                self.phase = ScenePhase::Active;
                self.ready = false;
                self.issued.clear();
                self.close_terminal_if_finished();
            }
            (ScenePhase::Ending, SceneEvent::End) if current.plan.cues.is_empty() => {
                self.phase = ScenePhase::Stable
            }
            (ScenePhase::Stable, SceneEvent::Prepare)
                if self.closing.as_ref().is_some_and(|c| c.has_next) =>
            {
                self.phase = ScenePhase::Prepare;
                self.closing = None;
            }
            _ => {
                return Err(SceneError(
                    "scene lifecycle barrier is not satisfied".into(),
                ));
            }
        }
        self.revision = next;
        self.history.push(LifecycleInput::Scene(input.clone()));
        self.register();
        Ok(self.output())
    }
    pub fn checkpoint(&self) -> SceneResult<BoundaryCheckpoint> {
        if self.phase != ScenePhase::Stable {
            return Err(SceneError(
                "M4.6 checkpoints require a stable scene boundary".into(),
            ));
        }
        // Provider snapshot must still correspond to the exact committed engine.
        let world = self.inner.checked_world_snapshot()?;
        Ok(BoundaryCheckpoint {
            schema_version: 1,
            story_id: self.inner.engine().story.id.clone(),
            story_version: self.inner.engine().story.version,
            configuration_id: self.configuration.id.clone(),
            configuration_version: self.configuration.version,
            revision: self.revision,
            history: self.history.clone(),
            world,
        })
    }
    pub fn restore(&mut self, save: BoundaryCheckpoint) -> SceneResult<SceneCueSheet> {
        if self.phase != ScenePhase::Stable
            && !(self.phase == ScenePhase::Prepare && self.history.is_empty())
        {
            return Err(SceneError(
                "M4.6 restore requires a stable scene boundary or fresh preparation".into(),
            ));
        }
        if save.schema_version != 1
            || save.story_id != self.inner.engine().story.id
            || save.story_version != self.inner.engine().story.version
            || save.configuration_id != self.configuration.id
            || save.configuration_version != self.configuration.version
            || save.history.len() > 65536
        {
            return Err(SceneError(
                "boundary checkpoint identity/version/history mismatch".into(),
            ));
        }
        let revision = self
            .revision
            .max(save.revision)
            .checked_add(1)
            .ok_or_else(|| SceneError("restore revision exhausted".into()))?;
        let mut replay = LifecycleEngine::new(
            (*self.inner.engine().story).clone(),
            self.configuration.clone(),
        )?;
        for input in &save.history {
            match input {
                LifecycleInput::Action(action) => {
                    replay.apply(action)?;
                }
                LifecycleInput::Cue(cue) => {
                    replay.cue_input(cue)?;
                }
                LifecycleInput::Observe {
                    revision,
                    scene_time_ms,
                } => {
                    replay.observe(*revision, *scene_time_ms)?;
                }
                LifecycleInput::Scene(scene) => {
                    replay.scene_input(scene)?;
                }
                LifecycleInput::RestoreEpoch { revision } => {
                    if *revision <= replay.revision {
                        return Err(SceneError("invalid lifecycle restore reservation".into()));
                    }
                    replay.revision = *revision;
                    replay.issued.clear();
                    replay.register();
                    replay.history.push(input.clone());
                }
            }
        }
        let world = replay.world_snapshot();
        if replay.phase != ScenePhase::Stable
            || replay.revision != save.revision
            || world.scope != save.world.scope
            || world.globals != save.world.globals
            || world.locals != save.world.locals
        {
            return Err(SceneError(
                "boundary checkpoint differs from deterministic replay".into(),
            ));
        }
        self.inner.replace_replayed(replay.inner.engine().clone())?;
        self.phase = replay.phase;
        self.ready = replay.ready;
        self.closing = replay.closing;
        self.history = replay.history;
        self.revision = revision;
        self.history.push(LifecycleInput::RestoreEpoch { revision });
        self.issued.clear();
        self.register();
        Ok(self.output())
    }
}
