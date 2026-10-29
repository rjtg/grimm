# M0 implementation notes

Historical M0 baseline. M1 replaces the stub with a reusable runtime, retains the
two-node demo, and migrates `SemanticOutput.text` to structured content. See
[M1](m1.md) for the current types, save format, and compatibility implications.

## Crates and dependency contract

| Crate | Responsibility | Normal dependencies |
|---|---|---|
| `grimm-semantic` | Action proposals, interaction offers, player-visible output | Serde |
| `grimm-presentation` | Text/choice commands and raw line input | Serde |
| `grimm-engine` | Private canonical stub state, validation and commit | Semantic protocol |
| `grimm-adapters` | Input/output traits and plain-text implementation | Both protocols |
| `grimm-cli-renderer` | Display commands, flush prompts, read raw input | Presentation protocol |
| `grimm-cli` | Composition root and executable loop | Engine, adapters, renderer, presentation protocol |

`crates/cli/tests/boundaries.rs` inspects Cargo metadata and enforces an explicit
dependency allowlist, including dev, build, optional, and target-specific edges.
It also rejects unreviewed workspace crates and redirected internal dependencies.
Serde JSON is used only in tests. Neither the adapter nor renderer can name or
mutate engine types through its dependencies. A compile-fail doctest also checks
canonical state privacy. Rust source uses workspace-wide `unsafe_code = "forbid"`.

The stub has one boolean state and one valid action (`continue`). Its initial
projection has revision 0; successful commitment produces revision 1 and no
offers. The engine rejects stale revisions, unknown choices, and attempts to act
after completion without changing state. There is no story graph or persistence.
The action result is the engine-produced projection; M0 does not add a separate
event journal, private NPC data, or event-processing framework.

The output adapter preserves text verbatim and remembers the current screen's
number-to-action mapping. Raw numbers never enter the engine. The renderer sees
labels and prose only, without semantic IDs, revisions, or canonical state.
Rendering, remapping offers, or changing an owned projection cannot change game
truth. Only `Engine::apply` commits the validated transition.

## Minimal protocol and compatibility

`GameAction` contains a revision and choice ID. `InteractionOffer` contains an ID
and label. `SemanticOutput` contains a revision, visible text, and current offers.
`PresentationCommand` is either text or numbered choice labels; `RawInput` is a
line. All derive Serde serialization and have round-trip tests.

These are the first provisional M0 public types, not a stable wire or save-file
schema. There are no existing persisted records to migrate. Future incompatible
changes must document their compatibility and migration implications before
being adopted. The boolean-derived revision is scoped to one stub engine
instance; cross-session identifiers are not a promise of this API. The CLI
composition root holds one engine and one adapter for the lifetime of the demo.
Empty offers end this demo; this is not a general lifecycle protocol for M1+.

Synchronous methods, owned projections, and concrete renderer I/O keep the loop
small. The only adapter contracts are `OutputAdapter` and `InputAdapter`. No
transport, plugin framework, async runtime, or future subsystem is scaffolded.

## Extension without changing engine dependencies

A future LLM text adapter implements `OutputAdapter` in a separate crate,
consumes only player-visible `SemanticOutput`, and emits the existing text
presentation protocol for the same CLI renderer. Provider dependencies stay in
that adapter. Exact authored text, validation and fallback remain M2 work.

A future Bevy frontend adds a visual presentation protocol, a semantic-to-visual
adapter, and a renderer that consumes those visual commands. The composition
root wires them together; the engine keeps its semantic-only dependency. Any
new public types and architectural changes require the documented review process.

## Acceptance coverage

| Requirement | Evidence |
|---|---|
| FND-01 | Six-crate edition-2024 workspace, lockfile, shared lints, build/run instructions |
| FND-02 | Cargo dependency policy; private-state compile-fail doctest; presentation-isolation integration test |
| FND-03 | Small Serde protocols with round-trip tests in each protocol crate |
| FND-04 | `executable_round_trip` launches the real binary; renderer and adapter unit tests; reproducible README command |
| FND-05 | Boundary policy covers all dependency kinds; CI runs formatting, Clippy, workspace tests and offline demo |
| FND-06 | Existing AGENTS.md and accepted ADRs preserved; crate map, protocol assumptions and extension notes documented here |

Additional tests verify deterministic transitions, invalid/stale action rejection,
invalid input retries, EOF handling, and renderer I/O failure propagation.

## Remaining open design questions

- Story schema and authored content model (M1).
- Stable protocol versioning and save migration policy (before persistence).
- Session/revision identity beyond the single demo instance.
- Event types, history and player-visible projection rules as real game state grows.
- Async transports, presentation acknowledgements and snapshot granularity when needed.
- Generated-text validation, exact-text preservation and deterministic fallback (M2).

No new dependency direction or architectural boundary is introduced beyond the
accepted ADRs, so M0 needs no additional ADR. These questions are deliberately
unimplemented rather than resolved through speculative interfaces.
