# M5 — WHITEOUT Graphical Adaptation: Production Pipeline and Gates

**Status:** Proposed sequence of **intermediate goals**, not an authorization to implement the entire graphic adaptation at once. **Prerequisite:** [M4.6 architecture contract](m4-6-scene-state-renderer-contract.md). **Product constraint:** preserve WHITEOUT's canonical authored game rules/decisions/endings as a single source; adapt and enrich presentation rather than fork into a separately scripted game.

M5 should be delivered in **reviewable increments** with explicit go/no-go gates. The story-engine, narrative writing and rendering pipelines have different owners and artifact formats; do not combine them into one oversized PR.

## Pipeline at a glance

| Gate | Deliverable | Acceptance / decision |
| --- | --- | --- |
| **M5a — Screenplay adaptation** | WHITEOUT screenplay in scenes, dialogue and stage directions | All narrative beats, meaningful choices and endings covered; stable semantic cue mapping; no gameplay change |
| **M5b — Character bible and concept art** | Consistent virtual actors, environments, visual rules | Approved visual references for Alex, Mara, Jonas, Lea; controlled character continuity across shots |
| **M5c — Rendering engine decision** | Honest 2D/sprite versus Bevy evaluation with vertical slices | Documented scored decision on art quality, asset pipeline, runtime/input, CI and team feasibility |
| **M5d — Required base assets** | Reusable actor/location/environment bases for chosen engine | All critical locations/actors available and loadable; right scale/material/style; license/provenance tracked |
| **M5e — Cueable presentation production** | Renderer-authored animations/shot/dialogue sequences tied to semantic IDs | Every referenced cue bound; timing, completion and scene preload tested |
| **M5f — Choice input presentation** | Legible interactive choices, timer UI, keyboard/controller navigation | Same action/offer semantics as M4.5; locks, deadline, pauses and accessibility covered |
| **M5g — Full integration** | Complete graphical WHITEOUT on canonical story | All endings and puzzles; debug inspections, boundary saves, CI and playtests |

Each gate ends with concrete artifacts and a short review. Do not force expensive 3D asset work before M5c.

## M5a — Screenplay rewrite of the *presentation*, not the rules

Rewrite the current WHITEOUT text binding into a **screenplay/staging format** while preserving the narrative facts, characters, narrative IDs, action IDs, character outcomes, puzzle clues and timing contracts. The screenplay may have sluglines, shots, camera language, blocking, actors' gestures, sound design and dialogue. Example:

```text
INT. AURORA — GENERATOR ANNEX — NIGHT
WIDE: Rime-coated machinery trembles as the intake duct buckles.
CLOSE ON: Alex's gloves closing around a frozen latch.
JONAS (O.S.): "Try the upper hinge, not the handle."
SFX: A sharp crack; the turbine settles into an uneasy hum.
[GRIMM presentation: generator.latch_crack]
```

**Important:** stage directions are instructions for the production team/renderer assets, **not** new generic `camera_move` or `actor_move` commands in the GRIMM DSL. Preserve semantically equivalent text binding for CLI and accessibility; do not replace canonical player-visible narration with only shot directions. Avoid adding a new gameplay variable just to trigger a shot.

Deliver scene-by-scene screenplay files with a machine-checkable mapping `cue_id -> narrative_id/presentation_id -> screenplay beat`, all choice labels, branching alternatives, a spoiler walkthrough and editorial change notes. Specify optional timing: target shot duration, whether playback is fixed/timed versus callback-completed, and `after_cue` barriers. The screenplay can be written before choosing a rendering engine.

## M5b — Virtual actors and visual bible

Produce a small, versioned **character/visual bible** and approved **concept art**, not final 3D mesh rigs prematurely. Define:
- Alex, Mara, Jonas, Lea: stable silhouette, relative age impression, face and body features, hair, outfit layers, palette, signature props and expression range; include front/side/profile and important emotion/expression studies.
- Locations: Aurora station exterior, stores/triage, dispatch hall, engineering/generator, communications/radio cabin, ridge/crisis, interiors and lighting/weather mood.
- Composition/cinematography: shot vocabulary, lens/composition references, lighting continuity, legibility, snow/ice/material rules and temporal scene changes.
- A consistent reference/identity workflow: approved reference sheets, immutable asset IDs, versions, asset metadata and composition checks across subsequent generations; do not assume a text prompt alone guarantees character identity.
- Rights and provenance: source, author/model/tool where applicable, permission/licensing, reuse rights, review/approval status. Do not present placeholder generation as production-ready.

Deliver approved art sheets, variants and a manifest linked to screenplay roles/locations. **No final engine commitment before the M5c decision.**

## M5c — Rendering strategy: sprites/2D versus Bevy

The comparison must be based on a **real representative vertical slice** from WHITEOUT, not abstract engine preferences. “Sprite engine” is a rendering approach, not by itself a specific engine; identify at least one concrete 2D/sprite runtime candidate and compare it to Bevy (and optionally hybrid 2.5D, if evidence strongly supports it).

Prototype the same 30–60 second sequence in both paths with:
- The same approved character/setting reference and dialog beat;
- Scene preload/readiness and semantic presentation ID binding;
- Character movement and an expressive camera/shot sequence;
- Overlapping radio cue, deadline visibility and input offer/lock;
- Callback/timed completion and a clean transition;
- A reproducible development build.

Evaluate **visual coherence**, character identity fidelity, animation labor, 2D/3D asset conversion cost, tools/editor and iteration speed, integration with Rust/GRIMM, performance and portability, test automation, accessible choice UI, debug inspector, maintenance risk and actual team/tool availability. Use a weighted scoring rubric with explicit assumptions and measured evidence, not a predetermined winner.

**Decision gate:** document an ADR with chosen engine/approach, rejected options, cost/risk tradeoffs and consequences for asset generation. The decision must come **before** bulk model/sprite production.

## M5d — Generate the required base assets

Only after the engine choice, build the minimum viable reusable assets:
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
- Save/restore **only at the M4.6 stable scene boundary**, with a predictable visible checkpoint UI, not a mid-animation snapshot.

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

## After M5: renderer-aware Authoring Studio

A future authoring UI may combine story graph, scene timeline, screenplay, and renderer-exported catalogs. It should offer available-ID selection, missing/unused asset checks, optional duration/completion mismatches, scene preload validation and preview. **Do not create a new proprietary third source of truth.** The GRIMM story and the renderer's actual assets remain canonical; the IDE edits/validates their exported representations.

M5 does **not** need a full IDE. It needs a reliable renderer catalog exporter and GRIMM validator first.
