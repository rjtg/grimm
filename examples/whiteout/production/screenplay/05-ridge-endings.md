# Act III — The names that answer

## `crisis` — INT. RIDGE DOOR / EXT. MARKED CROSSING

**Location:** `loc.ridge_crossing`, incorporating the last threshold, heater
manifold and antenna shelter sightline. **Cast:** all four, spatially separated:
Mara at Alex's shoulder, Jonas down at the manifold, Lea near the shelter.
**Objective:** make prior preparation and deliberate priorities visible as a
rescue plan. **45000ms visible from T=0**; briefing consumes 8000ms. Storm is
scene-local authored pressure, not an invented global simulation.

<a id="c01"></a>
### C01 — One quickly; two with preparation

**Story [C] — TEXT `crisis.arrival`.** Exhaust stack broke a guy wire; drifting
snow will bury the crossing, then intake vents. Remaining inside is unsafe even
with generation. Mara can help carry one quickly. Treatment, power, call and
antenna/line preparation enable coordinated rescue; radio-first weakens Jonas
so the line matters. Flare is an exposed coastal fallback.

**Performance [A].** Start behind Mara's shoulder at the door. A broken wire
whips across the frame without hitting anyone. Rack focus between Jonas low
by the duct and Lea high beside the mast. Keep both in the same geography;
a split cut cannot imply they are easily reached sequentially for free. The
crossing markers fade into snow in depth, showing where choices will commit.

MARA [C]: “We can carry one person quickly. Two need preparation.”

When the line was taken, show its real coil on Alex's pack; if consumed copper
made a lever, Lea's hands still hold the manual bearing. When radio is automatic,
show the jumper in the shelter insert and her free hands. Don't telegraph an
unearned full rescue through triumphant music before the decision.

**Cue reference [R].** `crisis / deadline`, `arrival`, `choices`;
`cam.ridge_two_targets`, `fx.ridge_drift`, `snd.wire_strain`, `snd.ridge_wind`.
Let music thin to a low unresolved texture (`music.ridge`), an optional [A]
production proposal. Sound and spatial layout must carry the scene without it.

<a id="c02"></a>
### C02 — Coordinate is a proposal, not a promise

**Story [C] — TEXT `crisis.missing_preparation`.** `crisis.coordinate` is always
an offered zero-duration attempt after input opens. Success requires all:

- `generator_done && radio_done && jonas_treated`;
- `radio_auto || safety_line`;
- `first_task == 1 || safety_line`;
- **post-accounting** `cold_exposure < 120000`.

Otherwise stay in `crisis`, show 1800ms feedback and continue the same deadline.
The failure must not become a disabled option with invented preconditions.

**Performance [A].** Alex starts to gesture towards both sides. If equipment
or capability is missing, Mara catches the wrist before anyone runs. Brief
inserts show the absent support or Alex's failing grip, without exposing raw
state counters. Return to both targets with less time, not a new 45-second shot.

MARA [A, failed plan]: “Not with what we have. Choose what we can reach.”

On success, Mara and Alex rig existing equipment, then move together. Automatic
jumper leaves Lea free; otherwise a possessed line secures the antenna. The
line also guides Jonas after radio-first. Show these as distinct uses of the
same authored capability; do not spawn spare ropes or demand an extra unmodeled
resource count. Proceed to `ending_full` only after the real outcome resolves.

**Cue reference [R].** `ACTION crisis.coordinate`, outcome 1 GOTO `ending_full`,
outcome 2 / `missing_preparation`. `anim.stop_plan`, `anim.rig_line`,
`anim.support_carry`, `cam.line_insert`. Full exact rules are in RULES.

<a id="c03"></a>
### C03 — Go to Lea

**Story [C].** `crisis.help_lea` is always offered, duration 0. Ordered results:

1. Call + power + treatment + **generator first** + cold <120000 → full rescue.
   Neither line nor automatic radio is required by this particular route.
2. Otherwise, call **or possessed flare** → partial survival, `jonas_alive=false`.
3. Otherwise → all four dead, `ending_lost`.

**Performance [A].** Alex runs towards Lea; Mara turns to Jonas. On the full
branch, treated Jonas can move with Mara's support after early heat, while Alex
hand-guides the last bearing and frees Lea. Use supported steps, not a suddenly
healed ankle. On the partial branch Jonas cannot cross; camera stays on Mara
looking back, then the gap where a reply should be. Do not animate his rescue
because the player happened to carry a line: that would execute a different
choice's coordinated plan. Radio-first keeps this cost even after warming hands.

