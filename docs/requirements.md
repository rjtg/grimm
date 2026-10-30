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
- **SCENE-01** An authored choice may commit effects and update offers **within the same narrative context** without a forced story-graph node transition or a fixed Inspect/Use/Take taxonomy. Distinguish narrative situation/context, authored graph node and renderer-specific visual shot/setting; their boundaries are not inherently identical.
- **SCENE-02** After one resolved choice, GRIMM derives offered choices and engine-selected semantic presentation cues/state projections from committed state; the renderer is responsible for **how** to present them. New choices may become interactive only after required presentation/readiness; stale revision-scoped offers are rejected without restarting the narrative context or its deadline.
- **SCENE-03** Support a **scene-completion deadline** across multiple successive actions, separate from the existing M1.75 **choice-selection deadline**. Repeated choices do not reset scene time or silently convert a scene objective into a selection deadline.
- **SCENE-04** Extend bounded checking beyond static graph reachability to include authored context/node, relevant committed state and time budget; follow **only explicitly authored legal transitions**. Do not infer return to earlier nodes merely because they have already been visited: a Telltale-like locked door needs an authored `leave_and_search_for_key` choice if the key rack remains reachable. Detect trapped nonterminal states, preserve authored failure/timeout endings, and return *inconclusive* when bounded analysis cannot establish solvability. An unconditional exit from every node is not mandatory.
- **SCENE-05** Playable text demo `The Locked Room`: drawers, wardrobe, keys, fireplace, iron bar, state-driven choices, timed attempts and multiple outcomes. Show success with remaining time and failure when the known scene deadline cuts an otherwise successful door-opening action short.
- **SCENE-06** Preserve old CYOA stories, immediate actions, M1.75/M1.8 lifecycle and versioned saves; keep extensions minimal and regression tested.
- **SCENE-07** An authored choice may have a nonnegative **execution duration**. On a choice input containing host-correlated selection time, GRIMM performs a **single atomic deterministic resolution** using canonical state, authored action duration, scene deadline and interruption/timeout rules. It computes the result, logical time change, effects and subsequent offers **at choice selection**, not at animation completion. No per-action progress inputs, pending game-action phase or renderer outcome callback are needed for authored sequential actions.
- **SCENE-08** A known scene deadline may cut the effective execution interval short: for deadline T=10s, five-second `break_door` selected at T=6s, GRIMM determines **failure/interruption at T=10s with 4/5 seconds of effective progress**, commits its authored conditional outcome immediately, and emits corresponding four-second attempted-action/failure presentation. The nominal T=11s success must not be committed.
- **SCENE-09** Canonical action resolution and presentation/readiness are separate: GRIMM resolves a selected action immediately, but next choices remain unavailable until authorized cues finish and existing M1.8 readiness rules permit input. Preserve already supported M1.75/M1.8 save/pause/replay behavior; do **not** mandate a new M2 checkpoint of unfinished cue playback.
- **SCENE-10** CLI/fake-time tests exercise resolved drawer opening, a door attempt arithmetically cut short by a scene deadline, input gated until presentation readiness, and legacy immediate-action plus existing save/pause regressions. Save/reload **during unfinished M2 cue playback** is not a new M2 acceptance criterion.
- **SCENE-11** The presentation sequence's duration must match the **effective action interval** chosen by GRIMM: five seconds for a completed five-second action; four seconds of attempted action when a T=10s deadline interrupts a five-second action selected at T=6s, followed by the authored failure presentation. Renderer latency cannot change resolved game outcomes or cause extra in-game action time.
- **SCENE-12** Preserve two explicitly distinct timeout semantics: Telltale-style choice-selection deadlines resolve on a timely accepted choice or an authored no-response transition; authored scene-completion deadlines require an objective to finish within the scene's time budget, accounting for sequential choice costs. Do not reinterpret old `decision.limit_ms`.
- **SCENE-13** For an action cut short by an **already-known** scene deadline, provide deterministic arithmetic progress (e.g. 4/5) and committed world/character attributes as inputs to an authored conditional result (e.g. detective arrives before the weapon is hidden). Do not model progress as continuous persistent action state or hardcode a detective/detection rule.
- **SCENE-14** Resolve exact-time ties, selection timestamp validation, deadline boundary crossing and source of elapsed idle-time observations deterministically in one choice transition. Inputs for an earlier revision cannot retroactively change a completed result. No autonomous core clock.
- **SCENE-15** M2 demo covers unanswered timed dialogue, successive scene-local actions consuming one scene budget, a deadline-cut action and state/progress-dependent authored outcomes, including optional lightweight attribute checks. No M3 RPG framework is required.
- **SCENE-16** For idle real-time deadlines **without a choice**, the host still supplies externally observed elapsed time as in M1.75; this is separate from action execution. Tests confirm idle expiry and that a choice at T=6s can be resolved using its complete authored duration and T=10s deadline **without further host reports**.
- **SCENE-17** GRIMM retains authoritative game variables (e.g. `drawer_open`, `door_damage`) and derives offered actions and semantic `cue` entries from them. **M2 Cue Sheets do not require a world-state snapshot.** Renderer presentation state is separate; reconstruction/saving graphical presentation belongs to its renderer and the game's later save integration.
- **SCENE-18** Renderer/input adapters submit **choice selection and correlated time** only, never a success/failure verdict. GRIMM returns the fully resolved authored outcome, semantic presentation sequence and next eligible choices, with input gating until presentation readiness.
- **SCENE-20** Support **repeated authored in-scene attempts** where each independently selected Choice atomically updates persistent progress and reoffers itself or reveals alternatives. Authoring conditions can combine that progress with existing typed values (optional authored character-like attributes); the renderer does not decide progress or success. Illustrative door example: `strength` may require one, two or three `break_door` selections, each with an authored time cost, under one non-resetting scene deadline.
- **SCENE-21** GRIMM chooses a **renderer-neutral presentation cue per resolved attempt**, e.g. `try_break_door` for failed/unfinished attempts and `success_break_door` for the final success; authored variants may depend on `strength` or `door_break_progress` to visually signal likely success. These identifiers are example authored cues, not mandatory engine action verbs, asset IDs or universal RPG rules. The next Choice is gated until the selected presentation is complete.
- **SCENE-22** Demonstrate the multi-attempt door sequence in a playable M2 test with simple state variables, different number of attempts, distinct presentation cues, cumulative scene-time consumption and deadline cutoff. Require the full authored trace and branching example in [M2 Cue Sheet / Game Action reference](milestones/m2-cue-sheet-reference.md): `door_damage += character_strength`, `squeeze_through` conditional availability and dexterity check. A future authored game may revisit the same demo with typed RPG-style character attributes; do not introduce a new M2 save/load lifecycle merely for this fixture.
- **SCENE-19** Do **not** introduce provisional `will_succeed_if_completed` gameplay outcomes for deterministically resolvable M2 actions. At T0, return the already-determined success/failure/timeout branch and presentation appropriate to it. Late or independent changes arising from external physics/host actors are future integration concerns rather than required M2 action-progress simulation.

