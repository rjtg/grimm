# Act II — Someone must keep holding on

## `radio` — INT. RADIO CABIN / EXT. MAST THRESHOLD

**Location:** `loc.radio_cabin`, including its adjacent mast platform; one
canonical context with interior controls and exterior handwheel, not two timers.
**Cast:** Alex and Lea; Mara/Jonas remain connected by handset from the hall.
**Objective:** get a real acknowledged bearing out and establish whether Lea
can leave her post. **60000ms visible reserve from T=0**, six-second briefing
inside it. No fresh sixty seconds on a camera cut to the mast.

<a id="r01"></a>
### R01 — A bearing costs somebody's hands

**Story [C] — TEXT `radio.arrival`.** Powered dish can be bypassed with copper
through the ceramic mug handle. Otherwise somebody holds alignment manually;
Lea can do it but cannot leave while the line is live. A safety line can replace
her grip later. Generator power warms hands but does not itself grant alignment,
a jumper or a completed call. The cabin reserve is an independent deadline.

**Performance [A].** Frame the transmitter and doorway to the mast on one axis.
A battery readout is already falling at entry. Beyond the window, the dish wheel
rocks; Lea's gloves catch it. On the desk, isolate the shorted bracket in the
composition and give the mug's ceramic loop enough visual scale to suggest its
purpose if Alex took it. If copper was spent on the lever, show no spare braid.
If generator is running, warm practical light is present; storm outside remains.

LEA [A]: “A jumper keeps it live. Otherwise I hold the wheel.”

She looks towards the line on Alex's pack only if possessed. Her glance can
suggest the later rescue dependency, never install the line for free.

**Cue reference [R].** `radio / deadline`, `arrival`, `choices`;
`camera.hold` → `cam.radio_axis`; `audio.ambient` → `snd.radio_static`;
`environment.weather` → `fx.mast_snow`; `prop.radio_reserve`.

<a id="r02"></a>
### R02 — The painted gull faces the snow

**Story [C] — TEXT `radio.adapter`.** Three-second assembly consumes braid and
mug, sets `radio_adapter`. The insulated jumper keeps the antenna live without
a person once a transmission succeeds; the assembly alone is not a rescue call.

**Performance [A].** A macro insert follows copper threaded through the intact
ceramic handle, clear of the metal bracket. The painted gull rotates outwards
on the mounted mug. Preserve the gull's orientation across future shots; remove
the mug/braid from carried gear, not the installed assembly. Lea tests the wheel
with one hand but does not celebrate or depart before call acknowledgement.

LEA [A]: “That will hold. Now tell them where we are.”

**Cue reference [R].** `ACTION radio.make_adapter` / `adapter` (1800ms);
`camera.closeup` → `cam.jumper_insert`; `prop.animate` → `prop.radio_jumper`;
`audio.sfx` → `snd.jumper_clamp`. Available only with both components and no
existing adapter. No magical power-restoration prerequisite.

<a id="r03"></a>
### R03 — Human alignment

**Story [C] — TEXT `radio.alignment_failed`, `radio.aligned`.** Six-second
manual alignment is offered with no aligned dish, no call, no adapter. After
accounting work, cold at least 90000 fails; otherwise `dish_aligned=true`.

**Performance [A].** Track from Alex's hands to Lea bracing the wheel with her
shoulder. Success reduces the receiver's searching sweep to a steady beacon
mark. Failure is a sliding grip and Lea catching the swing before injury; do
not animate a broken wrist or mark alignment anyway. Maintain her strained pose
through later interior shots after manual success.

LEA [C, success]: “I can hold it, but you'll have to come back for me.”

**Cue reference [R].** `radio.hand_align / alignment_failed|aligned` (1800ms);
`actor.gesture` → `anim.align_dish`, `anim.catch_wheel`, `anim.hold_wheel`;
`prop.animate` → `prop.antenna_wheel`; `audio.sfx` → `snd.beacon_lock` only on
success. Camera insert cannot substitute for the accepted `dish_aligned` effect.

<a id="r04"></a>
### R04 — Gust at fifteen seconds

**Story [C] — TEXT `radio.gust_warning`.** At T=15000, if `!radio_auto`, Lea
catches a gust with both arms and warns she is still at the mast.

LEA [C]: “If we do this manually, remember that I'm still out here.”

**Performance [A].** A gust crosses the mast set as a short lateral snow burst;
Lea drops a shoulder and catches the handwheel. Keep an ongoing Alex assembly
or control gesture alive in parallel. Prior to manual alignment this is a
momentary catch, not an automatic successful alignment. Even with an adapter
assembled, the cue remains eligible until `radio_auto` is actually true; do not
suppress it just because the mug prop is mounted. After automatic call success,
remove pending gust work and avoid returning her to an obligatory hold pose.

