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
- **SCENE-01** An authored choice may commit effects and update offers **within the same narrative scene** without a forced node transition or a fixed Inspect/Use/Take taxonomy.
- **SCENE-02** After one resolved choice, offers are derived from committed state; new choices and action-specific feedback may be shown after their required presentation. Stale offers are rejected.
- **SCENE-03** Support a **scene-completion deadline** across multiple successive actions, separate from the existing M1.75 **choice-selection deadline**. Repeated choices do not reset scene time or silently convert a scene objective into a selection deadline.
- **SCENE-04** Extend bounded checking to include scene plus relevant state, conditional offers, time-cost outcomes, reachable exits and intentional timeout branches; keep explicit inconclusive limits.
- **SCENE-05** Playable text demo `The Locked Room`: drawers, wardrobe, keys, fireplace, iron bar, state-driven choices, timed attempts and multiple outcomes. Show success with remaining time and failure when the known scene deadline cuts an otherwise successful door-opening action short.
- **SCENE-06** Preserve old CYOA stories, immediate actions, M1.75/M1.8 lifecycle and versioned saves; keep extensions minimal and regression tested.
- **SCENE-07** An authored choice may have a nonnegative **execution duration**. On a choice input containing host-correlated selection time, GRIMM performs a **single atomic deterministic resolution** using canonical state, authored action duration, scene deadline and interruption/timeout rules. It computes the result, logical time change, effects and subsequent offers **at choice selection**, not at animation completion. No per-action progress inputs, pending game-action phase or renderer outcome callback are needed for authored sequential actions.
- **SCENE-08** A known scene deadline may cut the effective execution interval short: for deadline T=10s, five-second `break_door` selected at T=6s, GRIMM determines **failure/interruption at T=10s with 4/5 seconds of effective progress**, commits its authored conditional outcome immediately, and emits corresponding four-second attempted-action/failure presentation. The nominal T=11s success must not be committed.
- **SCENE-09** Canonical action resolution and player-facing **presentation/readiness** are separate: although GRIMM resolves the result at selection, the next choices must not become interactive until the action's authorized presentation finishes and host readiness has been acknowledged under the M1.8 revision rules. Pause/save/load preserve the canonical resolved state and required unfinished presentation, not a fictitious running action.
- **SCENE-10** End-to-end CLI and fake-time tests exercise a successful five-second drawer-opening action, a door attempt arithmetically cut short by scene expiry, input blocked until authorized presentation completes, pause/save/load during presentation, and unchanged immediate legacy actions.
- **SCENE-11** The presentation sequence's duration must match the **effective action interval** chosen by GRIMM: five seconds for a completed five-second action; four seconds of attempted action when a T=10s deadline interrupts a five-second action selected at T=6s, followed by the authored failure presentation. Renderer latency cannot change resolved game outcomes or cause extra in-game action time.
- **SCENE-12** Preserve two explicitly distinct timeout semantics: Telltale-style choice-selection deadlines resolve on a timely accepted choice or an authored no-response transition; authored scene-completion deadlines require an objective to finish within the scene's time budget, accounting for sequential choice costs. Do not reinterpret old `decision.limit_ms`.
- **SCENE-13** For an action cut short by an **already-known** scene deadline, provide deterministic arithmetic progress (e.g. 4/5) and committed world/character attributes as inputs to an authored conditional result (e.g. detective arrives before the weapon is hidden). Do not model progress as continuous persistent action state or hardcode a detective/detection rule.
- **SCENE-14** Resolve exact-time ties, selection timestamp validation, deadline boundary crossing and source of elapsed idle-time observations deterministically in one choice transition. Inputs for an earlier revision cannot retroactively change a completed result. No autonomous core clock.
- **SCENE-15** M2 demo covers unanswered timed dialogue, successive scene-local actions consuming one scene budget, a deadline-cut action and state/progress-dependent authored outcomes. M3 additionally demonstrates RPG-stat-influenced results.
- **SCENE-16** For idle real-time deadlines **without a choice**, the host still supplies externally observed elapsed time as in M1.75; this is separate from action execution. Tests confirm idle expiry and that a choice at T=6s can be resolved using its complete authored duration and T=10s deadline **without further host reports**.
- **SCENE-17** Emit a committed semantic state projection / valid offers so renderers can reconstruct persistent game facts (such as an open drawer or revealed keychain) after presentation, revisit and save/load. Animation-local state is not canonical.
- **SCENE-18** Renderer/input adapters submit **choice selection and correlated time** only, never a success/failure verdict. GRIMM returns the fully resolved authored outcome, semantic presentation sequence and next eligible choices, with input gating until presentation readiness.
- **SCENE-20** Support **repeated authored in-scene attempts** where each independently selected Choice atomically updates persistent progress and reoffers itself or reveals alternatives. Authoring conditions can combine that progress with existing typed values (later optional M3 character stats); the renderer does not decide progress or success. Illustrative door example: `strength` may require one, two or three `break_door` selections, each with an authored time cost, under one non-resetting scene deadline.
- **SCENE-21** GRIMM chooses a **renderer-neutral presentation cue per resolved attempt**, e.g. `try_break_door` for failed/unfinished attempts and `success_break_door` for the final success; authored variants may depend on `strength` or `door_break_progress` to visually signal likely success. These identifiers are example authored cues, not mandatory engine action verbs, asset IDs or universal RPG rules. The next Choice is gated until the selected presentation is complete.
- **SCENE-22** Demonstrate the multi-attempt door sequence in a playable M2 test with simple state variables, different number of attempts, distinct presentation cues, cumulative scene-time consumption, deadline cutoff and save/load persistence. M3 can revisit the same demo with properly typed RPG-style character attributes.
- **SCENE-19** Do **not** introduce provisional `will_succeed_if_completed` gameplay outcomes for deterministically resolvable M2 actions. At T0, return the already-determined success/failure/timeout branch and presentation appropriate to it. Late or independent changes arising from external physics/host actors are future integration concerns rather than required M2 action-progress simulation.

