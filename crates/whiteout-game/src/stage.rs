//! WHITEOUT's annex set and cue-owned audio/motion instances.
#[cfg(test)]
use crate::gameplay::{Generator, GeneratorStatus};
use crate::{
    cueables::{self, Binding, Catalog, Object, Sound},
    game::FrameSet,
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
struct Snow(u32);

pub(crate) fn install(app: &mut App) {
    app.init_resource::<Assets<AudioSource>>()
        .init_resource::<Staged>()
        .init_resource::<Catalog>()
        .init_resource::<Assets<Image>>()
        .init_resource::<crate::art::GraphicMode>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, remember_homes)
        .add_systems(Update, crate::art::check_loaded.before(FrameSet::Collect))
        .add_systems(Update, audit_assets.in_set(FrameSet::Collect))
        .add_systems(Update, report_audio_started.in_set(FrameSet::Present))
        .add_systems(
            Update,
            (reset_entry, sync_cues, animate, crate::art::paint)
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
fn setup(
    mut commands: Commands,
    mut audio: ResMut<Assets<AudioSource>>,
    server: Option<Res<AssetServer>>,
) {
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
    commands.insert_resource(crate::art::Paintings::load(server.as_deref()));
    crate::art::spawn_set(&mut commands);
    for (object, position) in [
        (Object::Jonas, Vec3::new(-340., 140., 5.)),
        (Object::Mara, Vec3::new(-420., 140., 5.)),
        (Object::Alex, Vec3::new(65., 115., 5.)),
    ] {
        let parent = commands
            .spawn((
                object,
                Transform::from_translation(position),
                Visibility::default(),
            ))
            .id();
        crate::art::spawn_actor(&mut commands, parent, object);
    }
    // Physical presentation anchors remain the existing cue targets. The inked
    // cel variants show their state; no geometric placeholder is rendered.
    for (object, position) in [
        (Object::Wheel, Vec3::new(140., 150., 3.)),
        (Object::Starter, Vec3::new(225., 230., 2.)),
        (Object::Deicer, Vec3::new(270., 230., 2.)),
    ] {
        commands.spawn((
            object,
            Transform::from_translation(position),
            Visibility::default(),
        ));
    }
    let shutter = commands
        .spawn((
            Object::Shutter,
            Transform::from_xyz(0., 930., 6.),
            Visibility::default(),
        ))
        .id();
    crate::art::spawn_shutter(&mut commands, shutter);
    // Small drawn snow streaks stay inside the sealed west window, not indoors.
    for i in 0..24 {
        let entity = rect(
            &mut commands,
            Color::srgba(0.8, 0.9, 0.95, 0.18),
            Vec2::new(6., 1.),
            Vec3::new(
                -525. + (i * 17 % 200) as f32,
                280. + (i * 19 % 100) as f32,
                -15.,
            ),
        );
        commands.entity(entity).insert(Snow(i));
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
                Binding::Panel | Binding::Artwork => {}
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
                Binding::Panel | Binding::Artwork => {}
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
        transform.translation.x = -525. + ((flake.0 * 17) as f32 - t * 7.).rem_euclid(200.);
        transform.translation.y = 280. + ((flake.0 * 19) as f32 - t * 3.).rem_euclid(100.);
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
