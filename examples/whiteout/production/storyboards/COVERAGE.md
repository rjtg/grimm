# Screenplay → authored cue → actual panel coverage

Canonical source: `examples/whiteout/story.yaml`; staging reference: the five screenplay chapters and `RULES.md`. This matrix is an editorial cross-reference, not a reachability solver. Each linked panel is a real image with the same-name editable SVG. Timing and conditions remain in the source and panel notes.

## All authored cue instances

| Source path / cue | Meaning | Actual drawings |
| --- | --- | --- |
| `blackout/cues/0` / `arrival` | `blackout.arrival` | [B01](panels/b01.png) |
| `blackout/cues/1` / `choices` | `actions` | [B05](panels/b05.png) |
| `stores/cues/0` / `deadline` | `timer` | [S03](panels/s03.png), [S04](panels/s04.png), [S05](panels/s05.png), [S06](panels/s06.png), [S07](panels/s07.png), [S08](panels/s08.png), [S09](panels/s09.png), [S10](panels/s10.png), [S11](panels/s11.png) |
| `stores/cues/1` / `arrival` | `stores.arrival` | [S01](panels/s01.png) |
| `stores/cues/2` / `choices` | `actions` | [S03](panels/s03.png), [S04](panels/s04.png), [S05](panels/s05.png), [S06](panels/s06.png), [S07](panels/s07.png), [S08](panels/s08.png), [S09](panels/s09.png), [S10](panels/s10.png), [S11](panels/s11.png) |
| `stores/actions/0/outcomes/0/cues/0` / `thermal_liner` | `stores.thermal_liner` | [S03](panels/s03.png) |
| `stores/actions/1/outcomes/0/cues/0` / `jonas_treated` | `stores.jonas_treated` | [S04](panels/s04.png) |
| `stores/actions/2/outcomes/0/cues/0` / `copper_braid` | `stores.copper_braid` | [S05](panels/s05.png) |
| `stores/actions/3/outcomes/0/cues/0` / `ceramic_mug` | `stores.ceramic_mug` | [S06](panels/s06.png) |
| `stores/actions/4/outcomes/0/cues/0` / `spanner` | `stores.spanner` | [S07](panels/s07.png) |
| `stores/actions/5/outcomes/0/cues/0` / `safety_line` | `stores.safety_line` | [S08](panels/s08.png) |
| `stores/actions/6/outcomes/0/cues/0` / `heat_pack` | `stores.heat_pack` | [S09](panels/s09.png) |
| `stores/actions/7/outcomes/0/cues/0` / `log_read` | `stores.log_read` | [S10](panels/s10.png) |
| `stores/timeout/cues/0` / `timeout` | `stores.timeout` | [S12](panels/s12.png) |
| `dispatch/cues/0` / `heat_returns` | `dispatch.heat_returns` | [D02](panels/d02.png) |
| `dispatch/cues/1` / `radio_first_cost` | `dispatch.radio_first_cost` | [D03](panels/d03.png) |
| `dispatch/cues/2` / `arrival` | `dispatch.arrival` | [D01](panels/d01.png) |
| `dispatch/cues/3` / `return_brief` | `dispatch.return_brief` | [D05](panels/d05.png) |
| `dispatch/cues/4` / `choices` | `actions` | [D01](panels/d01.png), [D04](panels/d04.png), [D06](panels/d06.png) |
| `dispatch/actions/2/outcomes/0/cues/0` / `warm_hands` | `dispatch.warm_hands` | [D04](panels/d04.png) |
| `dispatch/actions/2/outcomes/1/cues/0` / `warm_again` | `dispatch.warm_again` | [D04](panels/d04.png) |
| `generator/cues/0` / `deadline` | `timer` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/cues/1` / `arrival` | `generator.arrival` | [G01](panels/g01.png) |
| `generator/cues/2` / `choices` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/cues/3` / `radio_warning` | `generator.radio_warning` | [G16](panels/g16.png) |
| `generator/cues/4` / `reveal_clock` | `timer` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/0/outcomes/0/cues/0` / `extra_minute` | `timer` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/0/outcomes/0/cues/1` / `heat_pack` | `generator.heat_pack` | [G05](panels/g05.png) |
| `generator/actions/1/outcomes/0/cues/0` / `open_generator_latch_numb` | `generator.open_generator_latch_numb` | [G09](panels/g09.png) |
| `generator/actions/1/outcomes/1/cues/0` / `open_generator_latch_assisted_open` | `whiteout.latch_open` | [G08](panels/g08.png) |
| `generator/actions/1/outcomes/1/cues/1` / `open_generator_latch_assisted_nearly` | `whiteout.latch_nearly` | [G07](panels/g07.png) |
| `generator/actions/1/outcomes/1/cues/2` / `open_generator_latch_assisted_crack` | `whiteout.latch_crack` | [G06](panels/g06.png) |
| `generator/actions/1/outcomes/1/cues/3` / `open_generator_latch_assisted_release` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/1/outcomes/2/cues/0` / `open_generator_latch_steady_open` | `whiteout.latch_open` | [G08](panels/g08.png) |
| `generator/actions/1/outcomes/2/cues/1` / `open_generator_latch_steady_nearly` | `whiteout.latch_nearly` | [G07](panels/g07.png) |
| `generator/actions/1/outcomes/2/cues/2` / `open_generator_latch_steady_crack` | `whiteout.latch_crack` | [G06](panels/g06.png) |
| `generator/actions/1/outcomes/2/cues/3` / `open_generator_latch_steady_release` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/1/outcomes/3/cues/0` / `open_generator_latch_slow_open` | `whiteout.latch_open` | [G08](panels/g08.png) |
| `generator/actions/1/outcomes/3/cues/1` / `open_generator_latch_slow_nearly` | `whiteout.latch_nearly` | [G07](panels/g07.png) |
| `generator/actions/1/outcomes/3/cues/2` / `open_generator_latch_slow_crack` | `whiteout.latch_crack` | [G06](panels/g06.png) |
| `generator/actions/1/outcomes/3/cues/3` / `open_generator_latch_slow_release` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/2/outcomes/0/cues/0` / `work_latch_with_numb_hands_numb` | `generator.work_latch_with_numb_hands_numb` | [G09](panels/g09.png) |
| `generator/actions/2/outcomes/1/cues/0` / `work_latch_with_numb_hands_assisted_open` | `whiteout.latch_open` | [G08](panels/g08.png) |
| `generator/actions/2/outcomes/1/cues/1` / `work_latch_with_numb_hands_assisted_nearly` | `whiteout.latch_nearly` | [G07](panels/g07.png) |
| `generator/actions/2/outcomes/1/cues/2` / `work_latch_with_numb_hands_assisted_crack` | `whiteout.latch_crack` | [G06](panels/g06.png) |
| `generator/actions/2/outcomes/1/cues/3` / `work_latch_with_numb_hands_assisted_release` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/2/outcomes/2/cues/0` / `work_latch_with_numb_hands_steady_open` | `whiteout.latch_open` | [G08](panels/g08.png) |
| `generator/actions/2/outcomes/2/cues/1` / `work_latch_with_numb_hands_steady_nearly` | `whiteout.latch_nearly` | [G07](panels/g07.png) |
| `generator/actions/2/outcomes/2/cues/2` / `work_latch_with_numb_hands_steady_crack` | `whiteout.latch_crack` | [G06](panels/g06.png) |
| `generator/actions/2/outcomes/2/cues/3` / `work_latch_with_numb_hands_steady_release` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/2/outcomes/3/cues/0` / `work_latch_with_numb_hands_slow_open` | `whiteout.latch_open` | [G08](panels/g08.png) |
| `generator/actions/2/outcomes/3/cues/1` / `work_latch_with_numb_hands_slow_nearly` | `whiteout.latch_nearly` | [G07](panels/g07.png) |
| `generator/actions/2/outcomes/3/cues/2` / `work_latch_with_numb_hands_slow_crack` | `whiteout.latch_crack` | [G06](panels/g06.png) |
| `generator/actions/2/outcomes/3/cues/3` / `work_latch_with_numb_hands_slow_release` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/3/outcomes/0/cues/0` / `lever_made` | `generator.lever_made` | [G10](panels/g10.png) |
| `generator/actions/4/outcomes/0/cues/0` / `lever_failed` | `generator.lever_failed` | [G11](panels/g11.png) |
| `generator/actions/4/outcomes/1/cues/0` / `lever_open` | `generator.lever_open` | [G08](panels/g08.png) |
| `generator/actions/5/outcomes/0/cues/0` / `controls` | `generator.controls` | [G04](panels/g04.png) |
| `generator/actions/6/outcomes/0/cues/0` / `isolated` | `generator.isolated` | [G13](panels/g13.png) |
| `generator/actions/7/outcomes/0/cues/0` / `overload` | `generator.overload` | [G14](panels/g14.png) |
| `generator/actions/7/outcomes/1/cues/0` / `starter_cold` | `generator.starter_cold` | [G15](panels/g15.png) |
| `generator/actions/7/outcomes/2/cues/0` / `generation_deadline_cancelled` | `timer` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/7/outcomes/2/cues/1` / `startup_lock` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/actions/7/outcomes/2/cues/2` / `startup_prime` | `generator.startup_prime` | [G18](panels/g18.png) |
| `generator/actions/7/outcomes/2/cues/3` / `startup_ignition` | `generator.startup_ignition` | [G19](panels/g19.png) |
| `generator/actions/7/outcomes/2/cues/4` / `startup_heat` | `generator.startup_heat` | [G20](panels/g20.png) |
| `generator/actions/7/outcomes/2/cues/5` / `startup_ready` | `actions` | [G04](panels/g04.png), [G05](panels/g05.png), [G06](panels/g06.png), [G10](panels/g10.png), [G11](panels/g11.png), [G13](panels/g13.png), [G18](panels/g18.png), [G21](panels/g21.png), [G22](panels/g22.png) |
| `generator/timeout/cues/0` / `timeout` | `generator.timeout` | [G23](panels/g23.png) |
| `radio/cues/0` / `deadline` | `timer` | [R03](panels/r03.png), [R05](panels/r05.png), [R08](panels/r08.png), [R11](panels/r11.png), [R12](panels/r12.png), [R13](panels/r13.png) |
| `radio/cues/1` / `arrival` | `radio.arrival` | [R01](panels/r01.png) |
| `radio/cues/2` / `choices` | `actions` | [R03](panels/r03.png), [R05](panels/r05.png), [R08](panels/r08.png), [R11](panels/r11.png), [R12](panels/r12.png), [R13](panels/r13.png) |
| `radio/cues/3` / `gust_warning` | `radio.gust_warning` | [R07](panels/r07.png) |
| `radio/actions/0/outcomes/0/cues/0` / `adapter` | `radio.adapter` | [R03](panels/r03.png) |
| `radio/actions/1/outcomes/0/cues/0` / `alignment_failed` | `radio.alignment_failed` | [R06](panels/r06.png) |
| `radio/actions/1/outcomes/1/cues/0` / `aligned` | `radio.aligned` | [R05](panels/r05.png) |
| `radio/actions/2/outcomes/0/cues/0` / `cold_transmit_failure` | `radio.cold_transmit_failure` | [R08](panels/r08.png) |
| `radio/actions/2/outcomes/1/cues/0` / `automatic_call_deadline_cancelled` | `timer` | [R03](panels/r03.png), [R05](panels/r05.png), [R08](panels/r08.png), [R11](panels/r11.png), [R12](panels/r12.png), [R13](panels/r13.png) |
| `radio/actions/2/outcomes/1/cues/1` / `automatic_call` | `radio.automatic_call` | [R09](panels/r09.png) |
| `radio/actions/2/outcomes/2/cues/0` / `manual_call_deadline_cancelled` | `timer` | [R03](panels/r03.png), [R05](panels/r05.png), [R08](panels/r08.png), [R11](panels/r11.png), [R12](panels/r12.png), [R13](panels/r13.png) |
| `radio/actions/2/outcomes/2/cues/1` / `manual_call` | `radio.manual_call` | [R10](panels/r10.png) |
| `radio/actions/3/outcomes/0/cues/0` / `warmed_hands` | `radio.warmed_hands` | [R11](panels/r11.png) |
| `radio/actions/3/outcomes/1/cues/0` / `warm_again` | `radio.warm_again` | [R11](panels/r11.png) |
| `radio/timeout/cues/0` / `timeout` | `radio.timeout` | [R13](panels/r13.png) |
| `crisis/cues/0` / `deadline` | `timer` | [C03](panels/c03.png), [C06](panels/c06.png), [C08](panels/c08.png), [C10](panels/c10.png), [C11](panels/c11.png) |
| `crisis/cues/1` / `arrival` | `crisis.arrival` | [C01](panels/c01.png) |
| `crisis/cues/2` / `choices` | `actions` | [C03](panels/c03.png), [C06](panels/c06.png), [C08](panels/c08.png), [C10](panels/c10.png), [C11](panels/c11.png) |
| `crisis/actions/0/outcomes/1/cues/0` / `missing_preparation` | `crisis.missing_preparation` | [C03](panels/c03.png) |
| `crisis/timeout/cues/0` / `timeout` | `crisis.timeout` | [C12](panels/c12.png) |
| `ending_full/cues/0` / `arrival` | `ending_full.arrival` | [E01](panels/e01.png) |
| `ending_full/cues/1` / `truth` | `ending_full.truth` | [E02](panels/e02.png) |
| `ending_partial/cues/0` / `arrival` | `ending_partial.arrival` | [E03](panels/e03.png) |
| `ending_partial/cues/1` / `jonas` | `ending_partial.jonas` | [E04](panels/e04.png) |
| `ending_partial/cues/2` / `lea` | `ending_partial.lea` | [E06](panels/e06.png) |
| `ending_partial/cues/3` / `truth` | `ending_partial.truth` | [E07](panels/e07.png) |
| `ending_lost/cues/0` / `arrival` | `ending_lost.arrival` | [E08](panels/e08.png) |

