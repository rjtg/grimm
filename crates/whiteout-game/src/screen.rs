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
                replace_scene.run_if(
                    resource_changed::<SceneView>
                        .or_eager(resource_changed::<Generator>)
                        .or_eager(resource_changed::<crate::art::GraphicMode>),
                ),
                update_countdown,
            )
                .chain()
                .in_set(FrameSet::Present),
        );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, crate::cueables::Object::Camera));
}

fn replace_scene(
    mut commands: Commands,
    runtime: Res<Runtime>,
    view: Res<SceneView>,
    generator: Res<Generator>,
    mode: Res<crate::art::GraphicMode>,
    paintings: Option<Res<crate::art::Paintings>>,
    roots: Query<Entity, With<SceneRoot>>,
) {
    for root in &roots {
        commands.entity(root).despawn();
    }
    let canonical = view
        .presentation
        .as_ref()
        .is_some_and(|p| p.id.starts_with("canon."));
    commands.spawn((SceneRoot, Node {
        width: percent(100), position_type:PositionType::Absolute, bottom:px(0), padding: UiRect::all(px(16)),
        flex_direction: FlexDirection::Column, row_gap: px(7), ..default()
    },BackgroundColor(Color::srgba(0.025,0.04,0.06,0.92)))).with_children(|parent| {
        parent.spawn((Text::new("WHITEOUT / FIELD STATION"), font(20.0)));
        if paintings.as_ref().is_some_and(|p|!p.ready) && view.failure.is_none() { parent.spawn((Text::new("Loading illustrations…"),font(19.0))); }
        if !canonical {parent.spawn((Text::new(format!("{} / entry {}", runtime.sheet.node_id, runtime.sheet.epoch)), font(16.0)));}
        if let Some(panel) = view.panel() {
            parent.spawn((Node { width: percent(100), height: px(8), ..default() }, BackgroundColor(Color::srgb(panel.accent[0], panel.accent[1], panel.accent[2]))));
            parent.spawn((Text::new(panel.title), font(24.0)));
            parent.spawn((Text::new(panel.body), font(19.0)));
            if panel.countdown && view.failure.is_none() { parent.spawn((CountdownText, Text::new(""), font(20.0))); }
        }
        let status=if canonical {format!("Generator: {} | De-icer: {} | Annex: {}",generator.status.label(),if generator.deicer_isolated {"isolated"}else{"shorted"},if generator.engineering_lost {"sealed"}else{"open"})}else{format!("Generator: {} | Spare part: {}",generator.status.label(),if generator.has_spare_part {"carried"}else{"none"})};
        parent.spawn((Text::new(status),font(17.0)));
        if let Some(error) = &view.failure {
            parent.spawn((Text::new(error), font(18.0), TextColor(Color::srgb(1.0, 0.35, 0.25))));
        } else if view.actions_open() || canonical {
            for (index, (edge_id, edge)) in runtime.sheet.outgoing_edges.iter().enumerate() {
                let (suffix, color) = match crate::gameplay::blocked_reason(edge, &generator) {
                    Ok(None) if view.actions_open() => (String::new(), Color::srgb(0.55,0.85,0.95)),
                    Ok(None) => (" [locked: listen to the crew]".into(),Color::srgb(0.6,0.6,0.6)),
                    Ok(Some(reason)) => (format!(" [locked: {reason}]"), Color::srgb(0.6, 0.6, 0.6)),
                    Err(error) => (format!(" [binding error: {error}]"), Color::srgb(1.0, 0.35, 0.25)),
                };
                parent.spawn((Text::new(format!("[{}] {}{suffix}", index + 1, crate::content::edge_label(edge_id).expect("validated WHITEOUT edge"))), font(19.0), TextColor(color)));
            }
        } else { parent.spawn((Text::new("Controls locked while the presentation unfolds."), font(18.0))); }
        let staging=match *mode {crate::art::GraphicMode::Staged=>"A · Staged 2D",crate::art::GraphicMode::Layered=>"B · Layered 2.5D"};
        parent.spawn((Text::new(format!("{staging} · F2: compare staging depth\nNumber key: act · Release before selecting again · Close window: exit")),font(16.0)));
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
        app.insert_resource(Runtime::new(
            grimm_graph::GraphEngine::new(
                serde_yaml_ng::from_str(include_str!("gameplay/fixtures/gameplay-v3.yaml"))
                    .unwrap(),
            )
            .unwrap(),
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

#[cfg(test)]
mod gameplay_tests {
    use super::*;
    use crate::{game, scheduler};
    use bevy::{input::InputPlugin, time::TimeUpdateStrategy};
    use std::time::Duration;

    fn text(app: &mut App) -> String {
        let world = app.world_mut();
        world
            .query::<&Text>()
            .iter(world)
            .map(|text| text.0.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }
    #[test]
    fn ecs_changes_refresh_locked_options_and_keyboard_rechecks_stale_display() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, game::install, install))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::ZERO));
        app.insert_resource(Runtime::new(
            grimm_graph::GraphEngine::new(
                serde_yaml_ng::from_str(include_str!("gameplay/fixtures/gameplay-v3.yaml"))
                    .unwrap(),
            )
            .unwrap(),
        ));
        app.update();
        scheduler::tests::key(
            &mut app,
            bevy::input::ButtonState::Pressed,
            false,
            KeyCode::Digit1,
        );
        scheduler::tests::key(
            &mut app,
            bevy::input::ButtonState::Released,
            false,
            KeyCode::Digit1,
        );
        scheduler::tests::advance(&mut app, 5000);
        assert!(text(&mut app).contains(
            "[2] Repair the generator with the spare relay [locked: requires a spare part]"
        ));
        app.world_mut().resource_mut::<Generator>().has_spare_part = true;
        scheduler::tests::advance(&mut app, 0);
        let current = text(&mut app);
        assert!(current.contains("Spare part: carried"));
        assert!(current.contains("[2] Repair the generator with the spare relay"));
        assert!(!current.contains("requires a spare part"));
        assert!(!current.contains("generator_exploded"));
        app.world_mut().resource_mut::<Generator>().has_spare_part = false;
        // The old display offered repair, but execution reads the current ECS.
        scheduler::tests::key(
            &mut app,
            bevy::input::ButtonState::Pressed,
            false,
            KeyCode::Digit2,
        );
        assert_eq!(
            app.world().resource::<Runtime>().sheet.node_id,
            "GENERATOR_WARNING"
        );
        assert!(text(&mut app).contains("[locked: requires a spare part]"));
    }
}
