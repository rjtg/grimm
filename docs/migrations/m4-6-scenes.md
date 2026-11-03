# M4.6 opt-in lifecycle and storage contract

**Historical implemented baseline.** The 2026-11-03 maintainer amendment in
[ADR-0008](../adr/0008-m46-versioned-lifecycle-and-storage.md) supersedes this
file's replay-save, save-compatibility and external-write restrictions as target
architecture. Current code still implements the baseline described below;
host-owned boundary snapshots/non-replay restore and ECS refresh are planned,
not implemented. No development-save migration is required. Use the
[Bevy audit/plan](../bevy-integration.md) for current decisions and gaps.


M4.6 adds a host configuration and protocol envelope; it does not change the
schema-1/2 story DSL or the M4.5 `SceneEngine` API. Legacy `play-scene` commands,
mid-scene saves and archived inputs keep their existing behavior. Opt in with:

```bash
cargo run -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml --lifecycle examples/whiteout/scenes.yaml --debug
```

## Versions and ownership

- Scene configuration **1** names the story/version and explicitly maps every
  context to a stable semantic presentation scene ID. Several contexts may share
  a presentation scope. Reentry is distinguished by epoch, never by filenames.
- Lifecycle envelope **3** contains scene metadata and one unchanged protocol-2
  full-replacement plan (`plan.cues`). No ADD mode or per-track sheets. Scene IDs
  are deliberate renderer metadata; graph IDs, world values and asset paths are
  absent from this envelope. Presentation adapters translate the entire envelope
  and correlated scene feedback without importing engine internals.
- Boundary checkpoint **1** contains configuration/story identities, semantic
  history, revision reservation and a validated world snapshot. This is a
  different format from legacy SceneSave, even where both formats use version 1.
  The default filenames differ: `scene-<story>-boundary.json` versus legacy
  `scene-<story>.json`. Loading the wrong format fails; there is no implicit
  conversion of an arbitrary mid-scene save to a boundary checkpoint.
- `WorldStateProvider` supplies typed snapshots and atomic compare-and-swap
  commit/restore. `ProvidedSceneEngine` evaluates existing engine rules against a
  last-committed immutable snapshot and commits validated deltas once. The
  provider owns backing storage; the engine retains sole rule authority.
  `InMemoryWorldState` requires neither a database nor network. External writes,
  retries at a newer selection time and debug mutation are unsupported.

## Lifecycle and time

`Prepare → Ready → Begin → Active → Ending → End → Stable` is explicit. Ready is
sent only after the concrete renderer resolves resources. Preparation errors
keep the clock frozen and may be retried. Begin opens the pending epoch at T=0;
loading, checkpoint menus and outgoing media never consume that epoch's time.

At a GOTO or timeout the existing engine resolves effects and initializes the
pending destination locals atomically. Old offers are locked immediately. The
presentation envelope still names the outgoing source scene and freezes its
selection/expiry timestamp while authorized outgoing narratives finish. Debug
inspection may therefore show pending destination state before Begin. This is
intentional; inspection is separate from player presentation.

The shared host advances renderer-only media elapsed time during Ending. Cue
acknowledgements carry frozen canonical time, never book action cost/exposure,
and cannot change authored outcomes. As in the existing engine, GOTO result
narratives are untimed transition presentations; destination entry schedules do
not execute before Begin. Completed cues are not replayed. Accepted duplicate
lifecycle feedback is harmless; stale events cannot activate a new scene or
reactivate actions, including on reentry.

Relative deadlines start at actual timer-cue execution. Absolute deadlines,
reveal/extend/unset, cutoff, logical eligibility floors and arithmetic cold
accounting remain M4.5 rules. A REPLACE or a checkpoint never restarts a timer.

## Host controls and checkpoints

In the TUI, outgoing presentation finishes before a persistent checkpoint menu
opens. Save/Load are rejected during Active or Ending even if the player pauses;
Continue prepares and begins the next scene. Final endings also close and allow a
completed-game checkpoint. F2 shows scene ID/phase/epoch/readiness, next scene,
provider revision and read-only canonical state.

Redirected/manual-clock execution uses `:continue` at Stable, `:time MS` or
`:advance MS` only in Active, and `:present MS` to progress outgoing automatic
media without moving gameplay time. Manual reading still requires explicit Enter.
EOF acknowledges neither prose nor a boundary; already-active authored deadlines
and automatic outgoing media finish, then the host stops at the persistent
boundary. It does not implicitly Begin a pending successor with closed stdin.

Boundary restore is permitted at Stable or before the initial scene begins. It
replays semantic history offscreen, verifies the stored world/scope against that
replay, and atomically installs the continuation with a fresh revision. No old
scene graphics, effects or expired clocks replay to the live renderer. Unknown
story/configuration versions or inconsistent worlds fail without partial restore.

## Actual text resource catalog

