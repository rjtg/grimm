# WHITEOUT

Aurora's lights have gone out. Outside the polar research station, snow has erased
the ridge. Inside, four people have two broken systems and a dwindling chance to
call for help. You are Alex, the field engineer. Mara leads the station, Jonas
is injured, and Lea is trying to make a radio work without power.

Something is replaying an old distress call. The answer is in the station's
maintenance history, but finding out why the blackout happened will not, by
itself, get anyone home.

WHITEOUT is a complete survival-mystery text adventure, built with GRIMM's M2
scene DSL and checked with M3. Expect a short adventure with preparation,
interconnected repair puzzles, several routes and three distinct endings.
No graphics, account, network service or model is required to play.

## Play

From the repository root:

```sh
cargo run --locked -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml
```

- **Enter:** finish the displayed narrative. Briefings precede the countdown;
  after their completion, choices and the scene's timer become active.
- **Number key:** choose an available action immediately. Every game menu fits
  within nine choices. Work costs time; the next choice may wait for that cost
  and any required narrative to finish.
- **Esc:** pause and open the session menu. Its numbered buttons continue,
  save, load or quit. Reading work results against an active timer consumes
  time unless you pause.

Read the equipment clues. Not every problem needs brute force, and using an
object in one repair may take it away from another. The dispatch hall is the
best place to pause, save and plan. Its lack of a decision countdown does not
make an unheated hall warm. A thermal liner, restored heating and good triage
can change what is possible later. Failure is authored; the game does not demand
that you stumble onto an exact millisecond boundary to win.

Engineering's initial thirty-second coil budget is deliberately urgent. The
spare heat-pack battery can add a minute. Use it if you want time to read and
work carefully; the timer does not restart when its display wakes.

## Save and resume

In the dispatch hall, finish any work/result prose, press Esc, then **2** to save.
The existing CLI checkpoint is `.grimm-saves/scene-whiteout.json`. To resume:

```sh
cargo run --locked -p grimm-cli -- play-scene examples/whiteout/story.yaml examples/whiteout/text.yaml --load .grimm-saves/scene-whiteout.json
```

Offline time is frozen. The checkpoint preserves authored choices, semantic cue
history, world state and the current timer. Saves elsewhere use the existing
host behavior: unfinished prose can be shown again on load, without applying its
gameplay effects twice. Stable hall checkpoints avoid unfinished presentation.

## Authoring and tests

```sh
cargo run --locked -p grimm-cli -- check-scene examples/whiteout/story.yaml
cargo run --locked -p grimm-cli -- index-scene examples/whiteout/story.yaml > whiteout-index.json
cargo test --locked -p grimm-cli --test whiteout
```

`story.yaml` contains all game rules, conditions, clocks and effects. It contains
semantic narrative/action IDs rather than prose, option labels or asset paths.
`text.yaml` is this game's concrete text-renderer binding. A later M5 application
can load the **same story unchanged** and supply graphical bindings for those IDs.
The game reuses the existing GRIMM library, adapters and CLI; there is no puzzle
logic in the CLI and no new engine, crafting, inventory or NPC subsystem.

The [walkthrough](WALKTHROUGH.md) contains spoilers, exact ending routes and
reproducible CLI inputs. The [design and timing notes](DESIGN.md) describe state,
cold accounting, acceptance evidence and limits.
