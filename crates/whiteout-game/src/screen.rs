//! Fixed camera and a view of cues actually executed by the WHITEOUT host.
use crate::{
    game::FrameSet,
    scheduler::{Generator, Runtime, SceneView},
};
use bevy::{prelude::*, text::FontSize};

#[derive(Component)]
struct SceneRoot;
#[derive(Component)]
struct CountdownText;

pub(crate) fn install(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.025, 0.04, 0.06)))
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                replace_scene
                    .run_if(resource_changed::<SceneView>.or_eager(resource_changed::<Generator>)),
                update_countdown,
            )
                .chain()
                .in_set(FrameSet::Present),
        );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn replace_scene(
    mut commands: Commands,
    runtime: Res<Runtime>,
    view: Res<SceneView>,
    generator: Res<Generator>,
    roots: Query<Entity, With<SceneRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn();
    }
    commands.spawn((SceneRoot, Node {
        width: percent(100), height: percent(100), padding: UiRect::all(px(30)),
        flex_direction: FlexDirection::Column, row_gap: px(14), ..default()
    })).with_children(|parent| {
        parent.spawn((Text::new("WHITEOUT / FIELD STATION"), font(24.0)));
        parent.spawn((Text::new(format!("{} / entry {}", runtime.sheet.node_id, runtime.sheet.epoch)), font(16.0)));
        if let Some(panel) = view.panel() {
            parent.spawn((Node { width: percent(100), height: px(8), ..default() }, BackgroundColor(Color::srgb(panel.accent[0], panel.accent[1], panel.accent[2]))));
            parent.spawn((Text::new(panel.title), font(32.0)));
            parent.spawn((Text::new(panel.body), font(22.0)));
            if panel.countdown && view.failure.is_none() { parent.spawn((CountdownText, Text::new(""), font(24.0))); }
        }
        parent.spawn((Text::new(format!("Generator: {}", generator.status.label())), font(20.0)));
        if let Some(error) = &view.failure {
            parent.spawn((Text::new(error), font(18.0), TextColor(Color::srgb(1.0, 0.35, 0.25))));
        } else if view.actions_open() {
            for (index, edge_id) in runtime.sheet.outgoing_edges.keys().enumerate() {
                parent.spawn((Text::new(format!("[{}] {}", index + 1, crate::content::edge_label(edge_id).expect("validated WHITEOUT edge"))), font(22.0), TextColor(Color::srgb(0.55, 0.85, 0.95))));
            }
        } else { parent.spawn((Text::new("Controls locked while the presentation unfolds."), font(18.0))); }
        parent.spawn((Text::new("Press a number key to act. Release before selecting again.\nClose the window to exit."), font(16.0)));
    });
}

fn font(size: f32) -> TextFont {
    TextFont {
        font_size: FontSize::Px(size),
        ..default()
    }
}

fn update_countdown(runtime: Res<Runtime>, mut labels: Query<&mut Text, With<CountdownText>>) {
    let Some(deadline) = runtime.deadline_ms() else {
        return;
    };
    let text = format!(
        "Time remaining: {}s",
        deadline.saturating_sub(runtime.elapsed_ms).div_ceil(1000)
    );
    for mut label in &mut labels {
        if label.0 != text {
            label.0.clone_from(&text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        game,
        scheduler::{self, CueInstance, HostRequest},
    };
    use bevy::{
        input::{ButtonState, InputPlugin},
        time::TimeUpdateStrategy,
    };
    use std::time::Duration;

    fn texts(app: &mut App) -> Vec<String> {
        let world = app.world_mut();
        world
            .query::<&Text>()
            .iter(world)
            .map(|t| t.0.clone())
            .collect()
    }

    #[test]
    fn full_replacement_gates_countdown_and_removes_all_old_scene_entities_without_gpu() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, game::install, install))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
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
            .filter(|e| e.contains::<ChildOf>())
            .map(|e| e.id())
            .collect();
        scheduler::tests::key(&mut app, ButtonState::Pressed, false, KeyCode::Digit1);
        let world = app.world_mut();
        assert!(world.get_entity(root).is_err());
        for child in old_children {
            assert!(world.get_entity(child).is_err());
        }
        let text = texts(&mut app).join("\n");
        assert!(text.contains("GENERATOR_WARNING"));
        assert!(!text.contains("[1]"));
        assert!(!text.contains("Isolate the damaged generator"));
        assert!(!text.contains("generator_exploded"));
        scheduler::tests::advance(&mut app, 2000);
        assert!(
            texts(&mut app)
                .join("\n")
                .contains("Warning light / unstable")
        );
        scheduler::tests::advance(&mut app, 3000);
        let text = texts(&mut app).join("\n");
        assert!(text.contains("[1] Isolate the damaged generator"));
        assert!(text.contains("Time remaining: 25s"));
        // A duplicate delivery must not restart presentation or rebuild its UI.
        let root = app
            .world_mut()
            .query_filtered::<Entity, With<SceneRoot>>()
            .single(app.world())
            .unwrap();
        app.world_mut()
            .write_message(HostRequest::WakeCue(CueInstance {
                epoch: 1,
                cue_id: "enable_interaction".into(),
            }));
        scheduler::tests::advance(&mut app, 1000);
        assert!(app.world().get_entity(root).is_ok());
        assert!(texts(&mut app).join("\n").contains("Time remaining: 24s"));
        scheduler::tests::advance(&mut app, 24000);
        let text = texts(&mut app).join("\n");
        assert!(text.contains("GENERATOR_FAILED"));
        assert!(text.contains("Stopped / station dark"));
        assert!(!text.contains("Time remaining:"));
        assert!(!text.contains("Isolate the damaged generator"));
        let world = app.world_mut();
        assert_eq!(
            world
                .query_filtered::<Entity, With<SceneRoot>>()
                .iter(world)
                .count(),
            1
        );
        assert_eq!(world.query::<&Camera2d>().iter(world).count(), 1);
    }
}
