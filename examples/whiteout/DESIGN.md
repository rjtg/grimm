# WHITEOUT — rules, timing and acceptance

## Authored state and progression

The nine contexts are blackout, stores, dispatch, generator, radio, crisis and
three terminal endings. The dispatch hall connects the two repair routes; global
state gives repeat visits different dialogue and options. A sealed/abandoned
repair cabin cannot be reopened to reset its budget. All meaningful gameplay is
in `story.yaml`. Text, dialogue, instructions and labels are in `text.yaml`.

Important globals:

| State | Meaning |
| --- | --- |
| `thermal_liner`, `indoor_rate`, `exposure_rate` | Clothing and the current environment determine subsequent exposure. |
| `cold_exposure`, `accounted_ms` | Persistent exposure and the active scene's last booked logical time. |
| `jonas_treated` | Preparation improves latch effort and his chance of walking the ridge. |
| `copper_braid`, `ceramic_mug`, `spanner`, `lever`, `radio_adapter` | Ordinary possession/capability flags. Both combinations consume their component flags. |
| `safety_line`, `flare`, `heat_pack` | Prepared alternatives, fallback and one explicit timer extension. |
| `first_task` | 0 before dispatch, 1 generator first, 2 radio first. Used for delayed injury/rescue outcomes, not analytics. |
| `latch_damage`, `controls_known`, `deicer_isolated`, `generator_done` | Cumulative repair progress, knowledge and safe power restoration. |
| `engineering_lost`, `radio_lost` | Access/repair opportunity exhausted. A completed repair is not undone by losing access. |
| `dish_aligned`, `radio_done`, `radio_auto` | Manual setup, an acknowledged rescue call, and whether Lea can release the antenna. |
| `alex_alive`, `mara_alive`, `jonas_alive`, `lea_alive`, `ending` | Distinct final survivor facts and narrative outcome. |

The mystery is mechanical: a maintenance de-icer overload and failed test-call
relay, not secret sabotage. Jonas's tag, the repeating old call, the overload
flag and the log converge on the same explanation. The log is optional knowledge
with an ending payoff, and an alternate route to the generator wiring clue.

## Exact exposure accounting

No continuous simulation runs. Each action or authored timeout evaluates:

```text
cold_exposure += (time_passed - accounted_ms) * exposure_rate
accounted_ms = time_passed
```

`time_passed` is the engine's logical end including **effective** authored duration,
clamped to the deadline. Both expressions are ordinary authored effects, shared
through YAML anchors `charge_cold` and `book_time`. The order is significant.
Normal action effects run once before ordered success/failure outcomes.
On cutoff the engine skips normal effects and uses the context timeout; its
own accounting effects charge the effective interval exactly once. After GOTO,
that timeout sets the new anchor to `effective_ms`: the destination's interval
T=0..effective_ms is already booked work credit, matching the engine's rebased
input-eligibility floor. Idle timeout has effective_ms=0 and uses the deadline,
not late host-observation latency.

Exposure is integer milliexposure: rate 1 is 1,000 units per second, rate 2 is
2,000. This preserves subsecond intervals without per-action rounding loss.
The liner makes the sheltered stores/hall rate zero; engineering is rate 1 with
it and 2 without it. Restored generation warms the hall and radio cabin to rate
zero. The exposed crisis remains rate 1/2. At 40,000, latch progress weakens and
subsequent work is a five-second rather than three-second attempt. At 90,000,
fine radio manipulation fails through authored outcomes. Heating can recover
20,000 exposure through a four-second warming action. These are game balance
units, not physiological measurements or a general player-statistics subsystem.

### Concrete examples

- In the stores, think for 5s, then spend 1s equipping the liner. The old rate 1
  charges 6,000 once. Clothing changes the rate only **after** that interval;
  later sheltered work costs time but adds no exposure.
- Enter engineering with 6,000 exposure and an anchor at new T=0. Choose a 3s
  latch attempt at T=8. It books through T=11: 11,000 more, total 17,000. The
  renderer still resumes at T=8; at T=11 its advance does not charge again.
