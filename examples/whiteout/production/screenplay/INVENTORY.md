# Production inventory — stable identities, not runtime entities

IDs below are editorial production identifiers. They are not ECS Entity IDs,
new WorldState fields, protocol types or promises of exported assets. Reuse an
identity across contexts; variant pose, condition and costume are presentation
choices. **[C]** facts come from the canonical game; **[A]** details are proposed
for production and may be revised without changing the story. No assets are
generated or procured by this PR. Context names in tables are canonical IDs.

## Cast and relationships

| Identity / voice | Canonical characterization [C] | Proposed appearance / equipment [A] | Performance, emotion and continuity | Contexts |
| --- | --- | --- | --- | --- |
| `actor.alex` / `voice.alex` | Player field engineer; responsible for repair and rescue choices. No fixed gender/age/ethnicity. | Weathered neutral work coveralls, gloves, boots, torch; liner overlay only if taken. Player framing may favour hands/over-shoulder but needs a body for carry shots. | Assess, deliberate, work, cold grip failure, warm hands, supported carry, flare, restrained relief/grief. Never claim success before rules. | All nine; not alive in lost ending, exterior evidence only. |
| `actor.mara` / `voice.mara` | Counts four; cares for injured Jonas, assists rescues, reads names. Relationship is practical care, not invented romance/rank. | Teal outer layer (prototype continuity), torch, ordinary winter gloves; medical tag only in Jonas-loss coda. | Count, crouch, brace, squeeze wrist, stop unsafe plan, support carry, call unanswered names. Firm under pressure; relief or grief follows survivors. | blackout, stores, dispatch, generator threshold, crisis, full/partial; lost ending via absence. Radio by contact if needed. |
| `actor.jonas` / `voice.jonas` | Crushed ankle; temporary bypass signatory, not saboteur. Treatment enables help; heat/order influence later crossing. | Orange work layer (prototype), protective boots, visibly injured ankle; splint only after treatment, tag. Age/facial marks deliberately unspecified. | Seated pain, shiver, low brace with support, recognize diagram, exhale, supported steps only when branch allows. No miraculous healed gait; no villain tells. | blackout, stores, dispatch, generator if supported staging; radio voice/background contact; crisis; full or surviving partial. |
| `actor.lea` / `voice.lea` | Radio operator; owns gull mug; warns of budgets, holds manual antenna, needs rescue or hands-free arrangement. | Muted blue-grey outerwear [A], headset/handheld and gloves; distinct from Mara's teal through silhouette and equipment. | Listen to loop, point jumper, catch/hold wheel, strained shoulder, release/shaking hands, unresolved sentence after Jonas loss. Never leave manual bearing unattended. | blackout, stores, radio, crisis, full or surviving partial; dispatch/generator over handset or when actually free. |
| `voice.coast` | Unnamed real responding rescue/coastal voice; not the failed test loop. | Offscreen only; no named pilot or rescue character model required. | Live acknowledgement contrasts identical loop; no new backstory. | radio success, ending_full / ending_partial as relevant. |
| Offscreen spring searchers | Unnamed people find four names in spring. | No character asset required for current staging; camera sees evidence. | No surviving-player twist. | ending_lost. |

No new family relationships, betrayals, biographies or supernatural explanation
are established. Emergency call “performer” uses existing voice processing, not
a fifth named person. Costumes/age/accents beyond the source remain casting
questions, not facts presented as canonical.

### Injury / presence continuity for production

Mara keeps Jonas in dispatch before a route is selected. For the canonical
assisted latch, proposed blocking places treated Jonas low at the annex threshold
supported by Mara; unassisted versions keep him sheltered. This reconciles the
hall and bracing lines without requiring him to walk independently. Exact safe
spatial blocking needs rehearsal; it cannot change `jonas_treated` or help amount.
Lea's radio appearances are diegetic handset audio when physically at the mast.
Do not instantiate duplicate visible Leas in the hall and mast. Later crisis
moves the cast to the authored manifold/shelter positions without new injuries.

## Locations and reusable sets

