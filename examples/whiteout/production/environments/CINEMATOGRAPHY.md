# WHITEOUT cinematography — orientation before spectacle

All lens/framing/blocking choices here are **[A]** production proposals, not new
GRIMM instructions or gameplay cameras. Existing `cam.*` identities come from
PR #77. Three new establishing identities are explicitly [A]; concrete `shot.*`
prototype bindings remain implementation references, not exported metre/lens data.

## Shared film language

1. Establish the room and choice targets before a technical insert. Return to
   the same geography after an insert, not a mirrored newly generated set.
2. Mostly fixed human-height views, restrained short pushes/tracks. A 35–55mm
   equivalent perspective is a starting [A] framing shorthand, not an engine
   projection mandate. 2D/2.5D layers can reproduce framing without a literal
   lens. Avoid wide distortion of injury/hand contacts; no orbiting action camera.
3. Medium dialogue holds shoulder/hand business. Radio voice can overlay the
   working shot; not every line needs a cut to a physically absent speaker.
4. Close-ups show what makes a choice intelligible: same hinge, insulation gap,
   grip, coil/readout, log knowledge. They do not acquire clues or set state.
5. Tension is diminishing clear space, restrained motion and sound, not arbitrary
   shaking that hides timer/action target. Camera animation never pauses a clock.
6. Reframe transitions through a threshold on accepted GOTO. Ordinary staging
   nodes may bridge sets later, but add neither travel cost nor an extra rule.

## Axis and eye-line contract

PLANS uses south-view north for major sets. Hall radio stays screen-left,
engineering right, ridge ahead. Ridge Lea/shelter stays left, Jonas/manifold
right. Generator coverage stays south of front service plane; fuse remains
right of starter. Radio desk left and wheel right share one connected view.

Draw an interaction axis between partners or operator/device; keep coverage on
its established side. To cross it, show a neutral-on-axis or wide move that
re-establishes landmarks **before** the reverse close-up. No reverse generated
image may merely swap door, injury side, handwheel or mast location. Partners'
gazes reference their actual physical position or the handset, not an invisible
second Lea. Offscreen radio is an audio relationship, not co-location.

Jonas's proposed left injury side is anatomical, not screen-left. The injured
leg stays protected in reverse/support shots. Lighting and edits cannot justify
swapping the cast reference face, body or garment details.

## Reusable setup catalogue

Depth column states future realization needs, **not** a new blanket renderer
choice. “3D useful” means practical for that contact/parallax, not required
engine work in this PR. All actual variants require shot/cast/GUI rehearsal.

| Stable setup ID(s) | Set / coverage and axis | Depth / realization | Needed motion / visibility |
| --- | --- | --- | --- |
| `cam.corridor_establish` **new [A]** | Corridor four-face/light-failure axis. | Layered walls/windows + foreground cast usually enough; shallow 3D useful for carry turn. | Light segments, torch sweep, protected ankle insert; retain four-person count. |
| `cam.radio_insert`, `cam.injury_insert` | Handset/ankle from same established side. | Flat insert can work; 3D useful for gloved contact/carry clearance. | Loop breath versus live reply; splint only treated, no new injury reveal. |
| `cam.stores_establish` **new [A]** | South threshold → bench/tray/rack/log. | Layered set sufficient for fixed view; 3D useful for side-step to stations. | Four cast marks and clear transfer lane, no invented second exit. |
| `cam.stores_tray`, `cam.log_insert`, `cam.tool_insert` | Low tray/terminal/tool on southern coverage. | Detailed 2D insert/prop layers sufficient; 3D helps threaded assembly/hand occlusion. | Single braid, gull handle, sketch; log legibility only selected reveal. |
| `cam.hall_routes`, `cam.line_insert` | Repeat hall south view; real carried line insert. | 2.5D layers can reuse set/state light; shallow 3D helps support movement. | Fixed doors/duct, correct Lea presence, visible line only if possessed. |
| `cam.generator_establish` **new [A]** | Southwest → front housing, rail, right fuse. | Layered casing/rail can establish; 3D useful for operator shifting/primer/latch. | Working strip, same hinge progress, clear retreat. |
| `cam.coil_insert` | Same entrance coil/gauge/readout; no new timer. | 2D gauge/overlay enough; 3D not needed merely to display numbers. | Hidden → reveal current remaining budget, extension not restart. |
| `cam.radio_axis`, `cam.jumper_insert` | South cabin: desk left, working bay right; ceramic gap insert. | Layered connected set feasible; 3D useful for wheel reach/shoulder, cable and mug contact. | One Lea, installed/loose mug variants, overlapping gust/work readable. |
| `cam.threshold` | Accepted room departure, same door both sides. | Occluding jamb can mask 2D set swap; 3D useful for partnered carry. | People clear before shutter; no death/timing rule from transition animation. |
| `cam.ridge_two_targets`, `cam.ridge_targets` | South choice wide/medium, Lea west-left, Jonas east-right. | Fore/mid/back terrain layers can establish; 3D useful for branching carry/line contacts. | Two targets remain separable under snow; no easy shortcut between them. |
| `cam.shelter_loss`, `cam.coastal_track` | Branch-specific target absence/fallback from established axis. | Layered silhouette/occlusion sufficient for short beat. | Exact named loss; explicit flare action only when source consumes/fires it. |
| `cam.manifest_four`, `cam.manifest_empty` | Same four-position transport composition. | Layered cabin/cast may suffice; 3D useful for carried seating. | Match opening count versus exact empty place, injury retained. |
| `cam.tag_insert`, `cam.empty_handset` | Casualty-specific coda. | 2D insert enough. | Jonas tag versus Lea's final bearing, never swap grief props. |
| `cam.report_insert`, `cam.station_lost` | Conditional daylight evidence / same distant station and spring names. | Mostly static layers/dissolve sufficient. | Log gate; same exterior silhouette; no alive cast in lost ending. |

