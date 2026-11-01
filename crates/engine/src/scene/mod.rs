//! Opt-in M2 scenes. Legacy Engine, stories, timers and saves are unchanged.
pub mod authoring;
mod expressions;
mod validation;
use expressions::{Times, check, update};
use grimm_semantic::{scene::*, story::Variables};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneError(pub String);
impl fmt::Display for SceneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for SceneError {}
pub type SceneResult<T> = Result<T, SceneError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Started,
    Completed,
}

#[derive(Debug, Clone)]
pub struct SceneEngine {
    story: Arc<SceneStory>,
    context: usize,
    globals: Variables,
    locals: Variables,
    clock: SceneClock,
    revision: u64,
    eligible_ms: u64,
    plan: Vec<Cue>,
    statuses: BTreeMap<String, Status>,
    cue_started_at: BTreeMap<String, u64>,
    issued: BTreeMap<u64, (u64, BTreeSet<String>)>,
    offered: Vec<String>,
    records: Vec<ActionRecord>,
    history: Vec<SceneInput>,
}

impl SceneEngine {
    pub fn new(story: SceneStory) -> SceneResult<Self> {
        let story = Arc::new(validation::compile(story)?);
        let context = story
            .contexts
            .iter()
            .position(|ctx| ctx.id == story.start)
            .expect("validated start");
        let mut engine = Self {
            globals: story.globals.clone(),
            locals: story.contexts[context].locals.clone(),
            story,
            context,
            clock: SceneClock {
                epoch: 0,
                elapsed_ms: 0,
                running: false,
                deadline_ms: None,
                visible: false,
            },
            revision: 0,
            eligible_ms: 0,
            plan: vec![],
            statuses: BTreeMap::new(),
            cue_started_at: BTreeMap::new(),
            issued: BTreeMap::new(),
            offered: vec![],
            records: vec![],
            history: vec![],
        };
        engine.enter_clock();
        engine.plan = engine.materialize(&engine.ctx().cues, None, Times::default())?;
        engine.register();
        Ok(engine)
    }

    fn ctx(&self) -> &Context {
        &self.story.contexts[self.context]
    }
    fn enter_clock(&mut self) {
        self.clock.running = self.story.schema_version >= 2
            || !self.ctx().cues.iter().any(|cue| {
                matches!(
                    cue.instruction,
                    AuthoredInstruction::Timer {
                        start_ms: Some(_),
                        ..
                    }
                )
            });
    }
    fn times(&self, selected: u64) -> Times {
        Times {
            selection: selected,
            end: selected,
            deadline: self.clock.deadline_ms,
            ..Times::default()
        }
    }

    fn allowed(&self, action: &Action, time: Times) -> SceneResult<bool> {
        match &action.when {
            Some(test) => check(test, &self.globals, &self.locals, time, 0),
            None => Ok(true),
        }
    }

    /// Complete remaining plan only; no graph IDs, world values, assets or ending markers.
    pub fn output(&self) -> CueSheet {
        // Active offer state is idempotent coordination, not media replay. Keep
        // its last executed instruction for a fresh renderer/checkpoint restore.
        let active_offer = self.plan.iter().rev().find(|cue| matches!(&cue.instruction, Instruction::Actions { available_actions } if !available_actions.is_empty() && *available_actions == self.offered) && self.statuses.contains_key(&cue.cue_id)).map(|cue| &cue.cue_id);
        CueSheet {
            protocol_version: 2,
            revision: self.revision,
            clock: self.clock.clone(),
            cues: self
                .plan
                .iter()
                .filter(|cue| {
                    self.statuses.get(&cue.cue_id) != Some(&Status::Completed)
                        || active_offer == Some(&cue.cue_id)
                })
                .cloned()
                .map(|mut cue| {
                    cue.started_at_ms = if self.statuses.get(&cue.cue_id) == Some(&Status::Started)
                    {
                        self.cue_started_at.get(&cue.cue_id).copied()
                    } else {
                        None
                    };
                    if cue
                        .after_cue
                        .as_ref()
                        .is_some_and(|id| self.statuses.get(id) == Some(&Status::Completed))
                    {
                        cue.after_cue = None;
                    }
                    cue
                })
                .collect(),
        }
    }

