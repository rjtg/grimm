# WHITEOUT — Bevy gameplay demo (#67–#69)

A concrete WHITEOUT game application embedding `grimm-graph`. This small
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

**Repair / restored power**:

1. In `GENERATOR_ROOM`, press **2** to take the spare relay. At 600ms the
   inventory display changes; after 900ms you return to the room. Taking another
   part is now shown as locked.
2. Release and press **1** to inspect the panel. At 2s the generator becomes
   unstable; at 5s controls open with 25s left.
3. Press **2** to repair. The repair transition starts immediately, but the part
   is consumed and status becomes repaired only at 1200ms. At 1500ms the scene
   advances automatically to **Power restored**.

**Isolation / no restored power**:

1. Skip the relay: press **1** from the room. After 5s, repair is shown as locked
   with a missing-part reason.
2. Press **1** to isolate the generator. At 1200ms status becomes isolated; at
   1500ms the safe outcome appears, with power still unavailable.

**Timeout / blackout**: inspect the panel and leave it unanswered. At scene time
30000ms the hidden timeout edge reaches the blackout. It wins at equality.

Each outcome offers **1** to restart through an authored reset cue, clearing the
part and returning to a running generator. Release keys between selections.
Holding a key never skips a scene or auto-selects when controls open. Close the
window to exit. Hidden `continue`/timeout edges never appear as numbered choices.

See [GAMEPLAY.md](GAMEPLAY.md) for typed bindings, world-state ownership and live
availability/rechecks, and [SCHEDULER.md](SCHEDULER.md) for the preserved timing
and failure contract.

## Recorded desktop verification (2026-11-04)

Linux/X11, Bevy 0.20.1, Mesa 26.2.4 llvmpipe (LLVM 23.1.1): captured the real
1000×700 application window with locked initial controls, the status change,
ready controls with **25s** remaining, transition presentation and safe outcome.
A second pass without selection reached the blackout. Actual key press/release
events targeted the application window; the log showed:

```text
GENERATOR_WARNING epoch=1
GENERATOR_TRANSITION epoch=2
GENERATOR_DECISION epoch=3
GENERATOR_ROOM epoch=4
GENERATOR_WARNING epoch=5
GENERATOR_FAILED epoch=6
```

The automatic transition completed about 1.58s after entry (1500ms, on the next
render frame); the timeout occurred about 30.02s after warning entry. The window
closed cleanly. Exact timing/race assertions use the deterministic tests, not
wall-clock sleep or screenshot timing.

## Architecture, tests and limits

See the [scheduler contract](SCHEDULER.md) for node-relative Bevy real time,
central arbitration, instance deduplication, REPLACE, failure behavior and the
one-traversal-per-frame budget. `assets/bootstrap.yaml` is a small authored graph,
not a full port of the existing WHITEOUT adventure.

- `src/game.rs`: composition, asset validation and keyboard collection.
- `src/scheduler.rs`: WHITEOUT clock/arbiter; GRIMM owns node identity/epoch.
- `src/gameplay.rs`: typed conditions/effects over the actual generator resource.
- `src/content.rs`: text/color bindings and explicit host input/countdown gates.
- `src/screen.rs`: fixed camera, executed presentation, status and countdown UI.

```bash
CARGO_PROFILE_DEV_DEBUG=0 cargo test -p whiteout-game --locked
```

Tests use Bevy MinimalPlugins/InputPlugin with a deterministic fake clock,
without `sleep()`, a window or GPU. The manual steps above exercise actual
window/input/rendering. No WorldStateProvider, GRIMM cue ACK or second story FSM.

Effects remain synchronous and game-specific; execution errors are visible and
stop the entry. `duration_ms` is metadata, not media completion. Full WHITEOUT
(#70), save/restore (#63), asset generation and codegen remain out of scope.

Architecture: [#66](https://github.com/rjtg/grimm/issues/66), scheduler
[#68](https://github.com/rjtg/grimm/issues/68),
[ADR-0009](../../docs/adr/0009-story-graph-03-foundation.md).
