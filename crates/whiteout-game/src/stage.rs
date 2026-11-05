//! WHITEOUT's annex set and cue-owned audio/motion instances.
use crate::{
    cueables::{self, Binding, Catalog, Object, Sound},
    game::FrameSet,
    gameplay::{Generator, GeneratorStatus},
    scheduler::{CueInstance, Runtime, SceneView},
};
use bevy::{
    audio::{AudioPlayer, AudioSink, AudioSource, PlaybackSettings, Volume},
    prelude::*,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Resource, Default)]
struct Staged {
    epoch: Option<u64>,
    started: BTreeSet<CueInstance>,
}
#[derive(Resource)]
struct AudioBank(BTreeMap<Sound, Handle<AudioSource>>);
#[derive(Component)]
pub(crate) struct CueEntity(pub CueInstance);
#[derive(Component)]
struct Motion {
    target: Object,
    start_ms: u64,
    length_ms: Option<u64>,
    from: Transform,
    binding: Binding,
}
#[derive(Component)]
struct Home(Transform);
#[derive(Component)]
struct Lamp;
#[derive(Component)]
struct Snow(u32);

pub(crate) fn install(app: &mut App) {
    app.init_resource::<Assets<AudioSource>>()
        .init_resource::<Staged>()
        .init_resource::<Catalog>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, remember_homes)
        .add_systems(Update, audit_assets.in_set(FrameSet::Collect))
        .add_systems(Update, report_audio_started.in_set(FrameSet::Present))
        .add_systems(
            Update,
            (reset_entry, sync_cues, animate, paint)
                .chain()
                .in_set(FrameSet::Present),
        );
}
fn rect(commands: &mut Commands, color: Color, size: Vec2, position: Vec3) -> Entity {
    commands
        .spawn((
            Sprite::from_color(color, size),
            Transform::from_translation(position),
        ))
        .id()
}
fn setup(mut commands: Commands, mut audio: ResMut<Assets<AudioSource>>) {
    let bank = Sound::ALL
        .into_iter()
        .map(|s| {
            (
                s,
                audio.add(AudioSource {
                    bytes: s.bytes().into(),
                }),
            )
        })
        .collect();
    commands.insert_resource(AudioBank(bank));
    rect(
        &mut commands,
        Color::srgb(0.035, 0.07, 0.11),
        Vec2::new(1400., 900.),
        Vec3::new(0., 80., -20.),
    );
    rect(
        &mut commands,
        Color::srgb(0.12, 0.2, 0.26),
        Vec2::new(940., 350.),
        Vec3::new(0., 130., -15.),
    );
    for x in [-400., -200., 0., 200., 400.] {
        rect(
            &mut commands,
            Color::srgb(0.17, 0.27, 0.32),
            Vec2::new(6., 340.),
            Vec3::new(x, 130., -14.),
        );
    }
    rect(
        &mut commands,
        Color::srgb(0.08, 0.1, 0.12),
        Vec2::new(1200., 90.),
        Vec3::new(0., -70., -10.),
    );
    // Generator casing, cooling fins, flywheel, exhaust and tagged fuse panel.
    rect(
        &mut commands,
        Color::srgb(0.22, 0.31, 0.3),
        Vec2::new(210., 150.),
        Vec3::new(50., 70., 0.),
    );
    for x in [-20., 0., 20., 40., 60.] {
        rect(
            &mut commands,
            Color::srgb(0.08, 0.12, 0.14),
            Vec2::new(6., 100.),
            Vec3::new(x, 80., 1.),
        );
    }
    rect(
        &mut commands,
        Color::srgb(0.25, 0.28, 0.3),
        Vec2::new(30., 140.),
        Vec3::new(20., 210., 0.),
    );
    let wheel = commands
        .spawn((
            Object::Wheel,
            Transform::from_xyz(140., 60., 3.),
            Visibility::default(),
        ))
        .id();
    for rotation in [0., std::f32::consts::FRAC_PI_2] {
        let child = commands
            .spawn((
                Sprite::from_color(Color::srgb(0.45, 0.5, 0.48), Vec2::new(78., 12.)),
                Transform::from_rotation(Quat::from_rotation_z(rotation)),
            ))
            .id();
        commands.entity(wheel).add_child(child);
    }
    for (object, x, color) in [
        (Object::Starter, 225., Color::srgb(0.9, 0.6, 0.2)),
        (Object::Deicer, 185., Color::srgb(0.8, 0.18, 0.12)),
    ] {
        rect(
            &mut commands,
            Color::srgb(0.07, 0.11, 0.13),
            Vec2::new(24., 55.),
            Vec3::new(x, 140., 1.),
        );
        let id = rect(
            &mut commands,
            color,
            Vec2::new(8., 38.),
            Vec3::new(x, 145., 2.),
        );
        commands.entity(id).insert(object);
    }
    let lamp = rect(
        &mut commands,
        Color::srgb(0.9, 0.2, 0.15),
        Vec2::new(26., 15.),
        Vec3::new(75., 165., 4.),
    );
    commands.entity(lamp).insert(Lamp);
    let shutter = rect(
        &mut commands,
        Color::srgb(0.3, 0.38, 0.4),
        Vec2::new(120., 280.),
        Vec3::new(310., 400., 6.),
    );
    commands.entity(shutter).insert(Object::Shutter);
    for (object, x, color) in [
        (Object::Jonas, -340., Color::srgb(0.75, 0.48, 0.24)),
        (Object::Mara, -420., Color::srgb(0.28, 0.58, 0.66)),
    ] {
        let person = commands
            .spawn((
                object,
                Transform::from_xyz(x, 70., 5.),
                Visibility::default(),
            ))
            .id();
        let coat = commands
            .spawn((
                Sprite::from_color(color, Vec2::new(36., 70.)),
                Transform::default(),
            ))
            .id();
        let head = commands
            .spawn((
                Sprite::from_color(Color::srgb(0.75, 0.66, 0.56), Vec2::splat(24.)),
                Transform::from_xyz(0., 48., 0.),
            ))
            .id();
        let legs = commands
            .spawn((
                Sprite::from_color(Color::srgb(0.12, 0.16, 0.2), Vec2::new(24., 32.)),
                Transform::from_xyz(0., -48., 0.),
            ))
            .id();
        let label = commands
            .spawn((
                Text2d::new(if object == Object::Jonas {
                    "JONAS"
                } else {
                    "MARA"
                }),
                TextFont {
                    font_size: bevy::text::FontSize::Px(14.0),
                    ..default()
                },
                Transform::from_xyz(0.0, 75.0, 1.0),
            ))
            .id();
        commands
            .entity(person)
            .add_children(&[coat, head, legs, label]);
    }
    // Deterministic snow outside the annex, no random/time-dependent test setup.
    for i in 0..50 {
        let e = rect(
            &mut commands,
            Color::srgba(0.7, 0.85, 0.95, 0.4),
            Vec2::new(12., 2.),
            Vec3::new(
                (i * 71 % 1100) as f32 - 550.,
                (i * 31 % 230) as f32 + 240.,
                10.,
            ),
        );
        commands.entity(e).insert(Snow(i));
    }
}

