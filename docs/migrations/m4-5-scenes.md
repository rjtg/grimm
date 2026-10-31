# M4.5 scene timing migration

This revision preserves the existing dependency boundaries. Engine rules depend
only on the semantic crate; the adapter maps to presentation types; the terminal
renderer has no engine/semantic dependency. Ratatui is confined to CLI and text
renderer. The host coordinates time, files and read-only debug snapshots.

## Authored schema and checkpoints

Scene DSL schema **2** starts every context with a running clock at T=0. GOTO
starts a new epoch implicitly. There is no authored mid-context restart command.
Schema **1** remains accepted with its original timer-start bootstrap behavior.
Legacy M0–M1.8 story schemas and checkpoints use their existing separate loader.

The shipped WHITEOUT and Locked Room stories (including the detective fixture)
are version 2/schema 2. Their exact original rule and text files are archived in
`examples/whiteout/legacy-v1` and `examples/locked-room/legacy-v1`. Load a previous
checkpoint using its matching archived story, for example:

```sh
cargo run --locked -p grimm-cli -- play-scene \
  examples/whiteout/legacy-v1/story.yaml \
  examples/whiteout/legacy-v1/text.yaml --load OLD_SAVE.json
```

New checkpoints use schema 2, the matching story version and the same semantic
replay approach. A mismatched schema/story version is rejected transactionally.
There is no automatic old-clock conversion: presentation elapsed time that never
advanced under the old clock cannot be safely inferred. Existing schema-1
content-derived cue IDs retain their original hash algorithm. Editing timing or
new cue content deliberately changes generated IDs; prefer explicit author IDs.

## Deadline instructions

```yaml
schema_version: 2
# Within a context's cues:
# at_ms is absolute scene time; omitted author time lowers to sequential order.
cues:
  - {cue_id: deadline, at_ms: 0, kind: timer, set_deadline_ms: 30000, visible: false}
  - {cue_id: intro, at_ms: 0, kind: narrative, narrative_id: arrival, duration_ms: 4000}
  - {cue_id: choices, kind: actions, available_actions: eligible}
  - {cue_id: warning, at_ms: 10000, kind: narrative, narrative_id: warning, duration_ms: 2200}
  - {cue_id: reveal, at_ms: 10000, kind: timer, visible: true}
```

`set_deadline_ms` sets a relative deadline at instruction execution; `start_ms`
is its legacy accepted alias. `deadline_at_ms` sets an absolute scene timestamp.
`extend_ms` adds to an existing deadline. `visible` (authored alias
`show_deadline`) changes visibility only. `unset_deadline: true` removes the
trigger and hides the deadline without stopping scene time. Setting, absolute
setting, extension and removal are mutually exclusive in one instruction.
An authored timeout remains required for a context that can establish a deadline.
New absolute/removal instructions require schema 2. Deadline expiry consumes the
deadline once, including a timeout that stays in the same context.

A successful generator startup in WHITEOUT emits removal as its first result
control, before the startup presentation. Choosing an unrelated latch action
emits no removal; REPLACE preserves the existing deadline. Crossing the deadline
resolves the authored interruption before normal result controls can execute.
An action completing exactly at the boundary may remove its deadline; an idle
observation at that boundary expires first. No action duration is charged twice.

## Emitted protocol 2 and renderer feedback

Every emitted Cue Sheet has `protocol_version: 2`. Every emitted cue has a
numeric, mandatory `at_ms`, plus optional `after_cue` and `started_at_ms`.
Authored omitted times lower to the selection timestamp (or new-epoch zero),
with completion dependencies preserving sequential order. Explicitly scheduled
cues are independent of list position; explicit `after_cue` remains a barrier.
Nonempty action offers also respect the logical action-duration eligibility floor.
Timed GOTO result instructions remain forbidden; use destination entry cues.

Narratives may declare `duration_ms`. Automatic completion is due at actual
semantic start plus duration, not original authored schedule time if a dependency
delays the start. Retained running cues carry their original `started_at_ms`,
including after restore, so REPLACE does not extend playback or restart media.
Cues without a duration require explicit completion. Repeated feedback is safe;
revision and epoch checks protect controls and actions from stale input.
Protocol-2 instantaneous controls complete semantically on Started; an explicit
Completed remains a safe duplicate. Schema-1 control feedback retains its prior
lifecycle for replay compatibility.

The synchronous host processes observations chronologically: deadline first at
an equal timestamp, then automatic narrative completions, then newly due starts.
A new epoch discards leftover observation time from the previous epoch. Action
selection freezes the host clock while rules and immediate control instructions
are evaluated; presentation resumes from selection T, never T plus authored cost.

Presentation-only revision history is bounded to 32 sheets. It retains completed
and replaced/cancelled lifecycle rows, independently of the canonical replay log.
Scrollback is renderer-owned and bounded to 512 text entries. Neither is a source
of game truth or a substitute for semantic execution history.

## Host-only world inspection and future mutation design

`SceneEngine::inspect_world()` returns a cloned `WorldStateSnapshot` containing
context ID, epoch, revision, typed globals and typed locals. It has no side effects.
The host converts it to textual read-only debug rows; Cue Sheets never contain
world variables or graph data. Changed values are highlighted in the inspector.

No state editor is implemented and no F3 editor is advertised. A safe future
editor would accept a typed, scoped transaction, validate every reference/type
before mutation, recompute eligibility on a candidate engine, then atomically
commit both state and a recorded DebugTransaction event. That needs an explicitly
versioned checkpoint format and a persistent debug_modified flag. Rejected
transactions must leave state, revision, timer and history unchanged. An
alternative is to disable checkpoint export for marked debug sessions. Direct
unrecorded writes, timer manipulation and arbitrary DSL execution are excluded.
