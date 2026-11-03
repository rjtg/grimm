//! Headless stand-in for mapped resource/component fields. No shared handle,
//! world copy inside the runtime, Bevy dependency or raw entity ID binding.
use grimm_engine::scene::{
    SceneError, SceneResult,
    lifecycle::LifecycleEngine,
    state::{ProvidedSceneEngine, WorldStateProvider},
};
use grimm_semantic::{lifecycle::*, scene::*, story::Value, world::*};
const STORY: &str = r#"
schema_version: 2
id: mapped
version: 1
title: Mapped fields
start: room
globals: {enabled: false, count: 0, crew_ready: false}
contexts:
- id: room
  locals: {marker: 1}
  cues:
  - {cue_id: timer, at_ms: 0, kind: timer, set_deadline_ms: 1000, visible: false}
  - {cue_id: ambient, at_ms: 0, kind: narrative, narrative_id: wind, duration_ms: 900}
  - {cue_id: choices, at_ms: 0, kind: actions, available_actions: eligible}
  actions:
  - id: work
    when: {op: all, tests: [{op: eq, left: {var: global.enabled}, right: true}, {op: eq, left: {var: global.crew_ready}, right: true}]}
    duration_ms: 5
    effects: [{op: add, var: local.marker, value: 1}, {op: add, var: global.count, value: 1}]
    outcomes:
    - cues:
      - {cue_id: result, kind: narrative, narrative_id: worked, duration_ms: 100}
      - {cue_id: release, at_ms: 0, kind: actions, available_actions: [work, leave]}
  - id: inspect
    outcomes: [{}]
  - id: leave
    when: {op: ge, left: {var: global.count}, right: 1}
    effects: [{op: set, var: global.enabled, value: false}, {op: add, var: global.count, value: 10}]
    outcomes: [{goto: hall}]
  timeout: {goto: end}
- id: hall
  locals: {marker: 7}
  cues: [{cue_id: choices, kind: actions, available_actions: eligible}]
  actions: [{id: finish, outcomes: [{goto: end}]}]
- id: end
  locals: {marker: 11}
  terminal: true
  cues: []
