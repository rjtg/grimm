# Verification

Baseline `5aaeaf8`; production-storyboard-only change. Canonical story/text,
Rust sources and existing cast/environment references are unchanged.

Successful local checks:

```text
cargo fmt --all --check
CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 cargo build --workspace --locked
CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 cargo clippy --workspace --all-targets --locked -- -D warnings
CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2 cargo test --workspace --locked
```

All exited 0; workspace tests: **258 passed, 0 failed**, across 39 test/doc-test
suite results (including empty suites). No new Rust behavior or tests are needed
for static art. The build environment flags limit local memory/debug output.

Artifact checks:

```bash
python3 -m py_compile examples/whiteout/production/storyboards/sources/*.py
python3 examples/whiteout/production/storyboards/sources/draw_boards.py
python3 examples/whiteout/production/storyboards/sources/publish_pages.py
python3 examples/whiteout/production/storyboards/sources/audit.py
```

Native SVG raster export succeeds for all 107 panels and 21 sheets. The audit
checks unique shot IDs, required metadata, actual SVG XML/PNG integrity and
1600×900 panel dimensions, local Markdown links, all nine contexts, all 57
narrative bindings and all 35 actions. COVERAGE lists all 95 authored cue instances.
It does not prove reachability or aesthetic quality. The separate visual review
of all scene sheets is recorded in CONTINUITY.

Python semantic diagnostics were attempted through codex-lsp-bridge. The server
is unavailable (`pyright-langserver` ENOENT); compilation, full export and narrow
artifact audit were used as fallback. No clean LSP result is claimed.

Runtime screenshot tests, final animation/rig checks and graphical gameplay
changes are outside this task. CI results are reported on the PR for its exact
head commit rather than permanently asserting a future CI state in this file.
