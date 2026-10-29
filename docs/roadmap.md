# Roadmap

Every milestone ends with an executable demo and regression tests. Milestones describe capabilities, not mandatory features for every game.

| Milestone | Scope | Acceptance demo |
|---|---|---|
| M0 Architecture Foundation | All core boundaries, minimal semantic messages, stub engine/adapter/renderer loop, dependency tests | End-to-end action -> engine -> adapter -> renderer -> input -> engine |
| M1 Classic CYOA | Authored graph, choices, conditions/effects, authored text adapter, CLI, save/load | Play `The Last Witness` through multiple endings offline |
| M2 Generative Presentation | Second text adapter using optional LLM, visible-context projection, exact text preservation, deterministic fallback | Same story in authored and runtime-narrated presentation |
| M3 Free Input | Free text, intent resolver interface, rule-based and optional model resolver, action validation | Alternative phrasings resolve to valid actions |
| M4 Dynamic Narrative | Validated dynamic events/nodes, runtime graph overlay, persistence | Unscripted action leads to a new playable event/node |
| M5 NPC Agency | Optional goals, beliefs/knowledge, rule controllers, event-triggered planning | NPC changes plan when it learns relevant information |
| M6 Narrative Direction | Optional milestones, constraints, director proposals | Milestone reached by two different causal paths |
| M7 Visual Presentation | Additional adapter/renderer, asset references, presentation synchronization | Same game logic presented visually without core changes |

M0 must establish architecture boundaries; M2 is **not** a belated adapter refactor. Only one simple text adapter is needed in M1. Do not scaffold all future modules in M0.

## Delivery rules
- Each requirement has tests and acceptance criteria.
- The classic game remains playable after every milestone.
- Later features do not become mandatory dependencies for earlier experiences.
- Unknown future APIs remain deliberately unspecified.
