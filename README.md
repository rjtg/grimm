# GRIMM

**Generate Real IMMersion** — a modular, headless narrative game engine in Rust.

GRIMM supports authored choose-your-own-adventure graphs and, as optional extensions, free-form player actions, dynamic story events, goal-driven NPCs, narrative direction, and multiple presentation frontends.

**M4/M5 delivery direction:** Build the complete M4 adventure as an authored text game first. In M5, adapt that **same narrative/gameplay definition** into a graphical game application that uses GRIMM as a headless Rust library, while the concrete application/renderer provides assets, UI, animation and its own presentation state. Keep both example applications **in this repository** (preferably under `examples/`) and load the **identical GRIMM game script**; only renderer-specific cue/action bindings, text, dialogue, assets and input presentation differ. External-host APIs beyond this example remain open. See [roadmap](docs/roadmap.md).

## Principles
- The engine owns canonical game state and rules.
- Engine output is semantic; adapters translate it into presentation instructions.
- Renderers handle presentation and raw interaction; input adapters translate interaction into semantic actions.
- A playable game is a **concrete composition** of authored narrative, GRIMM, a concrete runtime/renderer, assets and other components. A standalone GRIMM-led game can select its renderer; an external 3D/game engine may instead embed GRIMM for narrative events. The supported host-integration contract remains an open question.
- AI is optional and never directly mutates canonical state.
- Classic games work without AI or graphics.
- Gameplay milestones include playable demos showing their new capabilities; tooling-only M3 validates and indexes existing authored content, with regression tests.

See [vision](docs/vision.md), [architecture](docs/architecture.md), [requirements](docs/requirements.md), [roadmap](docs/roadmap.md), [open questions](docs/open-questions.md), and [M0 brief](docs/milestones/m0.md).

Status: M0 through M4 implemented: an authored CYOA engine with YAML stories, typed
conditions/effects, structured narrative output, and versioned session saves.
Four reference adventures exercise persistent state, bounded playability checks,
optional deterministic time, narrative-only continuation and presentation-gated decisions.
M2 adds a scene DSL, full replacement Cue Sheets and The Locked Room. M3 adds source-aware validation, a semantic story index and structural exit checks. M4 delivers **WHITEOUT**, a complete survival-mystery text adventure. The next milestone is **M5 Graphic Adaptation** (the **same authored M4 game** presented through a concrete graphical renderer and game-specific assets). The former **M3 Inventory/Puzzles/Character-State scope is cancelled and replaced** by authoring tools because ordinary M2 state/choice rules already express the required lightweight gameplay. Choices remain the shared state-dependent action model, with optional RPG attributes and item combinations rather than hard-coded player verbs. Choices may take **authored execution time**: GRIMM resolves each choice **immediately at selection** using its duration, known scene deadline and the externally supplied selection time. It commits the outcome and sends the prescribed presentation (e.g. a five-second logical drawer-opening action or a four-second logical attempt cut short by a deadline); the next gameplay choices become usable at the Cue Sheet's explicit availability time, subject to safe input readiness, even if an animation remains in progress. Classic **choice-selection** deadlines still resolve on timely selection or authored silence. GRIMM never reads/ticks a wall clock and needs no running action/progress callback for these authored cases. Runtime Dynamic Node Providers, generative story extensions, free text and NPC direction are deferred toward a later open-world/Story-RPG phase; their boundaries are intentionally not defined yet.

## Build and run

Use a current stable Rust toolchain with Cargo, rustfmt, and Clippy (edition 2024).
From the repository root:

```sh
cargo build --workspace --locked
cargo run --locked -p grimm-cli -- play stories/the-last-witness.yaml
```

Play **WHITEOUT**, M4's complete polar survival-mystery adventure:

```sh
cargo run --locked -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml
```

Repair Aurora's generator and radio, improvise with limited equipment, and decide
how to bring Alex, Mara, Jonas and Lea home. Three endings reflect your repairs,
preparation and survival decisions. See the [player introduction](examples/whiteout/README.md),
[spoiler walkthrough](examples/whiteout/WALKTHROUGH.md) and
[state/time acceptance notes](examples/whiteout/DESIGN.md). All rules and semantic
IDs are independent of the text bindings so the same story can support M5.

