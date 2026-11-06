//! The concrete WHITEOUT application. No reusable host/adapter SDK.
mod art;
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
    // A device opt-out, not a different scheduler or altered media duration.
    if std::env::args().any(|a| a == "--mute") {
        plugins = plugins.disable::<bevy::audio::AudioPlugin>();
    }
    let mode = if std::env::args().any(|a| a == "--layered") {
        art::GraphicMode::Layered
    } else {
        art::GraphicMode::Staged
    };
    App::new()
        .add_plugins(plugins)
        .add_plugins((game::install, stage::install, screen::install))
        .insert_resource(mode)
        .run();
}
