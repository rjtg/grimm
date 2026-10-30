# WHITEOUT — M4 Adventure Design Brief (proposed)

**Status:** M4 implemented in [WHITEOUT](../../examples/whiteout/README.md), following merged M0–M3. The original brief below remains the acceptance reference; concrete rules, playtesting evidence and timing choices are in the [implementation notes](../../examples/whiteout/DESIGN.md). M5 graphical adaptation remains future work.
**Working title:** WHITEOUT
**Setting:** Isolated polar research station Aurora during an escalating storm and power failure.
**Delivery:** A finished M4 text adventure authored in the M2 DSL, later adapted in M5 to a graphical renderer using **the same GRIMM story script and semantic IDs**. Both example applications live in this repository.

## Premise and dramatic spine

The station loses its primary power supply. Emergency power and heating are failing. Repairing generation, restoring radio communications, and coordinating a rescue require splitting the remaining crew. Each success costs time or limited supplies. Secondary failures and irreversible decisions create situations in which at least one character can die; good preparation must open meaningful alternatives rather than forcing the same loss in every playthrough.

Proposed principal cast (names and roles are provisional):
- **Alex:** player character, moves between authored narrative contexts.
- **Mara:** station lead; knows safety protocols and must prioritize survival.
- **Jonas:** injured technician; can help with generator work at personal risk.
- **Lea:** radio specialist; can restore a rescue call if power and antenna equipment permit.

Tone: isolation, competence under pressure, emotionally consequential tradeoffs. Puzzle solutions must remain logically discoverable rather than relying on pixel hunting or unexplained item combinations.

## Mandatory mechanics and their concrete demonstrations

| Acceptance target | WHITEOUT scenario / evidence |
|---|---|
| **At least one hidden timer** | Emergency-power cutoff begins hidden; a radio warning or alarm later reveals the **already running** countdown. The deadline does not reset when UI visibility changes. |
| **Repeated action with accumulating progress** | Work the generator's ice-locked latch repeatedly. Each attempt increases authored progress (e.g. `latch_damage`), takes logical time, and may be less effective at high `cold_exposure`. |
| **MacGyver item combination** | Combine recoverable station objects into a functional improvised repair/heating or antenna adapter. Item possession/consumption and the resulting capability are represented using ordinary authored conditions/effects, not a new hardcoded inventory subsystem. |
| **Three or more distinct endings** | Rescue/evacuation, partial survival with lasting losses, and catastrophic station loss; distinct authored outcomes and survivor combinations, not merely alternate prose. |
| **Death through a player choice** | A consequential choice between helping Jonas in engineering or Lea at communications can lead to a character's death. Prior preparation and alternate problem solutions can change the options; **do not force an unavoidable death on every route**. |
| **A stat spanning multiple contexts** | `cold_exposure` persists across authored scenes and affects strength, action cost, outcome and available authored alternatives. |
| **Continuous-feeling survival pressure** | Player deliberation consumes observed scene time. `cold_exposure` rises deterministically for elapsed time spent in cold conditions, plus authored action costs and events. Prompt decisions therefore genuinely reduce exposure. |
| **Concurrent timed cue presentation** | A station radio call or warning starts at an absolute scene-clock `at` during a generator/door-opening presentation; timer visibility or other presentation instructions can occur concurrently. |
| **Sequential presentation with input lock** | During a significant generator startup sequence, `available_actions: []` closes the offer; after presentation, a new complete `available_actions` list is presented. Demonstrate both one long `narrative_id` rendered as a complex sequence and multiple small ordered narrative cues. |
| **Order-dependent, delayed consequences** | Repairing generator before radio (or vice versa) has later effects on survivors, warmth, signal availability and ending branches. Don't confine all consequences to the immediate cue response. |
| **Alternative solutions** | The generator latch can be forced through repeated attempts **or** opened with an improvised tool; the latter saves effort/time but may use material needed elsewhere. |
| **Save/restore** | Host/game allows saves at suitable stable checkpoints, preferably scene starts; load restores GRIMM canonical state, semantic action/cue history and ongoing timer context, while renderer restores its own presentation state. Do not require mid-animation saves. |
| **Deadline-truncated action support** | Authored rules cover an action selected shortly before a deadline, whose logical cost extends beyond it. Demonstrate with deterministic test/fixture; a normal playthrough need not accidentally encounter this timing. |

**Explicitly not required for WHITEOUT:** revisiting the same graph node in a cycle, generic cyclic-navigation demonstration, separate local-vs-global variable demonstration, an unrestricted inventory/RPG framework, background NPC simulation, physics-driven hazards, or a new continuous world-state ticker. Multiple distinct authored story nodes may represent changed circumstances in the same physical room.

## Suggested authored narrative structure (not prescribed graph grammar)

1. **Blackout / emergency briefing:** introduce people, failures and first irreversible resource constraints.
2. **Station inventory and triage:** acquire usable objects, discover insulated clothing and damaged systems; establish the cold-exposure rate.
3. **Engineering / frozen latch:** repeated attempts versus improvised tool. Jonas's assistance is contingent on earlier care and decisions.
4. **Communications / failing signal:** MacGyver adapter, hidden communications/power countdown, a radio warning that arrives while another task runs.
5. **Split-crew crisis:** radio and generator become simultaneously urgent. Choices and prior preparation determine whom Alex can reach and protect; a character can die.
6. **Extraction or collapse:** resolve previous decisions and surviving characters into at least three distinct playable endings.

These are distinct authored narrative contexts. Story-graph cycles are allowed by GRIMM but need not be showcased in this game.

