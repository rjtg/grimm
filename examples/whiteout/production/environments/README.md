# WHITEOUT Environment and Cinematography Bible — v1 proposals (#35)

Baseline main **`6f46a94`**, after PR #78; full screenplay PR #77, cast bible
PR #78 and concrete Bevy excerpt PR #76. The nine existing `loc.*` identities
remain authoritative production references. No executable story or engine change.

**[C]** canonical facts/rules; **[A]** proposed design/layout/material/shot;
**[O]** decision pending production review. After review, use the accepted
revision as the reference for asset derivation; concept art is not automatically
approved architecture, navigable geometry, a working rig or a shipped scene.

## Reading guide

1. [Shared architecture/material/art direction](STYLE.md).
2. [Station adjacency, floor plans, blocking and threshold joins](PLANS.md).
3. Location specifications: [station/corridor](01-station-corridor.md),
   [stores](02-stores.md), [dispatch](03-dispatch.md),
   [generator](04-generator.md), [radio](05-radio.md),
   [ridge](06-ridge.md), [all terminal locations](07-endings.md).
4. [Cinematography and stable camera catalogue](CINEMATOGRAPHY.md),
   [lighting/weather/state continuity](STATES.md).
5. [Architecture, prop, camera, FX and sound inventory](ASSETS.md).
6. [Actual concept references, prompts, provenance and inspection](REFERENCES.md).
7. [Pragmatic Bevy realization and preserved contracts](BEVY-HANDOFF.md).
8. [Validation, unresolved decisions and production order](VALIDATION.md).

## World identity

A low, practical insulated station under snow, not generic futuristic sci-fi.
Pale grey/sage service panels, charcoal floors, exposed cable trays, sealed
windows and small reserve lights repeat between rooms. The cast's slate/teal/
ochre/blue-grey silhouettes stay distinguishable. The hall never rearranges:
radio left, engineering right, ridge ahead; repeat visits change people/light/
access, not geometry. Low-support Jonas and a physically required manual Lea
remain actual staging constraints.

The ridge keeps radio-side shelter separate from engineering-side manifold;
no permanent rail, added safe path or escape door solves the source choices.
Stores has one shared threshold. A radio mast working bay is part of the same
scene, never a new traversal or second timer. Endings reuse the station/manifest
count and exact named absence; no new survivor or location puzzle.

## Production recommendations

This is a **provisional environment/cinematography bible**, not completed art
sign-off, a drawn storyboard or final metric level specification. See the
[maintainer review and next gates](VALIDATION.md#maintainer-review--next-production-gates).

- Before detailed stage assets/animation, create a **separate visual storyboard /
  animatic**: generator first, then radio/ridge. Draw actors, POV/edit order,
  hand/prop inserts, actual bottom UI and alternate outcomes per screenplay
  beat/cue/camera ID. The concept sheets and top-down plans here do not do that.
- First rehearse module scale, registered thresholds, carry/brace/wheel contacts
  and the longest actual GUI panel using the existing cast placeholders. Verify
  door/hand/ankle/target visibility before commissioning polished environments.
- Build the small shared wall/frame/window/floor/service/light kit and the
  repeatable hall. Keep layout stable across powered/unpowered returns.
- Produce stores/annex interactive props with actual progression/possession
  variants; retain full canonical cold/cutoff/combination rules in later host work.
- Build connected radio cabin/bay with one Lea and real manual versus hands-free
  duty; then ridge with route-dependent contacts/line and authored snow occlusion.
- Dress terminal transport/report/exterior vignettes after survivor composition
  is verified. Match four faces, exact empty place, four names without faces.
- Only then finalize materials, weather/lighting polish and audio with target
  hardware and listening checks. Freeze the reviewed reference revision before
  derivative generations; a fresh prompt alone is not identity continuity.

This is a dependency order for future concrete asset work, not authorization to
build a new pipeline. No final levels/meshes, new cast, renderer choice, protocol,
generic adapter, screenshot automation, IDE, Save/Restore (#63) or codegen (#75).
Refs [#35](https://github.com/rjtg/grimm/issues/35),
[#24](https://github.com/rjtg/grimm/issues/24),
[PR #77](https://github.com/rjtg/grimm/pull/77),
[PR #78](https://github.com/rjtg/grimm/pull/78).
