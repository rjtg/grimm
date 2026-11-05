# Canonical interaction ledger

Static editorial transcription of `story.yaml` at main `a4adba6`. This is a
reference, not a new execution language. All conditions/effects below are
canonical [C]; chapters provide adapted staging [A]. Outcome order is binding.
Globals omit the `global.` prefix for readability. Time is milliseconds.

Every action first books cold using the ordered effects shown. `time_passed`
includes only effective work, with deadline truncation. No work is charged twice.
Canonical selection outcomes commit once; media duration does not change them.

## Initial globals

| Field | Initial value |
| --- | --- |
| `thermal_liner` | `false` |
| `jonas_treated` | `false` |
| `copper_braid` | `false` |
| `ceramic_mug` | `false` |
| `spanner` | `false` |
| `safety_line` | `false` |
| `heat_pack` | `false` |
| `flare` | `false` |
| `log_read` | `false` |
| `lever` | `false` |
| `controls_known` | `false` |
| `deicer_isolated` | `false` |
| `generator_done` | `false` |
| `engineering_lost` | `false` |
| `radio_adapter` | `false` |
| `dish_aligned` | `false` |
| `radio_done` | `false` |
| `radio_auto` | `false` |
| `radio_lost` | `false` |
| `cold_exposure` | `0` |
| `accounted_ms` | `0` |
| `exposure_rate` | `0` |
| `indoor_rate` | `1` |
| `first_task` | `0` |
| `latch_damage` | `0` |
| `alex_alive` | `true` |
| `mara_alive` | `true` |
| `jonas_alive` | `true` |
| `lea_alive` | `true` |
| `ending` | `"unresolved"` |

## `blackout`

[Screenplay](01-blackout-stores.md). Terminal: **false**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `arrival` | sequential; `true` | TEXT `blackout.arrival`; 12000ms |
| `choices` | sequential; `true` | actions eligible |

### `blackout.begin`

Offer: `true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `accounted_ms := 0; exposure_rate := 1` | GOTO `stores` |

## `stores`

[Screenplay](01-blackout-stores.md). Terminal: **false**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `deadline` | T=0; `true` | set_deadline_ms=180000; visible=true |
| `arrival` | sequential; `true` | TEXT `stores.arrival`; 7000ms |
| `choices` | sequential; `true` | actions eligible |

### `stores.take_thermal_liner`

Offer: `thermal_liner = false`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; thermal_liner := true; exposure_rate := 0; indoor_rate := 0`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `thermal_liner`: TEXT `stores.thermal_liner`; 1800ms |

### `stores.jonas_treated`

Offer: `jonas_treated = false`. Work: **3000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; jonas_treated := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `jonas_treated`: TEXT `stores.jonas_treated`; 1800ms |

### `stores.take_copper_braid`

Offer: `copper_braid = false`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; copper_braid := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `copper_braid`: TEXT `stores.copper_braid`; 1800ms |

### `stores.take_ceramic_mug`

Offer: `ceramic_mug = false`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; ceramic_mug := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `ceramic_mug`: TEXT `stores.ceramic_mug`; 1800ms |

### `stores.take_spanner`

Offer: `spanner = false`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; spanner := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `spanner`: TEXT `stores.spanner`; 1800ms |

### `stores.take_safety_line`

Offer: `safety_line = false`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; safety_line := true; flare := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `safety_line`: TEXT `stores.safety_line`; 1800ms |

### `stores.take_heat_pack`

Offer: `heat_pack = false`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; heat_pack := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `heat_pack`: TEXT `stores.heat_pack`; 1800ms |

### `stores.log_read`

Offer: `log_read = false`. Work: **2000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; log_read := true; controls_known := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `log_read`: TEXT `stores.log_read`; 1800ms |

### `stores.leave`

Offer: `true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `thermal_liner = true` | `accounted_ms := 0; exposure_rate := 0` | GOTO `dispatch` |
| 2 | `true` | `accounted_ms := 0; exposure_rate := 1` | GOTO `dispatch` |

### `stores` timeout

Effects: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; accounted_ms := effective_ms; exposure_rate := indoor_rate`.

GOTO `dispatch`. Feedback: `timeout`: TEXT `stores.timeout`; 1800ms.

## `dispatch`

