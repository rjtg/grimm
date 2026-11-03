//! Atomic backing storage. Engine rule evaluation remains authoritative; this
//! interface never runs story conditions or reads wall-clock time.
use super::{SceneError, SceneResult};
use grimm_semantic::{
    story::{Value, Variables},
    world::*,
};

/// Implementations must reject a failed transaction without *any* persisted
/// changes. Snapshot/commit is one revision-CAS transaction, never a retry loop.
/// Empty mutation lists atomically validate the revision without incrementing it.
/// Host writes must advance the storage revision and be synchronized through
/// refresh before further narrative input. Independent unrevisioned writes are
/// contract violations; input is never silently retried against changed facts.
///
/// This is a trusted storage contract, not a rollback service. A successful
/// commit/restore must atomically persist and return the exact requested state
/// with the required revision. Conformance checks happen after that operation;
/// the engine cannot undo persisted writes from a provider that violates this
/// contract (including one that writes and then returns an error). Such a
/// violation requires host intervention; never retry it as an ordinary action.
/// Custom implementations should run the rejection, CAS, type/overflow, scope
/// and restore conformance scenarios in the workspace's world_provider tests.
pub trait WorldStateProvider: std::fmt::Debug {
    /// One consistent typed projection; missing/ambiguous bindings and failed
    /// reads return Err. No default/fallback values may mask a missing field.
    fn snapshot(&self) -> SceneResult<WorldSnapshot>;
    fn commit(
        &mut self,
        expected_revision: u64,
        mutations: &[StateMutation],
    ) -> SceneResult<WorldSnapshot>;
    /// Called only after engine replay has validated a checkpoint. Persist the
    /// whole restored scope/state atomically and reserve a fresh storage revision.
    fn restore(
        &mut self,
        expected_revision: u64,
        snapshot: &WorldSnapshot,
    ) -> SceneResult<WorldSnapshot>;
}
#[derive(Debug, Clone)]
pub struct InMemoryWorldState {
    state: WorldSnapshot,
}
impl InMemoryWorldState {
    pub fn new(scope: WorldScope, globals: Variables, locals: Variables) -> SceneResult<Self> {
        if scope.context_id.trim().is_empty() {
            return Err(SceneError("empty world scope".into()));
        }
        Ok(Self {
            state: WorldSnapshot {
                revision: 0,
                scope,
                globals,
                locals,
            },
        })
    }
    fn next_revision(&self, expected: u64) -> SceneResult<u64> {
        if self.state.revision != expected {
            return Err(SceneError(format!(
                "stale world revision: expected {expected}, actual {}",
                self.state.revision
            )));
        }
        expected
            .checked_add(1)
            .ok_or_else(|| SceneError("world revision exhausted".into()))
    }
}
fn same_type(a: &Value, b: &Value) -> bool {
    matches!(
        (a, b),
        (Value::Bool(_), Value::Bool(_))
            | (Value::Int(_), Value::Int(_))
            | (Value::Text(_), Value::Text(_))
    )
}
fn variables(state: &mut WorldSnapshot, scope: VariableScope) -> &mut Variables {
    match scope {
        VariableScope::Global => &mut state.globals,
        VariableScope::Local => &mut state.locals,
    }
}
impl WorldStateProvider for InMemoryWorldState {
    fn snapshot(&self) -> SceneResult<WorldSnapshot> {
        Ok(self.state.clone())
    }
    fn commit(
        &mut self,
        expected_revision: u64,
        mutations: &[StateMutation],
    ) -> SceneResult<WorldSnapshot> {
        if mutations.is_empty() {
            if self.state.revision != expected_revision {
                return Err(SceneError("stale world revision".into()));
            }
            return self.snapshot();
        }
        let revision = self.next_revision(expected_revision)?;
        let mut candidate = self.state.clone();
        let mut entered = false;
        for mutation in mutations {
            match mutation {
                StateMutation::Set { scope, name, value } => {
                    let old = variables(&mut candidate, *scope)
                        .get_mut(name)
                        .ok_or_else(|| SceneError(format!("unknown world variable '{name}'")))?;
                    if !same_type(old, value) {
                        return Err(SceneError(format!("world variable '{name}' type mismatch")));
                    }
                    *old = value.clone();
                }
                StateMutation::Add { scope, name, delta } => {
                    let old = variables(&mut candidate, *scope)
                        .get_mut(name)
                        .ok_or_else(|| SceneError(format!("unknown world variable '{name}'")))?;
                    let Value::Int(value) = old else {
                        return Err(SceneError(format!(
                            "world variable '{name}' is not integer"
                        )));
                    };
                    *value = value
                        .checked_add(*delta)
                        .ok_or_else(|| SceneError(format!("world variable '{name}' overflow")))?;
                }
                StateMutation::Enter { scope, locals } => {
                    if entered
                        || scope.context_id.trim().is_empty()
                        || candidate.scope.epoch.checked_add(1) != Some(scope.epoch)
                    {
                        return Err(SceneError(
                            "invalid or repeated world scope transition".into(),
                        ));
                    }
                    candidate.scope = scope.clone();
                    candidate.locals = locals.clone();
                    entered = true;
                }
            }
        }
        candidate.revision = revision;
        self.state = candidate;
        self.snapshot()
    }
    fn restore(
        &mut self,
        expected_revision: u64,
        snapshot: &WorldSnapshot,
    ) -> SceneResult<WorldSnapshot> {
        let revision = self.next_revision(expected_revision)?;
        if snapshot.scope.context_id.trim().is_empty()
            || snapshot.globals.len() != self.state.globals.len()
            || self.state.globals.iter().any(|(name, value)| {
                snapshot
                    .globals
                    .get(name)
                    .is_none_or(|new| !same_type(value, new))
            })
        {
            return Err(SceneError("restored world schema mismatch".into()));
        }
        let mut candidate = snapshot.clone();
        candidate.revision = revision;
        self.state = candidate;
        self.snapshot()
    }
}