- **SCENE-23** Use a **Cue Sheet** as the working conceptual description of GRIMM's single resolved semantic response, carrying ordered `cue` entries, offered `actions`, optional visible `timer` and revision. Example: `cue: [enter_corridor, dialog_scene_1]; actions: [A,B,C]; timer: 20s`. Precise serialized format remains open.
- **SCENE-24** After required presentation is complete and choices are available, the renderer starts the associated visible countdown. A selection five seconds later produces the equivalent of `[action_selected(A), timer_elapsed(5s)]`; no selection through expiry produces a revision-scoped elapsed/expiry observation. GRIMM alone validates and commits outcomes. Preserve legacy M1.8 readiness protection and do not reset an ongoing scene objective deadline when issuing a fresh Cue Sheet.

- **SCENE-25** Cue Sheets do **not** expose story graph node IDs/transitions/ending markers or canonical world-state variables merely for engine bookkeeping. The minimal M2 Cue Sheet carries ordered `cue(cue_id)` cues, allowed `actions`, optional current `[elapsed of total]` timer and response `id`. The concrete renderer interprets animation IDs and may own presentation state; no universal world-state projection is required in M2.
- **SCENE-26** Use an input equivalent to `game_action { ref, action, scene_timer }`, where `action:timeout` represents unselected expiry at the reported full elapsed time. Authoring can handle timeout as an ordinary conditional outcome. Implement a minimal response-reference check for stale or duplicate inputs; avoid a second gameplay lifecycle framework.
- **SCENE-27** Authored canonical state variables can have persistent/global or narrative-context-local lifetime. Define initialization/reset on context entry/exit; preserving a drawer's state across encounters is an author's decision. Scope is independent of whether the renderer maintains its own representation of that variable's consequences.
- **SCENE-28** M2 authoring checker starts with a designated start node, at least one ending, valid targets and a deterministic covered outcome for every executable choice (`ELSE` or logically exhaustive cases). Follow **only explicit authored transitions**. A structural check does not claim all stateful puzzles are solvable; defer exhaustive state-space analysis and new M2 save mechanics while preserving existing compatibility.