## Time and cold-exposure contract

**Core rule: elapsed time is game-significant, without continuous GRIMM simulation.**

- The renderer owns the progressing scene clock and submits observed elapsed time at player selection, per the M2 contract.
- The authored game tracks a **last-accounted time anchor** for the active timed scene (e.g. in world-state variables), using M2 clock expressions and action effects. At the next action or authored timeout, it computes the *unaccounted elapsed interval once* and applies the authored environmental rate to `cold_exposure`. The time anchor advances atomically with the calculation. **Scene time resets on `goto`:** settle any prior scene's exposure before transition, then initialize the next scene's anchor against its new T=0 origin; never subtract timestamps from different scene epochs.
- Cold-exposure rate may depend on authored conditions (room temperature, heating restored, protective clothing). When a rate changes, account for the previous interval before adopting the new rate. An authored event/timeout provides an evaluation point when necessary; there is **no requirement for per-second GRIMM state updates**.
- Duration spent deliberating counts; action logical cost also counts when it occurs within an exposure interval. **Do not double-charge** time that is already represented by the renderer's clock or the committed authored action-cost interval. In particular, GRIMM must define and test whether it books action cost at selection and how the next observed timestamp excludes already-accounted duration.
- Thresholds are evaluated when an action/timeout is processed, not continuously streamed to the UI. **Do not remove or reject a previously offered action merely because exposure increased while the player was thinking.** Treat it as a valid authored attempt whose outcome may now fail.

Conceptual authored gameplay branch (not fixed DSL grammar):

```text
ON repair_generator {
    APPLY_UNACCOUNTED_COLD_EXPOSURE_AT_SELECTION
    IF cold_exposure < 50 {
        EFFECT generator_repaired = true
        CUE generator_repair_success
    } ELSE IF cold_exposure < 80 {
        CUE generator_repair_failed_shaking_hands
    } ELSE {
        CUE generator_repair_failed_hypothermia
    }
}
```

The selection is valid when it was offered. GRIMM produces a **gameplay failure result** if conditions deteriorated, rather than an invalid-input protocol error. Unknown, malformed, or never-authorized action IDs are still rejected as protocol/game-input errors.

No UI refresh is required at intermediate cold thresholds while the player is idle. Significant events may use authored timers/timeouts and subsequent full REPLACE Cue Sheets. Example balance numbers and item recipes are game-design placeholders, not commitments about medically accurate body temperature or physical engineering.

## Cue Sheet / renderer requirements in WHITEOUT

- Use the already agreed **single ordered `cues: []` / full REPLACE** model. Do not introduce ADD/patch semantics.
- Every instruction has stable scene-qualified `cue_id`; a narrative cue's reusable `narrative_id` is distinct. Auto-generated IDs follow the M2/M3 rules; authors can fix explicit IDs for continuity.
- `available_actions` replaces the complete offer; `[]` temporarily locks input. Timed `at` instructions use absolute scene-clock time and can overlap a longer narrative presentation.
- GRIMM owns authored rules, player decisions, canonical game state and semantic cue history; renderers handle the speaker, artwork, dialogue text, audio, animation, choreography, UI and cue granularity.
- Demonstrate one **coarse-grained** renderer mapping (a narrative ID triggers a multi-step sequence) and one **fine-grained** set of sequential cues; neither demands extra GRIMM rendering primitives.
- Scene transitions are represented through authored graph transitions; save eligibility is a host/renderer decision.
- The M4 text game and M5 graphic adaptation consume the *same authored gameplay script*.

## Proposed acceptance scenarios

1. **Hidden countdown:** Start an unseen 30s emergency timer; show it only at T=10 with 20s remaining, and verify no reset on a subsequent action/Cue Sheet.
2. **Overlapping events:** Choose `open_generator_latch` at T=8 with 3s authored cost. A radio cue scheduled at T=10 starts during its presentation; offered actions are eligible no earlier than T=11.
3. **Player-speed effect:** Two otherwise identical playthroughs choose the same repair action at different times in a cold room; the slower attempt accumulates more exposure and can produce an authored failure instead of a protocol rejection. Check accounted intervals are neither skipped nor double-charged.
4. **Repeated attempts:** Different attempt counts and exposure/strength states alter latch progress, duration and result.
5. **Alternate puzzle routes:** Both the repeated force approach and combined-item solution can progress, with different time/resource consequences.
6. **Consequential split:** Branches reflect a prior radio-versus-generator order and lead to different characters surviving; include a player choice causing a death and at least one preparedness-dependent better alternative.
7. **Save/reload:** Restore at a chosen safe checkpoint; subsequent authored decision sequence yields the same world state, timer, cold exposure, cue plan and ending.
8. **Deadline boundary:** Verify action duration truncated by an approaching deadline through a deterministic/fake-clock test rather than depending on human decision timing.
9. **Presentation barriers:** Input disabled during a narrative sequence, restored with an entirely new authorized choice list after presentation completion.

## Delivery and scope

This design is for later **M4 authoring**, building on the now-merged M2 foundation. It does not retroactively change M2 acceptance requirements or require an immediate new engine capability. M3's authoring validator can help report generated cue/action IDs and structural exits, but remains independent of renderer asset manifests. M5 uses the same story script with a separate concrete renderer.

When M4 implementation begins, finalize actual character arcs, location graph, item recipes, difficulty balancing and the precise authored DSL syntax against real M2 capabilities. Include automated outcome/time tests and a complete playable text walkthrough. Keep WHITEOUT implementation separate from the completed M2 milestone.
