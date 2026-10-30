# Cross-milestone consistency audit — M2 Cue Sheets and DSL versus M3–M5

## Current Cue Sheet protocol decision — full replacement and stable cue IDs (2026-10-30)

This section supersedes earlier **working-design** descriptions in this document that show top-level `cue`, `actions`, `actions_available_at`, `new_timer`, `show_timer`, or an `ADD` mode. Older M2 acceptance traces remain historical compatibility evidence, not the proposed newer serialization. Concrete wire/DSL grammar remains illustrative.

- **Every response is REPLACE.** GRIMM derives one *complete authoritative remaining presentation plan* for the current narrative context from canonical authored world state, scene time, authored conditions, player-action history, and semantic cue execution history. The renderer replaces the pending schedule on each response; no `ADD`, merge, inherited action-list patches, or cross-revision cue ordering.
- **One ordered `cues: []` list.** Entries can contain `narrative_id` (semantic content binding), `available_actions` (complete replacement of the offered action list, including `[]` to lock input), timer start/visibility instructions, and optional `at` absolute time on the current scene clock. The illustrative notation is a logical model, not an imposed Rust parser grammar. Within one sheet, untimed entries run in order, including presentation completion barriers; an `at` event is clock-scheduled and may overlap a running narrative cue. An incoming REPLACE sheet's untimed prefix begins at the current paused/resumed clock time, **not** retroactively at T=0.
- **Timing:** The renderer pauses its scene clock at player selection T while GRIMM resolves `game_action`; resumes from T on the reply. GRIMM resolves authored action duration and deadline at selection once. New `available_actions` instructions are never enabled before that action's computed earliest logical time, even if their preceding narrative finishes sooner. Conversely a sequenced choice update may wait for renderer presentation completion if it is placed *after* that narrative. Timed instructions use absolute scene-clock `at`, so a T=10 radio event can begin during a T=8–11 drawer opening. A new timer starts at its explicit instruction, after any preceding intro presentation; subsequent clock-scheduled items do not block the start.
- **One cue identity, separate content identity.** Each authored cue instruction has a unique, stable `cue_id` **within its story/scene namespace**, independent of `narrative_id` (the reusable renderer binding). Two distinct cues can reference the same `narrative_id`. Authors may explicitly name `cue_id`; otherwise GRIMM tooling deterministically generates an ASCII-qualified ID from the authored instruction/content and timing, e.g. `locked_room.narrative_radio_johnny_returns_at_10`. Derive from authored definition, **not output list position**; M3 must reject collisions and offer diagnostics. If an author edits content/timing, the generated identity may change; explicitly authored IDs are advised when continuity and save compatibility matter. No second `cue_instance_id` is required in this model.
- **Renderer feedback:** The renderer reports `cue_started` and `cue_completed` with `cue_id` and sheet correlation/revision, optionally current scene time. GRIMM records enough action/cue history to avoid rerunning started/completed cues while constructing the next REPLACE sheet. The same stable `cue_id` in a replacement sheet denotes **continuation** of presentation already in flight, not replay; renderer owns media-specific fades, duration and detailed resume state. Stale or duplicate events must be idempotently handled, including events from a now-replaced sheet.
- **State boundaries:** GRIMM records semantic action decisions, cue execution statuses, and canonical game rules/state for deterministic recomputation and snapshot/restore. Cue lifecycle acknowledgements are presentation coordination **not gameplay effects**; they never independently mutate authored world variables or authorize actions. Concrete assets, text, animation/audio, UI geometry and presentation playback positions remain renderer-owned and saved by the host independently.
- **Availability invariants:** `available_actions` always replaces the full visible/actionable offer; `[]` disables them during a narrative cue. GRIMM alone validates `game_action` input and retains rule authority even if a UI is stale. If a new sheet is issued while old action/media instructions run, only actions from the current sheet may become eligible; stale scheduled `available_actions` instructions are discarded.
- **Timer UI:** A timer can start hidden, with a later cue entry showing it; visibility does not start/reset the timer or affect game conditions.