**Cue reference [R].** `crisis.help_lea` outcomes 1/2/3 → full/partial/lost;
`actor.move` → `anim.reach_lea`, `anim.support_step` or `anim.unanswered_call`;
`cam.ridge_targets`. No additional confirmation dialog buys time or changes result.

<a id="c04"></a>
### C04 — Carry Jonas

**Story [C].** `crisis.help_jonas` is always offered, duration 0. Ordered results:

1. Call + **automatic** radio + power + cold <120000 → full rescue. Treatment and
   repair order are **not** extra conditions on this route.
2. Otherwise, call or possessed flare → partial, `lea_alive=false`.
3. Otherwise → all dead, lost.

**Performance [A].** Alex and Mara take Jonas's weight at the manifold. If Lea
has the automatic jumper, she leaves the shelter with free hands. If this is the
partial manual branch, hold on her still-braced hands while the carry clears the
frame. She sends the bearing; snow obscures the shelter before Alex can return.
Do not let possessing a line silently rescue her: the chosen action is carrying
Jonas, not coordinating both. Do not show both named casualties for one branch.

**Cue reference [R].** `crisis.help_jonas` outcomes 1/2/3;
`anim.support_carry`, `anim.hold_wheel`, `snd.last_bearing`, `cam.shelter_loss`.
The final line is the existing bearing, not a new mystery message.

<a id="c05"></a>
### C05 — Flare or false shelter

**Story [C].** `crisis.flare_retreat` requires a flare, consumes it, kills Jonas
and reaches partial survival regardless of other preparation. It is not secretly
a fourth full-rescue route. `crisis.stay` is always offered and kills all four.

**Performance [A].** For explicit flare retreat, Alex fires one red arc towards
the coastal track; Mara helps the moving survivors into its brief reflected
light. Hold the untraversable stretch behind them instead of an action-movie
escape carrying everyone. For staying, pull back behind the closing door; the
warm vent coughs as intake snow thickens. A lit generator panel may persist
briefly, emphasizing that repaired equipment is not breathable shelter.

**Cue reference [R].** `crisis.flare_retreat` → `ending_partial`;
`crisis.stay` → `ending_lost`; `prop.flare`, `fx.flare_arc`, `snd.flare_fire`,
`anim.fire_flare`, `cam.coastal_track`, `snd.vent_choke`.

<a id="c06"></a>
### C06 — The crossing disappears

**Story [C] — TEXT `crisis.timeout`.** At the 45000ms boundary without a timely
choice, all survivor flags become false and destination is `ending_lost`.

**Performance [A].** Snow successively erases the same crossing markers seen at
entry. Mara calls names into the wind. Leave space for answers; none resolves
into a new survivor. No last-second inventory animation overrides timeout.
A short 1800ms outgoing beat gives way to the lost ending's silence.

**Cue reference [R].** `crisis / timeout / timeout` then `ending_lost`;
`environment.weather` → `fx.crossing_erased`; `audio.sfx` → `snd.names_in_wind`.
Use existing deadline-first host arbitration for selection at exactly T=45000.

## Terminal scenes — outcomes are not repair rooms

Ending IDs are canonical narrative contexts; they refer to three staging
locations, not selectable return routes. There are no actions or countdowns.
The native 1800ms cards are subtitle/visual beats; a full voiced cinematic is a
later editorial timing decision, not permission to alter work deadlines.

<a id="e01"></a>
### `ending_full` — INT. RESCUE HELICOPTER / FOUR NAMES ON THE MANIFEST

**Location:** `loc.rescue_cabin`. **Cast:** Alex, Mara, Jonas, Lea; unnamed coast
voice may remain offscreen. **Story [C] — TEXT `ending_full.arrival`.** All four
survive. Mara reads Alex, Mara, Jonas, Lea into the helicopter mic; four answers.
Station becomes a warm square behind the snow; no empty seat.

**Performance [A].** Match the opening's four-face torch sweep with four faces
in cabin practical light. Jonas's splint is still present. Lea's hands shake but
hold nothing to keep others alive. Mara taps each name on the manifest; four
human acknowledgements replace the repeated automated breath. End on a wide
with four occupied positions; do not require four long spoken answers inside
1.8s. Use visual nods/subtitles if keeping the native duration.