    /// Immutable host-only state; never included in a Cue Sheet.
    pub fn inspect_world(&self) -> WorldStateSnapshot {
        WorldStateSnapshot {
            context_id: self.ctx().id.clone(),
            scene_epoch: self.clock.epoch,
            revision: self.revision,
            globals: self.globals.clone(),
            locals: self.locals.clone(),
        }
    }

    /// Host/author inspection, never sent to a renderer.
    pub fn globals(&self) -> &Variables {
        &self.globals
    }
    pub fn locals(&self) -> &Variables {
        &self.locals
    }
    pub fn action_history(&self) -> &[ActionRecord] {
        &self.records
    }
    pub fn is_terminal(&self) -> bool {
        self.ctx().terminal
    }
    pub fn title(&self) -> &str {
        &self.story.title
    }

    fn register(&mut self) {
        self.issued.insert(
            self.revision,
            (
                self.clock.epoch,
                self.plan.iter().map(|cue| cue.cue_id.clone()).collect(),
            ),
        );
    }
    fn next_revision(&mut self) -> SceneResult<()> {
        self.revision = self
            .revision
            .checked_add(1)
            .ok_or_else(|| SceneError("scene revision exhausted".into()))?;
        Ok(())
    }
    fn selection_valid(&self, revision: u64, time: u64) -> SceneResult<()> {
        if revision != self.revision {
            return Err(SceneError("stale Cue Sheet revision".into()));
        }
        if time < self.clock.elapsed_ms
            || time > i64::MAX as u64
            || (!self.clock.running && time != 0)
        {
            return Err(SceneError(
                "invalid/regressing scene time or clock has not started".into(),
            ));
        }
        Ok(())
    }

    /// Clone candidate state so effects, time, context, histories and plan commit together.
    pub fn apply(&mut self, input: &SceneAction) -> SceneResult<CueSheet> {
        let mut candidate = self.clone();
        candidate.act(input)?;
        candidate.history.push(SceneInput::Action(input.clone()));
        *self = candidate;
        Ok(self.output())
    }

    pub fn cue_input(&mut self, input: &CueInput) -> SceneResult<CueSheet> {
        let mut candidate = self.clone();
        if candidate.ack(input)? {
            candidate.history.push(SceneInput::Cue(input.clone()));
        }
        *self = candidate;
        Ok(self.output())
    }

    pub fn observe(&mut self, revision: u64, scene_time_ms: u64) -> SceneResult<CueSheet> {
        let mut candidate = self.clone();
        candidate.selection_valid(revision, scene_time_ms)?;
        let newly_due = candidate.story.schema_version >= 2
            && !candidate.is_terminal()
            && candidate
                .clock
                .deadline_ms
                .is_some_and(|end| scene_time_ms >= end);
        if scene_time_ms != candidate.clock.elapsed_ms || newly_due {
            candidate.clock.elapsed_ms = scene_time_ms;
            if candidate
                .clock
                .deadline_ms
                .is_some_and(|end| scene_time_ms >= end)
                && !candidate.is_terminal()
            {
                candidate.expire(scene_time_ms)?;
            }
            // Consecutive observations have no intervening semantic event.
            // Retain their latest value so an idle host does not grow saves
            // with every renderer clock sample.
            if let Some(SceneInput::Observe {
                revision: previous_revision,
                scene_time_ms: previous_time,
            }) = candidate.history.last_mut()
                && *previous_revision == revision
            {
                *previous_time = scene_time_ms;
            } else {
                candidate.history.push(SceneInput::Observe {
                    revision,
                    scene_time_ms,
                });
            }
        }
        *self = candidate;
        Ok(self.output())
    }

