use bevy::prelude::*;
use grimm_graph::{GraphEngine, Instruction, StoryGraph};

use crate::{
    content,
    cueables::{self, Catalog},
    gameplay::{Condition, Effect},
    scheduler::{self, Generator, HostRequest, Runtime, SceneView},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FrameSet {
    Collect,
    Execute,
    Present,
}

pub(crate) fn install(app: &mut App) {
    let graph = load_graph().expect("the embedded WHITEOUT bootstrap must be valid");
    app.insert_resource(Runtime::new(graph))
        .init_resource::<SceneView>()
        .init_resource::<Generator>()
        .add_message::<HostRequest>()
        .configure_sets(
            Update,
            (FrameSet::Collect, FrameSet::Execute, FrameSet::Present).chain(),
        )
        .configure_sets(
            Update,
            (FrameSet::Collect, FrameSet::Execute).run_if(crate::stage::assets_ready),
        )
        .add_systems(
            Update,
            (keyboard_input, scheduler::queue_cues).in_set(FrameSet::Collect),
        )
        .add_systems(Update, scheduler::drive.in_set(FrameSet::Execute));
}

fn load_graph() -> Result<GraphEngine, Box<dyn std::error::Error>> {
    let graph: StoryGraph = serde_yaml_ng::from_str(include_str!("../assets/bootstrap.yaml"))?;
    validate_bindings(&graph)?;
    Ok(GraphEngine::new(graph)?)
}

pub(crate) fn validate_bindings(graph: &StoryGraph) -> Result<(), String> {
    validate_with_catalog(graph, &Catalog::default())
}

pub(crate) fn validate_with_catalog(graph: &StoryGraph, catalog: &Catalog) -> Result<(), String> {
    for node in &graph.nodes {
        for cue in &node.cues {
            match &cue.instruction {
                Instruction::Presentation {
                    presentation_id,
                    args,
                    ..
                } if args.is_empty() => {
                    cueables::resolve(presentation_id, catalog)
                        .map_err(|error| format!("{}.{}: {error}", node.id, cue.cue_id))?;
                }
                Instruction::Effect { effect_id, args } => {
                    Effect::parse(effect_id, args)
                        .map_err(|error| format!("{}.{}: {error}", node.id, cue.cue_id))?;
                }
                Instruction::TraverseEdge { .. } => {}
                _ => {
                    return Err(format!(
                        "unsupported WHITEOUT cue '{}.{}'",
                        node.id, cue.cue_id
                    ));
                }
            }
        }
        for (id, edge) in &node.edges {
            if let Some(condition) = &edge.condition {
                Condition::parse(condition)
                    .map_err(|error| format!("{}.{id}: {error}", node.id))?;
            }
            if edge.expose && content::edge_label(id).is_none() {
                return Err(format!("unsupported WHITEOUT edge '{}.{id}'", node.id));
            }
        }
    }
    Ok(())
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Real>>,
    runtime: Res<Runtime>,
    mut requests: MessageWriter<HostRequest>,
) {
    for (index, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3]
        .iter()
        .enumerate()
    {
        if keys.just_pressed(*key) {
            if let Some((edge_id, _)) = runtime.sheet.outgoing_edges.iter().nth(index) {
                requests.write(HostRequest::Selection {
                    edge_id: edge_id.clone(),
                    epoch: runtime.sheet.epoch,
                    observed_ms: runtime.observed_ms(time.elapsed()),
                });
            }
            break;
        }
    }
}