- Leave that annex at T=15. The zero-duration departure adds only the 4,000 from
  T=11..15. The outgoing context is settled, then `accounted_ms` is reset to 0
  and the destination's rate is selected. The new clock is a new epoch; old
  timestamps are never subtracted from it.
- **All normal GOTO actions have zero authored duration.** Travel is abstracted
  into the narrative. Work takes time inside its context; the separate normal
  departure starts the destination anchor at zero. A regression asserts this
  authoring invariant. Future nonzero travel costs would require an explicit
  accounting design, not silently changing these rules.
- **Interrupted work is different from normal travel.** With 10,000 exposure,
  select a 3s generator attempt at old T=29 with rate 2. Only 1s fits before T=30;
  booking through the deadline yields 70,000 total. The new hall clock starts
  at T=0, with anchor **1,000ms** (effective work credit), never old T=30,000.
  Choices become usable at T=1s. Choosing immediately then adds zero exposure;
  thinking until new T=5s adds only 4,000 at the hall's rate 1, for 74,000.
  An idle timeout instead carries zero credit, so the first new second counts.
  No timestamp from the old epoch is reused in either case.
- Under the old rate, starting generation books its four seconds before setting
  the current/indoor rate to zero. Warming applies only to future intervals.
- With 15,000 exposure booked through radio T=3 at rate 2, select transmission at
  T=40. Its 3s cost books the unaccounted 3..43 interval: 80,000 more. Total
  95,000 produces the authored `cold_transmit_failure`, not a protocol error.
  The same offered action at T=3 ends at T=6 and succeeds with 21,000.

Offer guards select real capabilities and the current work mode. Success guards
for deteriorating grip/cold are inside outcomes. Observations alone never apply
exposure effects, and an invalid/stale action never applies them. No public
protocol, engine timing model or world-state ownership changed for WHITEOUT.

## Timers and presentation

Stores have 180s, engineering 30s (initially hidden), radio 60s and crisis 45s.
Briefing completion executes each start instruction. A one-use heat-pack adds
60s to engineering's existing budget; it does not reset elapsed time. At T=10,
Lea's warning and a visibility cue reveal the running timer (20s remaining when
unextended). Replacing a plan never restarts that timer or replays the warning.

At T=8 a three-second latch presentation may still be running when the warning
starts at T=10. Its explicit action-release cue is floored at T=11 and may overlap
that media. By contrast, generator startup explicitly locks input, sequences
primer/ignition/heat cues and releases a full new offer only after all three
completions **and** the logical cost. There is deliberately no scheduled entry
choice refresh capable of overriding that sequence's lock.

The opening is a coarse renderer binding with three narrative beats/paragraphs
under one semantic ID and one completion. Startup is fine-grained: three IDs and
three acknowledgements. M5 may stage the coarse ID as a complete cutscene while
retaining the same rules and IDs. Player prose never contains canonical globals.

An explicit `at_ms` is never authored in a GOTO result; timed cues live in context
entry plans. A late generator start at T=27 costs only the effective three seconds
before the T=30 cutoff and does not manufacture power. A start at T=26 completes
exactly at T=30 and is allowed: power remains on when the shutter then seals.
Ordinary play has no requirement to find either exact boundary.

## Acceptance evidence

The deterministic `crates/cli/tests/whiteout.rs` suite executes the actual shipped
story through engine → adapter → cue player; no tests sleep.

