# Environment / set inventory — stable production references

Existing IDs are inherited from the full screenplay. Every new `env.*` and the
three new establishing `cam.*` IDs below are explicitly **[A] production
proposals**, not GRIMM protocol variants, runtime bindings, ECS IDs or delivered
level files. Reuse one identity/geometry beneath state variants; don't create
a second dispatch room because light changes.

## Set identity / source mapping

| Existing location ID | Canonical context / beats | Specification / reuse |
| --- | --- | --- |
| `loc.aurora_corridor` | blackout / B01 | 01; shared wall/door/service kit, four-person initial space. |
| `loc.emergency_stores` | stores / S01–S04 | 02; same shared threshold, preparation objects removed only as taken. |
| `loc.dispatch_hall` | dispatch / D01–D03 | 03; fixed four thresholds, northwest duct/bench across visits. |
| `loc.generator_annex` | generator / G01–G08 | 04; same hinge/cover/front isolator, protected support/retreat. |
| `loc.radio_cabin` | radio / R01–R07 | 05; cabin + adjacent mast one connected set/time context. |
| `loc.ridge_crossing` | crisis / C01–C06 | 06; same markers, west shelter/east manifold and named targets. |
| `loc.rescue_cabin` | ending_full / E01; ending_partial / E03–E04 | 07; four-position transport composition, exact casualty dressing. |
| `loc.daylight_report` | conditional truth / E02/E04 | 07; small insert set, no new playable office. |
| `loc.aurora_exterior` | lost / E05, master silhouette reference | 07 and 01; same building, burial/spring overlays, no new level. |

## Reusable architecture / environment modules — all new [A]

| Proposed asset ID | Reuse / scope | Required variants / constraints |
| --- | --- | --- |
| `env.wall_panel` | All station interiors/exterior envelope. | Pale insulated panel, charcoal kick strip; seams survive reverse views. |
| `env.door_frame` | Corridor, stores, hall, annex, radio, ridge threshold. | Wide service frame, local closure/dressing; not an automatically usable story edge. |
| `env.sealed_window` | Corridor/hall/annex/radio. | Cool snow fill, condensation/frost; never escape opening. |
| `env.floor_mat` | Interior working/carry lanes. | Dry/damp/local snow overlay; no random collision clutter. |
| `env.cable_tray` | Hall/annex/radio/stores. | Wall/roof routing, not a trip/rope solution. |
| `env.service_duct` | Hall heat and engineering connections. | Cold/warm appearance follows actual power, no added heater mechanic. |
| `env.rest_bench` | Stores triage / hall rest, reusable model with distinct placed instances. | Ankle/support clearance; not two independent Jonases. |
| `env.brace_rail` | Annex supported brace / radio maintenance support. | Hand/shoulder contacts; not a permanent ridge safety cable or free game solution. |
| `env.reserve_lamp` | Local shutter/radio/service indicators. | Small battery light, separate from restored ceiling/work supply. |
| `env.work_lamp` | Powered station task lighting. | Off/steady powered, same fixture; not random new practicals each shot. |
| `env.snow_layer` | Thresholds/exterior/mast/ridge/ending burial. | Same surface identity, controlled depth/occlusion; not weather simulation. |
| `env.frost_layer` | Cold frames/windows/hinge dressing. | Local material mask; cannot hide an offered control. |
| `env.mast_structure` | Radio bay and ridge shelter sightline. | Same modest mast/wheel/cables in both scenes; no new structure on wrong side. |
| `env.exhaust_stack` | Annex exterior/ridge/lost station. | Engineering-side exhaust and broken-guy-wire relationship. |

## One-off set dressing / larger pieces — all new [A]

| Proposed asset ID | Set | Role / no new gameplay |
| --- | --- | --- |
| `env.stores_shelf` | stores | Specific prop stations and gaps, accessible low shelves. |
| `env.stores_rack` | stores | Line/flare/battery placements; no extra usable inventory. |
| `env.repair_tray` | stores | Single braid/mug/spanner grouping, no duplicated copper. |
| `env.heater_manifold` | ridge engineering side | Jonas target's authored physical context; no repair action added. |
| `env.antenna_shelter` | radio/ridge | Lea target and wind protection geometry; not an early escape route. |
| `env.report_table` | optional truth insert | Recovered paper framing, no investigation room/actor. |

