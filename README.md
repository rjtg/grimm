# GRIMM

**Generate Real IMMersion** — a modular, headless narrative game engine in Rust.

GRIMM supports authored choose-your-own-adventure graphs and, as optional extensions, free-form player actions, dynamic story events, goal-driven NPCs, narrative direction, and multiple presentation frontends.

## Principles
- The engine owns canonical game state and rules.
- Engine output is semantic; adapters translate it into presentation instructions.
- Renderers handle presentation and raw interaction; input adapters translate interaction into semantic actions.
- AI is optional and never directly mutates canonical state.
- Classic games work without AI or graphics.
- Each roadmap milestone ends with a playable or executable demonstration.

See [vision](docs/vision.md), [architecture](docs/architecture.md), [requirements](docs/requirements.md), [roadmap](docs/roadmap.md), and [M0 brief](docs/milestones/m0.md).

Status: M0 through M1.75 implemented: an authored CYOA engine with YAML stories, typed
conditions/effects, structured narrative output, and versioned session saves.
Three reference adventures exercise persistent state, bounded playability checks,
and optional deterministic world time and real-time decisions.
AI and graphical frontends remain future milestones.

## Build and run

Use a current stable Rust toolchain with Cargo, rustfmt, and Clippy (edition 2024).
From the repository root:

```sh
cargo build --workspace --locked
cargo run --locked -p grimm-cli -- play stories/the-last-witness.yaml
```

Choose displayed numbers to play **The Last Witness**. Investigate a mill fire,
find evidence, and decide whether to earn the witness's trust. Three endings
depend on your choices. Session commands are `:save PATH`, `:load PATH`, and
`:quit`. Paths may contain spaces; enter them without surrounding quotes.
EOF exits without taking a choice. Completed stories display their ending and
exit. The engine API can also save and restore completed sessions.

For example, save at Mara's first appearance by entering `1`, `1`, then
`:save /tmp/last-witness.json`. Resume with:

```sh
cargo run --offline --locked -p grimm-cli -- play stories/the-last-witness.yaml --load /tmp/last-witness.json
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

To save at the night watch, enter `1`, `1`, `1`, `1`, then
`:save /tmp/lighthouse.json`. Resume with:

```sh
cargo run --offline --locked -p grimm-cli -- play stories/the-last-lighthouse.yaml --load /tmp/lighthouse.json
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

During a live deadline, use `:pause`, then `:save /tmp/train.json`. `:resume`
continues after displaying the prompt again. To load the checkpoint later:

```sh
cargo run -p grimm-cli -- play stories/the-last-train.yaml --load /tmp/train.json
cargo run --offline --locked -p grimm-cli -- check stories/the-last-train.yaml
```

Closed games do not consume decision time. Saving alone does not pause a running
session. The engine preserves remaining time; the host activates it only after
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
