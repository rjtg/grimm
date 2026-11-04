# WHITEOUT host cue scheduler (#68)

Authority: [#68](https://github.com/rjtg/grimm/issues/68),
[architecture #66](https://github.com/rjtg/grimm/issues/66). Based on main `7f22db2`
(merged PR #72). All execution lives in the concrete WHITEOUT application;
`grimm-graph` and its protocol are unchanged.

## Time and arbitration

`Runtime` holds GRIMM, its complete current sheet, a node-entry origin in Bevy's
`Time<Real>`, and successful cue-instance keys. Integer scene milliseconds are
`floor((real_elapsed - entry_origin).as_millis())`. This intentionally avoids
`Time<Virtual>`'s frame-delta clamp: a delayed frame still sees every due cue.
No clock or scheduler input is sent to GRIMM.

The Bevy Update schedule has three host phases:

1. **Collect**: keyboard input captures `(edge_id, epoch, observed_ms)` from the
   current full sheet. The cue collector posts due instance identities. The two
   collectors may run in either order.
2. **Execute**: one arbiter resolves all proposals, ordering them by scene time,
   then **cues before player selections**, then original authored cue-list order.
   Equal-time player selections use edge-ID order. The keyboard collector itself
   emits at most one selection per frame, in number-key order.
3. **Present**: show only executed presentation, current ECS generator status and
   the current input gate. Update the countdown from the same scene timeline.

Consequently an input observed at 29999ms beats the authored 30000ms timeout,
including when both arrive in a frame sampled at 30001ms. At **exactly 30000ms the
timeout wins**. This does not depend on collector/system/message delivery order.
All input collectors must participate before Execute; a later-frame event cannot
undo a traversal already accepted. A selected timestamp must lie between the
last processed scene sample and this frame's sample, inclusive; future or older
backdated selections are rejected rather than replaying world changes.

`at_ms` orders work, not list position across different timestamps. Several cues
can run in one frame, including after a large frame skip. At equal timestamps the
source list is authoritative. An accepted traversal cancels the remainder of the
old entry, even if the skipped frame would otherwise have made it due.

## Full REPLACE and bounded work

Both player and cue traversals call `GraphEngine::traverse_edge(edge, epoch)`.
Only a successful result replaces the full sheet. Its outgoing discovery map
replaces the old one; hidden edges are never player options. GRIMM validates
structure/correlation and never evaluates conditions or executes effects.

On REPLACE the host clears old consumed keys and view, discards old queued work,
and records the current physical frame sample as the destination's origin. The
new entry starts at **T=0**, without carrying the old frame's overshoot forward.
Already committed generator effects stay in the host ECS; they are not undone or
reapplied by replacing a presentation plan. Authored destination T=0 presentation
and effects start in the same render frame.

At most **one graph traversal per frame** is accepted. If a destination has a
T=0 traversal cue, that cue and anything after it wait for the next scheduler
pass. This bounds zero-time cycles while preserving authored order; it is a host
execution budget, not an additional story lifecycle, turn clock or core FSM.

## Cue identity, callbacks and effects

A cue instance is `(node_entry_epoch, cue_id)`. Its parameters are always looked
up in the current authoritative sheet. `HostRequest::WakeCue` carries identity
only; the normal cue collector and delayed/duplicate wake-ups share the same
execution path. Unknown, premature or stale callbacks cannot execute a payload.
Successful consumed instances are ignored, including repeated wake-ups. Reentry
creates fresh epochs, so an old callback cannot act on a new visit with the same
cue/edge ID. These queues belong to one Bevy App; no saved/restored runtime or
cross-session callback support is claimed here.

`set_generator_status {status: running|unstable|isolated|stopped}` is the one
WHITEOUT-specific synchronous effect. It validates exactly one text argument and
preflights its application counter before changing either field of the
`Generator` resource. This is a small demonstration handler, not #69's general
handler/condition architecture. The direct `grimm-semantic` dependency supplies
only the existing scalar parameter DTO; no interpreter or provider is imported.

On effect/presentation/traversal execution error the instance is **not** recorded
as successfully consumed. The host displays a specific cue error, closes input
and stops the current entry, including pending timed traversal. It never silently
continues or retries the effect. Earlier successful effects remain committed;
the failed handler itself makes no partial writes. A graph traversal and later
host effects are not one transaction: if a destination T=0 effect fails, GRIMM
has already entered that destination. The host visibly halts that entry without
showing its later success presentation or rolling the graph back. Recovery in this demo is to
close and restart the application. No async effects run: there is no in-flight
work to finish/roll back or generic async execution framework.

## Presentation is not media completion

A presentation cue selects a concrete WHITEOUT panel. Its cue identity and
`duration_ms` are retained as metadata; elapsed duration never creates a success
ACK, unlocks input or proves a real animation completed. Explicit presentation
bindings select the host UI behavior: `generator_ready` opens the interaction and
shows the countdown; warning/transition bindings keep input closed. These are
WHITEOUT conventions, not new GRIMM protocol kinds or generic choice semantics.

The countdown is derived from the current node's next authored traverse cue;
it does not start/reset a separate timer. The warning hides it until the ready
cue at 5000ms, then shows 25s remaining until the 30000ms timeout. The display uses
rounded-up seconds, while arbitration uses integer milliseconds. A frame that
first renders readiness at 6000ms correctly shows 24s, not a restarted 25s.

The placeholder presentation is static text/color, not an animation player.
The ordinary GENERATOR_TRANSITION node explicitly schedules status isolation at
1200ms and hidden `continue` at 1500ms. This follows authored timing; it does not
infer that arbitrary media has finished. A future asynchronous media/effect
handler must define cancellation/completion barriers before using this path.

## Deterministic evidence and remaining scope

`src/scheduler/tests.rs` uses `TimeUpdateStrategy::ManualDuration` with actual
Bevy schedules/InputPlugin. Tests cover no early/duplicate execution, skipped
frames, equal-time order, timed transitions, early selection cancellation,
30000ms races with swapped messages **and swapped Bevy collectors**, stale
callbacks/inputs, canceled pending effects, reentry, typed failures/overflow,
duration metadata and bounded zero-time traversal. `screen.rs` tests initial UI,
gating, status, full hierarchy replacement, countdown continuity under duplicate
delivery and timeout without a window/GPU. Cargo metadata boundary tests still
allow only `grimm-semantic` as the graph's runtime dependency and explicitly
reject graph → WHITEOUT/Bevy/legacy-engine dependencies.

#69 (conditions/gameplay handlers), #70 (full WHITEOUT slice), #63 (host saves),
assets/animation, codegen and optional rules extraction remain separate. No
canonical CLI WHITEOUT rules or legacy scene/lifecycle interpreters are changed.
