# WHITEOUT cast — visual direction v1

**[A] Selected production proposal:** restrained cinematic naturalism, expressed
as painterly reference art with realistic anatomy and simplified material masses.
This document is binding guidance for deriving assets from a **reviewed revision**;
its proposed casting details are not retroactive story canon. **[O]** Maintainer
approval of casting and final art is still required. No approved-art claim.

## Three directions considered

| Direction | Strength | Cost / weakness for this cast | Decision |
| --- | --- | --- | --- |
| Photographic microdetail | Faces and winter fabrics can look immediate in close-up. | Generated skin/pores and fine garment details drift between shots; costly to match with unfinished real-time assets, poor silhouette payoff. | Not the baseline. |
| Strongly graphic / exaggerated proportions | Easy silhouette separation, low detail burden. | Risks trivializing injured weight and understated crew performances; exaggerated faces depart from the full screenplay's tone. | Keep readability, not exaggeration. |
| Restrained cinematic naturalism | Human proportions, practical silhouettes, broad colour/material blocks; expressive weight and hands. | Requires consistency review and layout rehearsal, but does not depend on photoreal skin or elaborate shaders. | Selected [A]. |

No engine comparison: Bevy remains the graphical host. The existing sprite/set
prototype can already test massing and colour. Later 2D layered actors or modest
3D models can derive from these sheets; this task chooses neither a new renderer
nor a rig format. Fine facial detail cannot be a prerequisite for identifying a
cast member. Do not claim a concept image is automatically a usable game asset.

## Shared shape and proportion rules [A]

Real adults, approximately 7–7.5 head units rather than heroic proportions.
Use insulated clothing volume honestly: elbows bend, shoulders can reach a
handwheel and jacket hems clear hips in crouch. No exposed skin for glamour,
armour plating, glowing trim, dangling ornaments or unexplained weapons.
Proposed scale envelope (not gameplay stats): Alex 1.72–1.82m; Mara 1.66–1.74m;
Jonas 1.80–1.88m; Lea 1.68–1.76m. These are **[A] layout ranges**, not cast canon
or final mesh measurements. **[O]** Final casting/model scale. Mara is compact,
Jonas has a longer broad torso, Lea a narrower vertical line; Alex's neutral
engineer silhouette is defined by the work suit and hood, not gendered anatomy.

All actors have believable winter boots, gloves and layered work clothes.
Base station layer can remain under the outdoor shell. No scene introduces an
unearned jacket, toolkit, radio, line or medicine as a gameplay capability.
Colour/cut is identity; inventory attachments follow authored state.

## Palette and material rules [A]

| Actor | Dominant shell | Secondary / silhouette cue |
| --- | --- | --- |
| `actor.alex` | Slate `#505D63` | Muted sand reinforcement `#8B8270`; hooded workwear, engineer hand poses. |
| `actor.mara` | Deep teal `#355E60` | Charcoal `#30363A`; short hem, low folded hood, stable square stance. |
| `actor.jonas` | Desaturated ochre-orange `#9B6338` | Soot grey `#424441`; longer torso, injured protected leg and supported weight. |
| `actor.lea` | Blue-grey `#4F6178` | Dull cream `#B7B4A6`; narrower shell, ribbed cap and compact radio gear. |

No faction meanings or rank system. Hex values are neutral-reference swatches,
not a requirement that lit pixels retain exact RGB. Retain their relative
value/temperature under lighting; silhouettes and gear must work in greyscale.
Teal Mara / orange Jonas extend the existing prototype's **[A]** colour coding;
blue-grey Lea follows PR #77's proposal. None is a canonical wardrobe fact.

Matte ripstop shells, brushed fleece collars, knitted caps, scuffed rubber boot
toes, restrained metal/ceramic highlights. Avoid wet-looking plastic skin,
mirror-finish cloth and random “grunge” that changes every generation. Place wear
at cuffs, knees and boot edges consistently. Snow sits on upward-facing shell
surfaces; damp hems darken, not recolour an entire costume. Weathering is staging,
not an extra exposure or inventory variable.

## Light continuity and shot readability [A]

| Condition | Requirements |
| --- | --- |
| Neutral sheet | Broad soft studio light, plain grey backdrop; full body, same scale, no perspective tricks hiding seams. Establish material albedo and face identity. |
| Blackout / torch | Keep faces and glove action visible inside a motivated torch pool. A thin cool edge separates dark coats from wall. No built-in glowing costume trims. |
| Warm powered interior | Warm key on skin/fleece, cool window rim; same colour blocks and material roughness. Heat changes posture, never heals Jonas. |
| Radio / snow bounce | Cool diffuse fill, small equipment practicals, dark handwheel readable against gloves. Lea's profile and cap/headset remain clear. |
| Ridge | Broad snow bounce and windward frost, face/hand contrast maintained; line and ankle remain legible. No blizzard filter that hides the decision targets. |
| Ending daylight | Reduced emergency contrast; retain injury, wear and survivor-specific absence. Do not beautify faces into different actors. |

Reference sheets include neutral faces plus small performance studies, not
independent redesigns. The same hairline, nose/chin proportions, headwear seams,
colour blocks and boot/jacket shapes must survive front/side/back and close-up.
A hood can occlude a face; it cannot justify substituting a different face later.
Alex's unassigned casting is treated separately in their specification.

## Reference practice

Use each actor's single contact sheet and written invariants together. A fresh
text prompt alone does not establish identity. For later variants, provide the
reviewed sheet as identity reference, change only the named state and inspect
again. Similar lighting is not proof of the same face. Never mirror Jonas's
injury or a reference's garment asymmetry silently. No celebrity/reference artist
is requested; no third-party photos are imported. Provenance and review status
are recorded in REFERENCES.md.
