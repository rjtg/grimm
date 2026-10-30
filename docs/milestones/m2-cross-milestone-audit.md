# Cross-milestone consistency audit — M2 Cue Sheets and DSL versus M3–M5

Status: **review findings, not new adopted APIs**. Read with [M2 locked-room reference](m2-cue-sheet-reference.md), [requirements](../requirements.md), [architecture](../architecture.md), [roadmap](../roadmap.md) and [open questions](../open-questions.md). This audit distinguishes settled principles, unresolved extensions and **actual wording conflicts**. Do not expand M2 scope merely to solve later host/graphics issues.

## Current architectural decision (supersedes earlier snapshot proposal)

**World State is GRIMM-internal.** Story-authored variables and their current values belong to GRIMM and are used to evaluate game rules. **M2 Cue Sheets do not include a `world_state` field**: only ordered `cue(cue_id)`, accepted actions, optional timer and response ID. A concrete renderer may maintain its own presentation state and later contribute that state to a coordinated game save, but renderer state is not authoritative for story rules. Earlier claims below that a full renderer-visible state snapshot is required in every Cue Sheet are **superseded**. The remaining M5 question is how to coordinate/restage renderer presentation without introducing a universal snapshot contract into M2.

## Settled by M2 / implications for later milestones

1. **Shared authored action language:** repeated interactions, stat tests, progress, branches, alternative solutions and time costs are rules over state; no fixed `Inspect/Take/Use` vocabulary or separate mini-game execution engine. Historical M2 traces may use local IDs. The **replacement M3** establishes and indexes the later-game qualified cue/action naming convention and provides basic structural exit/dead-end linting. Asset comparisons remain renderer/IDE work; optional item or character frameworks are not roadmap commitments.
2. **Narrative nodes are primarily authoring structure:** one node/context can represent a complete mini-game. M4 must not require a graph node per movement, puzzle attempt or animation. Context transitions/returns are explicitly authored for GRIMM-led adventures.
3. **Renderer-facing Cue Sheets are graph-private:** M5's concrete renderer can bind semantic `cue` entries and `actions` to actual assets/hotspots. It does not need a `node_changed` field to display authored gameplay. **The surrounding concrete game composition** remains responsible for supplying those bindings and assets.
4. **Single outcome at choice selection:** GRIMM determines canonical effects and resultant presentation from `game_action` plus current state and authored rules; no renderer success callback or continuous action-progress simulation for authored M2 interactions.
5. **GRIMM-internal canonical World State:** state variables and rules stay inside GRIMM; M2 emits only the selected cues, offered actions and optional timer. Later renderers maintain/reconstruct their own presentation state, potentially contributing a renderer-owned save component.
6. **Renderer scene clock:** the renderer maintains the timer while action cues play. A new Cue Sheet's `scene_timer` reflects already-booked authored action cost and defines **earliest next-action eligibility**, not an instruction to begin counting after the cues finish. The renderer can still be presenting the cue when the target is reached; gameplay resolution remains independent of media completion.

## Confirmed documentation conflicts / implementation hazards

**C1 — Resolved in PR #13: M2 Save/Pause scope.** `SCENE-09/10` now require input readiness and regression of already implemented M1.75/M1.8 saves/pause, **not** a new M2 save checkpoint during unfinished cue playback. Additional graphical save coordination remains M5.

**C2 — Resolved in PR #13: new M3 scope.** The original M3 inventory/item/RPG proposals are historical/deferred. The new M3 concerns story authoring and validation tools only. M2 already provides typed state, puzzle progression and **qualified cue/action IDs**; M3 indexes them and checks optional renderer bindings.

**C3 — Resolved in PR #13: concrete renderer coverage.** M5 validates its **actual game-specific** cues, action bindings and assets, rather than a generic `supports 3D` capability flag. The same M4 script is the required basis for M5 and important gameplay results must match.

**C4 — Clarified working contract: choice-time resolution.** M2 evaluates a selected action once, including time cutoff and authored consequences, then emits the relevant ordered cue sequence. No renderer progress/outcome callback or pending gameplay action is necessary. Historical text should be interpreted under that agreed contract.

**C5 — Presentation gating versus total state commitment is not identical.** GRIMM may commit `drawer_open=true` before the renderer finishes its opening animation, but the state variable is **not** placed in the M2 Cue Sheet. A graphical renderer must be able to animate from a previously shown state to a final snapshot without revealing an object in its final pose too early. The M2 contract makes renderer responsible for staged presentation, but **does not yet say whether a single final snapshot and ordered cues always suffice** for M5. **Action:** M5 must test a stateful animated object and optional mid-sequence visibility/choice reveal; do not expose story node transitions as a workaround.

