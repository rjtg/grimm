//! Concrete WHITEOUT staging bindings. Not a media framework.
use bevy::prelude::*;
use std::collections::BTreeSet;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Object {
    Camera,
    Alex,
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
}
impl Sound {
    pub const ALL: [Self; 3] = [Self::Wind, Self::Motor, Self::Alarm];
    pub fn name(self) -> &'static str {
        match self {
            Self::Wind => "wind.wav",
            Self::Motor => "motor.wav",
            Self::Alarm => "alarm.wav",
        }
    }
    pub fn bytes(self) -> &'static [u8] {
        match self {
            Self::Wind => include_bytes!("../assets/audio/wind.wav"),
            Self::Motor => include_bytes!("../assets/audio/motor.wav"),
            Self::Alarm => include_bytes!("../assets/audio/alarm.wav"),
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub(crate) enum Binding {
    Panel,
    Artwork,
    Camera { position: Vec2, scale: f32 },
    Move { object: Object, position: Vec2 },
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
            "art.open" | "art.controls" | "art.latch_crack" | "art.latch_nearly"
            | "art.latch_open" | "art.primer" | "art.wide" | "art.retreat" | "art.warm_reveal"
            | "pose.alex.work" | "pose.mara.support" | "pose.jonas.brace" => Binding::Artwork,
            "shot.wide" => Binding::Camera {
                position: Vec2::new(0., 100.),
                scale: 1.,
            },
            "shot.hall" => Binding::Camera {
                position: Vec2::new(-50.0, 110.0),
                scale: 1.0,
            },
            "shot.controls" => Binding::Camera {
                position: Vec2::new(40., 90.),
                scale: 0.72,
            },
            "shot.starter" => Binding::Camera {
                position: Vec2::new(100., 80.),
                scale: 0.62,
            },
            "move.jonas" => Binding::Move {
                object: Object::Jonas,
                position: Vec2::new(-210., 140.),
            },
            "move.mara" => Binding::Move {
                object: Object::Mara,
                position: Vec2::new(-285., 135.),
            },
            "move.shutter" => Binding::Move {
                object: Object::Shutter,
                position: Vec2::new(0., 220.),
            },
            "move.deicer" => Binding::Rotate {
                object: Object::Deicer,
                radians: -1.1,
            },
            "move.starter" => Binding::Rotate {
                object: Object::Starter,
                radians: 1.1,
            },
            "motor.spin" => Binding::Spin,
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
