# Act II — Engineering under ice

## `generator` — INT./EXT. GENERATOR ANNEX / ACCESS COIL FAILING

**Location:** `loc.generator_annex`, connected to the hall by a visible threshold.
**Cast:** Alex at the panel; Jonas assists only to the degree his treatment allows;
Mara supports him at the sheltered threshold, Lea on the backup set. These
positions are staging proposals: no unsupported walk by the injured man.
**Objective:** open access, identify/isolate the overloaded de-icer and restore
heat before losing access. The coil starts at **T=0, deadline 30000ms, hidden**;
arrival is 4000ms. Inventory, knowledge and injury carry in unchanged.

<a id="g01"></a>
### G01 — The dark display is already counting

**Story [C] — TEXT `generator.arrival`.** Ice shakes the annex; pressure gauge
marks thirty seconds until the shutter coil cuts out. Heat-pack battery can add
one minute. Repeated work or an insulated spanner can free the latch.

**Performance [A].** Open wide on the coil gauge and rising white drift behind
the set. The digital display remains dark; the pressure needle is moving.
Frame the starter behind the frozen cover, never already accessible. Jonas's
voice gives the latch advice from the threshold; if treated, show his hands
finding a bracing point while Mara keeps weight off his ankle. If untreated,
keep him seated and let Alex work without the assisting pose.

JONAS [A]: “Same pin each time. Or use a wrapped spanner.”

A short close-up connects coil cable, shutter and dead display. This is not a
cinematic pause; all four seconds count. Action choices open after the briefing.

**Cue reference [R].** `generator / deadline`, `arrival`, `choices`;
`shot.wide`, `sound.wind` are usable prototype references. `camera.closeup` →
`cam.coil_insert`; `prop.animate` → `prop.generator_latch`, `prop.annex_shutter`.

<a id="g02"></a>
### G02 — Borrowing a minute

**Story [C] — TEXT `generator.heat_pack`.** Deploying the pack consumes it and
extends the existing deadline by 60000ms. It does not restore generation or
reset T. If selection/work is cut off before deployment, normal effects and
extension do not happen.

**Performance [A].** Alex plugs the battery into the coil's auxiliary socket;
a small status diode changes, not the generator lamp. The gauge gets a second
range of travel. Leave the main engine still until a real start succeeds.

MARA [A]: “Borrowed time. We still need power.”

**Cue reference [R].** `ACTION generator.deploy_heat_pack` (1000ms,
`heat_pack`), `extra_minute` timer + `heat_pack` caption (1800ms);
`prop.animate` → `prop.heat_pack`; `audio.sfx` → `snd.connector`. Later warning
shows actual remaining budget (80s at T=10 if already extended), never hardcodes 20.

<a id="g03"></a>
### G03 — Ice gives way by degrees

**Story [C] — TEXT `whiteout.latch_crack`, `whiteout.latch_nearly`,
`whiteout.latch_open`, `generator.open_generator_latch_numb`,
`generator.work_latch_with_numb_hands_numb`.** Damage/progress persists; opening
requires 6. The offered work mode uses the currently booked cold: below 40000,
3000ms work; at least 40000, 5000ms work. After charging the action's elapsed
interval, outcomes use the new cold: at least 90000 means no progress; otherwise
below 40000 with treated Jonas adds 3, below 40000 alone adds 2, otherwise adds 1.

**Performance [A].** Lock a useful close-up on the same hinge, glove and ice
bridge across attempts. For crack, a small top-pin fracture; nearly, a narrow
remaining blue bridge; open, the final bridge breaks and cover swings clear.
Never replay the untouched hinge after progress. Treated Jonas braces the frame
from a low supported position; do not make him shoulder the whole housing.
When grip fails, Alex's fingers slide without the pin moving; return to the
still-closed latch. Failure is visible, not a successful animation with a red UI.

ALEX [A, nearly]: “One joint left.”

Do not say this line on crack or numb outcomes. Wind and the same metal tick
span the repeated attempts. Hands can deteriorate during deliberation, so a
previously fast offered attempt can resolve slowly or fail after its cost.

