# First Bevy host: audit and contract plan

Audited 2026-11-03 against main `6c5ab68` (merged PR #22). This is the focused
preparation for #63/#53/#25/#36/#37, not a Bevy application or completed save rewrite.
[ADR-0008](adr/0008-m46-versioned-lifecycle-and-storage.md) records the maintainer
amendment. Current code, accepted target, implemented foundation and pending work
are deliberately distinguished below.

## Decisions and issue reconciliation

- **#25:** Bevy is the intended first graphical host, including fixed-camera
  point-and-click. The old sprite-engine competition in the roadmap is superseded.
- **#53:** Bevy owns ECS storage and physical time; GRIMM interprets authored rules;
  a small concrete adapter maps fields, inputs, cues and lifecycle. No second
  independently mutable store, generic renderer selection or parallel story FSM.
- **#63:** the host automatically captures a post-transition/pre-Begin boundary
  snapshot; Save copies that snapshot; Load installs it without replaying effects.
  The 2026-11-02 maintainer decision and this assignment override older issue/ADR
  bullets requiring development-save migration. No released saves exist.
- **#36:** define one fixed-camera WHITEOUT slice and deterministic acceptance
  cases, not an engine comparison. The generator slice is specified below.
- **#37:** implement that slice only after the provider and snapshot-continuation
  contracts pass headless tests. #38 later records measured Bevy integration risks;
  it is not an invitation to reopen alternative engine selection.

## Implementation audit

| Concern | Actual implementation | Gap / next contract |
| --- | --- | --- |
| Authored rules | `SceneEngine` conditions, effects, duration/cutoff, GOTO and outcomes are deterministic; no wall-clock/background task | Reuse these rules; do not copy them into Bevy |
| World backing | `WorldStateProvider` has typed snapshot, atomic CAS commit/restore; `ProvidedSceneEngine::transact` evaluates a cloned candidate and publishes after commit | Provider is owned long-term, initialized at exact story defaults and `ensure_current` rejects any externally changed snapshot. It is not yet a borrow-scoped ECS integration or an external-write refresh API |
| Evaluated state | `SceneEngine` has globals/locals, plus `committed` snapshot in the wrapper | Today these are private evaluation/cache copies, not writable through adapters. For Bevy they must become disposable immutable transaction projections read from ECS, never a competing world authority |
| Mutations | Ordered effects validate inside the candidate; provider receives final `Set`/`Enter` delta | Multi-field preflight/CAS/apply must be indivisible in ECS; provider validates schema, type, overflow and stable identities before any write |
| Lifecycle | `LifecycleEngine` exposes Prepare/Ready/Begin/Ending/End/Stable, explicit scopes and correlated ACKs | Reuse. Bevy readiness means bindings/dependencies/pipelines needed for this scene are ready, not just a spawned hierarchy |
| Transition order | GOTO initializes destination locals and effects in the same provider transaction; outgoing metadata still names the source until Begin | Capture destination continuation and destination world at Stable, not source presentation metadata. Never initialize destination locals again on restore/Begin |
| Time | `SceneAction` carries revision and selection time; `observe` and cue ACKs can resolve due deadlines. `SceneSession::advance_to` is a CLI host scheduler | No autonomous engine timer exists. Bevy must not drive that CLI scheduler or submit frame ticks. Use host action-at-T and deadline notification, plus presentation ACKs at actual host times |
| Timeout foundation (this PR) | `DeadlineReached` + `LifecycleEngine::deadline_reached` validate scene ID, epoch, revision, exact active deadline and observed time, then reuse timeout rules | Library-only additive input; early/stale/duplicate notifications cannot apply effects twice. No outbound Cue Sheet version or checkpoint format changed; old replay logs still record an observation. Bevy transport/journal remains pending |
| Checkpoints | `checkpoint()` constructs world + full `LifecycleInput` history on request; `restore()` starts at defaults, replays, compares world and reserves a new revision | Not host snapshot-slot semantics. No public non-replay resume/import or coherent ECS+continuation installation exists |
| History | Core `ActionRecord` includes synthetic timeout records; lifecycle history mixes actions, observations, presentation ACKs and restore reservations | Need ordered semantic action/timeout/transition/entry events separate from playback and world reconstruction; current history is not that journal |
| Host/UI | CLI backend fixes `LifecycleEngine` to InMemory, Save serializes current boundary checkpoint; TUI/pipe paths and catalogs are implemented | Keep CLI behavior working during contract work; it is not evidence that ECS, automatic snapshot slots or host-owned Load already exist |
| Bevy | No Bevy dependency, ECS provider, graphical resources or host executable | #37 supplies these after prerequisites; no additional engine framework is needed |

Relevant files: `crates/engine/src/scene/{mod,state,lifecycle}.rs`,
`crates/semantic/src/{scene,world,lifecycle}.rs`,
`crates/cli/src/scene_session{.rs,/backend.rs}`, existing lifecycle/provider,
scene-timing and WHITEOUT integration tests. LSP diagnostics were requested for
representative implementation files; timeouts are not evidence of clean code.

## Smallest provider evolution (planned, not implemented here)

Keep `Value` types, `WorldScope`, complete `WorldSnapshot` and CAS semantics.
Refactor the provider access seam to borrow the concrete provider for a narrative
transaction instead of requiring a long-lived owned ECS World. An exclusive Bevy
system can lend `&mut World` to a short-lived provider; runtime storage in a Bevy
resource must be scoped so its borrow does not alias the World borrow. Prove that
borrowing pattern in a headless ECS test before selecting its exact Rust signature.
Do not use raw pointers, an Arc<Mutex<World>> workaround or a copied world resource.

One exclusive operation:

1. Resolve explicit semantic field mappings to actual resources/components using
   stable logical keys (e.g. `crew.jonas`); reject missing/duplicate targets and
   lossy/wrong type conversions. Do not persist raw Bevy Entity handles as identity.
2. Read one immutable snapshot/revision under the same scheduler exclusion used
   for commit. Conditions/effects evaluate only that projection.
3. Preflight **all** destinations, types, arithmetic and scope initialization in
   scratch values; recheck the expected revision; apply all fields + scope + new
   revision before releasing exclusive access. No fallible lookup after first
   write. A deferred Commands queue is not an atomic world transaction.
4. Publish GRIMM continuation/cues and semantic events only after successful
   commit. Rejection publishes neither partial fields nor accepted actions.

Bevy systems may update mapped simulation facts only through an explicit
revisioned host transaction at a defined schedule boundary. A changed revision
must refresh GRIMM's rule projection and current eligible offers, issue a new
input revision and invalidate queued choices, without replaying timers/media.
A conflict with an already selected action returns a stale/conflict result; never
silently retry at a different time. Unmapped presentation fields need no narrative
revision. The current wrapper **does not implement this refresh**; remove its
exact-snapshot rejection only together with this refresh/CAS test suite.

For #37 map at least `global.generator_done` to a bool resource field and
`global.latch_damage` to an i64 resource field, plus a stable crew component
binding such as `global.jonas_treated`. Account for every other world field used
by the canonical story in authoritative host resources/components too; do not
keep unhandled flags in a mutable GRIMM fallback map. Globals persist; scene-local
fields are initialized/reset only by committed narrative scope changes.

Provider atomicity remains a trusted backend contract. GRIMM cannot roll back a
custom provider that writes before reporting failure or misreports its snapshot.
Tests must check actual persisted ECS fields, not just the returned snapshot.

## Host-driven time and event arbitration

Bevy owns elapsed scene time and the optional deadline timer. GRIMM stores only
semantic observations/deadline specifications for rule evaluation. `Begin` resets
and starts the destination scene time at T=0; Prepare/Stable do not consume it.
Ending freezes outgoing gameplay time while media can finish on presentation time.
Menu pause stops the relevant host scene clock; resume continues without reset.
Relative deadline cues start at their actual Started acknowledgement; absolute
ones use scene-relative absolute times. Reveal never restarts a timer; extension
reschedules its deadline token; unset cancels it. REPLACE retains started timer
identity. No physical deadline discovery occurs inside GRIMM. The current lifecycle enum has
no explicit Pause/Resume events: CLI menus pause host time locally. #53 must
specify the concrete adapter pause cause/state alongside lifecycle running/frozen
outputs; do not claim a new pause instruction already exists.

Adapter inputs (conceptual examples; new event envelope beyond the existing
library DTOs remains to be specified with the adapter):

```text
ActionSelected { revision: r, epoch: e, action_id: generator.open_generator_latch,
                 scene_time_ms: 8000 }
DeadlineReached { revision: r, scene_id: whiteout.generator_annex, epoch: e,
                  deadline_ms: 30000, scene_time_ms: 30007 }
```

Freeze selection time before evaluation, then resume from that same T. Authored
logical duration is resolved once in GRIMM; never advance Bevy by that cost too.
Cue ACKs at actual host times remain presentation coordination, not success votes.

**Slice arbitration policy:** at T >= active deadline, dispatch deadline first
and discard that epoch/revision's selected action; before it, dispatch the action
once. Process deadline before same-time presentation completion. Deduplicate host
notifications by scene epoch/revision/deadline and preserve chronological input
ordering within a frame. Late delivery carries observed time but resolves the
authored deadline/cutoff. Multiple successive actions within one epoch are valid;
"one event" means one resolution of each interaction, not one action per scene.

**Existing gap:** raw `SceneEngine::act` uses `selected > deadline` and can accept
an otherwise eligible zero-duration choice exactly at D, whereas the shared CLI
host observes expiry first at equality. Do not claim raw apply already implements
universal deadline-first arbitration. The Bevy action boundary must enforce the
policy and test both delivery orders; existing WHITEOUT authored rules stay intact.
The new deadline method rejects duplicate/stale reports rather than acknowledging
success again. Refresh a queued notification's input revision after a compatible
replacement only if the same timer remains current; discard it on extend/unset,
transition or restore. Never repurpose an old event for a new epoch.

## Host snapshot and GRIMM continuation (planned #63)

The host owns the aggregate save: selected authoritative ECS fields, logical
entity identities and a minimal opaque GRIMM continuation. It must not serialize
the whole render world, reconstruct world state from narrative history, or save
only the visible source scene ID. Proposed continuation content:

| Data | Reason |
| --- | --- |
| Continuation format version, story/config IDs and versions | Reject incompatible rules/mappings before mutating the world |
| Pending **context** and semantic presentation scene ID | Several contexts may share one presentation scope; scene ID alone is insufficient |
| Marker `PostTransitionPreBegin` or `Completed` | Distinguishes destination entry from already finished terminal presentation |
| Committed destination epoch/scope + transition/entry identity | Correct same-context reentry and pairing with mapped locals |
| Remaining logical eligibility floor | GOTO can have nonzero action duration/cutoff credit; `eligible_ms` is not always zero in the destination even though clock T=0 |
| Monotonic cue/action run identity seed | Existing generated `__run_N` uses action record count; removing the journal must not recycle identities |
| Fresh host session generation / revision reservation | Pre-load inputs/ACKs must never match the restored session, including load into a fresh process |

These are requirements for the concrete continuation implementation, not a new
persisted DTO introduced here. No started cue IDs, source media playback, active
clock position or running deadline is required at this supported boundary.
The destination clock/deadline have not begun. A saved Completed marker does
not replay ending media. Optional semantic history is inspection data, not needed
to materialize variables or validate a trusted host snapshot by replay.

Order is `commit effects + initialize destination once → finish authorized
outgoing media → End → Stable → capture boundary slot → Prepare/Ready → Begin`.
The host freezes a consistent world revision/continuation pair under exclusive
access; an intervening authorized write invalidates/retries capture before Begin.
Capture failure keeps the boundary stable, retains the prior slot and exposes
retry; never continue with a torn snapshot. Initial entry also gets an
`InitialPreBegin` boundary slot after defaults are initialized once, before its
first Begin. Do not fake an outgoing transition or replay the opening to save it.

**Save:** atomically copy the immutable last boundary slot to a persistent slot;
no rule evaluation, new ECS snapshot, initialization or gameplay effect. For the
first slice, disable Save while Active/Ending to avoid implying current progress
is saved. Repeated Save is identical gameplay-wise. A later UI may explicitly
choose last-boundary rollback semantics, never silently mix active values into it.
Persistent-copy failure leaves the previous slot intact; saving retries copy only.

**Load:** parse/check versions, identities, mapped types/scope and continuation
in candidates; stage ECS installation and GRIMM resume under one host-exclusive
transaction. No previous action or transition is evaluated. Do not call current
`LifecycleEngine::restore`, `new` at the saved context's defaults, or reapply
`StateMutation::Enter`. Preserve the saved destination locals as captured. Clear
presentation correlations, reserve a fresh session generation/revision, rebuild
only entry presentation/offer evaluation, then wait for Ready/Begin at T=0.
Publish neither world nor continuation on validation/installation failure.

The continuation's remaining eligibility credit must be restored, not recharged
or lost. Destination entry cues materialize from the restored world at Begin;
Completed stays completed. Host timer/presentation state is reset, not restored
mid-animation. The adapter is a coordinator of this sequence, not a new narrative
FSM; GRIMM owns its actual lifecycle/transition validation.

Development save/checkpoint formats can be replaced incompatibly. Keep story
formats/gameplay correct; no migration implementation is required. The current
CLI/replay APIs remain operational in this PR as a development baseline, **not**
as the target. Removal/replacement belongs to the snapshot-continuation change.

## Semantic journal (planned, independent of persistence reconstruction)

One chronological host-observable journal records accepted action + resolved
outcome, timeout and transition/entry, with sequence, epoch, action ID, relevant
selection time/effective duration and source/destination identity. Publish action
then its committed transition in the same successful transaction; failed CAS or
stale events append neither. Begin adds entry once; same-context reentry gets a
new epoch. Timeout produces timeout + transition (or in-scene consequence).
Initial entry and completed-game closure are explicit. Keep cue ACK deduplication
in live presentation coordination; do not require its log in a boundary save.
Optional journal truncation/storage never changes resumed world values. Restore
may record a new session/resume marker; it does not append old effects again.

## #37 implementation order and exact proof

| Step | Issue / prerequisite | Deliverable and tests |
| --- | --- | --- |
| 1 | #53 provider seam; this audit | Borrow-scoped transaction access + authoritative snapshot refresh. Headless typed field fixture: resource bool/i64, stable component identity, missing/type/duplicate mapping rejection, external write changes offers, stale CAS leaves fields/history untouched, overflow/multi-write rollback, local reset atomicity |
| 2 | #63 after step 1 | Minimal boundary continuation export/import + optional semantic action/transition journal independent of replay. Host snapshot fixture tests: initialized destination locals preserved once, initial/pre-Begin, nonzero GOTO eligibility credit, reentry, completed ending, repeated Save copy, post-boundary changes excluded, journal absent, invalid/load failure atomicity, fresh-generation stale ACK/input rejection |
| 3 | #36 + #53 time contract; steps 1–2 before full host | Freeze generator slice spec; deadline-first tests at D−1/D/D+1 including zero-cost action, late/duplicate/old events, extend/reveal/unset, pause/resume, no action, no double charged action duration; no real sleeps |
| 4 | #37 headless Bevy 0.20.1, steps 1–3 | Actual ECS field provider and automatic boundary-slot capture/copy/load; `App::update` with controlled host time, all provider/load tests against real fields, no graphics/GPU needed |
| 5 | #37 visible proof after step 4 | Fixed-camera scene, core picking/UI, real handles/catalog and loading gate, overlapping media, one accepted transition; graphical manual smoke plus CPU-only CI rules/event tests |
| 6 | #25/#38 evidence, no extra engine evaluation | Exact build commands, used features/APIs, screenshots/interaction script, measured limitations, packaging follow-ups and next content tasks; no complete WHITEOUT |

**Concrete #36 slice:** use canonical `examples/whiteout/story.yaml`. Prepare a
trusted test boundary by playing the existing stores/preparation/dispatch route
headlessly up to pending generator entry, then capture its host snapshot. This
is fixture construction, not replay during Load and not a fork of story.start.
Use one fixed Camera2d/background composition, latch hotspot and the same offered
semantic action labels in UI. At T=8 select the latch; preserve its authored cost;
at T=10 radio warning overlaps and the already running hidden 30s deadline is
revealed with 20s remaining. Verify earliest subsequent offers and no duplicate
radio/timer start across a later replacement. An abandon/return choice proves
outgoing Ending → Stable capture → destination Prepare/Begin. A separate no-input
run reaches the authored timeout. Include a terminal-boundary fixture even if
full rescue graphics are out of scope. Do not add an alternate graph to manufacture
the timing or hardcode puzzle conditions in the host.

Initial resources: a simple station/generator background, latch hotspot image,
readable choice/timer UI, warning audio/subtitles (with silent-test substitute),
and outgoing/next-scene placeholder. Label placeholders honestly; no bulk assets,
free-roaming camera, navigation framework or screenplay compiler is needed.

## Bevy 0.20.1 baseline and remaining review questions

Official versioned [crate documentation](https://docs.rs/bevy/0.20.1/bevy/) was
reachable during this audit. Target `=0.20.1` for the future host, no third-party
plugins initially. The official [sprite picking example](https://bevy.org/examples/picking/sprite-picking/)
uses `Camera2d`, core pointer observers and the `sprite_picking` feature; the
[loading example](https://bevy.org/examples/games/loading-screen/) distinguishes
asset loading from render-pipeline readiness. [BSN composition](https://bevy.org/examples/scene/bsn/)
is useful, but hierarchy readiness alone cannot authorize GRIMM Ready.
These online examples can change; #37 must compile exact pinned APIs/features,
including Time/Timer and exclusive World access, before claiming compatibility.
Individual 0.20.1 World/Timer API pages could not be fetched in this audit; no
untested method signature or plugin compatibility is asserted here.

Remaining questions requiring concrete proof, not invented abstractions:

- Whether existing lifecycle running/frozen metadata plus host-menu pause is
  sufficient for the slice or a narrowly scoped pause/resume host instruction is
  needed; no physical ticking or separate narrative FSM either way.
- Exact safe Rust borrowing signature for scoped provider access to a Bevy World
  with runtime stored as a resource; scheduler exclusion and allowed writer order.
- Concrete continuation representation and validation of persisted identity/epoch,
  eligibility credit and fresh correlation generation; exact atomic install path.
- Which selected ECS fields/entities outside mapped narrative variables the
  concrete game snapshot must include; never blanket-save transient render data.
- Physical presentation duration versus authored logical costs for placeholder
  media; readiness policy for audio device unavailable in CPU-only tests.
- Platform/package targets: start desktop; WASM/distribution support only after
  measured CI/build evidence. No portable-host or multi-renderer platform promise.

**Next implementation:** #53's scoped provider/refresh prerequisite, then #63's
non-replay continuation/snapshot tests. #37 follows those gates; this PR does not
claim them complete or start the graphical application.

## Verification of this preparation

Formatting, `cargo build --workspace --locked`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` and
`cargo test --workspace --locked` passed: **210 tests, 0 failures** (208 baseline
plus two deadline-notification regressions). Tests use explicit times, no sleeps
for the new input cases: exact/late expiry, duplicate/stale action/timeout, early
notification, wrong scene/epoch/revision/deadline and preparation rejection.
Current story/prose assets, dependencies and outbound protocol are unchanged.
No ECS/Bevy build or non-replay save is claimed. LSP was current/clean for the
new semantic DTO and tests; some engine diagnostics/locations were stale or timed
out, so Cargo supplied semantic verification.
