# Roadmap

## Draft 0.3 migration step (2026-11-04)

The current assignment targets [issue #66](https://github.com/rjtg/grimm/issues/66)
with an independent `grimm-graph` foundation: named nodes/edges, discovery-only
`expose`, immediate epoch-correlated traversal, full cue declarations and minimal
graph continuation. See the [mapping and next PRs](migrations/story-graph-03.md).
The concrete [WHITEOUT Bevy bootstrap](../crates/whiteout-game/README.md) (#67)
now embeds this core with a fixed camera, numbered keyboard interactions and
full-sheet scene replacement. [Host cue scheduling](../crates/whiteout-game/SCHEDULER.md)
(#68) adds physical scene time, deterministic arbitration, synchronous status
effects and the 5s/30s generator demo. [Typed gameplay bindings](../crates/whiteout-game/GAMEPLAY.md)
(#69) now demonstrate spare-part pickup, live conditions and repair versus
isolation/timeout. The canonical [generator-annex slice](../crates/whiteout-game/SCREENPLAY.md)
(#70) adds overlapping camera/character/audio cues, a 30-second decision, powered
and cold outcomes, and bounded replacement cleanup. It is a prepared excerpt,
not full canonical gameplay parity; host snapshots (#63) remain deferred. The older stateful-host sequencing below records the implemented baseline; it must not force WorldState/rules/ACKs
into the new core. PR #65 is closed without merge and is not part of this branch;
any later reuse of its refresh requires the reviewed completed-control regression
fix. No whole-game migration or handler codegen is included in this first step.

This is the canonical milestone sequence, reconciled on 2026-11-03. The
[repository audit](milestones/production-baseline-audit.md) distinguishes shipped
features from future production work. Historical M2 timing examples remain in
their reference documents; current serialized behavior is defined by the
[scene format](scene-format.md), [M4.5 migration](migrations/m4-5-scenes.md) and
[opt-in M4.6 contract](migrations/m4-6-scenes.md).

## Verified foundation

| Milestone | Status and evidence |
| --- | --- |
| M0 Architecture Foundation | Complete: headless engine, semantic/adapters/presentation/renderer boundaries and executable CLI; dependency tests. |
| M1 Classic CYOA | Complete: choices/conditions/effects, offline Last Witness, transactional actions and save/load. |
| M1.5 World Modeling & Dogfooding | Complete: Last Lighthouse, recurring characters/locations, three endings, state-driven choices and bounded deadlock analysis. No exhaustive reachability claim. |
| M1.75 Temporal Foundation | Complete: timed choices, authored advancement, timeout/pause/resume and save/load. |
| M1.8 Narrative Flow & Presentation Gating | Complete: explicit presentation/interaction readiness, narrative-only continuation, Silent Signal gated timed scenes and real CLI tests. |
| M2 Interactive Scene Foundation | Complete: declarative typed global/local DSL, deterministic durations/cutoffs, stable full-REPLACE cues and playable Locked Room; [evidence](milestones/m2-implementation.md). |
| M3 Story Authoring & Validation Toolchain | Complete: source-aware IDs/index, compiler diagnostics and bounded exit/deadlock checks; [guide](milestones/m3.md). Renderer inventory comparison stays outside core authoring. |
| M4 Complete Text Adventure | Complete: [WHITEOUT](../examples/whiteout/README.md), nine contexts, linked puzzles, alternative repairs, state/cold consequences, three endings and replay tests. |
| M4.5 Timer Lifecycle & Interactive Terminal UI | Complete and merged on main at `065d207` (PR #20). Protocol/story schema 2, independent scene clock/deadline lifecycle, timed/overlapping text, TUI/inspector and legacy compatibility; [evidence](milestones/m4-5-implementation.md). |
| M4.6 Scene Lifecycle, Provider & Renderer Catalog | Merged in PR #22 at `6c5ab68`. Existing lifecycle, atomic InMemory provider, text preparation and replay checkpoints are implemented. Amended ADR-0008 now requires authoritative host ECS and non-replay host snapshots; those follow-ups are **not implemented**. See the [Bevy audit and #37 plan](bevy-integration.md). |

## WHITEOUT production: M5a–M5g

**Story Graph → Screenplay → Visual Identity → Bevy Integration Proof → Assets →
Cueables → Choice UI → Playable Game.** The canonical game is
`examples/whiteout/story.yaml`; text and graphics use those same rules and action
IDs. Do not fork puzzles, world state or branching into presentation code.

**Numbering:** M5 remains **Graphic Adaptation of the M4 Adventure**. The requested
production M5–M11 work is represented below as M5a–M5g, preserving the separately
proposed [M6 execution-history/statistics milestone](milestones/m6-execution-history-statistics.md).
The former M3 inventory/RPG scope and old free-input/dynamic/NPC numbering are
superseded, not revived by these production phases.

| Phase | Dependencies and status | Verifiable deliverables / exit gate |
| --- | --- | --- |
| M5a Screenplay adaptation (task's M5) | M4/M4.5 + verified M4.6; M5a.1 implemented, **not a complete screenplay** | Writer-friendly screenplay-style alternative text analogous to prose presentation assets: dialogue, action, blocking, shots, lighting and sound. Optional existing semantic-ID notes; human editorial review for narrative/branch consistency. This increment delivers one useful scene sample, not a screenplay compiler, schema, automated coverage/text-equality gate or whole-game rewrite. Continue writing in later content increments. |
| M5b Visual identity & virtual actors (task's M6) | Editorially reviewed M5a sample; **Missing** | Character and location specification formats, canonical appearance/silhouette/proportions, clothing/variants, expressions, poses, lighting/style guide, example actor, variant validation and provenance/versioning. Canonical specs remain separate from replaceable generated artifacts; no provider lock-in. Evaluate 2D references/sprites, meshes/rigs or hybrid before bulk production. |
| M5c Bevy integration proof (task's M7) | #25 tracking; #53 provider/time design + #63 continuation + #36 slice before #37; **Planned**, small correlated timeout input implemented | Fixed-camera point-and-click WHITEOUT slice with actual typed ECS fields/CAS, Bevy-owned deadline events, lifecycle/preload, full-REPLACE cues, UI/hotspots and host boundary snapshot save/load. Bevy 0.20.1 first; no alternate sprite engine or generic renderer framework. Exact ordering/tests in [audit/plan](bevy-integration.md); #38 records integration evidence/risks. |
| M5d Asset production (task's M8) | M5b + verified M5c Bevy slice; **Missing** | Versioned manifest with stable asset ID/type/logical identity, required variants, source specification, physical resource path/format/version, dependencies, provenance and production status. Extract requirements from screenplay, compare actual available registry for missing/unused resources, produce/load sample assets and document reproducible import/build. Cover characters, expressions/animation, locations, props, environment/camera/audio/UI as required by the chosen renderer. |
| M5e Cueable presentation production (task's M9) | M5a + Bevy host + sample M5d assets + M4.6 catalog/ACK; **Partially implemented** foundations | Map staging to renderer-authored presentations: movement, expressions, animations, camera pan/zoom/target/close-up, fades, light/environment/audio, waits and transitions. Specify targets, optional duration, ordering/parallelism, blocking/completion, error/cancellation and scene ownership; compile one scene timeline and test deterministically through adapters. Reuse existing CuePlayer/ACK/REPLACE semantics. Extend schemas only for a demonstrated gap; no renderer-specific movement commands in story nodes. |
| M5f Graphical choice input (task's M10) | Bevy host + M4.6 lifecycle/checkpoints + vertical scene/cue integration; **Partially implemented** semantics/text UI | Choice interface and selected-engine implementation with layout/focus, keyboard/pointer/controller where appropriate, feedback, timed appearance, full-set disabling/replacement, stale input protection and concurrent presentation. Test offered action IDs against graph transitions; renderer never owns branching. |
| M5g WHITEOUT integration (task's M11) | Thin samples from M5a–f + M4.6 lifecycle/provider/resource gate; **Missing** graphical integration | First executable vertical slice: load graph, resolve actual renderer scene/presentations, validate actual assets, preload, enter, run staging/dialogue, offer choices, resolve decisions, transition, retain/release resources. Verify IDs, cues, choices and missing-reference diagnostics with automated tests. Only then extend to all locations/routes/endings; publish content completion plan, setup, playtests and canonical parity evidence with CLI. |

M5a writing/editorial review does not require choosing a renderer or changing the
engine. Identity work may overlap writing; use Bevy for the first fixed-camera graphical proof; do not evaluate alternative
sprite engines as a prerequisite. M5d/e/f converge on a
thin playable slice before bulk asset production or whole-game conversion. M5g
has separate slice and full-coverage gates; a prototype is not a finished game.
The [existing production plan](milestones/m5-whiteout-production-plan.md) supplies
supporting detail; this table is the canonical ordering/status source.

### Interfaces inherited from M4.6

Every later phase uses explicit presentation scene IDs and preserves the semantic
cue/content/action distinction. IDs are independent of physical paths and generated
files. The protocol remains a single ordered full-REPLACE plan with optional
presentation duration, absolute scheduling, completion dependencies, retained
identities and revision-bound acknowledgements. No required duration for every cue,
fixed verb API, renderer camera/movement commands in story nodes or extra tracks.

M5c must evaluate ready/error before Begin, frozen outgoing media, scoped asset
retention/release and deterministic CI. M5d/e export the selected renderer's actual
registry in the M4.6 catalog format and compare **all** authored references,
including hidden outcomes. M5f/g implement host-owned boundary snapshot slots,
explicit save-slot copies and non-replay Load; no mid-animation snapshot pretending to be
a boundary checkpoint. Engine validates canonical rules; a replaceable provider
owns backing storage. Renderer inspection never leaks private world state into
player output. Authored stories/gameplay remain correct; development save formats may be
replaced without migration. Host snapshot-slot/non-replay restore is a planned
prerequisite, not a claim that current replay saves already satisfy it.

### First increment and next task

With M4.6's runtime gates verified, M5a.1 supplies a readable
[blackout screenplay sample](../examples/whiteout/production/blackout.screenplay.md):
an alternative presentation text associated with `blackout.arrival`, using the
same semantic association as prose assets. It adapts exposition into dialogue,
visible action and staging. Human editorial review checks narrative/branch
consistency; no screenplay schema, compiler, runtime, structural validation,
coverage gate or exact-text requirement applies. Gameplay rules remain canonical
in `story.yaml` and current CLI prose bindings are unchanged.

Next, continue screenplay writing with the generator's branching warning/startup
sequence in a later content increment. Review dramatic clarity, alternatives,
clue timing and production needs, without making creative adaptation a parser
contract. This PR does not rewrite the whole game.

## Cross-cutting authoring work

Use existing CLI graph validation and, at later concrete renderer production
gates, validation against actual exported resources and cue bindings. Screenplay
writing remains editorial content. Human readers may use optional semantic-ID
notes for traceability; a machine-checkable screenplay layer is not required.

The long-term vision of descriptions evolving through screenplay and
previsualization into polished graphics is a creative workflow, not an immediate
requirement for asset tiers, runtime layer interchangeability, fallback selection
or an authoring studio. Do not build generalized production infrastructure for
this scene sample. Later tooling must be justified by actual production needs.

## Open decisions and later work

- ADR-0008 is accepted by maintainer `rjtg` for lifecycle/state/timing semantics.
  Concrete Rust APIs and serialized contracts retain an explicit stability
  reservation until graphical WHITEOUT integration; changes require deliberate
  versioning, migration/compatibility tests and review. Earlier ADRs remain unchanged.
- Further screenplay writing needs human review of branch meaning, clue timing
  and staging. Editorial pacing notes do not change authored gameplay timing.
- M5b chooses identity formats/style and provenance; generated images are replaceable
  outputs, not actor identity or a promise of game-ready rigs/models.
- Bevy is the first graphical game host, including fixed-camera 2D/2.5D/3D
  point-and-click. #53 provider access/refresh comes next, then #63 non-replay
  continuation and #37 after the #36 slice gate. No engine comparison required.
- M5d/e specify the chosen renderer's physical import/export/build and cancellation
  behavior. Text preparation does not prove graphical asset readiness.
- M6 statistics remains a separate proposal. Free-text intent, human-DM/dynamic
  graphs, NPC planning and narrative direction are later optional capabilities;
  baseline authored games stay offline. See [open questions](open-questions.md).

## Delivery rules

Deliver tested increments with reproducible examples, reviewable commits and
explicit incomplete criteria. Preserve dependency directions and authoritative
engine rule ownership. Record significant new boundary decisions before runtime
implementation. Run fmt, build, workspace tests and Clippy; do not claim main
integration or milestone completion from labels alone. Never merge PRs implicitly.
