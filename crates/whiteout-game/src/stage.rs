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

pub(crate) fn install(app: &mut App) {
    app.init_resource::<Assets<AudioSource>>()
        .init_resource::<Staged>()
        .init_resource::<Catalog>()
        .init_resource::<Loading>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, remember_homes)
        .add_systems(Update, check_loading.before(FrameSet::Collect))
        .add_systems(Update, audit_assets.in_set(FrameSet::Collect))
        .add_systems(Update, report_audio_started.in_set(FrameSet::Present))
        .add_systems(
            Update,
            (
                reset_entry,
                sync_cues,
                animate,
                update_frost,
                pose_limbs,
                paint,
            )
                .chain()
                .in_set(FrameSet::Present)
                .run_if(assets_ready),
        );
}
#[derive(Resource, Default)]
pub(crate) struct Loading {
    pub pending: usize,
    handles: Vec<Handle<bevy::world_serialization::WorldAsset>>,
}
pub(crate) fn assets_ready(loading: Option<Res<Loading>>) -> bool {
    loading.is_none_or(|loading| loading.pending == 0)
}
#[derive(Component)]
struct ModelRoot(Option<Object>);
#[derive(Component)]
struct Limb {
    actor: Object,
    kind: String,
    rest: Transform,
}
#[derive(Component)]
struct WarmLight;
#[derive(Component)]
struct Frost;

