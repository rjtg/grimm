# Cross-milestone consistency audit — M2 Cue Sheets and DSL versus M3–M5

Status: **review findings, not new adopted APIs**. Read with [M2 locked-room reference](milestones/m2-cue-sheet-reference.md), [requirements](requirements.md), [architecture](architecture.md), [roadmap](roadmap.md) and [open questions](open-questions.md). This audit distinguishes settled principles, unresolved extensions and **actual wording conflicts**. Do not expand M2 scope merely to solve later host/graphics issues.

## Current architectural decision (supersedes earlier snapshot proposal)

**World State is GRIMM-internal.** Story-authored variables and their current values belong to GRIMM and are used to evaluate game rules. **M2 Cue Sheets do not include a `world_state` field**: only ordered `play(animation_id)`, accepted actions, optional timer and response ID. A concrete renderer may maintain its own presentation state and later contribute that state to a coordinated game save, but renderer state is not authoritative for story rules. Earlier claims below that a full renderer-visible state snapshot is required in every Cue Sheet are **superseded**. The remaining M5 question is how to coordinate/restage renderer presentation without introducing a universal snapshot contract into M2.

## Settled by M2 / implications for later milestones

1. **Shared authored action language:** repeated interactions, stat tests, progress, branches, alternative solutions and time costs are rules over state; no fixed `Inspect/Take/Use` vocabulary or separate mini-game execution engine. M3 should extend **domain semantics** (item identity, quantity, ownership, combinations and possibly character ownership), not duplicate the choice runner.
2. **Narrative nodes are primarily authoring structure:** one node/context can represent a complete mini-game. M4 must not require a graph node per movement, puzzle attempt or animation. Context transitions/returns are explicitly authored for GRIMM-led adventures.
3. **Renderer-facing Cue Sheets are graph-private:** M5's concrete renderer can bind semantic `play` cues and `actions` to actual assets/hotspots. It does not need a `node_changed` field to display authored gameplay. **The surrounding concrete game composition** remains responsible for supplying those bindings and assets.
4. **Single outcome at choice selection:** GRIMM determines canonical effects and resultant presentation from `game_action` plus current state and authored rules; no renderer success callback or continuous action-progress simulation for authored M2 interactions.
5. **GRIMM-internal canonical World State:** state variables and rules stay inside GRIMM; M2 emits only the selected cues, offered actions and optional timer. Later renderers maintain/reconstruct their own presentation state, potentially contributing a renderer-owned save component.
6. **Renderer elapsed timer input:** an M2 Cue Sheet contains the relevant elapsed/total timer. The renderer plays required cues first, then enables actions and counts from that post-action baseline. No double-charging of action duration; `game_action { ref, action: timeout, scene_timer: [total of total]s }` expresses silence.

## Confirmed documentation conflicts / implementation hazards

**C1 — M2 Save/Pause promises conflict.** `SCENE-09` and `SCENE-10` still mandate save/load or pause during unfinished presentation, whereas `TIME-AUTH-04` and later working decisions propose *no new M2 pause feature* and deferring M2 saves at active timers. Existing M1.75/M1.8 save/pause capabilities are already implemented and must remain backwards compatible. **Action:** distinguish preserving existing APIs/tests from requiring a new M2 mid-cue-sheet checkpoint behavior; revise acceptance criteria before implementation.

**C2 — M2 versus M3 responsibilities overlap.** M2 DSL already covers typed variables, `strength`, `dex`, state-dependent repeated attempts, alternate puzzle solutions and time. `STAT-01/02`, `PUZZLE-03/04` and M3 roadmap imply some are introduced first in M3. **Action:** frame M3 as first-class inventory/item ownership/quantity/combinations and richer character semantics; reuse M2 DSL/rule engine. Avoid promising redundant mechanics.

**C3 — M5 renderer compatibility phrasing outdated.** M5 roadmap acceptance still says **`compatible renderer capability checks`**, while architecture explicitly rejects assuming a renderer is suitable merely because it says “supports 3D”. **Action:** replace generic capability wording with validation of the **concrete selected runtime, assets, cues and choice/hotspot bindings**. Cross-renderer parity remains optional when explicitly authored.

**C4 — M2 timeout/action duration wording predates Cue Sheets.** Some `SCENE-*` acceptance text still suggests continuous interruption, saved pending action/progress, or tests while animation runs. The newer agreed M2 model computes the result *once* and schedules a shortened cue when a known deadline is crossed. **Action:** scope these tests to computed effective action duration, authored branching and ordered presentation; do not require action-completion callbacks.

**C5 — Presentation gating versus total state commitment is not identical.** GRIMM may commit `drawer_open=true` before the renderer finishes its opening animation, but the state variable is **not** placed in the M2 Cue Sheet. A graphical renderer must be able to animate from a previously shown state to a final snapshot without revealing an object in its final pose too early. The M2 contract makes renderer responsible for staged presentation, but **does not yet say whether a single final snapshot and ordered cues always suffice** for M5. **Action:** M5 must test a stateful animated object and optional mid-sequence visibility/choice reveal; do not expose story node transitions as a workaround.