    fn materialize(
        &self,
        authored: &[AuthoredCue],
        run: Option<usize>,
        time: Times,
    ) -> SceneResult<Vec<Cue>> {
        let mut result = vec![];
        let mut previous_sequential = None;
        let local_ids: BTreeSet<_> = authored
            .iter()
            .filter_map(|cue| cue.cue_id.as_ref())
            .collect();
        for cue in authored {
            if let Some(test) = &cue.when
                && !check(test, &self.globals, &self.locals, time, 0)?
            {
                continue;
            }
            let base = cue.cue_id.as_ref().expect("compiled cue ID");
            let cue_id = run.map_or_else(|| base.clone(), |run| format!("{base}__run_{run}"));
            if self.statuses.get(&cue_id) == Some(&Status::Completed) {
                continue;
            }
            let instruction = match &cue.instruction {
                AuthoredInstruction::Narrative {
                    narrative_id,
                    duration_ms,
                } => Instruction::Narrative {
                    narrative_id: narrative_id.clone(),
                    duration_ms: *duration_ms,
                },
                AuthoredInstruction::Timer {
                    start_ms,
                    deadline_at_ms,
                    unset_deadline,
                    extend_ms,
                    visible,
                } => Instruction::Timer {
                    start_ms: *start_ms,
                    deadline_at_ms: *deadline_at_ms,
                    unset_deadline: *unset_deadline,
                    extend_ms: *extend_ms,
                    visible: *visible,
                },
                AuthoredInstruction::Actions { available_actions } => {
                    let mut names = vec![];
                    for action in &self.ctx().actions {
                        let requested = match available_actions {
                            ActionSet::Eligible(_) => true,
                            ActionSet::Only(ids) => ids.contains(&action.id),
                        };
                        if requested && self.allowed(action, time)? {
                            names.push(action.id.clone());
                        }
                    }
                    Instruction::Actions {
                        available_actions: names,
                    }
                }
            };
            let authored_at = cue.at_ms.unwrap_or(time.selection);
            let mut at_ms = if run.is_some() {
                authored_at.max(time.selection)
            } else {
                authored_at
            };
            if matches!(&instruction, Instruction::Actions { available_actions } if !available_actions.is_empty())
            {
                at_ms = at_ms.max(self.eligible_ms);
            }
            let explicit_after = cue.after_cue.as_ref().map(|id| {
                if let Some(run) = run.filter(|_| local_ids.contains(id)) {
                    format!("{id}__run_{run}")
                } else {
                    id.clone()
                }
            });
            let after_cue = explicit_after.or_else(|| {
                if cue.at_ms.is_none() {
                    previous_sequential.clone()
                } else {
                    None
                }
            });
            if cue.at_ms.is_none() {
                previous_sequential = Some(cue_id.clone());
            }
            result.push(Cue {
                cue_id,
                at_ms,
                started_at_ms: None,
                after_cue,
                instruction,
            });
        }
        Ok(result)
    }