[Screenplay](02-dispatch.md). Terminal: **false**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `heat_returns` | sequential; `generator_done = true` | TEXT `dispatch.heat_returns`; 1800ms |
| `radio_first_cost` | sequential; `(first_task = 2 ∧ radio_done = true)` | TEXT `dispatch.radio_first_cost`; 1800ms |
| `arrival` | sequential; `first_task = 0` | TEXT `dispatch.arrival`; 1800ms |
| `return_brief` | sequential; `first_task ≠ 0` | TEXT `dispatch.return_brief`; 1800ms |
| `choices` | sequential; `true` | actions eligible |

### `dispatch.engineering`

Offer: `(generator_done = false ∧ engineering_lost = false)`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `(first_task = 0 ∧ thermal_liner = true)` | `first_task := 1; accounted_ms := 0; exposure_rate := 1` | GOTO `generator` |
| 2 | `first_task = 0` | `first_task := 1; accounted_ms := 0; exposure_rate := 2` | GOTO `generator` |
| 3 | `thermal_liner = true` | `accounted_ms := 0; exposure_rate := 1` | GOTO `generator` |
| 4 | `true` | `accounted_ms := 0; exposure_rate := 2` | GOTO `generator` |

### `dispatch.communications`

Offer: `(radio_done = false ∧ radio_lost = false)`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `(first_task = 0 ∧ thermal_liner = true)` | `first_task := 2; accounted_ms := 0; exposure_rate := 1` | GOTO `radio` |
| 2 | `first_task = 0` | `first_task := 2; accounted_ms := 0; exposure_rate := 2` | GOTO `radio` |
| 3 | `generator_done = true` | `accounted_ms := 0; exposure_rate := 0` | GOTO `radio` |
| 4 | `thermal_liner = true` | `accounted_ms := 0; exposure_rate := 1` | GOTO `radio` |
| 5 | `true` | `accounted_ms := 0; exposure_rate := 2` | GOTO `radio` |

### `dispatch.warm_hands`

Offer: `(generator_done = true ∧ cold_exposure ≥ 1)`. Work: **4000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `cold_exposure ≥ 20000` | `cold_exposure += -20000` | remain; `warm_hands`: TEXT `dispatch.warm_hands`; 1800ms |
| 2 | `true` | `cold_exposure := 0` | remain; `warm_again`: TEXT `dispatch.warm_again`; 1800ms |

### `dispatch.ridge`

Offer: `((generator_done = true ∨ engineering_lost = true) ∧ (radio_done = true ∨ radio_lost = true))`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `thermal_liner = true` | `accounted_ms := 0; exposure_rate := 1` | GOTO `crisis` |
| 2 | `true` | `accounted_ms := 0; exposure_rate := 2` | GOTO `crisis` |

## `generator`

[Screenplay](03-generator.md). Terminal: **false**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `deadline` | T=0; `true` | set_deadline_ms=30000; visible=false |
| `arrival` | sequential; `true` | TEXT `generator.arrival`; 4000ms |
| `choices` | sequential; `true` | actions eligible |
| `radio_warning` | T=10000; `generator_done = false` | TEXT `generator.radio_warning`; 2200ms |
| `reveal_clock` | T=10000; `generator_done = false` | visible=true |

### `generator.deploy_heat_pack`

