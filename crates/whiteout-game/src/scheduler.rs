//! WHITEOUT's synchronous cue executor. All time/ordering stays in this host.
use std::{collections::BTreeSet, time::Duration};

use bevy::prelude::*;
use grimm_graph::{CueSheet, GraphEngine, Instruction, Parameters};
use grimm_semantic::story::Value;

use crate::content;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CueInstance {
    pub epoch: u64,
    pub cue_id: String,
}

#[derive(Message, Clone)]
pub(crate) enum HostRequest {
    Selection {
        edge_id: String,
        epoch: u64,
        observed_ms: u64,
    },
    // A wake-up carries identity, never an effect payload or permission to run
    // early. It uses the same checks as automatic due-cue delivery.
    WakeCue(CueInstance),
}

#[derive(Resource)]
pub(crate) struct Runtime {
    graph: GraphEngine,
    pub sheet: CueSheet,
    origin: Option<Duration>,
    pub elapsed_ms: u64,
    processed_ms: u64,
    consumed: BTreeSet<CueInstance>,
}

impl Runtime {
    pub fn new(graph: GraphEngine) -> Self {
        Self {
            sheet: graph.current(),
            graph,
            origin: None,
            elapsed_ms: 0,
            processed_ms: 0,
            consumed: BTreeSet::new(),
        }
    }

    pub fn observed_ms(&self, now: Duration) -> u64 {
        u64::try_from(now.saturating_sub(self.origin.unwrap_or(now)).as_millis())
            .unwrap_or(u64::MAX)
    }

    pub fn deadline_ms(&self) -> Option<u64> {
        self.sheet
            .cues
            .iter()
            .filter_map(|cue| match cue.instruction {
                Instruction::TraverseEdge { .. } => Some(cue.at_ms),
                _ => None,
            })
            .min()
    }

    fn instance(&self, index: usize) -> CueInstance {
        CueInstance {
            epoch: self.sheet.epoch,
            cue_id: self.sheet.cues[index].cue_id.clone(),
        }
    }

    fn due_at(&self, elapsed_ms: u64) -> Vec<Event> {
        self.sheet
            .cues
            .iter()
            .enumerate()
            .filter(|(i, cue)| {
                cue.at_ms <= elapsed_ms && !self.consumed.contains(&self.instance(*i))
            })
            .map(|(index, cue)| Event {
                at_ms: cue.at_ms,
                kind: EventKind::Cue(index),
                epoch: self.sheet.epoch,
            })
            .collect()
    }

    fn traverse(
        &mut self,
        edge: &str,
        epoch: u64,
        now: Duration,
        view: &mut SceneView,
    ) -> Result<(), String> {
        let result = self
            .graph
            .traverse_edge(edge, epoch)
            .map_err(|e| e.to_string())?;
        info!(
            "WHITEOUT {} epoch={}",
            result.cue_sheet.node_id, result.cue_sheet.epoch
        );
        self.sheet = result.cue_sheet;
        self.origin = Some(now);
        self.elapsed_ms = 0;
        self.processed_ms = 0;
        self.consumed.clear();
        *view = SceneView::default();
        Ok(())
    }

