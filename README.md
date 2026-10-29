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

Status: M0 architecture foundation implemented. The engine is a one-transition stub;
story graphs, YAML, persistence, AI, and graphics belong to later milestones.

## Build and run

Use a current stable Rust toolchain with Cargo, rustfmt, and Clippy (edition 2024).
From the repository root:

```sh
cargo build --workspace --locked
cargo run --locked -p grimm-cli
```

Enter `1` and press Enter. The demo displays:

```text
Welcome to GRIMM's architecture demo.
1. Continue
> The engine accepted your choice. Demo complete.
```

Other input prompts for a displayed number and repeats the current screen. EOF
exits cleanly without choosing. For a reproducible noninteractive demonstration:

```sh
printf '1\n' | cargo run --offline --locked --quiet -p grimm-cli
```

The first build needs the locked Serde dependencies available locally (run
`cargo fetch --locked` while online if necessary). Once cached, builds, tests,
and the demo work with `--offline`. Running the built `target/debug/grimm-cli`
requires no network, browser, graphics, model, or other service.

## Verification

```sh
cargo fmt --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Tests include the actual CLI process, invalid input and EOF, engine validation,
serialization, presentation-state isolation, a compile-fail state-privacy check,
and Cargo dependency-boundary checks. CI runs these checks and the offline demo.
See [M0 implementation notes](docs/milestones/m0-implementation.md) for the
crate map, acceptance coverage, protocol assumptions, and remaining questions.
