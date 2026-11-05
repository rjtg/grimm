# Cast continuity against the canonical screenplay

Baseline main **`09273b1`**, merged PR #77. Source hierarchy: runnable
[story rules](../../story.yaml), [canonical prose](../../text.yaml), full
[screenplay](../screenplay/README.md). Screenplay [A] staging is not promoted to
[C] biography. This task adds no state, predicate, possession or rescue rule.

## State → visible treatment, not new characters

| Existing source condition / event [C] | Allowed visual treatment [A] | Invariants / prohibited inference |
| --- | --- | --- |
| Initial blackout | Four crew members, Jonas injured, torch/radio contact. | Same faces/outfits as subsequent scenes. No free tool inventory; no named fifth crew member. |
| `thermal_liner=true` | Alex liner overlay/read at collar after pickup. | Alex's ordinary shell is not itself the gameplay liner. NPC costume does not alter Alex's exposure. |
| `jonas_treated=false` | Protected ankle, resting/support pose, no fitted treatment splint. | No uninjured idle/walk. Proposed injury side remains same body side. |
| `jonas_treated=true` | Splint and careful brace, medication implied by treatment performance. | Not healed; no amputation or new blood/wound. May still need carrying. |
| `generator_done=false` | Colder body language, no restored-heat reaction. | Costume alone cannot provide power. No quantitative physiological simulation. |
| `generator_done=true` | Shivering eases, warm practical light, Alex can recover hands via an authored action. | Do not undo ankle injury or radio-first history; no instant hand recovery on camera cut. |
| Action outcome reports cold grip / alignment / transmission failure | Specific failed grasp/gesture, tired posture. | No success prop state/celebration; no new frostbite injury or permanent disability. |
| `first_task==2` | Jonas's later supported mobility reflects longer unheated wait. | Later power/hand warming does not clear this fact. No visible death before survivor resolution. |
| `copper_braid`, `ceramic_mug`, `spanner` | Selected carried props only while possessed. | Concept kit cannot mint spare copper; don't reuse mug as costume emblem. |
| `lever=true` | Insulated tool replaces consumed spanner/braid possession silhouettes. | Do not show a second usable bare spanner or braid from a hip pouch. |
| `radio_adapter=true`, `radio_auto=false` | Installed mug assembly, no carried duplicate; success not yet implied. | Lea is not automatically free on assembly alone. Conditional gust still possible until auto success. |
| `radio_done=true`, `radio_auto=false` | Lea retains braced manual duty; voice-only from remote shots. | Successful call removes reserve deadline but not her responsibility. No duplicate Lea in dispatch and mast. |
| `radio_auto=true` | Lea releases wheel with shaking hands; free-hand variant. | Same cap, face and gloves. No replay of obsolete mandatory hold/gust. |
| Radio fails without completed call | Lea comes inside after timeout/abandon. | Do not invent a finished call or keep her at an unnecessary mast. |
| `safety_line=true` | Coil/attachment and accepted coordinated rescue staging. | Possession alone cannot silently improve a different chosen rescue action. No extra rope-count rule. |
| `engineering_lost` / `radio_lost` | Cast acknowledges closed access, routes no longer offered. | A door closure doesn't erase completed power/call or change actor identity. |
| `jonas_alive=false` | No live Jonas in partial pickup; Mara has tag coda. | No simultaneous cheerful Jonas model off to the side. |
| `lea_alive=false` | No live Lea in partial pickup; last-bearing absence. | Jonas-loss tag business must not substitute for Lea loss. |
| All alive in full result | Four occupied positions, injury/support retained. | Do not require a new treatment prerequisite on a full branch whose source doesn't have one. |
| All false in lost result | No surviving cast; exterior/name evidence only. | No cliffhanger silhouette of an alive Alex/Mara. |

Indoor/exterior outfit and snow/dampness are **[A] presentation variants chosen
by shot/environment**, not new global costume flags. They cannot change rates,
choice guards or timer behavior. Faces, proportions, colour blocks, garment seams
and injuries remain the same identity beneath them.

## Presence / interactions by canonical context

| Context / screenplay beats | Actor relationships and performance | Reference constraints |
| --- | --- | --- |
| `blackout`, B01 | Alex assesses; Mara counts/supports; Jonas protects ankle; Lea hears repeated breath. | Four-face ensemble; cause of repeated call remains unknown, no villain coding of Jonas. |
| `stores`, S01–S04 | Mara braces Jonas, Alex equips/treats/chooses, Jonas explains lever, Lea explains ceramic. | Splint only after treatment, one braid; log recognition is not a new betrayal. |
| `dispatch`, D01–D03 | Repeated same hall, Mara beside Jonas, Alex chooses/warm-hands, Lea contact/presence according to radio duty. | Stable positions; heat and repair-order effects distinct. |
| `generator`, G01–G08 | Alex works/starts; treated Jonas can brace low with Mara; Lea warns remotely. | Threshold assistance [A], no autonomous injured walk; overload/cold failure visibly distinct from ignition. |
| `radio`, R01–R07 | Alex assembles/aligns/transmits, Lea catches/holds/releases; Mara/Jonas remote or hall. | One Lea, same face and cap; wheel contacts, jumper possession and release follow actual outcome. |
| `crisis`, C01–C06 | Mara/Alex choose reach/carry/coordinate; Jonas/Lea spatially separated. | Do not conflate three rescue predicates; flare helper ambiguity remains source question, not new actor gesture effects. |
| `ending_full`, E01–E02 | Four responses, same injured Jonas; optional log clears cause. | Keep face/splint/gear continuity; no new rescuer cast or healed ankle. |
| `ending_partial`, E03–E04 | Exact named absence, Mara/Lea/Jonas reactions conditional. | Correct survivor cast; installed mug isn't returned to Lea by a grief prop gag. |
| `ending_lost`, E05 | Cast absent, broken loop/name evidence. | No living background actor. |

## Contradiction checks / editorial decisions

- Physical appearance is almost entirely unspecified in canon. All newly chosen
  faces, age impressions, colours, clothes/material details, sizes and handedness
  are **[A]** or **[O]**, never [C]. Teal/orange and blue-grey have previous [A]
  production provenance, not canonical status.
- Jonas's injury side is unspecified. v1 art proposes left; approval is open.
  This is one side for all views, not an accidental mirror change between scenes.
- Canonical help in annex versus hall rest is reconciled by the screenplay's
  proposed supported threshold blocking. Rehearsal remains needed; this bible
  does not add an escort action or alter latch help rules.
- Lea's owned gull mug is optional carried inventory and can become an installed
  jumper. Keep the gull on that prop, not permanently on every outfit.
- Alex's reference intentionally does not provide a definitive unmasked face.
  Costume/silhouette can be reviewed independently of player casting. Any visible
  generated eye region is not a locked demographic identity.
- Canonical cold thresholds are game-balance numbers. Do not derive medical
  diagnoses or scars from them. Gesture changes reflect authored outcomes only.
