# Vision

GRIMM (Generate Real IMMersion) is a Rust-based, headless narrative game engine. It is a framework, not a single game design or an AI-first application.

## Supported experiences
- Classic authored choose-your-own-adventure: fixed nodes, fixed choices, deterministic traversal.
- Hybrid narratives: authored structure with optional free-form actions and dynamic reactions.
- More emergent narratives: optional dynamic events/nodes, autonomous NPCs and story milestones.
- Presentation from CLI text to web/comic and eventually Bevy-based 3D.

## Invariants
- A static story must remain playable offline without LLMs or graphical rendering.
- Engine truth is separate from presentation interpretation.
- Every capability beyond basic graph traversal is opt-in where practical.
- AI may help interpret actions, narrate, or propose events, but engine rules commit consequences.
- Story authors retain control over authored content and constraints.

## Non-goals for first release
Visual editor, multiplayer, 3D, distributed simulation, complex plugin marketplace, AI-generated stories, NPC autonomy.