## Existing interactive / narrative prop identities — retain, don't duplicate

| Existing IDs | Sets / variants |
| --- | --- |
| `prop.torch`, `prop.handset`, `prop.coveralls`, `prop.gloves` | Cast continuity/illumination, not pickups added by these concepts. |
| `prop.thermal_liner`, `prop.medical_kit`, `prop.splint`, `prop.jonas_tag` | Stores treatment and exact surviving/lost coda; injured leg remains protected. |
| `prop.copper_braid`, `prop.ceramic_mug`, `prop.spanner`, `prop.insulated_lever` | Single-resource trade-off, carried/consumed/installed states; gull stays on mug. |
| `prop.safety_line`, `prop.flare`, `prop.heat_pack` | Optional preparation and accepted branch use, no free costume gear. |
| `prop.maintenance_log` | Stores terminal/carried sketch/report insert; knowledge gate unchanged. |
| `prop.stores_shutter`, `prop.annex_shutter`, `prop.radio_door` | Correct room access closure, cast clear first, no new injury. |
| `prop.generator`, `prop.generator_latch`, `prop.latch_ice`, `prop.generator_diagram`, `prop.deicer_fuse`, `prop.primer_tube`, `prop.generator_coil`, `prop.duct_snow` | Annex: same mechanism and progressive variants, external accessible fuse, no padlock/key puzzle. |
| `prop.radio_transmitter`, `prop.radio_reserve`, `prop.antenna_wheel`, `prop.radio_jumper` | Connected cabin/bay: reserve, manual hold versus auto, isolated mug bracket. |
| `prop.ridge_markers`, `prop.guy_wire`, `prop.intake` | Fixed ridge geometry, broken wire and eventual burial; no permanent safe path. |
| `prop.manifest`, `prop.ridge_names`, `prop.rescue_vehicle` | Exact terminal count/evidence/transport variant; no new route or pilot. |

## Existing weather / light FX and sound sources

`fx.light_failure`, `fx.restored_heat`, `fx.mast_snow`, `fx.mast_gust`,
`fx.ridge_drift`, `fx.crossing_erased`, `fx.intake_buried`, `fx.flare_arc`,
`fx.daylight`, `fx.spring_light` retain screenplay intent; STATES maps conditions.
No new canonical effects or weather timing fields are added.

| Existing sound IDs | Placement / continuity |
| --- | --- |
| `snd.station_wind`, `snd.ridge_wind` | Outside envelope/threshold attenuation versus exposed crossing. |
| `snd.test_loop`, `snd.test_loop_decay` | Opening handset and lost battery; no extra ghost source. |
| `snd.shutter_coil`, `snd.shutter_close`, `snd.protector_trip` | Correct door or radio protector; independent local reserve, not engine success. |
| `snd.cloth`, `snd.kit`, `snd.tools`, `snd.connector`, `snd.terminal` | Selected equipment/treatment/read insert Foley, never another pickup. |
| `snd.ice_crack`, `snd.starter_turn`, `snd.motor_run` | Same latch/engine/duct source; only accepted progress/success. |
| `snd.radio_static`, `snd.jumper_clamp`, `snd.beacon_lock`, `snd.real_reply`, `snd.last_bearing` | Desk/mast/handset, with real reply distinguished from old loop. |
| `snd.mast_gust`, `snd.wire_strain`, `snd.names_in_wind`, `snd.vent_choke` | Scheduled gust or authored crisis/ending events, not new deaths. |
| `snd.flare_fire`, `snd.helicopter`, `snd.coastal_patrol` | Actual selected flare/transport branch; not invented equipment success. |

Camera identities (21 inherited `cam.*`, three new [A], plus four existing
concrete `shot.*` references) are fully catalogued in CINEMATOGRAPHY.md. Stage
actor roots remain `actor.alex`, `actor.mara`, `actor.jonas`, `actor.lea` from
Character Bible; model/costume assets are not redefined here.
