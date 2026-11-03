# M5 — WHITEOUT Graphical Adaptation: Production Pipeline and Gates

**2026-11-03 decision:** Bevy is the intended first graphical host. #25/#53/#63
and the [current audit/plan](../bevy-integration.md) supersede prior alternative
engine evaluation and replay-save direction. No generic renderer framework.


**Status:** Supporting production gates; current ordering/status is maintained in the [canonical roadmap](../roadmap.md). M5a.1 adds a writer-friendly [blackout screenplay sample](../../examples/whiteout/production/README.md); the complete M5 remains unfinished. Proposed sequence of **intermediate goals**, not an authorization to implement the entire graphic adaptation at once. **Prerequisite:** [M4.6 architecture contract](m4-6-scene-state-renderer-contract.md). **Product constraint:** preserve WHITEOUT's canonical authored game rules/decisions/endings as a single source; adapt and enrich presentation rather than fork into a separately scripted game.

M5 should be delivered in **reviewable increments** with explicit go/no-go gates. The story-engine, narrative writing and rendering pipelines have different owners and artifact formats; do not combine them into one oversized PR.

## Pipeline at a glance

| Gate | Deliverable | Acceptance / decision |
| --- | --- | --- |
| **M5a — Screenplay adaptation** | WHITEOUT screenplay in scenes, dialogue and stage directions | Current increment: readable scene sample and human editorial review; later content increments extend writing. Optional existing semantic-ID notes; no gameplay change or screenplay-specific validation subsystem |
| **M5b — Character bible and concept art** | Consistent virtual actors, environments, visual rules | Approved visual references for Alex, Mara, Jonas, Lea; controlled character continuity across shots |
| **M5c — Bevy integration proof** | Bevy-first fixed-camera point-and-click slice | Actual ECS provider, host timers/snapshots, readiness, cues/input and CI evidence |
| **M5d — Required base assets** | Reusable actor/location/environment bases for chosen engine | All critical locations/actors available and loadable; right scale/material/style; license/provenance tracked |
| **M5e — Cueable presentation production** | Renderer-authored animations/shot/dialogue sequences tied to semantic IDs | Every referenced cue bound; timing, completion and scene preload tested |
| **M5f — Choice input presentation** | Legible interactive choices, timer UI, keyboard/controller navigation | Same action/offer semantics as M4.5; locks, deadline, pauses and accessibility covered |
| **M5g — Full integration** | Complete graphical WHITEOUT on canonical story | All endings and puzzles; debug inspections, boundary saves, CI and playtests |

Each gate ends with concrete artifacts and a short review. Do not force expensive 3D asset work before M5c.

## M5a — Screenplay rewrite of the *presentation*, not the rules

Adapt WHITEOUT's prose presentation into writer-friendly screenplay text,
analogous to the existing prose assets. Freely turn exposition into dialogue,
acting, blocking, visible movement, shots, camera direction, lighting and sound.
Preserve narrative facts and branch/outcome meaning through **human editorial
review**, not exact prose reproduction or a screenplay-specific schema gate.

Use an author-friendly format such as Markdown. Optional notes with the existing
semantic presentation IDs help associate text with the same authored situation;
they are not a compiler contract, execution/specification layer or second story
graph. Stage directions guide creative production, not GRIMM runtime commands.
Keep `story.yaml` rules and current CLI prose behavior intact.

Acceptance for the current increment is one readable, useful scene sample with
clear staging and creative requirements. See the
[blackout sample](../../examples/whiteout/production/blackout.screenplay.md).
Continue actual screenplay writing in subsequent content increments. No
`check-screenplay`, structural validation, automatic story-coverage gate or
text-equality rule is required. Technical validation of actual renderer-exported
resources and cue bindings remains a separate, later production gate.

Descriptions, screenplay, dummy/previsualization renders and polished graphics
may eventually form a creative workflow. That vision is **not** authorization to
build asset tiers, versioned presentation layers, fallback selection, a new
runtime or an authoring studio in this PR.

## M5b — Virtual actors and visual bible

Produce a small, versioned **character/visual bible** and approved **concept art**, not final 3D mesh rigs prematurely. Define:
- Alex, Mara, Jonas, Lea: stable silhouette, relative age impression, face and body features, hair, outfit layers, palette, signature props and expression range; include front/side/profile and important emotion/expression studies.
- Locations: Aurora station exterior, stores/triage, dispatch hall, engineering/generator, communications/radio cabin, ridge/crisis, interiors and lighting/weather mood.
- Composition/cinematography: shot vocabulary, lens/composition references, lighting continuity, legibility, snow/ice/material rules and temporal scene changes.
- A consistent reference/identity workflow: approved reference sheets, immutable asset IDs, versions, asset metadata and composition checks across subsequent generations; do not assume a text prompt alone guarantees character identity.
- Rights and provenance: source, author/model/tool where applicable, permission/licensing, reuse rights, review/approval status. Do not present placeholder generation as production-ready.

Deliver approved art sheets, variants and a manifest linked to screenplay roles/locations. **No bulk production assets before the Bevy proof establishes requirements.**