Illustrative T=8 drawer action with three seconds of authored cost, where the T=10 radio cue overlaps the drawer presentation:

```text
game_action { action: open_drawer, scene_timer: [8 of 30]s }
cue_sheet {
  cues: [
    { cue_id: drawer_input_lock, available_actions: [] },
    { cue_id: drawer_opening, narrative_id: drawer_opened },
    { cue_id: radio_warning, at: 10, narrative_id: radio_johnny_returns },
    { cue_id: countdown_reveal, at: 10, timer: { visible: true } },
    { cue_id: drawer_new_choices, at: 11, available_actions: [take_key, search_desk, hide] },
    { cue_id: johnny_footsteps, at: 20, narrative_id: footsteps_approaching }
  ]
}
```

The clock resumes from T=8 after GRIMM responds; radio/countdown trigger at T=10 regardless of a concurrent drawer animation. The T=11 choice offer is a *minimum* logical-time release and must not be prematurely unblocked by short media. When choosing an action after T=10, GRIMM uses cue-history acknowledgements to omit already finished events, or preserve an in-flight cue under its same `cue_id`. When entering another authored scene, the next full REPLACE sheet starts the new scene's plan; no explicit graph-node ID leaks to renderers.


Status: **review findings, not new adopted APIs**. Read with [M2 locked-room reference](m2-cue-sheet-reference.md), [requirements](../requirements.md), [architecture](../architecture.md), [roadmap](../roadmap.md) and [open questions](../open-questions.md). This audit distinguishes settled principles, unresolved extensions and **actual wording conflicts**. Do not expand M2 scope merely to solve later host/graphics issues.

## Current architectural decision (supersedes earlier snapshot proposal)

**World State is GRIMM-internal.** Story-authored variables and their current values belong to GRIMM and are used to evaluate game rules. **M2 Cue Sheets do not include a `world_state` field**: only ordered `cue(cue_id)`, accepted actions, optional timer and response ID. A concrete renderer may maintain its own presentation state and later contribute that state to a coordinated game save, but renderer state is not authoritative for story rules. Earlier claims below that a full renderer-visible state snapshot is required in every Cue Sheet are **superseded**. The remaining M5 question is how to coordinate/restage renderer presentation without introducing a universal snapshot contract into M2.

## Settled by M2 / implications for later milestones

1. **Shared authored action language:** repeated interactions, stat tests, progress, branches, alternative solutions and time costs are rules over state; no fixed `Inspect/Take/Use` vocabulary or separate mini-game execution engine. Historical M2 traces may use local IDs. The **replacement M3** establishes and indexes the later-game qualified cue/action naming convention and provides basic structural exit/dead-end linting. Asset comparisons remain renderer/IDE work; optional item or character frameworks are not roadmap commitments.
2. **Narrative nodes are primarily authoring structure:** one node/context can represent a complete mini-game. M4 must not require a graph node per movement, puzzle attempt or animation. Context transitions/returns are explicitly authored for GRIMM-led adventures.
3. **Renderer-facing Cue Sheets are graph-private:** M5's concrete renderer can bind semantic `cue` entries and `actions` to actual assets/hotspots. It does not need a `node_changed` field to display authored gameplay. **The surrounding concrete game composition** remains responsible for supplying those bindings and assets.
4. **Single outcome at choice selection:** GRIMM determines canonical effects and resultant presentation from `game_action` plus current state and authored rules; no renderer success callback or continuous action-progress simulation for authored M2 interactions.
5. **GRIMM-internal canonical World State:** state variables and rules stay inside GRIMM; M2 emits only the selected cues, offered actions and optional timer. Later renderers maintain/reconstruct their own presentation state, potentially contributing a renderer-owned save component.
6. **Renderer scene clock:** the renderer maintains the timer while action cues play. The new Cue Sheet's shared `actions_available_at` reflects already-booked authored action cost and defines **earliest next-action eligibility** through the sheet's shared `actions_available_at`, not an instruction to begin counting after the cues finish. The renderer can still be presenting the cue when the target is reached; gameplay resolution remains independent of media completion.

