# Requirements catalog (initial)

IDs are stable references for issues and acceptance tests. Detailed API shapes are intentionally deferred.

## Cross-cutting architecture
- **ARCH-001** Headless engine; no renderer or input technology dependency.
- **ARCH-002** Explicit semantic boundary and adapter layer in M0.
- **ARCH-003** Engine solely owns canonical state and commits validated changes.
- **ARCH-004** Presentation state belongs to adapters/renderers.
- **ARCH-005** AI is optional and non-authoritative.
- **ARCH-006** Static graph and dynamic features share engine action/event/state primitives.
- **ARCH-007** Player-visible projections exclude hidden information.
- **ARCH-008** Deterministic core transitions and testable replay with recorded external results.
- **ARCH-009** World entities (for example characters, locations and objects) have stable identities and potentially persistent state independent of narrative scenes.
- **ARCH-010** Narrative scenes and world entities are distinct: scenes may reference multiple entities, and entities may appear in multiple scenes; neither location nor scene identity defines story progress. These are conceptual boundaries, not a requirement for inheritance, an ECS or a universal entity store.

## M0
- **FND-01** Rust workspace and project conventions.
- **FND-02** Engine/semantic protocol/adapter/presentation protocol/renderer boundaries.
- **FND-03** Minimal action, semantic output, interaction and presentation command types.
- **FND-04** End-to-end executable loop using stub components.
- **FND-05** Boundary/compile-time dependency tests and CI.
- **FND-06** Architecture documentation and agent instructions.

## M1
- **CYOA-01** Authored story node/transition model.
- **CYOA-02** YAML story loading and validation.
- **CYOA-03** Choices and currently valid interaction offers.
- **CYOA-04** State variables, conditions and effects.
- **CYOA-05** Authored text output adapter.
- **CYOA-06** CLI renderer and input adapter.
- **CYOA-07** Save/load with schema version.
- **CYOA-08** Multiple endings and regression demo.

## M1.5 — World modeling and dogfooding
- **CYOA-09** Narrative deadlock handling and story playability: distinguish intentional endings from unintended multi-step, state-dependent nonterminal deadlocks; provide actionable authoring diagnostics and explicit runtime behavior. Preserve the guarantee that offered choices are immediately executable. Full global solvability analysis is **not** required. Track implementation and acceptance tests in [issue #4](https://github.com/rjtg/grimm/issues/4).
- **WORLD-01** Explore stable world-entity identities across scenes, including at least one character and one location, without implementing a generic entity framework by default.
- **WORLD-02** Demonstrate three distinct authored narrative scenes associated with the same physical/logical location; scene content and choices may differ with the narrative situation and game state. Revisiting the *same graph node* is not required.

## M2
- **GEN-01** Optional generative text adapter behind same boundary.
- **GEN-02** Safe player-visible context projection.
- **GEN-03** Exact authored dialogue preservation.
- **GEN-04** Deterministic authored/plain-text fallback on model failure.
- **GEN-05** Tests for prohibited canonical fact invention and fallback behavior (best-effort semantic validation).

## M3
- **ACT-01** Free-text input offers and intent resolver interface.
- **ACT-02** Rule-based resolver and optional model implementation.
- **ACT-03** Validated proposed actions, ambiguity and failure responses.

## M4
- **WORLD-03** When persistent world-entity state becomes necessary, support validated state changes shared across scenes (and persistence); defer general-purpose components and simulation unless required by an accepted milestone.
- **DYN-01** Validated runtime event and node proposals.
- **DYN-02** Runtime graph overlay with persistence.
- **DYN-03** Reproducible state transitions and event history.

## M5
- **NPC-01** NPC state, knowledge/beliefs, goals.
- **NPC-02** Rule-based optional controllers and event-triggered actions.
- **NPC-03** Optional validated AI planning.

## M6
- **NAR-01** Declarative milestones and conditions.
- **NAR-02** Optional director and hard/soft constraints.
- **NAR-03** Alternate-path milestone tests.
- **NAR-04** Narrative progress can be represented independently of location and current scene/graph position; use world state and optional explicit milestones without requiring a second parallel progress graph.

## M7
- **VIS-01** Visual adapter/renderer implementation.
- **VIS-02** Presentation-state snapshots, synchronization and acknowledgements.
- **VIS-03** Visual/spatial interaction mapped to semantic actions.
