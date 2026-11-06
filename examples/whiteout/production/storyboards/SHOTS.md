# Complete shot index

Stable editorial shot IDs, not new GRIMM nodes/edges or camera protocol.
Each linked PNG is an actual drawn image; SVG is editable original. Order within
a scene is a browsing order, **not** a claim all alternatives play linearly.
Follow BRANCHES for the actual interactive sequence.

| Shot / image | Scene / location | Camera | Beat | Clock / duration |
| --- | --- | --- | --- | --- |
| [B01](panels/b01.png) — Ceiling lights / establish four | `blackout` / `loc.aurora_corridor` | `cam.corridor_establish` | B01 | Within arrival 12000ms |
| [B02](panels/b02.png) — Crushed ankle / protected space | `blackout` / `loc.aurora_corridor` | `cam.injury_insert` | B01 | Opening sub-beat, no extra duration |
| [B03](panels/b03.png) — Same broken breath twice | `blackout` / `loc.aurora_corridor` | `cam.radio_insert` | B01 | Opening sub-beat |
| [B04](panels/b04.png) — Four-person choice / hold | `blackout` / `loc.aurora_corridor` | `cam.corridor_establish` | B01 | After 12000ms; no deadline |
| [B05](panels/b05.png) — Carry into stores START | `blackout` / `loc.aurora_corridor` | `cam.threshold` | B01→S01 | begin cost 0 |
| [S01](panels/s01.png) — Arrival / carry END / reserve | `stores` / `loc.emergency_stores` | `cam.stores_establish` | S01 | T=0 D=180000; arrival 7000ms |
| [S02](panels/s02.png) — Preparation choices / visible reserve | `stores` / `loc.emergency_stores` | `cam.stores_establish` | S01–S04 | After briefing, deadline continues |
| [S03](panels/s03.png) — Liner over coveralls | `stores` / `loc.emergency_stores` | `cam.stores_establish` | S02 | 1000 work / 1800 feedback |
| [S04](panels/s04.png) — Splint / Mara wrist reaction | `stores` / `loc.emergency_stores` | `cam.injury_insert` | S02 | 3000 work / 1800 feedback |
| [S05](panels/s05.png) — Single copper braid pickup | `stores` / `loc.emergency_stores` | `cam.stores_tray` | S02 | 1000 / 1800 |
| [S06](panels/s06.png) — Gull mug / intact handle | `stores` / `loc.emergency_stores` | `cam.stores_tray` | S02 | 1000 / 1800 |
| [S07](panels/s07.png) — Bare spanner / later trade | `stores` / `loc.emergency_stores` | `cam.tool_insert` | S02 | 1000 / 1800 |
| [S08](panels/s08.png) — Line and flare / one choice | `stores` / `loc.emergency_stores` | `cam.line_insert` | S02 | 1000 / 1800 |
| [S09](panels/s09.png) — Heat-pack battery pickup | `stores` / `loc.emergency_stores` | `cam.stores_establish` | S02 | 1000 / 1800 |
| [S10](panels/s10.png) — Log / signature is not sabotage | `stores` / `loc.emergency_stores` | `cam.log_insert` | S03 | 2000 / 1800 |
| [S13](panels/s13.png) — Prepared stores / leave only | `stores` / `loc.emergency_stores` | `cam.stores_establish` | S01–S04 | After accepted setup/work and current gates; remaining deadline is not restarted |
| [S11](panels/s11.png) — Leave same threshold / chosen gear | `stores` / `loc.emergency_stores` | `cam.threshold` | S04 | leave cost 0 |
| [S14](panels/s14.png) — Untreated voluntary stores exit | `stores` / `loc.emergency_stores` | `cam.threshold` | S04 | leave cost 0 |
| [S12](panels/s12.png) — Shutter / untaken gear remains | `stores` / `loc.emergency_stores` | `cam.threshold` | S04 | At D or work cutoff; 1800 timeout |
| [D01](panels/d01.png) — Two routes / fixed hall | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D01 | first visit briefing 1800; no deadline |
| [D02](panels/d02.png) — Same hall / heat returns | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D02 | eligible entry 1800ms |
| [D03](panels/d03.png) — Radio-first cost / line reaction | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D02 | eligible entry 1800ms |
| [D04](panels/d04.png) — Hands at real warm duct | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D03 | 4000 work / 1800 feedback |
| [D05](panels/d05.png) — Return briefing / remaining jobs | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D02 | 1800 repeat brief; no deadline |
| [D07](panels/d07.png) — Radio resolved / engineering still open | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D01 | After eligible return briefing; no decision deadline |
| [D08](panels/d08.png) — Both resolved / warm hands or ridge | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D01 | After eligible return briefing; no decision deadline |
| [D09](panels/d09.png) — Both lost / ridge only | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D01 | After eligible return briefing; no decision deadline |
| [D10](panels/d10.png) — Power first / radio or warm hands | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D01 | After eligible return briefing; no decision deadline |
| [D11](panels/d11.png) — Engineering lost / radio still open | `dispatch` / `loc.dispatch_hall` | `cam.hall_routes` | D01 | After eligible return briefing; no decision deadline |
| [D06](panels/d06.png) — Ridge commitment / threshold | `dispatch` / `loc.dispatch_hall` | `cam.threshold` | D03→C01 | ridge cost 0 |
| [G01](panels/g01.png) — Arrival / hidden coil | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01 | T=0; arrival 4000ms; hidden D=30000 |
| [G02](panels/g02.png) — Supported Jonas / Mara reaction | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01/G03 | During arrival / accepted latch work |
| [G03](panels/g03.png) — Unassisted / untreated rest | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01/G03 | Same spatial mark as G02 |
| [G04](panels/g04.png) — External diagram / knowledge insert | `generator` / `loc.generator_annex` | `shot.controls` | G05 | inspect_controls cost 1000; feedback 1800ms |
| [G05](panels/g05.png) — Heat pack / another minute | `generator` / `loc.generator_annex` | `cam.coil_insert` | G02 | deploy_heat_pack 1000; extend +60000; feedback 1800 |
| [G06](panels/g06.png) — Same hinge / first crack | `generator` / `loc.generator_annex` | `cam.tool_insert` | G03 | S+effective 3000 or 5000 cost; media 3500 |
| [G07](panels/g07.png) — Same hinge / nearly open | `generator` / `loc.generator_annex` | `cam.tool_insert` | G03 | Media 3500; release no earlier than logical cost |
| [G08](panels/g08.png) — Access open / progressive payoff | `generator` / `loc.generator_annex` | `cam.tool_insert` | G03/G04 | Media 3500 on work / 1800 on lever |
| [G09](panels/g09.png) — Numb attempt / pin does not move | `generator` / `loc.generator_annex` | `cam.tool_insert` | G03 | 3000/5000 work; failure feedback 1800 |
| [G10](panels/g10.png) — Make insulated lever / resource trade | `generator` / `loc.generator_annex` | `cam.tool_insert` | G04 | make_lever cost 2000; feedback 1800 |
| [G27](panels/g27.png) — Crafted lever / use offer | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01–G08 | After arrival/cost/gates; deadline continues |
| [G11](panels/g11.png) — Lever fails through cold grip | `generator` / `loc.generator_annex` | `cam.tool_insert` | G04 | use_lever 3000; feedback 1800 |
| [G12](panels/g12.png) — Lever succeeds / isolated alternate | `generator` / `loc.generator_annex` | `cam.tool_insert` | G04 | use_lever 3000; feedback 1800 |
| [G13](panels/g13.png) — Pull external tagged fuse | `generator` / `loc.generator_annex` | `shot.controls` | G05 | isolate_deicer cost 2000; feedback 1800 |
| [G14](panels/g14.png) — Unsafe start / overload reaction | `generator` / `loc.generator_annex` | `shot.starter` | G07 | start_generator cost 4000; feedback 1800 |
| [G15](panels/g15.png) — Cold primer / no false success | `generator` / `loc.generator_annex` | `shot.starter` | G07 | start_generator 4000; feedback 1800 |
| [G16](panels/g16.png) — Radio overlays latch / countdown reveal | `generator` / `loc.generator_annex` | `cam.generator_establish` | G03/G06 | T=10000; radio 2200ms; S=8000 latch cost ends T=11000 |
| [G17](panels/g17.png) — Decision / conditional offers | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01–G08 | After arrival/cost/gates; deadline continues |
| [G29](panels/g29.png) — Starter offered BEFORE isolation | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01–G08 | After warm assisted latch work; current timer, no isolation knowledge from cover |
| [G25](panels/g25.png) — Open isolated engine / actual start offer | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01–G08 | After accepted setup/work and current gates; remaining deadline is not restarted |
| [G28](panels/g28.png) — Cold work mode / five-second offer | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01–G08 | After arrival/cost/gates; deadline continues |
| [G18](panels/g18.png) — Successful primer / locked sequence | `generator` / `loc.generator_annex` | `shot.starter` | G07 | start cost 4000 resolved at selection; prime media 1200 |
| [G19](panels/g19.png) — Ignition / two turns catch | `generator` / `loc.generator_annex` | `shot.starter` | G07 | After primer; ignition media 1500 |
| [G20](panels/g20.png) — Heat / return to human need | `generator` / `loc.generator_annex` | `cam.generator_establish` | G07 | After ignition; heat media 1800; then gated offers |
| [G26](panels/g26.png) — Powered / return offer after startup | `generator` / `loc.generator_annex` | `cam.generator_establish` | G01–G08 | After success/completion gate; deadline removed |
| [G21](panels/g21.png) — Warm return / doorway keyframe | `generator` / `loc.generator_annex` | `cam.threshold` | G08 | return cost 0; ordinary outgoing staging proposal |
| [G22](panels/g22.png) — Abandon silent engine | `generator` / `loc.generator_annex` | `cam.threshold` | G08 | abandon cost 0 |
| [G23](panels/g23.png) — Timeout / pull clear START | `generator` / `loc.generator_annex` | `cam.threshold` | G08 | D=30000 or extended D; timeout feedback 1800 |
| [G24](panels/g24.png) — Sealed access END / same doorway | `generator` / `loc.generator_annex` | `cam.threshold` | G08 | End of outgoing timeout beat; no new timer |
| [R01](panels/r01.png) — Connected cabin / mast establish | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01 | T=0 D=60000; arrival 6000 |
| [R02](panels/r02.png) — Reserve / conditional alternatives | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | after briefing; D continues |
| [R18](panels/r18.png) — Radio first / no heater offer | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | Entry T=6, D=60000, 54 seconds remain |
| [R21](panels/r21.png) — Lever spent copper / manual route | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | after briefing; D continues |
| [R03](panels/r03.png) — Thread copper through ceramic START | `radio` / `loc.radio_cabin` | `cam.jumper_insert` | R02 | make_adapter 3000; feedback 1800 |
| [R04](panels/r04.png) — Clamp isolated assembly END | `radio` / `loc.radio_cabin` | `cam.jumper_insert` | R02 | same accepted work, not another cost |
| [R05](panels/r05.png) — Hand-align / shoulder duty | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R03 | 6000 work / 1800 |
| [R14](panels/r14.png) — Setup ready / transmit offer | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | After accepted setup/work and current gates; remaining deadline is not restarted |
| [R19](panels/r19.png) — Unpowered setup / transmit or abandon | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | After setup; current reserve, no reset |
| [R06](panels/r06.png) — Cold alignment fails / caught swing | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R03 | 6000 / 1800 |
| [R07](panels/r07.png) — Gust overlays ongoing work | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R04 | T=15000; 1800ms |
| [R08](panels/r08.png) — Cold transmission / no false reply | `radio` / `loc.radio_cabin` | `cam.radio_insert` | R05 | send_signal 3000 / 1800 |
| [R09](panels/r09.png) — Automatic call / freed hands | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R05 | 3000 / 1800; deadline removed |
| [R10](panels/r10.png) — Manual call / hold persists | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R05 | 3000 / 1800; deadline removed |
| [R15](panels/r15.png) — Call acknowledged / return offer | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | After success/completion gate; deadline removed |
| [R16](panels/r16.png) — Automatic call / return UI, hands free | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | After success/completion gate; deadline removed |
| [R20](panels/r20.png) — Unpowered acknowledged call / return only | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R01–R07 | Call acknowledged; deadline removed even without generator power |
| [R11](panels/r11.png) — Heater / recover fine grip | `radio` / `loc.radio_cabin` | `cam.radio_axis` | R06 | 4000 / 1800 |
| [R12](panels/r12.png) — Return with bearing / correct Lea | `radio` / `loc.radio_cabin` | `cam.threshold` | R07 | return cost 0 |
| [R17](panels/r17.png) — Automatic return / both can leave | `radio` / `loc.radio_cabin` | `cam.threshold` | R07 | return cost 0 |
| [R13](panels/r13.png) — No call / abandon or reserve trips | `radio` / `loc.radio_cabin` | `cam.threshold` | R07 | abandon 0 or timeout D=60000; 1800 feedback |
| [C01](panels/c01.png) — Two targets / entry END | `crisis` / `loc.ridge_crossing` | `cam.ridge_two_targets` | C01 | T=0 D=45000; arrival 8000 |
| [C02](panels/c02.png) — Choice / line versus human cost | `crisis` / `loc.ridge_crossing` | `cam.ridge_two_targets` | C01–C06 | After 8000; remaining clock |
| [C03](panels/c03.png) — Imaginary plan stopped | `crisis` / `loc.ridge_crossing` | `cam.ridge_targets` | C02 | coordinate 0; feedback 1800 |
| [C04](panels/c04.png) — Coordinated rig / START | `crisis` / `loc.ridge_crossing` | `cam.line_insert` | C02 | coordinate cost 0; accepted full branch |
| [C05](panels/c05.png) — Coordinated crossing / END | `crisis` / `loc.ridge_crossing` | `cam.ridge_targets` | C02 | same accepted outcome, no extra decision |
| [C15](panels/c15.png) — Coordinated automatic route / no rope | `crisis` / `loc.ridge_crossing` | `cam.ridge_targets` | C02 | same accepted outcome, no extra decision |
| [C06](panels/c06.png) — Reach Lea / Jonas can follow | `crisis` / `loc.ridge_crossing` | `cam.ridge_targets` | C03 | help_lea cost 0 |
| [C07](panels/c07.png) — Reach Lea / Jonas left behind | `crisis` / `loc.ridge_crossing` | `cam.ridge_targets` | C03 | same source choice, alternative outcome |
| [C08](panels/c08.png) — Carry Jonas / auto Lea free | `crisis` / `loc.ridge_crossing` | `cam.ridge_targets` | C04 | help_jonas cost 0 |
| [C14](panels/c14.png) — Untreated Jonas / carried full rescue | `crisis` / `loc.ridge_crossing` | `cam.ridge_targets` | C04 | help_jonas cost 0 |
| [C09](panels/c09.png) — Carry Jonas / manual Lea lost | `crisis` / `loc.ridge_crossing` | `cam.shelter_loss` | C04 | same source choice, alternative outcome |
| [C10](panels/c10.png) — Explicit flare / coastal retreat | `crisis` / `loc.ridge_crossing` | `cam.coastal_track` | C05 | flare_retreat cost 0 |
| [C11](panels/c11.png) — Stay / false shelter | `crisis` / `loc.dispatch_hall` | `cam.ridge_targets` | C05 | stay cost 0 |
| [C12](panels/c12.png) — Marked crossing erased START | `crisis` / `loc.ridge_crossing` | `cam.ridge_two_targets` | C06 | D=45000 timeout; feedback 1800 |
| [C13](panels/c13.png) — No crossing END | `crisis` / `loc.ridge_crossing` | `cam.ridge_two_targets` | C06 | end of timeout beat |
| [E01](panels/e01.png) — Four names / occupied seats | `ending_full` / `loc.rescue_cabin` | `cam.manifest_four` | E01 | arrival 1800; no choices |
| [E11](panels/e11.png) — Four names / occupied seats / untreated Jonas | `ending_full` / `loc.rescue_cabin` | `cam.manifest_four` | E01 | arrival 1800; no choices |
| [E02](panels/e02.png) — Truth / optional daylight | `ending_full` / `loc.daylight_report` | `cam.report_insert` | E02 | truth 1800 if log_read |
| [E03](panels/e03.png) — Empty Jonas place | `ending_partial` / `loc.rescue_cabin` | `cam.manifest_empty` | E03 | arrival 1800; terminal |
| [E04](panels/e04.png) — Mara glove / Jonas tag | `ending_partial` / `loc.rescue_cabin` | `cam.tag_insert` | E04 | conditional coda 1800 |
| [E05](panels/e05.png) — Empty Lea place | `ending_partial` / `loc.rescue_cabin` | `cam.manifest_empty` | E03 | arrival 1800; terminal |
| [E12](panels/e12.png) — Empty Lea place / untreated Jonas | `ending_partial` / `loc.rescue_cabin` | `cam.manifest_empty` | E03 | arrival 1800; terminal |
| [E06](panels/e06.png) — Handset / missing operator | `ending_partial` / `loc.rescue_cabin` | `cam.empty_handset` | E04 | conditional coda 1800 |
| [E07](panels/e07.png) — Truth cannot undo loss | `ending_partial` / `loc.daylight_report` | `cam.report_insert` | E04 | 1800 only if log_read |
| [E10](panels/e10.png) — Lost station / pull-away START | `ending_lost` / `loc.aurora_exterior` | `cam.station_lost` | E05 | arrival 1800; terminal |
| [E08](panels/e08.png) — Station lost / pull-away END | `ending_lost` / `loc.aurora_exterior` | `cam.station_lost` | E05 | arrival 1800; terminal |
| [E09](panels/e09.png) — Spring / four names without faces | `ending_lost` / `loc.aurora_exterior` | `cam.station_lost` | E05 | same coarse ending beat, no new playable season |
