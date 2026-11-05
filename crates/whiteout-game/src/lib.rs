//! The concrete WHITEOUT application. No reusable host/adapter SDK.
mod content;
mod cueables;
mod game;
mod gameplay;
mod scheduler;
mod screen;
mod stage;

pub fn run() {
    use bevy::prelude::*;
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "WHITEOUT — generator station".into(),
                resolution: (1000, 700).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((game::install, stage::install, screen::install))
        .run();
}
