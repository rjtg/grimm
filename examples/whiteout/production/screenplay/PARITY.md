# Parity matrix — canonical story to screenplay

Baseline: main `a4adba6` (merged PR #76). Every row references existing content;
editorial beats are not graph nodes. All nine contexts, 35 actions, 57 narrative
bindings and 95 authored cue instances are accounted for below and in RULES.

## Context / branch coverage

| Context | Screenplay | Interactive branches |
| --- | --- | --- |
| `blackout` | [blackout](01-blackout-stores.md) | `begin` |
| `stores` | [stores](01-blackout-stores.md) | `take_thermal_liner`, `jonas_treated`, `take_copper_braid`, `take_ceramic_mug`, `take_spanner`, `take_safety_line`, `take_heat_pack`, `log_read`, `leave`; timeout |
| `dispatch` | [dispatch](02-dispatch.md) | `engineering`, `communications`, `warm_hands`, `ridge` |
| `generator` | [generator](03-generator.md) | `deploy_heat_pack`, `open_generator_latch`, `work_latch_with_numb_hands`, `make_lever`, `use_lever`, `inspect_controls`, `isolate_deicer`, `start_generator`, `return`, `abandon`; timeout |
| `radio` | [radio](04-radio.md) | `make_adapter`, `hand_align`, `send_signal`, `warm_hands`, `return`, `abandon`; timeout |
| `crisis` | [crisis](05-ridge-endings.md) | `coordinate`, `help_lea`, `help_jonas`, `flare_retreat`, `stay`; timeout |
| `ending_full` | [ending_full](05-ridge-endings.md) | terminal |
| `ending_partial` | [ending_partial](05-ridge-endings.md) | terminal |
| `ending_lost` | [ending_lost](05-ridge-endings.md) | terminal |

Exact action guards, costs, ordered outcome effects, targets, entry scheduling
and timeout effects are transcribed in [RULES.md](RULES.md). Cinematic meaning
and visual alternatives appear in the chapter choice tables/branch beats.

## Every narrative binding

| Canonical TEXT | Screenplay beat | Preservation |
| --- | --- | --- |
| `blackout.arrival` | [B01](01-blackout-stores.md#b01) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.arrival` | [S01](01-blackout-stores.md#s01) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.thermal_liner` | [S02](01-blackout-stores.md#s02) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.jonas_treated` | [S02](01-blackout-stores.md#s02) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.copper_braid` | [S02](01-blackout-stores.md#s02) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.ceramic_mug` | [S02](01-blackout-stores.md#s02) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.spanner` | [S02](01-blackout-stores.md#s02) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.safety_line` | [S02](01-blackout-stores.md#s02) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.heat_pack` | [S02](01-blackout-stores.md#s02) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.log_read` | [S03](01-blackout-stores.md#s03) | Fact/outcome retained; [A] performance adapts delivery. |
| `stores.timeout` | [S04](01-blackout-stores.md#s04) | Fact/outcome retained; [A] performance adapts delivery. |
| `dispatch.arrival` | [D01](02-dispatch.md#d01) | Fact/outcome retained; [A] performance adapts delivery. |
| `dispatch.heat_returns` | [D02](02-dispatch.md#d02) | Fact/outcome retained; [A] performance adapts delivery. |
| `dispatch.radio_first_cost` | [D02](02-dispatch.md#d02) | Fact/outcome retained; [A] performance adapts delivery. |
| `dispatch.warm_hands` | [D03](02-dispatch.md#d03) | Fact/outcome retained; [A] performance adapts delivery. |
| `dispatch.warm_again` | [D03](02-dispatch.md#d03) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.arrival` | [G01](03-generator.md#g01) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.radio_warning` | [G06](03-generator.md#g06) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.heat_pack` | [G02](03-generator.md#g02) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.open_generator_latch_numb` | [G03](03-generator.md#g03) | Fact/outcome retained; [A] performance adapts delivery. |
| `whiteout.latch_open` | [G03](03-generator.md#g03) | Fact/outcome retained; [A] performance adapts delivery. |
| `whiteout.latch_nearly` | [G03](03-generator.md#g03) | Fact/outcome retained; [A] performance adapts delivery. |
| `whiteout.latch_crack` | [G03](03-generator.md#g03) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.work_latch_with_numb_hands_numb` | [G03](03-generator.md#g03) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.lever_made` | [G04](03-generator.md#g04) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.lever_failed` | [G04](03-generator.md#g04) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.lever_open` | [G04](03-generator.md#g04) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.controls` | [G05](03-generator.md#g05) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.isolated` | [G05](03-generator.md#g05) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.overload` | [G07](03-generator.md#g07) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.starter_cold` | [G07](03-generator.md#g07) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.startup_prime` | [G07](03-generator.md#g07) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.startup_ignition` | [G07](03-generator.md#g07) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.startup_heat` | [G07](03-generator.md#g07) | Fact/outcome retained; [A] performance adapts delivery. |
| `generator.timeout` | [G08](03-generator.md#g08) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.arrival` | [R01](04-radio.md#r01) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.gust_warning` | [R04](04-radio.md#r04) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.adapter` | [R02](04-radio.md#r02) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.alignment_failed` | [R03](04-radio.md#r03) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.aligned` | [R03](04-radio.md#r03) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.cold_transmit_failure` | [R05](04-radio.md#r05) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.automatic_call` | [R05](04-radio.md#r05) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.manual_call` | [R05](04-radio.md#r05) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.timeout` | [R07](04-radio.md#r07) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.warmed_hands` | [R06](04-radio.md#r06) | Fact/outcome retained; [A] performance adapts delivery. |
| `radio.warm_again` | [R06](04-radio.md#r06) | Fact/outcome retained; [A] performance adapts delivery. |
| `crisis.arrival` | [C01](05-ridge-endings.md#c01) | Fact/outcome retained; [A] performance adapts delivery. |
| `crisis.missing_preparation` | [C02](05-ridge-endings.md#c02) | Fact/outcome retained; [A] performance adapts delivery. |
| `crisis.timeout` | [C06](05-ridge-endings.md#c06) | Fact/outcome retained; [A] performance adapts delivery. |
| `ending_full.arrival` | [E01](05-ridge-endings.md#e01) | Fact/outcome retained; [A] performance adapts delivery. |
| `ending_full.truth` | [E02](05-ridge-endings.md#e02) | Fact/outcome retained; [A] performance adapts delivery. |
| `ending_partial.arrival` | [E03](05-ridge-endings.md#e03) | Fact/outcome retained; [A] performance adapts delivery. |
| `ending_partial.jonas` | [E04](05-ridge-endings.md#e04) | Fact/outcome retained; [A] performance adapts delivery. |
| `ending_partial.lea` | [E04](05-ridge-endings.md#e04) | Fact/outcome retained; [A] performance adapts delivery. |
| `ending_partial.truth` | [E04](05-ridge-endings.md#e04) | Fact/outcome retained; [A] performance adapts delivery. |
| `ending_lost.arrival` | [E05](05-ridge-endings.md#e05) | Fact/outcome retained; [A] performance adapts delivery. |
| `dispatch.return_brief` | [D02](02-dispatch.md#d02) | Fact/outcome retained; [A] performance adapts delivery. |

## Every authored cue instance

Paths are context-relative YAML locations (one-based action/outcome/list indices).
They distinguish reused text from unique cue instructions. Complete conditions
and timings are in RULES. No cue is removed by this screenplay.

| Context / source path | Cue ID | Content / operation | Screenplay |
| --- | --- | --- | --- |
| `blackout/cues/1` | `arrival` | TEXT `blackout.arrival`; 12000ms | [B01](01-blackout-stores.md#b01) |
| `blackout/cues/2` | `choices` | actions eligible | [blackout](01-blackout-stores.md) |
| `stores/cues/1` | `deadline` | set_deadline_ms=180000; visible=true | [stores](01-blackout-stores.md) |
| `stores/cues/2` | `arrival` | TEXT `stores.arrival`; 7000ms | [S01](01-blackout-stores.md#s01) |
| `stores/cues/3` | `choices` | actions eligible | [stores](01-blackout-stores.md) |
| `stores/actions/1/outcomes/1/cues/1` | `thermal_liner` | TEXT `stores.thermal_liner`; 1800ms | [S02](01-blackout-stores.md#s02) |
| `stores/actions/2/outcomes/1/cues/1` | `jonas_treated` | TEXT `stores.jonas_treated`; 1800ms | [S02](01-blackout-stores.md#s02) |
| `stores/actions/3/outcomes/1/cues/1` | `copper_braid` | TEXT `stores.copper_braid`; 1800ms | [S02](01-blackout-stores.md#s02) |
| `stores/actions/4/outcomes/1/cues/1` | `ceramic_mug` | TEXT `stores.ceramic_mug`; 1800ms | [S02](01-blackout-stores.md#s02) |
| `stores/actions/5/outcomes/1/cues/1` | `spanner` | TEXT `stores.spanner`; 1800ms | [S02](01-blackout-stores.md#s02) |
| `stores/actions/6/outcomes/1/cues/1` | `safety_line` | TEXT `stores.safety_line`; 1800ms | [S02](01-blackout-stores.md#s02) |
| `stores/actions/7/outcomes/1/cues/1` | `heat_pack` | TEXT `stores.heat_pack`; 1800ms | [S02](01-blackout-stores.md#s02) |
| `stores/actions/8/outcomes/1/cues/1` | `log_read` | TEXT `stores.log_read`; 1800ms | [S03](01-blackout-stores.md#s03) |
| `stores/timeout/cues/1` | `timeout` | TEXT `stores.timeout`; 1800ms | [S04](01-blackout-stores.md#s04) |
| `dispatch/cues/1` | `heat_returns` | TEXT `dispatch.heat_returns`; 1800ms | [D02](02-dispatch.md#d02) |
| `dispatch/cues/2` | `radio_first_cost` | TEXT `dispatch.radio_first_cost`; 1800ms | [D02](02-dispatch.md#d02) |
| `dispatch/cues/3` | `arrival` | TEXT `dispatch.arrival`; 1800ms | [D01](02-dispatch.md#d01) |
| `dispatch/cues/4` | `return_brief` | TEXT `dispatch.return_brief`; 1800ms | [D02](02-dispatch.md#d02) |
| `dispatch/cues/5` | `choices` | actions eligible | [dispatch](02-dispatch.md) |
| `dispatch/actions/3/outcomes/1/cues/1` | `warm_hands` | TEXT `dispatch.warm_hands`; 1800ms | [D03](02-dispatch.md#d03) |
| `dispatch/actions/3/outcomes/2/cues/1` | `warm_again` | TEXT `dispatch.warm_again`; 1800ms | [D03](02-dispatch.md#d03) |
| `generator/cues/1` | `deadline` | set_deadline_ms=30000; visible=false | [generator](03-generator.md) |
| `generator/cues/2` | `arrival` | TEXT `generator.arrival`; 4000ms | [G01](03-generator.md#g01) |
| `generator/cues/3` | `choices` | actions eligible | [generator](03-generator.md) |
| `generator/cues/4` | `radio_warning` | TEXT `generator.radio_warning`; 2200ms | [G06](03-generator.md#g06) |
| `generator/cues/5` | `reveal_clock` | visible=true | [generator](03-generator.md) |
| `generator/actions/1/outcomes/1/cues/1` | `extra_minute` | extend_ms=60000 | [generator](03-generator.md) |
| `generator/actions/1/outcomes/1/cues/2` | `heat_pack` | TEXT `generator.heat_pack`; 1800ms | [G02](03-generator.md#g02) |
| `generator/actions/2/outcomes/1/cues/1` | `open_generator_latch_numb` | TEXT `generator.open_generator_latch_numb`; 1800ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/2/cues/1` | `open_generator_latch_assisted_open` | TEXT `whiteout.latch_open`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/2/cues/2` | `open_generator_latch_assisted_nearly` | TEXT `whiteout.latch_nearly`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/2/cues/3` | `open_generator_latch_assisted_crack` | TEXT `whiteout.latch_crack`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/2/cues/4` | `open_generator_latch_assisted_release` | actions eligible; explicit at_ms=0 | [generator](03-generator.md) |
| `generator/actions/2/outcomes/3/cues/1` | `open_generator_latch_steady_open` | TEXT `whiteout.latch_open`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/3/cues/2` | `open_generator_latch_steady_nearly` | TEXT `whiteout.latch_nearly`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/3/cues/3` | `open_generator_latch_steady_crack` | TEXT `whiteout.latch_crack`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/3/cues/4` | `open_generator_latch_steady_release` | actions eligible; explicit at_ms=0 | [generator](03-generator.md) |
| `generator/actions/2/outcomes/4/cues/1` | `open_generator_latch_slow_open` | TEXT `whiteout.latch_open`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/4/cues/2` | `open_generator_latch_slow_nearly` | TEXT `whiteout.latch_nearly`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/4/cues/3` | `open_generator_latch_slow_crack` | TEXT `whiteout.latch_crack`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/2/outcomes/4/cues/4` | `open_generator_latch_slow_release` | actions eligible; explicit at_ms=0 | [generator](03-generator.md) |
| `generator/actions/3/outcomes/1/cues/1` | `work_latch_with_numb_hands_numb` | TEXT `generator.work_latch_with_numb_hands_numb`; 1800ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/2/cues/1` | `work_latch_with_numb_hands_assisted_open` | TEXT `whiteout.latch_open`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/2/cues/2` | `work_latch_with_numb_hands_assisted_nearly` | TEXT `whiteout.latch_nearly`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/2/cues/3` | `work_latch_with_numb_hands_assisted_crack` | TEXT `whiteout.latch_crack`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/2/cues/4` | `work_latch_with_numb_hands_assisted_release` | actions eligible; explicit at_ms=0 | [generator](03-generator.md) |
| `generator/actions/3/outcomes/3/cues/1` | `work_latch_with_numb_hands_steady_open` | TEXT `whiteout.latch_open`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/3/cues/2` | `work_latch_with_numb_hands_steady_nearly` | TEXT `whiteout.latch_nearly`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/3/cues/3` | `work_latch_with_numb_hands_steady_crack` | TEXT `whiteout.latch_crack`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/3/cues/4` | `work_latch_with_numb_hands_steady_release` | actions eligible; explicit at_ms=0 | [generator](03-generator.md) |
| `generator/actions/3/outcomes/4/cues/1` | `work_latch_with_numb_hands_slow_open` | TEXT `whiteout.latch_open`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/4/cues/2` | `work_latch_with_numb_hands_slow_nearly` | TEXT `whiteout.latch_nearly`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/4/cues/3` | `work_latch_with_numb_hands_slow_crack` | TEXT `whiteout.latch_crack`; 3500ms | [G03](03-generator.md#g03) |
| `generator/actions/3/outcomes/4/cues/4` | `work_latch_with_numb_hands_slow_release` | actions eligible; explicit at_ms=0 | [generator](03-generator.md) |
| `generator/actions/4/outcomes/1/cues/1` | `lever_made` | TEXT `generator.lever_made`; 1800ms | [G04](03-generator.md#g04) |
| `generator/actions/5/outcomes/1/cues/1` | `lever_failed` | TEXT `generator.lever_failed`; 1800ms | [G04](03-generator.md#g04) |
| `generator/actions/5/outcomes/2/cues/1` | `lever_open` | TEXT `generator.lever_open`; 1800ms | [G04](03-generator.md#g04) |
| `generator/actions/6/outcomes/1/cues/1` | `controls` | TEXT `generator.controls`; 1800ms | [G05](03-generator.md#g05) |
| `generator/actions/7/outcomes/1/cues/1` | `isolated` | TEXT `generator.isolated`; 1800ms | [G05](03-generator.md#g05) |
| `generator/actions/8/outcomes/1/cues/1` | `overload` | TEXT `generator.overload`; 1800ms | [G07](03-generator.md#g07) |
| `generator/actions/8/outcomes/2/cues/1` | `starter_cold` | TEXT `generator.starter_cold`; 1800ms | [G07](03-generator.md#g07) |
| `generator/actions/8/outcomes/3/cues/1` | `generation_deadline_cancelled` | unset_deadline=true | [generator](03-generator.md) |
| `generator/actions/8/outcomes/3/cues/2` | `startup_lock` | actions [] | [generator](03-generator.md) |
| `generator/actions/8/outcomes/3/cues/3` | `startup_prime` | TEXT `generator.startup_prime`; 1200ms | [G07](03-generator.md#g07) |
| `generator/actions/8/outcomes/3/cues/4` | `startup_ignition` | TEXT `generator.startup_ignition`; 1500ms | [G07](03-generator.md#g07) |
| `generator/actions/8/outcomes/3/cues/5` | `startup_heat` | TEXT `generator.startup_heat`; 1800ms | [G07](03-generator.md#g07) |
| `generator/actions/8/outcomes/3/cues/6` | `startup_ready` | actions eligible | [generator](03-generator.md) |
| `generator/timeout/cues/1` | `timeout` | TEXT `generator.timeout`; 1800ms | [G08](03-generator.md#g08) |
| `radio/cues/1` | `deadline` | set_deadline_ms=60000; visible=true | [radio](04-radio.md) |
| `radio/cues/2` | `arrival` | TEXT `radio.arrival`; 6000ms | [R01](04-radio.md#r01) |
| `radio/cues/3` | `choices` | actions eligible | [radio](04-radio.md) |
| `radio/cues/4` | `gust_warning` | TEXT `radio.gust_warning`; 1800ms | [R04](04-radio.md#r04) |
| `radio/actions/1/outcomes/1/cues/1` | `adapter` | TEXT `radio.adapter`; 1800ms | [R02](04-radio.md#r02) |
| `radio/actions/2/outcomes/1/cues/1` | `alignment_failed` | TEXT `radio.alignment_failed`; 1800ms | [R03](04-radio.md#r03) |
| `radio/actions/2/outcomes/2/cues/1` | `aligned` | TEXT `radio.aligned`; 1800ms | [R03](04-radio.md#r03) |
| `radio/actions/3/outcomes/1/cues/1` | `cold_transmit_failure` | TEXT `radio.cold_transmit_failure`; 1800ms | [R05](04-radio.md#r05) |
| `radio/actions/3/outcomes/2/cues/1` | `automatic_call_deadline_cancelled` | unset_deadline=true | [radio](04-radio.md) |
| `radio/actions/3/outcomes/2/cues/2` | `automatic_call` | TEXT `radio.automatic_call`; 1800ms | [R05](04-radio.md#r05) |
| `radio/actions/3/outcomes/3/cues/1` | `manual_call_deadline_cancelled` | unset_deadline=true | [radio](04-radio.md) |
| `radio/actions/3/outcomes/3/cues/2` | `manual_call` | TEXT `radio.manual_call`; 1800ms | [R05](04-radio.md#r05) |
| `radio/actions/4/outcomes/1/cues/1` | `warmed_hands` | TEXT `radio.warmed_hands`; 1800ms | [R06](04-radio.md#r06) |
| `radio/actions/4/outcomes/2/cues/1` | `warm_again` | TEXT `radio.warm_again`; 1800ms | [R06](04-radio.md#r06) |
| `radio/timeout/cues/1` | `timeout` | TEXT `radio.timeout`; 1800ms | [R07](04-radio.md#r07) |
| `crisis/cues/1` | `deadline` | set_deadline_ms=45000; visible=true | [crisis](05-ridge-endings.md) |
| `crisis/cues/2` | `arrival` | TEXT `crisis.arrival`; 8000ms | [C01](05-ridge-endings.md#c01) |
| `crisis/cues/3` | `choices` | actions eligible | [crisis](05-ridge-endings.md) |
| `crisis/actions/1/outcomes/2/cues/1` | `missing_preparation` | TEXT `crisis.missing_preparation`; 1800ms | [C02](05-ridge-endings.md#c02) |
| `crisis/timeout/cues/1` | `timeout` | TEXT `crisis.timeout`; 1800ms | [C06](05-ridge-endings.md#c06) |
| `ending_full/cues/1` | `arrival` | TEXT `ending_full.arrival`; 1800ms | [E01](05-ridge-endings.md#e01) |
| `ending_full/cues/2` | `truth` | TEXT `ending_full.truth`; 1800ms | [E02](05-ridge-endings.md#e02) |
| `ending_partial/cues/1` | `arrival` | TEXT `ending_partial.arrival`; 1800ms | [E03](05-ridge-endings.md#e03) |
| `ending_partial/cues/2` | `jonas` | TEXT `ending_partial.jonas`; 1800ms | [E04](05-ridge-endings.md#e04) |
| `ending_partial/cues/3` | `lea` | TEXT `ending_partial.lea`; 1800ms | [E04](05-ridge-endings.md#e04) |
| `ending_partial/cues/4` | `truth` | TEXT `ending_partial.truth`; 1800ms | [E04](05-ridge-endings.md#e04) |
| `ending_lost/cues/1` | `arrival` | TEXT `ending_lost.arrival`; 1800ms | [E05](05-ridge-endings.md#e05) |

## Source integrity

SHA-256 at the baseline; validation compares executable files byte-for-byte:

- `story.yaml`: `92401bc48a168ed187ca391dbbb5cf7abc8320e39f0b2dd6ae2843d86bdb6a09`
- `text.yaml`: `746908a36395c4bbceccee468574eaf398b79bf030512ef8c579afeeec085652`
- `scenes.yaml`: `555e7c20b9948d72f3d27c9250fe56440a5012cc6ddf7941ffd249eaeaf94343`