**C6 — Timer display versus cinematic/external world time.** The agreed M2 timer counts only after Cue Sheet `play` finishes, even though authored action cost is already booked. M4 cinematic action chains or M5 host-world real-time physics may continue while a cinematic cue plays, creating a different desired timing model. **Action:** treat M2 timers as *authored interactive budgets*, not universal physical time. If later use cases need continuously running hazards during playback, explicitly design a separate opt-in timing/host-integration policy rather than silently changing M2 semantics.

**C7 — Canonical game rule state versus renderer/host state.** GRIMM's World State stays internal; in M5, the renderer may save its presentation state and an embedded host may separately own physical objects/inventory. **Action:** design game-level coordinated save/load and consistency, without adding mandatory state projections to M2 Cue Sheets.

## Clarification: animation IDs are already sufficient at the GRIMM boundary

A Cue Sheet's `play: [animation_id]` is **fully sufficient to command presentation**. The **concrete renderer** interprets each ID as whatever game-specific animation, text, camera staging, sound or sequence it implements. No additional GRIMM fields, central asset-mapping registry or game-composition mapping layer is needed **by the Cue Sheet protocol**. The term “game composition” only means the complete game ships a renderer and whatever assets/configuration that concrete renderer itself requires. Thus M5 needs to **implement and optionally check support for all authored cue IDs**, not redesign or enrich the Cue Sheet. Previous wording implying a new mapping contract at GRIMM's boundary was misleading.

This does **not** settle the independent issue in C5 of when a renderer displays the final committed world-state snapshot during its animation. That is a presentation sequencing concern, not a demand to replace `play(animation_id)`.

## Open questions narrowed or answered in principle

- `OPEN-PUZZLE-02/03`: **answered in principle**: item discovery, combinations and changed choice availability use the same conditional choice + state + effect pipeline. Item identity/quantities/ownership remain open (`OPEN-PUZZLE-01`).
- `OPEN-STAT-01`: simple numeric attributes need no new base system; **only whether richer character-specific ownership/skills need more** stays open. `OPEN-STAT-02` probabilistic resolution remains open/deferred.
- `OPEN-ADV-02`: LucasArts puzzles and Telltale branches **share one DSL and Cue Sheet protocol**; designing complete location-spanning authored adventure remains M4.
- `OPEN-ADV-03`: a hotspot/text-menu activates the same offered semantic action ID; concrete M5 binding, capability/asset validation and input focus still open.
- `OPEN-SCENE-08/14`, `OPEN-GAME-04/09`: selected authored cue ID/ordered presentation is settled; how concrete media is mapped, duration-fitted and validated remains M5.
- `OPEN-GAME-08`: likely no new renderer declaration is needed in existing CLI stories for M2; verify in implementation rather than impose a manifest.
- `OPEN-GEN-04/11`: later generated descriptions must not misrepresent GRIMM's committed state. Cue Sheet separation provides an authority boundary but **does not yet solve prose truthfulness**.

## New later-milestone questions (do not pull into M2)

- **L-M3-01:** When does the M2 local/global variable scheme become insufficient for first-class item identity, location, quantity, possession, references and context-local lifetime?
- **L-M4-01:** Can a single long-running interactive narrative situation reuse/resume its authored state and timer across explicit graph/context exits and re-entry, or must a new context begin? What does context-local reset mean?
- **L-M4-02:** For linked mini-games spanning multiple authored contexts, how do structural authored graph validation and conditional state reachability stay tractable without assuming free backtracking?
- **L-M5-01:** How does a graphical renderer use ordered semantic Cue Sheets and renderer-owned presentation state for correct starting pose, animation, final pose and hotspot visibility, especially after load or reconnect?
- **L-M5-02:** How does a concrete renderer or its game build optionally validate that every authored `play(animation_id)` is supported, without changing GRIMM's Cue Sheet protocol or requiring an extra generic binding layer?
- **L-M5-03:** In an embedded host, how do GRIMM-internal canonical state, renderer-owned presentation state and host-owned world state stay consistent across game-level checkpoints?
- **L-M5-04:** Is M2's pause-during-`play` timer behavior sufficient for graphical/host-owned ongoing hazards, or does a later mode need timers continuing through presentation? How does that interact with externally cancellable actions?
- **L-LATER-01:** If independent host events invalidate a pre-resolved choice during playback, what explicit opt-in integration contract preserves deterministic narrative authority without breaking ordinary Cue Sheet replay?

## Recommendation

**Do not expand the M2 implementation.** Before the agent begins, reconcile C1–C4's outdated acceptance wording, and ensure C5's staged presentation is explicitly tested. Retain C6–C7 and L-M5-* as future questions. Keep new M2 DSL and `The Locked Room` reference as the main acceptance source; no general RPG framework, renderer package format, host physics integration or dynamic node generation should enter M2 by implication.