    fn act(&mut self, input: &SceneAction) -> SceneResult<()> {
        self.selection_valid(input.revision, input.scene_time_ms)?;
        if self.is_terminal() {
            return Err(SceneError("terminal context cannot accept actions".into()));
        }
        if input.action_id == "timeout" {
            if !self
                .clock
                .deadline_ms
                .is_some_and(|end| input.scene_time_ms >= end)
            {
                return Err(SceneError("timeout before an active deadline".into()));
            }
            return self.expire(input.scene_time_ms);
        }
        if !self.offered.contains(&input.action_id) || input.scene_time_ms < self.eligible_ms {
            return Err(SceneError(
                "action is not currently available or its logical time has not elapsed".into(),
            ));
        }
        let action = self
            .ctx()
            .actions
            .iter()
            .find(|action| action.id == input.action_id)
            .cloned()
            .ok_or_else(|| SceneError("unknown action".into()))?;
        let selected = input.scene_time_ms;
        if self.clock.deadline_ms.is_some_and(|end| selected > end) {
            return self.expire(selected);
        }
        if let Some(test) = &action.when
            && !check(test, &self.globals, &self.locals, self.times(selected), 0)?
        {
            return Err(SceneError("action condition no longer holds".into()));
        }
        let nominal = selected
            .checked_add(action.duration_ms)
            .filter(|end| *end <= i64::MAX as u64)
            .ok_or_else(|| SceneError("action time overflow".into()))?;
        let end = self
            .clock
            .deadline_ms
            .map_or(nominal, |deadline| nominal.min(deadline));
        let time = Times {
            selection: selected,
            end,
            duration: action.duration_ms,
            effective: end.saturating_sub(selected),
            deadline: self.clock.deadline_ms,
            interrupted: end < nominal,
        };
        let branches = if time.interrupted {
            &action.interrupted_outcomes
        } else {
            &action.outcomes
        };
        if !time.interrupted {
            update(&action.effects, &mut self.globals, &mut self.locals, time)?;
        }
        let outcome = if branches.is_empty() {
            self.ctx().timeout.clone().ok_or_else(|| {
                SceneError("interrupted action has no authored timeout outcome".into())
            })?
        } else {
            let mut selected_outcome = None;
            for branch in branches {
                if match &branch.when {
                    Some(test) => check(test, &self.globals, &self.locals, time, 0)?,
                    None => true,
                } {
                    selected_outcome = Some(branch.outcome.clone());
                    break;
                }
            }
            selected_outcome.ok_or_else(|| SceneError("action has no matching outcome".into()))?
        };
        if time.interrupted && self.story.schema_version >= 2 {
            self.clock.deadline_ms = None;
            self.clock.visible = false;
        }
        self.records.push(ActionRecord {
            action_id: action.id,
            selected_ms: selected,
            effective_ms: time.effective,
            logical_end_ms: end,
            interrupted: time.interrupted,
        });
        self.resolve(outcome, time)
    }

    fn expire(&mut self, at: u64) -> SceneResult<()> {
        let outcome = self
            .ctx()
            .timeout
            .clone()
            .ok_or_else(|| SceneError("no authored timeout outcome".into()))?;
        let time = Times {
            end: self.clock.deadline_ms.unwrap_or(at),
            ..self.times(at)
        };
        // A v2 deadline expires once, including timeout outcomes that stay in
        // the same context. Its old value remains available in Times for rules.
        if self.story.schema_version >= 2 {
            self.clock.deadline_ms = None;
            self.clock.visible = false;
        }
        self.records.push(ActionRecord {
            action_id: "timeout".into(),
            selected_ms: at,
            effective_ms: 0,
            logical_end_ms: time.end,
            interrupted: true,
        });
        self.resolve(outcome, time)
    }

