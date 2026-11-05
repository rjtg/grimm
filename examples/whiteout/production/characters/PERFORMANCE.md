# Performance inventory — planned clips, no animations supplied

Stable screenplay `anim.*` IDs remain semantic production references. The actor
IDs below specialize their performance where necessary; they are not new GRIMM
cue types, handlers or a general animation framework. **[A]** All staging/clip
choices here are proposals realizing the cited **[C]** action or reaction.

| Class / production reference | Actors | Performance and invariants | Screenplay association |
| --- | --- | --- | --- |
| Shared `idle.breathe` | All four | Small breathing/listening loop; Jonas injured version never shifts weight onto bad ankle. Does not tick cold gameplay. | B01, S01, D01–D02, R01, C01 |
| Shared `move.cautious` | Alex, Mara, Lea; Jonas only supported variant | Short practical steps, credible winter boot placement; no free independent Jonas walk. | S04, D01, G08, R07, C03–C04 |
| Shared `talk.listen`, `gaze.partner` | All four | Eyelines tied to actual speaker/location; radio listening gaze to handset, no duplicate actor. Viseme/voice design remains future work. | B01, S01, D01–D02, G06, R04–R05 |
| Shared `reach.device`, `anim.pickup` | Alex, contextual Lea | Gloved reach/hold, prop attachment only after accepted source effect. | S02, G02/G04/G05, R02 |
| Shared `cold.shiver` | All four where staged | Restrained shoulders/breath, same body/face; NPC acting not a new exposure-rate rule. | B01, D02, R01, C01 |
| Shared `anim.warm_hands` | Alex | Recovering grip over real restored heater; no instant injury fix. | D03, R06 |
| Shared expression core | Mara, Jonas, Lea; Alex upper-face/body subset | Neutral, focused, concerned, effort, tired relief, grief; same facial landmarks. Don't force a fixed player personality. | Entire script, branch-specific endings |
| Character `actor.mara.anim.support_jonas` / `anim.support_carry` | Mara + Alex + Jonas | Partnered contact, splint clearance, shared weight, protected ankle; not a generic solo grab. | B01→S01, S02/S04, G01/G08, C04, E01 |
| Character `actor.jonas.anim.supported_step` / `anim.support_step` | Jonas + Mara/Alex | Careful supported crossing only on permitted branch. Same injury and load restrictions after heat. | C02–C04, E01 |
| Character `actor.lea.anim.radio_manual` / `anim.hold_wheel` | Lea | Sustained shoulder/hand contacts, listening while holding; can persist under other cues. | R03/R05, manual C01/C04 |
| Character `actor.lea.anim.release_wheel` / `anim.release_wheel` | Lea | Fingers uncurl and shake only after hands-free success/actual release conditions. | R05, automatic C01/E01 |
| Character `anim.jonas_cold`, `anim.jonas_supported` | Jonas | Seated shiver / supported low brace, protected leg; no guilt caricature. | D02, G01/G03 |
| Character `anim.latch_work`, `anim.grip_fail`, `anim.prime` | Alex, Jonas brace if eligible | Same hinge progress; actual failure doesn't move pin. Primer fuel and ignition only accepted success. | G03, G07 |
| Character `anim.align_dish`, `anim.catch_wheel` | Alex + Lea | Fine alignment versus transient gust catch; don't grant aligned state by animation. No broken wrist. | R03, R04 |
| Scene `anim.equip`, `anim.splint` | Alex + Jonas + Mara | Liner after pickup; splint after treatment. Partner holds ankle space, Mara wrist squeeze. | S02 |
| Scene `count_four` / `anim.name_response` | Mara + all survivors | Opening four-face count matched to final responses. Partial/lost cannot reuse a four-survivor loop. | B01, E01/E03/E05 |
| Scene `recognize_bypass` | Jonas + Alex | Diagram recognition/exhale; no smirk implying sabotage. | S03, G05 |
| Scene `anim.stop_plan`, `anim.rig_line` | Mara + Alex (+ crew) | Failed coordinate stops before run; successful plan uses possessed gear. | C02 |
| Scene `anim.reach_lea`, `anim.fire_flare` | Alex, relevant partners | Exact selected route; firing only explicit consuming flare-retreat action. No automatic coordinated rescue. | C03, C05 |
| Scene `anim.unanswered_call`, `tag_grief`, `empty_handset` | Mara, Lea/Jonas as actual survivors | Right named absence, same face and prop history, no resurrected offscreen actor. | C06, E04 |

Shared labels without `anim.*` are descriptive performance families, not added
screenplay IDs to be mistaken for already bound runtime clips. Catalogued
actor-specific `*.anim.*` IDs are in ASSETS.md. Existing `move.jonas`/`move.mara`
are only geometric-root motion in the prototype, not proof of completed rigs.

## Timing / blending / partner production notes [A]

Breathing, listening and the manual hold can coexist with radio dialogue and a
scheduled gust. Camera motion must not terminate a hand contact. A later REPLACE
cancels obsolete performance instances while preserving correct destination
poses; it cannot revive a stale manual requirement or replay an accepted action.

Action work and presentation are different. A 3500ms latch depiction can overlap
the T=10000 warning even when logical work eligibility is floored at S+3000ms.
Startup's 1200/1500/1800ms sequence locks input according to the existing story;
no rig completion callback commits power. This bible changes no durations,
input gates, deadlines or graph lifecycle.

Before final rigs, record hand/rail heights, support partners, anatomical injury
side, glove thickness, boot clearance, line routing, pose entry/exit, interruption
pose and camera visibility. These are production requirements, not new host
API contracts. Rehearse injury/carry and wheel contacts with #35 layout first.