fn audit_assets(
    runtime: Res<Runtime>,
    mut view: ResMut<SceneView>,
    catalog: Res<Catalog>,
    bank: Res<AudioBank>,
    sources: Res<Assets<AudioSource>>,
    objects: Query<&Object>,
) {
    if view.failure.is_some() {
        return;
    }
    for cue in &runtime.sheet.cues {
        let grimm_graph::Instruction::Presentation {
            presentation_id, ..
        } = &cue.instruction
        else {
            continue;
        };
        let result = (|| -> Result<(), String> {
            match cueables::resolve(presentation_id, &catalog)? {
                Binding::Panel => {}
                Binding::Audio { sound, .. } => {
                    let handle = bank
                        .0
                        .get(&sound)
                        .ok_or_else(|| format!("missing loaded audio '{}'", sound.name()))?;
                    if sources.get(handle).is_none() {
                        return Err(format!("missing loaded audio data '{}'", sound.name()));
                    }
                }
                binding => {
                    let target = match binding {
                        Binding::Camera { .. } => Object::Camera,
                        Binding::Move { object, .. } | Binding::Rotate { object, .. } => object,
                        _ => Object::Wheel,
                    };
                    if !objects.iter().any(|object| *object == target) {
                        return Err(format!(
                            "missing live stage object {target:?} for '{presentation_id}'"
                        ));
                    }
                }
            }
            Ok(())
        })();
        if let Err(error) = result {
            error!("{error}");
            view.failure = Some(error);
            break;
        }
    }
}

fn remember_homes(mut commands: Commands, objects: Query<(Entity, &Transform), With<Object>>) {
    for (entity, transform) in &objects {
        commands.entity(entity).insert(Home(*transform));
    }
}
fn reset_entry(
    mut commands: Commands,
    runtime: Res<Runtime>,
    view: Res<SceneView>,
    mut staged: ResMut<Staged>,
    old: Query<Entity, With<CueEntity>>,
    mut objects: Query<(&Home, &mut Transform), With<Object>>,
) {
    let changed = staged.epoch != Some(runtime.sheet.epoch);
    if changed || view.failure.is_some() {
        for entity in &old {
            commands.entity(entity).despawn();
        }
        staged.started.clear();
        staged.epoch = Some(runtime.sheet.epoch);
        if changed {
            for (home, mut transform) in &mut objects {
                *transform = home.0;
            }
        }
    }
}