**Cue reference [R].** `radio / gust_warning`, 1800ms at 15000;
`environment.weather` → `fx.mast_gust`; `audio.sfx` → `snd.mast_gust`;
`actor.gesture` → `anim.catch_wheel`. A one-shot scene event, not a looping
script that can damage actors or extend transmitter time.

<a id="r05"></a>
### R05 — A real voice, and the cost of a manual signal

**Story [C] — TEXT `radio.cold_transmit_failure`, `radio.automatic_call`,
`radio.manual_call`.** Transmission costs 3000ms, offered with no finished call
and either adapter or aligned dish. After accounting, cold at least 90000 fails
before either success. With adapter, success sets call and automatic mode;
otherwise call succeeds manually and Lea remains necessary. Both successes
remove the reserve deadline. Failed transmission removes neither reserve nor
requirement; the player may warm hands, retry, or abandon if still possible.

**Performance [A].** On cold failure Alex keys the wrong bank; carrier falls to
static before a bearing goes out. Lea keeps physical alignment where applicable;
no answering helicopter voice or victory light. On success, contrast the old
identical broken breath with a live, responsive voice (unidentified coast crew,
not a new named subplot). Close on Alex's send switch, then Lea:

LEA [C, automatic]: “Now we can both leave it.”

Her fingers slowly uncurl; she shows both shaking hands. On manual success she
stays braced, the door behind her still a separation from the others.

LEA [C, manual]: “Don't leave me here thinking the radio works by itself.”

No cinematic automatically cuts the manual line just to bring her indoors.
Successful call removes main reserve pressure, not her low-power beacon duty.

**Cue reference [R].** `radio.send_signal`, ordered outcomes /
`cold_transmit_failure`, `automatic_call_deadline_cancelled` + `automatic_call`,
or `manual_call_deadline_cancelled` + `manual_call`; captions 1800ms.
`dialogue.play` → `voice.coast`; `audio.sfx` → `snd.real_reply`;
`actor.gesture` → `anim.release_wheel` only when the actual outcome permits.
Work finishing exactly at deadline can succeed; cutoff before completion cannot
invent a sent signal. No generic “timer elapsed = success” media handler.

<a id="r06"></a>
### R06 — Heat for the fine adjustment

**Story [C] — TEXT `radio.warmed_hands`, `radio.warm_again`.** Power and positive
cold allow four seconds of warming; reduce booked cold by 20000 or to zero.
No automatic radio success, alignment or repaired battery.

**Performance [A].** In a warm cabin, Alex holds hands over a small vent while
Lea stabilizes controls. A close-up on returning finger control is enough;
do not include a leisurely separate break outside the running reserve. If no
power exists, vent remains cold and no heating choice is offered.

**Cue reference [R].** `radio.warm_hands` / `warmed_hands|warm_again` (1800ms);
`anim.warm_hands`, `fx.restored_heat`, low `snd.motor_run` through the wall.

<a id="r07"></a>
### R07 — The protector trips, the bearing does not un-happen

**Story [C] — TEXT `radio.timeout`.** Timeout loses cabin access and returns to
dispatch. A completed call has supplied a bearing; its low-power beacon may
still need Lea or the jumper. Without a call, Lea comes inside. In ordinary play
success already cancels the deadline; do not force a second post-success timeout
just to stage this conditional line. Voluntary abandon is offered only before
a finished call; return only after one.

**Performance [A].** Before a failed reserve expires, the battery indicator dims
and the main transmitter relay opens. Without a call, Lea lets the now-useless
wheel go and follows Alex through the door. With a maintained manual bearing,
keep her at the mast in return shots; with auto mode, both may leave. A sealed
cabin is shown as inaccessible, never a justification to erase `radio_done`.

**Cue reference [R].** `radio.return`, `radio.abandon`, or timeout / `timeout`
(1800ms) → `dispatch`; `cam.threshold`, `snd.protector_trip`, `prop.radio_door`.

| Action | Work | Availability / result |
| --- | --- | --- |
| `radio.make_adapter` | 3000ms | Braid, mug, no adapter → consume components, assemble. |
| `radio.hand_align` | 6000ms | No alignment/call/adapter → cold failure or alignment. |
| `radio.send_signal` | 3000ms | No call and (adapter or alignment) → cold failure, automatic or manual call. |
| `radio.warm_hands` | 4000ms | Power and cold >0 → recover grip, remain in cabin. |
| `radio.return` | 0ms | Call done → dispatch. |
| `radio.abandon` | 0ms | Call not done → `radio_lost`, dispatch. |

Every route continues with the same Lea, mug/gull and handwheel identities.
Her location is a narrative responsibility, not an independent NPC planner.
