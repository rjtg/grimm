# Instructions for coding agents

Read `docs/vision.md`, `docs/architecture.md`, `docs/requirements.md`, `docs/roadmap.md`, and applicable ADRs before making changes.

1. Work only on the explicitly assigned milestone and requirements. Do not pre-build later features.
2. Preserve boundaries: core -> protocol types only; adapters depend on semantic protocol; renderers depend on presentation protocol, not engine internals. Composition roots may wire components together.
3. GRIMM owns narrative rules and validation. In the Bevy host, mapped ECS fields are authoritative world-state storage; GRIMM evaluates consistent provider snapshots and commits validated narrative effects atomically. Do not add an independently mutable shadow store or let presentation code invent narrative truth. See ADR-0008.
4. No required LLM, browser, graphics, or network dependency for classic CYOA.
5. Every feature needs automated tests and a reproducible demonstration; run `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` once a workspace exists.
6. Do not silently expand the public protocol. Document changes and migration implications.
7. Propose an ADR for changes to architectural boundaries or dependency directions before implementing them.
8. Keep PRs focused; document assumptions and unresolved questions rather than inventing large abstractions.
9. Treat generated content as proposals; validate game effects inside the engine.
10. Do not expose hidden NPC knowledge or other private game events to the presentation layer.