Prototype `shot.wide` maps to generator-establish intent, `shot.controls` to
external-fuse/control detail, `shot.starter` to primer detail, `shot.hall` to
hall-return intent. They remain distinct **existing concrete binding IDs**.
Their current transforms/annex set are not a built full hall or these final shots.
No additional camera protocol, ACK or generic shot-dispatch system is introduced.

## Decision / timer composition contract [A]

Current prototype is 1000×700 with a **variable-height bottom panel**, sometimes
near half the image for radio + choices. Don't assume a fixed 25% safe band.
For a conservative planning frame, reserve the lower half for the longest
observed panel and place essential hands/targets/route indicators in the upper
half with margin. Final game may reframe or revise panel layout deliberately;
this document does not implement UI changes or claim the test is already done.

| Situation | Essential in planned world crop | Proposed normalized target region [A] |
| --- | --- | --- |
| Hall choice | Door axes, supported Jonas, route/line clue. | x .08–.92, y .07–.44; injury gets a clear insert if full-body scale too small. |
| Generator radio over work | Hinge/hand or fuse, active coil clue, readable caption/countdown. | Device/hand x .35–.82, y .10–.42; UI owns actual countdown, not another physical timer. |
| Radio gust/manual choice | Wheel hands and operator silhouette plus transmitter relation. | Two stations x .10–.90, y .08–.43; avoid window glare over gloves. |
| Ridge decision | Distinct Lea-left/Jonas-right and relevant line/carry clue. | Both targets x .10–.90, y .08–.43; weather can't erase them before authored outcome. |

These are **acceptance targets**, not measured rendered results. The full-frame
concept art has extra floor/architecture and must be reframed for gameplay UI;
it is not already a shipped camera screenshot. Test reference silhouettes around
64px body height, greyscale and longest captions with actual assets before
claiming readability. The Character Bible's casting/pose limits still apply.

## Timing / continuity motifs

Latching media may overlap generator T=10000 warning/reveal; it must not stop or
restart because a caption changes. Radio T=15000 gust overlays ongoing work and
checks actual `radio_auto`. Startup has locked sequential 1200/1500/1800ms beats;
no camera movement or length declares power. A late truncated action cannot use
a successful engine/signal shot. Clock starts/removal/extension remain canonical.

Opening four faces match full manifest four answers; partial repeats the frame
with exact absence; lost reduces the count to four names without faces.
Door-jamb transitions repeat the geography without suggesting free re-entry or
teleporting an injured person. Snow/metal/radio motifs connect scenes but do not
add a new mystery, safe route or universal cinematography framework.
