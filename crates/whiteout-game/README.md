# WHITEOUT — Bevy bootstrap (#67)

A concrete WHITEOUT game application embedding `grimm-graph`. This three-scene
prototype is intentionally smaller than the existing canonical CLI adventure.
It does not provide a reusable Bevy adapter or another narrative state machine.

## Build and run

Rust **1.97.1 or newer**; Bevy is pinned to **0.20.1**. The verified toolchain is
Rust/Cargo 1.98.1. From the repository root:

```bash
CARGO_PROFILE_DEV_DEBUG=0 cargo build -p whiteout-game --locked
CARGO_PROFILE_DEV_DEBUG=0 cargo run -p whiteout-game --locked
```

Disabling development debug symbols reduces Bevy's disk/memory requirements; it
changes no game rules. After dependencies are downloaded the same commands work
with `--offline`. No external images, fonts, audio or runtime network are needed:
the YAML graph is embedded and Bevy supplies its built-in default font.

On Linux, use a working X11 or Wayland session and a Vulkan-capable graphics
driver. Native build prerequisites on Debian/Ubuntu (in addition to a C compiler
and Rust):

```bash
sudo apt-get install pkg-config libwayland-dev libxkbcommon-dev
```

CI installs the Wayland/xkbcommon headers explicitly; it needs no display/GPU to
build or run the tests. The selected Bevy features include UI rendering, windowing, keyboard and
multithreading, but no audio, gamepad, PBR/3D or third-party game framework.
The first dependency build is substantially larger than the CLI build. The CLI
still runs independently with `cargo run -p grimm-cli --locked`.

If the installed hardware Vulkan driver cannot provide a Bevy-compatible
adapter (as with the Haswell driver on the development machine), a Mesa software
Vulkan driver can run this small prototype. On Manjaro/Arch:

```bash
sudo pacman -Syu vulkan-swrast
```

The normal run command then works; the desktop smoke test selected llvmpipe
automatically. Software rendering is suitable for this static UI proof, not a
performance claim for the later graphical game.

## Play and manual smoke test

1. Start the command above. A window titled **WHITEOUT — generator station**
   shows `GENERATOR_ROOM`, entry 0, atmospheric text, a blue accent and one option.
2. Press **1** once: `GENERATOR_WARNING`, entry 1, replaces the old text/options
   and the accent becomes red. Holding the key must not skip the warning.
3. Release, then press **1** again: `GENERATOR_DECISION`, entry 2, shows the
   breaker scene with an amber accent and the demo restart option.
4. Release and press **1** to return to the generator room at a fresh epoch.
   Repeat as desired. Close the window to exit.

Only `outgoing_edges` from the full GRIMM sheet become numbered options. The
hidden `maintenance_bypass` edge must never appear. The host decides to traverse
on a fresh number-key press; it submits the epoch captured with that selection.
Rejected old/duplicate requests are logged, leaving the current display intact.

## Ownership and files

- `assets/bootstrap.yaml`: graph only; three ordinary nodes, immediate static
  presentation cues and unconditional edges. No duplicate of the full WHITEOUT
  story or alternative DSL.
- `src/game.rs`: concrete Bevy resources, input requests and graph invocation.
  GRIMM alone owns node/epoch. `SceneSheet` caches the complete returned packet;
  successful traversal replaces it, never merges it. No gameplay world variables
  exist in this bootstrap; future gameplay values belong in the host ECS.
- `src/content.rs`: game-specific text/color and edge-label bindings.
- `src/screen.rs`: fixed `Camera2d` and disposable UI hierarchy. Every replacement
  despawns the old root and its children and builds the new sheet's presentation.
- `src/lib.rs`/`main.rs`: WHITEOUT composition/entry point using Bevy DefaultPlugins.

Conditions and effects are not interpreted. Startup rejects unsupported cues,
missing bindings, conditional edges and nonzero/duration-bearing timing instead
of pretending to execute them. No WorldStateProvider, cue ACK, timer or generic
adapter is involved. The hidden bypass demonstrates discovery metadata, not an
access-control policy.

## Automated proof

```bash
CARGO_PROFILE_DEV_DEBUG=0 cargo test -p whiteout-game --locked
```

Tests run actual Bevy schedules with `MinimalPlugins` and `InputPlugin` without
opening a window or initializing a GPU. `KeyboardInput` messages pass through
Bevy's input system and the same WHITEOUT systems as the desktop game. They
verify the full three-node sequence, held-key rejection, duplicate/stale requests
(including return to a previous node), invalid IDs and hidden discovery. A UI
entity test verifies one fixed camera, visible destination text, new options and
destruction of the complete previous UI hierarchy. This is not a GPU screenshot
or a real OS-input test; use the manual procedure above for graphical checks.

## Recorded desktop verification (2026-11-04)

Built and launched on Linux/X11 with Rust 1.98.1, Bevy 0.20.1 and Mesa 26.2.4
llvmpipe (LLVM 23.1.1), after installing the software Vulkan driver. Captures of
the application's own 1000×700 window showed the blue generator-room scene,
red warning and amber decision scene, with each scene's distinct text and option.
X11 key press/release events sent to that window produced these actual game logs:

```text
WHITEOUT GENERATOR_WARNING epoch=1
WHITEOUT GENERATOR_DECISION epoch=2
WHITEOUT GENERATOR_ROOM epoch=3
```

The restart returned to a fresh entry; window-close exited cleanly. The initial
hardware-only launch failed to find a compatible adapter; installing the driver
resolved that without an application backend/framework change. Headless tests
remain the CI proof; the manual steps above reproduce the visual inspection.

## Deliberate limits / follow-ups

- **#68:** host cue scheduler, physical timing/deadline arbitration, timed
  traversal/effects, asynchronous cancellation/dedup and presentation readiness.
- **#69:** concrete conditions/effects and gameplay state/handlers.
- **#70:** real WHITEOUT vertical slice and game-specific art/input polish.
- **#63:** coherent host snapshots and save/restore; none here.

No full game port, asset generation, authoring IDE, codegen or optional rules
library extraction. The legacy WHITEOUT game and its rules are untouched.

Architecture: [#66](https://github.com/rjtg/grimm/issues/66),
[ADR-0009](../../docs/adr/0009-story-graph-03-foundation.md).