| Location ID | Space / canonical contexts | Objects, light and weather | Required framing / reuse |
| --- | --- | --- | --- |
| `loc.aurora_corridor` | Linear corridor at blackout; laboratory/sleeping directions establish station, no new explorable rooms. | Windows into horizonless snow, sequential failing lights, intake/motor indicators. | Wide ceiling axis, four-face tableau, ankle/radio inserts. Reuse door/wall kit with hall. |
| `loc.emergency_stores` | Stores shelf, tool tray, evacuation rack, terminal, shutter threshold. `stores`. | Single braid, gull mug, spanner, liner/kit, line/flare, battery; torch + reserve indicator. | One readable equipment wide, tray insert, log insert, low splint view, exit shutter. |
| `loc.dispatch_hall` | Engineering/right, radio/left, ridge/depth; repeated `dispatch` visits. | Duct/resting place, doors marking repairs/seals, torch versus restored practical light. | Repeatable routes wide with state-dependent cast poses; hand-warming/line inserts. |
| `loc.generator_annex` | Engine body and latch, external diagram/isolator, coil shutter, hall threshold. `generator`. | Ice, primer tube, fuse, gauge, heat-pack socket; red overload/steady power lamp, wind at exterior. | Wide, hinge/tool/coil/controls/primer inserts; supported low Jonas pose. Keep fuse accessible without opening starter cover. |
| `loc.radio_cabin` | Cabin desk + adjacent mast platform as one connected set. `radio`. | Transmitter/battery protector, bracket, wheel/dish, vent, mug jumper; warm interior only with power, constant storm outside. | Interior-to-mast axis, assembly/control inserts, exterior wheel medium. Both halves share one clock. |
| `loc.ridge_crossing` | Door, manifold, broken guy wire, marked crossing, shelter, coastal-track sightline. `crisis`. | Burial drifts, intake, line, flare; dark storm and small practical lights. | Two separated rescue targets, line and face inserts, reverse to false shelter, obscured track. Reuse annex/mast background elements, preserve geography. |
| `loc.rescue_cabin` | Terminal pickup/transport composition, helicopter or coastal-patrol dressing. `ending_full`, `ending_partial`. | Manifest, four or fewer occupied positions, medical tag/handset; emergency light transitioning away from storm. | Four-person / empty-place match, tag and handset inserts. Vehicle type varies with actual call/fallback. No new playable rescue vehicle node. |
| `loc.daylight_report` | Small daylight document vignette, conditional full/partial truth coda. | Recovered sketch and report; no new speaking investigator. | Reusable report insert; grief context retained in partial version. |
| `loc.aurora_exterior` | Distant station buried by snow, spring ridge-door evidence; `ending_lost`. | Intake/exhaust buried, dead window, four name marks. | Pull-away then evidence dissolve; no season simulation or full spring level. |

Door adjacency is a staging plan, not added traversal choices. Hall re-use is
essential: repeated scene visits are different situations at the same location.
Production scale must support blocking; final dimensions and camera lens values
remain open to layout rehearsal rather than arbitrary gameplay constraints.

## Props / models

All IDs are stable editorial asset IDs; each row may have carried/installed or
state variants under that identity. Canonical possession stays in `story.yaml`.

| Asset IDs | Description / required variants | Contexts |
| --- | --- | --- |
| `prop.coveralls`, `prop.gloves`, `prop.torch`, `prop.handset` | Player/cast winter gear, torch beam, live/loop handset. Glove pose does not set cold. | blackout, stores, dispatch, generator, radio, crisis, endings |
| `prop.thermal_liner`, `prop.medical_kit`, `prop.splint`, `prop.jonas_tag` | Wearable overlay, kit and fitted splint; tag retained in Jonas-loss coda. | stores, generator, dispatch, crisis, full/partial |
| `prop.copper_braid`, `prop.ceramic_mug`, `prop.spanner` | One braid; mug with intact handle and painted gull; bare spanner. No duplicates after combination. | stores, generator, radio |
| `prop.safety_line`, `prop.flare`, `prop.heat_pack` | Coil/rigged supports, one flare, battery with coil connector. Line is a capability flag, not a newly simulated rope economy. | stores, generator, dispatch, radio, crisis |
| `prop.maintenance_log` | Terminal record and carried folded sketch; report insert reuses provenance. | stores, generator knowledge, ending_full/partial truth |
| `prop.stores_shutter`, `prop.annex_shutter`, `prop.radio_door` | Separate physical IDs; open/closing/sealed state, coil coupling where applicable. | stores, generator, radio; dispatch status views |
| `prop.generator`, `prop.generator_latch`, `prop.latch_ice` | Housing/cover/flywheel, stable hinge progress variants (<4, 4–5, ≥6). Do not reset cumulative progress. | generator; dispatch background |
| `prop.generator_diagram`, `prop.deicer_fuse`, `prop.primer_tube` | External readable sketch/tagged isolator; transparent primer/fuel and starter. Known/unknown is narrative access, not camera proximity. | generator |
| `prop.insulated_lever` | Spanner + braid assembly, carries resource trade-off. | generator, later carried silhouette if useful |
| `prop.generator_coil`, `prop.duct_snow` | Pressure gauge/dead digital readout and auxiliary socket; warming duct with loose snow. | generator, dispatch heat view |
| `prop.radio_transmitter`, `prop.radio_reserve`, `prop.antenna_wheel`, `prop.radio_jumper` | Battery readout/protector, dish/handwheel, mug+braid isolated bracket assembly. Keep mug gull visible on installed variant. | radio, crisis shelter insert |
| `prop.ridge_markers`, `prop.guy_wire`, `prop.intake` | Spatially coherent crossing markers; broken exhaust wire; intake progressively buried. | crisis, ending_lost |
| `prop.manifest`, `prop.ridge_names` | Four-name record/empty place; four name marks found in spring. Distinct from maintenance signature. | endings |
| `prop.rescue_vehicle` | Minimal helicopter cabin/exterior suggestion or coastal patrol dressing. No pilot gameplay or vehicle simulation. | ending_full/partial |