"#;
const CONFIG: &str = "schema_version: 1\nid: mapped_host\nversion: 1\nstory_id: mapped\nstory_version: 1\nscenes: {room: demo.room, hall: demo.hall, end: demo.end}\n";
#[derive(Debug, Clone, PartialEq, Eq)]
struct Actor {
    logical_key: String,
    transient_id: u64,
    ready: bool,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct World {
    revision: u64,
    scope: WorldScope,
    enabled: bool,
    count: i64,
    marker: i64,
    actors: Vec<Actor>,
    reject: bool,
    conflict: bool,
    wrong_type: bool,
}
impl World {
    fn new() -> Self {
        Self {
            revision: 4,
            scope: WorldScope {
                context_id: "room".into(),
                epoch: 0,
            },
            enabled: false,
            count: 2,
            marker: 3,
            actors: vec![Actor {
                logical_key: "crew.jonas".into(),
                transient_id: 99,
                ready: true,
            }],
            reject: false,
            conflict: false,
            wrong_type: false,
        }
    }
    fn host_change(&mut self, enabled: bool, count: i64) {
        self.enabled = enabled;
        self.count = count;
        self.revision += 1;
    }
}
#[derive(Debug)]
struct Fields<'a>(&'a mut World);
fn error(message: &str) -> SceneError {
    SceneError(message.into())
}
impl Fields<'_> {
    fn actor(world: &World) -> SceneResult<&Actor> {
        let mut matched = world
            .actors
            .iter()
            .filter(|a| a.logical_key == "crew.jonas");
        let actor = matched
            .next()
            .ok_or_else(|| error("missing logical crew.jonas component"))?;
        if matched.next().is_some() {
            return Err(error("ambiguous logical crew.jonas component"));
        }
        Ok(actor)
    }
}
impl WorldStateProvider for Fields<'_> {
    fn snapshot(&self) -> SceneResult<WorldSnapshot> {
        let actor = Self::actor(self.0)?;
        Ok(WorldSnapshot {
            revision: self.0.revision,
            scope: self.0.scope.clone(),
            globals: [
                ("enabled".into(), Value::Bool(self.0.enabled)),
                (
                    "count".into(),
                    if self.0.wrong_type {
                        Value::Text("wrong binding".into())
                    } else {
                        Value::Int(self.0.count)
                    },
                ),
                ("crew_ready".into(), Value::Bool(actor.ready)),
            ]
            .into(),
            locals: [("marker".into(), Value::Int(self.0.marker))].into(),
        })
    }
    fn commit(&mut self, expected: u64, mutations: &[StateMutation]) -> SceneResult<WorldSnapshot> {
        if self.0.conflict {
            self.0.conflict = false;
            self.0.host_change(self.0.enabled, self.0.count + 1);
        }
        if self.0.revision != expected {
            return Err(error("stale CAS"));
        }
        Self::actor(self.0)?;
        if self.0.reject {
            return Err(error("backend rejected transaction"));
        }
        if mutations.is_empty() {
            return self.snapshot();
        }
        // Scratch fields are discarded on any preflight failure. The one
        // publication below is performed under this exclusive host borrow.
        let mut candidate = self.0.clone();
        let mut entered = false;
        for mutation in mutations {
            match mutation {
                StateMutation::Set {
                    scope: VariableScope::Global,
                    name,
                    value,
                } => match (name.as_str(), value) {
                    ("enabled", Value::Bool(v)) => candidate.enabled = *v,
                    ("count", Value::Int(v)) => candidate.count = *v,
                    ("crew_ready", Value::Bool(v)) => {
                        candidate
                            .actors
                            .iter_mut()
                            .find(|a| a.logical_key == "crew.jonas")
                            .unwrap()
                            .ready = *v
                    }
                    _ => return Err(error("unknown field or wrong type")),
                },
                StateMutation::Set {
                    scope: VariableScope::Local,
                    name,
                    value: Value::Int(v),
                } if name == "marker" => candidate.marker = *v,
                StateMutation::Add {
                    scope: VariableScope::Global,
                    name,
                    delta,
                } if name == "count" => {
                    candidate.count = candidate
                        .count
                        .checked_add(*delta)
                        .ok_or_else(|| error("overflow"))?
                }
                StateMutation::Enter { scope, locals }
                    if !entered
                        && Some(scope.epoch) == candidate.scope.epoch.checked_add(1)
                        && !scope.context_id.is_empty() =>
                {
                    let Some(Value::Int(marker)) = locals.get("marker") else {
                        return Err(error("missing or mistyped local"));
                    };
                    if locals.len() != 1 {
                        return Err(error("unknown local"));
                    }
                    entered = true;
                    candidate.marker = *marker;
                    candidate.scope = scope.clone();
                }
                _ => return Err(error("invalid field, type or scope")),
            }
        }
        candidate.revision = candidate
            .revision
            .checked_add(1)
            .ok_or_else(|| error("revision overflow"))?;
        *self.0 = candidate;
        self.snapshot()
    }
    fn restore(&mut self, _: u64, _: &WorldSnapshot) -> SceneResult<WorldSnapshot> {
        Err(error("host snapshot restoration is #63"))
    }
}
fn new(world: &mut World) -> ProvidedSceneEngine<()> {
    ProvidedSceneEngine::from_provider(serde_yaml_ng::from_str(STORY).unwrap(), &mut Fields(world))
        .unwrap()
}
fn pump<P: WorldStateProvider>(engine: &mut ProvidedSceneEngine<P>, time: u64) {
    let mut seen = std::collections::BTreeSet::new();
    for _ in 0..32 {
        let sheet = engine.engine().output();
        let Some(cue) = sheet
            .cues
            .iter()
            .find(|c| c.at_ms <= time && c.after_cue.is_none() && !seen.contains(&c.cue_id))
        else {
            return;
        };
        seen.insert(cue.cue_id.clone());
        engine
            .cue_input(&CueInput {
                revision: sheet.revision,
                cue_id: cue.cue_id.clone(),
                scene_time_ms: time,
                event: CueEvent::Started,
            })
            .unwrap();
    }
    panic!("unbounded cues");
}
fn act<P: WorldStateProvider>(
    engine: &mut ProvidedSceneEngine<P>,
    id: &str,
    time: u64,
) -> SceneResult<CueSheet> {
    engine.apply(&SceneAction {
        revision: engine.engine().output().revision,
        action_id: id.into(),
        scene_time_ms: time,
    })
}
#[test]
fn scoped_mapping_reads_external_values_and_refreshes_offers_without_media_or_timer_replay() {
    let mut world = World::new();
    let mut engine = new(&mut world);
    engine
        .with_provider_access(&mut Fields(&mut world), |e| {
            pump(e, 0);
            Ok(())
        })
        .unwrap();
    let old_revision = engine.engine().output().revision;
    assert!(!engine.engine().output().cues.iter().any(|c|matches!(&c.instruction,Instruction::Actions{available_actions} if available_actions.contains(&"room.work".into()))));
    let deadline = engine.engine().output().clock.deadline_ms;
    world.host_change(true, 9);
    world.actors[0].transient_id = 1001; // binding follows logical identity
    engine
        .with_provider_access(&mut Fields(&mut world), |e| {
            assert!(act(e, "room.leave", 0).is_err()); // explicit synchronization, no silent retry
            assert!(e.refresh()?);
            pump(e, 0);
            assert!(e.engine().output().revision > old_revision);
            assert!(
                e.apply(&SceneAction {
                    revision: old_revision,
                    action_id: "room.leave".into(),
                    scene_time_ms: 0
                })
                .is_err()
            );
            act(e, "room.work", 0)?;
            Ok(())
        })
        .unwrap();
    assert_eq!(world.count, 10);
    assert_eq!(world.marker, 4);
    assert_eq!(world.revision, 6);
    assert_eq!(engine.engine().output().clock.deadline_ms, deadline);
    assert_eq!(engine.engine().action_history().len(), 1);
    assert_eq!(
        engine
            .engine()
            .output()
            .cues
            .iter()
            .filter(|c| c.cue_id == "room.ambient")
            .count(),
        1
    );
    world.host_change(false, world.count);
    engine
        .with_provider_access(&mut Fields(&mut world), |e| {
            assert!(e.refresh()?);
            assert!(!e.refresh()?);
            assert!(!e.engine().output().cues.iter().any(|c|matches!(&c.instruction, Instruction::Actions{available_actions} if available_actions.contains(&"room.inspect".into()))));
            pump(e, 0);
            assert!(act(e, "room.leave", 0).is_err());
            pump(e, 5);
            act(e, "room.leave", 5)?;
            Ok(())
        })
        .unwrap();
    assert_eq!(world.scope.context_id, "hall");
    assert_eq!(world.scope.epoch, 1);
    assert_eq!(world.marker, 7);
    assert_eq!(world.count, 20);
    assert_eq!(engine.engine().action_history().len(), 2);
}
#[test]
fn read_failures_schema_and_scope_errors_never_publish_a_projection() {
    let mut world = World::new();
    let mut engine = new(&mut world);
    let before = engine.engine().save();
    for failure in 0..4 {
        let mut invalid = world.clone();
        invalid.revision += 1;
        match failure {
            0 => invalid.actors.clear(),
            1 => invalid.actors.push(invalid.actors[0].clone()),
            2 => invalid.wrong_type = true,
            _ => invalid.scope.epoch += 1,
        }
        assert!(
            engine
                .with_provider_access(&mut Fields(&mut invalid), |e| e.refresh())
                .is_err()
        );
        assert_eq!(engine.engine().save(), before);
        assert!(
            ProvidedSceneEngine::from_provider(
                serde_yaml_ng::from_str(STORY).unwrap(),
                &mut Fields(&mut invalid)
            )
            .is_err()
        );
    }
    world.enabled = true; // unauthorized value change without a revision
    assert!(
        engine
            .with_provider_access(&mut Fields(&mut world), |e| e.refresh())
            .is_err()
    );
    assert_eq!(engine.engine().save(), before);
}
#[test]
fn overflow_rejected_effects_and_cas_conflict_publish_no_accepted_narrative_event() {
    for failure in 0..3 {
        let mut world = World::new();
        world.enabled = true;
        if failure == 0 {
            world.count = i64::MAX;
        }
        let mut engine = new(&mut world);
        engine
            .with_provider_access(&mut Fields(&mut world), |e| {
                pump(e, 0);
                Ok(())
            })
            .unwrap();
        let before = engine.engine().save();
        let marker = world.marker;
        let count = world.count;
        if failure == 1 {
            world.reject = true;
        }
        if failure == 2 {
            world.conflict = true;
        }
        assert!(
            engine
                .with_provider_access(&mut Fields(&mut world), |e| act(e, "room.work", 0))
                .is_err()
        );
        assert_eq!(engine.engine().save(), before);
        assert_eq!(world.marker, marker);
        assert_eq!(world.count, count + if failure == 2 { 1 } else { 0 });
    }
}
#[test]
fn provider_preflight_and_scoped_lifecycle_keep_destination_initialization_atomic() {
    let mut world = World::new();
    let before = world.clone();
    let changes = [
        StateMutation::Set {
            scope: VariableScope::Global,
            name: "count".into(),
            value: Value::Int(77),
        },
        StateMutation::Set {
            scope: VariableScope::Local,
            name: "marker".into(),
            value: Value::Bool(true),
        },
    ];
    assert!(Fields(&mut world).commit(4, &changes).is_err());
    assert_eq!(world, before);
    let mut runtime = LifecycleEngine::from_provider(
        serde_yaml_ng::from_str(STORY).unwrap(),
        serde_yaml_ng::from_str(CONFIG).unwrap(),
        &mut Fields(&mut world),
    )
    .unwrap();
    runtime
        .with_provider_access(&mut Fields(&mut world), |e| {
            for event in [SceneEvent::Ready, SceneEvent::Begin] {
                let s = e.output();
                e.scene_input(&SceneFeedback {
                    revision: s.plan.revision,
                    scene_id: s.scene.scene_id,
                    epoch: s.scene.epoch,
                    event,
                })?;
            }
            let s = e.output();
            e.cue_input(&CueInput {
                revision: s.plan.revision,
                cue_id: "room.choices".into(),
                event: CueEvent::Started,
                scene_time_ms: 0,
            })?;
            Ok(())
        })
        .unwrap();
    world.host_change(true, 4);
    runtime
        .with_provider_access(&mut Fields(&mut world), |e| {
            e.refresh()?;
            let s = e.output();
            let id = s
                .plan
                .cues
                .iter()
                .find(|c| {
                    matches!(c.instruction, Instruction::Actions { .. })
                        && c.cue_id.contains("choices")
                })
                .unwrap()
                .cue_id
                .clone();
            e.cue_input(&CueInput {
                revision: s.plan.revision,
                cue_id: id,
                event: CueEvent::Started,
                scene_time_ms: 0,
            })?;
            let s = e.output();
            e.apply(&SceneAction {
                revision: s.plan.revision,
                action_id: "room.leave".into(),
                scene_time_ms: 0,
            })?;
            Ok(())
        })
        .unwrap();
    assert_eq!(world.count, 14);
    assert_eq!(world.marker, 7);
    assert_eq!(runtime.phase(), ScenePhase::Ending);
    assert_eq!(runtime.world_snapshot().scope, world.scope);
    runtime
        .with_provider_access(&mut Fields(&mut world), |e| {
            let s = e.output();
            e.scene_input(&SceneFeedback {
                revision: s.plan.revision,
                scene_id: s.scene.scene_id,
                epoch: s.scene.epoch,
                event: SceneEvent::End,
            })?;
            assert!(e.checkpoint().is_err());
            Ok(())
        })
        .unwrap();
}