/// Injected-storage composition without changing legacy SceneEngine clone/save
/// behavior. The inner engine is a consistent last-committed rule snapshot;
/// only this wrapper can mutate it. Providers own the persisted values.
#[derive(Debug)]
pub struct ProvidedSceneEngine<P> {
    engine: super::SceneEngine,
    provider: P,
    committed: WorldSnapshot,
    replay_supported: bool,
}
impl<P: WorldStateProvider> ProvidedSceneEngine<P> {
    pub fn new(
        story: grimm_semantic::scene::SceneStory,
        make_provider: impl FnOnce(WorldSnapshot) -> SceneResult<P>,
    ) -> SceneResult<Self> {
        Self::from_engine(super::SceneEngine::new(story)?, make_provider)
    }
    pub(super) fn from_engine(
        engine: super::SceneEngine,
        make_provider: impl FnOnce(WorldSnapshot) -> SceneResult<P>,
    ) -> SceneResult<Self> {
        let world = engine.inspect_world();
        let initial = WorldSnapshot {
            revision: 0,
            scope: WorldScope {
                context_id: world.context_id,
                epoch: world.scene_epoch,
            },
            globals: world.globals,
            locals: world.locals,
        };
        let provider = make_provider(initial.clone())?;
        if provider.snapshot()? != initial {
            return Err(SceneError(
                "provider initialization does not match story defaults".into(),
            ));
        }
        Ok(Self {
            engine,
            provider,
            committed: initial,
            replay_supported: true,
        })
    }
    pub(super) fn checked_world_snapshot(&self) -> SceneResult<WorldSnapshot> {
        self.ensure_current()?;
        Ok(self.committed.clone())
    }
    fn ensure_current(&self) -> SceneResult<()> {
        if self.provider.snapshot()? != self.committed {
            return Err(SceneError(
                "stale or independently mutated world provider; no retry performed".into(),
            ));
        }
        Ok(())
    }
    pub(super) fn transact(
        &mut self,
        operation: impl FnOnce(&mut super::SceneEngine) -> SceneResult<grimm_semantic::scene::CueSheet>,
    ) -> SceneResult<grimm_semantic::scene::CueSheet> {
        self.ensure_current()?;
        let mut candidate = self.engine.clone();
        operation(&mut candidate)?;
        let world = candidate.inspect_world();
        let scope = WorldScope {
            context_id: world.context_id,
            epoch: world.scene_epoch,
        };
        let mut mutations = vec![];
        for (name, value) in &world.globals {
            if self.committed.globals.get(name) != Some(value) {
                mutations.push(StateMutation::Set {
                    scope: VariableScope::Global,
                    name: name.clone(),
                    value: value.clone(),
                });
            }
        }
        if scope != self.committed.scope {
            mutations.push(StateMutation::Enter {
                scope: scope.clone(),
                locals: world.locals.clone(),
            });
        } else {
            for (name, value) in &world.locals {
                if self.committed.locals.get(name) != Some(value) {
                    mutations.push(StateMutation::Set {
                        scope: VariableScope::Local,
                        name: name.clone(),
                        value: value.clone(),
                    });
                }
            }
        }
        let next = self.provider.commit(self.committed.revision, &mutations)?;
        // A read-only transaction still performs an atomic revision check.
        let expected_revision = if mutations.is_empty() {
            Some(self.committed.revision)
        } else {
            self.committed.revision.checked_add(1)
        };
        if next.scope != scope
            || next.globals != world.globals
            || next.locals != world.locals
            || expected_revision != Some(next.revision)
        {
            return Err(SceneError(
                "world provider violated its atomic commit contract".into(),
            ));
        }
        self.committed = next;
        self.engine = candidate;
        Ok(self.engine.output())
    }
    /// Explicitly import an authorized host revision. Does not run effects,
    /// advance clocks or replay media. Caller must present the new action revision.
    pub fn refresh(&mut self) -> SceneResult<bool> {
        let world = self.provider.snapshot()?;
        if world == self.committed {
            if self.provider.commit(world.revision, &[])? != world {
                return Err(SceneError("provider changed during refresh".into()));
            }
            return Ok(false);
        }
        if world.revision <= self.committed.revision {
            return Err(SceneError("external world revision did not advance".into()));
        }
        validate_world(&self.engine, &world)?;
        let mut candidate = self.engine.clone();
        candidate.globals = world.globals.clone();
        candidate.locals = world.locals.clone();
        candidate.refresh_offers()?;
        if self.provider.commit(world.revision, &[])? != world {
            return Err(SceneError("provider changed during refresh".into()));
        }
        self.engine = candidate;
        self.committed = world;
        self.replay_supported = false;
        Ok(true)
    }
    pub fn apply(
        &mut self,
        action: &grimm_semantic::scene::SceneAction,
    ) -> SceneResult<grimm_semantic::scene::CueSheet> {
        self.transact(|engine| engine.apply(action))
    }
    pub fn cue_input(
        &mut self,
        input: &grimm_semantic::scene::CueInput,
    ) -> SceneResult<grimm_semantic::scene::CueSheet> {
        self.transact(|engine| engine.cue_input(input))
    }
    pub fn observe(
        &mut self,
        revision: u64,
        time: u64,
    ) -> SceneResult<grimm_semantic::scene::CueSheet> {
        self.transact(|engine| engine.observe(revision, time))
    }
    pub fn restore(
        &mut self,
        save: grimm_semantic::scene::SceneSave,
    ) -> SceneResult<grimm_semantic::scene::CueSheet> {
        if !self.replay_supported {
            return Err(SceneError(
                "external worlds require host snapshot restore (#63)".into(),
            ));
        }
        self.ensure_current()?;
        let mut candidate = self.engine.clone();
        candidate.restore(save)?;
        self.replace_replayed(candidate)
    }
    pub(super) fn replace_replayed(
        &mut self,
        candidate: super::SceneEngine,
    ) -> SceneResult<grimm_semantic::scene::CueSheet> {
        self.ensure_current()?;
        let world = candidate.inspect_world();
        let requested = WorldSnapshot {
            revision: self.committed.revision,
            scope: WorldScope {
                context_id: world.context_id,
                epoch: world.scene_epoch,
            },
            globals: world.globals,
            locals: world.locals,
        };
        let restored = self.provider.restore(self.committed.revision, &requested)?;
        if restored.scope != requested.scope
            || restored.globals != requested.globals
            || restored.locals != requested.locals
            || self.committed.revision.checked_add(1) != Some(restored.revision)
        {
            return Err(SceneError(
                "world provider violated its atomic restore contract".into(),
            ));
        }
        self.committed = restored;
        self.engine = candidate;
        Ok(self.engine.output())
    }
}

