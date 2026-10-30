# M2 acceptance example — The Locked Room Cue Sheets and Game Actions

This is a **normative behavioral example for the M2 implementation agent**, **not a prescribed parser grammar, Rust representation or exact wire syntax**. Preserve its authored choice/time/state semantics. The implementer designs a small story DSL that can express this complete scenario and emits equivalent renderer-facing Cue Sheets and correlated Game Actions.

## Why this is a mini-game, not a chain of micro-nodes

The drawer/keyring/door challenge is a complete **authored stateful mini-game**, expressed with the **same story DSL** as the surrounding narrative. One story node or narrative context may host all intermediate actions (search, take, wrong keys, repeated door forcing, squeeze through) and multiple resulting Cue Sheets. A separate graph node is **not needed** for every increment of `door_damage`, new action, failed attempt or presentation cue. Story nodes chiefly structure the authored adventure; explicit transitions are for narrative structure, and are invisible to the renderer. The DSL and its normal choice/state/timer rules handle the mini-game without a separate subsystem.

## Boundary and operating rules

- The story graph, current narrative context and node changes exist inside GRIMM. **Cue Sheets do not expose graph node IDs, node transitions or ending markers**. They tell the renderer what to play, which actions to accept and the currently relevant elapsed/total timer. **GRIMM's canonical `world_state` is not a mandatory Cue Sheet field.**
- The renderer first executes all `play` cues (in order), updates its own presentation state as needed, then displays/enables `actions` and **starts/resumes its timer at the supplied Cue Sheet baseline**. An authored action's animation duration is *already included* in GRIMM's post-action `scene_timer` value and **must not be counted twice**.
- The renderer reports `game_action { ref, action, scene_timer }` when the user chooses an offered action. The timer value is the **elapsed time at selection**, before GRIMM adds that action's authored cost. `action: timeout` is the ordinary reserved input for expiry without a player choice. GRIMM validates the referenced current offer and resolves all effects, conditions, time and presentation in one transaction.
- Timers do not automatically reset between Cue Sheets. Explicit authored time changes may increase the total budget, or start a new timer (e.g. `[0 of 12]s`). `time_passed` in a rule reads the renderer-reported elapsed value from the current `game_action`.
- The demonstration uses simple **GRIMM-owned** authored state (no built-in door mechanics): `drawer_open`, `has_keychain`, `door_damage`, `door_open`, `character_strength`, `character_dex`. Authors choose which are persistent across narrative contexts and which are local. The comments alongside the Cue Sheets annotate GRIMM's internal state for testability; these values do **not** belong to the M2 renderer-facing Cue Sheet.
- The story must include a start, at least one authored ending, exhaustive deterministic branches/ELSE for executable actions, and explicit node transitions when authored; no implied free navigation/backtracking.

**Canonical-state assertions accompanying the traces (not Cue Sheet fields):** after `open_drawer`, GRIMM has `drawer_open=true`; after `take_keychain`, `has_keychain=true`; after a wrong key `door_open=false`; a correct key or successful `break_door` sets `door_open=true`. In the alternative run, `door_damage` changes from 0 → 1 → 2 → 3 → 4 on successive `break_door` actions with `character_strength=1`. The renderer may mirror visual consequences internally but never decides them.

## 1. Original drawer and keys trace (30-second scene budget)

For this first trace, each drawer/keychain action costs **1 second** and each key attempt costs **2 seconds**. Values in `cue_sheet.scene_timer` are **post-action**. The example uses a single continuing 30-second situation budget.

