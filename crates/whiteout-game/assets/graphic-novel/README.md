# WHITEOUT illustrated pilot (#83)

Work in progress: native Bevy illustration integration compiles; graphical
composition/playtest, asset registration and regression verification are pending.
Do not treat these assets as an approved whole-game production direction.

## Run and compare

```sh
CARGO_PROFILE_DEV_DEBUG=0 cargo run -p whiteout-game --locked
CARGO_PROFILE_DEV_DEBUG=0 cargo run -p whiteout-game --locked -- --layered
```

F2 switches A (staged 2D) / B (bounded layered 2.5D) during the same playthrough.
Number keys retain the existing decisions. `--mute` disables device playback for
machines without a working audio device, without changing authored durations.

Both modes use the same illustrations, story, camera moves, scheduler and UI.
Only B adds bounded depth offsets; there is no second presentation engine.

## Asset source and import

`images/` contains the unchanged PNG outputs of the built-in image-generation
tool. The master is a source/style reference; nine other images are loaded by
Bevy's PNG loader. No generation service is used at runtime. `sources/*.txt`
contains each exact prompt. Source references are the existing WHITEOUT cast
sheets and generator environment reference, followed by the selected annex
master and its derived layers. No third-party stock assets were imported.

These are generated illustration proposals, not hand-painted originals. No
layered PSD or deterministic generation seed is available. The PNG layers are
the editable production sources; regeneration is nondeterministic and requires
review. Reproducing the runtime means checking out this repository and running
the command above, with no export tools or image transformation required.
Distribution licensing for the repository remains a maintainer decision; no
third-party license grant or exclusive copyright is asserted here.

| Picture / runtime ID | Image | Grid | Use |
|---|---|---|---|
| Room | annex-room-layer-v1 | 1×1 | G01 cold empty room |
| Machine | generator-layer-v1 | 1×1 | Closed cover before inspection |
| States | generator-states-v1 | 1×3 | Open/connected, isolated, powered |
| Bench | support-bench-v1 | 1×1 | Jonas's brace/seat |
| Alex | alex-poses-v1 | 3×1 | Alert, work, withdraw; masked face provisional |
| Mara | mara-poses-v1 | 3×1 | Alert, support, withdraw |
| Jonas | jonas-poses-v1 | 3×1 | Seated, brace/advice, withdraw; left splint provisional |
| Inserts | mechanism-inserts-v1 | 3×2 | Controls, fuse, latch crack/nearly/open, primer |
| Shutter | shutter-v1 | 1×1 | G23–24 closure behind retreating crew |

Grids are row-major. `Sprite.rect` selects cells from their decoded dimensions;
no Python cropping, alpha reconstruction or upscaling is performed. Room,
machine and bench use a 1200×600 world canvas centred at (0,220). Actors use
bottom anchors, square pose cells and explicitly authored root positions.
The generator-state atlas has its own registration, subject to visual calibration.
Inserts are 920×460 screen-scale plates above the decision UI. Parallax offsets
are limited to six world units; contacting characters and props share depth.
See `src/art.rs` for exact import bindings and transforms.

Cues named `art.*` and `pose.*` are concrete WHITEOUT presentation IDs. They use
the existing scheduler and epoch replacement semantics; no GRIMM protocol or
world-state ownership changes are introduced. Mechanical close-ups interpret
existing presentation beats; authoritative effects and graph edges are retained.

See [AUDIT.md](AUDIT.md) for the baseline and useful lessons from closed PR #82.
