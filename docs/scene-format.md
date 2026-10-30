# M2 scene DSL and Cue Sheets

The scene DSL is an additive format selected by `play-scene` / `check-scene`.
Legacy `play` / `check`, story schemas 1–3 and their checkpoint formats are
unchanged. Scene DSL `schema_version: 1` has its own checkpoint schema and must
not be passed to the legacy loader. See the executable
[Locked Room](../examples/locked-room/README.md) for a complete authored game.

A story declares `id`, `version`, `title`, `start`, `globals` and `contexts`.
Each context has an ID, optional namespace, typed `locals`, entry `cues`, actions,
optional `timeout` outcome and optional host-only `terminal: true`. Transitions
are explicit `goto` outcomes. Entering a context resets its locals and scene
clock; globals persist. Terminal status is never a presentation instruction.

Variables are bool, signed 64-bit integer or string. References explicitly use
`global.NAME` or `local.NAME`. An action has an ID, optional `when`, nonnegative
`duration_ms`, ordered base `effects` and `outcomes`. Each outcome list is ordered
IF/ELSE IF/ELSE: `when` guards followed by one unconditional final fallback.
Effects are ordered `set` or integer `add`, with `var` and `value`. Expressions
are literals, `{var: global.NAME}`, or `{calc: add, args: [EXPR, EXPR]}`; arithmetic
also supports `sub`, `mul`, `div`. Overflow and division by zero reject the whole
transaction. Conditions use `op: eq/ne/lt/le/gt/ge`, `left` and `right`, or
`all/any` with `tests`, or `not` with `test`. All/any short circuit.

Clock expressions use `{clock: selection_ms}`, `time_passed` (logical action
end), `duration_ms`, `effective_ms`, `deadline_ms` and `interrupted`. Durations and
clock values are integer milliseconds. Integer division truncates toward zero.
These permit authored partial effects such as strength times effective duration
divided by full duration. For deadline interruption, `interrupted_outcomes`
replace normal base effects and outcomes; absent cases use the context timeout.
An idle deadline observation also resolves that timeout. Exact-boundary normal
success/failure policy can be authored using logical end and deadline.

Cues have optional `cue_id`, `at_ms`, `when`, and one instruction:

```yaml
cues:
  - {cue_id: intro, kind: narrative, narrative_id: search_room_start}
  - {cue_id: start_clock, kind: timer, start_ms: 30000, visible: false}
  - {cue_id: choices, kind: actions, available_actions: eligible}
  - {cue_id: radio, at_ms: 10000, kind: narrative, narrative_id: radio_johnny_returns}
  - {cue_id: reveal, at_ms: 10000, kind: timer, visible: true}
```

`available_actions` is `eligible` or an explicit ID list, and always replaces the
entire offered set. `[]` locks input. Timer updates can set `visible` or add
`extend_ms`; a timer starts only when its start instruction executes. Timed
contexts must declare an unconditional sequential timer start and a timeout.
Untimed instructions execute in order after completion acknowledgements.
Explicit `at_ms` uses absolute current-scene time and may overlap running media.
An action release is never earlier than selection plus effective duration;
implicit sequential releases additionally wait for preceding presentation. A
transition resets the scene clock to zero and rebases the selected action's
effective duration onto that new clock, so it cannot unlock new-context actions
early.
Explicit transition results should contain narrative or input-lock cues; place
new offers and timer starts in the destination's entry plan.

Identifiers normalize ASCII letters to lowercase and spaces/hyphens to underscores;
invalid components and collisions fail compilation. Context namespaces qualify
local cue, action and narrative IDs. Already qualified narrative bindings can be
shared across contexts. Explicit cue IDs are recommended for long-lived media.
Otherwise content, condition and authored timing determine a versioned hash ID,
independent of list position. Identical generated occurrences need distinct
explicit IDs. A repeated action result gains a reserved occurrence suffix;
retained running media keeps its prior identity. Reserved runtime identifiers
cannot be authored. Changing ID derivation requires a schema/migration review.

Every output is a full replacement Cue Sheet: revision, scene-clock coordination
and one ordered cue list. No graph, variables, prose, asset paths or ending marker
crosses this boundary. Each output cue has an ID, optional absolute time and
optional `after_cue` dependency plus its instruction. Dependencies preserve author
order when a sequential action cue also needs an absolute eligibility floor.
Completed media disappear. The current executed offer instruction may remain as
an idempotent offer snapshot so a fresh renderer can reconstruct controls.

The renderer owns progression and pauses at selection T during evaluation. The
engine commits rules immediately and returns the new plan; presentation resumes
at T, without jumping by duration. Host observations drive idle expiry. Semantic
Started/Completed acknowledgements are idempotent. Old revisions cannot start
controls or submit choices; an old callback may only complete retained, already
running media in the same scene epoch. Replacement removes obsolete pending
cues and does not replay an executed timer or radio instruction.

Scene checkpoints store story identity/version, revision and a semantic replay
log of accepted actions, cue acknowledgements, clock observations and restore
epoch reservations. Consecutive idle observations coalesce. Restore validates the
log transactionally and reserves a fresh revision, invalidating earlier callbacks.
Offline time is frozen. No animation frame, camera or renderer asset is saved.
Started unfinished prose can be shown again by a fresh text renderer. Checkpoints
are trusted host data, not a player-facing projection or a tamperproof format.

Validation checks types, IDs, branch fallback structure, references, timer setup
and structural reachability. It does not prove state-dependent solvability or
termination. Conditions are evaluated when a plan is built and actions are
revalidated at selection; hosts must submit accurate monotonically increasing
scene times. This release does not implement M3 indexing, lint tooling, dynamic
node providers or a general scripting language.