| Required demonstration | Evidence |
| --- | --- |
| Hidden/revealed timer, overlap, timer continuity and remaining-plan replacement | `real_generator_warning_overlaps_latch_media_and_never_restarts_the_timer` |
| Repeated progress, different help/cold effort and duration | `treating_jonas_changes_cumulative_progress_and_attempt_count`, `slower_deliberation_changes_latch_progress_and_next_authored_duration` |
| Genuine combinations and alternatives with later resource cost | `full_crew_rescue_uses_force_crafting_and_preparation`, `lever_is_a_real_alternate_solution_with_a_later_radio_cost` |
| Three different final states | Full rescue, named partial-survival tests and `storm_timeout_has_a_catastrophic_canonical_ending` |
| Choice-caused death; better preparation can save all | `choosing_jonas_over_the_manual_radio_operator_has_a_named_cost`, `safety_line_preparation_can_also_save_everyone_after_radio_first` |
| Persistent exposure, deliberation, no double charge, rate/epoch changes | `elapsed_cold_is_booked_once_including_costs_rate_changes_and_departure`, `restored_heating_changes_future_rate_and_can_recover_a_fine_repair` |
| Offered attempt becomes authored failure | `an_offered_transmission_can_fail_from_deliberation_without_protocol_rejection` |
| Sequential locked input and coarse/fine cue granularity | `startup_fine_cues_lock_input_until_all_three_beats_and_the_cost_finish`, binding coverage test |
| Delayed order and time consequences | `radio_first_delay_changes_the_later_same_rescue_decision`, `cold_from_slow_engineering_changes_a_later_radio_outcome_and_can_be_recovered` |
| Cutoff, exact completion and cross-epoch credit | `deadline_cutoff_accounts_only_effective_time_and_does_not_invent_power`, `interrupted_work_credit_is_not_charged_again_in_the_new_scene` |
| Deterministic save/reload and actual CLI continuation | `save_at_dispatch_then_restore_has_identical_world_and_ending_continuation`, `actual_cli_checkpoint_load_resumes_the_remaining_adventure` |
| Actual CLI to all endings and alternate full rescue | `actual_cli_reaches_all_three_endings_and_the_lever_rescue` |
| Real M3 compilation/index and complete bindings | `m3_index_and_every_text_binding_are_complete_and_renderer_independent` |
| World-sensitive presentation and reduced pointless options | `preparation_feedback_and_pending_radio_cues_follow_the_changed_world` |

## Playtesting and remaining limits

A complete real-clock terminal playthrough collected supplies, treated Jonas,
forced the latch with his help, bought the extra minute, isolated the de-icer,
acknowledged the startup, warmed hands, saved via Esc in dispatch, built/transmitted
the radio jumper and coordinated a full-crew rescue. The ten-second radio cue
actually overlapped latch prose; the timer revealed roughly eighty seconds
remaining after the explicit extension. All choices used immediate number keys.

This playtest found and fixed authored friction: hide a now-useless lever recipe
after opening the latch; suppress a manual-mast warning after automatic radio
success; shorten repeat hall briefings; warn that blocked intake makes staying
inside unsafe. A deterministic test also found a scheduled offer refresh that
could supersede the startup lock; removing that redundant cue fixed the story.

No engine/schema/adapter/renderer changes or dependencies were necessary.
The model uses scene-local budgets, not a continuously ticking global storm.
The hall is a checkpoint; a warm hall permits recovery/planning, but cannot undo
order-dependent injury. Travel has zero logical cost. Text completion is coarse,
and existing host saves may replay unfinished prose without replaying effects.
M3 structural checks are not a state-space proof; the scripted routes verify
intended paths, not every possible sequence. Human difficulty balancing remains
open to further playtest feedback. No M5 assets or M6 analytics/statistics were
implemented, and no mandatory M4 demonstration is intentionally deferred.

Final local checks passed: `cargo fmt --all --check`, `cargo build --workspace
--locked`, `cargo test --workspace --locked` (**158 tests**, including all 137
M0–M3 tests/doctests and 21 WHITEOUT tests), and `cargo clippy --workspace
--all-targets --locked -- -D warnings`. The real M3 CLI exported nine contexts,
35 actions and 92 authored cues; every narrative/action binding is covered. Four
real CLI walkthroughs and checkpoint continuation reached their expected endings.
LSP feedback was requested; its later response was stale (retaining an obsolete
unused-method warning), so the clean Cargo checks provide final semantic evidence.
