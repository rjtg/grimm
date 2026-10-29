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

Status: initial planning documentation; no engine implementation yet.
