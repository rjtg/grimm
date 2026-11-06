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
    let mut plugins = DefaultPlugins
        .build()
        .set(AssetPlugin {
            file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/assets").into(),
            ..default()
        })
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "WHITEOUT — generator station".into(),
                resolution: (1000, 700).into(),
                ..default()
            }),
            ..default()
        });
    // A host-only opt-out for machines without usable sound output. Cues still
    // run and are cleaned up normally; no narrative timing/effects change.
    if std::env::args().any(|argument| argument == "--mute") {
        plugins = plugins.disable::<bevy::audio::AudioPlugin>();
    }
    App::new()
        .add_plugins(plugins)
        .add_plugins((game::install, stage::install, screen::install))
        .run();
}