## M5c — Bevy integration proof

Bevy is the first graphical host, also for classic fixed-camera point-and-click.
No second sprite engine or engine-selection matrix is required. Use Bevy 0.20.1
core features first; no plugins without demonstrated need. #53 establishes ECS
provider/event contracts; #63 supplies non-replay host snapshot continuation;
#36 fixes the generator slice and #37 implements the actual proof in that order.
See the [test/dependency plan](../bevy-integration.md). #38 records integration
risks and measured evidence, rather than reopening renderer selection.

## M5d — Generate the required base assets

After the Bevy slice establishes concrete requirements, build the minimum viable reusable assets:
- Characters: production-ready actor representations appropriate to the chosen engine (**3D meshes/rigs/animations if 3D; layered sprites/rigs/poses if 2D**), with consistent visual identity and reusable expression/motion sets.
- Locations: all key WHITEOUT environments in the target representation; props needed for latch, spanner, braid, mug, heat pack, radio, safety line and generator puzzle feedback.
- Weather/lighting/audio: enough to stage the dramatic story, including storm, power outage, repairs, radio and rescue.
- Resources: source files and exportable versions in appropriate repository/artifact storage, stable asset IDs, build/import instructions, licensing/provenance, performance budget and review status.

**Do not assume image generation yields a rigged, game-ready 3D model.** Plan distinct concept-to-model, retopology, UV/material, rigging and animation steps for 3D, or layered pose/rig/export steps for sprites. Validation must load assets in the selected engine; check actual actor appearance across scenes and alternative ending branches.

## M5e — Produce cueable presentations

Implement renderer-specific *presentations* for screenplay beats and bind them through a versioned **renderer-exported manifest**:
- Semantic GRIMM `narrative_id/presentation_id` maps to renderer-owned scene/cutscene/sequence resources.
- Scene `presentation_scope` selects preload dependency groups. The renderer owns transitive asset dependencies and reports ready/error before `Begin_Scene`.
- Timed sequences may have declared durations; variable length presentations complete through callbacks. Treat timing metadata as either enforced or estimated, never silently assume both.
- One full REPLACE Cue Sheet with `at_ms`, `after_cue`, stable IDs, overlapping playback, canceled/retained media and explicit scene boundary ACKs remains authoritative.
- No core GRIMM `camera_move`, `actor_move` or required per-track protocol. A renderer may use internal timelines/tracks and detailed actor/camera commands inside each presentation resource.
- Add automated catalog-vs-story-index validation and deterministic cue-playback regression tests; report missing IDs, incompatible scopes or conflicting duration/completion assumptions.

## M5f — Choice input and UI in the chosen engine

Make a renderer-specific choice presentation, not a second choice-rule engine. GRIMM remains the authority for offered/eligible actions. Support:
- Choice labels, visual focus, input navigation (keyboard, pointer, controller where supported), usable timing pressure and legible timer/deadline state;
- `available_actions: []` and revision-stale offers, lock/unlock during presentation barriers, and concurrent cues;
- Pause, accessibility/subtitles, text speed/settings, error/asset fallback, focus/contrast and tutorial affordances;
- Host boundary snapshot-slot Save/Load under amended ADR-0008: copy the pre-Begin snapshot, restore ECS + continuation without old-effect replay, and provide a clear boundary UI. Current replay checkpoints must not be reused as the new host contract.

Verify behavior against headless M4.5 semantics and the same authored choices, durations/cutoffs and outcomes.

## M5g — Full graphical WHITEOUT integration

Wire all scenes, presentations, actor/location assets, options, timers, host/world-state provider, lifecycle and boundary checkpoints into a playable experience. Preserve the same authored WHITEOUT story logic across CLI and graphic renderer, including full rescue, partial survival, catastrophic ending, alternative latch and radio solutions, time-dependent cold exposure, early/late choice effects, hidden/revealed deadlines, overlapping cues and terminal scene closure.

Acceptance:
1. Complete visual playthroughs to all endings with meaningful choices and canonical states matching deterministic engine tests.
2. Every authored narrative/presentation ID from the M3 index resolves in the chosen renderer's actual asset catalog.
3. At least one expressive complex sequence (movement, ducking/obstacle interaction, camera, sound) is **renderer-authored** and triggered through one semantic ID, demonstrating why no generic 3D DSL is necessary.
4. Replay/save consistency at explicit scene boundaries and no ghost old-scene timers/inputs.
5. Consistent characters across locations/camera angles; documented visual approval and rights metadata.
6. Graceful missing-asset diagnostics, renderer readyness/ACK behavior and CI-compatible smoke tests.
7. Usable choice presentation, accessibility, controls, stable performance and documented setup.
8. Gameplay rules and ending facts remain authored by GRIMM; renderer is not a competing gameplay source.

## Possible later authoring support

Any authoring UI or preview must be justified by later concrete production
needs. Screenplay writing does not require a new compiler, validation subsystem,
asset-tier architecture or editor. Current graph checks and M4.6 catalog tooling
remain available; technical resource validation is applied separately when actual
renderer assets/cue bindings exist. No authoring studio is committed by M5a.
