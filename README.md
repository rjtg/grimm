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

Status: M0 and M1 implemented: an authored CYOA engine with YAML stories, typed
conditions/effects, structured narrative output, and versioned session saves.
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
boundary/privacy regressions. CI runs these checks and offline M0/M1 demos.
See the [M1 acceptance plan and implementation report](docs/milestones/m1.md)
for coverage, compatibility notes and open questions, and the historical
[M0 implementation notes](docs/milestones/m0-implementation.md) for the foundation.
