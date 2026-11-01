# M4.6 — Scene Lifecycle, World State Boundary and Renderer Asset Contract

**Status:** Proposed architecture; not implemented. **Prerequisite:** M4.5 merged. **Scope:** architecture needed before WHITEOUT's graphical adaptation. See [M5 production plan](m5-whiteout-production-plan.md).

## Goals

Make semantic scene transitions observable to the renderer, make world-state storage replaceable without sacrificing determinism, and define how a renderer advertises its available scene/presentation assets. **Preserve** GRIMM's one ordered full-REPLACE Cue Sheet, stable `cue_id` vs reusable narrative/presentation ID, `at_ms`, optional `duration_ms`, `after_cue`, authoritative authored actions and deadline-cutoff policy.

GRIMM is a narrative orchestrator, **not** an animation player, camera rig, 3D scene graph or asset importer. The 3D/2D renderer owns all physical motion, choreography, camera, audio and resource dependencies.

## 1. Explicit scene lifecycle

The host and renderer must distinguish **Prepare Scene**, **Begin Scene**, **End Scene**, and **Stable Boundary**; scene ID is a *semantic presentation scope*, not a filesystem path or renderer instance ID.

```text
[Scene A running / T increases]
          |
  authored transition committed
          |
[End_Scene(A) pending] --renderer ACK--> [Stable boundary / checkpoint permitted]
                                                 |
                                          host preloads Scene B
                                                 |
                                        renderer confirms ready
                                                 |
                                          [Begin_Scene(B)]
                                      new epoch, T=0, clock runs
                                                 |
                                      timed presentation and choices
```

**Important ordering:** The exact transition transaction and when engine effects become canonical must be defined, tested, and recoverable. No half-committed transition or ghost old-scene action. `End_Scene` is a lifecycle barrier, not a media cue whose completion can change gameplay conditions. A transition may lock choices immediately while an outgoing presentation completes. Renderers ACK explicit lifecycle instructions; redundant/stale ACKs remain idempotent. A terminal ending also closes its scene.

**Engine clock authority:** `Begin_Scene` starts a fresh epoch at T=0, even without a deadline. Loading/preparation occurs **before** T=0 and costs no scene time. `End_Scene` freezes the outgoing epoch; it must settle authored elapsed-time effects and any in-flight deadline/cutoff deterministically. No unrestricted mid-scene reset.

**Deadline semantics:** `set_deadline_ms` starts a relative deadline when its cue **actually starts**: `deadline_at = started_at_ms + duration_ms`. `deadline_at_ms` declares an explicit absolute scene timestamp. `unset_deadline` removes the expiry without stopping the scene clock; `visible/show_deadline` only controls the display. A delayed `after_cue` must not silently backdate a relative deadline. Actions crossing a deadline resolve their cutoff **before** result cue cancellation. No implied timer restart from REPLACE, visibility, unrelated choices, preloading or save/restore. This preserves M4.5 semantics while making lifecycle explicit.

**Scheduling:** One ordered Cue Sheet (no separate per-track sheets). A scene preamble belongs to the boundary/prepare envelope, not as a `T=0` timed cue that could start before loading completes. Emitted scheduled cues have numeric `at_ms`; optional `duration_ms` is a presentation contract. Completion ACK is authoritative for variable-length presentations; `after_cue` can wait. Retained same-ID media continues across REPLACE.

### Illustrative protocol envelope (NOT valid current YAML)

```yaml
scene:
  scene_id: generator_annex
  presentation_scope: whiteout.generator
  epoch: 12
  phase: prepare
  preload: [whiteout.generator] # logical presentation scope, not asset files
# After renderer acknowledges readiness:
lifecycle: {kind: Begin_Scene, scene_id: generator_annex, epoch: 12, at_ms: 0}
cues:
  - {cue_id: intro, kind: narrative, narrative_id: generator.arrival, at_ms: 0, duration_ms: 4200}
  - {cue_id: coil, kind: timer, at_ms: 0, set_deadline_ms: 30000, visible: false}
  - {cue_id: radio, kind: narrative, narrative_id: generator.radio_warning, at_ms: 10000}
  - {cue_id: reveal, kind: timer, at_ms: 10000, visible: true}
  - {cue_id: choices, kind: actions, available_actions: eligible, at_ms: 4200}
# Later: lifecycle End_Scene; renderer ACK; checkpoint/prepare next context.
```

Scope/scene identity is renderer-visible **only as deliberate protocol metadata**. No globals, locals, story graph or asset paths in Cue Sheets. Distinguish reentering the same `scene_id` from a distinct `epoch`. Define what REPLACE means at the transition boundary and when the next `Begin_Scene` is emitted; do not leave two simultaneous gameplay epochs active.

## 2. Save/restore at stable scene boundaries only

Normal player save and restore **only** at a stable boundary after `End_Scene` and before `Begin_Scene` for the next context. The host may present a visible pause/checkpoint screen, auto-checkpoint, or defer begin until a save request is handled. An ending may allow a post-scene completed-game save. No save during a running scene, including even if there is temporarily no active narrative.

This is a **deliberate change** from M4.5 checkpoint behavior. Define versioned migration for existing checkpoint schemas and old WHITEOUT saves; do not silently claim older mid-scene saves comply. Design continuity for long scenes: choose meaningful boundaries rather than forcing arbitrarily long replay. The host must not present an unattainably short save window; stable boundary is a persistent state until the host commits to next begin.

