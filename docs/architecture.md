# Architecture — initial boundary contract

This document fixes **dependency boundaries**, not all future APIs. Public types will evolve through milestones.

## Flow

```text
Game definition -> Engine (state/rules/graph) -> semantic output
                                                 |
                                                 v
                                         Output adapter
                                                 |
                                                 v
                                        Presentation commands
                                                 |
                                                 v
                                             Renderer
                                                 |
                                          Raw player input
                                                 |
                                                 v
                                           Input adapter
                                                 |
                                                 v
                                        Semantic game action -> Engine
```

A composition root wires concrete components. Engine modules must never import adapters, renderers, UI frameworks, or LLM SDKs.

## Domain ownership
- **Engine:** authoritative world/game state; validates actions; commits effects; records events; graph traversal.
- **Semantic protocol:** typed actions, committed game events, narrative content, currently valid interaction offers, and player-visible projections. These are related but not interchangeable.
- **Output adapter:** maps semantic output to presentation commands; may hold a scene graph or call an LLM; cannot modify game truth.
- **Renderer:** displays presentation commands (text, comic, 3D) and reports raw user interactions / presentation completion.
- **Input adapter:** converts raw UI input or text into proposed semantic actions; engine validates proposals.
- **Composition root:** chooses engine capabilities, adapters, renderers and transports.

## Event and presentation semantics
Committed game events describe what actually happened, including offscreen events. They are not automatically visible to players. Narrative content describes what may be communicated; interaction offers describe currently valid inputs. A filtered projection exposes only player-visible information. A renderer must not receive hidden world state by default.

Authored text and semantically described actions are both valid narrative inputs. An authored text adapter can preserve exact prose; a generative text adapter may produce stylistic presentation without introducing canonical facts. Generated content is non-authoritative.

## State and lifecycle
- Engine state and presentation state are distinct.
- Adapters may be stateful; a filtered snapshot enables reconnection and adapter initialization.
- Input references an active interaction/session revision; stale choices are rejected.
- Presentation completion may be acknowledged, but simulation must not depend on rendering speed except where explicitly synchronized.
- State changes and emitted events are committed atomically; event-driven follow-up processing must be bounded and deterministic.
- For deterministic replay, record external model results or resolved actions rather than assuming an LLM can reproduce output.

## Extension model
Optional graph dynamics, intent resolution, NPC controllers, and narrative direction integrate through explicit engine interfaces and validated action/event processing. Do not implement a general-purpose plugin framework in M0.

## Open design questions
Exact Rust protocol types; async transport conventions; story file schema; versioning and migration policy; persistence format; adapter snapshot granularity; generated-text validation strategy. Resolve incrementally with tests and ADRs.
