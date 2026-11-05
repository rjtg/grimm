# `actor.alex` — the player's field engineer

Voice identity: `voice.alex`. Stable actor ID inherited from the screenplay,
not a second player model. This page governs clothing/body continuity while
**[O] facial casting, gender, ethnicity, precise age and voice remain unassigned**.
A concept candidate cannot silently settle them.

## Canonical extraction [C]

- `blackout.arrival`: player is Alex, Aurora's field engineer; four people depend
  on their repair/rescue choices. No canonical body size, face, hair, gender,
  age, skin tone or dominant hand is specified.
- `stores.thermal_liner` explicitly places the liner over Alex's coveralls.
  Work prose refers to gloves; their construction/colour is not specified.
- `stores.*`: can equip a liner, treat Jonas, take tools/line/flare/battery and
  read the log. None is present merely because concept art shows field clothing.
- `generator.*`: work/lever/primer actions can fail through cold grip. This is
  not a permanent disability or incompetence; no scar is implied.
- `radio.*`: fine manipulation, alignment and transmission; can warm hands with
  power. Cold recovery does not change repair order or Jonas's history.
- `crisis.*`: choices coordinate or prioritize people; final `alex_alive` controls
  survival. Lost ending shows evidence, not an alive Alex cameo.
- Relationships: practical responsibility to Mara, Jonas and Lea; no invented
  romantic attachment, hierarchy, pronouns or biography. Personality is expressed
  through player choices, not one compulsory heroic/cynical face.

Sources: screenplay B01, S02–S04, D01–D03, G01–G08, R01–R07, C01–C06 and all
endings; see [source map](CONTINUITY.md). Added dialogue in that script is [A].

## Design specification [A], with casting boundary [O]

| Element | Proposed design / invariant |
| --- | --- |
| Build | Neutral adult, balanced functional proportions; 7–7.5 heads, insulated volume without exaggerated gender cues. Proposed scale range 1.72–1.82m, not a fixed player statistic. |
| Head / face | Cold-weather hood plus plain lower-face covering in the reference sheet. Upper-face neutral portrait and eye/brow effort studies; **no definitive unmasked portrait**. Facial anatomy, skin/hair details remain open; do not infer them from incidental generated pixels. |
| Clothing | Slate hip-length insulated work shell over coveralls, muted sand elbow/shoulder patches; charcoal straight work trousers, dark boots, plain insulated gloves. No armour or rank. |
| Layers | Thin base layer, work coveralls, shell; thermal liner is a separate authored possession/overlay, never an inherent power-up of the shell drawing. Indoor hood-down view may conceal/crop face until casting review. |
| Materials | Matte ripstop, fleece neck, sturdy woven patches, scuffed rubber toes; no bright tech trim. Same pocket/seam layout throughout sheet. |
| Distinctive cue | Slate/sand block pattern and purposeful two-hand device handling, neither a face nor a weapon. No mandatory personal jewellery or tattoo. |
| Stance | Balanced at device, elbows close against cold; cautious low centre when supporting another person. Avoid default “hero with fists on hips”. |
| Props | Torch and handset are shared diegetic staging props; spanner/braid/lever, mug, pack, line and flare **only when actual source state permits**. Don't bake a spare toolkit or weapon into the silhouette. |
| Light | Torch reveals glove/patch edges; warm duct softens hands; snow bounce retains slate/sand contrast. Same coat, not a white snow-camouflage redesign. |

The masked sheet is an intentional **clothing/identity envelope**, not a claim
that the story requires Alex always masked indoors. **[O]** Decide player casting
policy and whether an unmasked close-up is desirable before facial production.
A requested “face close-up” is delivered as a masked upper-face study; the missing
unmasked casting is explicit, not falsely presented as finalized.

## Variants and performance

- **[A] station / exterior:** same base/shell identity, hood can be lowered or
  raised; no free equipment acquired on dressing. Reference sheet uses exterior
  headwear to preserve open casting.
- **[C→A] liner taken:** add the agreed liner read at collar/torso after
  `thermal_liner=true`; the already functional shell is not evidence of this flag.
- **[C→A] cold / recovered:** effort, shiver and grasp fidelity follow the actual
  accepted action/outcome. Use grip-fail versus primer success distinctly. Do not
  render injury or change body age as a cold effect.
- **[C→A] rescue:** carry/grab/line use follows the chosen branch and real gear.
  On full outcome retain tiredness and outfit wear; no magically clean hero suit.
- **[O] emotions:** neutral listening and restrained concentration are safe
  defaults; other reactions must respect player choice rather than author a new
  fixed personality.

Assets: `actor.alex.model.base`, `actor.alex.outfit.station`,
`actor.alex.outfit.exterior`, `actor.alex.outfit.liner`,
`actor.alex.pose.device`, `actor.alex.pose.cold`, `actor.alex.expr.upper_face`.
These are editorial asset IDs, not implemented model files or gameplay flags.