Play **The Locked Room**, the M2 scene-DSL mini-game:

```sh
cargo run --locked -p grimm-cli -- play-scene examples/locked-room/story.yaml examples/locked-room/text.yaml
cargo run --locked -p grimm-cli -- check-scene examples/locked-room/story.yaml
```

Enter finishes prose; number keys choose immediately and Esc opens the paused
session menu. The hidden thirty-second countdown starts after the introduction;
a radio warning reveals it at ten seconds. Escape with keys, strength, dexterity
or an iron bar. Rules and text bindings are separate. See the
[play guide](examples/locked-room/README.md) and [M2 format](docs/scene-format.md)
for deterministic clock controls, save/load and the replacement Cue Sheet contract.
Legacy adventures continue to use `play` and their existing schemas unchanged.

Validate and index scene stories without renderer assets:

```sh
cargo run --locked -p grimm-cli -- check-scene examples/locked-room/story.yaml
cargo run --locked -p grimm-cli -- check-scene examples/locked-room/detective.yaml --json
cargo run --locked -p grimm-cli -- index-scene examples/locked-room/story.yaml > locked-room-index.json
```

Invalid stories exit 1; valid stories, including informational state-dependent
checks, exit 0. The JSON index separates qualified action IDs, cue instruction
IDs and reusable narrative bindings, with source references. See the
[M3 guide](docs/milestones/m3.md) for diagnostics, output examples, stricter
structural checks and their deliberately non-exhaustive limits.

Play **The Silent Signal**, M1.8's atmospheric research-station mystery:

```sh
cargo run -p grimm-cli -- play stories/the-silent-signal.yaml
```

It has 14 nodes and three endings. Narrative-only segments carry the longer
storm and emergency buildup. Two interactive scenes require presentation completion:
the relay and the final decision. Read the relay freely, then press Enter to finish
its presentation. Choices appear next and the full 12-second countdown starts only
when they are usable. The investigation and final gated choice are untimed.
Evidence, trust and reserve power change rescue
options; silence after activation has an authored consequence. In a terminal,
press a choice's number key to act immediately, without Enter. **Esc opens the
session menu:** `1` Continue, `2` Save, `3` Load, `4` Quit. Esc also closes the
menu. All menu actions are single keypresses; no command typing. More than nine
choices use number + Enter. Redirected input keeps the original line-based format.

During required reading, only blank Enter completes presentation; numbers do not
acknowledge or choose. The menu pauses any active deadline and preserves unfinished
reading. Save/Load use one checkpoint per story under `.grimm-saves/` in the current
directory (path shown in the menu). For The Silent Signal, resume later using:

```sh
cargo run -p grimm-cli -- play stories/the-silent-signal.yaml --load .grimm-saves/story-the-silent-signal.json
```

The adventure is now story version 3; saves from playtest versions 1 and 2
are rejected rather than mapped onto the revised scenes. Unfinished presentation
replays; offline time stays frozen. Opening the menu and continuing
preserve the reading phase and remaining deadline. EOF exits without completing
required storytelling. On an active countdown it still permits authored expiry,
but cannot skip a successor's required reading. Stale lines and lifecycle commands
cannot acknowledge or alter a later interaction. Old games keep their usual prompts.

Reproduce the renderer-neutral lifecycle and all new game endings:

```sh
cargo test --locked -p grimm-cli --test flow
cargo test --locked -p grimm-cli --test train silent_signal_actual_cli_routes_reach_all_three_endings
cargo test --locked -p grimm-cli --test train actual_gated_cli_reading_can_outlast_deadline_then_choice_or_timeout_resolves
cargo test --locked -p grimm-cli --test train silent_signal_actual_cli_reading_outlasts_authored_deadline_without_spending_it
cargo test --locked -p grimm-cli --test train silent_signal_actual_cli_acknowledged_relay_expires_to_authored_lost_call
cargo run --locked -p grimm-cli -- check stories/the-silent-signal.yaml
```