- **SCENE-29** The authored DSL must support a **complete small stateful mini-game in one narrative context/story node**, without creating a node for every action, animation, progress step or repeated attempt. Story nodes remain authoring/narrative structuring units, not a mandatory micro-interaction topology. The existing choice, condition, effect, timer and Cue Sheet semantics are sufficient for these mini-games; do not implement a separate mini-game subsystem. Demonstrate this with the [complete locked-room door example](milestones/m2-cue-sheet-reference.md).

### M2 authoring DSL — required milestone deliverable

M2 **must implement a small, documented, human-authorable story DSL** for the authored LucasArts/Telltale adventure features below. The implementation agent may choose its concrete grammar, parsing strategy, Rust representation, file format and whether to embed or compile into the current authored story representation, but **must demonstrate every capability with an executable authored example**. The examples below express requirements, not a mandated `ON/IF/PLAY` parser syntax.

- **DSL-01 — Narrative structure:** Declare one entry/start, narrative contexts or graph nodes, explicit authored transitions, at least one reachable declared ending, and local interactions that update choices/state without forcing a graph transition. **A single context/node may contain a complete authored mini-game with repeated stateful actions and conditional solutions**; nodes are primarily for authoring/story organization, never required per gameplay step. The graph remains private to GRIMM; Cue Sheets do not expose node-transition bookkeeping.
- **DSL-02 — State declarations and lifetime:** Declare typed game/world variables and defaults, with **global/persistent** or **narrative-context-local** lifetime. The author decides whether a drawer's open state survives a later visit; define when context-local state initializes/clears. **No `renderer_visible`/`private` tagging is required for M2**: authored conditions determine `cue` entries and offered actions, rather than automatically exposing game variables.
- **DSL-03 — Events/actions:** Define handlers for named player choices and the reserved `timeout` action, as well as which choices are currently offered. Repeated invocation of a choice may update state and offer that same choice again; no fixed universal `Inspect/Take/Use` verb set.
- **DSL-04 — Conditions:** Support deterministic `IF/ELSE`-equivalent branching, boolean combinations, equality/inequality and ordered comparisons of typed state, light RPG-style numeric attributes, and the elapsed timer value `time_passed` supplied in the correlated `game_action`. Author-defined boundary/tie policy; no implicit engine-wide success-on-deadline rule.
- **DSL-05 — Effects:** Support atomic state assignments and arithmetic updates (e.g. `door_damage += character_strength`), authored conditional outcomes, and explicit transition/ending effects. Use existing conditions/effects semantics where possible; failures must not partially mutate state.
- **DSL-06 — Time:** Declare an optional current timer as elapsed/total budget, start or replace it through authored transitions (e.g. `[0 of 12]s`), carry its current baseline across repeated actions, allow authored modifications to its total budget, and declare each action's nonnegative duration/time cost. GRIMM includes that duration in the next Cue Sheet's post-action baseline. The renderer plays cues **before** enabling choices/counting from that baseline. Fictional chronology may be a separate authored state value.
- **DSL-07 — Presentation:** Authored branches select **ordered, renderer-neutral presentation cue IDs**, e.g. `open_drawer`, `break_door_progress`, `break_door_success`, with any action duration needed for the Cue Sheet. Concrete animations, camera tracks, text and audio assets are external renderer/game bindings, not engine DSL types.
- **DSL-08 — Available choices and output:** After every accepted action or timeout GRIMM evaluates state/conditions and generates a **single resolved Cue Sheet** with ID, ordered `cue(cue_id)` cues, allowed actions and optional current timer. No mandatory world-state dump or node-change/ending markers appear in the M2 renderer protocol. A correlated `game_action {ref, action, scene_timer}` is the minimal conceptual input.
- **DSL-09 — Deterministic branch coverage:** Every executable action must have **one unambiguous defined result** for all valid inputs, using a fallback/`ELSE` or provably exhaustive cases. The compiler/checker detects missing handlers, conflicting ambiguous cases, undeclared variables, invalid operators/types, unknown cues/references where resolvable, missing graph targets and timer misuse, with author-friendly diagnostics.
- **DSL-10 — Basic authored graph checks:** Require a designated start, at least one ending and valid explicit targets; check structurally reachable nodes and never assume implicit backtracking. Do **not** require an unconditional exit from every node or full symbolic model checking of all possible puzzle states for M2.
- **DSL-11 — Compatibility and usability:** Existing authored YAML stories and established M1–M1.8 behavior remain loadable and regression tested. Document syntax and semantics, show valid and invalid examples and provide a clear author-facing CLI validation workflow. A new generic scripting VM, ECS, runtime code generation, open-world framework or full RPG language is out of scope.
- **DSL-12 — Acceptance example:** Implement and play `The Locked Room` from DSL-authored content, following the [complete M2 Cue Sheet and Game Action reference](milestones/m2-cue-sheet-reference.md). The behavioral fixture explicitly covers `open_drawer`, `take_keychain`, right/wrong keys, repeated `break_door` with `door_damage += character_strength`, cues selected from progress thresholds (`nothing_happens`, `barely_moves`, `progress`, `almost_open`, `success`), conditional offer `squeeze_through` when `door_damage > 0`, both outcomes of `character_dex > door_damage`, continuing 30-second clock, a reserved `timeout` action, optional timer reset/bonus and no double-counting of animation time. The exact DSL grammar and final wire serialization are agent-owned, but the complete observable behavior must be executable and tested.