**Cue reference [R].** `generator.open_generator_latch` /
`generator.work_latch_with_numb_hands`; their `*_crack`, `*_nearly`, `*_open`
narrative cues last 3500ms; `*_numb` lasts 1800ms. Each authored `*_release`
action cue is floored at the logical work end. At S=8000 a 3000ms attempt may
still be visible when the radio begins at T=10000; actions cannot release before
T=11000, although the 3500ms media may remain. The radio must not cancel hinge
animation. `actor.gesture` → `anim.latch_work` / `anim.grip_fail`;
`prop.animate` → `prop.latch_ice`; `audio.sfx` → `snd.ice_crack`.

The numb-mode authored assisted/steady subbranches require cold below 40000
although their action requires at least 40000. Ordinary valid accounting cannot
reach them. They remain source references, not additional story routes; see
VALIDATION. Share the same performance variants rather than inventing scenes.

<a id="g04"></a>
### G04 — A lever costs the antenna its copper

**Story [C] — TEXT `generator.lever_made`, `generator.lever_failed`,
`generator.lever_open`.** Combine spanner + braid in 2000ms, consuming both and
creating a lever. Its 3000ms use opens the latch (`latch_damage=6`) unless
post-work cold is at least 90000. The lever is not consumed by use. Making it is
not offered once the cover is open.

**Performance [A].** Insert on the one braid wrapping the grip; keep the bare
mug visible in memory, not a second copper coil on the floor. Lea acknowledges
the manual route with a short radio click rather than new exposition. Success
lifts the pin smoothly; failure is Alex losing purchase while the tool remains
sound. Do not show a snapping lever or secretly consume another item.

LEA [A, assembly]: “Then we'll turn the dish by hand.”

**Cue reference [R].** `generator.make_lever / lever_made`,
`generator.use_lever / lever_failed|lever_open`, all feedback 1800ms;
`prop.animate` → `prop.insulated_lever`; `camera.closeup` → `cam.tool_insert`.
RULES holds exact guards. This resource trade-off must persist into radio staging.

<a id="g05"></a>
### G05 — The red branch, not the engine

**Story [C] — TEXT `generator.controls`, `generator.isolated`.** Inspection
(1000ms if controls unknown) identifies the automatic de-icer branch and Jonas's
bypass. The stores log is an alternative source of this knowledge. Isolation
(2000ms, knowledge required, not yet isolated) pulls the tagged fuse. Neither
action requires latch-open in the authored guards; preserve accessible external
diagram/fuse placement rather than adding a hidden physical prerequisite.

**Performance [A].** Put the wiring sketch and tagged isolator on an external
service panel beside the frozen starter cover. Inspection resolves its red
branch close-up; no new sabotage inscription. Jonas recognizes the diagram
without a villain reveal. On isolation the tagged fuse rotates/withdraws and
the shorting flicker stops; the motor remains silent. Jonas exhales.

ALEX [A]: “De-icer off. Engine next.”

**Cue reference [R].** `generator.inspect_controls / controls`,
`generator.isolate_deicer / isolated`, feedback 1800ms; prototype
`shot.controls`, `move.deicer`; `prop.generator_diagram`, `prop.deicer_fuse`.
A camera move on arrival does not grant `controls_known`.

<a id="g06"></a>
### G06 — Lea cuts across the work

**Story [C] — TEXT `generator.radio_warning`.** At T=10000, if generation is not
yet done, Lea reveals the already running timer.

LEA [C]: “Engineering, ten seconds elapsed on your coil. I'm putting the
remaining budget on your handset.”

**Performance [A].** Keep the latch/lever animation running. Sound the handset
beep and subtitle Lea in a separate radio band; reveal the readout without
cutting away from a crucial hand movement. A short controls reframing may
coexist with that work; do not overwrite another active motion or readout.
If generation already succeeded, omit the warning and reveal rather than
reviving a cancelled deadline. A replaced sheet must not replay the call.

**Cue reference [R].** `generator / radio_warning` (2200ms at 10000),
`reveal_clock` (same T), both `!generator_done`. `sound.alarm` is a prototype
beep reference, `dialogue.play` → `voice.lea`; `ui.offer` → existing offers, not
an unconditional refresh capable of breaking startup lock.

<a id="g07"></a>
### G07 — A start must actually succeed

