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

## M2 — Interactive Scene Foundation
- **SCENE-01** Authored choice actions may commit a validated state change **within the current narrative scene**, without a forced graph node transition or a mandatory fixed `Inspect/Use/Take` action taxonomy.
- **SCENE-02** Current interaction offers are recalculated after an in-scene action from committed state. An authored action can reveal, enable, disable or replace choices and provide fresh narrative feedback; invalid and stale offers remain non-executable.
- **SCENE-03** A time-critical situation may span multiple in-scene actions. A successful search, failed key attempt, inventory operation or newly exposed offer must not silently reset its shared deadline. Preserve M1.75 timeout precedence and M1.8 presentation/interaction readiness, pause, restore and revision safety.
- **SCENE-07** An authored choice may declare optional **execution duration**. On accepting an action the engine enters an authoritative action-in-progress state and suppresses conflicting gameplay input until the action resolves. The headless engine never sleeps, waits for the renderer or reads wall time; the host sends elapsed observations. Zero-duration legacy choices retain immediate behavior.
- **SCENE-08** The ongoing situation deadline continues during action execution unless a separate, explicit authored rule says otherwise. A timeout can occur before execution completes; define and test deterministic precedence at the exact boundary, one-time resolution and whether completion effects are committed only after successful execution. Revisions, stale actions and late completion observations cannot double-commit or bypass expiry.
- **SCENE-11** The player-visible action animation or progress indicator must use the **same authored execution-duration interval** as the authoritative in-progress action: a 5-second action has 5 seconds of animation/progress, with no extra presentation wait extending the action or scene deadline. Renderer delay must catch up/skip frames rather than prolong the canonical action; shorter/longer assets are fitted, trimmed or rejected. Keep pre-action M1.8 presentation gating separate from in-action playback; test timing equivalence and pause/resume synchronization across text and visual hosts.
- **SCENE-09** Pause/resume and save/load preserve or intentionally reconcile in-progress action state, remaining execution time and the unchanged situation deadline. Presentation progress feedback is distinct from canonical action completion; application/session commands remain operable while gameplay choices are blocked.
- **SCENE-10** End-to-end CLI demo and deterministic clock tests must show a long key attempt (input unavailable during execution), a successful action finishing before the deadline, an attempted door-forcing action that misses the deadline during execution, interrupted/paused/restored execution, and backward-compatible immediate choices.
- **SCENE-04** Extend bounded diagnostics to reason about scene+relevant state, reachable options and executable exits/intentional timeouts, not only graph node links. Identify accidental nonterminal deadlocks; retain explicit inconclusive limits.
- **SCENE-05** Playable text demo `The Locked Room`: search drawers/wardrobe/fireplace, discover a keyring and metal bar, attempt multiple wrong keys, change available choices and resolve an escape under an already-running deadline with distinct outcomes. Include visible input-locking action durations and timeout during an attempted action.
- **SCENE-06** Keep ordinary authored CYOA stories and current save formats working; any optional authoring/schema extensions must be minimal, documented, deterministic and regression tested.

## M3 — Inventory, Puzzles & Character State
- **PUZZLE-01** Optional items, discoveries, ownership and inventory-based conditions/effects; avoid an ECS or obligatory item taxonomy.
- **PUZZLE-02** Allow authored combinations (including MacGyver-like solutions) through ordinary contextual choices with explicit preconditions and state effects; items are not required to expose universal `Use` or `Combine` commands.
- **PUZZLE-03** Model multi-step puzzles, alternate solutions, failed attempts and optional action costs using the existing state/choice/time model. Collected or consumed items affect future offers and narrative branches.
- **STAT-01** Optional persistent RPG-style character attributes, skills, relationships and resources are typed game state used in choice eligibility and effects. No mandatory level, class, dice or combat engine.
- **STAT-02** Define optional attribute thresholds/checks and their deterministic resolution where a game requires them; distinguish player-visible stats from private NPC/world values and preserve replay.
- **PUZZLE-04** Playable authored puzzle demo with collected and combined items plus at least two attribute-dependent approaches, alternative results, **a timed assembly/combination action with blocked gameplay input and duration-matched feedback**, save/load and bounded checker coverage.

