# Concrete Bevy handoff — planning, not implementation

The existing `whiteout-game` from PR #76 uses Bevy 0.20.1 and `grimm-graph`.
`src/cueables.rs` supplies fixed camera/prop/audio bindings; `stage.rs` paints
sprite set pieces and owns cue instances; `scheduler.rs` arbitrates host time,
conditions/effects/input with epoch/deduplication/full replacement. This task
changes none of them or their native dependencies. No Bevy API upgrade claim.

## Realization options by set

| Set | Pragmatic 2D / 2.5D basis | Where shallow/simple 3D is useful | Later concrete animation work |
| --- | --- | --- | --- |
| Corridor / hall | Wall/door/window/duct layers, placed cast sprites/poses, local light overlays, repeatable fixed camera. | Supported carry turning through threshold, reliable foot/hand occlusion. | Count/torch, same rest position, door closure, warming. |
| Stores | Bench/shelf/tray/rack layers with state prop variants and insert art. | Hand pickup/treatment, partner/splint contacts, shelf occlusion during transfer. | Equip, splint, selected prop removal, shutter after cast clears. |
| Annex | Casing/hinge/fuse/rail layers, gauge overlay, short 2D camera push, flywheel sprite rotation. | Primer/fuse articulation, lever contact, supported brace with cast, consistent close parallax. | Crack/nearly/open variants, failed grip, external pull, successful startup, retreat. |
| Radio | Connected cabin/bay layered view, transmitter insert, wheel/mug state variants. | Hand/shoulder wheel contact, mug/coil threading, cable routing and partner alignment. | Catch/hold/release without duplicating Lea; gust overlap. |
| Ridge | Terrain/shelter/manifold/marker fore-mid-back layers, controlled snow overlays. | Route-dependent carry/line contacts, boots/terrain occlusion and measured distance. | Rig line/stop failed plan, reach/carry, marker burial and exact casualty. |
| Endings | Fixed transport/table/exterior layers, cast/state composition and dissolves. | Optional seating/carry transition if visible, not a full rescue vehicle simulation. | Correct manifest count/absence, tag/sketch/name inserts, station burial. |

No shot inherently requires a full 3D world. Interacting 3D pieces can be
advantageous where reliable hands/feet/occlusion would otherwise require many
2D pose variants; that does not choose a general renderer architecture. Actual
assets, measurement and performance testing must guide the later concrete game
implementation. Concept images are not sprites, collision meshes or texture maps.

## Preserve proven runtime contracts

- GRIMM graph core owns node/epoch and declarations, not environment simulation,
  WorldStateProvider, conditions, physical timers, assets or cue ACKs.
- Bevy host owns actual world rules/values, time, input and presentation. Add no
  shadow story FSM or independently mutable “environment state” from these
  artistic variants. Use source conditions/outcomes to select visual variants.
- Keep one full replacement plan: obsolete audio/animation/scene work stops,
  stale epoch callbacks cannot activate another interaction, duplicate cues don't
  replay. Camera/FX changes must not cancel unrelated concurrent work by accident.
- Missing bindings/assets remain explicit visible failures; duration metadata is
  not proof of successful animation or gameplay. No timer loop belongs in GRIMM.
- Ordinary transition nodes can stage accepted set changes later. They must keep
  canonical zero-cost travel, start destination timing correctly and preserve
  duration-cutoff rules. Don't copy the excerpt's protected late-start shortcut.
- A new backdrop cannot mint tools, offer new doors or relax rescue predicates.
  Complete canonical rule mapping remains future work; this bible doesn't claim
  the full graphical story has been implemented.

## Practical constraints to test before final asset spend

Current prototype uses 1000×700, variable bottom captions/choices, a software
Vulkan desktop smoke and three placeholder WAVs. It is not a full level, rig,
material-budget or target-hardware benchmark. Camera safe-region targets and
per-set depth needs are in CINEMATOGRAPHY. UI-aware actor/prop visibility, actual
contact/clearance and layered/3D performance require a concrete rehearsal before
final production approval. Local prior ALSA errors also require a later listening
pass; new sounds aren't produced here.

No renderer components, scene loader, asset pipeline, generic adapter, API,
new protocol type, screenshot automation, codegen (#75), Save/Restore (#63),
final mesh/sprite or executable graph change is delivered by this handoff.


The [maintainer handoff gates](VALIDATION.md#maintainer-review--next-production-gates)
explicitly require drawn storyboard/animatic, actual-viewport composition and
contact/metric rehearsal before detailed assets. This bible contains none of
those completed proofs; its plans/concepts remain provisional production input.