See [flow design](docs/milestones/m1.8-design.md),
[schema/lifecycle reference](docs/story-format.md#story-schema-3-narrative-flow-and-presentation-gating-m18),
and [authoring/playtest findings](docs/milestones/m1.8-playtest.md).

Choose displayed numbers to play **The Last Witness**. Investigate a mill fire,
find evidence, and decide whether to earn the witness's trust. Three endings
depend on your choices. In a terminal, Esc opens the session menu above.
For piped input, session commands remain `:save PATH`, `:load PATH`, and
`:quit`. Paths may contain spaces; enter them without surrounding quotes.
EOF exits without taking a choice. Completed stories display their ending and
exit. The engine API can also save and restore completed sessions.

For example, save at Mara's first appearance by pressing `1`, `1`, then Esc, `2`.
Resume with:

```sh
cargo run --offline --locked -p grimm-cli -- play stories/the-last-witness.yaml --load .grimm-saves/story-the-last-witness.json
```

Reproduce all endings from the repository root:

```sh
# A Voice in the Light: investigate, take ledger, reassure, testify
printf '1\n1\n1\n1\n' | cargo run --offline --locked --quiet -p grimm-cli -- play stories/the-last-witness.yaml
# Truth Across the Water: investigate, take ledger, press, publish
printf '1\n1\n2\n1\n' | cargo run --offline --locked --quiet -p grimm-cli -- play stories/the-last-witness.yaml
# The Price of Silence: leave
printf '2\n' | cargo run --offline --locked --quiet -p grimm-cli -- play stories/the-last-witness.yaml
```

To write another adventure, see the [story and save format](docs/story-format.md).
No story-specific code is required in the engine.

## The Last Lighthouse

Investigate a transmission from the **Orison**, a ship missing for forty-three
winters. Revisit the North Spit radio room in daylight, on the night watch and
at dawn; decide whether to trust keeper Lena Voss, answer the signal, or follow
the offshore lights. This independent, 15-node mystery has four authored endings
and uses the existing M1 story format.

```sh
cargo run --locked -p grimm-cli -- play stories/the-last-lighthouse.yaml
```

To save at the night watch, press `1`, `1`, `1`, `1`, then Esc, `2`. Resume with:

```sh
cargo run --offline --locked -p grimm-cli -- play stories/the-last-lighthouse.yaml --load .grimm-saves/story-the-last-lighthouse.json
```

Reproduce each ending:

```sh
# Witnesses on the Shore: evidence and partnership
printf '1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n' | cargo run --offline --locked --quiet -p grimm-cli -- play stories/the-last-lighthouse.yaml
# The Watch Is Over: callsign and grounded reply
printf '1\n1\n1\n1\n3\n1\n1\n1\n' | cargo run --offline --locked --quiet -p grimm-cli -- play stories/the-last-lighthouse.yaml
# An Unanswered Bell: caution without evidence
printf '2\n2\n3\n2\n1\n1\n' | cargo run --offline --locked --quiet -p grimm-cli -- play stories/the-last-lighthouse.yaml
# One More Voice: an intentional bad ending, not a deadlock
printf '2\n2\n3\n1\n2\n' | cargo run --offline --locked --quiet -p grimm-cli -- play stories/the-last-lighthouse.yaml
```

See the [authoring/playtest report](docs/milestones/m1.5-authoring-report.md) for
the identity conventions, actual CLI playtest, friction points and follow-ups.

## Check an authored story

Run this before publishing a story, in addition to its regression playthroughs:

```sh
cargo run --offline --locked -p grimm-cli -- check stories/the-last-lighthouse.yaml
cargo run --offline --locked -p grimm-cli -- check stories/the-last-witness.yaml
```

The check finds reachable nonterminal deadlocks using a deterministic bounded
traversal (defaults: 1,024 distinct narrative states and 8,192 choice inspections).
It reports node/choice routes and observed endings. A deadlock or budget cutoff
returns a failure status; **inconclusive is not validation**. Larger finite
stories can raise budgets with `--max-states N --max-transitions N` after the
story filename. This does not prove eventual completion or detect every possible
liveness problem; executable cycles can continue indefinitely. See the precise
[guarantees and limits](docs/story-format.md#bounded-authoring-check-m15).

## The Last Train — temporal thriller

```sh
cargo run -p grimm-cli -- play stories/the-last-train.yaml
```

Investigate a sabotaged railway junction before the last train arrives. The
14-node adventure has three endings, two visible decision countdowns (15 and
20 real seconds), and untimed investigation choices with fictional minute costs.
The opening lets you read the controls without a timer. A timely, prepared stop
and a late evacuation have different consequences; hesitation has an authored
outcome rather than an I/O error.

During a live deadline, press Esc to pause, then `2` to save. `1` (or Esc)
continues after displaying the prompt again. To load the checkpoint later:

```sh
cargo run -p grimm-cli -- play stories/the-last-train.yaml --load .grimm-saves/story-the-last-train.json
cargo run --offline --locked -p grimm-cli -- check stories/the-last-train.yaml
```

Closed games do not consume decision time. The terminal menu pauses before saving;
a piped `:save PATH` alone does not pause a running session. The engine preserves
remaining time; the host activates it only after
flushing the prompt. At the exact deadline, timeout wins because the host sends
its elapsed observation before a player action. No-input expiry works with an
open stdin; real hesitation never automatically advances fictional minutes.

For a replayable manual-clock demonstration of every ending and the temporal
branches (no sleeping in the core tests):

```sh
cargo test -p grimm-cli --test train full_routes_cover_every_ending_and_both_fictional_time_windows -- --exact
cargo test -p grimm-cli --test train unanswered_call_and_platform_have_authored_consequences_without_player_input -- --exact
```

The actual CLI is human playable with real deadlines; automated CLI tests use
paced input and short real-host timeout fixtures. If timeout changes the scene
while an old line is still pending, press Enter when asked to acknowledge the
new prompt; the old line cannot choose there and its next timer stays frozen
until acknowledgement. Pace scripted input by waiting for prompts.
See the [temporal authoring/lifecycle guide](docs/story-format.md#temporal-stories-schema-2),
[technical decisions](docs/milestones/m1.75-design.md), and
[Train playtest report](docs/milestones/m1.75-playtest.md).

## M0 regression demo

`cargo run --locked -p grimm-cli` without arguments still runs the original
architecture round trip, now using an ordinary two-node story. Enter `1`:

```text
Welcome to GRIMM's architecture demo.
1. Continue
> The engine accepted your choice. Demo complete.
Ending: Demo complete
```

Other input prompts for a displayed number and repeats the current screen. EOF
exits cleanly without choosing. For a reproducible noninteractive demonstration:

```sh
printf '1\n' | cargo run --offline --locked --quiet -p grimm-cli
```

The first build needs the locked serialization dependencies available locally (run
`cargo fetch --locked` while online if necessary). Once cached, builds, tests,
and the demo work with `--offline`. Running the built `target/debug/grimm-cli`
requires no network, browser, graphics, model, or other service.

## Verification

Unix terminal regression tests also require Python 3 (standard library only)
to drive a real pseudo-terminal and verify immediate keys and terminal restoration.

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Tests include complete reference playthroughs, invalid YAML/graphs, conditions
and atomic effects, save/load continuation, the actual CLI process, and all M0
boundary/privacy regressions. CI runs these checks, bounded story checks, and
offline demos of the reference stories and real-host temporal integration tests. M1.5 tests add shared-location scenes, recurring
character/state continuity, four endings, multi-step deadlocks, cycles and
inconclusive budgets. See the [M1.5 brief](docs/milestones/m1.5.md).
See the [M1 acceptance plan and implementation report](docs/milestones/m1.md)
for coverage, compatibility notes and open questions, and the historical
[M0 implementation notes](docs/milestones/m0-implementation.md) for the foundation.


## Game composition direction (planned M5)

A story graph is not a complete visual game. A standalone game's composition may explicitly select **where its concrete renderer comes from**, along with game-specific assets, scene/hotspot mappings and other required modules. An externally hosted game may already own its renderer and invoke GRIMM only for narrative decisions. A Bevy-capable renderer is not automatically capable of playing arbitrary authored 3D adventures. The headless engine remains reusable; the surrounding composition root assembles the actual product. Whether GRIMM is the application entry point or an embedded narrative subsystem is still under investigation. See [architecture](docs/architecture.md#game-composition-and-renderer-ownership-future-m5) and [open questions](docs/open-questions.md#game-composition-renderer-origin-and-packaging). The exact packaging and loading API will be designed from a complete M5 demonstration, not imposed on existing CLI games now. See [embedded-host questions](docs/open-questions.md#host-integration-standalone-game-versus-embedded-narrative-engine).