**C6 — Updated timer/pacing decision:** On action selection at T=3 with a five-second authored cost, GRIMM resolves its outcome and returns a Cue Sheet targeting T=8. The renderer clock **continues through cues**, and new offered choices can activate from T=8 even if animation is still running (subject to response/input readiness). Animation and text pacing are renderer choices, not mandatory physical matches for authored costs. Initial presentation gates can remain distinct; genuinely autonomous host hazards still require separate opt-in design.

**C7 — Canonical game rule state versus renderer/host state.** GRIMM's World State stays internal; in M5 a separate `saveAction`-like host input returns an opaque serialized canonical snapshot and `restoreAction(payload)` restores it. The game/renderer integrates this with presentation state in one coordinated save. API types, safe checkpoints and restore readiness remain to be designed with the graphical example; no state projections in M2 Cue Sheets.

## Clarification: authored cue IDs are sufficient at the GRIMM boundary

A Cue Sheet's `cue: [cue_id]` is **fully sufficient to command presentation**. The **concrete renderer** interprets each ID as whatever game-specific animation, text, camera staging, sound or sequence it implements. No additional GRIMM fields, central asset-mapping registry or game-composition mapping layer is needed **by the Cue Sheet protocol**. The term “game composition” only means the complete game ships a renderer and whatever assets/configuration that concrete renderer itself requires. Thus M5 needs to **implement and optionally check support for all authored cue IDs**, not redesign or enrich the Cue Sheet. Previous wording implying a new mapping contract at GRIMM's boundary was misleading.

This does **not** settle the independent issue in C5 of when a renderer displays the final committed world-state snapshot during its animation. That is a presentation sequencing concern, not a demand to replace `cue(cue_id)`.

## Open questions narrowed or answered in principle

- `OPEN-PUZZLE-02/03`: **answered in principle**: item discovery, combinations and changed choice availability use the same conditional choice + state + effect pipeline. Item identity/quantities/ownership remain open (`OPEN-PUZZLE-01`).
- `OPEN-STAT-01`: simple numeric attributes need no new base system; **only whether richer character-specific ownership/skills need more** stays open. `OPEN-STAT-02` probabilistic resolution remains open/deferred.
- `OPEN-ADV-02`: LucasArts puzzles and Telltale branches **share one DSL and Cue Sheet protocol**; designing complete location-spanning authored adventure remains M4.
- `OPEN-ADV-03`: a hotspot/text-menu activates the same offered semantic action ID; concrete M5 binding, capability/asset validation and input focus still open.
- `OPEN-SCENE-08/14`, `OPEN-GAME-04/09`: selected authored cue ID/ordered presentation is settled; how concrete media is mapped and validated remains M5; no universal media-duration matching is required.
- `OPEN-GAME-08`: likely no new renderer declaration is needed in existing CLI stories for M2; verify in implementation rather than impose a manifest.
- `OPEN-GEN-04/11`: later generated descriptions must not misrepresent GRIMM's committed state. Cue Sheet separation provides an authority boundary but **does not yet solve prose truthfulness**.

## New later-milestone questions (do not pull into M2)

- **L-DEFERRED-01:** If a later real game proves M2 local/global variables insufficient for item identity, location, quantity, possession or references, revisit first-class item semantics outside the currently planned M3 authoring-tool scope.
- **L-M4-01:** Can a single long-running interactive narrative situation reuse/resume its authored state and timer across explicit graph/context exits and re-entry, or must a new context begin? What does context-local reset mean?
- **L-M4-02:** For linked mini-games spanning multiple authored contexts, how do structural authored graph validation and conditional state reachability stay tractable without assuming free backtracking?
- **L-M5-01:** How does a graphical renderer use ordered semantic Cue Sheets and renderer-owned presentation state for correct starting pose, animation, final pose and hotspot visibility, especially after load or reconnect?
- **L-M5-02:** How does a concrete renderer or its game build optionally validate that every authored `cue(cue_id)` is supported, without changing GRIMM's Cue Sheet protocol or requiring an extra generic binding layer?
- **L-M5-03:** In an embedded host, how do GRIMM-internal canonical state, renderer-owned presentation state and host-owned world state stay consistent across game-level checkpoints?
- **L-M5-04:** Is M2's pause-during-`cue` timer behavior sufficient for graphical/host-owned ongoing hazards, or does a later mode need timers continuing through presentation? How does that interact with externally cancellable actions?
- **L-LATER-01:** If independent host events invalidate a pre-resolved choice during playback, what explicit opt-in integration contract preserves deterministic narrative authority without breaking ordinary Cue Sheet replay?

## Recommendation

**Keep historical M2 fixtures intact while clarifying later contracts.** C1–C3's historical wording conflicts have been reconciled; C4 remains a clarifying interpretation of immediate action resolution. Ensure C5's staged presentation is exercised with a concrete cue sequence. Retain C6–C7 and L-M5-* as future questions. Keep new M2 DSL and `The Locked Room` reference as the main acceptance source; no general RPG framework, renderer package format, host physics integration or dynamic node generation should enter M2 by implication.
