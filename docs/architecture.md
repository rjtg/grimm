# Architecture — initial boundary contract

This document fixes **dependency boundaries**, not all future APIs. Public types will evolve through milestones.

The implemented M1 crate map, authoring/checkpoint DTOs and provisional protocol
migration are described in [the M1 brief](milestones/m1.md) and
[story/save format](story-format.md). Engine dependencies remain semantic-only;
YAML/JSON file handling belongs to the CLI composition root.

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

A composition root wires concrete components. Engine modules must never import adapters, renderers, UI frameworks, or LLM SDKs. **Renderer independence is an engine property, not a guarantee that any game runs meaningfully on any renderer.** A **standalone GRIMM-led game** may select its concrete renderer implementation, required assets and components. In an **embedded narrative integration**, a host game engine (such as Bevy) may own its own renderer and main loop and instantiate GRIMM only as a narrative subsystem. The exact application entry point and dependency direction are open. A generic rendering capability profile is not enough to know a game's rooms, characters, cameras and animations. Exact packaging/loader contracts remain open.

## Domain ownership
- **Engine:** authoritative for GRIMM-owned narrative/game state; validates its actions and commits its effects/events; graph traversal. In embedded use, an external engine may own other world/simulation state.
- **Semantic protocol:** typed actions, committed game events, narrative content, currently valid interaction offers, and player-visible projections. These are related but not interchangeable.
- **Output adapter:** maps already-authorized semantic output to presentation commands; may maintain presentation state, but cannot modify game truth. Optional stylistic text transformation is deferred, not a requirement for dynamic narrative.
- **Renderer:** displays presentation commands (text, comic, 3D) and reports raw user interactions / presentation completion.
- **Input adapter:** converts raw UI input or text into proposed semantic actions; engine validates proposals.
- **Composition root:** chooses engine capabilities, adapters, renderers and transports.

## World entities, narrative scenes and progress
World entities are persistent identities in the game world: characters, locations and eventually objects or other meaningful things. Specialized entity kinds share identity and state-ownership principles but do **not** require one generic entity type, inheritance hierarchy, entity-component system, or mandatory central entity store.

Narrative scenes are authored or generated situations describing what happens, what may be shown and which actions are offered. A scene may reference zero, one or multiple entities; an entity may participate in many scenes. The current M1 story node is a narrative unit and must **not** be treated as synonymous with a physical location. A warehouse visited during daylight investigation, night surveillance and a return visit can be three separate scenes that share one location identity.

Story progress is likewise separate from a scene or location. The engine may express progress through state variables and committed effects, and later optional narrative milestones; a second parallel progress graph is not mandated. The engine remains authoritative for entity state and transitions. Presentation adapters receive only deliberately revealed context, not private entity state.

Implementation staging: M1.5 first tests these distinctions by writing a second game using existing primitives, then makes minimal format/API additions if evidence requires them. Minimal item and character state may be introduced as authored choice-driven adventure requirements demand it. Dynamic node proposals, NPC agency and narrative direction remain uncommitted later extensions, not milestones M2–M5. See [ADR-0005](adr/0005-world-entities-scenes-and-progress.md).

## Temporal semantics
Time is a cross-cutting but opt-in game rule, distinct from narrative progress and presentation animation time. **World time** advances through explicit engine-validated steps; **decision deadlines** express how long a player may act; **narrative progress** reflects consequences rather than a clock tick alone. The host/runtime supplies clock or elapsed-time input, but the engine owns committed temporal state and determines timeout outcomes. No system-clock, timer, sleep, or render-framework dependency enters the headless engine.

Decision deadlines require an explicit start condition (for example a host acknowledgement that a prompt became available), deterministic ordering for competing action/timeout inputs, and idempotent resolution for an interaction revision. Pausing and restoring must define whether the decision timer is suspended or expires offline; this policy must be documented and tested, not inferred from wall-clock timestamps. Renderers may display countdowns, but cannot authoritatively decide the story consequence. Existing untimed authored games must behave exactly as before.

