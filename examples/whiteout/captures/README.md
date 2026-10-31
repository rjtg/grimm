# M4.5 terminal captures

`play.txt` (100×40) and `inspector.txt` (150×40) were exported with F4 from an
actual interactive WHITEOUT session in the radio cabin. The same widget renderer
produces both layouts from one live session snapshot. The live timer shows its
remaining budget while choices stay below narrative; the inspector adds the
full issued sheet, execution lifecycle and current read-only world rows. Scroll
or change focus to inspect rows outside the visible viewport.

`ending-full.txt` was captured at the full-crew ending during the complete
interactive playthrough. It predates the final wording polish that labels an
absent deadline “no deadline” and a finished ending “No further actions”.

F4 writes local captures to `.grimm-captures/whiteout/`. These text exports retain
readable cue IDs and lifecycle symbols; the live terminal additionally uses the
same supplemental color for corresponding narrative/inspector cue entries.
Capturing does not acknowledge cues, advance scene time or mutate world state.
