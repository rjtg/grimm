# M4.5 — Scene Clock, Cue Timing & Developer Terminal UI

**Status:** Implemented; see [implementation evidence](m4-5-implementation.md). Based on WHITEOUT playtesting. **Target:** M4.5, before M5 graphical rendering. **Scope:** deliberately limited to the single-scene-clock M2/M4 architecture. This document remains the approved design contract; the implementation and migration notes describe its concrete representation.

Existing reference material: [scene format](../scene-format.md), [WHITEOUT game](../../examples/whiteout/README.md), [WHITEOUT timing notes](../../examples/whiteout/DESIGN.md), [M6 history proposal](m6-execution-history-statistics.md).

## Goals and ownership

1. A modern, comfortable, Codex-like terminal interface with **narrative and choices on the left**, **optional debug inspector on the right**, and a continuously visible timer/status area.
2. A **running scene clock independent of an optional deadline**. A completed relevant choice may remove a deadline; unrelated choices must not.
3. Scheduled cues with **explicit `at_ms`** and optional narrative `duration_ms`, allowing automatic text progression and overlapping cues without mandatory Enter acknowledgements.
4. A **read-only host inspection API** for context, epoch, clock and globals/locals; optional **controlled debug-only state mutation** only if replay/save semantics remain sound.
5. Preserve M2 full-REPLACE Cue Sheets, stable cue IDs, renderer independence and WHITEOUT's authored world-state/timing rules.

Non-goals: general multiple independent countdowns, a second gameplay clock, a graphical renderer, M6 statistics, an RPG/inventory framework, global background simulation or a new story DSL.

## UI mockup A — ordinary play (debug pane hidden)

The following fixed-width mockups are *layout targets*, not screenshots of running software. Text/clock values are illustrative. The production UI should adapt to terminal size.

```text
╭──────────────────────────────────────────────────────────────────────────────╮
│  GRIMM / WHITEOUT                                    ENGINEERING · T=12.4s │
├──────────────────────────────────────────────────────────────────────────────┤
│  GENERATOR ANNEX                                    ⏱ DEADLINE   00:18    │
├──────────────────────────────────────────────────────────────────────────────┤
│  NARRATIVE                                                                   │
│                                                                              │
│  The generator annex shudders beneath the ice. You hear the latch crack.     │
│                                                                              │
│  Lea: "Alex, the coil is losing power. You need to hurry."                   │
│                                                                              │
│  ↑↓ Scroll narrative · previous lines stay visible after cue completion     │
├──────────────────────────────────────────────────────────────────────────────┤
│  CHOICES                                                                     │
│                                                                              │
│  [1] Force the frozen latch again                                            │
│  [2] Use the insulated lever                                                 │
│  [3] Retreat to the dispatch hall                                            │
│                                                                              │
├──────────────────────────────────────────────────────────────────────────────┤
│  1–9 Choose   ↑↓ Scroll   Esc Pause/Save/Load   F2 Debug   ? Help           │
╰──────────────────────────────────────────────────────────────────────────────╯
```

**Layout behavior:** Narrative scrollback and choices are stable, independently redrawn regions; deadline/status never disappears during prose. No scrolling full-screen terminal log as a substitute. Default narrative playback advances automatically when an authored duration elapses. Explicit manual mode may be offered for accessibility/reading preference. Displayed text does not disappear when its cue completes.

## UI mockup B — debug inspector docked on the right

```text
╭────────────────────────────────────────────────────────────────────────────────────────────────╮
│ GRIMM / WHITEOUT                        GENERATOR ANNEX · T=12.4s · deadline 00:18 · [DEBUG]  │
├─────────────────────────────────────────────────┬──────────────────────────────────────────────┤
│ NARRATIVE                                       │ CUE INSPECTOR                                │
│                                                 │ context: generator  epoch: 3  rev: 42       │
│ ● arrival [C1]                                 │                                              │
│ The generator annex shudders beneath the ice.   │ ✓ generator.arrival          completed       │
│ You hear the latch crack.                       │ ✓ generator.deadline         completed       │
│                                                 │ ▶ generator.radio_warning    running         │
│ ● radio_warning [C2]                           │ ○ generator.choices          pending         │
│ Lea: "Alex, the coil is losing power!"          │                                              │
│                                                 │ selected cue: generator.radio_warning        │
│                                                 │ narrative_id: generator.radio_warning        │
│                                                 │ at_ms: 10000  duration_ms: 2200             │
│                                                 │ after_cue: —                                 │
│                                                 ├──────────────────────────────────────────────┤
│ CHOICES                                         │ WORLD STATE (READ-ONLY)                      │
│                                                 │ cold_exposure:     28000                     │
│ [1] Force the latch again                       │ latch_damage:      4                        │
│ [2] Use the insulated lever                     │ generator_done:    false                    │
│ [3] Retreat to dispatch hall                    │ globals / locals · changed values marked    │
│                                                 │                                              │
├─────────────────────────────────────────────────┴──────────────────────────────────────────────┤
│ 1–9 Choose   Tab Focus   F2 Hide inspector   F3 State editor (debug only)   Esc Menu         │
╰────────────────────────────────────────────────────────────────────────────────────────────────╯
```

