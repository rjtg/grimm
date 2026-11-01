# M4.6 integration evidence

Verified 2026-11-01 on `feat/whiteout-production-roadmap`, based on main
`065d20787a2e61eb126ed81e63415f89b2430cdf` (merged M4.5 PR #20).

## Actual integration and scope

Repository/remote/PR inspection found no M4.6 runtime implementation branch.
Open PR #21 (`docs/m46-scene-state-and-m5-production-plan`) contained only:

- `455cb081037f29cdc7fb2930c896a6c3634cc207`: M4.6 architecture proposal.
- `10ba7a7e4c5b464054dd6eaec04a6e077b698595`: M5a–M5g production proposal.

These were **actually merged into the feature target at `c117675`**, without
conflicts or discarded functionality. The proposal changed no Rust or story data.
Main and PR #21 were not merged by this operation. Missing blocking M4.6 runtime
work was then implemented on this feature branch, rather than recreating an
existing implementation. The original M5 experiment was isolated locally at
`323379b` until the M4.6 verification below passed.

M4.5 was already complete: the initial workspace run passed 178 tests. Its
engine, timing, TUI, inspection and legacy checkpoint behavior were preserved.
Accepted ADRs and crate dependency directions were not changed. Concrete opt-in
implementation choices are proposed for review in [ADR-0008](../adr/0008-m46-versioned-lifecycle-and-storage.md).

## Verified acceptance ledger

| Gate | Status / evidence |
| --- | --- |
| Prepare/ready/begin/end/stable | Complete: LifecycleEngine, typed correlated feedback and persistent boundary; duplicate/stale feedback, readiness failure, reentry/epoch and terminal closure regressions in lifecycle.rs. |
| Scene/preloading | Complete for text: explicit scenes.yaml, actual TextResources scope cache before Ready, failed preparation retains old cache; no filename-derived scene IDs or engine asset loading. |
| Provider | Complete: typed atomic WorldStateProvider, offline InMemoryWorldState, ProvidedSceneEngine candidate/CAS; rollback, overflow/type/scope/revision/local-reset and replay tests in world_provider.rs. |
| Boundary checkpoints | Complete opt-in: coherent semantic history/world/config validation, atomic restore, fresh revision, no old graphics replay, persistent window, initial load and completed-game saves. Actual lifecycle_cli and PTY round-trips; legacy schemas/saves unchanged. |
| Catalog | Complete neutral contract/text exporter: actual bindings, all-path M3 comparison, explicit scopes, exact/estimate and completion diagnostics, duplicate/missing/unused IDs and resource URI failure; catalog.rs and text_resources.rs tests. Physical graphics exporter/import is M5 work. |
| Time | Complete: relative actual-start, absolute/reveal/extend/unset and cutoff tests; 5,000ms outgoing media with frozen gameplay; existing overlap/interruption/credit tests; all three WHITEOUT endings preserve canonical world and action-cost history after every decision. |
| CLI/TUI/debug | Complete: opt-in flag, real scoped preparation, persistent checkpoint menu, phase/scene/epoch/readiness/next-scene/provider revision inspector, manual-clock pipe path, outgoing renderer-only media, actual PTY cleanup/resize/one-key/menu test. |
| Demo/migration | Complete: README exact WHITEOUT command; concrete configuration/protocol/checkpoint migration, resource exporter/checker and manual commands in the migration guide; actual CLI runs reach all three authored endings. |

## Incremental delivery

- `59476c4`: catalog validation and integration-gap ledger.
- `c2dcfa6`: headless lifecycle and transactional backing storage.
- `fc675c7`: presentation envelope/adapter and fresh initial boundary restore.
- `b3602c5`: shared host and renderer-only outgoing media clock; retained completed
  control ACK correlation fix.
- `7aaaf0b`: actual text registry exporter/preparation and WHITEOUT scene scopes.
- `d761553`: real CLI/TUI lifecycle and boundary-save integration; fixes boundary
  load-status overwrite found by the PTY regression.

These increments were pushed for inspection in draft PR #22. Reviews, general
comments and inline comments were checked before pushes. PR #21 received an
actionable architecture review; the per-item responses appear below. PR #22
had no additional feedback at the latest check. Review remains required; implementation status is not
an assertion that main already contains these changes.

## Exact verification before M5a

All passed on the integrated implementation:

```text
cargo fmt --all --check
cargo build --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

**205 tests passed; 0 failed.** Includes all existing games, M4.5 regressions,
three real PTY tests, real lifecycle boundary/reentry CLI runs, actual WHITEOUT
lifecycle CLI playthroughs to every ending, catalog export and deterministic
legacy-versus-lifecycle world/action-history parity. LSP diagnostics were
requested for changed Rust files: some were current/clean, others timed out with
stale data. Cargo verification supplies the semantic fallback; timeout responses
are not counted as clean diagnostics.

## Limits and review points

See the [concrete contract](../migrations/m4-6-scenes.md). Configuration 1 opts
into envelope 3 / boundary checkpoint 1 while stories stay schema 1/2. Destination
locals commit with transition effects at selection; destination execution waits
for Begin. Atomicity is a provider implementation contract; GRIMM cannot undo a
malicious provider that violates it. No independent external world writes, debug
mutation, database or graphics importer is introduced.

Text catalogs prove real text resources only. Graphical catalogs must later come
from the selected renderer's real assets. No graphics engine or asset production
is claimed. ADR-0008 remains proposed; reviewers should assess versioning,
commit ordering and storage contract. Existing accepted architecture remains the
baseline. M5a can now proceed as an offline representative contract increment;
the complete graphical milestone remains unfinished.

## PR #21 architecture-review checklist

A COMMENT review on PR #21 (2026-11-01) is design feedback, not approval. PR #21
stays design-only; implementation and evidence belong to PR #22. Responses:

1. **Real host/readiness:** scene_runtime.rs `prepare_renderer`, TextResources,
   scene_tui.rs and SceneSession; tests
   `missing_actual_resources_and_failed_preparation_never_silently_fall_back`,
   `real_cli_reentry_prepares_the_original_scene_again`,
   `lifecycle_tui_boundary_menu_save_load_and_cleanup`,
   `preparation_readiness_and_duplicate_lifecycle_feedback_cannot_start_early`,
   `revisiting_same_scene_rejects_prior_epoch_cue_and_lifecycle_acknowledgements`,
   `automatic_terminal_text_finishes_before_scene_closure_and_does_not_replay_on_load`.
2. **Authority/atomicity:** engine/scene/state.rs and lifecycle.rs; tests
   `engine_and_storage_commit_effects_and_scope_atomically_and_restore_by_replay`,
   `backend_failure_and_stale_revision_leave_engine_history_and_state_unchanged`,
   `typed_storage_rejects_partial_effects_overflow_unknown_variables_and_bad_scope`.
   Revision/epoch ownership is explicit in the migration guide.
3. **Save compatibility:** distinct APIs/formats/default filenames; tests
   `legacy_and_boundary_save_formats_are_rejected_cross_mode_without_mutation`,
   `boundary_restore_replays_no_outgoing_graphics_and_rejects_inconsistent_world`,
   `real_cli_only_saves_at_boundary_and_loads_without_replaying_outgoing_media`.
4. **Actual catalog:** catalog.rs / text_resources.rs, all-path M3 comparison and
   real text exporter; all catalog/resource tests, including
   `actual_export_command_produces_a_usable_catalog`. Physical graphics readiness
   remains an explicit M5 gate, not a claim inferred from a mock catalog.
5. **REPLACE/time:** test
   `lifecycle_replacement_retains_running_warning_once_and_cancels_obsolete_schedule`
   uses WHITEOUT T=8 latch, T=10 radio/reveal, later REPLACE and cancellation.
   Existing scene_timing tests remain; lifecycle.rs verifies actual-start deadlines,
   cutoff, terminal closure and 5,000ms frozen outgoing media. All-ending parity
   checks action histories/cold accounting at every decision.
6. **Verification:** the 205-test full run above follows CLI/TUI integration; it
   is not the old 181-test catalog baseline. Both review-specific regressions
   passed in the final 205-test full workspace run with all four checks.
7. **One game:** story.yaml/text.yaml remain unchanged. M5a checks original text
   and graph/cue/action references before broader conversion.
8. **Staging ownership:** descriptive production data only; no core camera/motion
   DSL or new per-track protocol.
9. **Renderer decision:** canonical roadmap M5c requires the same 30–60s scene,
   actual assets/preload/radio overlap/choices/deadline and a reviewed ADR before
   bulk sprites, final models or migration. No renderer selected here.
10. **Tooling scope:** CLI/catalog/screenplay validators first; stable identity
    specifications separate from generated artifacts. No authoring IDE.
