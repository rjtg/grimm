# `actor.jonas` — injured, useful, not guilty

Voice: `voice.jonas`. Injury/state variants are the same actor, never a new
identity named “injured Jonas”.

## Canonical extraction [C]

`blackout.arrival` establishes a crushed ankle. `stores.jonas_treated` splints it
and gives medication; Jonas can brace the generator and may walk with help if
heat returns soon. Mara supports him. He knows the latch/insulated lever and
recognizes his signed temporary maintenance bypass. The log says it was not
sabotage. Shivering eases with heat; the unheated radio-first wait still affects
later rescue. Full rescue retains him; partial-Jonas text says he never reaches
the crossing and Mara keeps his tag. No age, face, hair, skin colour, height,
injured side or pre-existing scar is canonical. Ankle injury is not amputation.

Script references: B01, S01–S03, D01–D03, G01/G03/G05/G08, C01–C06,
E01/E03/E04/E05 (the last as absence). He is not duplicated at the radio cabin
while Lea and Alex work there; diegetic contact can carry a line.

## Design specification — visible casting traits [A]

| Element | Proposed stable design |
| --- | --- |
| Build | Tall broad adult with longer torso and practical shoulders, approximately 7.5 heads; 1.80–1.88m layout range. Same proportions seated or supported. |
| Face | Mid-forties impression, light tan skin, long face, broad straight nose, short greying brown hair and salt-and-pepper stubble. Neutral fatigue, no villain scar/shadow coding. |
| Headwear | Short hair visible indoors; exterior hood fits around same hairline. No new cap identifying a different actor. |
| Layers | Grey fleece under desaturated ochre-orange `#9B6338` insulated shell, soot-grey `#424441` reinforcement, charcoal work trousers, dark winter boots and plain grey gloves. |
| Materials | Matte worn ripstop; modest cuff dampness, boot wear and knee creasing from seated posture. No blood/gore or random tears implying another injury. |
| Signature silhouette | Long orange torso, protective low seated/leaning pose, one extended unloaded leg. Size and weight remain human; not an oversized rescuer. |
| Props | Fitted splint only after treatment; medical tag appears in the appropriate narrative. Tool grip/frame brace is not a new inventory ability. |
| Posture | Resting, supported brace or supported step as actual outcome permits; no healthy upright turnaround that erases injury just for art convenience. |
| Light | Ochre must stay distinct from Mara's teal and warm station light; keep material value contrast in greyscale. Cold colour grade does not turn him a different ethnicity/age. |

## Injury contract and variants

**[C]** Before treatment: ankle remains injured, no fitted treatment splint yet;
protected rest and Mara's support. **[C]** After treatment: medication and splint,
more useful support, never normal healed gait. **[A]** The v1 reference proposes
**left ankle** purely for consistent drawing/contacts. **[O]** Injured side needs
maintainer approval; source does not fix it. Once selected, front/back/profile,
carry grips and all later variants must use the same anatomical side. Do not
mirror the drawing and accidentally swap it. An artist must not read “left on
the image” as “left on the body”.

The calibration reference uses supported seated front/side/back, not unsupported
standing. A neutral uninjured model-development pose may later exist internally,
but is not playable evidence of health and must not leak into story shots.

**[C→A]** `jonas_treated` controls splint/help portrayal. `generator_done` can
reduce visible shivering; `first_task==2` and later rescue outcome still matter.
No new ankle-health counter or timed recovery is introduced. Warming Alex's hands
does not heal Jonas. Source `help_jonas` can save everyone without treatment when
its own power/automatic-radio/cold conditions succeed; do not add a treatment
requirement just because a reference depicts a splint.

**[A]** Expressions: neutral fatigue, recognition at wiring sketch, relief at
isolation, strain, colder withdrawn attention, restrained survivor grief. Keep
nose/chin/stubble and hairline identical. No guilty smirk on seeing the log.

**[C→A]** `jonas_alive=false`: no living background model in partial ending.
Tag is a prop in Mara's glove; body need not be shown. If alive, keep ankle care
visible in rescue transport. Lost ending has no surviving cast cameo.

**[O]** Final casting, injured side, safe frame-bracing geometry and supported
carry contacts must be reviewed together with #35 layout. Do not infer medical
accuracy or approved rig constraints from a generated concept sheet.

Assets: `actor.jonas.model.base`, `actor.jonas.outfit.station`,
`actor.jonas.outfit.exterior`, `actor.jonas.variant.untreated`,
`actor.jonas.variant.treated`, `actor.jonas.pose.injured`,
`actor.jonas.pose.supported`, `actor.jonas.anim.supported_step`,
`actor.jonas.expr.core`. Variant IDs do not create new actor identities.