The agent chooses precise technical representations; the **observable behavior above is required**. Prefer the smallest surface that reuses already implemented GRIMM model concepts.

- **DSL-13 — Order-sensitive, untimed authored interactions:** The DSL must support several concurrently offered actions whose **execution order**, without any active countdown or additional graph node, changes later authored outcomes/offers via canonical state. Acceptance example: a detective at a crime scene may `search_crime_scene` or `examine_body` first. Searching while `suspect_present=true` reveals `confront_suspect`; examining the body first commits `suspect_present=false`, so later searching instead yields `find_suspect_footprints`. This is explicit authored cause/effect, **not an autonomous NPC timer or simulation**. Verify both orders produce distinct deterministic Cue Sheets. No bespoke sequence-engine primitive is required.

### M2 scope/design clarifications (proposed details, not yet a schema)
- **TIME-AUTH-01** For M2 authored conditions, explicit operators (`<`, `<=`, `=`, `>=`, `>`) decide whether a deadline boundary counts as a separate outcome. Define the `time_passed` reference interval and its units; do not hardcode a universal equal-boundary policy.
- **TIME-AUTH-02** Match authored action duration and required action-presentation duration; treat fictional world chronology independently as author-controlled state. No core wall-clock polling. M2 need not activate choice-selection and scene-completion deadlines concurrently.
- **TIME-AUTH-03** Host-supplied selection/expiry time is trusted as a runtime input (not independently verified against physical wall time), while revisions and duplicate-input rejection still protect canonical state.
- **TIME-AUTH-04** M2 does not require a new pause feature; assess save restrictions at active timers/unfinished presentation **without regressing existing M1.75 pause/save semantics**.
- **TIME-AUTH-05** Favor one fully resolved Cue Sheet with authorized ordered `cue` entries, allowed choices and optional timer; concrete serialization is still open. GRIMM need not send world-state variables to the M2 renderer. Renderer owns playback and its presentation state, while existing readiness/revision safety protects against premature or stale inputs.
- **AUTH-ELSE-01** Each executable authored choice needs deterministic, unambiguous outcome coverage (an ELSE/fallback or proven exhaustive guards). Do not require a separate ELSE for every tested variable; global puzzle solvability analysis can remain bounded and may be postponed beyond M2.

