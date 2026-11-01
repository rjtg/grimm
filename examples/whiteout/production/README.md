# WHITEOUT screenplay: first production fragment

This is M5a.1, not the complete screenplay or a graphical game. The M4.6
lifecycle/provider/catalog gates passed before this offline increment. No story
rules, text bindings, action IDs, deadlines or outcomes have changed.

```bash
cargo run -p grimm-cli -- check-screenplay examples/whiteout/story.yaml examples/whiteout/production/blackout.screenplay.yaml examples/whiteout/text.yaml examples/whiteout/scenes.yaml
```

## Contract (schema 1)

`Screenplay` names the canonical story/version and a list of production scenes.
Each scene has a stable `scene_id` matching the explicit M4.6 context-to-scene
configuration, a `context_id` checked through the M3 index, slugline, beats and
`decision_actions`. Each beat names its own editorial `beat_id`, an authored
`cue_id`, reusable `narrative_id`, ordered content and descriptive staging.

Content is tagged `narration {text}` or `dialogue {speaker, text}`. The first
fragment preserves the opening as narration; it does not invent spoken lines.
Joining content text with a blank line must exactly reproduce the current text
binding. This conservative traceability gate makes accidental fact edits visible.
It is not a requirement that the final graphical adaptation retain every prose
sentence; an explicit editorial migration will be needed for that later change.

Staging kinds are shot, blocking, movement, camera, transition, pause, environment
and audio, with a description and optional `duration_ms`. Duration is an editorial
target, **never** an override of authored gameplay cost, cue timing or deadline.
These are production notes, not GRIMM commands or executable tracks. No source
image, physical resource path, model/rig format or image provider is required.
The selected renderer will later author actual cueable sequences from these notes
and export its real asset catalog.

Validation rejects unsupported versions/fields/kinds, invalid or duplicate
production IDs, duplicate context/cue/action mappings, mismatched scene scopes,
unknown context/cue/narrative/action references, missing/duplicate text bindings,
empty content/directions and duration overflow. It validates an explicitly partial
fragment, not remaining graph coverage, actor-bible references or rendered timing.
The blackout regression separately proves this one included context's narrative
and action coverage, plus serialization round-trip and actual CLI execution.

## Conversion strategy and next gate

1. Keep the canonical story/text and stable IDs as the baseline. Extract a scene's
   actual references with the M3 index; name the same M4.6 presentation scope.
2. Split existing narrative into beats/content without changing facts or branching.
   Write staging separately, marking timing estimates and action decision points.
3. Validate one converted scene and review the readability/staging contract before
   a larger rewrite. The blackout scene is that first sample.
4. Next convert the generator warning/startup sequence as the second sample. Cover
   conditional outcome cues, the T=10 radio overlap, deadline reveal/removal,
   completion barriers and both successful and failed repair feedback. Decide
   conditional beat representation and actor identity references from that work.
5. Only after sample review extend coverage to all routes/endings. M5b defines
   visual identities; M5c compares the same 30–60s staged scene in sprite/2D and
   Bevy with actual assets/preload/input/timers before selecting a renderer.

Open: conditional-beat grouping, future actor-spec references, staging estimate
semantics and editorial text-to-dialogue migration. This increment does not
introduce a screenplay executor, full authoring studio or a renderer selection.
