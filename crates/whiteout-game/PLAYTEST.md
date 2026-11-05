# Generator-annex verification (#70)

## Automated checks (2026-11-05)

Rust/Cargo 1.98.1, Bevy 0.20.1, Linux. Commands from repository root, with
`CARGO_PROFILE_DEV_DEBUG=0 CARGO_BUILD_JOBS=2` for Cargo build/test/Clippy:

- `cargo fmt --all --check`: passed.
- `cargo build --workspace --locked`: passed.
- `cargo test --workspace --locked`: **258 passed**, zero failed; includes
  **37 WHITEOUT tests**, preserving the 26 existing #67–#69 tests.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: passed.

Tests use deterministic headless Bevy time, not wall-clock sleeps. They cover
full 32.5-second powered path, abandonment, timeout, exact deadline equality,
live condition recheck, timed effects, ordinary transition node, concurrent
caption/audio/camera, deduplication, stale requests, REPLACE cleanup over repeat
plays, fixed asset bounds, live missing assets/objects, real PCM decoding,
actual transform interpolation independent of effects, and canonical text parity.
The LSP bridge returned stale/time-out diagnostics; Cargo is the verification
source rather than a claim of clean LSP analysis.

## Desktop smoke

In progress before final PR readiness. Target is the actual 1000×700 Bevy window
on Linux/X11 with Mesa 26.2.4 llvmpipe (LLVM 23.1.1). Device-backed `AudioSink`
creation has been observed, with audio buffer underrun/overrun errors under
software rendering. Final route/capture results will be recorded after inspection.
No claim of production performance or subjective audio quality is made.
