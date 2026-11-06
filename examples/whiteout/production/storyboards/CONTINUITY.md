# Continuity review and remaining art decisions

Review baseline: main `5aaeaf8`, screenplay #77, cast #78, environments #79.
[C] canonical rules, [A] proposed visual blocking, [O] production approval.
This is a drawn-board review, not a rendered Bevy playtest or metric rig check.

## Reviewed scene sheets

All 21 exported contact sheets were visually inspected, including the full-size
panels behind them where UI or timing was ambiguous. The PNG/SVG presence audit
is separate from this visual assessment.

| Sequence / sheets | Checked continuity | Corrections made during review |
| --- | --- | --- |
| Blackout 01, B01–B05 | Four recurring silhouettes, same corridor; injured Jonas, masked Alex, torch and handset carried into stores | Added explicit carry start/end, no fitted splint before treatment |
| Stores 01–03, S01–S14 | Bench left, one tray/braid/mug/spanner, rack right, shared exit; nine initial choices; treated/untreated exits | Only Jonas receives splint; prepared leave empties picked-up equipment, log remains |
| Dispatch 01–02, D01–D11 | Radio west/left, engineering east/right, ridge ahead; bench/duct unchanged on returns | Removed duplicated Jonas in reaction close-up; added resolved/lost/order-specific UI; warming does not replay entry |
| Generator 01–05, G01–G29 | Engine north, working strip centre, Mara/Jonas left, external fuse right, same hinge and threshold; cover progression | Tightened partner reaction; removed engine-mounted coil claim; unsafe start remains offered; warning keeps input locked while latch media overlaps |
| Radio 01–04, R01–R21 | Desk left, connected wheel bay right, one Lea; manual hands held vs automatic release | Added adapter start/end, cold slip, powered/unpowered/consumed-braid UI and call-acknowledged deadline removal |
| Ridge 01–03, C01–C15 | Lea west/left, Jonas east/right, common markers and snowfield; specific person lost | Added actual carry pose, untreated full rescue, auto route without invented rope, erased-crossing end frame |
| Full ending 01, E01–E02 | Four occupied places, same gear/injury; truth conditional on log | No healed ankle or fifth cast member |
| Partial ending 01, E03–E07 | Correct named empty place, Jonas tag versus Lea handset; truth does not undo loss | Separate casualty compositions rather than generic empty seat |
| Lost ending 01, E10/E08/E09 | Same mast/exhaust silhouette, pullback start/end, four names at same ridge door in spring | Names are on metal beside door, not a report; no living crew after lost outcome |

## Axis, eyelines, motion and camera identity

Every board includes a schematic camera plan with direction, named cast/prop
positions and a framing label. Insets are orientation diagrams, not surveyed
floor plans. Main views use the established south half-plane; inserts preserve
hinge/hand direction. Fine arrows show gaze; rust arrows show actor/camera motion.
The reaction close-ups remain on that side of the axis. G01→G04 pushes toward
the already established right-hand controls. No reversal is implied by editing
to the insert. The three establishing camera names are [A] proposals extending
the existing camera list, not protocol changes. Existing camera/shot aliases are
retained in the index; crops are editorial framings, not new story nodes.

Complex movements have actual beginning/end drawings: B05→S01, G01→G04,
G23→G24, R03→R04, C04→C05, C12→C13 and E10→E08. E08→E09 is a seasonal
match cut, not an impossible continuous move through a wall.

## Timing, branches and UI

Source rules win over the narrower Bevy generator demonstration. In particular,
starting unisolated is an offered failure, and effects resolve once at selection,
not at the end of an animation. G16 explicitly draws concurrent hand media and
radio warning with the original countdown at 20 seconds, input locked until at
least T=11 for the T=8 / 3-second work example. See BRANCHES for deadlines,
truncation, outcome precedence and conditional returns.

The board is 1600×900. Its illustrated player viewport is 1000×700, matching the
current application aspect. UI profiles reserve 350 px vertically and reduce
the world composition above it; nine stores options fit without hiding the
clock. Full panels and Markdown captions provide readable notes; six-up contact
sheets are browsing thumbnails, not a small-print reading substitute. Real
localization, font scaling, input hit targets and minimum-window-size behavior
remain host implementation checks. No runtime usability certification is claimed.

## Open production decisions [O]

- Art director approval of the simplified vector blocking and two pencil studies.
  These are consistent production proposals, not final likeness/rig/lighting.
- Jonas injury side is proposed left; injury itself and persistent limited
  mobility are canonical. Final contact poses, carry supports and collision-safe
  blocking need rig validation. Treatment must not become healing.
- Alex remains hooded/masked with casting open. The reduced heads are approximately
  7.5-head figures; facial reference fidelity awaits actual character assets.
- Camera heights, focal lengths, set measurements and traversable strips need
  validation against the eventual host scene. Insets intentionally do not claim
  metric precision. Final timer/UI styling remains open.
- UI panels are representative conditional profiles. Arbitrary inventory, cold
  and millisecond permutations reuse framing with current data, not an initial
  screenshot. This is not exhaustive state-space verification.
- Eight dormant numb-latch cues remain documented in COVERAGE; resolving their
  contradictory guards would change gameplay and is outside this art task.

No missing indexed drawings remain. These open approvals do not stand in for
missing picture coverage; they describe the boundary between storyboard and
final production assets.
