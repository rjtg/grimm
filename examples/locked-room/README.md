# The Locked Room

A small M2 game. All puzzle rules live in `story.yaml`; `text.yaml` contains
only text-renderer bindings. No engine code knows about drawers, keys or doors.

From the repository root:

```sh
cargo run --locked -p grimm-cli -- check-scene examples/locked-room/story.yaml
cargo run --locked -p grimm-cli -- play-scene examples/locked-room/story.yaml examples/locked-room/text.yaml
```

Narrative playback advances automatically in the structured terminal UI. Numbered
keys choose immediately; Esc pauses and opens the session menu. F2 toggles the
read-only inspector, Tab changes focus and arrows scroll. The scene clock and
hidden thirty-second deadline start on entry, including during the introduction.
Use `--manual-reading` to finish text explicitly with Enter; Esc pauses reading. The radio reveals the remaining budget at
ten seconds even if another narrative is still open. Save/load freezes offline
elapsed time and reconstructs the semantic plan, rather than animation playback.

Routes to try:

- Open the drawer, take the keychain, try the notched key, then the round key.
- Force the door four times at ordinary strength, or brace and force it once.
- Force it twice and squeeze through while dexterity exceeds door damage.
- Look around, take the fireplace bar, then lever the door open.
- Open the wardrobe and climb through its window. Bracing first sacrifices the
  dexterity needed for a safe landing.
- Wait for Johnny or choose to surrender. These are explicit unsuccessful endings.

Each action commits immediately using its authored duration and the deadline.
Finish reading its result and wait for its logical availability time before the
next choice. An attempt whose duration crosses the deadline is truncated and
resolves its authored failure immediately; it never waits for an engine tick.

For reproducible tests use `--manual-clock`. Piped `:time MS` sets absolute scene
elapsed time; `:advance MS` advances it. `@locked_room.open_drawer` selects an
exact currently offered action ID. These are host testing controls, not story
verbs or free-text interpretation. Piped lifecycle commands are `:save PATH`,
`:load PATH` and `:quit`; `--load PATH` also restores at startup.

`detective.yaml` is the independent untimed, order-sensitive DSL regression:
searching before examination preserves a suspect opportunity; examination first
leaves footprints instead. Explicit reentry retains globals and resets locals.
It is exercised by native tests and does not ship text-renderer bindings.

The deterministic regression suite is:

```sh
cargo test --locked -p grimm-cli --test scenes
```

Authoring observations: scoped booleans and integers express the puzzle without
an inventory subsystem. Ordered cases make strength and cutoff outcomes explicit.
Separate text bindings keep the same rules usable by another renderer. Repeated
result prose needs a new semantic occurrence ID, while a pending radio warning
must keep its identity. Those different lifetimes are handled by the runtime.
Writing YAML expressions is verbose; a future authoring tool could offer shorthand
and validate missing renderer bindings. Structural checking is deliberately not a
proof that every combination of world state has a winning route. Text completion
is a coarse media acknowledgement; a graphical renderer would supply its own.

Schema-1 checkpoints require the matching files under `legacy-v1/`. See the
[M4.5 migration guide](../../docs/migrations/m4-5-scenes.md).
