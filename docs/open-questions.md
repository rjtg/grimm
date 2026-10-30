# Open design questions

This is GRIMM's running catalog of **unresolved** questions, not accepted APIs or milestone commitments. Record decisions, rationale, tests and ADR links when the relevant authored demo motivates them. Status: Open, Investigating, Decided (with link), Deferred.

## M2 Cue Sheet and authoring review — decisions (2026-10-30)

- **Renderer does not see story-graph transitions:** GRIMM alone owns nodes and contextual transitions. Cue Sheets communicate only ordered `play` cues, accepted `actions`, full **renderer-visible** world-state projection, currently relevant `scene_timer` when present, and response `id`. No `node_changed` or `ending_node` protocol field is required by this design. Ending/continuation is reflected in the next playback and accepted inputs.
- **Input is one revision-correlated game action:** `game_action {ref:3, action:unlock_door_key1, scene_timer:[8 of 30]s}`. Idle expiry uses the ordinary reserved choice `game_action {ref:3, action:timeout, scene_timer:[30 of 30]s}`; the renderer reports the timer's actual value, while GRIMM evaluates authored consequences. A minimal `ref` check remains valuable to prevent duplicate/late inputs from committing twice, even if the renderer is expected to behave correctly.
- **Timer value is explicit:** each Cue Sheet supplies the *currently relevant* elapsed/total pair. A newly authored 12-second timer may be issued as `[0 of 12]s`; a continuing one carries forward its updated `[elapsed of total]s`. The renderer first plays the cues, then activates choices and counts from the specified baseline. No renderer-side inference about graph nodes or timer identity is necessary in the illustrative M2 handshake.
- **Time conditions:** `time_passed` in an action rule is the elapsed timer value in its `game_action` input; authored action duration is a known story constant/rule and is applied by GRIMM to compute the subsequent Cue Sheet. The author explicitly determines success, tie and failure branches. Fictional world chronology may be a separate ordinary state variable.
- **Visible full-state projection with scoping:** GRIMM sends the complete state that the renderer is authorized to know, not incremental deltas and not hidden/private world state. Game state may have **persistent/global** and **narrative-context-local** lifetimes; local facts may be discarded on context exit. Scope does not force all object facts to reset on revisit: whether `drawer_open` is local or persistent is an author's choice. The exact authored local-state lifetime and reset semantics need specification.
- **Saving and exhaustive gameplay checking:** postpone new M2 save/load design and full state-space exploration; do not regress already implemented saves. Start with simple authored-graph validation: one start node, at least one ending, valid referenced targets, deterministic coverage of condition branches and reachable structure (only explicit edges, no imaginary backtracking). A structural graph check is not proof that every world-state puzzle can be solved.
- **Rust implementation:** not a user-facing gameplay choice. Once authoring format and Cue Sheet fields are settled, map them onto existing Rust types for story actions/conditions/effects, semantic outputs, host input, time handling and versioned persistence. Reuse existing implementations where possible instead of inventing competing systems.

**Remaining focused M2 questions:** (1) concrete story authoring syntax for choice cases, effects, time costs and play cues; (2) exact shape of global vs narrative-context-local state and how context exits affect it; (3) minimal serialized Cue Sheet and `game_action` types including reserved `timeout` and `ref`; (4) compatibility-safe implementation. A large graph solver, new save lifecycle and advanced renderer/host framework are not prerequisites.

## Active scope boundary for M2 (2026-10-30)

Focus subsequent discussion and M2 acceptance on **authored LucasArts/Telltale-style adventures**: context-sensitive choices, local puzzle/world-state changes, explicitly authored narrative transitions, scene deadlines, presentation cues and light optional RPG-influenced checks. Do not implicitly add host-owned traversal, Bevy integration, open-world navigation, a full RPG ruleset or generated narrative to M2. Those remain M4/M5/later questions and should not block the M2 choice/adventure model. Revisit remaining M2 questions one at a time within this scope.

**M2 renderer timer rule:** The renderer plays all required `play` cues in a Cue Sheet first. Only after those cues finish does it show/enable the offered actions and start or resume the countdown at the **post-action elapsed time and budget supplied in that Cue Sheet**. Action animation duration has already been accounted for by GRIMM and is **never counted again** by the renderer. Subsequent player hesitation advances from that supplied baseline. This rule applies uniformly to first and subsequent cue sheets; no active timer countdown during required cue playback. A newly emitted cue sheet retains the same situation budget unless authored rules explicitly modify it.

## M2 worked Cue Sheet trace — locked room and keyring (illustrative)

This **worked example** uses **cumulative time since the narrative situation's timer origin** and an authored 30-second time budget. GRIMM resolves each selected action at once and returns the post-action time/state in a new cue sheet. **Not a final serialized schema**.