Offer: `heat_pack = true`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; heat_pack := false`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `extra_minute`: extend_ms=60000; `heat_pack`: TEXT `generator.heat_pack`; 1800ms |

### `generator.open_generator_latch`

Offer: `(latch_damage < 6 ∧ cold_exposure < 40000)`. Work: **3000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `cold_exposure ≥ 90000` | `none` | remain; `open_generator_latch_numb`: TEXT `generator.open_generator_latch_numb`; 1800ms |
| 2 | `(jonas_treated = true ∧ cold_exposure < 40000)` | `latch_damage += 3` | remain; `open_generator_latch_assisted_open`: TEXT `whiteout.latch_open`; 3500ms IF `latch_damage ≥ 6`; `open_generator_latch_assisted_nearly`: TEXT `whiteout.latch_nearly`; 3500ms IF `(latch_damage ≥ 4 ∧ latch_damage < 6)`; `open_generator_latch_assisted_crack`: TEXT `whiteout.latch_crack`; 3500ms IF `latch_damage < 4`; `open_generator_latch_assisted_release`: actions eligible; explicit at_ms=0 |
| 3 | `cold_exposure < 40000` | `latch_damage += 2` | remain; `open_generator_latch_steady_open`: TEXT `whiteout.latch_open`; 3500ms IF `latch_damage ≥ 6`; `open_generator_latch_steady_nearly`: TEXT `whiteout.latch_nearly`; 3500ms IF `(latch_damage ≥ 4 ∧ latch_damage < 6)`; `open_generator_latch_steady_crack`: TEXT `whiteout.latch_crack`; 3500ms IF `latch_damage < 4`; `open_generator_latch_steady_release`: actions eligible; explicit at_ms=0 |
| 4 | `true` | `latch_damage += 1` | remain; `open_generator_latch_slow_open`: TEXT `whiteout.latch_open`; 3500ms IF `latch_damage ≥ 6`; `open_generator_latch_slow_nearly`: TEXT `whiteout.latch_nearly`; 3500ms IF `(latch_damage ≥ 4 ∧ latch_damage < 6)`; `open_generator_latch_slow_crack`: TEXT `whiteout.latch_crack`; 3500ms IF `latch_damage < 4`; `open_generator_latch_slow_release`: actions eligible; explicit at_ms=0 |

### `generator.work_latch_with_numb_hands`

Offer: `(latch_damage < 6 ∧ cold_exposure ≥ 40000)`. Work: **5000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `cold_exposure ≥ 90000` | `none` | remain; `work_latch_with_numb_hands_numb`: TEXT `generator.work_latch_with_numb_hands_numb`; 1800ms |
| 2 | `(jonas_treated = true ∧ cold_exposure < 40000)` | `latch_damage += 3` | remain; `work_latch_with_numb_hands_assisted_open`: TEXT `whiteout.latch_open`; 3500ms IF `latch_damage ≥ 6`; `work_latch_with_numb_hands_assisted_nearly`: TEXT `whiteout.latch_nearly`; 3500ms IF `(latch_damage ≥ 4 ∧ latch_damage < 6)`; `work_latch_with_numb_hands_assisted_crack`: TEXT `whiteout.latch_crack`; 3500ms IF `latch_damage < 4`; `work_latch_with_numb_hands_assisted_release`: actions eligible; explicit at_ms=0 |
| 3 | `cold_exposure < 40000` | `latch_damage += 2` | remain; `work_latch_with_numb_hands_steady_open`: TEXT `whiteout.latch_open`; 3500ms IF `latch_damage ≥ 6`; `work_latch_with_numb_hands_steady_nearly`: TEXT `whiteout.latch_nearly`; 3500ms IF `(latch_damage ≥ 4 ∧ latch_damage < 6)`; `work_latch_with_numb_hands_steady_crack`: TEXT `whiteout.latch_crack`; 3500ms IF `latch_damage < 4`; `work_latch_with_numb_hands_steady_release`: actions eligible; explicit at_ms=0 |
| 4 | `true` | `latch_damage += 1` | remain; `work_latch_with_numb_hands_slow_open`: TEXT `whiteout.latch_open`; 3500ms IF `latch_damage ≥ 6`; `work_latch_with_numb_hands_slow_nearly`: TEXT `whiteout.latch_nearly`; 3500ms IF `(latch_damage ≥ 4 ∧ latch_damage < 6)`; `work_latch_with_numb_hands_slow_crack`: TEXT `whiteout.latch_crack`; 3500ms IF `latch_damage < 4`; `work_latch_with_numb_hands_slow_release`: actions eligible; explicit at_ms=0 |

### `generator.make_lever`

Offer: `(spanner = true ∧ copper_braid = true ∧ lever = false ∧ latch_damage < 6)`. Work: **2000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; spanner := false; copper_braid := false; lever := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `lever_made`: TEXT `generator.lever_made`; 1800ms |

### `generator.use_lever`

Offer: `(lever = true ∧ latch_damage < 6)`. Work: **3000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `cold_exposure ≥ 90000` | `none` | remain; `lever_failed`: TEXT `generator.lever_failed`; 1800ms |
| 2 | `true` | `latch_damage := 6` | remain; `lever_open`: TEXT `generator.lever_open`; 1800ms |

### `generator.inspect_controls`

Offer: `controls_known = false`. Work: **1000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; controls_known := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `controls`: TEXT `generator.controls`; 1800ms |

### `generator.isolate_deicer`

Offer: `(controls_known = true ∧ deicer_isolated = false)`. Work: **2000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; deicer_isolated := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `isolated`: TEXT `generator.isolated`; 1800ms |

### `generator.start_generator`

Offer: `(latch_damage ≥ 6 ∧ generator_done = false)`. Work: **4000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `deicer_isolated = false` | `none` | remain; `overload`: TEXT `generator.overload`; 1800ms |
| 2 | `cold_exposure ≥ 110000` | `none` | remain; `starter_cold`: TEXT `generator.starter_cold`; 1800ms |
| 3 | `true` | `generator_done := true; exposure_rate := 0; indoor_rate := 0` | remain; `generation_deadline_cancelled`: unset_deadline=true; `startup_lock`: actions []; `startup_prime`: TEXT `generator.startup_prime`; 1200ms; `startup_ignition`: TEXT `generator.startup_ignition`; 1500ms; `startup_heat`: TEXT `generator.startup_heat`; 1800ms; `startup_ready`: actions eligible |

### `generator.return`

Offer: `generator_done = true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `accounted_ms := 0; exposure_rate := 0` | GOTO `dispatch` |