impl<P> ProvidedSceneEngine<P> {
    pub fn engine(&self) -> &super::SceneEngine {
        &self.engine
    }
    pub fn world_snapshot(&self) -> WorldSnapshot {
        self.committed.clone()
    }

    pub(super) fn rebind<Q>(&self, provider: Q) -> ProvidedSceneEngine<Q> {
        ProvidedSceneEngine {
            engine: self.engine.clone(),
            provider,
            committed: self.committed.clone(),
            replay_supported: self.replay_supported,
        }
    }
    pub(super) fn replay_supported(&self) -> bool {
        self.replay_supported
    }
}
impl ProvidedSceneEngine<()> {
    /// Attach to an already initialized external world. Does not initialize or
    /// write fields; only validates an immutable projection and a read-only CAS.
    pub fn from_provider<P: WorldStateProvider>(
        story: grimm_semantic::scene::SceneStory,
        provider: &mut P,
    ) -> SceneResult<Self> {
        let mut engine = super::SceneEngine::new(story)?;
        let world = provider.snapshot()?;
        validate_world(&engine, &world)?;
        engine.globals = world.globals.clone();
        engine.locals = world.locals.clone();
        engine.plan =
            engine.materialize(&engine.ctx().cues.clone(), None, super::Times::default())?;
        engine.register();
        if provider.commit(world.revision, &[])? != world {
            return Err(SceneError("provider changed during attachment".into()));
        }
        Ok(Self {
            engine,
            provider: (),
            committed: world,
            replay_supported: false,
        })
    }
    /// Lend storage for this scope only. Each engine operation is a separate
    /// atomic transaction; a later error does not undo earlier accepted inputs.
    pub fn with_provider_access<'p, P: WorldStateProvider, R>(
        &mut self,
        provider: &'p mut P,
        operation: impl FnOnce(&mut ProvidedSceneEngine<&'p mut P>) -> SceneResult<R>,
    ) -> SceneResult<R> {
        let bound = self.rebind(provider);
        let mut scope = ProviderScope {
            target: self,
            bound,
        };
        operation(&mut scope.bound)
    }
}
struct ProviderScope<'a, 'p, P> {
    target: &'a mut ProvidedSceneEngine<()>,
    bound: ProvidedSceneEngine<&'p mut P>,
}
impl<P> Drop for ProviderScope<'_, '_, P> {
    fn drop(&mut self) {
        *self.target = self.bound.rebind(());
    }
}
impl<P: WorldStateProvider + ?Sized> WorldStateProvider for &mut P {
    fn snapshot(&self) -> SceneResult<WorldSnapshot> {
        (**self).snapshot()
    }
    fn commit(&mut self, revision: u64, changes: &[StateMutation]) -> SceneResult<WorldSnapshot> {
        (**self).commit(revision, changes)
    }
    fn restore(&mut self, revision: u64, world: &WorldSnapshot) -> SceneResult<WorldSnapshot> {
        (**self).restore(revision, world)
    }
}
fn validate_world(engine: &super::SceneEngine, world: &WorldSnapshot) -> SceneResult<()> {
    fn schema(expected: &Variables, actual: &Variables) -> bool {
        expected.len() == actual.len()
            && expected
                .iter()
                .all(|(key, value)| actual.get(key).is_some_and(|other| same_type(value, other)))
    }
    if world.scope.context_id != engine.ctx().id
        || world.scope.epoch != engine.clock.epoch
        || !schema(&engine.story.globals, &world.globals)
        || !schema(&engine.ctx().locals, &world.locals)
    {
        return Err(SceneError(
            "world scope, fields or types do not match the narrative schema".into(),
        ));
    }
    Ok(())
}
