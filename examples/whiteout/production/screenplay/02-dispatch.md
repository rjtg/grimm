# Act II — The hall between promises

## `dispatch` — INT. DISPATCH HALL / FIRST AND RETURN VISITS

**Location:** `loc.dispatch_hall`, the same reusable set on every visit.
**Cast:** Alex and Mara; Jonas rests near the duct. Lea is present/at the radio
route as appropriate, often a voice on the handset. Objective: make repair order
and accumulated consequences legible. No decision deadline. Indoor exposure is
still booked on actions: unlined, unheated waiting can matter. “No countdown”
does not imply no consequence from cold.

**Spatial continuity [A].** Engineering door frame right, radio passage left,
ridge door deep centre; Jonas's resting place beside the duct stays consistent.
Doors show repaired or sealed status through light and physical closure, not
new locks unrelated to `generator_done`/`engineering_lost`/`radio_done`/`radio_lost`.
Do not offer a trip solely because a visible door exists.

<a id="d01"></a>
### D01 — Which need comes first?

**Story [C] — TEXT `dispatch.arrival`.** On the first visit, Mara keeps Jonas
here until Alex chooses. Power first brings heat; radio first buys a bearing
while he waits cold. Treatment and the safety line can compensate later. A
sealed repair cabin cannot be reopened.

**Performance [A].** Frame Mara between the two repair doorways. Alex's torch
picks out the silent duct, then Lea's handset indicator on the other side. Give
each route equal composition; do not make one look secretly mandatory.

MARA [A]: “Heat for him, or a bearing for rescue. Which first?”

LEA [A, handset]: “I can reach the controls. I can't make them work alone.”

Jonas keeps his foot off the floor. His condition must not reset between visits.
A wall diagram lets the player see the two routes and final ridge door at once;
this is navigation staging, not a minimap framework.

**Cue reference [R].** `dispatch / arrival` only while `first_task == 0`, 1800ms;
`dispatch / choices` opens after eligible sequential entry cues. The two added
lines are alternative compressed performances, not both required within 1.8s.
`camera.hold` → `cam.hall_routes`; `audio.ambient` → `snd.station_wind`.

<a id="d02"></a>
### D02 — The same place, changed people

**Story [C] — TEXT `dispatch.heat_returns`, `dispatch.radio_first_cost`,
`dispatch.return_brief`.** Entry plays heat feedback whenever generation is done,
then radio-first cost whenever `first_task == 2 && radio_done`, then the shorter
return briefing when a repair order has been chosen. These are not one-time
flags: later eligible visits may repeat them. Heat does not erase the radio-first
mobility consequence. Do not assert a death here; survivors resolve at the ridge.

**Performance [A].** Match the first hall camera position. With power, a strip
lamp steadies and the duct stirs Jonas's sleeve. He stops shaking but continues
protecting his ankle. Without it, Mara keeps the torch between them and Jonas's
answers shorten. After radio-first, Mara checks the line attachment on Alex's
pack before trying to lift Jonas; he cannot simply walk because a warm light
has turned on.

MARA [A, return]: “That door stays closed. What can we still finish?”

A completed call is heard as a real answering voice, not the old identical
breath loop. With manual radio, Lea's braced silhouette remains associated with
the mast; do not quietly seat her comfortably in the hall. With automatic radio,
her hands can be free. Use radio contact/offscreen eyelines when a single hall
shot cannot truthfully contain both stations.

**Cue reference [R].** `heat_returns`, `radio_first_cost`, `return_brief`, each
1800ms. `environment.light` → `fx.restored_heat`; `actor.expression` →
`anim.jonas_cold` / `anim.jonas_supported`; `camera.closeup` → `cam.line_insert`.
Reuse gestures, not long repeated dialogue; existing sequential durations stay.

<a id="d03"></a>
### D03 — Recovering a grip, not undoing a decision

**Story [C] — TEXT `dispatch.warm_hands`, `dispatch.warm_again`.** Four-second
warming is offered only with power and positive exposure. After booking work,
reduce exposure by 20000 if at least that much remains; otherwise set it to 0.
It never changes repair order, treatment or anyone's survival flag.

**Performance [A].** Alex spreads fingers over the duct. Close on a glove
unclenching; the camera retains Jonas's splint beyond it, so a better hand does
not read as healing him. Under a small motor hum, crew breathing becomes audible.
No reassuring “everyone will make it” line.

**Cue reference [R].** `ACTION dispatch.warm_hands`, outcomes / `warm_hands` or
`warm_again`, each 1800ms feedback; `actor.gesture` → `anim.warm_hands`;
`audio.ambient` → `snd.motor_run`. Eligibility waits for the four-second logical
work even if the insert ends first.

| Action | Work | Availability and destination |
| --- | --- | --- |
| `dispatch.engineering` | 0ms | `!generator_done && !engineering_lost`; first route sets `first_task=1`, enters `generator`. Liner selects annex exposure 1 versus 2. |
| `dispatch.communications` | 0ms | `!radio_done && !radio_lost`; first route sets `first_task=2`, enters `radio`. Power makes later radio exposure 0; otherwise liner 1 versus 2. |
| `dispatch.warm_hands` | 4000ms | Power and positive cold; remain in hall with recovery feedback. |
| `dispatch.ridge` | 0ms | Both jobs resolved: `(generator_done || engineering_lost) && (radio_done || radio_lost)`; enters `crisis`, exposed rate 1 with liner, otherwise 2. |

**Transition [A/R].** Follow a chosen repair door on acceptance, keeping the
other visibly available only when its guard permits. On ridge commitment, Mara
secures Jonas and Alex reaches for the final door while Lea answers over radio.
A proposed ordinary transition node can mask the set swap; it must not add travel
cost, undo a seal, begin the crisis timer before entry, or introduce a second
repair-order variable. The hall remains the canonical safe planning/checkpoint
location; this screenplay adds no Save/Restore implementation.
