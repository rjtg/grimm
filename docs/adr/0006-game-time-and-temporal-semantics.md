# ADR-0006: Authoritative game time and temporal semantics

Status: Proposed (for review in documentation PR)
Date: 2026-10-29

## Context
Timed decisions in narrative adventures create urgency: not choosing is itself consequential. Separately, investigating a crime scene or waiting for a ship may advance hours of fictional time. Renderer countdowns, player-facing drama, world chronology and story progress are different concerns. Implementing independent clocks in frontends, NPCs or story logic would compromise deterministic tests and replay.

## Decision
1. Distinguish **world time** (fictional chronology), **decision time** (deadline for a specific offered interaction) and **narrative progress** (result of committed consequences/milestones).
2. Temporal rules are optional. Ordinary authored CYOA stories have no countdown unless explicitly configured.
3. The authoritative engine owns temporal state and commits time-related transitions, but **never directly reads the wall clock, sleeps, or polls**. Host/runtime delivers time advances and lifecycle events through a defined boundary.
4. The temporal engine handles deadlines as stateful, versioned interactions. Start only after a defined readiness event/acknowledgement or other explicitly specified activation; define consistent same-instant ordering, so timeout and player input cannot both commit.
5. On expiry, a deterministic **authored timeout policy** selects a valid effect/action/transition; renderer failure or missing timer UI cannot invent a different outcome. If a deadline is configured without a valid fallback, story validation should reject it.
6. Explicit world-time advances are rule-validated and may affect narrative conditions. Real-time elapsed seconds and world-time minutes need not advance at the same rate.
7. Pause/resume, save/load and replay must use an explicit policy for frozen vs elapsed time while suspended and preserve the ability to resolve a pending deadline once. Tests use a fake/manual clock, not wall-clock sleeps.
8. The semantic boundary may expose only player-visible deadline information, while presentation adapters translate it into countdowns/audio/visual cues. Hidden world-timing state must remain private.
9. No always-on simulator, general scheduling platform, NPC agency, network synchronization protocol or graphical animation system is required by M1.75.

## Consequences
The core and CLI remain deterministic and headless. A minimal runtime driver may be needed to provide ticks/acknowledgements and receive input; dependency direction and canonical state ownership stay unchanged. Exact Rust types, time units and deadline UI representation are to be chosen during implementation with focused tests and versioning notes. M4+ can build on the same temporal model for timed world events and autonomous behaviors.

## Alternatives rejected
- Renderer-only timer with renderer-authoritative timeout: violates canonical state ownership.
- Core polling system time: makes replay, pause and tests nondeterministic.
- Mandatory global real-time simulation for all stories: unnecessarily breaks offline classic CYOA and overbuilds M1.75.
