# Roadmap

Every milestone ends with an executable demo and regression tests. Milestones describe capabilities, not mandatory features for every game.

| Milestone | Scope | Acceptance demo |
|---|---|---|
| M0 Architecture Foundation | All core boundaries, minimal semantic messages, stub engine/adapter/renderer loop, dependency tests | End-to-end action -> engine -> adapter -> renderer -> input -> engine |
| M1 Classic CYOA | Authored graph, choices, conditions/effects, authored text adapter, CLI, save/load | Play `The Last Witness` through multiple endings offline |
| M1.5 World Modeling & Dogfooding | Build a second independently authored test story; exercise stable characters/locations across different scenes; diagnose narrative deadlocks (CYOA-09); capture authoring friction | Play three distinct scenes at one location, with state-driven choices and at least three endings; publish authoring findings |
| M1.75 Temporal Foundation | Optional deterministic world clock, timed decisions and timeout consequences, authored time advancement, pause/resume and save/load semantics | Play a short scripted timed-choice scenario with manual/fake clock, timeout branch and reliable deterministic replay; untimed M1 stories unchanged |
| M1.8 Narrative Flow & Presentation Gating | Narrative-only presentation segments, host/renderer completion acknowledgement, explicit interaction readiness, bounded engine-owned continuation and player-paced CLI fallback | A storytelling segment finishes before a timed choice becomes available; reading/camera delay does not consume its deadline, and a narrative-only scene advances without a fake choice |
| M2 Dynamic Narrative Foundation | Engine-validated runtime node/event proposals, canonical dynamic graph overlay, persistence and replay; rule-based deterministic proposal provider, no LLM dependency | Play `The Unwritten Room`: a situation absent from the original authored graph is proposed, validated, committed and revisited after save/load; replay produces identical state |
| M3 Free Input | Free text, intent resolver interface, rule-based and optional model resolver, action validation | Alternative phrasings resolve to valid actions |
| M4 Generative Narrative | Optional LLM-backed narrative proposal provider over M2 dynamic infrastructure; safe context projection, typed events/choices, validation, authored fallback, reproducible recorded proposals | Play a short hybrid story with authored and generated nodes, invalid-proposal rejection and deterministic replay; same game works with a rule-based provider offline |
| M5 NPC Agency | Optional goals, beliefs/knowledge, rule controllers, event-triggered planning | NPC changes plan when it learns relevant information |
| M6 Narrative Direction | Optional milestones, constraints, director proposals | Milestone reached by two different causal paths |
| M7 Visual Presentation | Additional adapter/renderer, asset references, presentation synchronization | Same game logic presented visually without core changes |

M0 establishes architecture boundaries; M2 builds **dynamic narrative infrastructure**, not another presentation adapter. The authored text output adapter remains the reference presentation path. M2 requires no LLM, AI provider, new renderer, or free-text input. Do not scaffold all future modules in M0.

M1.5 does **not** automatically introduce an entity store, location simulation, scene hierarchy, or ECS. First author a second game against existing M1 primitives, then add only minimal, justified support for location/entity references as needed. The existing M1 `Node` may act as an authored narrative scene; no forced breaking schema rename. M2 may add only the persistent world-entity state actually required to validate dynamic proposals; M5 adds optional NPC agency; M6 adds optional progress milestones. The classic CLI game remains valid throughout.

M1.75 establishes a **single temporal model** before dynamic narrative and free-text input. It is independent from M1.5's world-entity model and may reuse the lighthouse test game for demonstration without retrofitting it. The core does not poll the wall clock, sleep or own a frontend timer. Host/runtime drivers supply elapsed time explicitly; adapters render player-visible remaining time and frontend readiness. M2 may propose time-driven story consequences through validated events; M3 may later pair timed offers with free-form intent; M4–M6 may react to time progression for generated narrative, NPC behavior and story direction. No background actor or full scheduling engine is required for M1.75.

M1.8 refines the lifecycle established by M1.75: **finishing presentation is not identical to making a choice available**. A narrative segment may stand alone and advance after an authorized completion acknowledgement, while an interactive segment's decision timer must start only after required storytelling and actual offer readiness. A text CLI should allow the player to acknowledge reading before a deadline starts; visual adapters/renderers will later report completion of dialogue, camera movement, animation or other required atmospheric work through the composition root. The engine remains authoritative over all transitions and deadlines; neither completion events nor scene IDs imply a location entity. M1.8 implements the compatible engine/semantic/presentation lifecycle before M2, with replayable timed storytelling demonstrating both paths. No generic cutscene sequencer, full auto-play graph or renderer-to-engine direct dependency is intended.

## Dynamic generation versus presentation

A **narrative proposal** is suggested before canonical commit: a new scene, authored-style narrative content, possible choices, explicit proposed events/effects and valid successor links. The host or optional generator may produce it, but only the engine can validate and commit it. Narrative prose does not by itself mutate world truth; an external model result must be recorded or resolved for deterministic replay. Generated nodes use the same semantic output, M1.8 presentation gating and text/visual renderers as authored nodes.

A **presentation adapter** translates already-authorized semantic content for a frontend. Optional stylistic rephrasing of fixed facts may be useful, but it is neither a source of new canonical events nor a dedicated milestone. The earlier **M2 Generative Presentation** milestone is retired in favor of M2 Dynamic Narrative Foundation. M4 adds optional generative proposal production only once M2 proposal validation, persistence and deterministic tests exist. The precise generation/validation model is intentionally unresolved; see [open questions](open-questions.md#generative-narrative-and-presentation).

## Delivery rules
- Each requirement has tests and acceptance criteria.
- The classic game remains playable after every milestone.
- Later features do not become mandatory dependencies for earlier experiences.
- Unknown future APIs remain deliberately unspecified.
