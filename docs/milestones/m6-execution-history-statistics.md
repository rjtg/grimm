# M6 proposal — Execution History API & optional player-choice statistics

**Status:** Proposed design; not an approved implementation milestone or a change to M3/M4 scope.
**Reference game:** WHITEOUT (see [M4 design](m4-whiteout-design.md)).
**Guiding principle:** **GRIMM records what happened. The game decides what matters. The optional statistics component compares outcomes.**

## Goal

Allow games to present optional, Telltale-style chapter-end comparisons such as the percentage of participating players who repaired the generator first, chose to save Lea instead of Jonas, or rescued the whole crew. These comparisons must never affect game rules. A GRIMM game remains fully playable offline and without statistics.

## Component boundary

| Component | Owns | Does not own |
|---|---|---|
| **GRIMM engine** | Canonical state and validated action decisions; structured read-only history of committed actions, narrative context/scene visit, observed selection time, effective action cost, deadline interruption, scene transition and timer states at meaningful boundaries | Statistical questions, player identities, sampling, uploads, percentages, analytics UI |
| **Game application / host** | Decides which history/world-state facts are meaningful, which chapter/reporting boundary has been reached, when to submit/query/show results, opt-in behavior | Engine game rules or renderer-specific playback state |
| **Optional statistics library/backend** | Accepts game-defined measurements; local/remote storage, deduplication, aggregation and distributions | Story graph interpretation, canonical world state mutation, renderer or GRIMM runtime dependencies |
| **Renderer** | Presentation of results (text table, graphical bars, selected player response) | Statistical computation or game rules |

No statistics annotations are required in the authored story DSL. Do not add analytics instructions to Cue Sheets. Both the M4 text and M5 graphical game use the same authored gameplay story; a host may integrate statistics or omit them.

## M6a — Public read-only execution history

M2 already has semantic action history and checkpoint replay. **Extend/project that existing canonical record** instead of maintaining a second independent truth. Keep the opaque save format private to GRIMM; callers must not need to parse it.

Proposed *conceptual* data model (names and Rust layout TBD):

```rust
pub struct ChoiceRecord {
    pub action_id: String,          // Stable qualified semantic action ID
    pub scene_epoch: u64,           // Distinguishes scene visits
    pub context_id: String,         // Host-only history; never in Cue Sheets
    pub selected_at_ms: u64,        // Scene clock at selection
    pub authored_duration_ms: u64,
    pub effective_duration_ms: u64,
    pub interrupted: bool,         // Deadline truncated the action
}
pub struct SceneRecord {
    pub scene_epoch: u64,
    pub context_id: String,
    pub choices: Vec<ChoiceRecord>,
    pub elapsed_at_exit_ms: u64,
    pub deadline_at_exit_ms: Option<u64>,
    pub remaining_at_exit_ms: Option<u64>,
    pub completed: bool,
}
```

Invariants:
- Expose only **committed** actions; an offered, rejected, stale or rolled-back input is not a choice.
- Multiple executions of the same action remain separate history entries; the game decides how to count them.
- Each scene visit has an unambiguous identity, including if a graph context is revisited.
- A departing scene's timer snapshot uses that visit's clock and deadline, **before** a GOTO resets the next visit's clock to T=0.
- Maintain an unambiguous ordering of action records and scene boundaries; no invented wall-clock timestamp is needed.
- Deterministic save/restore must expose the **current valid timeline**, not decisions from discarded branches.
- Read queries have no gameplay effects, renderer dependencies or timing changes.
- World-state inspection remains a separate read-only concern; do not require a snapshot of every variable after each action.
- Avoid mandatory capture of cue_started/cue_completed in the public choice statistics API; these remain presentation coordination.

### Total time

M2 has scene-relative clocks. Total elapsed gameplay time can initially be **derived from per-visit times and logical durations** using an explicitly documented accounting convention, without introducing a second continuously advancing engine clock. In particular, don't double-charge authored action duration versus subsequently observed elapsed scene time. A future cumulative clock is an optional design decision, not a prerequisite.

### Checkpoints and statistics reporting

A checkpoint restores GRIMM's valid semantic history. A host should generate statistics at an authored/game-defined reporting boundary (e.g. chapter end), rather than automatically submit every action. If the player reloads and takes a different branch, history queries reflect that new valid branch.

## M6b — Optional statistics aggregation

Game-specific measurement extraction takes the public history plus selected canonical world state and creates values external to GRIMM's DSL, for example:

```text
history:
  T=08 search_equipment
  T=19 repair_generator
  T=37 contact_lea
  T=52 save_jonas

world state:
  generator_repaired=true
  lea_alive=false
  jonas_alive=true

measurements:
  first_priority=generator
  rescue_choice=jonas
  crew_outcome=partial_survival
```

The optional component supports small, provider-neutral interfaces for submitting and querying game-defined measurements. Start with a local in-memory/file-based reference backend and seeded test data. A production hosted database/service is **not required** for initial acceptance; a network provider may be added later without binding GRIMM to it.

For online use, include:
- Clear player opt-in and privacy behavior; no analytics or network dependency for gameplay.
- Minimal derived measurements, not raw execution history or personal data by default.
- Idempotent submission identifiers for a game-defined reporting unit (e.g. chapter completion), so retrying does not produce duplicate votes.
- Explicit policy for replay/restart/multiple playthrough participation; not determined by GRIMM.
- Story/version compatibility; avoid mixing choice populations whose semantics changed.
- Sample sizes and sensible handling of insufficient data; don't present local seeded results as global community statistics.

A statistic may be a **selected option, derived order, number, duration or final world-state outcome**. The application may present a player's own choice next to the distribution. GRIMM does not need a general chapter system or a statistics UI.

## WHITEOUT demonstration / acceptance

At a game-selected chapter end, generate and display at least:
1. Generator or radio **first priority** (choice ordering).
2. Jonas, Lea or a prepared alternate **rescue choice**.
3. Full rescue, partial survival or catastrophe **survival outcome** (canonical world state).

Demonstrate multiple simulated playthroughs and an optional local statistics backend. Run the same WHITEOUT game without the statistics feature; the game logic, Cue Sheets and outcomes must be unchanged.

M6a tests should establish:
- Repeated actions are distinct committed records, while invalid inputs are absent.
- Qualified actions and context/scene-visit IDs are preserved across scene transitions.
- Selection time, effective cost, interruption and timer-at-exit are correct.
- Save/restore and divergent post-load decisions yield the correct current-timeline history.
- History reads are pure and don't expose internal checkpoint serialization.
- Timer accounting stays deterministic across T=0 resets and action-duration boundaries.

M6b tests should establish:
- Game-defined extraction works from history and world state only.
- Local submission and querying produces correct counts/distributions.
- Repeated submission with the same reporting identity does not double-count.
- Version-incompatible statistics are not accidentally merged.
- No backend/collector is required to play the game.

## Delivery sequence and non-goals

- **M6a:** General read-only Execution History API in GRIMM, based on M2's committed semantic history.
- **M6b:** Separate optional Statistics component and host-side WHITEOUT example, local-first.

Do **not** add analytics DSL directives, Cue Sheet fields, mandatory accounts, network dependencies in the engine, automated moral-choice interpretation, full historical world-state snapshots, or an engine-owned statistics interface. Do not implement WHITEOUT, M5 graphics or M6 during the ongoing M3 authoring milestone solely because this design exists.

**Open for implementation review:** exact Rust method/record types, representation of scene exits and active visits, snapshot strategy versus replay-derived projection, and how chapter reporting identities survive restarts across multiple saves. These details must not compromise GRIMM's existing authority and renderer boundaries.
