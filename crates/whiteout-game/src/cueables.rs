//! Concrete WHITEOUT staging bindings. Not a media framework.
use bevy::prelude::*;
use std::collections::BTreeSet;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Object {
    Camera,
    Alex,
    Latch,
    Jonas,
    Mara,
    Starter,
    Deicer,
    Shutter,
    Wheel,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Sound {
    Wind,
    Motor,
    Alarm,
    Latch,
    Mechanism,
    Ignition,
    Overload,
}
impl Sound {
    pub const ALL: [Self; 7] = [
        Self::Wind,
        Self::Motor,
        Self::Alarm,
        Self::Latch,
        Self::Mechanism,
        Self::Ignition,
        Self::Overload,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Self::Wind => "wind.wav",
            Self::Motor => "motor.wav",
            Self::Alarm => "alarm.wav",
            Self::Latch => "latch.wav",
            Self::Mechanism => "mechanism.wav",
            Self::Ignition => "ignition.wav",
            Self::Overload => "overload.wav",
        }
    }
    pub fn bytes(self) -> &'static [u8] {
        match self {
            Self::Wind => include_bytes!("../assets/audio/wind.wav"),
            Self::Motor => include_bytes!("../assets/audio/motor.wav"),
            Self::Alarm => include_bytes!("../assets/audio/alarm.wav"),
            Self::Latch => include_bytes!("../assets/audio/latch.wav"),
            Self::Mechanism => include_bytes!("../assets/audio/mechanism.wav"),
            Self::Ignition => include_bytes!("../assets/audio/ignition.wav"),
            Self::Overload => include_bytes!("../assets/audio/overload.wav"),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum Binding {
    Panel,
    Camera { position: Vec3, target: Vec3 },
    Move { object: Object, position: Vec3 },
    Rotate { object: Object, radians: f32 },
    Spin,
    Audio { sound: Sound, looping: bool },
}
#[derive(Resource)]
pub(crate) struct Catalog {
    pub sounds: BTreeSet<Sound>,
    pub objects: BTreeSet<Object>,
}
impl Default for Catalog {
    fn default() -> Self {
        Self {
            sounds: Sound::ALL.into(),
            objects: [
                Object::Camera,
                Object::Alex,
                Object::Latch,
                Object::Jonas,
                Object::Mara,
                Object::Starter,
                Object::Deicer,
                Object::Shutter,
                Object::Wheel,
            ]
            .into(),
        }
    }
}
pub(crate) fn resolve(id: &str, catalog: &Catalog) -> Result<Binding, String> {
    let binding = if crate::content::presentation(id).is_some() {
        Binding::Panel
    } else {
        match id {
            "shot.wide" => Binding::Camera {
                position: Vec3::new(0.1, 2.6, 3.9),
                target: Vec3::new(0., -0.4, -0.6),
            },
            "shot.hall" => Binding::Camera {
                position: Vec3::new(0., 2.65, 6.8),
                target: Vec3::new(0., -0.3, 2.0),
            },
            "shot.controls" => Binding::Camera {
                position: Vec3::new(2.0, 1.8, 1.1),
                target: Vec3::new(1.6, 1.05, -0.94),
            },
            "shot.starter" => Binding::Camera {
                position: Vec3::new(1.0, 1.65, 1.4),
                target: Vec3::new(0.35, 1.0, -0.9),
            },
            "move.jonas" => Binding::Move {
                object: Object::Jonas,
                position: Vec3::new(-1.35, 0., 0.5),
            },
            "move.mara" => Binding::Move {
                object: Object::Mara,
                position: Vec3::new(-1.8, 0., 0.55),
            },
            "move.shutter" => Binding::Move {
                object: Object::Shutter,
                position: Vec3::new(0., 1.35, 2.48),
            },
            "move.deicer" => Binding::Rotate {
                object: Object::Deicer,
                radians: -1.1,
            },
            "move.starter" => Binding::Rotate {
                object: Object::Starter,
                radians: 1.1,
            },
            "move.latch" => Binding::Rotate {
                object: Object::Latch,
                radians: -1.25,
            },
            "move.alex" => Binding::Move {
                object: Object::Alex,
                position: Vec3::new(0.1, 0., -0.1),
            },
            "move.alex_fuse" => Binding::Move {
                object: Object::Alex,
                position: Vec3::new(1.25, 0., -0.15),
            },
            "move.mara_retreat" => Binding::Move {
                object: Object::Mara,
                position: Vec3::new(-0.7, 0., 3.25),
            },
            "move.jonas_retreat" => Binding::Move {
                object: Object::Jonas,
                position: Vec3::new(-1.2, 0., 3.3),
            },
            "move.retreat" => Binding::Move {
                object: Object::Alex,
                position: Vec3::new(0.3, 0., 3.15),
            },
            "shot.latch" => Binding::Camera {
                position: Vec3::new(1.2, 1.55, 1.25),
                target: Vec3::new(0.45, 0.85, -0.9),
            },
            "motor.spin" => Binding::Spin,
            "sound.latch" => Binding::Audio {
                sound: Sound::Latch,
                looping: false,
            },
            "sound.mechanism" => Binding::Audio {
                sound: Sound::Mechanism,
                looping: false,
            },
            "sound.ignition" => Binding::Audio {
                sound: Sound::Ignition,
                looping: false,
            },
            "sound.overload" => Binding::Audio {
                sound: Sound::Overload,
                looping: false,
            },
            "sound.wind" => Binding::Audio {
                sound: Sound::Wind,
                looping: true,
            },
            "sound.motor" => Binding::Audio {
                sound: Sound::Motor,
                looping: true,
            },
            "sound.alarm" => Binding::Audio {
                sound: Sound::Alarm,
                looping: false,
            },
            _ => return Err(format!("missing WHITEOUT presentation handler '{id}'")),
        }
    };
    let object = match binding {
        Binding::Camera { .. } => Some(Object::Camera),
        Binding::Move { object, .. } | Binding::Rotate { object, .. } => Some(object),
        Binding::Spin => Some(Object::Wheel),
        _ => None,
    };
    if let Some(object) = object
        && !catalog.objects.contains(&object)
    {
        return Err(format!("missing stage asset {object:?} for '{id}'"));
    }
    if let Binding::Audio { sound, .. } = binding {
        if !catalog.sounds.contains(&sound) {
            return Err(format!("missing audio asset '{}' for '{id}'", sound.name()));
        }
        let bytes = sound.bytes();
        if !bytes.starts_with(b"RIFF") || bytes.get(8..12) != Some(b"WAVE") {
            return Err(format!("invalid WAV asset '{}'", sound.name()));
        }
    }
    Ok(binding)
}