## M4 — Narrative Adventure
- **ADV-01** Support semi-linear exploration across persistent locations with multiple narrative situations per location, revisiting and state-dependent offers. Do not equate location, scene and story progress.
- **ADV-02** Combine LucasArts-like discovery/inventory puzzles with Telltale-like staged narrative choices, cinematic pacing, timed decisions and persistent consequences.
- **ADV-03** State and puzzle solutions can enable/disable authored narrative branches without graph variant explosion; different approaches should produce distinct meaningful outcomes.
- **ADV-04** Deliver a complete short hybrid narrative adventure with multiple locations, linked puzzles, Telltale-style consequential scenes, at least three outcomes, **timed multi-step actions** and playability tests.

## M5 — Graphic Adventure Presentation
- **VIS-01** Add a graphical adapter/renderer for the existing canonical action and choice semantics, with location imagery and scene-specific clickable hotspots/menus.
- **VIS-02** Render inventory and optional character stats as views of authorized semantic state, never as a second game rules engine.
- **VIS-03** Synchronize required dialogue/animation/camera presentation and interaction readiness with M1.8 lifecycle; preserve deadlines and no early activation.
- **VIS-04** Demonstrate equivalent narrative/puzzle outcomes **and authoritative action-in-progress/input-lock behavior and duration-matched animation** for the same adventure in text and graphical frontends, without engine-specific UI logic.

## Playable demo acceptance rule
- **DEMO-01** Each new milestone M2–M5 ships a complete, actually playable reference story (or complete playable expansion of the prior game) demonstrating **all** gameplay mechanics introduced by that milestone, not merely synthetic tests. Include clear start/run instructions, reproducible successful and failed paths, relevant state/choice visibility, save/load coverage and an authoring/playtest report.
- **DEMO-02** Automated unit/integration/CLI (or later visual-host) regressions independently verify the same capabilities; bounded checker results distinguish deadlock, valid ending and inconclusive exploration. Existing demo stories and legacy behavior remain working.

## Deferred optional extensions — no near-term milestone commitment
- **ACT-01 (deferred)** Free-text input offers and intent resolver interface.
- **ACT-02 (deferred)** Rule-based resolver and optional model implementation.
- **ACT-03 (deferred)** Validated proposed actions, ambiguity and failure responses.
- **WORLD-03 (deferred)** Validated persistent world-entity state shared across scenes when justified by actual authoring, without a generic entity store by default.
- **DYN-01 (deferred)** Provider-neutral, engine-validated runtime narrative node/event proposals, possibly from a live human DM, authoring tool, rule provider or optional LLM.
- **DYN-02 (deferred)** Persisted runtime graph overlay and graph-extension validation, including reachable and safely escapable narrative frontiers where appropriate.
- **DYN-03 (deferred)** Recorded provider proposals and deterministic replay, revision-bound atomic commits and no partial graph mutation.
- **DYN-04 (deferred)** Complete future story-RPG demo with a human DM or other provider extending the graph during play.
- **GEN-01 (deferred)** Optional generative text adapter behind the existing presentation boundary.
- **GEN-02 (deferred)** Safe player-visible context projection for generated text.
- **GEN-03 (deferred)** Exact authored dialogue preservation in generative rendering.
- **GEN-04 (deferred)** Deterministic authored/plain-text fallback on text-model failure.
- **GEN-05 (deferred)** Best-effort validation against canonical-fact invention by generated presentation prose.
- **GEN-06 (deferred)** Optional generative narrative provider targeting a validated dynamic proposal contract.
- **GEN-07 (deferred)** Privacy-safe context projection for narrative proposal sources.
- **GEN-08 (deferred)** Validated explicit proposed world events/effects and safe fallback.
- **GEN-09 (deferred)** Consistency and author constraints on generative narrative.
- **GEN-10 (deferred)** Recorded model outputs/proposals for deterministic replay.
- **NPC-01 (deferred)** NPC state, knowledge/beliefs and goals.
- **NPC-02 (deferred)** Rule-based optional controllers and event-triggered actions.
- **NPC-03 (deferred)** Optional validated AI planning.
- **NAR-01 (deferred)** Declarative milestones and conditions.
- **NAR-02 (deferred)** Optional director and hard/soft constraints.
- **NAR-03 (deferred)** Alternate-path milestone tests.
- **NAR-04 (deferred)** Narrative progress independent of current scene/location.

Previously used requirement IDs remain traceable and are not silently reassigned. See the [open-points catalog](open-questions.md) before reviving deferred features.