**Color is supplemental, not identity:** the same color/marker is used for a narrative's `cue_id` in the narrative scrollback and in the inspector. Always show readable IDs and statuses for monochrome terminals, accessible themes, and repeated colors. A `narrative_id` is reusable content, while `cue_id` identifies a particular authored instruction; do not conflate them.

**Inspector requirements:** display full current ordered REPLACE sheet including `at_ms`, optional duration, `after_cue`, instruction, available actions and timer directives. Distinguish `pending`, `running`, `completed`, `replaced/cancelled`. Completed and replaced cues remain inspectable in a **bounded presentation-only revision history**, not appended as fake canonical gameplay events. Show current context, revision, epoch, clock, deadline and visibility. The inspector must not inspect internal graph/world data through Cue Sheets.

**Responsive behavior:** on a sufficiently wide terminal use a docked right pane; on narrow terminals hide it or offer a switchable full-width inspector view. Never compress choices or narrative into unreadable fragments. Handle resize, Unicode, alternate screen, raw mode and terminal restoration on errors. Keep test-friendly piped and `--manual-clock` modes operating without an interactive TUI.

## Protocol proposal — one clock, optional deadline

### Clock invariants

- On **entering a new scene/context**, the engine starts a **new epoch at T=0**, with a running scene clock regardless of whether a deadline exists.
- **`restart_timer`** denotes the new scene epoch's clock reset. It is **implicit/engine-owned on `goto`**, not something each story author must duplicate. Do not expose an unrestricted mid-scene restart in M4.5: that would invalidate existing `at_ms` schedules, eligibility floors and time accounting. If named as a protocol instruction, constrain it to a new epoch/boundary and document who applies it exactly once.
- **`set_deadline`** establishes a deadline relative to the current scene time at instruction execution or at an explicit absolute scene-clock timestamp; its exact field representation must distinguish these cases unambiguously.
- **`unset_deadline`** removes an active deadline, its timeout trigger and its visual countdown while **leaving the scene clock running**. The past deadline must not fire later.
- **`show_deadline`** toggles visibility only; hiding/showing **never resets or extends** the deadline.
- **Deadline extension** remains supported (either a documented form of `set_deadline` or an explicit `extend_deadline` operation). Keep the existing heat-pack extension gameplay behavior.
- Only authored rules/outcomes determine when to set/unset a deadline. Renderer selections must not automatically cancel deadlines. The engine retains authority over whether a selected action is completed, interrupted or timed out before its resulting timer instructions execute.
- An unrelated action or full REPLACE does **not** clear an existing deadline. Save/restore and idempotent feedback preserve running time state, visibility and cancellation.
- Clock time continues through narrative presentation, decision time and authored work. Environmental exposure is still booked only at authored evaluation points; it is **not** a continuous world-state tick.

### Example: choice countdown versus persistent deadline

```yaml
# Schematic future authored result cues; field names finalized by implementation.
# Decision: the player chose before expiry; the outcome cancels the deadline.
cues:
  - {cue_id: decision_made, at_ms: 0, kind: timer, unset_deadline: true}
  - {cue_id: decision_result, at_ms: 0, duration_ms: 1800, kind: narrative,
     narrative_id: crisis.decision_made}

# An unrelated generator action must omit unset_deadline.
# The existing active power cutoff continues, irrespective of REPLACE.
```

**Note:** if `at_ms` is absolute, action-result cue scheduling must use the current scene epoch's time (e.g. selection time), not unconditionally T=0. The snippet illustrates the instruction shape only. Timed result cues on `goto` remain forbidden: timed instructions must belong to the destination entry plan. An `unset_deadline` instruction that must take effect during a transition needs an explicit safe, atomic engine-side transition policy rather than a forbidden timed transition-result cue.

### Example: overlapping narrative and action availability

```yaml
cues:
  - {cue_id: intro, kind: narrative, narrative_id: generator.arrival,
     at_ms: 0, duration_ms: 4000}
  - {cue_id: radio_warning, kind: narrative, narrative_id: generator.radio_warning,
     at_ms: 2500, duration_ms: 2200}
  - {cue_id: choices, kind: actions, available_actions: eligible,
     at_ms: 4000}
```