```bash
cargo run -p grimm-cli -- export-text-catalog examples/whiteout/story.yaml examples/whiteout/scenes.yaml examples/whiteout/text.yaml > /tmp/whiteout-text-catalog.json
cargo run -p grimm-cli -- check-catalog examples/whiteout/story.yaml /tmp/whiteout-text-catalog.json
```

An optional `--catalog /tmp/whiteout-text-catalog.json` supplies this registry to
play-scene. The exporter reads actual narrative/action bindings and all-path M3
references. Preparation resolves the explicit `text://<scene_id>` scope into a
replacement cache before Ready. A failed preparation retains the previous cache;
there is no global-binding fallback. Missing text, unsupported resource URIs,
scope mismatch and conflicting exact duration/completion contracts fail clearly.
Unused bindings are warnings. Timed text uses authored exact durations; text
without duration completes through explicit acknowledgement. Callback estimates
are not enforced durations. Graphical asset import/export is an M5 renderer task;
this text validation does not prove mesh/audio/sprite availability.

The architectural semantics are accepted with an explicit API-stability reservation in [ADR-0008](../adr/0008-m46-versioned-lifecycle-and-storage.md). Concrete Rust APIs and serialized contracts remain deliberately versionable through graphical WHITEOUT integration; existing compatibility and migration guarantees still apply.

## Revision and epoch ownership

These counters have distinct jobs and are not interchangeable:

- `SceneEngine` owns its deterministic rule revision and scene epoch. GOTO
  increments the epoch exactly once; provider snapshots use that same scope.
- `ProvidedSceneEngine` owns the immutable last-committed rule snapshot and checks
  provider snapshot equality before every operation. The provider validates the
  expected storage revision atomically, returning a new revision only for a
  persisted mutation/restore. A rejected commit publishes neither candidate
  rules/history nor new world values. No automatic retry may change action time.
- `LifecycleEngine` owns the external envelope revision and maps issued feedback
  to its internal rule revision, scene epoch and issued cue IDs. Scene events and
  rule replacements reserve external revisions; same-plan time/ACK updates need
  not. A restore reserves a revision newer than both current and saved revisions.
  The frozen outgoing envelope may retain the source epoch while the engine and
  provider already hold the pending destination epoch. The mapping is explicit,
  not competing authoritative worlds. Begin switches the visible epoch and
  clears old issued correlations. Stale feedback cannot cross that boundary.

Inspection is read-only; the host never writes a cached inspector snapshot back
into game truth. Provider checkpoints are read only when they still match the
engine's committed snapshot, preventing mixed-revision continuation/world saves.


## Trusted provider conformance

`ProvidedSceneEngine::transact` and `replace_replayed` call the provider before
checking its returned snapshot. These checks detect contract violations; they
cannot roll back writes the custom backend has already persisted. A provider
that writes partially, returns a different snapshot, or writes then reports an
error breaks the atomicity guarantee. The engine withholds candidate/history
publication on detected violations, but the backing store can then disagree with
its last-committed snapshot. Stop and reconcile the backend; do not automatically
retry an action or continue treating that session as healthy.

Before exposing a custom provider, run conformance cases for expected-revision
CAS, no-op revision checks, exact returned/persisted snapshot equality, rejection
without writes, ordered/type/overflow validation, atomic local scope reset and
restore with a fresh revision. The existing injected/InMemory tests exercise
these cases. `contract_violating_provider_is_detected_but_persisted_writes_cannot_be_undone`
explicitly demonstrates the limit for both commit and restore; it is a failure
fixture, not a supported provider implementation.

## Checkpoint history costs and future compaction

A boundary checkpoint serializes full semantic history and a world snapshot.
Restore constructs an offline engine and replays that history from story defaults
before comparing the resulting world/scope and installing it atomically. Save
size grows with recorded events and values; restore work grows with replay and
per-event rule/candidate work. Candidate history cloning can make long replays
more expensive than a simple linear scan. The recorded world snapshot is a
consistency check, not currently a fast path that skips replay.

Restore rejects more than **65,536 history entries** before replay; this count is
not a file-size limit or a promised memory/latency bound. Deserialization and
large individual values still cost resources. Live history/checkpoint creation
are not compacted or capped by that restore check, so hosts running exceptionally
long sessions must track the history budget and avoid producing an oversized
checkpoint they cannot restore. WHITEOUT's tested playthroughs remain well below
this limit. No long-session performance guarantee is claimed by those tests.

Future follow-up (OPEN-M46-HISTORY-01): investigate a versioned trusted snapshot
plus sufficient semantic continuation/cue history, bounded replay since that
snapshot and explicit revision reservation. Preserve deterministic verification,
exactly-once effects, retained/canceled cues and old-save compatibility. Benchmark
representative long histories before choosing a compaction scheme. Compaction is
not implemented or required for this WHITEOUT milestone.

ADR-0008 is **Accepted (architectural semantics; API stability reserved)** by repository maintainer `rjtg` on 2026-11-01. This does not imply graphical renderer readiness or freeze concrete public Rust/serialized APIs.
