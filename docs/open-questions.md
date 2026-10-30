# Open design questions

This is GRIMM's running catalog of **unresolved** questions, not accepted APIs or milestone commitments. Record decisions, rationale, tests and ADR links when the relevant authored demo motivates them. Status: Open, Investigating, Decided (with link), Deferred.

## Choice-driven scene and adventure modeling

| ID | Status | Target | Question |
|---|---|---|---|
| OPEN-SCENE-01 | Open | M2 | How should a choice commit effects **without leaving the current narrative scene**? Can we reuse existing actions without a separate player-verb taxonomy? |
| OPEN-SCENE-02 | Open | M2 | How do we expose newly available choices and local narrative feedback after an in-scene action without replaying the entire required presentation? |
| OPEN-SCENE-03 | Open | M2 | How do interaction revisions change on in-scene actions while preserving **one running situation deadline** instead of resetting it with each refreshed choice list? How do M1.75 timeout precedence and M1.8 readiness continue to apply? |
| OPEN-SCENE-04 | Open | M2 | How should authored time costs, world-time advances and host-observed real time interact with several actions in one scene? |
| OPEN-SCENE-05 | Open | M2 | How does bounded checking distinguish intended revisits, meaningful in-scene loops, successful exits, explicit timeouts and accidental deadlocks using node plus relevant state? |
| OPEN-SCENE-06 | Open | M2 | What is the smallest backward-compatible story/schema and save change needed for local effects and feedback? |
| OPEN-SCENE-07 | Open | M2–M5 | What minimum **committed state projection** lets presentation rebuild a drawer's open/closed pose, revealed items and current hotspots after choice refresh, revisit, save/load and reconnection: snapshots, events or both? |
| OPEN-SCENE-08 | Open | M4–M5 | How do game-specific presentation bindings map authored facts such as `drawer_open` to mesh/sprite poses, items and animation endpoints without encoding renderer asset details in the headless engine? |
| OPEN-SCENE-09 | Open | M2–M5 | How is a provisional animation of an action reconciled after canonical success or a mid-action authored interruption (e.g. drawer partly open when an investigator enters)? |
| OPEN-SCENE-10 | Open | M4–M5 | How do GRIMM-owned narrative facts and host-owned spatial/physics object state remain consistent when both refer to the same object, without multiple competing authoritative owners? |
| OPEN-SCENE-11 | Open | M2–M5 | What minimal **planned-outcome / action-presentation contract** can GRIMM emit immediately at T0 (intended success/failure, duration, animation cue, revision), before completion-only effects commit? How can a timeout replace that provisional plan with an authored interruption outcome? |
| OPEN-SCENE-12 | Open | M2–M5 | Is a planned outcome determined unconditionally from the T0 state, or only conditionally (`will_succeed_if_completed`) given later deadlines, external events and mutable state? Which facts are snapshotted vs reevaluated on completion? |
| OPEN-TIME-01 | Open | M2 | Should action-duration completion be reported as a revision-bound host message (`action completed, elapsed 5s`) while fictional world-time cost commits only on resolution? How do host-observed real-time choice/scene deadlines remain independent? |
| OPEN-TIME-02 | Open | M2 | Which action effects commit at acceptance, successful completion or an explicitly authored partial-progress checkpoint? Which effects/costs survive a mid-action scene timeout? |
| OPEN-TIME-03 | Open | M2 | How does the engine resolve event-time ordering when completion is reported **after** a deadline that fell inside its elapsed interval (door started t=6s, deadline t=10s, nominal completion t=11s)? How are exact ties and overshooting time observations handled? |
| OPEN-TIME-04 | Open | M2 | What gameplay input is blocked while an action executes, and which host/session commands (pause, save, load, quit) remain available? Are queued or repeated choice inputs dropped safely? |
| OPEN-TIME-05 | Open | M2 | How do pause/resume, save/load, renderer disconnect and restart restore an in-progress action, its elapsed/remaining duration and a separate ongoing deadline? |
| OPEN-TIME-06 | Open | M2–M5 | How can animation/progress and host-mediated action-completion acknowledgements use the **same authored execution duration** (e.g. five seconds), while the renderer cannot itself authorize success, mutate fictional time or prevent a prior scene expiry? |
| OPEN-TIME-08 | Open | M2–M5 | How should pause/resume, renderer lag/dropped frames, interrupted playback and load mid-animation re-synchronize animation phase to the authoritative action timeline without prolonging a decision timer? |
| OPEN-TIME-07 | Open | M2 | Can overlapping/combinable actions ever exist, or is exactly one player action in progress at a time the minimal initial constraint? |
| OPEN-TIME-09 | Open | M2 | How does the authored schema explicitly distinguish a **choice-selection deadline** (response must be selected by expiry; silence is an authored choice-like consequence) from a **scene-completion deadline** (objective must finish by expiry), while preserving legacy M1.75 `limit_ms`? |
| OPEN-TIME-10 | Open | M2 | Which fields make up a deterministic mid-action **interruption snapshot**: action ID, elapsed/remaining execution, normalized progress, committed variables, partial effects and time-to-deadline? Which are derived versus persisted? |
| OPEN-TIME-11 | Open | M2–M3 | How can authors express conditional interruption results combining action progress and character/world state (e.g. weapon partially hidden when a detective arrives, with `suspicion` affecting detection), without building a bespoke detection mechanic into the core? |
| OPEN-TIME-12 | Open | M2 | How do authored no-response transitions for Telltale dialogues coexist with scene objective timeouts, and can both be active simultaneously? If so, which must win? |
| OPEN-TIME-13 | Open | M2 | Must intermediate partial progress be visible to the player or only to the transition checker? How should interruption feedback, scene transitions and ongoing animations stay consistent? |
| OPEN-TIME-14 | Open | M2 | What minimal input contract reports **choice selected at host-correlated T0**, later host-observed elapsed time and optional presentation acknowledgements? Can GRIMM derive authored action completion/interruption without a renderer success message? |
| OPEN-TIME-15 | Open | M2 | Can action time be **commit-on-completion** for fictional world time while an independent, host-observed scene deadline can expire during the action? How do we avoid double-counting a completion report and already-accounted realtime observations? |
| OPEN-TIME-16 | Open | M2 | When the host observes time late (e.g. a 5s action begins at t=6, scene expiry t=10, next observation t=11), what timing evidence lets GRIMM derive 4/5 progress at interruption without renderer-authoritative success? What is persisted for replay? |
| OPEN-DEMO-02 | Open | M2–M3 | How will demo games show both timer kinds, a choice that occurs before a deadline but finishes afterward, a mid-action interruption, and two different authored outcomes driven by progress/character stats? |
| OPEN-DEMO-01 | Open | Every milestone | How do we document and automatically exercise every new gameplay capability in a small *playable* acceptance game, including success, failure, timeout and save/restore? |
| OPEN-PUZZLE-01 | Open | M3 | Which item/inventory features genuinely require more than typed state variables: unique identity, possession, location, consumption, quantities or visibility? |
| OPEN-PUZZLE-02 | Open | M3 | How do MacGyver-like item combinations and multi-solution puzzles stay **authored conditional choices**, rather than universal `Inspect/Use/Combine` commands? |
| OPEN-PUZZLE-03 | Open | M3 | How do inventory changes reproject choices and preserve persistence/replay without combinatorial node proliferation? |
| OPEN-STAT-01 | Open | M3 | When do RPG-style attributes, skills, relationships and resources need explicit character ownership rather than regular typed state variables? |
| OPEN-STAT-02 | Open | M3 | Should optional skill thresholds be deterministic initially? If random checks are desired later, how are seeds and results recorded for replay? |
| OPEN-ADV-01 | Open | M4 | How do persistent locations, objects and recurring characters differ from narrative scenes and their presentation lifecycle? |
| OPEN-ADV-02 | Open | M4 | How do Telltale-style staged, consequential decisions and LucasArts-style exploration/puzzle solutions interact in one semi-linear authored story? |
| OPEN-ADV-03 | Open | M4–M5 | How do text choice numbers and graphical hotspots/inventory combinations map to **the same engine-offered semantic choice**? |
| OPEN-ADV-04 | Open | M4 | What counts as a valid intentional exploration loop versus an accidental softlock in bounded playability diagnostics? |

### Guiding boundary

A choice is an **authored, currently executable semantic offer**. It may investigate, reveal, collect, combine, affect character stats, consume time, refresh available options or advance the story. An optional execution duration makes the selected action **in progress** and blocks further gameplay choices while a previously running **scene-completion** timer keeps progressing; the host must not sleep inside the engine. A separate **choice-selection** timer is satisfied by accepting a choice (or triggers authored silence/default consequences if unanswered). On scene deadline expiry mid-action, the engine evaluates an authored interruption using deterministic partial progress and committed state, not merely a static timeout target. Animation/progress for that action has the same authored execution duration (five seconds means five seconds), with presentation driven by the action timeline rather than extending it. Precise completion, failure and persistence policies are OPEN-TIME questions. The renderer reports the chosen input, not a success verdict; GRIMM **immediately reports an engine-authorized planned outcome and matching presentation cue at T0**. The planned outcome remains provisional until GRIMM applies authored completion/interruption rules using host time observations. Persistent facts (e.g. drawer open) belong to the authoritative game state and are presented through game-specific bindings; transient animation state does not replace them. The engine validates effects; renderers only present authorized offers. Neither a fixed LucasArts verb API nor a mandatory RPG ruleset is needed. A narrative scene is not the same as a persistent location.

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