## Confirmed documentation conflicts / implementation hazards

**C1 — Resolved in PR #13: M2 Save/Pause scope.** `SCENE-09/10` now require input readiness and regression of already implemented M1.75/M1.8 saves/pause, **not** a new M2 save checkpoint during unfinished cue playback. Additional graphical save coordination remains M5.

**C2 — Resolved in PR #13: new M3 scope.** The original M3 inventory/item/RPG proposals are historical/deferred. The new M3 concerns story authoring and validation tools only. M2 already provides typed state and puzzle progression; historical traces may use local cue/action IDs. M3 standardizes/indexes later-game qualified IDs and structural exit checks; renderer-owned tooling may compare assets.

**C3 — Resolved in PR #13: concrete renderer coverage.** M5 validates its **actual game-specific** cues, action bindings and assets, rather than a generic `supports 3D` capability flag. The same M4 script is the required basis for M5 and important gameplay results must match.

**C4 — Clarified working contract: choice-time resolution.** M2 evaluates a selected action once, including time cutoff and authored consequences, then emits the relevant ordered cue sequence. No renderer progress/outcome callback or pending gameplay action is necessary. Historical text should be interpreted under that agreed contract.

**C5 — Presentation gating versus total state commitment is not identical.** GRIMM may commit `drawer_open=true` before the renderer finishes its opening animation, but the state variable is **not** placed in the M2 Cue Sheet. A graphical renderer must be able to animate from its previously shown presentation state to the desired final pose without revealing an object in its final pose too early. The M2 contract makes renderer responsible for staged presentation, but **does not yet say whether a single final snapshot and ordered cues always suffice** for M5. **Action:** M5 must test a stateful animated object and optional mid-sequence visibility/choice reveal; do not expose story node transitions as a workaround.

**C6 — Current timer/pacing decision:** An action selected at T=3 with cost 5s yields earliest logical eligibility T=8. The REPLACE plan schedules `available_actions` in ordered or timed cue instructions while the renderer resumes from T=3, allowing atmospheric overlap. No shared top-level `actions_available_at` in the proposed protocol.

**C7 — Canonical game rule state versus renderer/host state.** GRIMM's World State stays internal; in M5 a separate `saveAction`-like host input returns an opaque serialized canonical snapshot and `restoreAction(payload)` restores it. The game/renderer integrates this with presentation state in one coordinated save. API types, safe checkpoints and restore readiness remain to be designed with the graphical example; no state projections in M2 Cue Sheets.

## Clarification: authored cue IDs are sufficient at the GRIMM boundary

A Cue Sheet's ordered `cues` instructions have scene-scoped `cue_id` and reusable `narrative_id`, with full `available_actions` updates. The renderer maps semantic narrative IDs to game-specific visuals/text/audio; no world-state projection or engine asset registry is needed.

This does **not** settle the independent issue in C5 of how a renderer stages its own final presentation state during an animation. That is a presentation sequencing concern, not a demand to replace `cue(cue_id)`.

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
- **L-M5-04:** The renderer clock continues during action-result presentations except during GRIMM evaluation. A timer-start cue starts a fresh clock after preceding intro. Coordination with external autonomous host hazards remains a later integration question.
- **L-LATER-01:** If independent host events invalidate a pre-resolved choice during playback, what explicit opt-in integration contract preserves deterministic narrative authority without breaking ordinary Cue Sheet replay?

## Recommendation

**Keep historical M2 fixtures intact while clarifying later contracts.** C1–C3's historical wording conflicts have been reconciled; C4 remains a clarifying interpretation of immediate action resolution. Ensure C5's staged presentation is exercised with a concrete cue sequence. Retain C6–C7 and L-M5-* as future questions. Keep new M2 DSL and `The Locked Room` reference as the main acceptance source; no general RPG framework, renderer package format, host physics integration or dynamic node generation should enter M2 by implication.