    fn execute(
        &mut self,
        index: usize,
        now: Duration,
        view: &mut ResMut<SceneView>,
        generator: &mut ResMut<Generator>,
    ) -> Result<bool, String> {
        let instance = self.instance(index);
        if self.consumed.contains(&instance) {
            return Ok(false);
        }
        let cue = &self.sheet.cues[index];
        match &cue.instruction {
            Instruction::Presentation {
                presentation_id,
                duration_ms,
                ..
            } => {
                content::presentation(presentation_id)
                    .ok_or_else(|| format!("unknown presentation '{presentation_id}'"))?;
                view.presentation = Some(Presented {
                    instance: instance.clone(),
                    id: presentation_id.clone(),
                    duration_ms: *duration_ms,
                });
            }
            Instruction::Effect { effect_id, args } => generator.apply(effect_id, args)?,
            Instruction::TraverseEdge { edge_id } => {
                let edge_id = edge_id.clone();
                self.traverse(&edge_id, instance.epoch, now, view)?;
                return Ok(true);
            }
        }
        self.consumed.insert(instance);
        Ok(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Presented {
    pub instance: CueInstance,
    pub id: String,
    pub duration_ms: Option<u64>,
}

#[derive(Resource, Default)]
pub(crate) struct SceneView {
    pub presentation: Option<Presented>,
    pub failure: Option<String>,
}
impl SceneView {
    pub fn panel(&self) -> Option<content::Panel> {
        self.presentation
            .as_ref()
            .and_then(|p| content::presentation(&p.id))
    }
    pub fn actions_open(&self) -> bool {
        self.failure.is_none() && self.panel().is_some_and(|panel| panel.actions_open)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GeneratorStatus {
    #[default]
    Running,
    Unstable,
    Isolated,
    Stopped,
}
impl GeneratorStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Running => "Running",
            Self::Unstable => "Warning light / unstable",
            Self::Isolated => "Safely isolated",
            Self::Stopped => "Stopped / station dark",
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct Generator {
    pub status: GeneratorStatus,
    applications: u64,
}
impl Generator {
    fn apply(&mut self, id: &str, args: &Parameters) -> Result<(), String> {
        let status = match (id, args.len(), args.get("status")) {
            ("set_generator_status", 1, Some(Value::Text(status))) => match status.as_str() {
                "running" => GeneratorStatus::Running,
                "unstable" => GeneratorStatus::Unstable,
                "isolated" => GeneratorStatus::Isolated,
                "stopped" => GeneratorStatus::Stopped,
                _ => return Err(format!("unknown generator status '{status}'")),
            },
            _ => {
                return Err(format!(
                    "invalid effect '{id}': expected one text argument 'status'"
                ));
            }
        };
        // Preflight before either field changes. No automatic retry on failure.
        let applications = self
            .applications
            .checked_add(1)
            .ok_or("generator effect counter overflow")?;
        self.status = status;
        self.applications = applications;
        Ok(())
    }
}

// Authored order breaks cue ties; cues beat selections at equality. Selections
// with equal timestamps use edge-ID order, never message/system arrival order.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    Cue(usize),
    Selection(String),
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    at_ms: u64,
    kind: EventKind,
    epoch: u64,
}

pub(crate) fn queue_cues(
    time: Res<Time<Real>>,
    runtime: Res<Runtime>,
    mut requests: MessageWriter<HostRequest>,
) {
    for event in runtime.due_at(runtime.observed_ms(time.elapsed())) {
        let EventKind::Cue(index) = event.kind else {
            unreachable!()
        };
        requests.write(HostRequest::WakeCue(runtime.instance(index)));
    }
}

pub(crate) fn drive(
    time: Res<Time<Real>>,
    mut runtime: ResMut<Runtime>,
    mut view: ResMut<SceneView>,
    mut generator: ResMut<Generator>,
    mut requests: MessageReader<HostRequest>,
) {
    let now = time.elapsed();
    runtime.origin.get_or_insert(now);
    runtime.elapsed_ms = runtime.observed_ms(now);
    let mut events = Vec::new();
    for request in requests.read() {
        match request {
            HostRequest::Selection {
                edge_id,
                epoch,
                observed_ms,
            } if *epoch == runtime.sheet.epoch
                && *observed_ms >= runtime.processed_ms
                && *observed_ms <= runtime.elapsed_ms =>
            {
                events.push(Event {
                    at_ms: *observed_ms,
                    kind: EventKind::Selection(edge_id.clone()),
                    epoch: *epoch,
                });
            }
            HostRequest::WakeCue(instance) if instance.epoch == runtime.sheet.epoch => {
                if let Some(index) = runtime.sheet.cues.iter().position(|cue| {
                    cue.cue_id == instance.cue_id && cue.at_ms <= runtime.elapsed_ms
                }) {
                    events.push(Event {
                        at_ms: runtime.sheet.cues[index].at_ms,
                        kind: EventKind::Cue(index),
                        epoch: instance.epoch,
                    });
                } else {
                    warn!("WHITEOUT ignored unknown or premature cue wake: {instance:?}");
                }
            }
            _ => warn!("WHITEOUT ignored stale or invalid host request"),
        }
    }
    if view.failure.is_some() {
        return;
    }
    events.sort();
    for event in events {
        let instance = match &event.kind {
            EventKind::Cue(index) => runtime.instance(*index),
            EventKind::Selection(edge) => CueInstance {
                epoch: event.epoch,
                cue_id: format!("selection.{edge}"),
            },
        };
        let result = match event.kind {
            EventKind::Cue(index) => runtime.execute(index, now, &mut view, &mut generator),
            EventKind::Selection(edge) => {
                if !view.actions_open() || !runtime.sheet.outgoing_edges.contains_key(&edge) {
                    continue;
                }
                runtime
                    .traverse(&edge, event.epoch, now, &mut view)
                    .map(|()| true)
            }
        };
        match result {
            Ok(false) => {}
            Ok(true) => {
                // REPLACE drops every remaining event of the old entry. Start
                // destination T=0 presentation/effects in this render frame.
                // At most one traversal per frame: an immediate traversal cue
                // (and anything after it) waits for the next pass, avoiding
                // unbounded zero-time graph cycles without adding a scene FSM.
                for event in runtime.due_at(0) {
                    let EventKind::Cue(index) = event.kind else {
                        unreachable!()
                    };
                    if matches!(
                        runtime.sheet.cues[index].instruction,
                        Instruction::TraverseEdge { .. }
                    ) {
                        break;
                    }
                    if let Err(error) = runtime.execute(index, now, &mut view, &mut generator) {
                        fail(&mut view, runtime.instance(index), error);
                        break;
                    }
                }
                break;
            }
            Err(error) => {
                fail(&mut view, instance, error);
                break;
            }
        }
    }
    runtime.processed_ms = runtime.elapsed_ms;
}

fn fail(view: &mut SceneView, instance: CueInstance, error: String) {
    let message = format!(
        "Cue {}/{} failed: {error}. Execution stopped; close and restart the demo.",
        instance.epoch, instance.cue_id
    );
    error!("{message}");
    view.failure = Some(message);
}

#[cfg(test)]
pub(crate) mod tests;
