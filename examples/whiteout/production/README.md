# WHITEOUT screenplay writing

Read the [blackout screenplay sample](blackout.screenplay.md). It is an alternative
screenplay-style text asset associated with the existing `blackout.arrival`
presentation, analogous to the prose in `text.yaml`. Its dialogue, action,
blocking, shots, lighting and sound notes help later visual production.

Write and review it as text. There is no screenplay schema, compiler, runtime,
`check-screenplay` command or automated coverage/text-equality gate. Prose may be
freely adapted into dialogue and visible action. Optional semantic-ID notes help
associate a passage with the existing presentation; they do not define another
story graph or execution layer. The sample is not wired into the CLI, which keeps
using its existing prose assets and lookup behavior.

Human editorial review should consider whether the scene communicates its facts,
characters, clues and decision clearly, preserves branch/outcome meaning and
avoids revealing information too early. Creative pacing and later production
needs can be discussed without a parser restricting the writing. The canonical
`story.yaml` rules, prose bindings, authored durations and outcomes are unchanged.

This increment covers one readable scene. Next, adapt the generator's warning
and startup sequence in another content increment, including alternative repair
feedback and its dramatic timing. Technical completeness checks against actual
renderer-exported assets and cue bindings remain a separate later production gate.
The existing M4.6 text-resource checks are unchanged.

A possible long-term progression from descriptions through screenplay and
previsualization to polished graphics is a creative workflow vision. It does not
require an asset-tier/version framework, interchangeable runtime layers, fallback
selection, a screenplay-specific validator or an authoring studio now.