    fn resolve(&mut self, outcome: Outcome, time: Times) -> SceneResult<()> {
        update(&outcome.effects, &mut self.globals, &mut self.locals, time)?;
        let previous = self.plan.clone();
        self.eligible_ms = time.end;
        let mut result = self.materialize(&outcome.cues, Some(self.records.len()), time)?;
        self.clock.elapsed_ms = time.selection;
        self.eligible_ms = time.end;
        self.offered.clear();
        self.next_revision()?;
        let transitioned = outcome.goto.is_some();
        if let Some(target) = outcome.goto {
            self.context = self
                .story
                .contexts
                .iter()
                .position(|ctx| ctx.id == target)
                .expect("validated target");
            self.locals = self.ctx().locals.clone();
            self.clock = SceneClock {
                epoch: self
                    .clock
                    .epoch
                    .checked_add(1)
                    .ok_or_else(|| SceneError("scene epoch exhausted".into()))?,
                elapsed_ms: 0,
                running: false,
                deadline_ms: None,
                visible: false,
            };
            self.enter_clock();
            // A new scene clock starts at zero, but transitioning must not
            // bypass the selected action's remaining logical duration.
            self.eligible_ms = time.end.saturating_sub(time.selection);
            self.statuses.clear();
            self.cue_started_at.clear();
            self.issued.clear();
            // Authored transition results are untimed and belong to the new epoch.
            for cue in &mut result {
                cue.at_ms = 0;
            }
        }
        let ns = self.ctx().namespace.clone().expect("compiled namespace");
        self.plan = vec![Cue {
            cue_id: format!("{ns}.__grimm_lock_{}", self.revision),
            at_ms: self.clock.elapsed_ms,
            started_at_ms: None,
            after_cue: None,
            instruction: Instruction::Actions {
                available_actions: vec![],
            },
        }];
        let lock_id = self.plan[0].cue_id.clone();
        let sequential_result: BTreeSet<_> = outcome
            .cues
            .iter()
            .filter(|cue| cue.at_ms.is_none())
            .filter_map(|cue| cue.cue_id.as_ref())
            .map(|id| format!("{id}__run_{}", self.records.len()))
            .collect();
        for cue in &mut result {
            if sequential_result.contains(&cue.cue_id) && cue.after_cue.is_none() {
                cue.after_cue = Some(lock_id.clone());
            }
        }
        let prefix_tail = result
            .iter()
            .rfind(|cue| sequential_result.contains(&cue.cue_id))
            .map_or(lock_id.clone(), |cue| cue.cue_id.clone());
        self.plan.extend(result);
        if !transitioned {
            // Carry only running narrative identities, never obsolete action lists.
            let retained: Vec<_> = previous
                .into_iter()
                .filter(|cue| {
                    matches!(cue.instruction, Instruction::Narrative { .. })
                        && self.statuses.get(&cue.cue_id) == Some(&Status::Started)
                })
                .map(|mut cue| {
                    cue.at_ms = time.selection;
                    cue
                })
                .collect();
            self.plan.extend(retained);
            // Entry untimed instructions are one-time; future clock-scheduled
            // cues are regenerated from canonical facts and semantic history.
            let entry = self.materialize(&self.ctx().cues, None, time)?;
            let scheduled: BTreeSet<_> = self
                .ctx()
                .cues
                .iter()
                .filter(|cue| cue.at_ms.is_some())
                .filter_map(|cue| cue.cue_id.as_ref())
                .collect();
            let pending: Vec<_> = entry.into_iter().filter(|cue| scheduled.contains(&cue.cue_id) || matches!(&cue.instruction, Instruction::Actions { available_actions } if !available_actions.is_empty())).collect();
            self.plan.extend(pending);
        } else {
            let mut entry = self.materialize(&self.ctx().cues, None, self.times(0))?;
            for cue in &mut entry {
                let authored = self
                    .ctx()
                    .cues
                    .iter()
                    .find(|a| a.cue_id.as_ref() == Some(&cue.cue_id))
                    .expect("compiled entry cue");
                if authored.at_ms.is_none() && cue.after_cue.is_none() {
                    cue.after_cue = Some(prefix_tail.clone());
                }
            }
            self.plan.extend(entry);
        }
        // A local action with no explicit action-list instruction defaults to a
        // full eligible replacement after its sequential narrative result.
        if !transitioned
            && !outcome
                .cues
                .iter()
                .any(|cue| matches!(cue.instruction, AuthoredInstruction::Actions { .. }))
        {
            let mut names = vec![];
            for action in &self.ctx().actions {
                if self.allowed(action, time)? {
                    names.push(action.id.clone());
                }
            }
            let sequential_ids: BTreeSet<_> = outcome
                .cues
                .iter()
                .filter(|cue| cue.at_ms.is_none())
                .filter_map(|cue| cue.cue_id.as_ref())
                .map(|id| format!("{id}__run_{}", self.records.len()))
                .collect();
            let after_cue = self
                .plan
                .iter()
                .rfind(|cue| sequential_ids.contains(&cue.cue_id))
                .map(|cue| cue.cue_id.clone())
                .or(Some(lock_id));
            self.plan.push(Cue {
                cue_id: format!("{ns}.__grimm_actions_{}", self.revision),
                at_ms: self.eligible_ms,
                started_at_ms: None,
                after_cue,
                instruction: Instruction::Actions {
                    available_actions: names,
                },
            });
        }
        let mut seen = BTreeSet::new();
        self.plan.retain(|cue| seen.insert(cue.cue_id.clone()));
        self.register();
        Ok(())
    }