## Player decisions

| Context/action | Decision or consequence panels |
| --- | --- |
| `blackout.begin` | [B05](panels/b05.png) |
| `stores.take_thermal_liner` | [S03](panels/s03.png) |
| `stores.jonas_treated` | [S04](panels/s04.png) |
| `stores.take_copper_braid` | [S05](panels/s05.png) |
| `stores.take_ceramic_mug` | [S06](panels/s06.png) |
| `stores.take_spanner` | [S07](panels/s07.png) |
| `stores.take_safety_line` | [S08](panels/s08.png) |
| `stores.take_heat_pack` | [S09](panels/s09.png) |
| `stores.log_read` | [S10](panels/s10.png) |
| `stores.leave` | [S11](panels/s11.png) |
| `dispatch.engineering` | [D01](panels/d01.png) |
| `dispatch.communications` | [D01](panels/d01.png) |
| `dispatch.warm_hands` | [D04](panels/d04.png) |
| `dispatch.ridge` | [D06](panels/d06.png) |
| `generator.deploy_heat_pack` | [G05](panels/g05.png) |
| `generator.open_generator_latch` | [G06](panels/g06.png) |
| `generator.work_latch_with_numb_hands` | [G06](panels/g06.png) |
| `generator.make_lever` | [G10](panels/g10.png) |
| `generator.use_lever` | [G11](panels/g11.png) |
| `generator.inspect_controls` | [G04](panels/g04.png) |
| `generator.isolate_deicer` | [G13](panels/g13.png) |
| `generator.start_generator` | [G18](panels/g18.png) |
| `generator.return` | [G21](panels/g21.png) |
| `generator.abandon` | [G22](panels/g22.png) |
| `radio.make_adapter` | [R03](panels/r03.png) |
| `radio.hand_align` | [R05](panels/r05.png) |
| `radio.send_signal` | [R08](panels/r08.png) |
| `radio.warm_hands` | [R11](panels/r11.png) |
| `radio.return` | [R12](panels/r12.png) |
| `radio.abandon` | [R13](panels/r13.png) |
| `crisis.coordinate` | [C03](panels/c03.png) |
| `crisis.help_lea` | [C06](panels/c06.png) |
| `crisis.help_jonas` | [C08](panels/c08.png) |
| `crisis.flare_retreat` | [C10](panels/c10.png) |
| `crisis.stay` | [C11](panels/c11.png) |

The eight dormant narrative cues in the numb-latch assisted/steady branches remain mapped to latch imagery; their cold<40000 guards conflict with that action’s cold>=40000 availability. They are not claimed as reachable playthroughs, and no rules are changed.

Timer/action cues share the scene’s decision compositions; the individual panel notes and BRANCHES distinguish initial, updated, locked, expired and deadline-removed states. The matrix is coverage, not an instruction to replay a whole contact sheet after each action.
