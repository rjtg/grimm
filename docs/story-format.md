# Authored stories and saves

Play from the repository root:

```sh
cargo run --locked -p grimm-cli -- play stories/the-last-witness.yaml
```

## Story schema 1

Stories deserialize to `grimm_semantic::story::Story`. The CLI handles YAML;
`Engine::new` validates the resulting definition regardless of its source.
The parser is [serde_yaml_ng](https://docs.rs/serde_yaml_ng/0.10.0/serde_yaml_ng/),
which supports YAML 1.1. Quote strings such as `"true"` when you mean text rather
than booleans. Unknown fields, unknown operations and duplicate variable names
are rejected. Nodes and choices use sequences so duplicate IDs can be diagnosed.

A complete minimal story:

```yaml
schema_version: 1
id: example
version: 1
title: An Example
start: entrance
variables:
  courage: 0
nodes:
  - id: entrance
    content:
      - kind: narration
        text: "The door opens."
      - kind: dialogue
        speaker: Keeper
        text: "Will you enter?"
    choices:
      - id: enter
        label: Step inside.
        target: inside
        effects:
          - op: add
            variable: courage
            amount: 1
  - id: inside
    content:
      - kind: narration
        text: "You found a place to rest."
    ending:
      id: rest
      title: A Quiet Night
```

`schema_version` identifies this schema; `version` is the author's positive story
revision. IDs, titles, labels, speakers and prose must be nonblank. Node IDs and
ending IDs are unique across the story; choice IDs are unique within their node.
Every target and the start node must exist. Every node must be structurally
reachable from the start and have a structural path to an ending. Cycles with
exits are allowed. Endings have no choices; non-endings must have choices.
Content is an ordered, nonempty list of `narration` or `dialogue` blocks. Authored
text is preserved exactly: no substitutions, generated text or automatic effects.

## Variables, conditions and effects

Variables are declared with initial values: boolean, signed 64-bit integer, or
string. Their types never change. `variables` defaults to an empty map. Conditions
and effects can refer only to declared variables. No implicit conversions occur.

Every choice has `id`, `label` and `target`; `condition` is optional (always
eligible when absent), and `effects` defaults to an empty list.

| Condition `op` | Fields and meaning |
|---|---|
| `equals` | `variable`, `value`: same-type equality |
| `at_least` | `variable`, integer `value`: inclusive `>=` |
| `at_most` | `variable`, integer `value`: inclusive `<=` |
| `all` | `conditions`: every child is true; empty list is true |
| `any` | `conditions`: at least one child is true; empty list is false |
| `not` | `condition`: negates one child |

Conditions may nest at most 64 levels. For example:

```yaml
condition:
  op: all
  conditions:
    - op: at_least
      variable: courage
      value: 1
    - op: not
      condition:
        op: equals
        variable: courage
        value: 10
```

Effects use `op: set` with `variable` and same-type `value`, or `op: add` with
an integer `variable` and signed integer `amount`. Effects run in authored order
on a candidate copy of the state. Integer overflow rejects the entire action.
Conditions are evaluated before applying that choice's effects; the destination's
offers are evaluated afterward. Content reads and rendering never apply effects.

The engine advertises only executable choices: their conditions must hold,
ordered effects must succeed without overflow, and their destination must be an
ending or have a condition-eligible choice after those effects. `output()` and
`apply()` share the same pure, single-transition preview. Displaying offers never
commits effects. Every advertised choice can be applied successfully while its
state/revision is unchanged. No choices are advertised at revision exhaustion.
Direct proposals for excluded choices still return contextual errors, including
`EffectOverflow` and `NoOffers`, without changing the engine.

The engine commits the new node, variables and incremented revision together.
Initialization and restore reject non-endings with no executable choices and
report the offending choices and preview failures. Existing saves with only
overflowing/deadlocking choices may therefore now be rejected; the format is
unchanged. No other component can commit game state.

Preview is deliberately one transition, not recursive proof that a route will
eventually finish. A destination can have condition-eligible choices whose own
transitions are excluded when that destination is projected. Longer-range
deadlocks and state-changing cycles are not solved globally. If later play
reaches a non-ending with no executable choices, the CLI reports a story error
and exits rather than silently displaying a choice that would fail or treating
the state as an ending. A revision-exhausted checkpoint also has no executable
offers and is handled this way; direct proposals still return `RevisionOverflow`.

## Bounded authoring check (M1.5)

```sh
cargo run --locked -p grimm-cli -- check stories/the-last-lighthouse.yaml
# Optional limits after the filename:
cargo run --locked -p grimm-cli -- check stories/the-last-lighthouse.yaml --max-states 2048 --max-transitions 16384
```

This opt-in authoring step addresses CYOA-09 without changing one-step offers or
requiring a solver during gameplay. Structural validation runs first. The engine
then explores reachable executable transitions breadth-first from the loaded
story's initial state, applying the runtime's pure transition preview to candidate
copies. An author-facing report lists nonterminal states with no executable
offers, their offending choices/targets, shortest discovered node/choice routes,
and the IDs of explicit endings actually observed. Bad authored endings remain
valid terminal outcomes, even when their state reflects a player's mistake.

Default budgets are 1,024 discovered `(node, variables, world time)` states and 8,192 inspected
choices. All choice inspections, including condition-ineligible ones, consume
the choice budget. Exact state equality handles unchanged-state cycles without
letting a hash collision hide a state; variable-changing cycles consume the
state budget. A partially inspected node is not reported as a deadlock merely
because a limit interrupted it. Already discovered findings are still reported.
The CLI returns nonzero on either findings or **inconclusive budget exhaustion**;
it never prints a clean-check result after a cutoff. A zero-state budget examines
nothing and is inconclusive. Limits bound counts, not wall time or input size.

There is no global solvability or termination guarantee. Cycles with executable
moves are not nonterminal no-offer deadlocks, even if their conditionally guarded
exit is never enabled; the report may show no observed endings. The checker
does not synthesize repairs, force progress or silently turn failures into
endings. Raising budgets is an explicit author choice, not an automatic attempt
to enumerate unbounded changing-state loops. Run story-specific playthroughs too.

`Engine::check_playability(PlayabilityLimits)` can also analyze a restored/current
session. Its report is host/author data, never part of `SemanticOutput`. Analysis
does not change canonical state, advance revisions or write saves. Revision is
excluded from narrative-state deduplication; an already exhausted active session
is diagnosed explicitly, but future counter exhaustion is not modeled along
every hypothetical route. `validate_current_state()` supplies contextual runtime
errors. If unchecked play reaches a blocked non-ending, the CLI reports the node,
failed choices/targets and an authoring-check recommendation on stderr, with a
failure exit status and no invented ending. It does not change committed history
or pick a fallback on the author's behalf.

No story/save schema or semantic projection changes are needed. The engine still
depends only on semantic types. YAML, report formatting and terminal I/O remain
in the composition root. The Last Lighthouse uses stable character/location
names and persistent variables as authoring conventions (see its YAML identity
comment and the authoring report), not a newly invented entity registry.

M1.75 extends this checker with fictional time in state identity and an authored
`<timeout>` edge for each timed node. It explores timely choices and expiry, not
each millisecond or presentation phase. Pause/readiness do not create narrative
deadlocks. The notation is diagnostic, never an offered player action. As before,
bounded results are not exhaustive temporal model checking; host-only arbitrary
world advances and future revision-counter exhaustion are not enumerated.

## Player-visible protocol migration

M0's provisional `SemanticOutput.text: String` is removed. Construct
`content: Vec<NarrativeContent>` instead, with `Narration { text }` or
`Dialogue { speaker, text }`. `SemanticOutput` also has `ending: Option<Ending>`;
consumers detect completion explicitly rather than treating empty offers as an
ending. `GameAction`, `InteractionOffer` and raw input keep their M0 shapes.
M1.75 adds an optional semantic decision view and a presentation `Countdown`
command. The adapter emits the text unchanged, prefixes dialogue with
`SPEAKER: `, and appends `Ending: TITLE` on completion.

The protocol is still provisional, with no stable transport negotiation. Existing
Rust consumers must update struct literals and text access as above; M0 had no
persisted story or save files to migrate. Semantic messages now reject unknown
fields during deserialization rather than silently ignoring them. The engine's
`Default` constructor still runs the demo; use fallible `Engine::new(Story)` for
authored games. `ActionError` now includes contextual runtime failures and is
`Clone` rather than `Copy`. Story definitions and checkpoints are
separate host/engine DTOs. They are never attached to visible projections. Those
projections contain only the current node's authored content, eligible offers,
revision and intentionally visible ending metadata, not variables or future nodes.

## Save schema 1

At a choice prompt, enter `:save /tmp/example.json` or `:load /tmp/example.json`.
Loading can also happen at startup:

```sh
cargo run --offline --locked -p grimm-cli -- play stories/the-last-witness.yaml --load /tmp/last-witness.json
```

A checkpoint contains only session data, not the authored graph or UI state:

```json
{
  "schema_version": 1,
  "story_id": "the-last-witness",
  "story_version": 1,
  "node_id": "witness",
  "variables": { "evidence": true, "trust": 1, "promise": "unmade" },
  "revision": 2
}
```

The engine issues checkpoints via `save()` and validates them via `restore()`.
The host chooses the JSON codec and file path. Restore requires a supported
schema, matching story ID/version, existing node, the exact declared variable
names/types and a playable state (or explicit ending). Invalid restore leaves
the active engine unchanged. Restore uses the greater of the saved revision and
the active revision plus one, so loading an older checkpoint cannot reactivate
stale actions. Narrative, offers and variables are restored, while interaction
revisions never go backward within an engine instance. Revision exhaustion is
reported without changing state. Adapter mappings are rebuilt from the restored
projection. Checkpoints can also represent endings through the engine API.

Authors must increment story `version` after incompatible graph, variable or rule
edits. Identity/version matching is not a content hash: same-version edits are
the author's responsibility. Saves are local editable state, not authenticated
history or proof of reachability. No event journal or replay framework is added.
Unsupported schema versions are rejected rather than guessed or auto-migrated.

## Temporal stories (schema 2)

Timing is opt-in; existing schema-1 files are unchanged. Use `schema_version: 2`
and `time: {}` (origin zero), or `time: { start_minutes: 10 }`. All time values are
unsigned integers: world chronology in fictional **minutes**, decision budgets in
**milliseconds**. Negative values fail decoding; zero decision limits fail
validation. Ordinary choices default to zero cost and nodes to no deadline.

A complete small example:

```yaml
schema_version: 2
id: temporal-example
version: 1
title: A Signal in Time
time: {}
start: station
nodes:
  - id: station
    content: [{kind: narration, text: "Investigate now; this prompt is untimed."}]
    choices:
      - {id: tower, label: "Search the tower (15 minutes).", target: platform, advance_minutes: 15}
      - {id: run, label: "Run to the platform (2 minutes).", target: platform, advance_minutes: 2}
  - id: platform
    content: [{kind: narration, text: "Act within five seconds or the train passes."}]
    decision:
      limit_ms: 5000
      timeout: {target: missed}
    choices:
      - id: stop
        label: Stop the train while the fictional window remains open.
        target: stopped
        condition: {op: world_at_most, minutes: 10}
      - {id: withdraw, label: Leave the platform., target: missed}
  - id: stopped
    content: [{kind: narration, text: "The train stops safely."}]
    ending: {id: stopped, title: In Time}
  - id: missed
    content: [{kind: narration, text: "The train passes. This is an authored outcome."}]
    ending: {id: missed, title: Too Late}
```

### World time and conditions

`advance_minutes` is an optional choice cost. The engine checks the choice's
condition before applying its cost/effects, and destination conditions after
them. `world_at_least` and `world_at_most` use inclusive comparisons and can be
nested in the existing `all`, `any`, and `not` conditions. World costs, variable
effects, target and revision commit atomically. Overflow or failed transition
validation changes none of them; such predictably failing choices are excluded.
World chronology never advances just because a person or renderer takes time.

Hosts can explicitly submit revision-scoped `AdvanceWorld { minutes }` at an
untimed, nonterminal node. The duration must be positive and the resulting
offers valid. This is separate from decision elapsed observations, not a general
scheduled-event system. The normal game author uses choice costs instead.

### Authored deadlines and guaranteed consequences

`decision.limit_ms` must be positive. A `timeout` target is required and must
exist. Its optional `effects` use the existing typed `set` operation; arithmetic
and world costs are deliberately not supported for timeouts, so expiry cannot
be prevented by overflowing an accumulator. Use ordinary choices for those costs.

A timeout target must be an explicit ending or a scene with at least one
unconditional, effect-free, zero-cost choice to an ending. This scene may also
have richer choices or its own validated deadline. The rule ensures a usable
fallback without claiming to solve every possible ordinary story route. Endings
cannot have deadlines. Timeout edges count for graph reachability and ending
paths. Timed wait-only nodes may have no ordinary choices: their authored timeout
is still an intentional interaction outcome, not a nonterminal no-offer defect.

### Host protocol, readiness and exact boundary

The engine's `time_input(&TimeInput { revision, event })` accepts explicit events:

| Event | Canonical behavior |
|---|---|
| `Ready` | AwaitingReady → Active, only after the prompt is actually available |
| `Elapsed { total_ms }` | Strictly increasing cumulative active milliseconds since the latest Ready; updates remaining time or commits authored expiry |
| `Pause` | Freezes remaining duration; advances revision and invalidates old inputs |
| `Resume` | Paused → AwaitingReady; advances revision, preserving remaining duration |
| `AdvanceWorld { minutes }` | Explicit checked fictional advance at an untimed node |

Entering a timed node creates AwaitingReady at the full authored duration. The
host must render/flush, then acknowledge Ready, then start its observation
baseline. Renderable offers in this phase require Ready before execution.
`apply()` rejects choices while unready/paused. Paused output has no executable
choices. Ready cannot restart an active countdown. Resume requires another
presentation/Ready and a fresh cumulative baseline at the preserved remainder.

Elapsed is cumulative, **not** a delta: 100 then 200 consumes 200 milliseconds,
not 300. Equal/backward observations fail without mutation. Elapsed inputs while
unready/paused are rejected. The host observes elapsed **before** processing a
player action. At `delta >= remaining_ms`, timeout wins and commits one revision;
the competing old action is stale. A timely action commits first only when the
preceding observation is below the boundary. Repeated expiry or queued old ticks
cannot commit the interaction twice. Overrun never carries into an unseen next
deadline. The core makes no wall-clock guess when only a `GameAction` is supplied:
observation ordering is a required host responsibility, demonstrated by the CLI.

Only the semantic `decision: { remaining_ms, phase }` view reaches adapters.
World minutes, last observation bookkeeping and timeout policies are not in
`SemanticOutput`. `Countdown { remaining_ms, label }` is presentation data with
no authority to expire anything. A host can use a manually recorded sequence for
deterministic replay; no system clock, sleeping, or worker thread enters the core.

### Saving, loading and offline policy

Temporal sessions write save schema **2**, adding:

```json
"temporal": {
  "world_minutes": 17,
  "decision": { "remaining_ms": 12000, "phase": "paused" }
}
```

This is a field of the ordinary checkpoint, alongside story identity/version,
node, variables and revision. It is private host checkpoint data, not a player
projection. Untimed nodes/endings in a temporal story use `decision: null`.

**Closed games consume no time.** A restored Active decision becomes
AwaitingReady with its exact remainder; saved Paused remains Paused. World time
is preserved. Remaining durations must be in `1..=limit_ms`, deadline presence
must match the node, and world time cannot precede the authored origin. All
checks finish before replacement. A temporal restore reserves a revision strictly
above both the current and saved revisions, even in a fresh process, and resets
the observation baseline. Pre-save actions, ticks and lifecycle events are stale
after readiness too. Untimed v1 sessions keep their existing revision policy.

Saving is a pure snapshot, not an implicit Pause. The CLI first observes active
time; if the user continues playing, save I/O and later wall time count normally.
Use `:pause` before `:save PATH` to freeze the decision. Offline time is never
inferred from timestamps, and loading never silently triggers an arbitrary late
timeout. Invalid loads leave the current timer and game state intact.

### Real-time CLI and compatibility

Temporal stories use a host-owned stdin reader thread and bounded channel, plus
one `Instant` observation baseline per active segment. The host waits for input
with a maximum 250ms timeout, submits elapsed observations during silence, and
refreshes the visible indicator roughly once per second. EOF on an active
decision still permits expiry; EOF on an untimed/paused prompt exits without a
choice. Engine-selected consequences remain authored even with no input.
Completed lines carry a timestamp from the same monotonic host clock; that
observation is supplied before their game action, avoiding a needless penalty
for a line already queued while the host handles output. A consequence already
committed at that revision cannot be retroactively undone by a delayed message.

`:pause`, `:resume`, `:save PATH`, `:load PATH`, and `:quit` are global host
controls. Each outstanding read request owns a prompt revision. A line from that
read cannot act on a later timeout/restored revision. After an asynchronous scene
change with an old read still pending, the CLI asks for Enter to acknowledge;
that old line is discarded, then the new prompt is presented/activated. A new
deadline is frozen while waiting for acknowledgement. Normal action/load
transitions finish their read before requesting the next one. Pace scripted input
by waiting for prompts. Input scheduling and canonical terminal buffering are not
hard realtime fairness guarantees; a completed line carries the timestamp at
which the host observed its submission, not a hardware keystroke timestamp.
Ordinary untimed stories retain their existing blocking renderer/input loop.

Existing schema-1 stories, v1 saves and untimed serialized output retain their
behavior and shapes. A temporal story requires schema 2 and a v2 checkpoint;
missing temporal state in a v1 save is rejected rather than fabricated. Rust
consumers of provisional DTOs add `Story.time: None`, `Node.decision: None`,
`Choice.advance_minutes: 0`, `SemanticOutput.decision: None`, and
`SaveGame.temporal: None` to legacy struct literals. `GameAction` and offer shapes
are unchanged; new enum variants require matching the documented temporal
conditions/countdown. Absent optional fields and zero choice costs are omitted
when serializing legacy content. No automatic format migration is introduced.

Limits remain intentionally small: fictional minutes are discrete, timeout
arithmetic is unsupported, active-deadline world advances must be authored action
costs, and analysis is bounded. Counter exhaustion fails transactionally rather
than wrapping. No offline catch-up, event scheduler, NPC simulation, provider,
graphics, multiplayer arbitration or general async framework is implemented.
