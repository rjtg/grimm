# Character Bible review record and open decisions

## Scope and evidence

This task addresses #34 under #24, based on merged PR #77 (`09273b1`). All four
actor IDs are inherited, the full screenplay is retained, and canonical
`story.yaml`, `text.yaml`, `scenes.yaml`, playthroughs and Rust source remain
unchanged. No new gameplay state, timers, engine/renderer APIs, rigs or final
animation assets are introduced.

| Requirement | Evidence |
| --- | --- |
| Four characters and source extraction | ALEX/MARA/JONAS/LEA pages distinguish source role, relations, physical facts, props, injury, behavior and appearances from new casting design. |
| Coherent visual direction | STYLE compares three approaches and selects restrained cinematic naturalism; shared materials, colour/value, winter wear, lighting and silhouette rules. |
| Detailed specifications | Each page has build/proportions, face/headwear, layers, palette/material, silhouette, stance/gear, light and variant notes. All invented traits are [A] or [O]. |
| Actual visual references | Four PNG contact sheets in `references/`, each with complete front/profile/back, neutral close-up, expressions/pose studies. Exact prompts and hashes in REFERENCES. Jonas is seated/protected, Alex face intentionally masked rather than invented casting. |
| Consistent identity across variants | CONTINUITY maps actual WorldState/events to visible treatment without making new characters or adding rules. Face/wardrobe invariants and conditional equipment are explicit. |
| Performance planning | PERFORMANCE distinguishes reusable families, actor-specific work and one-off staging with cast and screenplay references. No clip-completion gameplay effects. |
| Stable asset preparation | ASSETS separates existing actor roots, planned models/outfits/poses/expressions and actual reference artifacts; model/texture/pose handoff and acceptance checks. |
| Locations / camera boundary | ENVIRONMENT-HANDOFF lists cast/prop/reach/visibility requirements by existing set/shot, keeps environment elaboration in #35. |
| Provenance / review | Built-in image generation documented; original prompts, no external image inputs, explicit proposal/approval status and no invented backend/version or asset license. Agent visual review in REFERENCES. |

All generated sheets were visually inspected. Written prompts do not themselves
prove identity consistency; selected visible output is retained. No claim of
human art approval, final casting, pixel-exact regeneration, medical validation,
metric orthographic accuracy, working rig or small-screen playtest is made.
The four sheets and detailed briefs are reviewable authoring artifacts.

## Open decisions [O]

1. Approve or revise visual casting proposals for Mara/Jonas/Lea, colour/cut and
   face landmarks. Ages/body scales are proposed ranges, not canonical facts.
2. Choose Alex casting policy separately from the approved costume envelope.
   v1 supplies a masked upper-face close-up, **not** an unmasked face. No gender,
   precise age, ethnicity, hairstyle or voice is locked by incidental pixels.
3. Approve Jonas's anatomical injury side (left proposed in v1); rehearse support,
   carry and brace safely with #35 before locking rig contacts. No gameplay
   change, healing or new injury is implied by the side choice.
4. Approve practical prop mounting/handedness (e.g. Mara's candidate torch clip),
   cap/headset routing and hood-down variants. Keep chosen details stable rather
   than mirroring them arbitrarily between shots.
5. Resolve final scale/export/texture budget and 2D versus 3D production path in
   the later concrete asset task, not a universal character framework here.
6. Table-read/VO length, the screenplay's flare-helper imagery ambiguity and
   injury-safe spatial layout remain the existing production questions. This
   bible does not rewrite their source rules to resolve them.

## Verification

Documentation/reference audit and required native workspace checks are being
run before final PR readiness; final results will replace this paragraph.
No source-file LSP diagnostics apply to Markdown/text/PNG-only changes. Existing
Rust tests remain appropriate regression evidence; no tests assert exact artwork
or dictate the author's dialogue/face choices.