## Performance / animation clips

Animation IDs identify reusable performance intentions, not required rig APIs.
Avoid making different failure/success branches indistinguishable.

| Asset IDs | Performance / conditions | Contexts |
| --- | --- | --- |
| `anim.equip`, `anim.pickup`, `anim.splint` | Pull liner, select single prop, protect ankle while securing splint; no healed walk. | stores |
| `anim.jonas_cold`, `anim.jonas_supported` | Seated shiver / protected low brace or supported rest; track treatment and order. | blackout, stores, dispatch, generator, crisis |
| `anim.latch_work`, `anim.grip_fail` | Repeated same-joint effort versus hand slipping with no progress. | generator; grip failure reused radio |
| `anim.prime` | Hold primer; fuel appears, no ignition before successful branch. | generator |
| `anim.warm_hands` | Cold glove opens in air stream; no injury reset. | dispatch, radio |
| `anim.align_dish`, `anim.catch_wheel`, `anim.hold_wheel`, `anim.release_wheel` | Adjustment, gust catch, sustained shoulder hold, visible freed hands only when permitted. | radio, crisis, endings |
| `anim.stop_plan`, `anim.rig_line` | Mara stops invalid coordinated plan; accepted plan rigs real equipment. | crisis |
| `anim.support_carry`, `anim.support_step`, `anim.reach_lea` | Two-person injured carry, conditional supported crossing, reach mast. | blackout/stores transfer, dispatch/annex, crisis, full ending |
| `anim.unanswered_call`, `anim.name_response` | No answer on casualty branch; four small responses on successful manifest. | crisis, endings |
| `anim.fire_flare` | Fire only explicit flare retreat; consume once via rules. | crisis |

Additional unnumbered facial acting (recognition, pain, relief, grief) uses these
actors' expression variants; it creates no relationship-stat system. Camera can
carry reactions before expensive facial production. Low-tech prop motion may
serve a first pass; skeletal/face assets are planned, not supplied here.

## Sound / music / dialogue

| Asset IDs | Sound intention / continuity | Contexts |
| --- | --- | --- |
| `snd.station_wind`, `snd.ridge_wind` | Sheltered/exposed storm beds with continuous phase/level transitions where feasible. | blackout, stores, dispatch, generator, radio / crisis, endings |
| `snd.test_loop`, `snd.test_loop_decay` | Exactly repeated broken call/breath, later fading battery; not a new haunted voice. | blackout, optional distant motif; ending_lost |
| `snd.shutter_coil`, `snd.shutter_close`, `snd.protector_trip` | Coil bed, closure, distinct radio protector relay. Never a success acknowledgement. | stores, generator, radio |
| `snd.cloth`, `snd.kit`, `snd.tools`, `snd.connector`, `snd.terminal` | Close pickup/treatment/plug/read Foley, subordinate to dialogue. | stores, generator |
| `snd.ice_crack`, `snd.starter_turn`, `snd.motor_run` | Progress-resolved ice, starter turns, sustained powered engine; silence on failed start. | generator, warm dispatch/radio, full outcome |
| `snd.radio_static`, `snd.jumper_clamp`, `snd.beacon_lock`, `snd.real_reply`, `snd.last_bearing` | Searching radio; assembly; genuine alignment cue; live acknowledgement; maintained manual bearing. | radio, crisis, partial coda |
| `snd.mast_gust`, `snd.wire_strain`, `snd.names_in_wind`, `snd.vent_choke` | Scheduled gust, broken wire, unanswered names, fatal loss of safe shelter. | radio, crisis, ending_lost |
| `snd.flare_fire`, `snd.helicopter`, `snd.coastal_patrol` | Explicit fired flare and correct rescue vehicle bed; don't imply a call never made. | crisis, ending_full/partial |
| `voice.alex`, `voice.mara`, `voice.jonas`, `voice.lea`, `voice.coast` | Cast dialogue; [C] quotations versus [A] adapted lines marked per beat. Subtitles first; voice timing needs review. | As cast table above |
| `music.ridge`, `music.release` | Optional restrained tension/release [A]; not canonical music requirements, never mask instructions. | crisis / successful ending only |

