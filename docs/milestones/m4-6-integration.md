# M4.6 integration evidence and remaining implementation

Audit date: 2026-11-01. Target: `feat/whiteout-production-roadmap`, based on
`origin/main` at `065d207` (merged M4.5 PR #20).

## What was actually merged

PR #21 (`docs/m46-scene-state-and-m5-production-plan`) contains only:

- `455cb081037f29cdc7fb2930c896a6c3634cc207`: M4.6 architecture proposal.
- `10ba7a7e4c5b464054dd6eaec04a6e077b698595`: M5a–M5g production proposal.

These commits were merged into the target branch with **merge commit `c117675`**.
There were no merge conflicts: the branch adds two documents and changes no Rust
code, existing story data or accepted ADR. `main` and GitHub PR #21 were not
merged by this operation. No separate M4.6 implementation branch was found in
remote heads or the repository's PR inventory.

The initial M5 screenplay/roadmap experiment is isolated at local
`wip/whiteout-screenplay-contract` (`323379b`), outside this integration branch.
It is not a completed milestone and must not be silently included before M4.6.

## Verified baseline

M4.5 is **complete**: `SceneEngine` schema 2 and protocol 2, semantic feedback,
`CuePlayer`, shared `SceneSession`, TUI, pure `inspect_world`, schema-1 archives,
WHITEOUT's three endings, automatic narrative playback and deadline cancellation
are implemented and tested. The baseline workspace run passed **178 tests**.
The source review included engine output/inspection, session coordination,
renderer cue lifecycle and dependency-boundary tests. LSP diagnostics were
requested for representative Rust files; one response timed out and CuePlayer
reported no diagnostics. Tests provide the additional semantic evidence.

## M4.6 acceptance ledger

The following are **missing M4.6 work**, not reasons to reopen M4.5:

| Area | Current evidence / status | Required completion evidence |
| --- | --- | --- |
| Explicit prepare/begin/end/stable boundary | **Missing**: current CueSheet has revision/clock/cues only; GOTO immediately enters a new epoch | Typed lifecycle, idempotent and stale ACK tests, reentry/endings, resource readiness/failure, executable host demo |
| Scene ID/preloading | **Missing** at runtime; namespaces are not a preload API | Stable semantic scene/scope metadata, readiness before T=0, missing-resource diagnostic without changed story outcomes |
| World-state provider | **Missing**: engine stores globals/locals internally and clones candidate state transactionally | Replaceable typed storage, InMemory implementation, atomic effects/local lifecycle, stale revision, rollback/replay and inspection tests |
| Stable-boundary checkpoints | **Missing**: M4.5 supports paused mid-scene saves | Explicit persistent boundary, versioned opt-in policy, next-scene deterministic restore, original schema-1/2 saves remain executable |
| Renderer catalog | **Partially implemented**: strict portable comparison CLI and all-path/scope/duration tests; actual renderer exporter/resource loading missing | Strict versioned catalog, all-path ID/scope/completion/duration diagnostics, real supplied resource validation at renderer production gate |
| Existing timing rules | **Complete M4.5 foundation**, M4.6 lifecycle interaction unimplemented | Preserve relative actual-start deadlines, absolute deadlines, unset/reveal/extend, interruption and cold accounting across lifecycle boundaries |
| Debug/CLI integration | **Partially implemented**: M4.5 inspector exists | Show prepare/end/stable states and provider snapshots; preserve noninteractive/manual-clock execution |
| Migration/demo/docs/checks | **Missing for M4.6** | Explicit versioning, end-to-end lifecycle demo, migration instructions, full fmt/build/test/Clippy verification |

M4.6 must not be marked complete until every remaining row has implementation
and test evidence. A document merge or catalog fixture alone is insufficient.

## Preservation and implementation order

Preserve schemas 1/2 and their M4.5 behavior. New lifecycle/boundary-save semantics
must be explicitly versioned rather than silently disabling working mid-scene
saves. Keep engine authority over validated rule transactions (ADR-0004); a
provider may own storage, never independently decide narrative consequences.
Do not move renderer catalogs, asset paths, world values or graph data into
player Cue Sheets. Propose/document the concrete lifecycle/storage decisions
before changing public contracts.

Implement the independent catalog validation first, then atomic provider and
versioned lifecycle/checkpoint coordination, and integrate the host/inspector.
Only after those blocking M4.6 gates pass, restore the small M5a screenplay
increment and continue the existing M5a–M5g production sequence. These named
phases correspond to the requested M5–M11 production work without renumbering
M5's existing graphical-adaptation commitment or M6's statistics proposal.

## Catalog increment

`crates/cli/src/catalog.rs` and `check-catalog` implement strict manifest parsing
and source-index comparison. Three regression tests cover hidden branches,
unknown/mismatched scopes, exact versus estimated duration/completion, duplicate
IDs, unused declarations and actual CLI execution. See the
[concrete catalog format](m4-6-catalog-format.md). This is one verified increment,
not completion of M4.6 or of renderer asset validation.

Verification after the actual document merge and catalog increment:
`cargo fmt --all --check`, `cargo build --workspace --locked`,
`cargo clippy --workspace --all-targets --locked -- -D warnings` and
`cargo test --workspace --locked` passed (**181 tests, 0 failures**).
This proves the existing M4.5 functionality still passes on the merged target;
it does not substitute for the missing M4.6 lifecycle/provider/checkpoint gates.
