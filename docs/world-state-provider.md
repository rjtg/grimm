# Scoped authoritative WorldStateProvider — #53 foundation

Implemented on top of main `ad99575` (PR #64). The engine remains Bevy-free.
The deterministic example is `crates/cli/tests/scoped_world.rs`:

```bash
cargo test -p grimm-cli --test scoped_world --locked
```

## Exact APIs and ownership

`WorldStateProvider::snapshot(&self)` now returns
`SceneResult<WorldSnapshot>`. Missing/ambiguous component bindings or failed
reads must return an error, never a fabricated default. This is a deliberate
breaking Rust API change for provider implementations; no story, Cue Sheet or
save wire schema changes. `commit(expected_revision, mutations)` still performs
atomic CAS, including read-only `commit(revision, &[])` without revision advance.
`restore` remains for the existing owned CLI backend, not the new host-save design.

For an externally initialized world:

- `ProvidedSceneEngine::from_provider(story, &mut provider)` returns
  `ProvidedSceneEngine<()>`.
- `LifecycleEngine::from_provider(story, configuration, &mut provider)` returns
  `LifecycleEngine<()>` with the same explicit Prepare/Begin/End lifecycle.
- Both expose `with_provider_access(&mut provider, |bound| ...)`. The bound
  runtime uses `&mut P` through the existing provider trait; the external world
  borrow lasts only for the scope. No provider/host reference is retained in the
  detached runtime. No provider trait is implemented for the detached `()` marker,
  so narrative mutation requires a provider scope at compile time.
- Initialization validates exact globals/locals names and types and the authored
  **initial** context/epoch, then performs a read-only CAS. Initial host values
  and revision may differ from story defaults. It writes/initializes no fields
  and applies no narrative effect. It is **not** a saved-boundary import API.

The runtime holds narrative control state and a private last-validated rule
projection, exposed only immutably. Transaction candidates/projections are
scratch/cache data, not another independently writable world. The host's mapped
fields remain authoritative. `output()`, `engine()` and `world_snapshot()` inspect
the last synchronized projection; they do not silently resnapshot the host.
Bind the same logical game world each time; the host/provider owns that identity.

The original owned-provider constructors remain available for CLI composition.
They retain default-state initialization. There is no separate narrative FSM,
reflection registry, universal entity store, graphics host or timer loop here.

## Explicit refresh and input revisions

After an authorized host transaction advances storage revision, call the bound
runtime's `refresh()` before presenting the changed offers. For example:

```rust
use grimm_engine::scene::{SceneResult, lifecycle::LifecycleEngine,
    state::WorldStateProvider};
use grimm_semantic::{lifecycle::SceneCueSheet, scene::SceneAction};

fn host_refresh<P: WorldStateProvider>(
    runtime: &mut LifecycleEngine<()>, provider: &mut P,
) -> SceneResult<SceneCueSheet> {
    runtime.with_provider_access(provider, |bound| bound.refresh())
}

fn selected<P: WorldStateProvider>(
    runtime: &mut LifecycleEngine<()>, provider: &mut P, input: &SceneAction,
) -> SceneResult<SceneCueSheet> {
    // Use the revision captured from the displayed offer. Do not refresh and
    // silently remap this already selected action onto changed world facts.
    runtime.with_provider_access(provider, |bound| bound.apply(input))
}
```

`ProvidedSceneEngine::refresh()` returns `SceneResult<bool>` (changed or not);
`LifecycleEngine::refresh()` returns the full lifecycle/Cue Sheet envelope.

Refresh requires the same context/epoch/local schema and a strictly newer storage
revision for changed values. Host writes cannot move narrative scope. Unrevisioned
changes, revision rollback, missing/extra/mistyped fields or failed CAS leave the
runtime projection, offers, counters and action history untouched. An unchanged
snapshot still validates with a read-only CAS and creates no new revision.

On success, refresh installs one validated projection and re-evaluates authored
**action predicates** for the action-set instructions already in the current
plan. It retains `eligible` versus explicit-list restrictions, including formerly
empty eligible sets that now gain choices. Fresh `__world_<revision>` action-cue
identities force the renderer to consume the replacement full offer rather than
reuse a completed control ACK. Old selection revisions are invalidated.
Logical eligibility floors and future scheduling/completion dependencies remain;
refresh cannot unlock an action early. Media IDs/statuses/start times and the
existing deadline persist. Reveal/start controls are not replayed.

This is not a reactive presentation scripting system. Outcome selection and
presentation-cue `when` conditions remain materialization-time decisions; refresh
neither reruns an old outcome nor introduces previously omitted narrative/cue
branches. Put dynamic availability on the action's `when`, with an `eligible`
(or explicitly restricted) offered set. Later accepted actions evaluate their
conditions/effects against the newly synchronized values as usual. Refresh does
not observe physical time or resolve action effects.

## Atomic transactions and scope lifetime

Every public bound input is its own transaction:

1. Read/check the current provider projection against the synchronized snapshot.
2. Evaluate candidate rules/effects with checked arithmetic.
3. Commit the complete global/local delta and any `Enter` under expected-revision
   CAS. Destination locals and global effects publish together.
4. Validate the provider result, then publish narrative state/history/cues.

Stale or rejected commits publish no partial narrative candidate or accepted
action. Never silently retry after a conflicting host write; refresh and present
a new offer. The scope itself is **not** a transaction enclosing all its calls:
earlier accepted inputs stay accepted if a later call returns Err. An internal
scope guard copies back the accepted narrative continuation even during unwind;
tests cover both return-error and panic after an accepted action. Provider panics
or contract violations during commit still require host intervention; no engine
can undo backend writes that violate atomicity.

The host must serialize all mapped-field writers and advance storage revision on
authorized writes. Temporary projections/candidates are allowed; a mutable mirror
that independently owns facts is not. Future Bevy composition can borrow concrete
World fields for an operation, release that borrow, then run other systems; this
PR proves the borrowing shape without claiming an actual Bevy scheduler/provider.

## Stable field identities and headless proof

The fixture maps real typed fields, not a second variable map:

| Narrative variable | Authoritative host field |
| --- | --- |
| global.enabled | resource-like `World.enabled: bool` |
| global.count | resource-like `World.count: i64` |
| global.crew_ready | component `Actor.ready: bool`, selected by logical key `crew.jonas` |
| local.marker | scope-local `World.marker: i64` |

Transient entity IDs may change without changing logical binding. Missing or
ambiguous `crew.jonas` fails; it never falls back to another actor. A wrong mapped
type fails before publication. The provider preflights a scratch field candidate
and publishes once under an exclusive `&mut World` borrow. That is a test fixture,
not an ECS implementation or a proposed generic field-mapping framework.

Tests cover external initial values, newly available/removed actions, explicit
list preservation, no early eligibility, stale input/CAS, missing/duplicate/wrong
mapping, unrevisioned writes and invalid scope, overflow, rejected multi-field
writes, source rollback/destination initialization, no duplicate effects, scope
error/unwind, and real adapter/CuePlayer consumption with retained live media and
timer. Existing InMemory/storage conformance and WHITEOUT tests remain.

## Save boundary and remaining work

Scoped/external mode **rejects** the old replay-based provider restore and lifecycle
checkpoint save/restore APIs; an owned runtime that imports an external refresh
also disables that replay path. A history built from defaults cannot reconstruct
externally supplied facts. Raw low-level `engine().save()` is still diagnostic
legacy history, not a valid external-world game save. The default owned CLI
without external refresh retains its existing replay behavior.

This intentionally does not implement #63. Next smallest step: a minimal validated
post-transition/pre-Begin continuation export/import with immutable host boundary
snapshot fixtures, preserving locals, epoch/run identities, eligibility credit and
fresh input correlation **without replay**. Automatic boundary-slot capture,
persistent-slot copy, initial/completed markers, atomic world+continuation Load
and a semantic action/transition journal follow that contract.

#37 still needs the real Bevy provider/exclusive-system composition, mapping and
writer schedule, time/deadline arbitration, pause/readiness, resources, picking/UI
and graphical smoke tests. #53 remains open: this PR implements only its
provider/transaction/refresh foundation, not the complete Bevy adapter.

## Verification

`cargo fmt --all --check`, `cargo build --workspace --locked`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` and
`cargo test --workspace --locked` passed: **217 tests, 0 failures** (210 baseline
plus seven headless provider tests). WHITEOUT story/prose, dependencies and
Cue Sheet schemas are unchanged. Semantic definition/references were checked
before refactoring provider/materialization access. LSP diagnostics for the test
files were initially current/clean; engine diagnostics and later responses timed
out/stale, so full Cargo verification supplies the semantic fallback.