Time advancement may activate conditions and future scheduled reactions, but M1.75 need not implement continuous simulation, background NPCs, or a general event scheduler. The engine validates any temporal effects and resulting transitions. See [ADR-0006](adr/0006-game-time-and-temporal-semantics.md) and [M1.75](milestones/m1.75.md).

## Narrative flow and presentation gating (M1.8)
A narrative scene is an engine-owned unit of story progression, not necessarily an immediate player choice or a physical location. Storytelling presentation and interaction activation are separate phases. A scene may be (a) narrative-only, continuing via an explicit authorized presentation-complete acknowledgement or deliberate player-paced Continue, or (b) interactive, whose offers become available only after any required storytelling has completed. Avoid mandatory dummy choices whose only purpose is to proceed after dialogue/camera motion.

The **output adapter/renderer** decides how to present the authorized narrative (text, spoken lines, comic panels, camera moves), and can signal presentation completion or interaction availability through a renderer-neutral acknowledgement to the **host/composition root**. The host validates/correlates the signal with the active presentation/interaction revision before proposing a typed lifecycle event to the **engine**. Neither renderer nor output adapter directly calls engine code, mutates canonical state, decides the next node, or authorizes timeout consequences. The engine validates and commits narrative continuation and timed-interaction activation, at most once per revision. Different renderer technologies may take different real durations without altering fictional chronology or spending a decision budget before interaction readiness.

Do not conflate presentation-complete with decision-ready: a pre-rendered choice list does not imply that an ongoing required dialogue/animation has completed, and a completed narration does not imply the next offer has been shown to the player. M1.75 `AwaitingReady` and `Ready` form the existing foundation but do not themselves guarantee the user has read CLI text. M1.8 should specify minimal state/protocol changes and prevent early, duplicate, stale and out-of-order signals from starting deadlines or committing non-choice transitions. Pending presentation, pause, restore and frontend disconnect behavior require explicit deterministic policies. Automatically chaining narrative-only nodes must be bounded and cycle-safe; waiting for renderer completion must never block the core synchronously.

CLI fallback: for reading-intensive storytelling, display the narrative and require an explicit Enter acknowledgement before exposing/activating the timed decision. A visual renderer can instead report required dialogue, animation, or camera completion. User-driven Continue remains a legitimate, intentional pacing choice, but is not the sole way to encode a non-interactive segment.

See [ADR-0007](adr/0007-narrative-flow-and-presentation-gating.md) and [M1.8](milestones/m1.8.md). These refine the existing M0 dependency boundaries and M1.75 temporal lifecycle without making visual rendering mandatory.