const ROOM_OBJECTS: [(&str, Object, Vec3); 5] = [
    (
        "prop.generator_starter",
        Object::Starter,
        Vec3::new(0.58, 1.12, -0.86),
    ),
    (
        "prop.deicer_fuse",
        Object::Deicer,
        Vec3::new(1.65, 1.2, -0.94),
    ),
    (
        "prop.annex_shutter",
        Object::Shutter,
        Vec3::new(0., 4.1, 2.48),
    ),
    (
        "prop.generator_wheel",
        Object::Wheel,
        Vec3::new(0.1, 0.8, -0.92),
    ),
    (
        "prop.generator_latch",
        Object::Latch,
        Vec3::new(-0.55, 0.35, -0.965),
    ),
];
fn setup(
    mut commands: Commands,
    mut audio: ResMut<Assets<AudioSource>>,
    server: Option<Res<AssetServer>>,
) {
    commands.insert_resource(AudioBank(
        Sound::ALL
            .into_iter()
            .map(|s| {
                (
                    s,
                    audio.add(AudioSource {
                        bytes: s.bytes().into(),
                    }),
                )
            })
            .collect(),
    ));
    let mut loading = Loading::default();
    let actor_placements = [
        (
            Object::Alex,
            "alex.glb",
            Vec3::new(0.2, 0., 1.15),
            std::f32::consts::PI,
        ),
        (Object::Mara, "mara.glb", Vec3::new(-1.85, 0., 1.15), 0.7),
        (Object::Jonas, "jonas.glb", Vec3::new(-1.7, 0., 1.0), 0.25),
    ];
    for (object, file, position, angle) in actor_placements {
        let home =
            Transform::from_translation(position).with_rotation(Quat::from_rotation_y(angle));
        let mut entity = commands.spawn((object, home, Home(home), Visibility::default()));
        if let Some(server) = &server {
            let handle = server
                .load(GltfAssetLabel::Scene(0).from_asset(format!("production/models/{file}")));
            loading.handles.push(handle.clone());
            loading.pending += 1;
            entity
                .insert((WorldAssetRoot(handle), ModelRoot(Some(object))))
                .observe(model_ready);
        }
    }
    if let Some(server) = server {
        let handle = server
            .load(GltfAssetLabel::Scene(0).from_asset("production/models/generator-annex.glb"));
        loading.handles.push(handle.clone());
        loading.pending += 1;
        commands
            .spawn((WorldAssetRoot(handle), ModelRoot(None)))
            .observe(model_ready);
        commands.insert_resource(GlobalAmbientLight {
            color: Color::srgb(0.55, 0.7, 0.85),
            brightness: 160.,
            ..default()
        });
        commands.insert_resource(bevy::light::DirectionalLightShadowMap { size: 1024 });
        commands.spawn((
            bevy::light::CascadeShadowConfigBuilder {
                num_cascades: 1,
                maximum_distance: 12.,
                ..default()
            }
            .build(),
            DirectionalLight {
                illuminance: 1200.,
                shadow_maps_enabled: true,
                color: Color::srgb(0.65, 0.8, 1.),
                ..default()
            },
            Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.6, 0.)),
        ));
        commands.spawn((
            PointLight {
                intensity: 35000.,
                range: 7.,
                color: Color::srgb(0.55, 0.75, 1.),
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_xyz(-0.6, 2.65, 0.),
        ));
        commands.spawn((
            PointLight {
                intensity: 350.,
                range: 4.,
                color: Color::srgb(1., 0.12, 0.04),
                ..default()
            },
            Transform::from_xyz(1., 1.8, -0.4),
            Lamp,
        ));
        commands.spawn((
            PointLight {
                intensity: 0.,
                range: 6.,
                color: Color::srgb(1., 0.64, 0.3),
                ..default()
            },
            Transform::from_xyz(0.5, 2.25, -0.4),
            WarmLight,
        ));
    } else {
        // Deterministic host tests have no renderer/asset server. These are the
        // same physical pivots, never a second gameplay state or fallback art.
        for (_, object, position) in ROOM_OBJECTS {
            let home = Transform::from_translation(position);
            commands.spawn((object, home, Home(home)));
        }
    }
    commands.insert_resource(loading);
}
fn model_ready(
    ready: On<bevy::world_serialization::WorldInstanceReady>,
    mut commands: Commands,
    roots: Query<&ModelRoot>,
    children: Query<&Children>,
    nodes: Query<(&Name, &Transform)>,
    mut loading: ResMut<Loading>,
    mut view: ResMut<SceneView>,
) {
    let Ok(root) = roots.get(ready.entity) else {
        return;
    };
    let mut count = 0;
    for child in children.iter_descendants(ready.entity) {
        if let Ok((name, transform)) = nodes.get(child) {
            count += 1;
            if name.as_str() == "prop.latch_ice" {
                commands.entity(child).insert(Frost);
            }
            if let Some((_, object, _)) =
                ROOM_OBJECTS.iter().find(|(id, _, _)| *id == name.as_str())
            {
                commands.entity(child).insert((*object, Home(*transform)));
            }
            if let Some(actor) = root.0
                && name.starts_with("bone.")
            {
                commands.entity(child).insert(Limb {
                    actor,
                    kind: name.to_string(),
                    rest: *transform,
                });
            }
        }
    }
    loading.pending = loading.pending.saturating_sub(1);
    view.set_changed();
    info!(
        "WHITEOUT imported GLB {:?}: {count} named nodes; {} models pending",
        root.0, loading.pending
    );
}
fn check_loading(
    server: Option<Res<AssetServer>>,
    loading: Res<Loading>,
    mut view: ResMut<SceneView>,
) {
    if let Some(server) = server {
        for handle in &loading.handles {
            if let bevy::asset::LoadState::Failed(error) = server.load_state(handle.id()) {
                view.failure = Some(format!("WHITEOUT model import failed: {error}"));
            }
        }
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
                Binding::Camera { position, target } => {
                    transform.translation = motion.from.translation.lerp(position, fraction);
                    let end = Transform::from_translation(position).looking_at(target, Vec3::Y);
                    transform.rotation = motion.from.rotation.slerp(end.rotation, fraction);
                }
                Binding::Move { position, .. } => {
                    transform.translation = motion.from.translation.lerp(position, fraction);
                }
                Binding::Rotate { radians, .. } => {
                    transform.rotation = motion.from.rotation.slerp(
                        if motion.target == Object::Latch {
                            Quat::from_rotation_y(radians)
                        } else {
                            Quat::from_rotation_z(radians)
                        },
                        fraction,
                    )
                }
                Binding::Spin => transform.rotation = Quat::from_rotation_z(elapsed as f32 * 0.009),
                _ => {}
            }
        }
    }
}
fn update_frost(
    objects: Query<(&Object, &Transform)>,
    mut frost: Query<&mut Transform, (With<Frost>, Without<Object>)>,
) {
    let angle = objects
        .iter()
        .find(|(object, _)| **object == Object::Latch)
        .map_or(0., |(_, transform)| {
            transform.rotation.angle_between(Quat::IDENTITY)
        });
    for mut transform in &mut frost {
        transform.scale = Vec3::splat((1. - angle / 1.25).clamp(0.02, 1.));
    }
}
fn pose_limbs(
    runtime: Res<Runtime>,
    view: Res<SceneView>,
    mut limbs: Query<(&Limb, &mut Transform), Without<Object>>,
) {
    let time = runtime.elapsed_ms as f32 / 1000.;
    let working = view.active.iter().any(|cue| {
        matches!(
            cue.id.as_str(),
            "move.latch" | "move.starter" | "move.deicer"
        )
    });
    let retreat = view.active.iter().any(|cue| cue.id == "move.retreat");
    for (limb, mut transform) in &mut limbs {
        *transform = limb.rest;
        if limb.kind == "bone.chest" {
            transform.scale.y = 1. + 0.012 * (time * 1.7).sin();
            if limb.actor == Object::Mara {
                transform.rotation = Quat::from_rotation_z(if retreat { -0.15 } else { 0.06 });
            }
        }
        if limb.kind == "bone.head" {
            transform.rotation = Quat::from_rotation_y(0.1 * (time * 0.6).sin());
        }
        if limb.kind == "bone.arm.R" && limb.actor == Object::Alex && working {
            transform.rotation = Quat::from_rotation_x(-1.1 + 0.12 * (time * 8.).sin());
        }
        if limb.kind == "bone.arm.L" && limb.actor == Object::Mara {
            transform.rotation = Quat::from_rotation_x(-0.6);
        }
        if limb.kind == "bone.head" && limb.actor == Object::Jonas {
            transform.rotation *= Quat::from_rotation_z(-0.08);
        }
    }
}
fn paint(
    generator: Res<Generator>,
    runtime: Res<Runtime>,
    mut lamps: Query<&mut PointLight, With<Lamp>>,
    mut warm: Query<&mut PointLight, (With<WarmLight>, Without<Lamp>)>,
) {
    let stable = matches!(
        generator.status,
        GeneratorStatus::Repaired | GeneratorStatus::Powered
    );
    let pulse = 0.45 + 0.45 * (runtime.elapsed_ms as f32 * 0.008).sin().abs();
    for mut lamp in &mut lamps {
        lamp.color = if stable {
            Color::srgb(0.3, 1., 0.45)
        } else {
            Color::srgb(1., 0.1, 0.04)
        };
        lamp.intensity = if stable { 1200. } else { 900. * pulse };
    }
    for mut lamp in &mut warm {
        lamp.intensity = if stable { 18000. } else { 0. };
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
