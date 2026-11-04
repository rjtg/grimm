# Host-driven graph foundation

```bash
cargo run -p grimm-graph --example locked_in --locked
cargo test -p grimm-graph --locked
```

`locked-in.yaml` declares `LOCKED_IN_SCENE → BREAK_OUT_SCENE → CORRIDOR_SCENE`.
The middle scene stages presentation, a host effect at 1200ms and a hidden
`continue` traversal cue at 1500ms. The host supplies time and executes the
operation; graph traversal activates each target immediately and returns its
full replacement sheet. Conditions are descriptors, not evaluated by the core.
No world provider, optional rules library, timer loop or cue ACK is required.

The example is a deterministic headless contract proof, not a new graphical game
or a WHITEOUT migration. The [migration mapping and API details](../../docs/migrations/story-graph-03.md)
explain how existing rules/hosts/saves can migrate without deleting legacy code.
