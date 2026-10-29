# Roadmap

Every milestone ends with an executable demo and regression tests. Milestones describe capabilities, not mandatory features for every game.

| Milestone | Scope | Acceptance demo |
|---|---|---|
| M0 Architecture Foundation | All core boundaries, minimal semantic messages, stub engine/adapter/renderer loop, dependency tests | End-to-end action -> engine -> adapter -> renderer -> input -> engine |
| M1 Classic CYOA | Authored graph, choices, conditions/effects, authored text adapter, CLI, save/load | Play `The Last Witness` through multiple endings offline |
| M1.5 World Modeling & Dogfooding | Build a second independently authored test story; exercise stable characters/locations across different scenes; diagnose narrative deadlocks (CYOA-09); capture authoring friction | Play three distinct scenes at one location, with state-driven choices and at least three endings; publish authoring findings |
| M2 Generative Presentation | Second text adapter using optional LLM, visible-context projection, exact text preservation, deterministic fallback | Same story in authored and runtime-narrated presentation |
| M3 Free Input | Free text, intent resolver interface, rule-based and optional model resolver, action validation | Alternative phrasings resolve to valid actions |
| M4 Dynamic Narrative | Validated dynamic events/nodes, runtime graph overlay, persistence | Unscripted action leads to a new playable event/node |
| M5 NPC Agency | Optional goals, beliefs/knowledge, rule controllers, event-triggered planning | NPC changes plan when it learns relevant information |
| M6 Narrative Direction | Optional milestones, constraints, director proposals | Milestone reached by two different causal paths |
| M7 Visual Presentation | Additional adapter/renderer, asset references, presentation synchronization | Same game logic presented visually without core changes |

M0 must establish architecture boundaries; M2 is **not** a belated adapter refactor. Only one simple text adapter is needed in M1. Do not scaffold all future modules in M0.

M1.5 does **not** automatically introduce an entity store, location simulation, scene hierarchy, or ECS. First author a second game against existing M1 primitives, then add only minimal, justified support for location/entity references as needed. The existing M1 `Node` may act as an authored narrative scene; no forced breaking schema rename. M4 may extend persistent world-entity state; M5 adds optional NPC agency; M6 adds optional progress milestones. The classic CLI game remains valid throughout.

## Delivery rules
- Each requirement has tests and acceptance criteria.
- The classic game remains playable after every milestone.
- Later features do not become mandatory dependencies for earlier experiences.
- Unknown future APIs remain deliberately unspecified.
