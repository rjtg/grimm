# Vision

GRIMM (Generate Real IMMersion) is a Rust-based, headless narrative game engine. It is a framework, not a single game design or an AI-first application.

## Supported experiences
- Classic authored choose-your-own-adventure: fixed nodes, fixed choices, deterministic traversal.
- Authored narrative adventures combining Telltale-style consequential, cinematic, optionally timed decisions with LucasArts-style exploration, object puzzles and multiple solutions.
- Optional RPG-like character attributes, skills and relationships driving conditional choices using the same canonical state model.
- Later open-world/Story-RPG extensions: optional free input, human-DM or generated narrative additions, autonomous NPCs and story milestones.
- Game-specific presentation targets, from CLI text to 2D graphic adventures and potentially Bevy-based 3D; each game may require a particular renderer capability profile. Renderer-independent engine rules do **not** imply that every game is playable with every renderer.

## Invariants
- A static story must remain playable offline without LLMs or graphical rendering.
- Engine truth is separate from presentation interpretation; game-authored presentation resources may be specific to one renderer family.
- Every capability beyond basic graph traversal is opt-in where practical.
- Choices are the shared gameplay mechanism: exploring, taking, combining, checking stats and progressing a narrative are authored conditional offers, not a mandatory fixed verb list.
- AI may later help interpret actions, narrate, or propose events, but engine rules commit consequences.
- Story authors retain control over authored content and constraints.

## Non-goals for first release
Visual editor, multiplayer, 3D, distributed simulation, complex plugin marketplace, AI-generated stories, NPC autonomy.