- **SCENE-30** For M2, `world_state` means GRIMM's authored canonical variables and current values used for rules, **not a mandatory Cue Sheet field**. A renderer may interpret `cue(cue_id)` to manage its own persistent presentation representation. Renderer-side state is never the authoritative source of `door_damage`, `drawer_open` or other game-rule facts.
- **SCENE-31** For later graphical games, renderer-owned presentation state may have its own save data. The game's save/load coordinator must restore GRIMM's canonical state and renderer presentation to the **same consistent checkpoint**; choosing atomicity, ownership and resynchronization mechanisms is postponed beyond M2. Preserve implemented M1.75/M1.8 save features.

## M3 — Story Authoring & Validation Toolchain (proposed)

The former M3 *Inventory/Puzzles/Character State* deliverables are **cancelled**. The earlier `PUZZLE-01..04` and `STAT-01..02` proposals are **not mandatory, scheduled deliverables**; they are preserved below only as historical candidates. M2 already supports authored item-like flags, repeated stateful puzzle interactions, alternative solutions and simple numeric attributes. Do not introduce an item-identity framework, generic inventory, character rules framework, reusable minigame templates or a DSL abstraction without a specific demonstrated need.

<details>
<summary>Historical M3 proposals — superseded, not acceptance criteria</summary>

- **PUZZLE-01** Optional items, discoveries, ownership and inventory-based conditions/effects; avoid an ECS or obligatory item taxonomy.
- **PUZZLE-02** Allow authored combinations through ordinary contextual choices with explicit preconditions and effects.
- **PUZZLE-03** Model multi-step puzzles, alternate solutions, failed attempts and action costs through the existing state/choice/time model.
- **STAT-01** Optional character attributes and relationships as typed state.
- **STAT-02** Optional deterministic attribute thresholds/checks.
- **PUZZLE-04** Historical MacGyver-style demonstration (no longer a milestone requirement).

</details>

### M2 minimum namespace contract (carried forward into M3 tooling)

- **DSL-14 — Stable qualified presentation/action identities:** M2 compiles local authored action and cue identifiers to effective IDs qualified by an authored namespace, defaulting to the **story-node name**. Authors may explicitly override that namespace to avoid breaking renderer bindings when restructuring the narrative. Normalize names to lowercase ASCII snake_case in a documented deterministic way, reject duplicate **effective** identifiers (including normalization collisions), and emit the same fully qualified opaque IDs on Cue Sheets and accepted `game_action` inputs. Within one context, repeating an action does not create a new ID; separate contexts may reuse the same local name. Do not change legacy story/CLI identifiers behind users' backs. Precise grammar and treatment of unsupported characters are implementation choices to test.

### Proposed M3 authoring/lint/index deliverables

