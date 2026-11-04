# ADR-0009: Isolated Story Graph foundation for draft 0.3

Status: Proposed implementation decision for review. Date: 2026-11-04.
Authority for this migration slice: maintainer assignment and
[architecture draft 0.3, issue #66](https://github.com/rjtg/grimm/issues/66).
This adds an independently usable foundation; it does not declare the entire
proposal accepted or replace the working scene engine/ADR-0008 in this PR.

## Boundary

Add `grimm-graph` with a sole runtime dependency on `grimm-semantic`. New graph
DTOs live in `grimm_semantic::graph`, separate from legacy scene/schema types.
Neither `grimm-engine`, its WorldStateProvider nor any rules interpreter is a
dependency of the graph crate. Keep existing interpreters and WHITEOUT intact.

The new core validates structural nodes/edges/cue references, owns current node
and entry epoch, emits complete declarative sheets and traverses immediately.
It never evaluates a condition, executes an effect, schedules physical work or
accepts cue ACKs. Host-native logic or a future optional rules library executes
condition/effect descriptors. Host controls disclosure to players.

## Concrete first-step choices

- A versioned graph definition has story identity/version, start and named nodes.
  Each node holds an ordered cue list and a map of named outgoing edges.
- `expose` defaults to true and filters discovery only. Hidden edges and edges
  with unevaluated condition descriptors remain structurally traversable. Every
  sheet carries a named `outgoing_edges` discovery map including opaque condition
  IDs/args, so scheduling and discovery share one node-entry payload. Hidden edges
  never appear in that projection; `exposed_edges()` is only a convenience API.
- IDs are explicit, case-preserving ASCII identifiers. No automatic normalization
  or generated authored cue IDs in this first format. Duplicate IDs and invalid
  targets/traverse-cue references fail construction/parsing.
- Cues have mandatory node-local `at_ms`, explicit `cue_id`, and nested typed
  instruction: presentation, effect or traverse_edge. Presentation duration is
  optional; the host interprets it. Reuse the existing boolean/i64/text scalar
  Value DTO for parameters; no expression or handler codegen dependency.
- Accepted traversal increments epoch once, activates the target immediately and
  returns its full ordered sheet plus a path event. No deferred entry/transition
  FSM; transitional staging is an ordinary node.
- Continuation records graph identity/version, node and epoch. Pure graph restore
  validates it and reserves an epoch newer than current and saved correlation,
  returning declarations without replaying edges/effects. This is not an aggregate
  game save or a guarantee that host effects have completed. A new host instance
  must discard old callback queues or use host session correlation as well.

## Compatibility and migration

Legacy scene protocols/CLI/save behavior stay executable. New schemas have their
own module/version and are not automatically converted. Map rules/effects to an
optional host-side library only in subsequent reviewable refactors. Do not import
the preserved branch of closed PR #65 to provide a world dependency for the graph. If its scoped-provider
refresh work is later retained for optional rules, fix its reviewed completed-
action-control regression before reuse.

The initial proof is LOCKED_IN_SCENE → BREAK_OUT_SCENE → CORRIDOR_SCENE, with the
second edge requested by a cue at 1500ms and a host-executed effect at 1200ms.
The example host uses supplied deterministic times; the graph has no clock.

## Open for later review

Handler manifest/domain parameter types, host cue dependencies and asynchronous
cancel/finish policy, safe snapshot boundaries/effect deduplication, actual host
integration and WHITEOUT conversion remain separate work. No new universal UI
or `host_control` vocabulary, handler generator, decision engine or ticker is
introduced here. See the migration mapping for the next concrete PRs.