### `generator.abandon`

Offer: `generator_done = false`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `thermal_liner = true` | `engineering_lost := true; accounted_ms := 0; exposure_rate := 0` | GOTO `dispatch` |
| 2 | `true` | `engineering_lost := true; accounted_ms := 0; exposure_rate := 1` | GOTO `dispatch` |

### `generator` timeout

Effects: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; engineering_lost := true; accounted_ms := effective_ms; exposure_rate := indoor_rate`.

GOTO `dispatch`. Feedback: `timeout`: TEXT `generator.timeout`; 1800ms.

## `radio`

[Screenplay](04-radio.md). Terminal: **false**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `deadline` | T=0; `true` | set_deadline_ms=60000; visible=true |
| `arrival` | sequential; `true` | TEXT `radio.arrival`; 6000ms |
| `choices` | sequential; `true` | actions eligible |
| `gust_warning` | T=15000; `radio_auto = false` | TEXT `radio.gust_warning`; 1800ms |

### `radio.make_adapter`

Offer: `(copper_braid = true ∧ ceramic_mug = true ∧ radio_adapter = false)`. Work: **3000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; copper_braid := false; ceramic_mug := false; radio_adapter := true`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `none` | remain; `adapter`: TEXT `radio.adapter`; 1800ms |

### `radio.hand_align`

Offer: `(dish_aligned = false ∧ radio_done = false ∧ radio_adapter = false)`. Work: **6000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `cold_exposure ≥ 90000` | `none` | remain; `alignment_failed`: TEXT `radio.alignment_failed`; 1800ms |
| 2 | `true` | `dish_aligned := true` | remain; `aligned`: TEXT `radio.aligned`; 1800ms |

### `radio.send_signal`

Offer: `(radio_done = false ∧ (radio_adapter = true ∨ dish_aligned = true))`. Work: **3000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `cold_exposure ≥ 90000` | `none` | remain; `cold_transmit_failure`: TEXT `radio.cold_transmit_failure`; 1800ms |
| 2 | `radio_adapter = true` | `radio_done := true; radio_auto := true` | remain; `automatic_call_deadline_cancelled`: unset_deadline=true; `automatic_call`: TEXT `radio.automatic_call`; 1800ms |
| 3 | `true` | `radio_done := true` | remain; `manual_call_deadline_cancelled`: unset_deadline=true; `manual_call`: TEXT `radio.manual_call`; 1800ms |

### `radio.warm_hands`

Offer: `(generator_done = true ∧ cold_exposure ≥ 1)`. Work: **4000ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `cold_exposure ≥ 20000` | `cold_exposure += -20000` | remain; `warmed_hands`: TEXT `radio.warmed_hands`; 1800ms |
| 2 | `true` | `cold_exposure := 0` | remain; `warm_again`: TEXT `radio.warm_again`; 1800ms |

### `radio.return`

Offer: `radio_done = true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `generator_done = true` | `accounted_ms := 0; exposure_rate := 0` | GOTO `dispatch` |
| 2 | `thermal_liner = true` | `accounted_ms := 0; exposure_rate := 0` | GOTO `dispatch` |
| 3 | `true` | `accounted_ms := 0; exposure_rate := 1` | GOTO `dispatch` |

### `radio.abandon`

Offer: `radio_done = false`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `generator_done = true` | `radio_lost := true; accounted_ms := 0; exposure_rate := 0` | GOTO `dispatch` |
| 2 | `thermal_liner = true` | `radio_lost := true; accounted_ms := 0; exposure_rate := 0` | GOTO `dispatch` |
| 3 | `true` | `radio_lost := true; accounted_ms := 0; exposure_rate := 1` | GOTO `dispatch` |

### `radio` timeout

Effects: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; radio_lost := true; accounted_ms := effective_ms; exposure_rate := indoor_rate`.