- **AUTH-01** Provide an author-facing CLI/compiler inspection command for GRIMM DSL content, building on M2's parser, type checking and basic graph validation; diagnostics include source file and location and severity.
- **AUTH-02 (M3 validation of M2 IDs)** The **effective namespaced cue/action identifiers are an M2 protocol/DSL foundation**, not a newly introduced M3 wire format: by default the story-node name supplies the namespace for local action and cue identifiers, with an optional authored stable namespace override. Namespace and local names normalize to lowercase ASCII snake_case and are emitted as stable qualified lookup keys (e.g. `interrogation_miller.tell_truth`) for both renderers. M3 validates/cross-references them and exports their usage. Names never depend on Cue Sheet revision, repeated action execution or renderer shots; exact syntax is implementation-defined.
- **AUTH-03** M2 must reject basic duplicate/ambiguous effective identifiers and invalid references as part of compilation. M3 improves diagnostics and indexing for **collisions after normalization** (e.g. `Interrogation-Miller` and `interrogation_miller`), pointing to both definitions and never silently suffixing IDs. A local name may legally recur in distinct namespaces. Keep legacy M1 story identifier behavior backward compatible.
- **AUTH-04** Export a machine-readable **per-context inventory** of effective action/cue IDs with source locations and usage references so game-specific tooling can compare it with text labels or graphical asset/media mappings. Both renderers consume the identical qualified IDs without reading GRIMM graph topology.
- **AUTH-05** Add useful structural reachability, dead-end and state/condition diagnostics. State-dependent reachability/exhaustiveness is not generally decidable under repeated numeric actions and timers: label bounded exploration results as such and never claim a path is impossible solely because bounded search did not find it.
- **AUTH-06** Demonstrate comparison of the GRIMM index against a **small illustrative renderer manifest** (text labels or placeholder graphical cue bindings) in M3; later M4 uses the complete text example's bindings and M5 the actual graphical/audio resources. The GRIMM engine does not interpret asset files or embed dialogue text; IDE support stays optional.
- **AUTH-07** M2 still requires elementary DSL compilation/typing/reference/graph checks. M3 builds incremental authoring tooling rather than blocking M2 on an IDE or full state-space model checker.

- **EXAMPLE-07 — Narration pacing via multiple cues:** One Cue Sheet can emit several ordered `cue` identifiers representing narrative beats. An M4 text renderer can reveal associated prose incrementally with manual or reading-speed-based pacing; an M5 renderer can stage separate shots/audio/animations. Per-cue presentation timing and skip/readability controls are renderer-owned. Renderers must finish the prescribed cue sequence before exposing the offered actions or starting/resuming a scene timer. The timer baseline and game consequences remain identical, regardless of presentation speed; do not add per-cue engine callbacks or text/asset timings to the GRIMM DSL.
- **EXAMPLE-08 — Author-defined cue granularity:** The story author alone decides how finely to divide presentation into cues, from one short dialogue line or narrative beat to a full scene with numerous internal animation/camera/audio cuts. GRIMM imposes no per-cue size, duration, shot count or granularity policy. Renderers choose internal staging/pacing within each cue and preserve the authored sequence of cue IDs. The M3 indexer reports those authored IDs unchanged and must not infer/split/merge cues.

### Cross-renderer story and dialogue contract (M4/M5)

- **EXAMPLE-01** Keep the **M4 text example and its M5 graphical adaptation within the GRIMM repository**, preferably under `examples/`. Both use **the same GRIMM-authored story script**, not copied or translated rule definitions; M5 changes renderer, media and input presentation only.
- **EXAMPLE-02** For equivalent action sequences and reported timer inputs, both examples must issue the **same GRIMM `game_action` IDs** and receive the **same logical Cue Sheets**, gameplay state transitions and outcomes. Cue Sheet `id`/revision semantics follow the actual session contract; test a representative transcript with a controlled clock.
- **EXAMPLE-03** M4's text renderer binds `cue` IDs to authored prose/dialogue and offered `action` IDs to visible choice labels. M5's graphical renderer binds the **same IDs** to game scenes, animation/camera/audio, voice lines and/or subtitles and visual/menu choices. No prose or label text is required in the core Cue Sheet, and no direct dialogue-content dependency is introduced into GRIMM rules.
- **EXAMPLE-04** The rule-author must choose semantically stable action/cue IDs (e.g. `tell_truth`, `lie`, `detective_believes_you`). Renderers must preserve those meanings while translating into dialogue wording, voices, captions and interaction choices. GRIMM determines which IDs to offer/emit and what effects they have; renderer bindings determine only how those IDs are presented.
- **EXAMPLE-05** For M4/M5 example games, all required cue IDs and action labels must be implemented by the concrete renderer; missing bindings should produce actionable game/example diagnostics. A generic asset-mapping subsystem in GRIMM is not required.
- **EXAMPLE-06** This is an **M4/M5 target architecture** for new DSL-authored example games, not a demand to retroactively delete M1's existing text-based story schema or break older games/CLIs.

