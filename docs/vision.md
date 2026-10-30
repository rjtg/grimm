# Vision

GRIMM (Generate Real IMMersion) is a Rust-based, headless narrative game engine. It is a framework, not a single game design or an AI-first application.

## Supported experiences
- Classic authored choose-your-own-adventure: fixed nodes, fixed choices, deterministic traversal.
- Authored narrative adventures combining Telltale-style consequential, cinematic, optionally timed decisions with LucasArts-style exploration, object puzzles and multiple solutions.
- Optional RPG-like character attributes, skills and relationships driving conditional choices using the same canonical state model.
- Later open-world/Story-RPG extensions: optional free input, human-DM or generated narrative additions, autonomous NPCs and story milestones.
- Concrete game compositions from CLI text through 2D graphic adventures to optional Bevy-based 3D. A game definition explicitly selects the renderer implementation/source and provides its own assets, authored bindings and other required modules; a renderer's abstract capabilities do not make it compatible with arbitrary games.

## Invariants
- A static story must remain playable offline without LLMs or graphical rendering.
- Engine truth is separate from presentation interpretation. A game is a concrete assembly of engine, authored gameplay, game-selected renderer, game-specific assets and potentially other components; renderer-specific code/resources remain outside the reusable headless engine.
- Every capability beyond basic graph traversal is opt-in where practical.
- Choices are the shared gameplay mechanism: exploring, taking, combining, checking stats and progressing a narrative are authored conditional offers, not a mandatory fixed verb list.
- AI may later help interpret actions, narrate, or propose events, but engine rules commit consequences.
- Story authors retain control over authored content and constraints.

## Non-goals for first release
Visual editor, multiplayer, 3D, distributed simulation, complex plugin marketplace, AI-generated stories, NPC autonomy.
