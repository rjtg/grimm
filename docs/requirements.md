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

## M7
- **VIS-01** Visual adapter/renderer implementation.
- **VIS-02** Presentation-state snapshots, synchronization and acknowledgements.
- **VIS-03** Visual/spatial interaction mapped to semantic actions.
