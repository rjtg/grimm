//! Real engine transactions against injected atomic storage; no renderer state
//! or mock rule implementation participates in outcome evaluation.
use grimm_engine::scene::{SceneError, SceneResult, state::*};
use grimm_semantic::{
    scene::*,
    story::{Value, Variables},
    world::*,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
const STORY: &str = r#"
schema_version: 2
id: storage_demo
version: 1
title: Storage demo
start: room
globals: {count: 0}
contexts:
  - id: room
    locals: {marker: 1}
    cues: [{cue_id: choices, kind: actions, available_actions: eligible}]
    actions:
      - id: step
        effects:
          - {op: add, var: global.count, value: 1}
          - {op: set, var: local.marker, value: 2}
        outcomes: [{}]
      - id: finish
        effects: [{op: add, var: global.count, value: 1}]
        outcomes: [{goto: end}]
  - id: end
    locals: {marker: 7}
    terminal: true
    cues: []
"#;
#[derive(Debug, Clone)]
struct Storage {
    state: Arc<Mutex<InMemoryWorldState>>,
    reject: Arc<AtomicBool>,
    misreport: Arc<AtomicBool>,
}
impl WorldStateProvider for Storage {
    fn snapshot(&self) -> SceneResult<WorldSnapshot> {
        self.state.lock().unwrap().snapshot()
    }
    fn commit(&mut self, revision: u64, mutations: &[StateMutation]) -> SceneResult<WorldSnapshot> {
        if self.reject.load(Ordering::SeqCst) {
            return Err(SceneError("storage unavailable".into()));
        }
        let mut result = self.state.lock().unwrap().commit(revision, mutations)?;
        if self.misreport.load(Ordering::SeqCst) {
            result.revision = u64::MAX;
        }
        Ok(result)
    }
    fn restore(&mut self, revision: u64, snapshot: &WorldSnapshot) -> SceneResult<WorldSnapshot> {
        if self.reject.load(Ordering::SeqCst) {
            return Err(SceneError("storage unavailable".into()));
        }
        let mut result = self.state.lock().unwrap().restore(revision, snapshot)?;
        if self.misreport.load(Ordering::SeqCst) {
            result.revision = u64::MAX;
        }
        Ok(result)
    }
}
fn engine() -> (ProvidedSceneEngine<Storage>, Storage) {
    let mut handle = None;
    let engine = ProvidedSceneEngine::new(serde_yaml_ng::from_str(STORY).unwrap(), |initial| {
        let storage = Storage {
            state: Arc::new(Mutex::new(InMemoryWorldState::new(
                initial.scope,
                initial.globals,
                initial.locals,
            )?)),
            reject: Arc::new(AtomicBool::new(false)),
            misreport: Arc::new(AtomicBool::new(false)),
        };
        handle = Some(storage.clone());
        Ok(storage)
    })
    .unwrap();
    (engine, handle.unwrap())
}
fn pump(engine: &mut ProvidedSceneEngine<Storage>) {
    for _ in 0..16 {
        let sheet = engine.engine().output();
        let Some(cue) = sheet
            .cues
            .iter()
            .find(|cue| matches!(cue.instruction, Instruction::Actions { .. }))
        else {
            return;
        };
        let before = engine.engine().save();
        engine
            .cue_input(&CueInput {
                revision: sheet.revision,
                cue_id: cue.cue_id.clone(),
                scene_time_ms: 0,
                event: CueEvent::Started,
            })
            .unwrap();
        if engine.engine().save() == before {
            return;
        }
    }
    panic!("unbounded controls");
}
fn action(engine: &mut ProvidedSceneEngine<Storage>, id: &str) -> SceneResult<CueSheet> {
    engine.apply(&SceneAction {
        revision: engine.engine().output().revision,
        action_id: id.into(),
        scene_time_ms: 0,
    })
}
#[test]
fn engine_and_storage_commit_effects_and_scope_atomically_and_restore_by_replay() {
    let (mut engine, storage) = engine();
    pump(&mut engine);
    let saved = engine.engine().save();
    action(&mut engine, "room.step").unwrap();
    assert_eq!(storage.snapshot().unwrap().globals["count"], Value::Int(1));
    assert_eq!(storage.snapshot().unwrap().locals["marker"], Value::Int(2));
    assert_eq!(storage.snapshot().unwrap(), engine.world_snapshot());
    pump(&mut engine);
    action(&mut engine, "room.finish").unwrap();
    assert!(engine.engine().is_terminal());
    assert_eq!(storage.snapshot().unwrap().scope.context_id, "end");
    assert_eq!(storage.snapshot().unwrap().scope.epoch, 1);
    assert_eq!(storage.snapshot().unwrap().locals["marker"], Value::Int(7));
    assert_eq!(storage.snapshot().unwrap().globals["count"], Value::Int(2));
    engine.restore(saved).unwrap();
    assert_eq!(storage.snapshot().unwrap().scope.context_id, "room");
    assert_eq!(storage.snapshot().unwrap().globals["count"], Value::Int(0));
    assert_eq!(storage.snapshot().unwrap().locals["marker"], Value::Int(1));
    assert_eq!(storage.snapshot().unwrap(), engine.world_snapshot());
}
#[test]
fn backend_failure_and_stale_revision_leave_engine_history_and_state_unchanged() {
    let (mut engine, mut storage) = engine();
    pump(&mut engine);
    let original = engine.engine().save();
    let world = storage.snapshot().unwrap();
    storage.reject.store(true, Ordering::SeqCst);
    assert!(action(&mut engine, "room.step").is_err());
    assert_eq!(engine.engine().save(), original);
    assert_eq!(storage.snapshot().unwrap(), world);
    assert!(engine.restore(original.clone()).is_err());
    assert_eq!(engine.engine().save(), original);
    assert_eq!(storage.snapshot().unwrap(), world);
    storage.reject.store(false, Ordering::SeqCst);
    storage
        .commit(
            0,
            &[StateMutation::Add {
                scope: VariableScope::Global,
                name: "count".into(),
                delta: 1,
            }],
        )
        .unwrap();
    assert!(
        action(&mut engine, "room.step")
            .unwrap_err()
            .to_string()
            .contains("stale")
    );
    assert_eq!(engine.engine().save(), original);
    assert_eq!(storage.snapshot().unwrap().globals["count"], Value::Int(1));
}
#[test]
fn typed_storage_rejects_partial_effects_overflow_unknown_variables_and_bad_scope() {
    let mut store = InMemoryWorldState::new(
        WorldScope {
            context_id: "room".into(),
            epoch: 0,
        },
        Variables::from([("count".into(), Value::Int(i64::MAX))]),
        Variables::from([("marker".into(), Value::Int(1))]),
    )
    .unwrap();
    let original = store.snapshot().unwrap();
    for invalid in [
        StateMutation::Add {
            scope: VariableScope::Global,
            name: "count".into(),
            delta: 1,
        },
        StateMutation::Set {
            scope: VariableScope::Local,
            name: "marker".into(),
            value: Value::Bool(true),
        },
        StateMutation::Set {
            scope: VariableScope::Local,
            name: "missing".into(),
            value: Value::Int(1),
        },
        StateMutation::Enter {
            scope: WorldScope {
                context_id: "end".into(),
                epoch: 5,
            },
            locals: Variables::new(),
        },
    ] {
        let changes = [
            StateMutation::Set {
                scope: VariableScope::Local,
                name: "marker".into(),
                value: Value::Int(2),
            },
            invalid,
        ];
        assert!(store.commit(0, &changes).is_err());
        assert_eq!(store.snapshot().unwrap(), original);
    }
    assert_eq!(store.commit(0, &[]).unwrap(), original);
    assert!(store.commit(1, &[]).is_err());
    let changes = [
        StateMutation::Set {
            scope: VariableScope::Global,
            name: "count".into(),
            value: Value::Int(7),
        },
        StateMutation::Enter {
            scope: WorldScope {
                context_id: "end".into(),
                epoch: 1,
            },
            locals: Variables::from([("new".into(), Value::Bool(true))]),
        },
    ];
    let next = store.commit(0, &changes).unwrap();
    assert_eq!(next.revision, 1);
    assert!(!next.locals.contains_key("marker"));
    assert_eq!(next.globals["count"], Value::Int(7));
    let mut invalid = original.clone();
    invalid.globals.insert("count".into(), Value::Bool(true));
    assert!(store.restore(1, &invalid).is_err());
    assert_eq!(store.snapshot().unwrap(), next);
}

#[test]
fn contract_violating_provider_is_detected_but_persisted_writes_cannot_be_undone() {
    for restore in [false, true] {
        let (mut engine, storage) = engine();
        pump(&mut engine);
        let initial = engine.engine().save();
        if restore {
            action(&mut engine, "room.step").unwrap();
        }
        let before = engine.engine().save();
        storage.misreport.store(true, Ordering::SeqCst);
        let error = if restore {
            engine.restore(initial)
        } else {
            action(&mut engine, "room.step")
        }
        .unwrap_err();
        assert!(error.to_string().contains("violated its atomic"), "{error}");
        assert_eq!(engine.engine().save(), before);
        // The backend wrote before misreporting success. Wrapper-side error
        // handling cannot reverse that contract violation or safely retry it.
        assert_eq!(
            storage.snapshot().unwrap().globals["count"],
            Value::Int(if restore { 0 } else { 1 })
        );
        assert_ne!(engine.world_snapshot(), storage.snapshot().unwrap());
        storage.misreport.store(false, Ordering::SeqCst);
        assert!(
            action(&mut engine, "room.step")
                .unwrap_err()
                .to_string()
                .contains("stale")
        );
        assert_eq!(engine.engine().save(), before);
    }
}
