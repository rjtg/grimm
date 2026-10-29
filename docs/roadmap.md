# Roadmap

Every milestone ends with an executable demo and regression tests. Milestones describe capabilities, not mandatory features for every game.

| Milestone | Scope | Acceptance demo |
|---|---|---|
| M0 Architecture Foundation | All core boundaries, minimal semantic messages, stub engine/adapter/renderer loop, dependency tests | End-to-end action -> engine -> adapter -> renderer -> input -> engine |
| M1 Classic CYOA | Authored graph, choices, conditions/effects, authored text adapter, CLI, save/load | Play `The Last Witness` through multiple endings offline |
| M1.5 World Modeling & Dogfooding | Build a second independently authored test story; exercise stable characters/locations across different scenes; diagnose narrative deadlocks (CYOA-09); capture authoring friction | Play three distinct scenes at one location, with state-driven choices and at least three endings; publish authoring findings |
| M1.75 Temporal Foundation | Optional deterministic world clock, timed decisions and timeout consequences, authored time advancement, pause/resume and save/load semantics | Play a short scripted timed-choice scenario with manual/fake clock, timeout branch and reliable deterministic replay; untimed M1 stories unchanged |
| M1.8 Narrative Flow & Presentation Gating | Narrative-only presentation segments, host/renderer completion acknowledgement, explicit interaction readiness, bounded engine-owned continuation and player-paced CLI fallback | A storytelling segment finishes before a timed choice becomes available; reading/camera delay does not consume its deadline, and a narrative-only scene advances without a fake choice |
| M2 Generative Presentation | Second text adapter using optional LLM, visible-context projection, exact text preservation, deterministic fallback | Same story in authored and runtime-narrated presentation |
| M3 Free Input | Free text, intent resolver interface, rule-based and optional model resolver, action validation | Alternative phrasings resolve to valid actions |
| M4 Dynamic Narrative | Validated dynamic events/nodes, runtime graph overlay, persistence | Unscripted action leads to a new playable event/node |
| M5 NPC Agency | Optional goals, beliefs/knowledge, rule controllers, event-triggered planning | NPC changes plan when it learns relevant information |
| M6 Narrative Direction | Optional milestones, constraints, director proposals | Milestone reached by two different causal paths |
| M7 Visual Presentation | Additional adapter/renderer, asset references, presentation synchronization | Same game logic presented visually without core changes |

M0 must establish architecture boundaries; M2 is **not** a belated adapter refactor. Only one simple text adapter is needed in M1. Do not scaffold all future modules in M0.

M1.5 does **not** automatically introduce an entity store, location simulation, scene hierarchy, or ECS. First author a second game against existing M1 primitives, then add only minimal, justified support for location/entity references as needed. The existing M1 `Node` may act as an authored narrative scene; no forced breaking schema rename. M4 may extend persistent world-entity state; M5 adds optional NPC agency; M6 adds optional progress milestones. The classic CLI game remains valid throughout.

M1.75 establishes a **single temporal model** before LLM presentation and free-text input. It is independent from M1.5's world-entity model and may reuse the lighthouse test game for demonstration without retrofitting it. The core does not poll the wall clock, sleep or own a frontend timer. Host/runtime drivers supply elapsed time explicitly; adapters render player-visible remaining time and frontend readiness. M3 may later pair timed offers with free-form intent; M4–M6 may react to time progression for events, NPC behavior and story direction. No background actor or full scheduling engine is required for M1.75.

M1.8 refines the lifecycle established by M1.75: **finishing presentation is not identical to making a choice available**. A narrative segment may stand alone and advance after an authorized completion acknowledgement, while an interactive segment's decision timer must start only after required storytelling and actual offer readiness. A text CLI should allow the player to acknowledge reading before a deadline starts; visual adapters/renderers will later report completion of dialogue, camera movement, animation or other required atmospheric work through the composition root. The engine remains authoritative over all transitions and deadlines; neither completion events nor scene IDs imply a location entity. The smallest compatible engine/semantic/presentation protocol extension should be implemented before M2, with a replayable timed story demonstrating both paths. No generic cutscene sequencer, full auto-play graph or renderer-to-engine direct dependency is intended.

## Delivery rules
- Each requirement has tests and acceptance criteria.
- The classic game remains playable after every milestone.
- Later features do not become mandatory dependencies for earlier experiences.
- Unknown future APIs remain deliberately unspecified.
