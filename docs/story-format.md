# Authored stories and saves (M1)

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

Default budgets are 1,024 discovered `(node, variables)` states and 8,192 inspected
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

## Player-visible protocol migration

M0's provisional `SemanticOutput.text: String` is removed. Construct
`content: Vec<NarrativeContent>` instead, with `Narration { text }` or
`Dialogue { speaker, text }`. `SemanticOutput` also has `ending: Option<Ending>`;
consumers detect completion explicitly rather than treating empty offers as an
ending. `GameAction`, `InteractionOffer`, presentation commands and raw input keep
their M0 shapes. The adapter emits the text unchanged, prefixes dialogue with
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