```text
cue_sheet { id:1; play:[locked_in];
  actions:[open_drawer, open_wardrobe, carefully_look_around];
  scene_timer:[0 of 30]s }

game_action { ref:1; action:open_drawer; scene_timer:[2 of 30]s }
  # 2s player hesitation; action duration 1s
cue_sheet { id:2; play:[open_drawer];
  actions:[close_drawer, take_keychain_from_drawer, open_wardrobe, carefully_look_around];
  scene_timer:[3 of 30]s }

game_action { ref:2; action:take_keychain_from_drawer; scene_timer:[5 of 30]s }
  # 2s additional player hesitation; action duration 1s
cue_sheet { id:3; play:[take_keychain];
  actions:[close_drawer, open_wardrobe, unlock_door_key1, unlock_door_key2,
           carefully_look_around];
  scene_timer:[6 of 30]s }

game_action { ref:3; action:unlock_door_key1; scene_timer:[8 of 30]s }
  # key 1 is wrong; trying it takes 2s
cue_sheet { id:4; play:[wrong_key_does_not_turn];
  actions:[close_drawer, open_wardrobe, unlock_door_key1, unlock_door_key2,
           carefully_look_around];
  scene_timer:[10 of 30]s }

game_action { ref:4; action:unlock_door_key2; scene_timer:[12 of 30]s }
  # key 2 fits; opening takes 2s
cue_sheet { id:5; play:[unlock_door_success, escape_room]; actions:[];
  scene_timer:[14 of 30]s }
```

GRIMM owns persistent facts (e.g. `drawer_open`, `has_keychain`, door status), authorized action lists and authored conditional outcomes; the concrete renderer owns how to play `open_drawer`, `take_keychain` and door-related visual cues. `game_action.scene_timer` means **time at player selection**, not the post-action value. `cue_sheet.scene_timer` means **time after GRIMM has already added the authored action cost**; rendering that same action duration must not deduct it *again*. The renderer must count subsequent hesitation from the post-action baseline once playback has made the next choices interactive. The opening `locked_in` cue must not silently consume the budget before its start/readiness semantics are specified.

**Time-buying example:** an authored two-second choice can extend a timer budget by five seconds, yielding `[14 of 35]s` from `[12 of 30]s`. This changes the allowed deadline, **not** how much time has already passed. The rule is authored and GRIMM chooses the resulting cue sheet; the renderer does not decide the bonus.

**Still to decide:** the exact lifecycle of running/restarting timer display for refreshed cue sheets; whether `[elapsed of total]` is sufficient to distinguish a continuing situation deadline from a new decision timer; the response's state projection and transition/ending markers; the time-consistency rule when the renderer performs a `play` cue that represents the already-accounted action duration; and whether a separately authored presentation cue has additional time cost or only presentation gating.

## M2 engine/renderer handshake — cue sheet (working design, 2026-10-30)

GRIMM's resolved output is conceptually a **Cue Sheet** specifying what to present and which choices to offer; the renderer executes it and reports user inputs plus elapsed time against its current timer. Example (illustrative notation, **not** a committed serialized schema):

```text
GRIMM → Renderer:
  cue_sheet(revision=12) {
    play: ["enter_corridor", "dialog_scene_1"]
    actions: [A, B, C]
    timer: 20s
  }

Renderer:
  play the two cues in order
  display/enable A, B, C and start the 20s timer only once choices are ready
  player selects A five seconds after that point

Renderer → GRIMM:
  [revision=12, action_selected(A), timer_elapsed(5s)]
```

The renderer provides the elapsed interval and not a wall-clock timestamp for this M2 example. GRIMM consumes the revision-scoped event and its elapsed duration, validates that A was offered and interprets the time according to authored rules, commits one deterministic transition and emits the **next Cue Sheet**. If the timer expires with no input, the renderer/host reports elapsed time without a choice and GRIMM selects the authored no-response consequence. The renderer never determines that story outcome.

**Presentation and timer start:** required `play` cues precede the availability of the listed `actions`. The timer shown alongside those choices starts when they actually become selectable, not while earlier presentation cues are still playing. M1.8's readiness/activation protection must remain, but exact acknowledgement/event names and whether the cue-sheet timer is a choice-selection or situation/objective timer require the minimum consistent protocol. An active situation deadline carried over from earlier interactions must not inadvertently reset just because a new Cue Sheet is emitted.

**Unresolved implementation details:** precise event envelope/field names, whether action and elapsed input are atomic, timer origin and cumulative vs incremental elapsed observations, whether the renderer or composition host is the trusted timer source, revision validation and explicit expiry on silence. Do not introduce an independent GRIMM wall clock or renderer-authoritative authored consequences.

## M2 time/deadline and interface review — current working direction (2026-10-30)

