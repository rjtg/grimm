# M4.5 implementation evidence

The [approved specification](m4-5-terminal-timing-spec.md) is implemented across
semantic/presentation cue types, `SceneEngine`, the output adapter, `CuePlayer`,
the shared CLI `SceneSession` coordinator, pipe transport and structured TUI.
No architecture dependency direction changed; no M5 graphics or M6 statistics
were introduced. Ratatui is the single added direct UI dependency, confined to
the CLI and text renderer. The engine still has only its semantic dependency.

## Protocol, authoring and compatibility

See the [migration contract](../migrations/m4-5-scenes.md) for every new field,
schema/checkpoint versioning, scheduling order and the read-only inspection API.
Schema 2 always starts scene time at zero on entry, independently of an optional
deadline. Authored relative/absolute setting, extension, visibility and removal
never restart that clock. GOTO owns the new epoch; unrestricted restart is absent.

Emitted protocol 2 uses mandatory numeric `at_ms`, optional narrative durations
and explicit completion dependencies. Original start timestamps preserve live
cue identity across REPLACE/restore. Full replacement never inherits old pending
controls. Canonical history is semantic feedback/actions/time; bounded inspector
history and scrollback remain presentation-only. The host resolves equal-time
expiry before auto-completions/starts and freezes time during action evaluation.

The shipped M2–M4 examples now have automatic durations and authored entry-time
deadlines. Successful WHITEOUT generator startup and transmission remove their
deadlines; other actions leave them running. Schema-1 originals remain archived
and executable for matching old checkpoints. Legacy CYOA formats are unchanged.

## Terminal behavior and controls

Normal TTY `play-scene` uses alternate-screen structured presentation. Header,
status, timer, scrollback and choices occupy separate regions. Long choices wrap;
Tab focuses choices for arrows/Enter navigation beyond nine entries. F2 toggles
the inspector; on narrow terminals it occupies full width. Tab cycles narrative,
choices, cues and world state; arrows/PageUp/PageDown scroll the focused pane.
`[`/`]` select older/newer issued sheets, bounded to 32 revisions. IDs and statuses
remain readable without color; matching cue colors are supplemental. World
values are scoped and read-only, with changes marked. No unsafe state editor/F3
shortcut is provided; the migration guide documents a replay-safe mutation design.

Esc pauses the renderer clock and opens Continue/Save/Load/Quit. Commands captured
from an expired revision cannot affect a successor. Queued terminal keys are
discarded before a changed revision or scheduled offer set is displayed, so
old numeric keys and Esc cannot act on the newly shown interaction. F4 exports both widget layouts
to `.grimm-captures/STORY_ID`. Ctrl-C exits with cleanup. Resize and ordinary error
unwinding restore raw-mode/alternate-screen state through RAII.

Automatic narrative duration playback is the interactive default. Enter may
explicitly finish running text; `--manual-reading` disables auto-completion.
Pipes/manual-clock sessions keep manual reading by default for scripted
compatibility; use `--auto-reading` for deterministic duration playback there.
`:time`, `:advance`, exact `@action` IDs and file commands remain available.
EOF never acknowledges text: real-time authored deadlines and auto cues continue;
a manual clock or unstarted schema-1 clock exits without inventing elapsed time.

## Evidence

| Requirement | Automated or interactive evidence |
| --- | --- |
| Clock without deadline, automatic text and dependency barrier | `scene_clock_runs_without_deadline_and_auto_text_unblocks_its_dependency` |
| Relative/absolute/extend/show/hide and REPLACE continuity | `unrelated_replace_visibility_and_extension_never_restart_the_clock` |
| Authored cancellation, continuing clock and restored cancellation | `authored_cancel_removes_deadline_but_clock_and_replay_continue` |
| Exact-boundary versus interrupted work; idle ordering | `completion_at_boundary_can_cancel_but_a_crossing_action_cannot`, `deadline_wins_equal_time_auto_completion_and_stale_input_cannot_choose_successor` |
| T=8 work, overlapping T=10 warning/reveal, duration eligibility and later replacement | `automatic_overlap_reveal_and_sequential_release_share_one_continuous_clock` |
| Retained identity, original start and deterministic replay | `retained_running_narrative_does_not_restart_or_extend_after_replace_or_restore`, `constructing_host_from_restored_engine_recovers_running_text_without_restarting_it` |
| Pending cancellation, retained historical rows and bounded history | `cancellation_updates_all_retained_revision_rows`, `pending_replacement_and_bounded_inspector_history_do_not_create_engine_events` |
| Pure scoped host-only inspection | `inspection_is_pure_typed_and_separate_from_player_output` |
| Diagnostics and versioning | `lifecycle_diagnostics_reject_conflicting_deadlines_bad_dependencies_and_overflow`, `legacy_generated_ids_are_unchanged_by_protocol_lowering`, `archived_schema_one_story_and_checkpoint_still_restore_with_original_clock_semantics` |
| Structured layouts, Unicode, bounded scrolling/focus | `scene_ui::tests` using Ratatui TestBackend |
| Real keys, resize, save/load, normal/error teardown, EOF timeout chain and stale menu | `scene_tui_automatic_text_inspector_resize_menu_and_cleanup` using a real PTY and pipe subprocesses |
| Cold accounting including automatic presentation and full rescue | `automatic_whiteout_books_presentation_once_and_reaches_full_rescue_without_text_input` |
| All previous endings, alternate routes, cutoff work credit, scripted CLI and M3 index | Existing WHITEOUT, scene, authoring, boundary and legacy suites |
| Actual interactive WHITEOUT full rescue with Inspector | [Playtest report](../../examples/whiteout/DESIGN.md#m45-timing-and-terminal-adaptation), [both layouts and ending capture](../../examples/whiteout/captures/README.md) |

The complete interactive route used immediate choice keys, automatic text,
engineering extension, cold-dependent latch attempts, isolation/startup, a paused
save in dispatch, warming, the ceramic radio jumper and coordinated full rescue.
The captured second session restored the actual dispatch checkpoint and entered
the radio cabin with its full ongoing deadline. No Enter text input was needed.

## Limits

The inspector reads current host state even when displaying a historical sheet;
it is not a historical world-state debugger. Renderer scrollback (512 entries)
and revision history (32 sheets) are intentionally bounded and not checkpointed.
Save/restore reconstructs unfinished semantic cues, not animation frames or prior
scrollback. Old checkpoints need their matching archived rules; no speculative
clock conversion is attempted. Hidden countdowns remain hidden in ordinary play.
This is one scene clock/optional deadline, not multiple timers or simulation.
Terminal support requires an interactive TTY; Windows uses the same cross-platform
backend but the real PTY regression runs on Unix. Abrupt process kill cannot run
RAII cleanup. Author-set durations are a text pacing default, not voice/animation
assets or an exhaustive difficulty-balancing guarantee.

## Verification

All required checks passed on the implementation branch:

- `cargo fmt --all --check`
- `cargo build --workspace --locked`
- `cargo test --workspace --locked`: **178 passed, 0 failed**, including the
  original 158 tests/doctests, 16 new timing tests, two layout tests, the combined
  PTY/EOF regression and the automatic WHITEOUT full-rescue test.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `git diff --check`

Semantic LSP feedback was requested for touched Rust files; several responses
were stale/timeouts. Cargo compilation, tests and clean Clippy provide final
verification for those files. PTY regressions verify normal and error cleanup;
the real interactive playthrough/captures provide additional usability evidence.
The CLI ending regression also requires an exact standalone heading line, matching
the offline CI walkthrough checks rather than merely a substring.
