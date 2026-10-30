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

## M1.75 — Temporal Foundation
- **TIME-01** Optional, authoritative game/world time: the engine owns the canonical temporal state while the host provides explicit elapsed-time or clock-advance inputs. Deterministic tests and replay must not rely on reading the system clock inside the core.
- **TIME-02** Timed interactions: an interaction may declare a decision deadline, with a deterministic, validated timeout consequence (such as an authored default action or distinct timeout transition). Decisions and timeout must resolve at most once for an interaction revision.
- **TIME-03** Time advancement: story actions may advance world time by a defined duration; temporal predicates can affect eligible choices and trigger authored, validated consequences. Real elapsed time and world-time advancement are separately configurable.
- **TIME-04** Temporal lifecycle: define when deadlines start, pause/resume behavior, save/restore semantics, and coordination with presentation readiness without depending on render speed or frontend-specific APIs.
- **TIME-05** Time-driven world behavior is an optional future capability: modules/NPCs may react to time progression via normal validated engine transitions. No always-running real-time simulator is required in M1.75.

## M1.8 — Narrative Flow & Presentation Gating
- **FLOW-01** Narrative-only segments: an authored scene/segment may present narrative content without offering an artificial player choice. Completion may advance to an authored successor through a validated engine-owned transition after an explicit host acknowledgement; never equate a physical location with a scene or require a new generic scene graph.
- **FLOW-02** Presentation-gated interactions: time-limited decisions remain inactive until the required narrative/atmospheric presentation has finished **and** the interaction is actually available. Rendering/flush alone is not proof of human reading or completion of dialogue, audio, animation or camera motion. The host turns a renderer/presentation-completion acknowledgement into a revision-scoped engine lifecycle input; no renderer-to-engine dependency.
- **FLOW-03** Separate presentation completion from decision readiness: distinguish completion of the narrative segment from availability of interaction offers. A countdown starts only on the engine-validated interaction-ready event, never on node entry, early drawing of choices or a slow frontend. Duplicate, stale or out-of-order acknowledgements cannot skip content or restart a timer.
- **FLOW-04** Safe flow and persistence: automatic/non-choice transitions remain engine-authoritative, validate targets/effects, commit at most once per revision and have bounded/cycle-safe advancement. Save/load, pause/resume, disconnect and replay preserve unfinished presentation and pending deadlines; a failed/interrupted renderer must not silently progress canonical story state.
- **FLOW-05** Renderer-neutral authoring and fallback: support explicit player-paced Continue as well as optional host-acknowledged narrative-only progression. In the text CLI, reading is acknowledged by the player (e.g. Enter) where needed; later visual renderers may acknowledge completed dialogue/camera sequences. Do not introduce renderer-authored timings, a cutscene timeline, ECS or a generic scheduler as a prerequisite.

## M2 — Dynamic Narrative Foundation (no LLM dependency)
- **DYN-01** Allow external, typed runtime narrative node/event proposals. Engine validates proposed node identity, references, permitted content, transitions, conditions/effects and current state before any atomic commit. Providers have no canonical state authority.
- **DYN-02** Maintain a persisted dynamic graph overlay alongside the authored graph. Resolve authored and dynamic destinations consistently without rewriting the original story definition. Preserve existing scenes, M1.8 lifecycle, deadlines and hidden-state boundaries.
- **DYN-03** Record accepted proposals/resolved events sufficiently for deterministic replay and save/restore. Reject invalid, stale, conflicting or duplicate proposals without partial mutation; bounded playability diagnostics retain explicit inconclusive outcomes.
- **DYN-04** Demonstrate M2 with a small complete offline game (working title `The Unwritten Room`) and a deterministic rule-based proposal source; a runtime-created node must lead to a meaningful choice and reachable endings, survive save/load and replay identically.
- **WORLD-03** When dynamic proposals require persistent world-entity state, support only the minimal validated state changes shared across scenes; defer a generic entity store, ECS and continuous simulation.

## M3 — Free Input
- **ACT-01** Free-text input offers and intent resolver interface.
- **ACT-02** Rule-based resolver and optional model implementation.
- **ACT-03** Validated proposed actions, ambiguity and failure responses.

## M4 — Generative Narrative (optional)
- **GEN-06** Provide an optional generative narrative proposal source that targets the M2 typed proposal contract. Model output is never canonical until the engine validates and commits it.
- **GEN-07** Give proposal providers an explicitly scoped, privacy-safe context projection. Hidden NPC knowledge and unavailable game facts must not leak through prompts, presentation, traces or replay artifacts.
- **GEN-08** Validate explicit proposed world events/effects and narrative transitions; prose alone must not create or silently alter canonical facts. Define bounded rejection, retry and safe authored/rule-based fallback behavior.
- **GEN-09** Preserve authored dialogue and narrative constraints where required; accept generated narrative only under documented consistency/trust boundaries. Do not equate successful JSON/schema parsing with semantic truth validation.
- **GEN-10** Record model proposals or resolved inputs for deterministic replay; demonstrate optional online and fully offline paths using the same engine and output adapters.

### Deferred legacy requirements from the former M2
The original IDs below are **retained as historical proposals**, not silently reassigned to new meanings or required by the revised roadmap. Resolve their fate explicitly through [the open questions](open-questions.md#generative-narrative-and-presentation) before any implementation.
- **GEN-01 (deferred)** Optional generative text adapter behind the existing presentation boundary.
- **GEN-02 (deferred)** Safe player-visible context projection for that text adapter.
- **GEN-03 (deferred)** Exact authored dialogue preservation in generative rendering.
- **GEN-04 (deferred)** Deterministic authored/plain-text fallback on text-model failure.
- **GEN-05 (deferred)** Best-effort validation against canonical-fact invention by generated presentation prose.

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