GOTO `dispatch`. Feedback: `timeout`: TEXT `radio.timeout`; 1800ms.

## `crisis`

[Screenplay](05-ridge-endings.md). Terminal: **false**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `deadline` | T=0; `true` | set_deadline_ms=45000; visible=true |
| `arrival` | sequential; `true` | TEXT `crisis.arrival`; 8000ms |
| `choices` | sequential; `true` | actions eligible |

### `crisis.coordinate`

Offer: `true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `(generator_done = true ∧ radio_done = true ∧ jonas_treated = true ∧ (radio_auto = true ∨ safety_line = true) ∧ (first_task = 1 ∨ safety_line = true) ∧ cold_exposure < 120000)` | `ending := "full_rescue"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_full` |
| 2 | `true` | `none` | remain; `missing_preparation`: TEXT `crisis.missing_preparation`; 1800ms |

### `crisis.help_lea`

Offer: `true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `(radio_done = true ∧ generator_done = true ∧ jonas_treated = true ∧ first_task = 1 ∧ cold_exposure < 120000)` | `ending := "full_rescue"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_full` |
| 2 | `(radio_done = true ∨ flare = true)` | `jonas_alive := false; ending := "partial_survival"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_partial` |
| 3 | `true` | `alex_alive := false; mara_alive := false; jonas_alive := false; lea_alive := false; ending := "station_lost"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_lost` |

### `crisis.help_jonas`

Offer: `true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `(radio_done = true ∧ radio_auto = true ∧ generator_done = true ∧ cold_exposure < 120000)` | `ending := "full_rescue"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_full` |
| 2 | `(radio_done = true ∨ flare = true)` | `lea_alive := false; ending := "partial_survival"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_partial` |
| 3 | `true` | `alex_alive := false; mara_alive := false; jonas_alive := false; lea_alive := false; ending := "station_lost"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_lost` |

### `crisis.flare_retreat`

Offer: `flare = true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `jonas_alive := false; flare := false; ending := "partial_survival"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_partial` |

### `crisis.stay`

Offer: `true`. Work: **0ms**.

Before outcomes: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed`.

| Ordered outcome | Predicate | Additional effects | Destination / feedback |
| --- | --- | --- | --- |
| 1 | `true` | `alex_alive := false; mara_alive := false; jonas_alive := false; lea_alive := false; ending := "station_lost"; accounted_ms := 0; exposure_rate := 0` | GOTO `ending_lost` |

### `crisis` timeout

Effects: `cold_exposure += ((time_passed − accounted_ms) × exposure_rate); accounted_ms := time_passed; alex_alive := false; mara_alive := false; jonas_alive := false; lea_alive := false; ending := "station_lost"; accounted_ms := effective_ms; exposure_rate := 0`.

GOTO `ending_lost`. Feedback: `timeout`: TEXT `crisis.timeout`; 1800ms.

## `ending_full`

[Screenplay](05-ridge-endings.md). Terminal: **true**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `arrival` | sequential; `true` | TEXT `ending_full.arrival`; 1800ms |
| `truth` | sequential; `log_read = true` | TEXT `ending_full.truth`; 1800ms |

## `ending_partial`

[Screenplay](05-ridge-endings.md). Terminal: **true**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `arrival` | sequential; `true` | TEXT `ending_partial.arrival`; 1800ms |
| `jonas` | sequential; `jonas_alive = false` | TEXT `ending_partial.jonas`; 1800ms |
| `lea` | sequential; `lea_alive = false` | TEXT `ending_partial.lea`; 1800ms |
| `truth` | sequential; `log_read = true` | TEXT `ending_partial.truth`; 1800ms |

## `ending_lost`

[Screenplay](05-ridge-endings.md). Terminal: **true**.

Entry cues in authored order:

| Cue | Scheduling / guard | Instruction |
| --- | --- | --- |
| `arrival` | sequential; `true` | TEXT `ending_lost.arrival`; 1800ms |