**Scope:** authored LucasArts/Telltale-style adventures, optionally using lightweight RPG state; no general real-time simulator.

- **Deadline types:** the conceptual distinction between choice-selection deadlines and multi-choice objective/situation deadlines is understood. **Do not require both timers to be active simultaneously in an M2 narrative context.** Exact authored syntax remains unspecified.
- **Author-defined boundary:** success/failure/tie conditions are authored explicit comparisons, e.g. `time_passed > 6s` vs `time_passed = 6s` vs `time_passed < 6s`, or `>= 6s` vs `< 6s`. Do not impose a universal inclusive/exclusive success rule. `time_passed` needs a precisely defined reference interval (notably whether it includes the just-selected action's duration) so comparisons are deterministic.
- **Time semantics:** authored action duration and its presentation/action animation duration match. Fictional chronology may be authored independent world state. Real host time matters only for timed narrative situations (which can appear frequently) and is supplied externally; GRIMM does not tick a wall clock.
- **Host clock trust:** for M2, **accept the host's reported selection/expiry timing as the runtime observation**, without independently verifying the physical clock. Preserve existing revision-scoped stale/duplicate-input protection so a validly timestamped but obsolete input cannot commit twice.
- **Presentation:** after a choice GRIMM returns one already-resolved authoritative outcome with ordered presentation cues, player-visible world-state projection and next allowed choices. The renderer owns **how/when it presents** cues and offers; retain the existing minimal M1.8 interaction-readiness boundary to reject premature/stale inputs rather than establishing an engine-owned animation player.
- **Pause/persistence:** a general GRIMM Pause mode is **not a new M2 requirement**. Proposal: disable saving during a running timed narrative situation and possibly any required incomplete presentation, then persist settled state at stable boundaries. Existing M1.75 pause/save compatibility **must not be removed or silently changed**; whether M2 disables those facilities for its newly authored situations requires explicit compatibility review.
- **Authoring validation:** require each executable action to have deterministic, unambiguous **complete outcome coverage** (a fallback/ELSE or provably exhaustive branches), not a separate ELSE for every individual variable. Comprehensive global puzzle solvability and exact milestone playthrough catalog can be deferred until concrete M2 authoring demands them.
- **Illustrative response only; schema not accepted:** `{ revision, world_state: <player-visible projection>, presentation: [{cue, duration_ms}], choices: [{id,label}], timer?: {remaining_ms} }`. A story-context/node ID or separate transition field might be useful but is not assumed. This example is **not** a final Rust API, schema or guarantee of full world-state disclosure. Choice input plus host timing/idle-expiry observations and existing presentation readiness are the likely minimal host messages, with no action-success callback.

**Topics for subsequent design:** exact clock reference and timer activation, host timer-event contract, interaction gating under M1.8, save compatibility, authoring format and a minimal semantic result. Distinguish these implementation details from the gameplay concepts already agreed.

## M2 scene-and-choice review — working decisions (2026-10-30)

- **Do not overload “scene”:** distinguish the **narrative situation / authored gameplay context** (rules, current available choices and deadline, owned by GRIMM), a **story graph node** (authored structural unit, not required for every local action), and a **presentation shot/setting** (visual animation, camera or text presentation, owned by the concrete renderer). Naming and exact schema remain to be chosen; none must be universally 1:1.
- **Single choice-time resolution:** GRIMM receives an authored choice plus host-correlated selection time, evaluates conditions and typed game/world state, commits the result and returns **authorized presentation cues, required visual state projection and allowed next choices**. The renderer decides the concrete *how*, not gameplay eligibility, consequences or timers. Presenting and choice readiness remain distinct under M1.8.
- **Authored repeated actions:** game state such as `door_damage` can be updated on each `break_door` choice using a `character_strength` value; the resulting thresholds can select `break_door_nothing_happens`, `break_door_progress`, `break_door_almost_open` or `break_door_success`. Other choices can be made available from the same facts (`door_damage > 0` → `squeeze_through`) and resolve using a `character_dex` check. These are **illustrative authored rules/cues**, not engine-hardcoded actions, thresholds or asset IDs.
- **Revision ≠ node ≠ deadline:** revision-scoped choices protect against stale clicks or duplicate lifecycle events after the offered set changes. Updating a revision does **not** automatically re-enter a narrative context or reset its countdown.
- **Timer authority:** a renderer can be asked to *display* a timer and may notify that it appears to have expired; only GRIMM can validate expiry based on its authored deadline and host-supplied time observations and commit the authored timeout consequence. The renderer cannot reset/extend the canonical deadline.
- **Navigation is never implicit:** a previously visited story node is **not** automatically accessible again. A Telltale-like return to the key rack needs an explicit authored choice/transition (e.g. `leave_and_search_for_key`); a host-led 3D/RPG game may instead exit the narrative interaction into host-owned free exploration and later trigger a new GRIMM interaction at the key rack. **Both must be modeled as legal transitions, not invented by the checker.**
- **Graph checking:** check only **actual reachable transitions from each reachable (story context, committed state, remaining time, host-navigation assumptions)**. Finding a key in some graph node does not make the door puzzle solvable unless the player can legally reach that node from the current situation. Conditional exits need not be universally available, but unintended trapped nonterminal states should be diagnosed; intentional terminal failure/timeout is allowed. Bounded analysis may return *inconclusive* rather than assume free backtracking.

**Conceptually answered:** local effects without node transition; renderer ownership of the *presentation process*; repeated attempts using world state; engine-selected presentation cues; persistence of canonical facts. **Still open in M2:** precise event/state-projection/schema contracts, revision lifecycle, condition/checker algorithm, expiry event handling and readiness timing. Renderer-specific cue-to-asset binding remains later M5 work.

## Choice-driven scene and adventure modeling

**M2 direction agreed:** GRIMM makes **one full, deterministic decision at choice selection**, using host-supplied selection time, canonical state, authored action duration and known scene deadlines. It commits the resulting narrative/world state and gives the renderer its **fully determined presentation sequence and subsequent valid choices**. The next choices are not input-ready until their presentation has finished under the M1.8 readiness boundary. No continuously running action, provisional `will_succeed` state, renderer `ActionSucceeded` event, or action progress callbacks are needed for these authored actions. Idle real-time timeouts still require external elapsed observations. The questions below concern details of that model, not whether to add a pending-action simulator.

| ID | Status | Target | Question |
|---|---|---|---|
| OPEN-SCENE-01 | Decided (direction above; API open) | M2 | How does a resolved choice change state and update offers **within the same narrative scene** while reusing existing action/event types? |
| OPEN-SCENE-02 | Investigating (renderer presents; protocol open) | M2 | What semantic feedback and presentation-sequence structure accompanies a resolved in-scene choice, and how are new offers gated until ready? |
| OPEN-SCENE-03 | Investigating (revision != timer reset) | M2 | How are interaction revisions updated after a committed in-scene action without replaying the original scene introduction or restarting a scene deadline? |
| OPEN-SCENE-04 | Open | M2 | How should authored real-time action durations, separate fictional world-time costs and host-supplied selection timestamps be represented and validated? |
| OPEN-SCENE-05 | Investigating (explicit reachability) | M2 | How does bounded checking cover **only explicitly traversable authored transitions** from each reachable context/state/time (including authored return choices), rather than assuming previous nodes can be revisited? How are deliberate bad endings distinguished from accidental trapped states? |
| OPEN-SCENE-06 | Investigating (minimal schema; save redesign deferred) | M2 | What is the minimum backward-compatible story/schema/save change for in-scene resolution, full presentation sequences and time costs? |
| OPEN-SCENE-07 | Decided (full visible projection; lifetime/schema open) | M2–M5 | What committed state projection (snapshot, events or both) lets presentation rebuild an open drawer, revealed item and hotspots after refresh, revisit, save/load or reconnection? |
| OPEN-SCENE-08 | Open | M4–M5 | How do game-specific bindings map semantic facts (e.g. `drawer_open`) to asset/mesh/sprite states, without putting renderer data in the headless engine? |
| OPEN-SCENE-09 | Open | M2–M5 | Since GRIMM already decides outcomes before presenting, how should a renderer stage the action sequence, final pose and next-choice activation without an extra outcome/commit callback? |
| OPEN-SCENE-10 | Open | M4–M5 | If an embedded host owns a physical object, which facts belong to GRIMM and how are host-world updates reconciled without competing authorities? |
| OPEN-SCENE-11 | Open | M2 | What renderer-neutral message describes GRIMM's **fully determined** outcome and prescribed presentation sequence at choice selection (including effective duration, target state, transition and choices)? |
| OPEN-SCENE-13 | Decided (direction above; schema open) | M2–M3 | How should authored progress across **separate repeated Choices** (e.g. 1–3 attempts to break a door based on `strength`) be modeled with existing state/effects and checked for deadlocks and unbounded repetition, without a dedicated universal action-progress primitive? |
| OPEN-SCENE-14 | Investigating (GRIMM chooses cue; asset binding open) | M2–M5 | How does a fully resolved choice select a **semantic presentation cue** (`try_break_door`, `success_break_door`, optional strength/progress variants), and where are those cues bound to a game's concrete animations/assets without renderer-specific types in GRIMM? |
| OPEN-SCENE-15 | Open | M2–M3 | How do distinct repeated attempts consume cumulative scene deadline, persist progress through save/load, and produce different authored success/failure outcomes as `strength` or progress changes? |
| OPEN-SCENE-12 | Open | Later | If independent physics/NPC/host events might invalidate an action *after* selection, is a different integration mode needed? Do not impose a provisional-action lifecycle on authored M2 scenes. |
| OPEN-TIME-01 | Decided (game_action elapsed timer input) | M2 | How is renderer/host-reported **elapsed duration relative to a known timer activation** represented, especially across successive cue sheets in the same narrative situation? How is it kept separate from fictional world chronology? |
| OPEN-TIME-02 | Open | M2 | How do authored success, failure and deadline-cut effects commit in one atomic transition, including any predeclared partial consequences? |
| OPEN-TIME-03 | Decided (author comparison semantics; reference interval open) | M2 | Equality at the deadline is resolved by authored comparisons (`<`, `<=`, `=`, `>=`, `>`), **not a global success rule**. Precisely define `time_passed` and deterministic units/overflow. |
| OPEN-TIME-04 | Open | M2 | Which gameplay inputs remain blocked during an already-resolved presentation sequence, and which host/session controls remain available? |
| OPEN-TIME-05 | Deferred for new M2 semantics (legacy compatibility required) | Later | Can M2 restrict saves to stable/untimed presentation boundaries and avoid a new Pause feature while **preserving existing M1.75 pause/save behavior**? How does restore reconstruct the settled view? |
| OPEN-TIME-06 | Open | M2–M5 | How does a renderer execute the engine-authorized presentation duration, including a **shortened action cut by a known scene deadline**, without adding fictional time or re-deciding the outcome? |
| OPEN-TIME-07 | Deferred | Later | Would concurrent player actions require a different model? M2 intentionally handles one selected authored choice at a time. |
| OPEN-TIME-08 | Open | M2–M5 | How does interrupted/paused/reloaded presentation resume using the same pre-resolved semantic outcome and original game timing? |
| OPEN-TIME-09 | Investigating (timer value on cue sheet) | M2 | How does the story format distinguish legacy **choice-selection deadlines** (silence causes authored transition) from **scene-completion deadlines** (objective must finish in time)? |
| OPEN-TIME-10 | Open | M2 | Which **derived arithmetic facts** (effective action duration, progress fraction, time remaining) may authored conditions use when the known deadline cuts a choice short? |
| OPEN-TIME-11 | Open | M2–M3 | How do authored outcomes combine arithmetic action progress with already committed character stats such as `suspicion` without hardcoding specific checks? |
| OPEN-TIME-12 | Deferred (exclude simultaneous active timers in M2) | Later | Should a later adventure ever have both deadline types active simultaneously? M2 does not need overlap/precedence rules. |
| OPEN-TIME-13 | Open | M2 | What presentation is emitted for an action already known to miss the deadline: the shortened attempt, immediate failure, follow-up scene narration and choice gating? |
| OPEN-TIME-14 | Decided (illustrative game_action + timeout; serialization open) | M2 | Define the minimal revision-scoped event shape for `[action_selected(A), timer_elapsed(5s)]` and silent `timer_elapsed(20s)`; when does the timer activate after the ordered Cue Sheet presentation, and how is duplicate/late input handled? |
| OPEN-TIME-15 | Open | M2 | How is canonical **logical scene time** advanced by each resolved action or deadline cut without double-counting real-time observations of player hesitation? |
| OPEN-TIME-16 | Open | M2 | If a host delivers a choice late, which timestamp/provenance determines whether it was still valid at selection? How are stale interactions rejected? |
| OPEN-DEMO-01 | Open | Every milestone | What complete playable acceptance game and regression routes demonstrate all new mechanics, state persistence and success/failure paths? |
| OPEN-DEMO-02 | Open | M2–M3 | How do demos prove that a door attempt at T=6 with a five-second duration and T=10 deadline is resolved at selection to a **four-second failure presentation**, without an action-completion callback? |
| OPEN-PUZZLE-01 | Open | M3 | Which item/inventory features require more than typed state variables: identity, ownership, location, consumption, quantities, visibility? |
| OPEN-PUZZLE-02 | Open | M3 | How do authored MacGyver-like combinations and multiple puzzle solutions remain conditional choices rather than universal `Inspect/Use/Combine` commands? |
| OPEN-PUZZLE-03 | Open | M3 | How do inventory changes affect subsequent offers and remain persistent/replayable without proliferating graph nodes? |
| OPEN-STAT-01 | Open | M3 | When do RPG attributes, skills, relationships and resources need explicit character ownership rather than normal typed variables? |
| OPEN-STAT-02 | Open | M3 | If probabilistic checks are desired later, how are input seeds and outcomes recorded; can initial threshold checks remain deterministic? |
| OPEN-ADV-01 | Open | M4 | How do persistent locations, objects and recurring characters differ from narrative scenes and presentation phases? |
| OPEN-ADV-02 | Open | M4 | How should LucasArts-like exploration and puzzles enable Telltale-style staged, consequential narrative choices in the same authored adventure? |
| OPEN-ADV-03 | Open | M4–M5 | How do graphical hotspots and text labels map to the same semantic offered choice within explicitly composed games? |
| OPEN-ADV-04 | Open | M4 | How does bounded playability distinguish explicitly authored returns, host-led open-world travel and intended failure endings from accidental softlocks, **without granting implicit access to previously visited story nodes**? |
| OPEN-ADV-05 | Open | M4–M5 | How does a narrative interaction **return control to an external host** (e.g. BG-style exploration resumes), and how does a later world trigger enter a key-rack or locked-door narrative context? What reachable host navigation can the authored checker safely know or require the host to declare? |

### Guiding boundary

The renderer reports **input**, not the outcome. GRIMM resolves the authored action **at selection**, commits the resulting state and outputs a prescribed presentation sequence and next offers. The renderer only presents that result; M1.8 readiness prevents showing or accepting the next choices too soon. Persistent state belongs to the responsible canonical game system; visual animation is presentation, not a source of gameplay truth. Real-time idleness still requires host time observations. More dynamic external interruptions belong to later embedded-host investigations.

## Game composition, renderer origin and packaging

**Working direction (not a fixed ownership model or schema):** A playable game composes authored gameplay/story rules, GRIMM's headless narrative engine, a **concrete game-specific presentation/runtime implementation**, the assets and mappings it needs, and potentially more components. In a standalone GRIMM-led game the game configuration may select the concrete renderer; in an embedded game (e.g. Bevy) the external host may instead own the application and instantiate GRIMM as a narrative subsystem. A story graph alone is not the game. **A renderer claiming 3D capability cannot render an arbitrary game's 3D world.** Renderer/profile interchangeability is not a product requirement. The details below remain OPEN until a complete graphical demo motivates a minimal contract.

| ID | Status | Target | Question |
|---|---|---|---|
| OPEN-GAME-01 | Open | M4–M5 | What exact artifacts make up a game definition/package versus an engine instance, authored story graph, configured concrete renderer, renderer-specific assets, auxiliary game modules and runtime host? |
| OPEN-GAME-02 | Open | M5 | How exactly does a game definition **identify the renderer implementation and its source** (workspace crate, compiled binary, shipped module, package, host factory, external dependency), configure it and establish compatible versions? What are the loading/trust constraints? |
| OPEN-GAME-03 | Open | M5 | What minimal game composition/bootstrapping contract should the host resolve to instantiate the selected renderer, engine, adapters and any additional game-specific components? |
| OPEN-GAME-04 | Open | M5 | Where are the game-specific rooms, scenes, hotspots, actor rigs, cameras, animations, audio and semantic choice bindings stored and loaded? Which require custom renderer code, and which can live in data? |
| OPEN-GAME-05 | Open | M5 | What constitutes a complete distributable game: executable/modules, story/game rules, renderer implementation and configuration, assets, resource locations, version and save compatibility? How are missing components diagnosed? |
| OPEN-GAME-06 | Open | M5 | Which game-specific checks run **before launch** for renderer source/version, asset existence, incompatible bindings, required components and broken choice/scene mapping? Which errors are author-time versus runtime? |
| OPEN-GAME-07 | Open | Later | Should multiple separate game compositions ever share one authored rules graph while selecting different renderers and asset packages? If supported, how is parity tested? **Do not assume cross-renderer compatibility by default.** |
| OPEN-GAME-08 | Open | M2–M5 | What, if anything, do current CLI games need to declare about their concrete renderer today? Can we leave them unchanged until an M5 full graphical-game package demonstrates the right format? |
| OPEN-GAME-09 | Open | M5 | How does the game-selected renderer bind action execution duration, partial-action interruption outcomes and M1.8 presentation gating to its specific authored animations without introducing a second authoritative timeline? |
| OPEN-GAME-10 | Open | M5+ | Which **additional game components** beyond engine/renderer/story/assets might be legitimate (game logic modules, dialogue tools, audio, physics, scripts, UI, save facilities), and how are their permissions and authority limited without pre-building a general plugin framework? |

**Boundaries:** Game-specific code/assets/configuration belong to the surrounding host/composition, not inside the headless core. Do not require the story graph to identify an external host's renderer in the embedded mode. The engine remains authoritative over state, choices and validated transitions. M2–M4 build full playable authored CLI demos with existing composition, and M5 designs the minimum concrete game packaging model using a real, asset-complete graphic adventure. Do not define a generic renderer capability profile as the primary identity or treat a 3D feature list as sufficient for game compatibility.

## Host integration: standalone game versus embedded narrative engine

**Open architectural direction:** GRIMM is a headless **narrative engine**, not necessarily the game's main loop or entry point. Standalone text adventures may launch GRIMM and let it drive the current narrative scene. A host game engine (e.g. Bevy) may instead own the 3D world, movement, physics, input and main loop and invoke GRIMM **only when narrative resolution is required**. Both are plausible; neither should be prematurely encoded as the sole composition model. Do not confuse a *game's main engine* with GRIMM's narrative domain authority.

**Illustrative integration:** player reaches a 3D area trigger → host requests narrative entry/transition from GRIMM → GRIMM evaluates authored conditions/choices and emits validated narrative outcomes → host applies permitted game-world consequences (such as unlocking a door, starting dialogue or updating a quest) through an explicit bridge. GRIMM must not silently claim authority over spatial simulation that belongs to the host; the host must not bypass GRIMM's validation for narrative state that GRIMM owns. Cross-boundary commits/retries need explicit policy.

| ID | Status | Target | Question |
|---|---|---|---|
| OPEN-HOST-01 | Open | M4–M5 | What is the application **entry point**: standalone GRIMM-driven game, externally driven host with embedded GRIMM, or both? Which is proven by which playable demo? |
| OPEN-HOST-02 | Open | M4–M5 | Which actor owns the **main loop**, time observations, pause/resume, input routing and lifecycle when GRIMM is embedded in Bevy or another runtime? |
| OPEN-HOST-03 | Open | M4–M5 | How does an external host **enter, suspend, resume, cancel or leave** a narrative scene from world events (e.g. entering a spatial trigger), without inventing renderer-specific node APIs? |
| OPEN-HOST-04 | Open | M4–M5 | Which state is **canonical to GRIMM** (story choices, narrative consequences, story deadlines) versus canonical to the host (position, physics, visual/world simulation, perhaps inventory)? What is the contract for projecting host facts into narrative conditions? |
| OPEN-HOST-05 | Open | M4–M5 | How do committed GRIMM story outcomes become authorized, idempotent **host commands/events** (e.g. unlock a 3D door), and how are failures, retries, atomicity and divergence handled across two state owners? |
| OPEN-HOST-06 | Open | M5 | Can a single game-specific **composition root** be owned by the external engine while GRIMM remains an embedded library/service, rather than the game definition loading a renderer as its dependency? |
| OPEN-HOST-07 | Open | M5 | How should action execution times, scene-completion deadlines, animation and external host pause/time-scale work when the 3D engine drives the frame clock? What happens during scene unload, disconnect or level transitions? |
| OPEN-HOST-08 | Open | M4–M5 | Are GRIMM narrative interactions modal (world gameplay temporarily suspended), concurrent with exploration, or selectable per authored scene? What new risks arise when the player can leave mid-action? |
| OPEN-HOST-09 | Open | M5 | What is the smallest **embedded-host demonstration** (e.g. entering a 3D/2D trigger, resolving an authored choice, applying a validated world effect) needed before choosing a concrete integration API? |
| OPEN-HOST-10 | Open | M5 | How are save/load, replay and compatibility coordinated when both the external world engine and GRIMM have independent state and versions? |

**Guardrails:** Reuse semantic events/actions and explicit host adapters rather than letting Bevy types leak into the GRIMM core. A shared crate or one universal state store is **not** required. GRIMM can be authoritative for the narrative domain without owning all world state of an embedded host game. Final entry-point, command/outbox, ownership and runtime contracts remain open until exercised with a real playable embedding demo.

## Later: open-world / dynamic narrative (deferred)

**Do not specify or implement dynamic node providers in M2–M5.** Once an authored adventure can model locations, inventory, puzzles, persistent character state and scene transitions, revisit whether an open-world/Story-RPG mode benefits from live graph extension. A future provider might be a human dungeon master, authoring interface, rule system or LLM, but its boundary and proposal schema are intentionally **undefined today**.

- **OPEN-DYN-01 — Deferred:** What would a provider need to extend in a mature adventure model: situations, locations, item interactions, character state or nodes? Do not assume nodes alone are enough.
- **OPEN-DYN-02 — Deferred:** How would a mature graph checker validate reachability, escape routes and explicitly open frontiers when content can arrive later?
- **OPEN-DYN-03 — Deferred:** Which runtime-authoring permissions, atomicity, consistency, persistence and replay guarantees would a human DM or optional generative provider require?

## Later: generative narrative and presentation (deferred)

All historical generative questions remain **deferred** rather than resolved. They are not dependencies of the authored adventure roadmap.

| ID | Status | Target | Question / choice to resolve |
|---|---|---|---|
| OPEN-GEN-01 | Deferred | Later | **Boundary of generation:** Which information is authoritative narrative content in a committed node, and which is purely a renderer's stylistic transformation? Should optional style rewriting exist at all, or be left to later presentation experimentation? |
| OPEN-GEN-02 | Deferred | Later | **Proposal granularity:** Should a generator propose a whole node, an event plus presentation content, a bounded sequence of nodes, or a constrained combination? How do these relate to authored nodes, narrative-only `continue_to` and M1.8 gated interactions? |
| OPEN-GEN-03 | Deferred | Later | **Proposal contracts:** What minimum typed structure carries node identity, content, choice targets, conditions/effects, references, new facts and event provenance? What can safely remain free text? |
| OPEN-GEN-04 | Deferred | Later | **Truth versus prose:** How can text be constrained not to claim canonical events, character deaths, opened doors, learned secrets or other state changes absent from committed events? Which checks can be deterministic, and which are only best-effort? What happens when prose contradicts state? |
| OPEN-GEN-05 | Deferred | Later | **New facts and authority:** Which new characters, objects, locations, relationships, knowledge or world variables may a proposal introduce, and under whose authorization? When must a proposed fact be represented by an explicit validated event/effect rather than narration? |
| OPEN-GEN-06 | Deferred | Later | **Safe context projection:** What world state, prior events, character beliefs, author constraints and player history may a generator see? How do we avoid leaking hidden facts via prompt, output, logs, traces and persisted proposals? |
| OPEN-GEN-07 | Deferred | Later | **Author constraints:** Which text/dialogue is immutable, which may be paraphrased, and what about canon, tone, genre, character voice, safety rules and continuity? Can constraints be declared in authoring data? |
| OPEN-GEN-08 | Deferred | Later | **Validation and fallback:** Which failures cause reject, repair/retry (bounded), asking a player for clarification, or authored/rule-based fallback? How do we avoid loops, softlocks and a generator silently manufacturing an unearned success? |
| OPEN-GEN-09 | Deferred | Later | **Identity and graph lifetime:** How are dynamic node/entity IDs assigned and collision-free across save/restore, branching, replay and imported authored stories? Are rejected proposals stored? How are overlay revisions and stale proposals ordered against timeouts/actions? |
| OPEN-GEN-10 | Deferred | Later | **Determinism:** Save raw model response, normalized accepted proposal, resolved action/event, or all of these? What is needed for exact engine replay, audit, crash recovery and optional re-rendering without recalling a model? |
| OPEN-GEN-11 | Deferred | Later | **Presentation variance:** If wording is generated after canonical commit, can different users or renderers see divergent implications? What must be pinned to a session, preserved on save/load, and held fixed for exact authored dialogue? |
| OPEN-GEN-12 | Deferred | Later | **Latency and interaction fairness:** When may content generation occur relative to `Presenting`, `PresentationComplete`, `InteractionReady`, pause/resume and timed decisions? Generation or render latency must not consume unseen decision time or silently advance the world clock. |
| OPEN-GEN-13 | Deferred | Later | **Provider lifecycle and privacy:** What optional interface handles timeouts, unavailable/offline providers, redaction, output limits and cancellation? No LLM SDK or network dependency may enter the headless engine. |
| OPEN-GEN-14 | Deferred | Later | **Testing and evidence:** How do rule-based fixtures, adversarial invalid proposals, bounded graph checks, schema fuzzing, recorded model responses and human narrative playtests demonstrate continuity and reject invented facts? |
| OPEN-GEN-15 | Deferred | Later | **Player action versus generated narrative:** When free-text intent introduces an unscripted action, who proposes the resulting node/event, and how are resolution, player consent/ambiguity and engine validation separated? |
| OPEN-GEN-16 | Deferred | Later | **Milestone scope:** Is a separate optional generative presentation adapter valuable enough to implement later? If so, how does it avoid duplicate narrative authority and reuse the existing presentation interface? This does not block M2. |

The future provider architecture, validation of generated prose, visibility of hidden facts and optional stylistic rendering must only be considered after the authored adventure state model is established.

## General architecture backlog

| ID | Status | Question |
|---|---|---|
| OPEN-ARCH-01 | Open | Which exact Rust protocol API and extension points should be stabilized, and when? |
| OPEN-ARCH-02 | Open | Which asynchronous host/transport conventions will be needed without making the core async or renderer-dependent? |
| OPEN-ARCH-03 | Open | How should future story schema and save formats evolve while retaining compatibility? |
| OPEN-ARCH-04 | Open | How granular should adapter/presentation snapshots be for reconnection and replay? |

The earlier unstructured list lived in `docs/architecture.md#open-design-questions`. This catalog preserves those concerns and expands the unresolved generation questions; no existing accepted ADR is retroactively altered.