Restore must reproduce the exact next-scene continuation with no replay of the prior scene's graphics, action effects, deadline, clock or canceled cues. A checkpoint contains story identity/version, pending next scene and transition identity, semantic history/continuation, provider state snapshot/revision, and schema versions. No dependence on serialized animation frames.

## 3. Provider-owned world state

The Engine owns **rules** and the orchestration of typed mutations, **not necessarily the backing world-state storage**. The provider owns persisted world values. Globals survive scene changes; locals belong to a particular scene epoch. Ship an InMemory implementation and preserve the simple host composition API.

Conceptual interface, not fixed Rust ABI:

```rust
trait WorldStateProvider {
    fn snapshot(&self) -> WorldSnapshot;
    fn commit(
        &mut self,
        expected_revision: u64,
        changes: &[TypedStateChange],
        transition: Option<SceneScopeChange>,
    ) -> Result<WorldSnapshot, StateError>;
}
```

Required semantics:
- Read a consistent immutable revision for all conditions/outcomes in one engine transaction. Check value types, scopes and signed arithmetic exactly as authored; reject overflow/invalid values atomically.
- All ordered effects and local initialization/reset/teardown commit atomically. A failed action must not partially change provider state or engine semantic history.
- Treat stale revisions explicitly; **never silently retry a choice at a newer game time** or duplicate an authored action.
- No independent wall-clock/background mutation visible inside a transaction. External writes, if supported, must be explicit host-authorized transactions with a revision protocol and deterministic history.
- Keep the engine's and provider's checkpoint/restore revisions aligned, ideally with a single host transaction; never store a provider snapshot and semantic continuation from different scene boundaries.
- World-state inspection is read-only, scoped, typed and separate from presentation. Controlled debug writes need an explicit replay-safe path, not direct mutation behind the engine's back.
- Avoid runtime dependency on a database or remote service. A provider contract need not imply an external network service.

**Open design choice for implementation review:** whether engine computes proposed effects against a snapshot and provider atomically commits the resulting delta, or a short-lived transaction object implements reads/writes. Prefer the smallest approach that preserves deterministic outcome evaluation, validation and checkpoint consistency. Do not accept a trait that can commit partial mutations without rollback.

## 4. Semantic presentation IDs and renderer catalogs

Keep GRIMM presentation IDs as *semantic bindings*, not hardcoded 3D commands. Complex character motion, ducking, collision avoidance, dialogue/lipsync and camera direction live **inside renderer-authored presentations** (e.g. Unity Timeline, a Bevy scene/animation integration, or a 2D animation timeline). There is no core `actor_move`, `camera_move` or per-track protocol in M4.6. Existing single Cue Sheet supports multiple simultaneous presentations.

Scene preamble advertises a logical `scene_id`/`presentation_scope` for resource preparation. The renderer resolves it to actual dependencies; GRIMM does not enumerate meshes, textures or shader files. If essential resources are missing, fail the scene preparation clearly before Begin; nonessential prefetching can remain advisory.

Define a **versioned portable renderer catalog** for tooling and CI, exported by renderer-specific tooling, not hand-maintained as a second game truth:

```yaml
manifest_version: 1
renderer: example-bevy
scenes:
  whiteout.generator:
    resource: renderer-specific://generator
presentations:
  generator.arrival:
    scene_id: whiteout.generator
    duration_ms: 4200 # optional metadata
    completion: timed
  generator.startup:
    scene_id: whiteout.generator
    completion: callback
```

This is an **illustrative proposed format** only. Validator needs to check missing scene/presentation IDs, mismatched scope, completion semantics, optional duration conflicts, unused/misconfigured resources and compatibility across renderer variants. **Do not falsely infer that declared duration is always exact**; distinguish authoritative scheduled duration from estimated media/runtime duration and completion callbacks. If the artist changes a physical animation resource, the story must not need a rewrite solely to update mesh/clip paths.

The existing M3 story index is the starting point. Provide an independent, machine-readable comparison of **all possible referenced IDs**, not merely the currently offered path. Renderer-specific exporters (Unity/Bevy/sprite) will arrive with the selected rendering milestone; M4.6 specifies the neutral catalog contract and tests against a small fixture.

## 5. Acceptance gates and exclusions

- Lifecycle: begin/end/ACK, stable boundary, repeated scene visits, terminal endings, stale feedback, preloading ready/failure and untimed scenes.
- Time: relative deadline from *actual cue start*, absolute deadline, hide/reveal, stop, cutoff, overlapping presentations, no double book across epochs and waiting on an outgoing presentation.
- Provider: typed atomic rollback, globals/locals, invalid scope/type, stale revision, deterministic history, boundary checkpoint, restore to next scene and debug inspection.
- Catalog: source-index cross-validation against supplied renderer manifest; missing IDs and duration/completion mismatch diagnostics; no engine knowledge of concrete asset files.
- Backward compatibility: explicit versioned migration for schema 1/2 and checkpoint provenance; preserve original archived playthroughs/tests. WHITEOUT may introduce additional scenes/boundaries without changing its authored choices or endings.
- Preserve headless CLI/manual-clock tests and M4.5 inspector; show scene prepare/begin/end and provider state in debug view.
- Ship an executable end-to-end demo, docs and CI: `cargo fmt --all --check`, `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`.

**Not in M4.6:** rewriting WHITEOUT screenplays, choosing an actual graphics engine, generating character/scene models, integrating runtime 3D, building a full visual IDE, M6 analytics, general multi-clock simulation or adding universal 3D movement primitives.