## M3 — Inventory, Puzzles & Character State
- **PUZZLE-01** Optional items, discoveries, ownership and inventory-based conditions/effects; avoid an ECS or obligatory item taxonomy.
- **PUZZLE-02** Allow authored combinations (including MacGyver-like solutions) through ordinary contextual choices with explicit preconditions and state effects; items are not required to expose universal `Use` or `Combine` commands.
- **PUZZLE-03** Model multi-step puzzles, alternate solutions, failed attempts and optional action costs using the existing state/choice/time model. Collected or consumed items affect future offers and narrative branches.
- **STAT-01** Optional persistent RPG-style character attributes, skills, relationships and resources are typed game state used in choice eligibility and effects. No mandatory level, class, dice or combat engine.
- **STAT-02** Define optional attribute thresholds/checks and their deterministic resolution where a game requires them; allow authored outcomes to combine **action-interruption progress** with committed attributes (e.g. suspicion for detection). Distinguish player-visible stats from private NPC/world values and preserve replay.
- **PUZZLE-04** Playable authored puzzle demo with collected and combined items plus at least two attribute-dependent approaches, alternative results, **a timed assembly/combination action with blocked gameplay input and duration-matched feedback**, save/load and bounded checker coverage.

## M4 — Narrative Adventure
- **ADV-01** Support semi-linear exploration across persistent locations with multiple narrative situations per location, revisiting and state-dependent offers. Do not equate location, scene and story progress.
- **ADV-02** Combine LucasArts-like discovery/inventory puzzles with Telltale-like staged narrative choices, cinematic pacing, timed decisions and persistent consequences.
- **ADV-03** State and puzzle solutions can enable/disable authored narrative branches without graph variant explosion; different approaches should produce distinct meaningful outcomes.
- **ADV-04** Deliver a complete short hybrid narrative adventure with multiple locations, linked puzzles, Telltale-style consequential scenes, at least three outcomes, **timed multi-step actions** and playability tests.

## M5 — Graphic Adventure Presentation
- **VIS-01** Add a graphical adapter/renderer for the existing canonical action and choice semantics, with location imagery and scene-specific clickable hotspots/menus.
- **VIS-02** Render inventory, persistent world-object state (open/closed drawers, discovered/removed items) and optional character stats as views of authorized committed state; re-render, revisiting and save/load must rebuild correct visual state independent of renderer-local interaction history.
- **VIS-03** Synchronize required dialogue/animation/camera presentation and interaction readiness with M1.8 lifecycle; preserve deadlines and no early activation.
- **VIS-05** A complete game composition provides the **actual renderer/runtime implementation and origin**, configured assets, hotspots and resources. In a standalone GRIMM-led game its game definition may select that renderer; in an embedded host game (e.g. Bevy), the external host may already own it and instantiate GRIMM. No assumption of arbitrary renderer compatibility or single required entry-point direction.
- **VIS-06** Keep game-specific rendering resources and semantic choice-to-hotspot bindings **outside headless engine internals**. Derive packaging, source and version contracts from complete M5 games, including external-host integration; do not force the narrative story graph to instantiate the external world's renderer.
- **VIS-04** Deliver a fully playable **graphical game composition** with its concrete renderer, specific assets, semantic choice bindings, authored scenes, correct choice-resolution/presentation-readiness input gating and duration-matched animation. Do not require an arbitrary text game to run in 3D, nor mandate text/graphic parity for every game; engine rules stay renderer-independent.

- **VIS-07** Verify game-level composition diagnostics for missing/invalid renderer/runtime integration, assets, choice/scene bindings and version mismatches, appropriate to the selected host. Preserve room for game-specific components without committing to a general-purpose plugin platform in M5.
- **VIS-08 (investigation, not a required product mode)** Evaluate a minimal **host-owned runtime integration** spike: a world-location/trigger event requests GRIMM-authored narrative interaction; GRIMM validates and resolves choices; an explicit result returns to the host for a world consequence. Document feasibility, state/clock/ownership, save/replay risks, and whether embedding should become a supported mode. Do not presume GRIMM is the mandatory application entry point or promise a general host API before the spike.

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
