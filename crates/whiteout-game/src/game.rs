use bevy::prelude::*;
use grimm_graph::{CueSheet, GraphEngine, Instruction, StoryGraph};

use crate::content;

// Graph position lives only in GRIMM. The sheet is a host presentation projection.
#[derive(Resource)]
pub(crate) struct Story(GraphEngine);

#[derive(Resource)]
pub(crate) struct SceneSheet(pub CueSheet);

#[derive(Message, Clone)]
pub(crate) struct TraverseRequest {
    edge_id: String,
    epoch: u64,
}

pub(crate) fn install(app: &mut App) {
    let graph = load_graph().expect("the embedded WHITEOUT bootstrap must be valid");
    app.insert_resource(SceneSheet(graph.current()))
        .insert_resource(Story(graph))
        .add_message::<TraverseRequest>()
        .add_systems(Update, (keyboard_input, apply_traversals).chain());
}

fn load_graph() -> Result<GraphEngine, Box<dyn std::error::Error>> {
    let graph: StoryGraph = serde_yaml_ng::from_str(include_str!("../assets/bootstrap.yaml"))?;
    // This bootstrap supports only immediate static presentation and
    // unconditional edges. Fail visibly rather than pretending to implement #68/#69.
    for node in &graph.nodes {
        for cue in &node.cues {
            match &cue.instruction {
                Instruction::Presentation {
                    presentation_id,
                    duration_ms: None,
                    args,
                } if cue.at_ms == 0
                    && args.is_empty()
                    && content::presentation(presentation_id).is_some() => {}
                _ => {
                    return Err(
                        format!("unsupported bootstrap cue '{}.{}'", node.id, cue.cue_id).into(),
                    );
                }
            }
        }
        for (id, edge) in &node.edges {
            if edge.condition.is_some() || (edge.expose && content::edge_label(id).is_none()) {
                return Err(format!("unsupported bootstrap edge '{}.{id}'", node.id).into());
            }
        }
    }
    Ok(GraphEngine::new(graph)?)
}

fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    sheet: Res<SceneSheet>,
    mut requests: MessageWriter<TraverseRequest>,
) {
    // Capture the epoch with the selection. At most one selection per frame;
    // holding a key does not become a fresh selection after replacement.
    for (index, key) in [KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3]
        .iter()
        .enumerate()
    {
        if keys.just_pressed(*key) {
            if let Some((edge_id, _)) = sheet.0.outgoing_edges.iter().nth(index) {
                requests.write(TraverseRequest {
                    edge_id: edge_id.clone(),
                    epoch: sheet.0.epoch,
                });
            }
            break;
        }
    }
}

pub(crate) fn apply_traversals(
    mut story: ResMut<Story>,
    mut sheet: ResMut<SceneSheet>,
    mut requests: MessageReader<TraverseRequest>,
) {
    for request in requests.read() {
        match story.0.traverse_edge(&request.edge_id, request.epoch) {
            Ok(result) => {
                info!(
                    "WHITEOUT {} epoch={}",
                    result.cue_sheet.node_id, result.cue_sheet.epoch
                );
                sheet.0 = result.cue_sheet;
            }
            Err(error) => warn!("WHITEOUT ignored traversal: {error}"),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use bevy::input::{
        ButtonState, InputPlugin,
        keyboard::{Key, KeyboardInput},
    };

    pub(crate) fn app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, install));
        app.update();
        app
    }

    pub(crate) fn key(app: &mut App, state: ButtonState, repeat: bool, key_code: KeyCode) {
        app.world_mut().write_message(KeyboardInput {
            key_code,
            logical_key: Key::Character("1".into()),
            state,
            text: None,
            repeat,
            window: Entity::PLACEHOLDER,
        });
        app.update();
    }

    #[test]
    fn actual_bevy_keyboard_system_traverses_the_bootstrap_and_ignores_held_keys() {
        let mut app = app();
        let initial = &app.world().resource::<SceneSheet>().0;
        assert_eq!(initial.node_id, "GENERATOR_ROOM");
        assert_eq!(initial.epoch, 0);
        assert_eq!(
            initial.outgoing_edges.keys().collect::<Vec<_>>(),
            ["inspect_panel"]
        );
        assert_eq!(initial.cues[0].cue_id, "room");
        key(&mut app, ButtonState::Pressed, false, KeyCode::Digit1);
        assert_eq!(
            app.world().resource::<SceneSheet>().0.node_id,
            "GENERATOR_WARNING"
        );
        let warning = app.world().resource::<SceneSheet>().0.clone();
        key(&mut app, ButtonState::Pressed, true, KeyCode::Digit1);
        app.update();
        assert_eq!(app.world().resource::<SceneSheet>().0, warning);
        key(&mut app, ButtonState::Released, false, KeyCode::Digit1);
        key(&mut app, ButtonState::Pressed, false, KeyCode::Digit1);
        let decision = &app.world().resource::<SceneSheet>().0;
        assert_eq!(decision.node_id, "GENERATOR_DECISION");
        assert_eq!(decision.epoch, 2);
        assert_eq!(decision.cues[0].cue_id, "decision");
        assert_eq!(
            decision.outgoing_edges.keys().collect::<Vec<_>>(),
            ["restart_demo"]
        );
        assert_eq!(*decision, app.world().resource::<Story>().0.current());
    }

    #[test]
    fn duplicate_invalid_and_stale_requests_cannot_change_a_later_entry() {
        let mut app = app();
        let initial = app.world().resource::<SceneSheet>().0.clone();
        app.world_mut().write_message(TraverseRequest {
            edge_id: "not_an_edge".into(),
            epoch: 0,
        });
        app.update();
        assert_eq!(app.world().resource::<SceneSheet>().0, initial);
        let request = TraverseRequest {
            edge_id: "inspect_panel".into(),
            epoch: 0,
        };
        app.world_mut()
            .write_message_batch([request.clone(), request.clone()]);
        app.update();
        assert_eq!(app.world().resource::<SceneSheet>().0.epoch, 1);
        for (edge_id, epoch) in [("approach_switch", 1), ("restart_demo", 2)] {
            app.world_mut().write_message(TraverseRequest {
                edge_id: edge_id.into(),
                epoch,
            });
            app.update();
        }
        let revisited = app.world().resource::<SceneSheet>().0.clone();
        assert_eq!(revisited.node_id, initial.node_id);
        assert_eq!(revisited.epoch, 3);
        app.world_mut().write_message(request); // valid ID again, obsolete epoch
        app.update();
        assert_eq!(app.world().resource::<SceneSheet>().0, revisited);
    }

    #[test]
    fn hidden_edges_are_not_keyboard_options_but_remain_host_traversable() {
        let mut app = app();
        key(&mut app, ButtonState::Pressed, false, KeyCode::Digit2);
        assert_eq!(app.world().resource::<SceneSheet>().0.epoch, 0);
        assert!(
            !app.world()
                .resource::<SceneSheet>()
                .0
                .outgoing_edges
                .contains_key("maintenance_bypass")
        );
        app.world_mut().write_message(TraverseRequest {
            edge_id: "maintenance_bypass".into(),
            epoch: 0,
        });
        app.update();
        assert_eq!(
            app.world().resource::<SceneSheet>().0.node_id,
            "GENERATOR_DECISION"
        );
    }
}
