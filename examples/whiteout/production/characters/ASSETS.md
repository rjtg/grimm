# Stable character asset catalogue — editorial, not a loader manifest

Root IDs already exist: `actor.alex`, `actor.mara`, `actor.jonas`, `actor.lea`.
Derived IDs below refer to **one actor**, not independently spawned story
characters. A variant never changes graph identity, survivor state or possession.
No model/texture/rig/animation files exist under these planned IDs yet.

## Character-specific catalogue

| Actor | Planned IDs | Meaning |
| --- | --- | --- |
| Alex | `actor.alex.model.base`, `actor.alex.outfit.station`, `actor.alex.outfit.exterior`, `actor.alex.outfit.liner` | Casting-neutral body/costume envelope; shell/hood variants; authored liner overlay only after pickup. |
| Alex | `actor.alex.pose.device`, `actor.alex.pose.cold`, `actor.alex.expr.upper_face` | Two-hand operation, impaired grip, non-final upper-face acting; no locked unmasked casting. |
| Mara | `actor.mara.model.base`, `actor.mara.outfit.station`, `actor.mara.outfit.exterior`, `actor.mara.expr.core` | Same proposed face/build/teal clothing under light and weather variants. |
| Mara | `actor.mara.pose.support_jonas`, `actor.mara.anim.support_jonas`, `actor.mara.pose.manifest` | Support/carry contacts and final count, with casualty-aware branches. |
| Jonas | `actor.jonas.model.base`, `actor.jonas.outfit.station`, `actor.jonas.outfit.exterior`, `actor.jonas.expr.core` | One proposed face/build/orange costume, even seated or exhausted. |
| Jonas | `actor.jonas.variant.untreated`, `actor.jonas.variant.treated`, `actor.jonas.pose.injured`, `actor.jonas.pose.supported`, `actor.jonas.anim.supported_step` | Ankle protected before treatment, splint after; consistent proposed side, no healed duplicate character. |
| Lea | `actor.lea.model.base`, `actor.lea.outfit.station`, `actor.lea.outfit.exterior`, `actor.lea.expr.core` | Same face/cap/headset/blue-grey silhouette. |
| Lea | `actor.lea.pose.radio_manual`, `actor.lea.anim.radio_manual`, `actor.lea.pose.hands_free`, `actor.lea.anim.release_wheel` | Braced duty versus free hands based on real radio outcome; no spontaneous release. |

Reference artifacts: `actor.alex.reference.v1`, `actor.mara.reference.v1`,
`actor.jonas.reference.v1`, `actor.lea.reference.v1` map to the corresponding
`references/*-v1.png` **only when actually delivered**, with status/provenance in
REFERENCES.md. Prompts are supporting briefs, not proof of exact pixel reproduction.

## Model / texture handoff content [A], decisions still open

A later concrete actor asset needs a stable identity/review revision, selected
scale and proportions, aligned front/profile/back reference, approved face or
Alex casting policy, garment seam map, material swatches, pose/contact needs,
prop attachment plan and source/licensing record. Use floor/foot reference and
neutral pose units consistently across the four; final export units, pivots,
rig skeleton and texture budget are **[O]** for the actual asset stage.

For a 3D path, plan neutral base colour without baked emergency lighting,
roughness, restrained normal detail and explicit cloth/fleece/skin/rubber
material zones. Metal highlights belong to actual fittings, not cloth. These
are proposed deliverables, not generated texture maps. A 2D layered path needs
the same identity/occlusion/pose continuity without pretending a contact sheet
is already a sprite sheet. Bevy format selection is outside this PR.

Each actor needs dry station and restrained damp/frost overlays; preserve face,
colour layout and seams beneath them. Jonas needs untreated/treated ankle
coverage; Alex needs optional liner; Lea needs held/free hand poses, not two
different facial models. Snow/dampness is presentation and grants no resistance
bonus. Expression sets and partner animations are in PERFORMANCE.md.

Keep conditional props separate from costume models: use existing
`prop.copper_braid`, `prop.ceramic_mug`, `prop.spanner`, `prop.insulated_lever`,
`prop.safety_line`, `prop.flare`, `prop.heat_pack`, `prop.splint`, `prop.jonas_tag`,
`prop.handset`, `prop.torch` from the screenplay inventory. Do not mint alternate
IDs for duplicate mug/rope systems. Clip/cable placement is costume detail,
not an inventory framework.

## Consistency checks before accepting generated/outsourced assets

1. Confirm exact actor ID and reviewed reference revision; no invented character.
2. Compare head/face landmarks, hairline, cap/hood, body ratio and jacket silhouette
   in front/profile/back and neutral face close-up. Alex casting must remain open.
3. Compare seam/pocket count, reinforcement shapes, colour blocks and boot/glove
   shape under neutral light. Local fabric folds may vary, construction may not.
4. Check anatomical injury side in **body** coordinates; splint only treated
   variant, protected load in all poses. Correct rather than mirror a failure.
5. Check hands/contacts, no extra fingers/limbs, prop scale, unreachable wheel or
   support posture, clipping splint and glove grip. Concept art is not medical
   or rig proof.
6. Check optional/consumed/installed gear against actual source state and no
   resurrected cast in endings; lighting cannot invent a repaired device.
7. Compare silhouettes in greyscale and small framing; no reliance on colour alone.
8. Inspect torch/warm/snow/daylight derivatives against the same reference;
   do not regenerate faces independently for each shot. Reject unexplained drift.
9. Record tool/source/prompt/reference hash, selected artifact and reviewer result.
   A new variant is a named revision/derivative, not silent replacement.
10. Human approval is separate from this agent's inspection. No automatic approval,
    legal clearance, final production readiness or rights exclusivity is claimed.