fn sync_cues(
    mut commands: Commands,
    mut view: ResMut<SceneView>,
    catalog: Res<Catalog>,
    bank: Res<AudioBank>,
    mut staged: ResMut<Staged>,
    objects: Query<(&Object, &Transform)>,
) {
    if view.failure.is_some() {
        return;
    }
    for cue in view.active.clone() {
        if staged.started.contains(&cue.instance) {
            continue;
        }
        let result = (|| -> Result<(), String> {
            let binding = cueables::resolve(&cue.id, &catalog)?;
            match binding {
                Binding::Panel => {}
                Binding::Audio { sound, looping } => {
                    let source = bank
                        .0
                        .get(&sound)
                        .ok_or_else(|| format!("missing loaded audio '{}'", sound.name()))?;
                    let settings = if looping {
                        PlaybackSettings::LOOP
                    } else {
                        PlaybackSettings::DESPAWN
                    };
                    commands.spawn((
                        CueEntity(cue.instance.clone()),
                        AudioPlayer::new(source.clone()),
                        PlaybackSettings {
                            volume: Volume::Linear(0.35),
                            ..settings
                        },
                    ));
                }
                _ => {
                    let target = match binding {
                        Binding::Camera { .. } => Object::Camera,
                        Binding::Move { object, .. } | Binding::Rotate { object, .. } => object,
                        _ => Object::Wheel,
                    };
                    let (_, from) = objects
                        .iter()
                        .find(|(object, _)| **object == target)
                        .ok_or_else(|| {
                            format!("missing live stage object {target:?} for '{}'", cue.id)
                        })?;
                    commands.spawn((
                        CueEntity(cue.instance.clone()),
                        Motion {
                            target,
                            start_ms: cue.at_ms,
                            length_ms: cue.duration_ms,
                            from: *from,
                            binding,
                        },
                    ));
                }
            }
            Ok(())
        })();
        if let Err(error) = result {
            error!("{error}");
            view.failure = Some(error);
            break;
        }
        staged.started.insert(cue.instance);
    }
}

fn animate(
    runtime: Res<Runtime>,
    jobs: Query<(&CueEntity, &Motion)>,
    mut objects: Query<(&Object, &mut Transform)>,
    mut snow: Query<(&Snow, &mut Transform), Without<Object>>,
) {
    // Later authored jobs override earlier jobs for the same target. Entity
    // iteration order is never the tie-breaker.
    let mut jobs: Vec<_> = jobs
        .iter()
        .filter(|(cue, _)| cue.0.epoch == runtime.sheet.epoch)
        .collect();
    jobs.sort_by_key(|(cue, motion)| {
        (
            motion.start_ms,
            runtime
                .sheet
                .cues
                .iter()
                .position(|c| c.cue_id == cue.0.cue_id)
                .unwrap_or(usize::MAX),
        )
    });
    for (_, motion) in jobs {
        let elapsed = runtime.elapsed_ms.saturating_sub(motion.start_ms);
        let fraction = motion.length_ms.map_or(1., |length| {
            if length == 0 {
                1.
            } else {
                (elapsed as f32 / length as f32).min(1.)
            }
        });
        if let Some((_, mut transform)) = objects
            .iter_mut()
            .find(|(object, _)| **object == motion.target)
        {
            match motion.binding {
                Binding::Camera { position, scale } => {
                    transform.translation.x = motion.from.translation.x
                        + (position.x - motion.from.translation.x) * fraction;
                    transform.translation.y = motion.from.translation.y
                        + (position.y - motion.from.translation.y) * fraction;
                    transform.scale = motion.from.scale.lerp(Vec3::splat(scale), fraction);
                }
                Binding::Move { position, .. } => {
                    transform.translation.x = motion.from.translation.x
                        + (position.x - motion.from.translation.x) * fraction;
                    transform.translation.y = motion.from.translation.y
                        + (position.y - motion.from.translation.y) * fraction;
                }
                Binding::Rotate { radians, .. } => {
                    transform.rotation = motion
                        .from
                        .rotation
                        .slerp(Quat::from_rotation_z(radians), fraction)
                }
                Binding::Spin => transform.rotation = Quat::from_rotation_z(elapsed as f32 * 0.009),
                _ => {}
            }
        }
    }
    for (flake, mut transform) in &mut snow {
        let t = runtime.elapsed_ms as f32 / 1000.;
        transform.translation.x = ((flake.0 * 71) as f32 - t * 65.).rem_euclid(1100.) - 550.;
        transform.translation.y = (flake.0 * 31 % 230) as f32 + 240. - (t * 10.).rem_euclid(100.);
    }
}
fn paint(
    generator: Res<Generator>,
    runtime: Res<Runtime>,
    mut lamps: Query<&mut Sprite, With<Lamp>>,
) {
    let stable = matches!(
        generator.status,
        GeneratorStatus::Repaired | GeneratorStatus::Powered
    );
    let pulse = 0.45 + 0.45 * (runtime.elapsed_ms as f32 * 0.008).sin().abs();
    for mut lamp in &mut lamps {
        lamp.color = if stable {
            Color::srgb(0.2, 0.9, 0.35)
        } else {
            Color::srgb(pulse, 0.12, 0.08)
        };
    }
}

#[cfg(test)]
mod tests;

fn report_audio_started(started: Query<&CueEntity, Added<AudioSink>>) {
    for cue in &started {
        info!(
            "WHITEOUT audio started epoch={} cue={}",
            cue.0.epoch, cue.0.cue_id
        );
    }
}
