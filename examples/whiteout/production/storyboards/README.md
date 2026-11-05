# WHITEOUT — visual shot-by-shot storyboard (#80)

**Working draft; final coverage/continuity review in progress.** Based on main
`5aaeaf8`, screenplay PR #77, cast PR #78, environments PR #79 and Bevy PR #76.
Both phases belong to this task: generator pilot **and the remaining story**.
The issue is not completed by the pilot.

## Browse actual pictures

1. Phase A reference: [generator pages](scenes/generator.md).
2. Story order: [blackout](scenes/blackout.md), [stores](scenes/stores.md),
   [dispatch](scenes/dispatch.md), generator in either repair order,
   [radio](scenes/radio.md), [ridge](scenes/crisis.md),
   [full ending](scenes/ending_full.md), [partial endings](scenes/ending_partial.md),
   [lost ending](scenes/ending_lost.md).
3. [Complete shot index](SHOTS.md). Each shot has a real PNG and editable SVG.
   Per-scene pages contain contact sheets, full panels and readable metadata.

The frame includes a 1000×700 viewport (current Bevy aspect), a camera/blocking
inset and editorial notes on a 1600×900 / 16:9 storyboard page. Rust arrows mark
motion/camera; fine arrows mark gaze. Grey winter clothing silhouettes preserve
Alex hood/mask, Mara short teal mass/bun, Jonas ochre injured/support pose and
Lea long blue-grey cap/headset. No new cast or unmasked Alex casting is created.

All visuals are [A] blocking/art, rules/times [C], final casting/contact/art
approval [O]. The drawings are not screenshots, finished character assets or
runtime camera exports. UI is an authoring mock-up at actual viewport dimensions,
not a claim of testing the Bevy application. Metadata is editorial, not a new DSL.

`sources/shots.json` holds stable editorial IDs/metadata; `sources/draw_boards.py`
is a WHITEOUT-only drawing/export source, not an IDE, game engine or screenshot
pipeline. The editable vector originals are also checked in. Re-export with:

```bash
python3 examples/whiteout/production/storyboards/sources/draw_boards.py
```

Requires Python and `rsvg-convert` only for this optional art export; no Cargo or
runtime dependencies are added. Generated pencil artist studies for G01/R01 use
the existing own cast/environment references, exact prompts checked in. They are
alternate development studies, not additional coverage or replacement worlds.

Pending before readiness: finish explicit branching/coverage/continuity records,
inspect every scene sheet and run full native checks. Canonical executable
story/text/scopes, Rust code and original reference sheets remain unchanged.
No final animations/models, gameplay, generic authoring framework, save/restore,
codegen, screenshot automation or full Bevy port is delivered here. No auto-merge.