M1.8's implemented lifecycle is specified in the [pre-API design](milestones/m1.8-design.md)
and [schema/protocol reference](story-format.md#story-schema-3-narrative-flow-and-presentation-gating-m18).
Opt-in schema-3 nodes expose Presenting → AwaitingInteraction → Interactive,
with orthogonal pause state. Revision-scoped presentation feedback travels from
renderer to host, through adapter translation into `FlowInput`, then engine
validation. Completion reserves a new revision before offers become available;
readiness activates the existing deadline only afterward. Narrative-only
completion commits one validated successor edge. The core never waits for a
renderer, and continuation-only cycles are rejected. Required CLI reading uses
explicit Enter; EOF cannot stand in for completion. ADR-0007 retains Proposed status.

## Event and presentation semantics
Committed game events describe what actually happened, including offscreen events. They are not automatically visible to players. Narrative content describes what may be communicated; interaction offers describe currently valid inputs. A filtered projection exposes only player-visible information. A renderer must not receive hidden world state by default.

Authored narrative and semantically described choices remain the primary inputs. M2 develops **in-scene choices** that change state and available offers without changing the narrative node; M3 adds optional inventory/item combinations and RPG-like attributes as conditional choice state; M4 brings together LucasArts-style puzzle exploration and Telltale-style consequential narrative scenes; M5 adds graphical presentation of the **same** engine-authored offers. No fixed Inspect/Use/Take/Combine primitive is required. Dynamic graph/node providers and generative narration belong to deferred open-world questions, not current milestones. See [open questions](open-questions.md).

## Game composition and renderer ownership (future M5)

A **story graph** contains authored narrative structure, rules and choices, but is **not** by itself a complete distributable game. A playable **game** composes its narrative/gameplay definition, GRIMM's reusable narrative engine, its actual runtime/rendering implementation and the game-specific assets, bindings and other systems it requires. In **standalone mode**, a game-level composition may select/instantiate the concrete renderer. In **embedded mode**, another runtime can already own the renderer and main loop while embedding GRIMM as a library/service; a story definition need not load or own that renderer. A game may also provide additional configuration, authored modules, tooling or resources; this list is not an exhaustive plugin contract.

A generic 3D renderer does not magically know how to render every game's 3D world. The game must provide and configure its actual world, models, cameras, animation bindings and choice/hotspot mappings, potentially using custom game-specific renderer code. Shared renderer implementations are allowed, but are not assumed. Different presentation editions are separate **explicit compositions** with their own renderer+asset definitions, potentially sharing an underlying authored rules graph.

**Ownership boundary:** the surrounding game/host composition root wires the actual runtime, renderer, adapters, GRIMM and assets, with the direction determined by the host. GRIMM remains headless and authoritative for **canonical narrative state**, its own choices, story timing and validated narrative effects; an external host may separately own position, physics, world simulation and other game-world state. No GRIMM engine crate imports Bevy or presentation asset types. Crossing the ownership boundary (narrative result → host world consequence or host fact → narrative condition) needs an explicit validated, replayable integration contract, not implicit shared mutation. Validate that required code/resources and semantic bindings are present before starting a game. Precise renderer identifiers, source references, trust/loading policy, packaging, versioning and deployment are unresolved in [open questions](open-questions.md#game-composition-renderer-origin-and-packaging); do not prescribe a generic plugin platform prematurely.

M2–M4 may continue to ship complete CLI-composed authored games. M5 should include a complete authored graphical game and **exercise an external host invoking GRIMM from a world event** before locking in the integration/packaging contract. Whether standalone and embedded entry points share any game-definition manifest remains open in [host integration questions](open-questions.md#host-integration-standalone-game-versus-embedded-narrative-engine).

## State and lifecycle
- Engine state and presentation state are distinct.
- Adapters may be stateful; a filtered snapshot enables reconnection and adapter initialization.
- Input references an active interaction/session revision; stale choices are rejected.
- Presentation completion may be acknowledged, but simulation must not depend on rendering speed except where explicitly synchronized.
- State changes and emitted events are committed atomically; event-driven follow-up processing must be bounded and deterministic.
- For deterministic replay, record external model results or resolved actions rather than assuming an LLM can reproduce output.

## Extension model
Optional graph dynamics, intent resolution, NPC controllers, and narrative direction integrate through explicit engine interfaces and validated action/event processing. Do not implement a general-purpose plugin framework in M0.

## Deadline semantics for authored choices and scenes

Two distinct, opt-in authoritative time constraints must be preserved. The **M1.75 choice-selection deadline** ends when the player *chooses* an eligible offer; failure to respond triggers the authored no-response/timeout transition (for example a conversation partner becomes angry). A timely accepted choice is not retrospectively rejected solely because its subsequent animation/action takes longer. In contrast, a new **M2 scene-completion deadline** requires an authored objective to be *completed* before expiry; multiple choices and timed actions may occur within one scene without resetting that deadline. Beginning a solution does not count as completing it.

When a known scene deadline lies inside an authored action duration, GRIMM determines the **effective execution portion and authored result at choice selection**. Example: a detective's T=10s arrival cuts a five-second hiding action selected at T=6s to four seconds of attempted concealment. The engine can evaluate an authored `suspicion` check using the 4/5 arithmetic fraction and current committed state, commit its chosen transition, and emit the corresponding presentation immediately. There is **no pending action snapshot, periodic progress callback or late success decision** for this M2 case. Independent external events during action presentation belong to a separate future host-integration question. Preserve existing schema and M1.75 legacy choice-selection deadlines.

**M2 renderer timer rule:** The renderer plays all required `play` cues in a Cue Sheet first. Only after those cues finish does it show/enable the offered actions and start or resume the countdown at the **post-action elapsed time and budget supplied in that Cue Sheet**. Action animation duration has already been accounted for by GRIMM and is **never counted again** by the renderer. Subsequent player hesitation advances from that supplied baseline. This rule applies uniformly to first and subsequent cue sheets; no active timer countdown during required cue playback. A newly emitted cue sheet retains the same situation budget unless authored rules explicitly modify it.

### Cue Sheet hides the story graph and canonical world state

The output adapter is not told whether GRIMM changed graph nodes, entered a terminal node or stayed in the same authored context. The M2 renderer only needs **ordered `play(animation_id)` cues**, **allowed player actions** and an optional **current elapsed/total timer**. Canonical `world_state` variables are internal to GRIMM, not a required Cue Sheet field. The story graph and private state remain engine-owned. Narrative endings are conveyed through authored presentation/action availability, not a mandatory node-transition flag.

An illustrated timeout is an ordinary revision-correlated input, e.g. `game_action {ref:3, action:timeout, scene_timer:[30 of 30]s}`. For any other input, `scene_timer` gives `time_passed` as observed at choice selection; authored action duration is applied by GRIMM to produce the subsequent Cue Sheet. A newly started timer can appear as `[0 of 12]s`; a continuing deadline carries its current budget. The renderer plays required cues before activating the new timer baseline.

Authored game facts may be **persistent/global** or **narrative-context-local**, and GRIMM uses them to select the correct cues and actions. They need not be sent as variables to the renderer. When a context ends, authored local state may be discarded; a persistent door/drawer may retain its game-rule condition across later contexts. The concrete renderer interprets cue IDs to maintain its own visual state, potentially including live 2D/3D object poses. Exact lifetime declarations/reset rules remain to be designed.

**Renderer presentation-state ownership:** A prerecorded renderer may simply play the chosen media IDs. A live-rendered renderer may translate the same IDs into animation, camera, object-pose and other visual updates, and keep its own presentation state. It may persist that presentation state as part of the **concrete game's** save. GRIMM remains authoritative for authored world-state/game-rule variables. The game-level save coordinator (future graphical work) must restore both parts of one consistent checkpoint; M2 adds no mandatory projection or universal renderer save schema.

## Cue Sheets and timed choice feedback (M2 working contract)

GRIMM can return a **Cue Sheet** after resolving authored narrative rules, e.g. `play: ["enter_corridor", "dialog_scene_1"]`, `actions: [A,B,C]`, `timer: 20s`, plus an interaction revision. The renderer plays cues in order, then displays/enables the actions and starts the visible countdown **at actual interaction readiness**, not while prior presentation is playing. If A is chosen five seconds after activation it returns a revision-scoped `[action_selected(A), timer_elapsed(5s)]` to GRIMM. The engine validates the offered action and applies authored time consequences; in the absence of selection, externally observed expiry yields a no-choice input, and only GRIMM chooses the authored outcome.

The timer observation for this interaction is an **elapsed duration**, not inherently a wall-clock timestamp. A composition host may relay or correlate renderer messages; the concrete data envelope and trusted timing source remain a protocol design question. Existing M1.8 readiness prevents premature activation. A refreshed cue sheet **must not reset any continuing narrative-situation deadline** merely because its visual choices changed: the timer needs an explicit identity/scope or remaining-budget semantics for that case. The engine does not play media or poll a wall clock.

## Mini-games are authored rules, not miniature node graphs (M2)

The M2 DSL can define a **complete small stateful mini-game inside one narrative context**: inspect furniture, collect keys, increment `door_damage` with repeated `break_door` choices, conditionally unlock `squeeze_through`, evaluate `character_dex` and scene time, then reach an authored success/failure outcome. **Story nodes are primarily an authoring and narrative-organization tool**; they are not the required unit for every individual choice, attempt, animation, state update or mini-game step. This does not remove internal node identities or explicitly authored transitions where useful. One narrative node/context can host a whole mini-game, while a later authored transition can move to the next story section.

GRIMM evaluates the mini-game's rules through ordinary choice/state/time semantics and emits Cue Sheets. A renderer never needs to know if the action was local to a node, crossed a node boundary or concluded the mini-game. No separate engine-owned mini-game framework or hard-coded puzzle action taxonomy should be introduced for M2. See [complete door/minigame reference](milestones/m2-cue-sheet-reference.md).

## M2 vocabulary: narrative context, authored node, presentation shot

Treat **narrative situation/context**, **story graph node** and **visual presentation shot/setting** as different concepts. The narrative context determines ongoing choices, rules and deadlines; a node is authored story structure; a renderer may play multiple shots/animations without leaving the context or changing nodes. Renderer instructions such as `play try_break_door`, state projections such as `door_damage = 2` and offered choices such as `break_door` / `squeeze_through` are complementary output, not separate renderer-owned gameplay decisions. GRIMM chooses authored cues and valid offers after resolving each choice; the renderer maps these to game-specific presentation.

**Interaction revision is not narrative progress or a new timer.** Changing offers invalidates prior revision-scoped inputs, but does not by itself reset an ongoing scene-completion deadline or re-present the initial introduction. A renderer may display a countdown and indicate possible expiry; the engine validates a host-supplied time observation against the authored deadline before committing any timeout transition.

A structural graph search is only a first-pass validator. Puzzle playability depends on bounded reachability through **explicitly allowed transitions** over **narrative context/node + committed state + remaining time**, with any host-owned navigation modeled separately. **No implicit backtracking:** visiting the key rack earlier does not make it reachable from a locked door later. A Telltale-style game must author `leave_and_search_for_key` (or another explicit transition) if returning is possible. In a host-led open-world game, GRIMM can instead resolve the door interaction and yield to the host's exploration; a later world trigger may re-enter a narrative context at the key rack. Such host movement is not a GRIMM graph edge unless an explicit host integration contract exposes its reachability. Do not impose an unconditional exit on every nonterminal node, but diagnose genuinely trapped states and recognize deliberate authored failure/end states. If host-navigation reachability or the state space is unknown, report *inconclusive* rather than assuming teleportation, backwards navigation or puzzle solvability.

## Choice-driven adventure evolution (M2–M5)

A narrative scene may offer choices whose effects change state and refresh offers **without moving to a new narrative node**. A location is not a scene. A choice can have an authored duration; GRIMM does **not** need to track a running action across time observations. Instead the host supplies the **choice selection time**, and GRIMM resolves the full authored action and all known deadline interactions **at selection**, atomically. With deterministic input and canonical state, it computes the effective action interval, terminal outcome or refreshed in-scene state, world-time changes and any relevant authored interruption branch. This transition may account for the fractional progress of an interrupted action **arithmetically** without creating a persistent in-progress gameplay state.

A **choice-selection deadline** (existing M1.75) requires the player to select a choice by expiry; failure to answer follows an authored timeout branch. A **scene-completion deadline** (M2) requires an objective to be completed by a fixed scene deadline; multiple choices may successively consume that budget. At T=6s, choosing a five-second `break_door` action when the deadline is T=10s is **resolved immediately as a failure after an effective four-second attempt**, with any authored conditional transition computed from committed facts and that arithmetic progress. GRIMM commits that result at choice handling, not at T=10s. The renderer receives the authorized four-second attempt plus failure presentation and subsequent choices, never permission to decide the narrative outcome.

**Engine clock ownership:** GRIMM never polls wall time, runs a scheduler or autonomously advances time. The host supplies player choice timestamps and, for **idle expiry** with no input, explicit elapsed observations as already required by M1.75. At choice resolution the engine updates its authoritative logical time from the selected action's authored duration/effective deadline cutoff and effects. Fictional world-minute costs may remain distinct. An action does **not** need a separate renderer `ActionCompleted` input or periodic progress/timeout checks to resolve its known authored outcome.

**Presentation ownership:** the engine emits a semantic presentation sequence whose durations reflect the resolved action (five seconds for a successful drawer opening; four seconds of forcing plus failure for the example above). The renderer/adapter presents it. The *next* offers may already be known and committed but must not be usable until their required presentation ends and M1.8 revision-scoped readiness is satisfied. The renderer acknowledges presentation readiness, **not gameplay success**. This is the necessary distinction between canonical action resolution and player-visible pacing.

### Repeated attempts, stateful progress and visual feedback

Repeatedly selecting the **same authored choice** can produce different fully resolved outcomes because committed state changes **between** attempts. For example, `break_door` can increase `door_break_progress` on each attempt, while an authored `strength` attribute determines whether success takes one, two or three separate attempts. Each attempt is a single choice-time transition: a non-final attempt stays in the current scene, updates progress and emits a semantic `try_break_door` presentation cue; the final successful attempt opens the door and emits `success_break_door`. A game may author variants by strength or progress to communicate near-success or futility (e.g. `try_break_door_almost`). The renderer presents the **engine-selected cue** and does not decide whether the door opened. Bind cues to game-specific animations outside GRIMM's headless core. Each attempt uses its authored duration and is checked against the shared scene-completion deadline before committing its outcome. This is **persistent progress across discrete Choices**, not an in-progress-action state or a renderer-driven progress callback. The numeric threshold scheme is illustrative, not a mandatory RPG mechanic.

### Persistent scene state versus transient graphics

Choosing `open_drawer` triggers one validated engine transition: GRIMM commits `drawer_open = true`, computes `take_keychain` as an eligible subsequent offer, and emits a five-second opening presentation. After it is presented, the renderer displays the canonical **open drawer**, revealed keychain and available choice. Revisit, reconnection and save/load reconstruct the open drawer from the authorized state projection rather than local interaction history. Presentation may animate from a closed pose to the committed open pose without causing a second gameplay commit.

If an authored action is interrupted **by a deadline already known at selection**, GRIMM selects the interruption outcome and corresponding shorter presentation immediately. No running intermediary gameplay state, success acknowledgement, animation-dependent result or renderer reversal is necessary in this scoped model. Whether future external host events or physics can interrupt an action **after** it was resolved is a separate deferred embedded-host question, not a reason to introduce a running action lifecycle for M2.

A LucasArts-style puzzle is an authored sequence of choices involving searching, discovering, collecting and combining objects; a Telltale-style scene offers tightly staged consequential choices, optionally timed or presentation-gated. Both share one conditional-choice mechanism. RPG-style stats/skills/relationships are additional optional state inputs and effects, not a required character-system framework. A renderer may display these as numbered choices, graphical hotspots or inventory views. Never bind location identity, scene identity and narrative progress into one concept.

Every milestone must ship a small playable game that demonstrates each new gameplay capability and relevant failure paths, with automated end-to-end coverage and authored playtest notes. Dynamic nodes and provider boundaries are intentionally **not** specified before the authored adventure mechanics have been exercised. Later open-world/Story-RPG proposals could come from human DMs, rule systems or AI through an engine-validated boundary, but that design is postponed.

## Open design questions
The living [open-points catalog](open-questions.md) tracks in-scene choice lifecycle, non-resetting timers, puzzle/state graph checking, inventory and character stats. Runtime node providers and generative models are deferred to a later open-world phase; resolve no proposed APIs for those yet.
