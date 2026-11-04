# Draft 0.3: first graph migration step

Target: [issue #66](https://github.com/rjtg/grimm/issues/66), as explicitly requested
by the maintainer assignment. Baseline: main `ad99575` (merged PR #64). PR #65 was closed without merge as superseded by #66 and is **not included**. The draft is still proposed; this additive foundation
is reviewable independently of changing accepted legacy behavior. Boundary choice:
[proposed ADR-0009](../adr/0009-story-graph-03-foundation.md).

## New modules and exact API

- `grimm_semantic::graph`: `StoryGraph`, `Node`, named `Edge`, `ConditionCall`,
  timed `Cue`/`Instruction`, `CueSheet`, `StoryContinuation`, `PathEvent` and
  `TraversalResult`. Its parameter values reuse the existing scalar DTO
  (`bool`, signed i64, text), not the expression/effect interpreter.
- `grimm-graph`: `GraphEngine::new(graph)`, `current()`, `exposed_edges()`,
  `traverse_edge(edge_id, expected_epoch)`, `snapshot()` and `restore(continuation)`.
  Its only runtime dependency is `grimm-semantic`; YAML/JSON are dev/example
  dependencies. Cargo's existing boundary-policy test now checks the new crate,
  including rejecting a dependency on the old engine, adapters or Bevy.

`new` validates graph schema/story identity, unique nodes/cue IDs, start/targets,
edge IDs, descriptor IDs/parameter names and current-node traverse-cue references.
Parsing rejects duplicate named edges, unknown fields/kinds and negative times.
IDs are explicit nonempty ASCII tokens (letters/digits plus `_`, `-`, `.`, `:`),
case-preserving and independent of filenames. No normalization or generated cue
identifiers are introduced. Edge/cue names are scoped to their source node;
reentry is distinguished by epoch. The same edge name may occur on other nodes.

`expose` defaults to true. Every `CueSheet` includes a required
`outgoing_edges: BTreeMap<String, Edge>` discovery projection with condition IDs
and args, alongside node ID, epoch and the full cue list. Initial `current()`,
traversal and restore all deliver this coherent node-entry payload; the map
replaces the previous discovery set, including when empty. Hidden edges are
omitted. `exposed_edges()` remains a convenience read of the same projection.
This discovery metadata does not evaluate, authorize or imply player UI.
A hidden edge is traversable exactly like a discovered one. An unknown condition
handler is not a graph construction/traversal error; handler validation belongs
to later tooling. A host must filter what it shows to players itself.

A successful traversal immediately changes current node and increments epoch
once, even on a self-loop, then returns that target's **entire ordered cue list**
and a path event. No readiness or cue ACK precedes activation, no `enter_next`,
`advance` or transition commit API exists. Stale epoch, non-current edge and epoch
overflow are errors with no state change or accepted path event.

Every `current` read returns the complete declaration again; this core never
consumes a cue. On a changed node/epoch the host replaces/cancels the old pending
plan. Cue order and exact node-local millisecond times are preserved; the core
neither sorts by time nor schedules playback. Optional presentation duration is
data for the host, not a promise of actual completion. Effects/traversal cues do
not have implicit execution or runtime side effects when a sheet is produced.

The serialized new graph definition, new sheet and graph continuation each have
an explicit version **1** in their separate `graph` module. The pre-merge
review adds required `outgoing_edges` to the new sheet contract; update draft
host consumers/struct literals to read or populate it (no default silently
accepts an incomplete old payload). This unreleased draft stays at version 1.
These versions are separate from legacy scene schema/protocol 1. Presentation cues use an opaque `presentation_id`;
effects use `effect_id + args`; traversal cues reference a named current-node
edge. A nested `instruction` follows the existing cue DTO convention. The
illustrative flat syntax from issue #66 is not an alternative parser here.

## Existing functionality → new responsibilities

| Current source/function | What it actually does today | Target responsibility / migration |
| --- | --- | --- |
| semantic scene `SceneStory`/Context/Action, scene compiler | Conditions/effects/duration/outcomes and graph targets in one schema | Preserve existing authoring for compatibility; new pure graph schema is explicit. Later adapt one scene manually, exposing edges/condition descriptors and host-owned scheduled operations; do not invent an automatic equivalence converter |
| `SceneEngine::act`, `allowed`, `resolve` | Checks offers/predicates, evaluates action cost/outcome, mutates globals/locals and transitions | Structural traversal is in new GraphEngine; host decides edge and timing. Standard predicates/cost/branch/effect helpers can later become an optional host library; no mandatory decision engine |
| scene `expressions::{eval,check,update}` and top-level expression interpreter | Existing typed expressions, tests, set/add and checked arithmetic | Retain implementations/tests; extract reusable handlers in a separate PR without a dependency back from graph to rules |
| `SceneEngine::materialize`, `output`, semantic scene CueSheet, adapters | Materializes state-dependent cues and carries pending/started/completed controls across REPLACE | New graph emits full declarations and no execution state. Keep old path for legacy stories; host executes, replaces/cancels, deduplicates and decides whether in-flight work finishes |
| `SceneEngine::{observe,expire,ack}`, authored timers/cutoffs | Resolves deadlines at reported times and coordinates timer/cue ACK state | Host timing/execution. New timeout is a scheduled ordinary traverse_edge cue; countdown is host presentation, no ticker/logical round counter |
| `LifecycleEngine` prepare/begin/ending/stable | Owns entry/exit barriers while target locals may be initialized before Begin | Retain for old CLI. New core activates target immediately; host readiness/loading and safe-save points remain host concerns. Transitional staging uses ordinary nodes, not another core scene FSM |
| WorldStateProvider / ProvidedSceneEngine | Typed snapshots, expected revision, transactional narrative state writes | Optional host-side rules/handler integration. No WorldState access in new core; host-native effects may bypass that optional library entirely |
| existing M4.6 checkpoint/restore | Replays full semantic history from story defaults and compares world | Host-owned coherent snapshot of world + graph continuation + necessary cue/effect execution state, without replaying old edges/effects. Actual host snapshot coordination remains future #63 work |
| ActionRecord/lifecycle history | Mixed decision/time/presentation/replay bookkeeping | Optional path recorder can consume returned PathEvent; graph owns no log. New event sequence uses monotonic destination epoch, permitting gaps on restore |
| CLI `SceneSession`, CuePlayer, renderer and TUI | Host scheduler, media completion, offer display, saves and input | Preserve as working legacy host. Reuse useful host execution/inspection pieces in later adapters, not as a graph-core dependency |
| WHITEOUT story/text and tests | Authored rules, multiple endings, cold accounting, overlapping generator cue, save continuation and CLI proof | Golden legacy behavior stays tested. Later migrate one generator/transition sample with explicit host effect timing before whole-game conversion |

The new semantic descriptors deliberately carry no world lookup, callback,
condition evaluator, expression bytecode, effect implementation or clock handle.
The host may trigger an edge from pointer input, combat/physics/RPG state or a cue
through the same `traverse_edge`. No generic player-choice API is implied.

## Reference graph and runnable host proof

Source: [locked-in.yaml](../../examples/graph-foundation/locked-in.yaml).
Run from the repository root:

```bash
cargo run -p grimm-graph --example locked_in --locked
```

```text
LOCKED_IN_SCENE epoch=0
BREAK_OUT_SCENE epoch=1
host T=0: present break_door
host T=1200: door becomes broken
host T=1500: CORRIDOR_SCENE epoch=2
```

BREAK_OUT_SCENE is an ordinary node. It declares an effect at 1200ms and a
traverse_edge cue at 1500ms targeting hidden `continue`. The example host performs
its simple game-native effect, then calls the normal traversal operation. It
supplies fake timestamps; no physical timer, WorldStateProvider, optional rules
library or ACK is needed. Core traversal alone cannot set the host's door state.

## Continuation and host snapshots

`StoryContinuation` contains version, story ID/version, node ID and epoch. Restore
validates identity/node and reserves `max(current_epoch, saved_epoch) + 1`, then
returns the full current declaration. It replays no path, conditions or effects.
This fresh epoch invalidates callbacks from prior entries in the same runtime;
an epoch cannot wrap. No path event is generated by restore.

This is **only graph position**, not a complete game save. The host must bracket
world writes, settle required prior effects and choose a coherent snapshot point,
including host-owned pending/completed effect/media state as needed. The graph
has no cue ACK information from which to certify a safe boundary. A simple later
snapshot policy can capture after required source effects and before first
new-node cue execution; this PR does not implement that policy or copy/load slots.

The host must also isolate queues/session correlation when replacing an entire
runtime, and decide how restored execution records map to the freshly reserved
epoch. Do not blindly execute a returned sheet and assume no host effect has
already happened. These host restore/dedup rules remain explicit follow-up work.

## Preserved PR #65 reference

The branch for this work starts at main, not #65. None of #65's provider/refresh
implementation or schema is merged/cherry-picked, and it is not required to use
the new graph. Its scoped borrow, fallible typed reads, logical field mapping,
CAS/preflight and unwind tests remain candidates for an **optional host rules
library**. They must not become mandatory graph dependencies.

The current review finding is acknowledged: completed action controls are
reissued by `refresh_offers`, so an older consumed control can overwrite a newer
live offer. If that implementation is adopted later, first add the multiple-
action-control regression and preserve completed statuses/live-offer ordering.
PR #65 is closed without merge; its branch and tests remain a reference. This
migration does not claim that finding is fixed or import the superseded work.
Its ownership-policy comments do not create a universal policy for the new graph;
condition/effect handling is supplied by the concrete host.

## Next concrete PRs and open choices

1. Review the new structural/cue/continuation contract and proposed ADR-0009.
   Resolve host cancel/finish and safe snapshot policy on one reference host,
   preserving epoch-based stale traversal rejection. Decide any pure dependency
   declarations or host-control convention from that example, not a second FSM.
2. Extract existing expression/condition/effect and typed transaction helpers with
   their regression tests into an optional host-side rules library. Evaluate #65
   separately; fix its completed-control regression before reusing refresh. Do
   not add rules dependencies to grimm-graph.
3. Convert one WHITEOUT generator/transition path using host-side condition/effect
   handlers and ordinary transitional nodes. Compare outcomes/clues/timing to
   current WHITEOUT tests; scheduling effects instead of commit-at-selection is
   a semantic migration that needs explicit review, not a parser-only conversion.
4. Continue with the concrete WHITEOUT host: #67 bootstraps the Bevy application,
   #68 owns cue scheduling/effects/timed traversal, #69 binds gameplay conditions
   and effects, and #70 demonstrates the complete slice. The superseded #53/#37
   adapter plan is not a dependency of this core. Reconcile coherent host snapshots
   from #63 with actual effect/cancellation state before adding save slots. Handler
   manifests/domain parameter types and optional Rust codegen follow a demonstrated
   handler API in separate PRs.

Open: custom/domain parameter types vs today's bool/i64/text, handler registry
validation, optional cue dependencies, variable-length effect completion and
cancellation, effect deduplication at host resume, snapshot/session correlation,
UI/control conventions. No full save overhaul, Bevy application, screenplay DSL,
handler generator, universal decision engine, ticker or legacy-engine removal.
