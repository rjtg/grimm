# `actor.lea` — the radio must not work by itself

Voice: `voice.lea`. One person across handset audio, mast performance and ending.

## Canonical extraction [C]

Opening radio repeats an unrecognized call. Lea explains ceramic insulation and
owns the mug with a painted gull. She announces generator elapsed time, catches
a mast gust and can hold the dish manually. A successful jumper lets her release
the wheel; manual success still needs her there. She warns Alex not to forget
her. After Jonas loss, she stops asking whether another minute would help;
a Lea-loss branch leaves her final bearing and lost antenna shelter. No fixed
age, appearance, skin, hair, height, dominant hand or official rank is stated.

Script: B01, S01/S02, generator G04/G06 over radio, R01–R07, C01–C06,
E01/E03/E04; dispatch only as presence/contact permits. E05 shows no survivor.
Do not read the opening's mystery as an invented supernatural backstory for her.

## Design specification — all visible casting details [A]

| Element | Proposed stable design |
| --- | --- |
| Build | Narrow upright adult torso with practical long forearms, 7–7.5 heads, 1.68–1.76m proposed layout range. Reach based on a workable handwheel, not elongated cartoon limbs. |
| Face | Early-thirties impression, medium olive skin, softly angular jaw, straight dark eyebrows; focused listening, not glamour. |
| Hair / cap | Straight black hair tucked into charcoal ribbed knit cap, small consistent temple edges; cap seam/headset placement repeat in all views. Exterior hood fits over rather than redesigns the head. |
| Layers | Blue-grey `#4F6178` longer, thigh-skimming field shell, dull cream `#B7B4A6` fleece collar, charcoal trousers, dark winter boots and plain grey gloves. |
| Materials | Matte ripstop and ribbed knit, textured fleece, restrained damp cuffs and scuffed boot toes. Same hem/pockets/gear routing across views. |
| Signature silhouette | Narrower vertical coat than Mara, cap/headset profile, close listening tilt; wheel-hold stance changes posture, not face/body identity. |
| Props | Handheld/headset as staged; short secured cable, no tactical vest. Gull belongs to the optional mug, **not** a permanent jacket logo. Mug disappears from carried gear when installed as jumper. |
| Posture | Attentive listening, shoulder braced at wheel when required, both hands visibly strained; release produces shaking hands, not relaxed glamour pose. |
| Light | Blue-grey stays cooler/lighter than Mara teal; dull cream collar separates jaw in torch/radio shots. Snow edge light preserves cap/headset silhouette, never a glowing accessory. |

## States and performances

**[C→A]** Before manual alignment, gust catch is a momentary arrest, not an
unearned aligned dish. After manual success, sustained `radio_manual` pose
persists through remote dialogue; Lea cannot sit in dispatch while her visible
duplicate holds the mast. A completed call cancels reserve pressure but does not
free a manual antenna operator.

**[C→A]** `radio_adapter=true` shows the installed mug/braid but does not itself
invent a successful call or automatic mode. `radio_auto=true` after successful
transmission permits the free-hand release performance and suppresses the
pending conditional gust. If no call exists when cabin reserve fails, Lea comes
inside. Source success normally cancels timeout; do not stage a forced later
failure just to show a variant.

**[C→A]** Generator heat can improve cabin conditions, not automatically set
alignment/call. Gloves/cap/exterior shell are protective costume, not added
bonuses or possession of `thermal_liner` for Alex.

**[A]** Expressions: neutral listening, concentration, caught alarm, physical
strain, tired relief, survivor grief. Maintain eyes/jaw/cap seam in every study.
The same eyebrow movement reads differently at neutral/warm/ridge lighting;
no new face to intensify emotion.

**[C→A]** If Lea dies, no alive model in pickup transport. If Jonas dies, Lea's
unfinished question is grief, not a new resentment/relationship stat. A carried
mug must not reappear after installation for a comforting ending gag.

**[O]** Final casting and cap/headset design; handwheel contact geometry,
injury-safe catch animation, audio-only presence conventions, final voice/accent.
A concept wheel is not an environment specification for #35.

Assets: `actor.lea.model.base`, `actor.lea.outfit.station`,
`actor.lea.outfit.exterior`, `actor.lea.expr.core`,
`actor.lea.pose.radio_manual`, `actor.lea.anim.radio_manual`,
`actor.lea.pose.hands_free`, `actor.lea.anim.release_wheel`.
