# WHITEOUT — playable Bevy vertical slice (#67–#70)

A concrete WHITEOUT game using `grimm-graph` as a library. The generator-annex
excerpt adapts the existing canonical text adventure: prepare the damaged
de-icer, start the generator before access locks, or return without power.
There is no universal adapter, second story FSM or rule evaluation in GRIMM.

## Build and run

Rust **1.97.1+**, verified with 1.98.1; Bevy pinned to **0.20.1**.
From the repository root:

```bash
CARGO_PROFILE_DEV_DEBUG=0 cargo build -p whiteout-game --locked
CARGO_PROFILE_DEV_DEBUG=0 cargo run -p whiteout-game --locked
```

Debug symbols are disabled to reduce Bevy build size. Once dependencies are
cached, `--offline` works. Graph, placeholder WAVs and font are embedded; there
are no runtime network downloads. Linux needs an X11/Wayland session, Vulkan
support and an audio output for sound. Debian/Ubuntu build prerequisites:

```bash
sudo apt-get install pkg-config libwayland-dev libxkbcommon-dev libasound2-dev
```

CI installs these headers but opens no window/audio device. With unsupported
hardware Vulkan, Mesa's software driver can run this small scene. On Arch/Manjaro:

```bash
sudo pacman -Syu vulkan-swrast
```

The normal run command then works with llvmpipe. This is no performance claim
for a full game. The CLI remains independent: `cargo run -p grimm-cli --locked`.

## Play

1. Watch the 18-second approach: Jonas and Mara move towards the generator;
   the camera shows the de-icer diagram.
2. The annex clock starts automatically. At two seconds the fuse is isolated;
   at four seconds choices unlock. **1** leaves the annex; **2** starts the
   generator. Gray choices explain their lock. Release keys between selections.
3. Wait ten seconds into the annex to hear Lea's warning and see **20 seconds**
   remaining. This reveals the running clock; it does not start a new countdown.
4. **2** enters the animated 4.5-second startup. Power commits after four seconds;
   the hall then warms. Alternatively, **1** leaves with no power, or silence
   through 30 seconds seals the annex automatically. Timeout wins at equality.
5. Outcomes settle for six seconds (warm) or nine (cold), then **1** replays.
   Holding a key does not carry an action into the next scene. Close to exit.

A full route takes roughly 31–57 seconds plus decision/outcome waiting. Keyboard
input is implemented; there are no clickable choices. The side-on annex,
characters, moving levers, flywheel, shutter, camera shots, snow and three sounds
are intentionally simple [original placeholders](assets/audio/README.md).

The separate [SCREENPLAY.md](SCREENPLAY.md) specifies each authored beat, cue,
asset/handler mapping, canonical source and **gameplay simplifications**, including
the prepared latch/warm hands and protected startup after timely selection.
This is not the entire WHITEOUT adventure.

## Architecture and verification

- `src/game.rs`: composition, binding validation, keyboard collection.
- `src/scheduler.rs`: host clock, cue/input arbitration, epoch deduplication and REPLACE.
- `src/gameplay.rs`: actual Bevy generator state and typed conditions/effects.
- `src/content.rs`: text and explicit host input/countdown gates.
- `src/cueables.rs`, `src/stage.rs`: concrete sprite/camera/audio bindings and lifecycle.
- `src/screen.rs`: current captions, conditions, status and countdown.

See [SCHEDULER.md](SCHEDULER.md) and [GAMEPLAY.md](GAMEPLAY.md). Presentation
instances can overlap. A declared duration is not proof of completed playback.
Missing bindings/resources visibly stop the entry. REPLACE despawns old audio
and motion work while retaining the bounded fixed set and three source assets.
No change to `grimm-graph`, cue ACK, provider, save system or generic media layer.

```bash
CARGO_PROFILE_DEV_DEBUG=0 cargo test -p whiteout-game --locked
```

Tests use deterministic Bevy time, no sleeps, window, GPU or audio output. The
original #67–#69 fixtures remain tested unchanged alongside canonical success,
abandonment, timeout, timed effects, overlapping cues, current conditions,
deadline equality, cleanup, missing assets and actual WAV decoding.
Desktop verification is recorded separately in [PLAYTEST.md](PLAYTEST.md).

Save/Restore (#63), codegen (#75), finished art/voice, mouse controls and the
remaining canonical locations/routes are outside this slice.
Architecture: [#66](https://github.com/rjtg/grimm/issues/66), slice
[#70](https://github.com/rjtg/grimm/issues/70),
[ADR-0009](../../docs/adr/0009-story-graph-03-foundation.md).
