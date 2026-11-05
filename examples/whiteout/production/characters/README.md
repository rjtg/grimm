# WHITEOUT Character Bible — v1 proposals for review (#34)

Baseline **main `09273b1`**, after [PR #77](https://github.com/rjtg/grimm/pull/77).
Four persistent virtual actors, authored from the complete screenplay and
canonical game. This is a **human-reviewable reference**, not final production
meshes, rigs, animation clips or approved casting.

## Authority and reading order

- **[C] Canon:** facts/conditions already in `story.yaml` / `text.yaml`.
- **[A] Art proposal:** chosen production direction, new appearance/clothing,
  blocking/performance interpretation. Not retroactive story canon.
- **[O] Open:** explicit decision still awaiting maintainer/production review.

After review, use the accepted revision's identity invariants as the reference
for derivatives. Until then all generated sheets are labelled **DESIGN PROPOSAL**.
Do not claim human approval or infer a face/age from incidental generated detail.

1. [Shared style decision](STYLE.md): three directions compared, cinematic
   naturalism selected, palette/material/light and silhouette rules.
2. Individual specifications: [Alex](ALEX.md), [Mara](MARA.md),
   [Jonas](JONAS.md), [Lea](LEA.md).
3. [WorldState/presence/wardrobe continuity](CONTINUITY.md).
4. [Performance inventory](PERFORMANCE.md): shared, actor-specific and scene work.
5. [Stable character asset catalogue](ASSETS.md) and downstream checks.
6. [Reference sheets, exact prompts, provenance and inspection](REFERENCES.md).
7. [Cast handoff to #35](ENVIRONMENT-HANDOFF.md), without implementing locations.
8. [Verification and outstanding decisions](REVIEW.md).

## Identity at a glance — all visual details [A]

| Stable actor | Silhouette / colour | Canonical continuity that must survive |
| --- | --- | --- |
| `actor.alex` | Neutral hooded work suit, slate/sand, engineer hand poses. | Player field engineer; no fixed gender, precise age, ethnicity or face. Optional liner/gear follows choices. |
| `actor.mara` | Compact teal mass, short hem, stable supporting stance. | Counts/carries/cares, protects injured ankle, final name count or exact casualty grief. |
| `actor.jonas` | Longer broad ochre torso, protected unloaded ankle. | Injured from opening; treatment helps, never heals. Bypass is a mistake, not sabotage. |
| `actor.lea` | Narrower blue-grey coat, cream collar, cap/headset, wheel stance. | One operator; manual duty persists, automatic success frees hands. Mug is a conditional prop, not costume. |

Alex's face reference is intentionally a masked upper-face study, not a definitive
unmasked casting. That limitation is documented, with reproducible prompts. Jonas
uses proposed left-ankle continuity in concept art, **not** a claim of canonical
side. Concept images cannot mint inventory or add gameplay bonuses.

No engine/renderer/story changes, character-generation framework, final models,
rigs, assets pipeline, IDE, codegen (#75) or Save/Restore (#63). The existing Bevy
prototype remains unchanged; these references guide later asset work.

Refs [#34](https://github.com/rjtg/grimm/issues/34),
[#24](https://github.com/rjtg/grimm/issues/24);
environment work remains [#35](https://github.com/rjtg/grimm/issues/35).