- Make `at_ms` **mandatory in the emitted Cue Sheet** for every instruction. Authored YAML can use a defined migration/lowering rule, e.g. compile sequential cues into an explicit schedule, but must not silently reinterpret old order or break M0–M4 stories.
- Narrative `duration_ms` is **optional**. A fixed-duration cue may send `cue_completed` automatically at the scheduled logical time once it has actually started; without a duration, renderer completion remains explicit.
- Keep `after_cue` dependencies where completion is a real barrier; `at_ms` alone does not override them. Overlapping cues remain legal and independent. A scheduled start is subject to dependencies and the engine's action eligibility floor.
- Renderer scrollback is not a source of canonical state. A cue replaced before beginning must not start later. A retained same-ID running cue must not restart or extend its duration on REPLACE.
- Explicit and automatic completion use the same idempotent semantic feedback; replay and restore do not double-complete or double-apply effects.
- Define what happens if auto-completion becomes due while an action is selected or a deadline expires in the same observation: deterministic ordering and boundary tests required.
- `duration_ms` specifies *presentation scheduling*, not an additional action duration or gameplay effect. Do not double-book cold exposure.

## World State Inspection API

Provide a **host-only read API**, separate from the Cue Sheet and renderer presentation contract:

```rust
// Conceptual only; exact Rust types are design choices.
pub struct WorldStateSnapshot {
    pub context_id: String,
    pub scene_epoch: u64,
    pub revision: u64,
    pub globals: BTreeMap<String, Value>,
    pub locals: BTreeMap<String, Value>,
}
impl SceneEngine {
    pub fn inspect_world(&self) -> WorldStateSnapshot;
}
```

Requirements: immutable snapshot, typed values, distinguish global/local scope, stable context/epoch/revision identity, no gameplay effects. The TUI can highlight differences between successive snapshots without adding those diff events to canonical game history. Prefer reusing existing engine accessors if they already satisfy the API; don't duplicate stored world state.

### Optional controlled debug mutation

Debugging may benefit from changing individual globals/locals (e.g. forcing `cold_exposure=85000` to exercise an authored failure). This must **not** use normal player-action channels, and must not be silently written outside the canonical semantic replay log.

If implemented in M4.5:
- Accept explicit debug transactions, validate existence, scope and type; reject invalid changes without partial mutation.
- Re-evaluate eligibility and Cue Sheet consistently without silently resetting an active timer or replaying old one-shot cues.
- Preserve deterministic restore/replay by recording debug transactions in an appropriately versioned checkpoint event stream, *or* explicitly prevent normal save/export for debug-mutated sessions.
- Mark and expose `debug_modified` state so that saves or future statistics cannot be mistaken for ordinary player progress.
- Make editing **opt-in, debug-mode-only** with clear before/after values. No arbitrary DSL execution, graph edits or timer manipulation through the state editor.

**Delivery guard:** if deterministic mutation requires a major checkpoint/protocol change, deliver the read-only API and a written safe mutation design rather than a hidden unsafe shortcut. F3 in the mockup is conditional on mutation support; otherwise disable/hide it.

## Acceptance and migration

Implementation must reconcile the current M2 contract (sequential untimed cues, clock initially started by timer cues, `start_ms/extend_ms/visible`) with the new scene-clock contract. This is a **planned protocol revision**, not merely a UI facelift. Specify versioning/migration for authored stories, emitted sheets and save/checkpoint compatibility. Do not silently break Locked Room or WHITEOUT.

Required regression and interactive evidence:

1. Scene with **no deadline**: scene clock progresses and a narrative cue finishes automatically without Enter.
2. A choice **unsets** its countdown; elapsed scene time continues; old timeout never occurs.
3. An unrelated choice **does not unset** the countdown; REPLACE, visibility toggles and duration completion do not restart it.
4. Action with cost crossing a deadline resolves/truncates under existing policy before stopping the deadline; exact-boundary cases are deterministic.
5. Hidden deadline reveal and heat-pack extension retain remaining time, not a fresh countdown.
6. Overlapping narratives and a timed radio warning run simultaneously; `after_cue` gates dependent actions.
7. Same-ID running cue, canceled cue, repeated feedback, restore and new epoch behave correctly.
8. WHITEOUT cold exposure charges presentation/decision/work time exactly once, including across `goto` epoch resets.
9. Read-only `inspect_world` returns correctly scoped state and remains pure. Debug mutation, if implemented, validates typing and is replay-safe.
10. TUI displays narrative, choices, timer, header and right-side inspector simultaneously; test scrollback, resize, color/ID correspondence, keyboard controls and terminal teardown.
11. Piped scripted walkthroughs, manual-clock tests, M3 validation/index and existing WHITEOUT endings continue to work. Run format/build/test/Clippy and an actual terminal playthrough.

Deliver a single focused implementation PR from current `main`, with the new TUI, protocol migration notes, sample terminal captures, updated DSL documentation, automated tests and explicit residual limitations. **Do not merge automatically.**

M4.5 is complete when a player can finish WHITEOUT without mandatory Enter-by-Enter text playback, timers end only when authored rules say so, and a developer can inspect Cue Sheets and the current world state from the docked right pane without the renderer learning story rules.