#[test]
fn accepted_inputs_survive_scope_error_and_unwind_without_duplicate_effects() {
    let mut world = World::new();
    world.enabled = true;
    let mut engine = new(&mut world);
    let result: SceneResult<()> = engine.with_provider_access(&mut Fields(&mut world), |e| {
        pump(e, 0);
        act(e, "room.work", 0)?;
        Err(error("later host failure"))
    });
    assert!(result.is_err());
    assert_eq!(world.count, 3);
    assert_eq!(engine.engine().action_history().len(), 1);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        engine
            .with_provider_access(&mut Fields(&mut world), |e| -> SceneResult<()> {
                pump(e, 5);
                act(e, "room.work", 5)?;
                panic!("host unwound after accepted input")
            })
            .unwrap();
    }));
    assert!(result.is_err());
    assert_eq!(world.count, 4);
    assert_eq!(engine.engine().action_history().len(), 2);
    assert_eq!(engine.world_snapshot().revision, world.revision);
}
#[test]
fn rejected_transition_preserves_source_fields_and_retries_at_the_same_revision_once() {
    let mut world = World::new();
    world.enabled = true;
    let mut engine = new(&mut world);
    engine
        .with_provider_access(&mut Fields(&mut world), |e| {
            pump(e, 0);
            Ok(())
        })
        .unwrap();
    let before = engine.engine().save();
    world.reject = true;
    assert!(
        engine
            .with_provider_access(&mut Fields(&mut world), |e| act(e, "room.leave", 0))
            .is_err()
    );
    assert_eq!(engine.engine().save(), before);
    assert_eq!(world.count, 2);
    assert_eq!(world.marker, 3);
    assert_eq!(world.scope.context_id, "room");
    world.reject = false;
    engine
        .with_provider_access(&mut Fields(&mut world), |e| {
            act(e, "room.leave", 0)?;
            assert!(
                e.apply(&SceneAction {
                    revision: before.revision,
                    action_id: "room.leave".into(),
                    scene_time_ms: 0
                })
                .is_err()
            );
            Ok(())
        })
        .unwrap();
    assert!(!world.enabled);
    assert_eq!(world.count, 12);
    assert_eq!(world.marker, 7);
    assert_eq!(world.scope.epoch, 1);
    assert_eq!(engine.engine().action_history().len(), 1);
}
#[test]
fn renderer_consumes_refreshed_full_action_set_and_retains_live_media() {
    use grimm_adapters::cues as adapter;
    use grimm_cli_renderer::cues::CuePlayer;
    let mut world = World::new();
    let mut engine = new(&mut world);
    let mut player = CuePlayer::default();
    engine
        .with_provider_access(&mut Fields(&mut world), |e| {
            player.replace(adapter::present(&e.engine().output()));
            for _ in 0..16 {
                let Some(cue) = player.due().into_iter().next() else {
                    break;
                };
                let input = player.started(&cue.cue_id).unwrap();
                e.cue_input(&adapter::feedback(input))?;
                if !matches!(
                    cue.instruction,
                    grimm_presentation::cues::Instruction::Narrative { .. }
                ) {
                    e.cue_input(&adapter::feedback(player.completed(&cue.cue_id).unwrap()))?;
                }
            }
            Ok(())
        })
        .unwrap();
    assert!(!player.actions().contains(&"room.work".into()));
    assert_eq!(player.running_narratives(), ["room.ambient"]);
    world.host_change(true, world.count);
    engine
        .with_provider_access(&mut Fields(&mut world), |e| {
            e.refresh()?;
            player.replace(adapter::present(&e.engine().output()));
            assert!(player.actions().is_empty());
            for cue in player.due() {
                let input = player.started(&cue.cue_id).unwrap();
                e.cue_input(&adapter::feedback(input))?;
            }
            Ok(())
        })
        .unwrap();
    assert!(player.actions().contains(&"room.work".into()));
    assert_eq!(player.running_narratives(), ["room.ambient"]);
    assert_eq!(player.clock().unwrap().deadline_ms, Some(1000));
}
