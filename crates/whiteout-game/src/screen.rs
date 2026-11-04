//! Fixed camera and disposable presentation entities for the current full sheet.
use bevy::{prelude::*, text::FontSize};
use grimm_graph::Instruction;

use crate::{
    content,
    game::{self, SceneSheet},
};

#[derive(Component)]
struct SceneRoot;

pub(crate) fn install(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.025, 0.04, 0.06)))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            replace_scene
                .after(game::apply_traversals)
                .run_if(resource_changed::<SceneSheet>),
        );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn replace_scene(
    mut commands: Commands,
    sheet: Res<SceneSheet>,
    roots: Query<Entity, With<SceneRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn();
    }
    commands.spawn((SceneRoot, Node {
        width: percent(100), height: percent(100), padding: UiRect::all(px(36)),
        flex_direction: FlexDirection::Column, row_gap: px(20), ..default()
    })).with_children(|parent| {
        parent.spawn((Text::new("WHITEOUT / FIELD STATION"), TextFont { font_size: FontSize::Px(24.0), ..default() }));
        parent.spawn((Text::new(format!("{} · entry {}", sheet.0.node_id, sheet.0.epoch)), TextFont { font_size: FontSize::Px(16.0), ..default() }));
        for cue in &sheet.0.cues {
            if let Instruction::Presentation { presentation_id, .. } = &cue.instruction {
                let panel = content::presentation(presentation_id).expect("validated WHITEOUT binding");
                parent.spawn((Node { width: percent(100), height: px(8), ..default() }, BackgroundColor(Color::srgb(panel.accent[0], panel.accent[1], panel.accent[2]))));
                parent.spawn((Text::new(panel.title), TextFont { font_size: FontSize::Px(36.0), ..default() }));
                parent.spawn((Text::new(panel.body), TextFont { font_size: FontSize::Px(24.0), ..default() }));
            }
        }
        for (index, edge_id) in sheet.0.outgoing_edges.keys().enumerate() {
            parent.spawn((Text::new(format!("[{}] {}", index + 1, content::edge_label(edge_id).expect("validated WHITEOUT edge"))), TextFont { font_size: FontSize::Px(24.0), ..default() }, TextColor(Color::srgb(0.55, 0.85, 0.95))));
        }
        parent.spawn((Text::new("Press a number key to act. Release before selecting again.\nClose the window to exit. No countdown in this bootstrap."), TextFont { font_size: FontSize::Px(16.0), ..default() }));
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::input::ButtonState;

    #[test]
    fn fixed_camera_and_all_scene_entities_follow_full_sheet_replacement_without_a_gpu() {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            bevy::input::InputPlugin,
            game::install,
            install,
        ));
        app.update();
        let world = app.world_mut();
        assert_eq!(world.query::<&Camera2d>().iter(world).count(), 1);
        let root = world
            .query_filtered::<Entity, With<SceneRoot>>()
            .single(world)
            .unwrap();
        let old_children: Vec<_> = world
            .query::<EntityRef>()
            .iter(world)
            .filter(|entity| entity.contains::<ChildOf>())
            .map(|entity| entity.id())
            .collect();
        let texts: Vec<_> = world
            .query::<&Text>()
            .iter(world)
            .map(|text| text.0.clone())
            .collect();
        assert!(texts.iter().any(|text| text.contains("GENERATOR_ROOM")));
        assert!(
            texts
                .iter()
                .any(|text| text.contains("Inspect the control panel"))
        );
        assert!(!texts.iter().any(|text| text.contains("maintenance_bypass")));
        game::tests::key(&mut app, ButtonState::Pressed, false, KeyCode::Digit1);
        let world = app.world_mut();
        assert!(world.get_entity(root).is_err());
        for child in old_children {
            assert!(world.get_entity(child).is_err());
        }
        assert_eq!(
            world
                .query_filtered::<Entity, With<SceneRoot>>()
                .iter(world)
                .count(),
            1
        );
        assert_eq!(world.query::<&Camera2d>().iter(world).count(), 1);
        let texts: Vec<_> = world
            .query::<&Text>()
            .iter(world)
            .map(|text| text.0.clone())
            .collect();
        assert!(texts.iter().any(|text| text.contains("GENERATOR_WARNING")));
        assert!(
            texts
                .iter()
                .any(|text| text.contains("Move to the breaker"))
        );
        assert!(
            !texts.iter().any(|text| text.contains("GENERATOR_ROOM")
                || text.contains("Inspect the control panel"))
        );
    }
}