No audio is generated in this PR. Existing `sound.wind`, `sound.motor`,
`sound.alarm` are prototype implementations for corresponding wind/engine/beep
intentions. They are not production dialogue or a complete effects library.

## Visual effects / lighting

| Asset IDs | Intended readable change | Contexts |
| --- | --- | --- |
| `fx.light_failure` | Sequential ceiling failure, then torch readability. | blackout |
| `fx.restored_heat` | Warm practical light/duct air; reflects accepted power, never reverses injury history. | generator, dispatch, radio |
| `fx.mast_snow`, `fx.mast_gust` | Continuous snow versus authored 15s gust, can overlap control animation. | radio |
| `fx.ridge_drift`, `fx.crossing_erased`, `fx.intake_buried` | Marker visibility and intake burial progression tied to canonical context/deadline. | crisis, ending_lost |
| `fx.flare_arc` | Short red arc/reflection on actual flare retreat. | crisis |
| `fx.daylight`, `fx.spring_light` | Ending report / spring evidence lighting, no new explorable scenes. | full/partial truth / lost |

## Camera setups

All setups are fixed or short authored moves within the existing mostly fixed
camera direction. No roaming-camera system or universal shot dispatcher is
specified. Final lens/FOV and move curves require layout testing.

| Setup IDs | Coverage | Contexts |
| --- | --- | --- |
| `cam.radio_insert`, `cam.injury_insert` | Handset repeating breath, ankle/splint; return to ensemble. | blackout, stores |
| `cam.stores_tray`, `cam.log_insert`, `cam.tool_insert` | One braid between mug/spanner; restricted clue reveal; assembly insert. | stores, generator |
| `cam.hall_routes`, `cam.line_insert` | Repeated hall axis and actual safety-line preparation. | dispatch, crisis |
| `cam.coil_insert`, `cam.threshold` | Moving gauge/dark display; accepted transition/closure bridge. | generator; thresholds reused stores/radio |
| `cam.radio_axis`, `cam.jumper_insert` | Transmitter through mast doorway; ceramic isolation detail. | radio |
| `cam.ridge_two_targets`, `cam.ridge_targets`, `cam.shelter_loss`, `cam.coastal_track` | Establish separated targets, branch-dependent reach/carry reverses, shelter obscured, flare route. | crisis |
| `cam.manifest_four`, `cam.manifest_empty`, `cam.tag_insert`, `cam.empty_handset` | Match opening count versus exact named absence. | full/partial |
| `cam.report_insert`, `cam.station_lost` | Conditional evidence; exterior pull-away and four names. | endings |
| `shot.wide`, `shot.controls`, `shot.starter`, `shot.hall` | Existing concrete prototype views; re-layout for canonical sets without changing their intent. | generator/dispatch reference slice |

## Cueability / implementation gap map

| Category | Current evidence | Production gap; no code in this PR |
| --- | --- | --- |
| `camera.closeup`, `camera.track`, `camera.hold` | Concrete `shot.*` transforms in Bevy slice. | More sets, authored lenses/framing, collision-free layout and actual shot bindings. |
| `actor.move`, `actor.gesture`, `actor.expression` | `move.jonas`, `move.mara` geometric roots; lever rotations. | Rigged actors, injured variants, carry/hand contacts and face direction. No NPC planner needed. |
| `prop.animate` | `move.deicer`, `move.starter`, `move.shutter`, `motor.spin`. | Progress variants, inventory attachments, gauge/readout and installed jumper continuity. |
| `environment.light`, `environment.weather` | Snow sprites, state lamp; fixed set. | Set lighting, torch visibility, storm/marker readability; not a global weather simulation. |
| `audio.ambient`, `audio.sfx` | Three WAVs, overlapping cue-owned audio instances, REPLACE cleanup. | Production recordings/loop seams, mixing, spatial intent, device listening pass (prototype had ALSA errors). |
| `dialogue.play` | Text panels / radio captions. | Casting, subtitle/voice alignment, speaker masking and overlap policy. Current narration duration is not proof of real VO completion. |
| `music.texture` | No production score. | Optional composer decision; omit until it earns its place under speech. |
| `ui.offer` | Keyboard choices, locks, countdown, current conditions. | Point-and-click affordance layout with the same actions; full cold/cutoff rules must first be preserved by the game host. |

A transition is an ordinary graph node with presentation and a later traversal
cue when needed, not a protocol category. A door-travel bridge must preserve
canonical zero work cost and destination timer semantics. Concrete mapping of
all full-story rules to Bevy remains subsequent implementation, not accomplished
by this inventory or a new generic framework.