```text
cue_sheet {
  id: 1
  play: [locked_in]
  actions: [open_drawer, open_wardrobe, carefully_look_around]

  scene_timer: [0 of 30]s
}

# Renderer plays locked_in, then offers actions and starts at 0/30.
# Player considers for 2s.
game_action { ref: 1, action: open_drawer, scene_timer: [2 of 30]s }

# GRIMM commits drawer_open=true, adds authored 1s duration.
cue_sheet {
  id: 2
  play: [open_drawer]
  actions: [close_drawer, take_keychain_from_drawer,
            open_wardrobe, carefully_look_around]

  scene_timer: [3 of 30]s
}

# Renderer plays 1s open_drawer animation; 3/30 already includes it.
# Player decides after 2s of renewed countdown.
game_action { ref: 2, action: take_keychain_from_drawer,
              scene_timer: [5 of 30]s }

# GRIMM adds authored 1s and stores has_keychain=true.
cue_sheet {
  id: 3
  play: [take_keychain]
  actions: [close_drawer, open_wardrobe, unlock_door_key1,
            unlock_door_key2, carefully_look_around, break_door]

  scene_timer: [6 of 30]s
}

# Wrong key; player hesitates 2s, then 2s attempt duration.
game_action { ref: 3, action: unlock_door_key1,
              scene_timer: [8 of 30]s }
cue_sheet {
  id: 4
  play: [wrong_key_does_not_turn]
  actions: [close_drawer, open_wardrobe, unlock_door_key1,
            unlock_door_key2, carefully_look_around, break_door]

  scene_timer: [10 of 30]s
}

# Correct key; player hesitates 2s, then 2s attempt duration.
game_action { ref: 4, action: unlock_door_key2,
              scene_timer: [12 of 30]s }
cue_sheet {
  id: 5
  play: [unlock_door_success, escape_room]
  actions: []

  scene_timer: [14 of 30]s
}
```

For the successful key branch, the 30-second budget has **14 seconds elapsed and 16 seconds remaining**. No special `node_changed` or `ending` property is necessary in the Cue Sheet. The separate examples below start from an **alternative path / fresh game state**, not after the successful key escape above.

## 2. Authoring intent — repeated `break_door`, strength and partial alternatives

Illustrative **DSL semantics**, not mandated surface syntax:

```text
STATE (game-authored examples)
  door_damage: integer = 0      # local or persistent, author decides
  door_open: boolean = false
  character_strength: integer = 1
  character_dex: integer = 3

OFFER break_door WHEN door_open == false
OFFER squeeze_through WHEN door_damage > 0 AND door_open == false

ON break_door:
    COST 2s
    ADD door_damage character_strength

    IF door_damage >= 4:
        SET door_open true
        PLAY break_door_success
        GOTO corridor
    ELSE IF door_damage == 3:
        PLAY break_door_almost_open
        # remain in current narrative context; offer break_door again
    ELSE IF door_damage == 2:
        PLAY break_door_progress
    ELSE IF door_damage == 1:
        PLAY break_door_barely_moves
    ELSE:
        PLAY break_door_nothing_happens

ON squeeze_through:
    COST 2s
    IF character_dex > door_damage:
        PLAY squeeze_through_success
        GOTO corridor
    ELSE:
        PLAY squeeze_through_does_not_fit

ON timeout:
    PLAY caught_while_escaping
    GOTO caught_ending
```

The `character_dex > door_damage` condition is intentionally **the user's illustrative example**, not a hardcoded or universally realistic physics model. An author can write a different one. The `door_damage` increments are **committed world-state changes across separate choices**, not renderer-reported progress within an animation. An author may select variants of `break_door_*` based on `character_strength`, `door_damage`, or both; GRIMM chooses the cue, and the renderer binds cue IDs to game-specific media.

The time-budget and incomplete-action branches remain authored. If the current remaining time is less than the chosen action's declared 2s duration, GRIMM must use the selected action's known effective interval and resolve the applicable authored timeout/failure branch **before issuing the next Cue Sheet**. The renderer does not decide success. Specific tie comparisons, partial effects and cue selection are game-authored.

## 3. Concrete Game Action → Cue Sheet trace for door damage

This is a **separate run**, with `door_damage=0`, `character_strength=1`, `character_dex=3`, and 30 seconds total. For concise demonstration, no time elapses thinking between consecutive door attempts; each attempt is 2s and the renderer plays its animation first before the next action is selectable.

