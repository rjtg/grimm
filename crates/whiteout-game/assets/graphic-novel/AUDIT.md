# Graphic-novel pilot audit (#83)

Baseline: main `3dc9b8a` after #81. Work is isolated on
`feat/whiteout-graphic-novel`. PR #82 was closed by the maintainer and remains
closed; no further 3D implementation is part of this pilot.

Reuse: GRIMM graph/epoch traversal; WHITEOUT scheduler, input/deadline arbitration,
condition/effect handlers, content and decision UI; the existing three PCM masters. Keep the
prepared-route simplifications documented in SCREENPLAY, not full canon claims.
No GRIMM protocol or authoritative gameplay change is needed.

The rejected 3D experiment proved native import, named pivots, asset preloading
and cleanup, but the generated geometric characters read as mannequins; materials
and lighting did not carry the illustrated reference quality. Full rig/contact,
face, costume and lighting production would be disproportionate for this pilot.
Its sources remain available, without treating the meshes as approved assets.
The local software renderer also exposed expensive shadow settings and the known
ALSA underrun/POLLERR issue. Asset-ready gating and an explicit mute option are
useful host lessons; the GLB/PBR dependency expansion is unnecessary here.

Compare A/B on the same authored sequence and same master illustrations:
A stages cutouts with cuts/pushes; B adds only bounded depth offsets to those same
layers. Implement both in the existing WHITEOUT stage, not two presentation engines.
The eventual production recommendation remains subject to this pilot's review.

The inherited three WAV bindings remain unchanged in this pilot; no additional
#82 SFX or model files have been copied. Its GLB geometry/rigging sources remain
an archived experiment, potentially useful for future spatial blocking only.
