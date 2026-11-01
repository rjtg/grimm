# ADR-0008: Versioned M4.6 lifecycle and transactional storage

Status: Accepted (architectural semantics; API stability reserved). Date: 2026-11-01.
Decision owner: `rjtg`, repository maintainer.
Decision: Maintainer explicitly accepted ADR-0008 on 2026-11-01 with an API
reservation. The scene lifecycle, state ownership and transactional semantics,
commit-at-selection, destination-local initialization, gameplay/presentation
clock separation, opt-in behavior and legacy compatibility are accepted as
architectural direction. Concrete Rust public APIs, protocol envelopes and
serialized formats remain versionable and may be revised deliberately until
WHITEOUT's graphical integration validates them end-to-end. Such revisions must
be explicit, versioned where persisted/on-wire data is affected, covered by
migration/compatibility tests and reviewed against these accepted invariants;
there is no blanket license to silently break existing schemas or saves.
This acceptance does not certify graphical renderer/importer readiness, approve
external provider implementations, or mark M5 production complete.
Supersedes no accepted ADR. Implements the scope of the M4.6 design after the
integration audit found no existing runtime implementation.

## Context and preservation

M4.5 schema-1/2 games, APIs, timing and paused mid-scene checkpoints must remain
working. M4.6's explicit preparation/end boundaries and normal boundary-only
checkpoints cannot be silently imposed on those games. ADR-0004 continues to
make the engine the only validator and committer of narrative truth; replaceable
backing storage does not grant a provider independent gameplay authority.

## Implementation direction

- Introduce explicit opt-in versioning for new scene/lifecycle/checkpoint data.
  Legacy schema 1/2 keeps its old execution and save policy. The opt-in lifecycle
  configuration is separate from the unchanged story; the new versioned envelope
  and boundary checkpoint do not force a story rewrite. No automatic
  conversion of existing mid-scene checkpoints into boundary checkpoints.
- Use prepare/readiness, begin, outgoing-end and stable-boundary coordination.
  Initial preparation and inter-scene resource loading cost no scene time.
  A stable boundary persists until the host deliberately begins the next scene.
  All lifecycle inputs carry revision and scene correlation; stale/duplicate
  feedback must not activate choices or start a new epoch twice.
- Rules resolve action/timeout effects exactly once at selection/expiry, as now.
  When a transition is committed, lock old actions and retain only explicitly
  authorized outgoing presentation. The next context cannot execute while the
  outgoing scene is closing. Canonical destination locals initialize atomically
  with the selected transition; the destination remains inactive until begin.
  Host inspection can therefore show pending destination state while the
  presentation envelope still identifies the closing source scene.
- End freezes the outgoing *gameplay* clock and consumes its obsolete deadline.
  Renderer media may still finish using its own presentation elapsed time;
  completion reports the frozen canonical scene time. Such media time is not a
  second gameplay/world clock and must never book exposure or authored costs.
- Normal M4.6 save/restore uses a checked boundary API. Preserve low-level legacy
  replay export for compatibility; an active-scene export is not a valid M4.6
  player checkpoint. Store/verify transition continuation, storage snapshot and
  versions together. Restore does not replay outgoing graphics or effects.
- A small injected provider interface supplies immutable typed snapshots and a
  compare-and-swap atomic commit. Engine evaluates ordered rules against one
  snapshot and commits a complete validated delta/scope change once. No silent
  retries at later times. Default in-memory storage stays offline and simple.
  Provider failure or stale revision changes neither engine history nor state.
  External independent writes and debug mutation are unsupported unless a later
  recorded host transaction explicitly introduces them.
- Provider snapshot/revision is host-only, separate from Cue Sheets. Renderer
  catalogs are independent tooling; concrete paths/resources stay opaque to core.

## Verification evidence for this architectural decision

Atomic provider rejection/overflow/local reset/stale revision; begin/end/ready
idempotency; persistent stable boundary; preparation failure/retry without time
cost; outgoing presentation and deadline ordering; terminal closure; exact next-
scene checkpoint restore; schema-1/2 compatibility; existing WHITEOUT endings and
cold accounting; headless manual-clock and TUI/inspector integration. Keep M5
screenplay and rendering work isolated until these blocking M4.6 tests pass.


## Review clarifications

Providers are trusted implementations of an atomic persistence contract. The
wrapper checks their returned snapshot **after** commit/restore; it cannot roll
back a provider that persists incorrect state, misreports its result, or writes
before returning an error. Engine candidate/history publication is withheld on a
detected violation, but persisted provider state may already have changed. Halt
and reconcile that backend; do not retry the player action. See the provider
conformance and deliberately non-conforming fixture tests in world_provider.rs.

Boundary checkpoints retain full semantic history plus a world snapshot. Restore
replays from story defaults and rejects histories above 65,536 entries before
replay. This is an entry budget, not a serialized-byte or latency bound. There is
no compaction in M4.6; long-running games need a future versioned, validated
snapshot/continuation strategy preserving cue deduplication and stale-revision
protection. See the migration guide for performance and size implications.