**Cue reference [R].** `ending_full / arrival`; `cam.manifest_four`,
`anim.name_response`, `snd.helicopter`, optional `music.release` only after outcome.

<a id="e02"></a>
### Full rescue coda — EXT./INT. DAYLIGHT REPORT

**Story [C] — TEXT `ending_full.truth`, only `log_read`.** In daylight, the log
clears Jonas of sabotage. Automatic de-icer and test-call relay failed together;
his bypass was an understandable mistake, not malice.

**Performance [A].** Cut from the manifest to the recovered sketch beside a
maintenance report. Jonas's signature remains visible without a red accusation
mark. A gloved hand sets the paper flat. Daylight replaces emergency blue.
No log means no explanatory report shot: equipment repair alone does not reveal
all the mystery. Do not invent an investigating officer or trial.

**Cue reference [R].** `ending_full / truth` (1800ms); `loc.daylight_report`,
`prop.maintenance_log`, `cam.report_insert`, `environment.light` → `fx.daylight`.

<a id="e03"></a>
### `ending_partial` — INT. RESCUE CABIN / THE EMPTY PLACE

**Location:** `loc.rescue_cabin` with helicopter or coastal-patrol variant.
**Story [C] — TEXT `ending_partial.arrival`.** Help follows the call or flare,
but the manifest is incomplete. It is an authored terminal outcome, not a
nonterminal puzzle deadlock.

**Performance [A].** Reuse the full ending's seating axis, leaving the appropriate
place empty. Use rotor noise when an acknowledged call supports that staging;
for coastal fallback, use a low patrol engine and a lamp through blowing snow,
not a helicopter invented by the screenplay. Exact pickup geography is [A],
not another escape action. Never show a dead actor seated alive to save an asset.

**Cue reference [R].** `ending_partial / arrival` (1800ms);
`cam.manifest_empty`, `snd.helicopter` or `snd.coastal_patrol`, `prop.manifest`.

<a id="e04"></a>
### Partial codas — whose voice is absent

**Story [C] — TEXT `ending_partial.jonas` if `!jonas_alive`:** Jonas never reaches
the crossing. Mara keeps his medical tag; Lea stops asking about another minute.
**Performance [A].** Close on the tag held inside Mara's glove, then Lea leaving
an unfinished sentence. Do not show a surviving Jonas in the background.

**Story [C] — TEXT `ending_partial.lea` if `!lea_alive`:** Lea's last transmission
was the bearing; antenna shelter is gone on return. Jonas lives because he was
carried. **Performance [A].** Jonas keeps staring at an empty handset cradle;
the final carrier tone fades rather than repeating the old test call as a twist.
Mara does not hand over Jonas's death tag on this branch.

**Story [C] — TEXT `ending_partial.truth` if `log_read`:** Log explains what
failed but cannot restore the person lost. **Performance [A].** The same report
insert as full truth, with the empty place still present in an adjoining frame.
Do not make clearing Jonas a rescue or require him to be alive for this coda.

**Cue reference [R].** `ending_partial / jonas`, `lea`, `truth`, each 1800ms,
ordered after arrival as eligible; `prop.jonas_tag`, `cam.tag_insert`,
`cam.empty_handset`, `cam.report_insert`. These are conditional beats, not new nodes.

<a id="e05"></a>
### `ending_lost` — EXT. AURORA / WHITEOUT

**Location:** `loc.aurora_exterior`. **Story [C] — TEXT `ending_lost.arrival`.**
Drift chokes the exhaust; even repaired generation cannot keep the last room
warm. Broken call repeats until its battery dies. In spring, searchers find
four names by the ridge door. All four cast members are dead; no survivor reveal.

**Performance [A].** Pull away from the last window until the building is only
a low white contour. The broken breath loop loses power, leaving exterior wind.
For the spring image, dissolve to the ridge-door name marks in pale thaw light;
searchers stay unnamed, offscreen. Match the opening count, now without faces.
The final card is a compressed ending image, not a simulated season or additional
playable location. No music can suggest an unresolved rescue branch.

**Cue reference [R].** `ending_lost / arrival` (1800ms); `cam.station_lost`,
`fx.intake_buried`, `snd.test_loop_decay`, `prop.ridge_names`, `fx.spring_light`.
The short native duration supports a still/dissolve with text; fully voiced
production would require a separately reviewed terminal presentation length.