    fn due(&self, index: usize, at: u64) -> bool {
        let cue = &self.plan[index];
        let timing = at >= cue.at_ms && (self.clock.running || cue.at_ms == 0);
        timing
            && cue
                .after_cue
                .as_ref()
                .is_none_or(|id| self.statuses.get(id) == Some(&Status::Completed))
            && (!matches!(&cue.instruction, Instruction::Actions { available_actions } if !available_actions.is_empty())
                || (at >= self.eligible_ms && self.clock.running))
    }

    fn ack(&mut self, input: &CueInput) -> SceneResult<bool> {
        let issued = self
            .issued
            .get(&input.revision)
            .is_some_and(|(epoch, ids)| *epoch == self.clock.epoch && ids.contains(&input.cue_id));
        if !issued {
            return Err(SceneError(
                "cue acknowledgement was not issued in this scene epoch".into(),
            ));
        }
        if self.statuses.get(&input.cue_id) == Some(&Status::Completed)
            && (self.story.schema_version >= 2 || input.revision == self.revision)
        {
            return Ok(false);
        }
        let Some(index) = self.plan.iter().position(|cue| cue.cue_id == input.cue_id) else {
            return Err(SceneError("cue is absent from the replacement plan".into()));
        };
        let cue = self.plan[index].clone();
        let status = self.statuses.get(&input.cue_id).copied();
        if status == Some(Status::Completed)
            || (status == Some(Status::Started) && input.event == CueEvent::Started)
        {
            return Ok(false);
        }
        let current = input.revision == self.revision;
        if !current
            && !(status == Some(Status::Started)
                && matches!(cue.instruction, Instruction::Narrative { .. })
                && input.event == CueEvent::Completed)
        {
            return Err(SceneError(
                "stale cue cannot activate a replacement plan".into(),
            ));
        }
        self.selection_valid(self.revision, input.scene_time_ms)?;
        if self
            .clock
            .deadline_ms
            .is_some_and(|end| input.scene_time_ms >= end)
            && !self.is_terminal()
        {
            self.expire(input.scene_time_ms)?;
            return Ok(true);
        }
        self.clock.elapsed_ms = input.scene_time_ms;
        match input.event {
            CueEvent::Started => {
                if !self.due(index, input.scene_time_ms) {
                    return Err(SceneError(
                        "cue is not yet due or preceding presentation is unfinished".into(),
                    ));
                }
                match &cue.instruction {
                    Instruction::Actions { available_actions } => {
                        self.offered = available_actions.clone()
                    }
                    Instruction::Timer {
                        start_ms,
                        deadline_at_ms,
                        unset_deadline,
                        extend_ms,
                        visible,
                    } => {
                        if let Some(duration) = start_ms {
                            self.clock.deadline_ms = Some(
                                input
                                    .scene_time_ms
                                    .checked_add(*duration)
                                    .filter(|end| *end <= i64::MAX as u64)
                                    .ok_or_else(|| SceneError("timer overflow".into()))?,
                            );
                            self.clock.running = true;
                        }
                        if let Some(at) = deadline_at_ms {
                            if *at < input.scene_time_ms {
                                return Err(SceneError(
                                    "absolute deadline cannot precede its execution time".into(),
                                ));
                            }
                            self.clock.deadline_ms = Some(*at);
                        }
                        if *unset_deadline {
                            self.clock.deadline_ms = None;
                            self.clock.visible = false;
                        }
                        if let Some(duration) = extend_ms {
                            self.clock.deadline_ms = Some(
                                self.clock
                                    .deadline_ms
                                    .ok_or_else(|| {
                                        SceneError("cannot extend a timer before start".into())
                                    })?
                                    .checked_add(*duration)
                                    .filter(|end| *end <= i64::MAX as u64)
                                    .ok_or_else(|| SceneError("timer extension overflow".into()))?,
                            );
                        }
                        if let Some(visible) = visible {
                            self.clock.visible = *visible;
                        }
                    }
                    Instruction::Narrative {
                        duration_ms: Some(duration),
                        ..
                    } => {
                        input
                            .scene_time_ms
                            .checked_add(*duration)
                            .filter(|end| *end <= i64::MAX as u64)
                            .ok_or_else(|| {
                                SceneError("narrative completion time overflow".into())
                            })?;
                    }
                    Instruction::Narrative { .. } => {}
                }
                let status = if self.story.schema_version >= 2
                    && !matches!(cue.instruction, Instruction::Narrative { .. })
                {
                    Status::Completed
                } else {
                    Status::Started
                };
                self.statuses.insert(input.cue_id.clone(), status);
                self.cue_started_at
                    .insert(input.cue_id.clone(), input.scene_time_ms);
            }
            CueEvent::Completed => {
                if status != Some(Status::Started) {
                    return Err(SceneError("cue_completed requires cue_started".into()));
                }
                self.statuses
                    .insert(input.cue_id.clone(), Status::Completed);
            }
        }
        Ok(true)
    }