**Story [C] — TEXT `generator.overload`, `generator.starter_cold`,
`generator.startup_prime`, `generator.startup_ignition`, `generator.startup_heat`.
Start is offered once latch damage is at least 6 and power not done; isolation
is an outcome test, **not an offer guard**. Four seconds of logical work is
booked once. First, an unisolated de-icer trips overload; otherwise cold at least
110000 misses the primer; otherwise power is restored and deadline removed.

**Performance [A].** An overload is a starter kick and red relay flag, then
silence. A cold miss is a thumb slipping off the detent, no ignition. Neither
shows a running flywheel or green lamp. Keep retreat readable after failure.

On success, sequence the three canonical beats under locked input:

1. **1200ms primer.** Close on fuel entering clear tubing; starter lever held.
   ALEX's breathing against a soft fluid sound. Prototype `shot.starter`,
   `move.starter`; `prop.primer_tube`, `anim.prime`.
2. **1500ms ignition.** Two uneven starter rotations then low engine roar;
   camera widens enough to see the flywheel catch. `motor.spin`, `sound.motor`,
   `snd.starter_turn`.
3. **1800ms heat.** Duct snow lifts, lamp settles, Mara's shoulders release.
   `environment.light` → `fx.restored_heat`; `prop.animate` → `prop.duct_snow`.

MARA [C]: “Power is stable. Come back. Lea still needs us.”

No input until the sequence completes and the logical cost floor is satisfied.
Canonically effects resolve at selection, not on an animation callback; the
visual sequence reveals the accepted result. Do not copy the prototype's delayed
4000ms effect commit as the full-story rule. Starting at T=27000 is cut off at
30000 and **does not create power**; starting at 26000 can finish exactly at the
deadline and cancel it. A start selected at the deadline itself loses to timeout.

**Cue reference [R].** `generator.start_generator`, ordered outcomes
`overload` / `starter_cold` / success: `generation_deadline_cancelled`,
`startup_lock`, `startup_prime`, `startup_ignition`, `startup_heat`, `startup_ready`.
Failure feedback 1800ms. These remain one canonical context; a future ordinary
startup transition node may stage them only with equivalent rule/timing behavior.

<a id="g08"></a>
### G08 — Return, abandon, or be shut out

**Story [C] — TEXT `generator.timeout`.** Powered `generator.return` is a
zero-cost return to dispatch. Unpowered `generator.abandon` seals access and
returns. Timeout also seals access and returns; any already started generator
would keep running. No route reopens engineering after `engineering_lost`.

**Performance [A].** On return, track warm air and Alex's retreat through the
threshold; Mara helps Jonas back into his hall position. On abandon, Alex
releases the silent starter and pulls clear. On timeout, coil hum dies, shutter
drops, Mara catches Alex's sleeve and hauls them inward. Mechanical access loss
is the consequence, never crushing a cast member. Keep engine sound according
to real power state; closing a door cannot turn off a repaired generator.

**Cue reference [R].** `generator.return`, `generator.abandon`, or context
`timeout / timeout` narrative (1800ms) + GOTO `dispatch`;
`move.shutter`, `sound.wind`, `cam.threshold`, `snd.shutter_close`.

| Action | Work | Offer / principal result |
| --- | --- | --- |
| `deploy_heat_pack` | 1000ms | Possessed pack → consume, extend 60000ms. |
| `open_generator_latch` | 3000ms | Damage <6 and booked cold <40000 → conditional progress/failure. |
| `work_latch_with_numb_hands` | 5000ms | Damage <6 and booked cold ≥40000 → conditional progress/failure. |
| `make_lever` | 2000ms | Spanner, braid, no lever, latch <6 → consume components. |
| `use_lever` | 3000ms | Lever and latch <6 → cold failure or open. |
| `inspect_controls` | 1000ms | Controls unknown → learn. |
| `isolate_deicer` | 2000ms | Controls known, de-icer connected → isolate. |
| `start_generator` | 4000ms | Latch ≥6, not done → ordered overload/cold/success outcomes. |
| `return` | 0ms | Power done → dispatch. |
| `abandon` | 0ms | Power not done → lose access, dispatch. |

All short action IDs in this table are under `generator.`; exact predicates,
accounting and all cue-instance paths are in RULES and PARITY.