## M4 — Complete Text Adventure

- **ADV-01** Ship an **actual, finished, text-based LucasArts/Telltale hybrid adventure** authored in the M2 story DSL and played through a concrete text renderer. Several narrative contexts/locations are connected **only by authored transitions**, with meaningful optional revisits. Keep the story graph invisible to renderer-facing Cue Sheets.
- **ADV-02** Integrate multiple related authored puzzles, investigation, optional simple item-like variables, order-dependent events, character relationships/attributes, dialogue and timed choices into one coherent narrative. **No first-class inventory system is required** unless a concrete game obstacle demonstrates a need for it.
- **ADV-03** Consequences of earlier decisions affect later locations/dialogues/actions and endings without an exponential authored node graph. Support intentional failure and at least three meaningful authored endings.
- **ADV-04** Playable, self-contained M4 **text game** is the canonical game content to reuse for M5; preserve a tested story definition, canonical rules/state and semantic Cue Sheet/game_action contract for the graphical adaptation. Test alternative routes, cross-context effects, timeout behavior, save/load compatibility where existing mechanisms apply, and absence of renderer-owned gameplay decisions.
- **ADV-05** Keep optional authoring ergonomics driven by real friction found while writing this game. Do not require generic rule reuse, scripted templates, first-class item semantics or a general game UI composition framework.

## M5 — Graphic Adaptation of the M4 Adventure

- **VIS-01** **Graphically render the same authored M4 adventure**, not a new unrelated demonstration story. Preserve the canonical M4 DSL-defined story rules and gameplay logic; do not duplicate or fork gameplay conditions/effects in graphical renderer code.
- **VIS-02** The concrete graphical renderer interprets the existing `cue(cue_id)` cue IDs into appropriate game-specific graphics/animations, camera/audio and presentation changes. It owns its visual/presentation state. **Do not require `world_state` snapshots or graph node IDs in Cue Sheets.**
- **VIS-03** Provide or generate and curate usable assets and a concrete renderer/input implementation for the M4 game, including clickable/menu choices mapped to the same offered action IDs and timer/readiness semantics. Asset generation is part of game production, not a GRIMM engine feature.
- **VIS-04** Deliver a complete playable graphical **example application in this repository**, using GRIMM as a Rust library and loading the exact M4 authored script. This example owns the renderer, media, input and lifecycle; GRIMM owns canonical story/gameplay rules and World State. No separate repository is required.
- **VIS-05** Coordinate renderer-owned presentation save/load with existing GRIMM canonical saves to produce a consistent game checkpoint as demanded by the chosen actual renderer. Avoid a generic engine-level save format for arbitrary renderer state.
- **VIS-06** Verify equivalent meaningful narrative behavior between M4's text game and M5's graphical adaptation (key offered actions, consequences, state transitions, deadlines and endings). Rendering style and animation content need not match text one-to-one.
- **VIS-07** Use actual game implementation to identify and fix any *necessary* GRIMM/host integration seams; validate missing/unsupported cue IDs, inputs and assets within the concrete renderer or game build rather than adding a speculative generic media-mapping framework.
- **VIS-08 (investigation only)** A minimal embedded-host interaction may be demonstrated if it naturally fits the same-repository graphical example. Do not require a standalone Bevy/3D framework or general-purpose plugin platform; external-host integration beyond the example stays an independent future question.

## Playable demo acceptance rule
- **DEMO-01** Gameplay milestones M2, M4 and M5 ship/play a complete reference game or complete playable expansion, with reproducible success/failure paths, documentation and regression tests. **M3 is tooling-only**: it must demonstrate meaningful compiler/linter/indexer checks and a renderer-manifest comparison against existing playable authored examples or small fixtures, **not produce another complete game**. Existing games and legacy behavior must continue working.
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
