//! WHITEOUT's illustrated plates and cutouts. All timing comes from the existing
//! host runtime; these are game-specific asset bindings, not another cue player.
use crate::{
    cueables::Object,
    gameplay::{Generator, GeneratorStatus},
    scheduler::{Runtime, SceneView},
};
use bevy::{prelude::*, sprite::Anchor};
use std::collections::BTreeMap;

#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GraphicMode {
    #[default]
    Staged,
    Layered,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Picture {
    Room,
    Machine,
    States,
    Bench,
    Alex,
    Mara,
    Jonas,
    Inserts,
    Shutter,
}
impl Picture {
    pub const ALL: [Self; 9] = [
        Self::Room,
        Self::Machine,
        Self::States,
        Self::Bench,
        Self::Alex,
        Self::Mara,
        Self::Jonas,
        Self::Inserts,
        Self::Shutter,
    ];
    pub fn file(self) -> &'static str {
        match self {
            Self::Room => "annex-room-layer-v1.png",
            Self::Machine => "generator-layer-v1.png",
            Self::States => "generator-states-v1.png",
            Self::Bench => "support-bench-v1.png",
            Self::Alex => "alex-poses-v1.png",
            Self::Mara => "mara-poses-v1.png",
            Self::Jonas => "jonas-poses-v1.png",
            Self::Inserts => "mechanism-inserts-v1.png",
            Self::Shutter => "shutter-v1.png",
        }
    }
    fn grid(self) -> UVec2 {
        match self {
            Self::Alex | Self::Mara | Self::Jonas => UVec2::new(3, 1),
            Self::States => UVec2::new(1, 3),
            Self::Inserts => UVec2::new(3, 2),
            _ => UVec2::ONE,
        }
    }
}
#[derive(Resource, Default)]
pub(crate) struct Paintings {
    handles: BTreeMap<Picture, Handle<Image>>,
    pub ready: bool,
}
pub(crate) fn ready(paintings: Option<Res<Paintings>>) -> bool {
    paintings.is_none_or(|p| p.ready)
}
impl Paintings {
    pub fn load(server: Option<&AssetServer>) -> Self {
        Self {
            handles: server
                .map(|s| {
                    Picture::ALL
                        .into_iter()
                        .map(|p| (p, s.load(format!("graphic-novel/images/{}", p.file()))))
                        .collect()
                })
                .unwrap_or_default(),
            ready: server.is_none(),
        }
    }
}
#[derive(Component)]
pub(crate) struct Illustration {
    picture: Picture,
    cell: usize,
    home: Transform,
    depth: f32,
    actor: Option<Object>,
    warm: bool,
}
fn layer(
    commands: &mut Commands,
    picture: Picture,
    cell: usize,
    size: Vec2,
    home: Transform,
    depth: f32,
    actor: Option<Object>,
    warm: bool,
) -> Entity {
    commands
        .spawn((
            Sprite {
                custom_size: Some(size),
                color: Color::NONE,
                ..default()
            },
            home,
            Illustration {
                picture,
                cell,
                home,
                depth,
                actor,
                warm,
            },
        ))
        .id()
}
pub(crate) fn spawn_set(commands: &mut Commands) {
    for (picture, z, depth, warm, cell) in [
        (Picture::Room, -20., 0.96, false, 0),
        (Picture::Machine, -10., 1., false, 0),
        (Picture::States, -9., 1., false, 0),
        (Picture::States, -8., 1., true, 2),
        (Picture::Bench, 4., 1., false, 0),
    ] {
        layer(
            commands,
            picture,
            cell,
            if picture == Picture::States {
                Vec2::new(1000., 500.)
            } else {
                Vec2::new(1200., 600.)
            },
            Transform::from_xyz(
                if picture == Picture::States { 50. } else { 0. },
                if picture == Picture::States {
                    240.
                } else {
                    220.
                },
                z,
            ),
            depth,
            None,
            warm,
        );
    }
    layer(
        commands,
        Picture::Inserts,
        0,
        Vec2::new(1100., 550.),
        Transform::from_xyz(0., 220., 18.),
        1.,
        None,
        false,
    );
}
pub(crate) fn spawn_actor(commands: &mut Commands, parent: Entity, object: Object) {
    let (picture, height) = match object {
        Object::Alex => (Picture::Alex, 250.),
        Object::Mara => (Picture::Mara, 245.),
        _ => (Picture::Jonas, 215.),
    };
    let child = layer(
        commands,
        picture,
        0,
        Vec2::splat(height),
        Transform::from_xyz(0., -70., 0.),
        1.,
        Some(object),
        false,
    );
    commands.entity(child).insert(Anchor::BOTTOM_CENTER);
    commands.entity(parent).add_child(child);
}
pub(crate) fn spawn_shutter(commands: &mut Commands, parent: Entity) {
    let child = layer(
        commands,
        Picture::Shutter,
        0,
        Vec2::new(1200., 600.),
        Transform::IDENTITY,
        1.04,
        None,
        false,
    );
    commands.entity(parent).add_child(child);
}
pub(crate) fn check_loaded(
    server: Option<Res<AssetServer>>,
    images: Res<Assets<Image>>,
    mut paintings: ResMut<Paintings>,
    mut view: ResMut<SceneView>,
) {
    let Some(server) = server else {
        return;
    };
    if view.failure.is_some() {
        return;
    }
    let was_ready = paintings.ready;
    let mut all = true;
    for (picture, handle) in &paintings.handles {
        if let bevy::asset::LoadState::Failed(error) = server.load_state(handle.id()) {
            view.failure = Some(format!(
                "WHITEOUT illustration '{}' failed to load: {error}",
                picture.file()
            ));
            all = false;
            break;
        }
        if images.get(handle).is_none() {
            all = false;
            if was_ready {
                view.failure = Some(format!("missing decoded illustration '{}'", picture.file()));
                break;
            }
        }
    }
    if paintings.ready != all {
        paintings.ready = all;
        view.set_changed();
    }
}
fn artwork(view: &SceneView, id: &str) -> Option<u64> {
    view.active
        .iter()
        .rev()
        .find(|p| p.id == id)
        .map(|p| p.at_ms)
}
fn fraction(now: u64, start: u64, length: f32) -> f32 {
    ((now.saturating_sub(start)) as f32 / length).clamp(0., 1.)
}
/// A bounded depth offset, disabled for the identical staged-2D comparison.
fn parallax(mode: GraphicMode, depth: f32, camera: Vec3) -> Vec2 {
    if mode == GraphicMode::Staged {
        Vec2::ZERO
    } else {
        (camera.truncate() - Vec2::new(0., 100.)) * (1. - depth).clamp(-0.08, 0.08)
    }
    .clamp(Vec2::splat(-6.), Vec2::splat(6.))
}
pub(crate) fn paint(
    paintings: Res<Paintings>,
    images: Res<Assets<Image>>,
    mode: Res<GraphicMode>,
    runtime: Res<Runtime>,
    view: Res<SceneView>,
    generator: Res<Generator>,
    camera: Query<&Transform, With<Camera2d>>,
    mut layers: Query<(&mut Illustration, &mut Sprite, &mut Transform), Without<Camera2d>>,
) {
    let (camera, camera_scale) = camera
        .iter()
        .next()
        .map_or((Vec3::ZERO, 1.), |t| (t.translation, t.scale.x));
    let time = runtime.elapsed_ms;
    let open = artwork(&view, "art.open").map_or(0., |at| fraction(time, at, 350.));
    let powered = matches!(
        generator.status,
        GeneratorStatus::Powered | GeneratorStatus::Repaired
    );
    let warmth = if powered {
        artwork(&view, "art.warm_reveal").map_or(1., |at| fraction(time, at, 500.))
    } else {
        0.
    };
    let retreat = artwork(&view, "art.retreat").is_some();
    let insert = view.active.iter().rev().find_map(|p| match p.id.as_str() {
        "art.controls" => Some((if generator.deicer_isolated { 1 } else { 0 }, p.at_ms)),
        "art.latch_crack" => Some((2, p.at_ms)),
        "art.latch_nearly" => Some((3, p.at_ms)),
        "art.latch_open" => Some((4, p.at_ms)),
        "art.primer" => Some((5, p.at_ms)),
        "art.wide" => Some((usize::MAX, p.at_ms)),
        _ => None,
    });
    for (mut art, mut sprite, mut transform) in &mut layers {
        *transform = art.home;
        transform.translation += parallax(*mode, art.depth, camera).extend(0.);
        let mut alpha = if paintings.ready { 1. } else { 0. };
        match art.picture {
            Picture::Machine => alpha *= 1. - open,
            Picture::States => {
                if art.warm {
                    alpha *= open * warmth;
                } else {
                    art.cell = usize::from(generator.deicer_isolated);
                    alpha *= open * (1. - warmth);
                }
            }
            Picture::Alex | Picture::Mara | Picture::Jonas => {
                art.cell = if retreat {
                    2
                } else if artwork(
                    &view,
                    if art.actor == Some(Object::Alex) {
                        "pose.alex.work"
                    } else if art.actor == Some(Object::Mara) {
                        "pose.mara.support"
                    } else {
                        "pose.jonas.brace"
                    },
                )
                .is_some()
                {
                    1
                } else {
                    0
                };
                // Anchored, subpixel breathing, not a skeleton or gameplay effect.
                transform.translation.y += (time as f32 * 0.0017).sin() * 0.65;
                if retreat {
                    transform.translation.z += 10.;
                    transform.translation.x -= 35. * fraction(time, 0, 1200.);
                }
            }
            Picture::Inserts => {
                if let Some((cell, at)) = insert.filter(|(cell, _)| *cell != usize::MAX) {
                    art.cell = cell;
                    alpha *= fraction(time, at, 240.);
                    transform.translation.x = camera.x;
                    transform.translation.y = camera.y + 135. * camera_scale;
                    sprite.custom_size = Some(Vec2::new(920., 460.) * camera_scale);
                } else {
                    alpha = 0.;
                }
            }
            _ => {}
        }
        // The existing motor cue produces a restrained illustrated vibration.
        if matches!(art.picture, Picture::Machine | Picture::States)
            && artwork(&view, "motor.spin").is_some()
        {
            transform.translation.y += (time as f32 * 0.045).sin() * 0.45;
        }
        if let Some(handle) = paintings.handles.get(&art.picture) {
            sprite.image = handle.clone();
            if let Some(image) = images.get(handle) {
                let grid = art.picture.grid();
                let size = image.size().as_vec2() / grid.as_vec2();
                let start = Vec2::new(
                    (art.cell as u32 % grid.x) as f32,
                    (art.cell as u32 / grid.x) as f32,
                ) * size;
                sprite.rect = Some(Rect::from_corners(start, start + size));
            }
        }
        let pulse = artwork(&view, "sound.alarm")
            .filter(|at| time.saturating_sub(*at) < 800)
            .map_or(0., |at| ((time - at) as f32 * 0.025).sin().abs() * 0.12);
        sprite.color = Color::srgba(1., 1. - pulse, 1. - pulse, alpha);
    }
}
