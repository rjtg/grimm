# WHITEOUT gameplay bindings (#69)

**Current default (#70):** the canonical [generator-annex excerpt](SCREENPLAY.md)
replaces the old relay demo. The contracts and old regression fixtures below
remain supported. Concurrent presentation instances live in `SceneView.active`;
the latest caption alone remains in `SceneView.presentation`. The actual Bevy
resource also owns `deicer_isolated`, `engineering_lost` and `powered` status.
New typed effects are `prepare_annex`, `isolate_deicer`, `start_generator` and
`seal_annex`; `can_start_generator` requires isolated de-icer and open access.
REPLACE cancels all cue-owned sound/motion entities. No graph-core API changes.

Based on main `87db412` (PR #73). Architectural authority:
[#66](https://github.com/rjtg/grimm/issues/66), assignment
[#69](https://github.com/rjtg/grimm/issues/69). This is a small extension of the
existing generator graph, not the full WHITEOUT migration.

## State and ownership

The existing Bevy `Generator` resource now lives in `src/gameplay.rs` and owns:

- `status: GeneratorStatus`: running, unstable, isolated, stopped, repaired.
- `has_spare_part: bool`: the one relay needed by this demonstration.
- `applications: u64`: diagnostic successful-effect count, retained from #68.

There is no inventory framework, variable map, copied mutable world or provider.
All condition reads and effect writes use the actual resource. Bevy's scheduler
holds the resource borrow during execution; no other system can interleave a
write between the final condition check and `traverse_edge`. Game-system writes
should use normal Bevy change detection and finish before the Execute phase.

GRIMM still owns only graph position/epoch and declarations. It never evaluates
these handlers, sees canonical ECS values, or executes an effect. Runtime,
`SceneView` and UI widgets remain host presentation/execution projections.

## Typed condition and effect contract

[handlers.yaml](assets/handlers.yaml) explicitly declares names, parameter names
and domain types for later codegen/tooling. Runtime dispatch is a concrete Rust
`Condition`/`Effect` enum and functions, not an interpreted manifest/DSL or generic
registry. There is no code generation here.

| Kind / ID | Exact args | Behavior |
| --- | --- | --- |
| condition `can_repair_generator` | `required_item: ItemId` | Requires carried spare part **and** unstable generator |
| condition `can_take_spare_part` | none | Requires running generator and no carried part |
| effect `set_generator_status` | `status: GeneratorStatus` | Sets status, preserving carried part; used by existing warning/isolation/timeout cues |
| effect `take_item` | `item: ItemId` | Takes the spare part, rejects duplicate ownership |
| effect `repair_generator` | `required_item: ItemId` | Rechecks repair prerequisites, then consumes the part and sets repaired status together |
| effect `reset_demo` | none | Sets running status and clears the part for another attempt |

`ItemId` accepts only `spare_part`. `GeneratorStatus` accepts the five values
listed above. Parsing rejects unknown IDs, wrong scalar types, missing/extra
arguments and unknown domain values. No integer/bool-to-string coercion. Startup
validates every authored binding, including hidden edges. Runtime also validates,
so stale/unvalidated descriptors never become a permissive default.

Manifest tests exercise every declared ID/domain value, missing/extra/wrong-type
parameters and every story reference. Types are explicitly declared, never
inferred from example literals. The manifest is preparation for future typed
codegen, not a second gameplay rule source.

## Availability and stale input

`expose` remains discovery only. The full sheet carries discoverable definitions;
the host's `blocked_reason(edge, &Generator)` computes current availability.
The UI displays locked edges with a reason and keeps their number slots stable.
Keyboard events carry the selected semantic edge ID and entry epoch, not an
approval from the last displayed frame.

The UI recomputes conditions whenever the generator resource/view changes.
Immediately before accepting a player traversal, the central #68 arbiter calls
the same condition handler on the **current** resource again. A missing part
blocks repair even when a stale frame offered it. A due world-changing cue in
the same frame is processed in authored time/order before a simultaneous input,
so the final check also sees that mutation. No explicit refresh API, offer-ID
rewriting or independently mutable offer state is introduced.

A false condition rejects that selection without mutation and leaves other
choices/timeout available. An invalid condition descriptor is a binding error,
not false/true: validation reports it; runtime stops the entry visibly through
the existing error path. Hidden edges remain absent from player discovery, but
scheduled/internal traversals still use the normal structural GRIMM API.

## Scheduled writes, atomicity and existing timing

Only an authored effect cue calls the handler. Accepting `take_spare_part` or
`repair_generator` itself does **not** mutate inventory/status:

- `GENERATOR_SUPPLIES`: take at 600ms, hidden return at 900ms.
- Existing warning: status at 2000ms, controls at 5000ms, timeout at 30000ms.
- `GENERATOR_REPAIR`: repair at 1200ms, hidden continuation at 1500ms.
- Existing isolation: isolate at 1200ms, continue at 1500ms.
- `GENERATOR_RESET`: explicit T=0 reset cue, hidden return on the next pass.

Handlers parse and preflight prerequisites plus the checked application counter
before assigning any field. Failure cannot consume a part without repairing the
generator, update status without advancing the counter, or silently continue to
a success scene. Earlier successful effects remain committed; graph traversal is
not rolled back if a later destination effect fails. #68's visible fail-closed
entry behavior is preserved.

The scheduler remains responsible for `(epoch, cue_id)` deduplication, stale
callback rejection, full REPLACE, real scene-time sampling and timeout-first
arbitration at equality. Duration is presentation metadata; effects remain
synchronous. No new timer/lifecycle/ACK semantics, save/restore or async engine.

## Routes and verification

See [README](README.md#play-and-manual-smoke-test) for exact keys and timing.
Repair restores power; isolation protects the damaged drive without restoring
power; waiting causes the authored blackout. Both pickup and repair update the
visible resource display only when their scheduled effect actually executes.

The #68 scheduler regression suite uses its original story-version-2 fixture
(`src/scheduler/fixtures/scheduler-v2.yaml`), preserving its timings, counts and
assertions. It tests the same production scheduler/handlers as the extended
version-3 authored story (graph schema remains 1). Gameplay tests
exercise the actual current graph, typed failure paths, stale displayed offers,
same-frame effects and full playthroughs. All tests use deterministic Bevy time,
without a window/GPU or sleep.

## Reuse and remaining scope

PR #65's scoped provider/CAS/refresh and the legacy expression interpreters were
considered. They do not simplify this one-resource, two-field game: typed Rust
predicates and preflighted resource writes are smaller and need neither a world
mapping framework nor a script interpreter. Nothing from #65 is required/imported;
its known refresh issue is not inherited.

Full staged WHITEOUT (#70), concurrent media (#41), host snapshots/session
isolation (#63), larger inventory/skills, handler codegen and optional rules
extraction remain separate. This manifest is deliberately game-specific.
