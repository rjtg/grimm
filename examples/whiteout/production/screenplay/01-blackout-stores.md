# Act I — Four people, one station

## `blackout` — INT. AURORA CORRIDOR / 03:17

**Location:** `loc.aurora_corridor`. **Cast:** `actor.alex`, `actor.mara`,
`actor.jonas`, `actor.lea`. Establish the whole rescue problem before separating
its repairs. No countdown; `arrival` lasts 12000ms, then `choices` opens.
Alex is the player engineer; keep their gender and appearance open.

<a id="b01"></a>
### B01 — The same breath twice

**Story [C] — TEXT `blackout.arrival`.** Lights die along the laboratory/sleeping
corridor. Jonas has a crushed ankle. Four people are present. An unrecognized
broken distress call repeats with identical breath spacing. The intake is
freezing; antenna motors have no power; the ridge will close to a helicopter.

**Performance [A].** Begin wide along the ceiling strip. Extinguish its segments
in order until Alex is a silhouette. Wind fills the brief black frame. Mara's
torch cuts across four faces, then lowers to Jonas's ankle; she kneels without
moving that leg. Lea holds the handset close to her ear, thumb away from SEND.
Alex follows the strip of dark antenna indicators towards engineering.

Within the existing twelve seconds, prioritize visible beats over extra speech:
0–3s dying light/count; 3–7s injured ankle and identical clipped radio burst;
7–12s intake grille icing, dark motor indicators, corridor axis towards stores.
Use sound overlap instead of four consecutive explanatory close-ups.

MARA [A]: “Four. Stay together.”

LEA [A]: “That was the same breath. Did you send it?”

Alex shakes their head. No accusatory cut to Jonas. His culpability is not yet
known; a reaction must not manufacture a sabotage clue.

MARA [A]: “Supplies first. Then choose which repair.”

Return to the four-person composition. After the input gate, hold a breathing
ensemble loop, not an exit animation. The choice is the player's commitment.

**Cue reference [R].** `blackout / arrival`, then `blackout / choices`.
`environment.light` → `fx.light_failure`; `audio.sfx` → `snd.test_loop`;
`camera.closeup` → `cam.radio_insert`, `cam.injury_insert`;
`dialogue.play` uses the three adapted lines, with subtitle timing reviewed
inside 12000ms. If voice does not fit, retain only the first line and radio sound.

| Action | Work | Decision and consequence |
| --- | --- | --- |
| `blackout.begin` | 0ms | Help Mara carry Jonas to `stores`. Exposure begins at sheltered rate 1 on arrival; no item or treatment granted. |

**Transition [A/R].** On acceptance, Alex and Mara take Jonas's weight; Lea
backs through the visible stores door with the torch/radio. Track past the door
jamb, masking the set change. This is a proposed ordinary outgoing staging node,
not a second choice and not a new gameplay travel duration.

## `stores` — INT. EMERGENCY STORES / SHUTTER RESERVE

**Location:** `loc.emergency_stores`. **Cast:** all four. Dramatic question:
what does Alex secure before the room becomes inaccessible? Deadline **180000ms
from T=0**, visible. Seven-second briefing consumes this budget. Do not stage a
separate leisurely equipment cutscene before starting the clock.

<a id="s01"></a>
### S01 — A tray with only one braid

**Story [C] — TEXT `stores.arrival`.** Mara braces Jonas by the shelves. Medical
kit and liner are nearest, tools in a spilled tray, evacuation gear on a rack,
maintenance terminal blinking. There is one braid, usable in two competing
assemblies. Jonas knows the lever; Lea knows ceramic insulation.

**Performance [A].** Establish a wide triangle: Jonas seated at a low shelf,
Mara supporting him; Alex between the tray and evacuation rack; Lea at the
handset. Keep the closing shutter motor in the ceiling edge of frame. A glance
up and a visible 3:00 readout establish the threat without a fresh alarm.
Push into the single braid between bare spanner and gull-painted mug. Their
composition teaches the choice without spawning a second braid for decoration.

JONAS [A]: “Wrap the spanner. Then steel won't stick to your hands.”

LEA [A, over handset if outside the composition]: “Or thread it through my mug.
Ceramic keeps the jumper off the bracket. We only have that one.”

MARA [C]: “We have three minutes before the storm shutter seals this room.
Choose deliberately.”

Play these as overlapping short exchanges/subtitles within seven seconds, with
the final choice framing on the three equipment groups. A full unabridged voiced
performance needs pacing review; it does not authorize a longer reserve.

**Cue reference [R].** `stores / deadline`, `arrival`, `choices`;
`camera.track` → `cam.stores_tray`; `audio.ambient` → `snd.shutter_coil`;
`ui.offer` binds only eligible equipment, not prop colliders inventing actions.

