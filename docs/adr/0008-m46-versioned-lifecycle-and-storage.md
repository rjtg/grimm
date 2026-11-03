# ADR-0008: Scene lifecycle, authoritative host state and boundary saves

Status: Accepted architectural direction; concrete API stability reserved.
Accepted: 2026-11-01. Amended: 2026-11-03 for maintainer decisions in
[#63](https://github.com/rjtg/grimm/issues/63),
[#53](https://github.com/rjtg/grimm/issues/53), #25/#36/#37 and the current assignment.
Decision owner: `rjtg`.

This amendment supersedes the original requirement to preserve development save
formats and replay semantic history to restore world state. There are no released
saves. Existing M4.6 checkpoint APIs may be replaced incompatibly; migration is
not required. Story correctness and authored WHITEOUT rules remain required.
Earlier headless/semantic/adapter boundaries remain. ADR-0004's narrative authority
means sole rule interpretation/validation, not an engine-owned duplicate of Bevy
ECS fields. This explicitly refines that distinction for the embedded host.

## Ownership

- GRIMM is the narrative engine: authored conditions, valid decisions, effects,
  semantic cues and transitions. It validates narrative changes and resolves an
  action's logical duration/cutoff once. No rendering or physical timer loop.
- Bevy is the intended first graphical game host: authoritative mapped ECS
  component/resource fields, physical scene timing/deadline detection, scene
  resources, animation/audio, input, rendering and aggregate saves. A fixed-camera
  point-and-click slice is sufficient; no alternative sprite-engine evaluation.
- A thin adapter maps existing semantic IDs/typed fields, invokes GRIMM for
  host-observed events, and translates plans/lifecycle into Bevy operations.
  It does not implement another narrative rule engine or generic host framework.

## Typed world transactions

`WorldStateProvider` must support mapped Bevy component/resource fields selected
by stable logical identity, not persisted raw Entity handles. Read a consistent
immutable snapshot/revision; evaluate narrative rules against that projection;
preflight every typed field/scope mutation and CAS revision; publish all effects,
destination locals and scope atomically under host scheduler exclusion. Failed
validation/CAS publishes neither partial ECS state nor accepted narrative events.

No second independently mutable world store is permitted. Temporary rule
projections/candidates are allowed but are disposable and must derive from the
current authoritative provider snapshot. An explicit revisioned host-write path
must refresh predicates/offers and invalidate stale inputs. Other Bevy systems
may not silently write mapped fields behind a transaction. Presentation-only
state remains separate from narrative truth. The engine crate stays Bevy-free.

Providers are trusted atomic implementations. Snapshot checks after mutation
cannot undo a backend that violates its contract; stop/reconcile such a session,
never retry a selected action at a different time. External provider conformance
requires actual persisted-field tests, not just returned-snapshot assertions.

## Time, cues and lifecycle

Reuse Prepare/Ready/Begin/Ending/End/Stable and one full-REPLACE ordered Cue Sheet.
Resource loading happens before Begin/T=0. Bevy begins/resets/pauses/resumes its
scene time according to GRIMM lifecycle/host pause coordination. It runs physical
deadline timers, reports action ID selected at T or a deadline reached without
selection, and sends presentation ACKs at actual observed times. No frame tick
or periodic countdown event is required by GRIMM.

GRIMM interprets those inputs against authored deadlines. The first Bevy adapter
uses deadline-first ordering at T >= D, independent of frame delivery order,
without changing authored gameplay rules. The existing raw apply boundary does
not universally enforce this tie policy; see the audit/plan. Logical action cost
must not also jump the host clock. Relative timers start on their actual cue
Started ACK; show/reveal never resets them. REPLACE retains started identities;
obsolete/stale actions, timeouts and cue ACKs cannot activate another epoch.

At transition commit, destination local initialization and effects occur once in
the world transaction. Old actions lock immediately. Outgoing media may finish
while outgoing gameplay time is frozen; the destination is inactive until Begin.
At Stable the host captures the committed destination world and continuation,
even though the closing presentation metadata may still name the source scene.

## Host-owned saves, without replay

At every stable inter-scene boundary the host atomically captures an immutable
**boundary snapshot slot**: selected authoritative ECS world/entity identities,
story/config versions and minimal GRIMM continuation. Capture is after transition
effects/destination initialization and before destination Begin. Initial entry
gets an explicit initialized/pre-Begin slot; terminal closure gets a Completed
slot. No destination timer has elapsed. Capture failure blocks Begin and preserves
the previous coherent slot for retry.

Save copies that already captured slot to a persistent user slot atomically.
It neither snapshots current active-scene ECS fields nor evaluates story rules.
For the initial slice Save is disabled while Active/Ending; an eventual
last-boundary Save UI must explicitly communicate progress rollback. Failed copy
leaves the prior persistent slot intact; retry copies only.

Load stages/validates the host world and GRIMM continuation together, then
installs them under host transaction exclusion. It must not replay old actions,
transition effects, destination local initialization or old outgoing media.
Preserve post-transition locals and remaining logical eligibility credit, reserve
fresh input/cue/session correlation, and begin only the saved destination entry
at T=0 after readiness. Completed remains completed. No mid-animation restore.

A semantic scene ID alone is insufficient: context/scope, pending-vs-completed
marker, identity/version, epoch/transition identity, eligibility credit and fresh
correlation/run-ID reservation must be accounted for. The precise minimal
continuation DTO/atomic install API is a prerequisite implementation, not supplied
by this ADR. The host snapshot, not a full event replay, reconstructs world values.

## Semantic history

Keep one chronological semantic journal of accepted actions/resolved outcomes,
relevant times, timeouts and scene transitions/entries. Failed transactions append
nothing. This history is useful for inspection/statistics and may accompany the
host save, but is independent of world reconstruction. Presentation cue ACK
coordination need not become mandatory durable save history. Missing/truncated
optional journal must not change a restored boundary's world or continuation.

## Implementation status and staged change

The merged M4.6 implementation still owns its provider long-term, caches defaults,
rejects changed external snapshots and restores by replay. It does **not** yet
implement the amended ECS refresh or host snapshot-slot contract. Its existing
CLI/save behavior remains operational while those seams are replaced; it is not
an alternative accepted save direction or a compatibility obligation.

This preparation adds only a correlated `DeadlineReached` input and
`LifecycleEngine::deadline_reached` using existing authored timeout rules. It has
no physical clock/Bevy dependency and keeps current replay serialization intact.
It does not supply the new semantic journal, external provider refresh or
non-replay restore. Those require the ordered headless/Bevy proof in the
[audit and #37 implementation plan](../bevy-integration.md).

Concrete API/format changes remain deliberately versionable through graphical
WHITEOUT integration. Save migration tests are not required for development
formats; atomicity, exactly-once effects, typed scope/identity checks, fresh
correlation and gameplay tests are required. No generic renderer framework,
second Bevy story FSM, screenplay DSL, or full graphical game is authorized.
