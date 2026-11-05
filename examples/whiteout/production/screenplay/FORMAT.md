# Editorial format, revision 1

This is a human-editable interactive screenplay, not executable YAML, a new
screenplay DSL, a compiler input or an extension of the GRIMM protocol. English
matches the existing game's language. No runtime consumes these documents.

## Reading a scene

Each chapter begins with the **canonical context ID**, INT./EXT., stable location
and actor identities, dramatic objective, entry state and clock contract. Short
numbered **beats** provide internal editorial anchors; they are not new graph
nodes. A repeated location or actor retains its identity across visits.

Each beat separates:

- **Story [C]**: the canonical fact, clue, action result or exact quoted dialogue.
  The associated narrative/action ID identifies the source. Quoted text marked
  [C] is from `text.yaml`; prose summaries need not reproduce its wording.
- **Performance [A]**: adapted dialogue (speaker in capitals), blocking, visible
  information, camera, sound and light. These are production proposals. They may
  realize a canonical fact but cannot create equipment, knowledge, success or a
  survivor independently of the rules. Added dialogue is never presented as an
  existing quotation. Nothing in [A] is an approved character biography.
- **Cue reference [R]**: existing context/cue/narrative/action association and
  proposed conceptual cueables. References describe correspondence, not new
  runtime instructions. `CUE context / cue_id` is an existing cue; `TEXT id` is
  reusable content, not instance identity. For action outcomes use
  `ACTION context.action / outcome N / cue_id`; N is the authored ordered branch,
  starting at 1. Complete paths and predicates are in [RULES.md](RULES.md).

Scene choice tables show every authored action, work duration and branch
meaning. Exact guards, assignments and **first-matching outcome order** live in
RULES.md; shorthand in a filmic beat cannot override them. `true` means always
eligible after the authored input gate, not that the outcome always succeeds.
The parity matrix maps all text IDs and all 95 cue instances to the chapters.

## Timing notation

- **T**: current canonical context's scene time, milliseconds. New entry means a
  new clock; dispatch return is another visit, not another physical hall.
- **S**: time an action is selected. Its work duration is a logical cost; do not
  advance host time by that cost and also wait it again. Outcome presentation
  begins at selection after resolution; input waits for the existing release
  floor and, for sequential cues, preceding completion.
- **beat + offset**: relative within a presentation, never a new gameplay clock.
  Proposed subdivisions fit inside the existing duration. Dialogue may require
  subtitles or compression; a longer performance requires a separate timing
  review, not silently extending a deadline.
- An untimed sequential narrative follows its predecessor. An explicit `at_ms`
  is absolute T and may overlap it. Deadline start is at entry, not after speech.
  Pause freezes relevant scene time; do not add cinematic pauses to buy time.
- A late work action is truncated by canonical rules. Completion exactly at the
  deadline may succeed and cancel it (generator/radio); an idle selection at the
  deadline loses to timeout in the host. These are different boundary cases.
- Transition staging cannot sneak in logical travel cost: canonical normal GOTOs
  have duration 0. Outgoing camera/audio bridges are editorial proposals to map
  to ordinary transition nodes in a future graph adaptation, with origin/target
  IDs recorded. No such nodes are added in this authoring PR.

## Conceptual cue vocabulary

These labels are production categories, **not new GRIMM instruction variants**:
`camera.closeup`, `camera.track`, `camera.hold`, `actor.move`, `actor.gesture`,
`actor.expression`, `dialogue.play`, `environment.light`, `environment.weather`,
`audio.ambient`, `audio.sfx`, `music.texture`, `prop.animate`, `ui.offer`.
An inventory entry owns a stable production ID; a cue proposes an operation on
it. Scene-local beat IDs distinguish repeated uses of a reusable shot or line.

Where concrete bindings exist, use them: `shot.wide`, `shot.controls`,
`shot.starter`, `shot.hall`, `move.jonas`, `move.mara`, `move.deicer`,
`move.starter`, `move.shutter`, `motor.spin`, `sound.wind`, `sound.motor`,
`sound.alarm`. They belong to the simplified Bevy excerpt, not automatically to
the full canonical story. Other categories need future game-specific bindings.

## Editorial continuity and review

No camera cut discovers a clue before its authored action. Item silhouettes can
show affordances, but possession follows effects. Show a failed attempt as failed
immediately; a pretty animation is never proof of a rule result. Do not display
raw globals or private state in player-facing production UI.

Maintain source IDs when revising prose. Additions that would change timing,
conditions, route access, casualty or cause of the disaster are **rule-change
proposals**, outside this document's authority. Record disagreements explicitly.
No new cast/locations may appear without an inventory entry. The old blackout
sample remains an editorial source, not a second competing canonical scene.