<a id="s02"></a>
### S02 — Preparing people, not a shopping montage

**Story [C].** Each selected preparation has its own effect and short feedback.
No automatic “take all”; no possession before the accepted action. Taking
supplies is repeat-limited by the existing guard, not by a new inventory UI.

**Performance [A].** Keep the ensemble wide between inserts. For the liner,
Alex pulls it over their coveralls; subsequent breath/hand movement is steadier,
not instant recovery. For treatment, hold on splint straps and Mara's hand
supporting the ankle; Jonas tests his grip on the shelf, never stands healed.
Mara squeezes Alex's wrist once. Other pickups use a gloved-hand insert then
return to the tray; remove only the actual taken object. A line/flare bundle is
one selection, not two separate actions.

JONAS [C, treated]: “I can brace the generator frame.”

LEA [A, mug]: “Keep the gull facing out. I'll want that back.”

| Action / feedback TEXT | Work | Gate and result |
| --- | --- | --- |
| `stores.take_thermal_liner` / `stores.thermal_liner` | 1000ms | If not worn; set liner, sheltered current/indoor exposure rates to 0 **after** charging old-rate work. |
| `stores.jonas_treated` / `stores.jonas_treated` | 3000ms | If untreated; splint/medication, treated true. Helps latch and later mobility; not a cure. |
| `stores.take_copper_braid` / `stores.copper_braid` | 1000ms | If not possessed; obtain the single braid. |
| `stores.take_ceramic_mug` / `stores.ceramic_mug` | 1000ms | If not possessed; mug with intact ceramic handle. |
| `stores.take_spanner` / `stores.spanner` | 1000ms | If not possessed; bare adjustable spanner. |
| `stores.take_safety_line` / `stores.safety_line` | 1000ms | If no line; obtain both line and flare. This is the only flare pickup. |
| `stores.take_heat_pack` / `stores.heat_pack` | 1000ms | If not possessed; one spare battery, not a generator repair. |

All feedback captions have 1800ms authored duration. Work and caption lengths
are separate; animate within the existing release/completion rules, never charge
both. `actor.gesture` → `anim.equip`, `anim.splint`, `anim.pickup`;
`prop.animate` → selected `prop.*` inventory entry; `audio.sfx` → `snd.cloth`,
`snd.kit`, `snd.tools`. Distinguish splinting from pickup even in a placeholder pass.

<a id="s03"></a>
### S03 — The signature is not sabotage

**Story [C] — TEXT `stores.log_read`.** Yesterday's test loop is replaying through
a failed relay; a de-icer left connected during maintenance overloaded the
feeder. Jonas signed a temporary bypass. Reading sets `log_read` and
`controls_known`; it does not isolate the load.

**Performance [A].** On selection, the terminal resolves a dated TEST LOOP
entry and a wiring sketch. Close on Jonas's signature, then the overloaded
branch, not a thriller-style guilty face. Jonas watches Alex's hand stop on the
page. Alex folds the sketch into a pocket. Keep all terminal details illegible
before this action; a wide shot cannot unlock information ahead of the rules.

ALEX [A]: “A bypass. Not an order to shut us down.”

Mara looks back to Jonas's splint; no instant reconciliation speech. The
mechanical explanation is available now, its public ending payoff remains
conditional on retaining this knowledge.

**Cue reference [R].** `ACTION stores.log_read` (2000ms, `!log_read`),
outcome 1 / `log_read` (1800ms); `camera.closeup` → `cam.log_insert`;
`prop.animate` → `prop.maintenance_log`; `audio.sfx` → `snd.terminal`.

<a id="s04"></a>
### S04 — What remains behind

**Story [C] — TEXT `stores.timeout`, or deliberate `stores.leave`.** A voluntary
zero-cost departure goes to dispatch with what was taken. At 180000ms, or work
cutoff, shutter drops; Mara gets Jonas through. Left supplies are inaccessible.
There is no route back to stores.

**Performance [A].** On leave, Alex closes the tray and looks to the hall; no
screen of unchosen items magically appears in a pack. For timeout, cut along
the descending shutter to Mara hauling the splinted leg clear. End with the
untaken silhouettes behind metal. No casualty, no new injury, no forced item
loss from already carried gear. Wind briefly dulls as the door seals.

MARA [C, timeout]: “Generator or radio?”

**Cue reference [R].** `stores / timeout` narrative 1800ms + GOTO `dispatch`.
`prop.animate` → `prop.stores_shutter`; `audio.sfx` → `snd.shutter_close`;
`camera.track` → `cam.threshold`. A filmic door bridge can overlap destination
establishment only under the later host staging contract; it cannot defer or
reset the next active timer arbitrarily. See RULES for cutoff credit/accounting.