```text
cue_sheet {
  id: 20
  play: [door_locked]
  actions: [break_door]

  scene_timer: [0 of 30]s
}

game_action { ref: 20, action: break_door, scene_timer: [0 of 30]s }
# door_damage = 0 + strength(1) = 1; +2s
cue_sheet {
  id: 21
  play: [break_door_barely_moves]
  actions: [break_door, squeeze_through]

  scene_timer: [2 of 30]s
}

game_action { ref: 21, action: break_door, scene_timer: [2 of 30]s }
# door_damage = 1 + strength(1) = 2; +2s
cue_sheet {
  id: 22
  play: [break_door_progress]
  actions: [break_door, squeeze_through]

  scene_timer: [4 of 30]s
}

# Alternative A: try squeezing now, because door_damage > 0.
game_action { ref: 22, action: squeeze_through, scene_timer: [4 of 30]s }
# character_dex(3) > door_damage(2) -> success; +2s
cue_sheet {
  id: 23
  play: [squeeze_through_success]
  actions: []

  scene_timer: [6 of 30]s
}

# Alternative B: instead of squeeze_through, keep forcing from cue_sheet 22.
game_action { ref: 22, action: break_door, scene_timer: [4 of 30]s }
# door_damage = 3; +2s
cue_sheet {
  id: 24
  play: [break_door_almost_open]
  actions: [break_door, squeeze_through]

  scene_timer: [6 of 30]s
}

# At damage 3 the author's dex(3) > damage(3) condition is false.
# This failure branch is optional; shown to exercise authored IF/ELSE.
game_action { ref: 24, action: squeeze_through, scene_timer: [6 of 30]s }
cue_sheet {
  id: 25
  play: [squeeze_through_does_not_fit]
  actions: [break_door, squeeze_through]

  scene_timer: [8 of 30]s
}

# From id 24 instead (without the failed squeeze), the fourth door attempt works:
game_action { ref: 24, action: break_door, scene_timer: [6 of 30]s }
# door_damage = 4; door_open = true; +2s
cue_sheet {
  id: 26
  play: [break_door_success]
  actions: []

  scene_timer: [8 of 30]s
}
```

**Branching note:** `game_action ref:22` has **two alternative possible inputs**, and `game_action ref:24` likewise shows *alternative branches*, not two events to execute in one timeline. Thus Cue Sheet IDs 23 and 24, or 25 and 26, must never be treated as concurrent outputs. Any distinct issued Cue Sheet ID is only an illustration; runtime IDs/revision handling are for the implementation agent to choose.

### Strength variants (same game-authored rule, no separate engine verb)

- `character_strength=4`: door_damage reaches 4 and succeeds on first `break_door`.
- `character_strength=2`: damage 2 → 4, so success on second `break_door`.
- `character_strength=1`: damage 1 → 2 → 3 → 4, so success on fourth `break_door`.

These numeric examples differ from an earlier informal “1/2/3 attempts” illustration; the actual threshold is governed by the **explicit author-defined formula**. An author wanting 1/2/3 attempts may set different thresholds or stats. **This trace is the concrete acceptance example for the formula above.**

## 4. Timer timeout, fresh timer and time bonus

```text
# No player selection, after cues finish and current timer reaches full budget:
game_action { ref: 22, action: timeout, scene_timer: [30 of 30]s }
cue_sheet {
  id: 27
  play: [caught_while_escaping]
  actions: []

  scene_timer: [30 of 30]s
}

# A new authored timer may instead appear as:
cue_sheet { id: 30; play: [next_challenge]; actions: [A, B];


# A two-second time-buying action can increase a 30s budget to 35s:
game_action { ref: 31, action: buy_time, scene_timer: [12 of 30]s }
cue_sheet { id: 32; play: [blocked_mechanism]; actions: [A, B];

```

## 5. Required DSL / runtime acceptance behavior

- Fully author the story, actions, conditions, variable declarations, time costs and cue IDs in the **new small M2 DSL**; do not hardcode this door scenario in Rust.
- Check the **exact** `door_damage` update across repeated action calls, fresh conditional availability of `squeeze_through`, both `character_dex > door_damage` outcomes, animation selection at each progress threshold, `door_open` only on successful forcing, and preservation of the 30s timer across local interactions.
- The M2 renderer receives **cue IDs, actions and optional time**, not GRIMM's canonical state or graph. It may interpret cues to maintain graphical presentation state. State-changing gameplay effects remain inside GRIMM, and a later graphical game may persist its renderer's presentation state alongside GRIMM's canonical save.
- Verify elapsed-at-selection versus post-action elapsed, that required cue animation plays **before** timer resumes, and that animation time is **never charged twice**.
- Check one explicit `timeout` action, an authored timer reset, one time-buying bonus, and authored condition/ELSE validation. The Rust implementation agent may determine the concrete parser, representation, validations, serialization, revisions and integration points.