    pub fn save(&self) -> SceneSave {
        SceneSave {
            schema_version: self.story.schema_version,
            story_id: self.story.id.clone(),
            story_version: self.story.version,
            revision: self.revision,
            history: self.history.clone(),
        }
    }

    /// Validate by deterministic replay before replacing canonical state. Offline
    /// time is never inferred. Old callbacks cannot cross the restore epoch.
    pub fn restore(&mut self, save: SceneSave) -> SceneResult<CueSheet> {
        if save.schema_version != self.story.schema_version
            || save.story_id != self.story.id
            || save.story_version != self.story.version
            || save.history.len() > 65536
        {
            return Err(SceneError(
                "scene checkpoint schema/story/version/history does not match".into(),
            ));
        }
        let mut candidate = Self::new((*self.story).clone())?;
        // Self::new normalizes already compiled IDs and regenerates neither
        // explicit IDs nor graph semantics; replay validates every logged input.
        for event in &save.history {
            match event {
                SceneInput::Action(action) => {
                    candidate.apply(action)?;
                }
                SceneInput::Cue(cue) => {
                    candidate.cue_input(cue)?;
                }
                SceneInput::Observe {
                    revision,
                    scene_time_ms,
                } => {
                    candidate.observe(*revision, *scene_time_ms)?;
                }
                SceneInput::RestoreEpoch { revision } => {
                    if *revision <= candidate.revision {
                        return Err(SceneError("invalid restore epoch in history".into()));
                    }
                    candidate.revision = *revision;
                    candidate.issued.clear();
                    candidate.register();
                    candidate.history.push(event.clone());
                }
            }
        }
        if candidate.revision != save.revision {
            return Err(SceneError(
                "checkpoint revision does not match replay".into(),
            ));
        }
        candidate.revision = self
            .revision
            .max(save.revision)
            .checked_add(1)
            .ok_or_else(|| SceneError("restore revision exhausted".into()))?;
        candidate.issued.clear();
        candidate.register();
        candidate.history.push(SceneInput::RestoreEpoch {
            revision: candidate.revision,
        });
        *self = candidate;
        Ok(self.output())
    }
}
