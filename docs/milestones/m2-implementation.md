# M2 implementation and acceptance

M2 implements the 2026-10-30 replacement Cue Sheet decision. Historical reference
traces supply gameplay expectations; their older message syntax is not emitted.
The concrete compatible syntax is documented in [scene-format](../scene-format.md).

The engine remains headless and depends only on semantic types. The presentation
crate mirrors renderer instructions; adapters translate both directions; the text
renderer consumes presentation types only. The CLI is the composition root and
owns input, file I/O, text bindings and the progressing scene clock. Only existing
workspace serde is added as a direct CLI dependency; no external package or async
framework is introduced. Accepted ADRs and legacy engine timing are unchanged.

The scene DSL is an additive loader and API, rather than a reinterpretation of
legacy story schemas. This avoids changing M0–M1.8 behavior or checkpoint meaning.
All ordinary puzzle state is typed authored globals/locals. Context entry resets
locals; explicit transition rules preserve globals. No inventory manager, fixed
verbs, entity simulation or generated narrative is added.

| Acceptance | Executable evidence in `crates/cli/tests/scenes.rs` |
| --- | --- |
| Real DSL, separate complete renderer bindings | `locked_room_is_a_real_authored_dsl_with_complete_text_bindings` |
| 30s hidden timer, drawer T8–11, radio/reveal T10, 20s remaining, later replacement and context transition | `mandatory_drawer_radio_overlap_uses_one_clock_and_full_replacement` |
| Stable generated IDs, duplicate detection, reusable narrative binding | `generated_cue_identity_depends_on_content_and_timing_not_list_position` |
| Sequential barriers plus logical eligibility, including narration after release | `sequential_release_waits_for_both_logical_time_and_presentation_completion`, `untimed_narration_after_logically_delayed_actions_keeps_authored_order` |
| Idempotent cue execution, retained running identity, stale revision protection | `cue_acks_are_idempotent_and_old_callbacks_only_complete_retained_running_media` |
| Cross-scene duration floor | `transition_to_interactive_context_preserves_action_duration_on_the_new_clock` |
| Keys reference trace: 3/6/10/14s logical ends, one charge per action | `complete_locked_room_keys_trace_books_fourteen_seconds_once` |
| Strength 1/2/4 gives 4/2/1 attempts, damage and dexterity outcomes | `repeated_door_attempts_have_distinct_cues_strength_and_dexterity_outcomes` |
| Cutoff resolved immediately, effective fraction and exact-boundary authored policy | `five_second_action_at_six_is_resolved_as_four_seconds_at_ten_without_more_input`, `deadline_cutoff_and_authored_boundary_policy_commit_failure_immediately` |
| Fireplace/bar, wardrobe, window success/failure, extra budget | `alternate_tool_window_failure_and_timer_extension_are_authored` |
| Order-sensitive detective case and local reset/global persistence | `untimed_action_order_and_explicit_reentry_preserve_globals_reset_locals` |
| Save reconstruction and private projection | `cue_sheets_and_replay_checkpoints_round_trip_without_private_projections` |
| Real CLI success and cutoff, deterministic host clock | `actual_cli_plays_keys_escape_and_deadline_failure_without_real_sleeps` |
| Real CLI save/load continuation | `actual_cli_save_load_continues_with_keys_without_replaying_committed_actions` |
| Strict parse/type/ID/reference/timer errors, atomic runtime failures | `parsing_and_basic_compilation_reject_bad_rules_ids_timers_and_references`, `rule_overflow_invalid_time_and_invalid_checkpoint_leave_entire_state_unchanged`, `timeout_transition_cannot_offer_source_context_actions_in_its_destination` |
| Bounded idle replay growth | `idle_clock_samples_coalesce_without_changing_checkpoint_reconstruction` |

Reproduce with:

```sh
cargo fmt --all --check
cargo build --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo run --locked -p grimm-cli -- check-scene examples/locked-room/story.yaml
cargo run --locked -p grimm-cli -- play-scene examples/locked-room/story.yaml examples/locked-room/text.yaml
```

The actual terminal playtest read the introduction, opened the wardrobe with the
single `3` key, acknowledged its result, selected `5` to climb through the window,
and reached the authored roof escape. No Enter was needed to choose an option.
The same process exercised the real clock and completion barriers, rather than
only the deterministic test host. All M2 automated timing scenarios use explicit
clock observations, without sleeps; existing legacy real-time regressions remain.

Final local verification: formatting, workspace build, Clippy with warnings denied,
and **121 tests** (20 M2 tests plus the 101 legacy tests, including doctests).
The real CLI scripted demo and interactive terminal roof escape also passed.

Remaining limits and questions:

- Structural validation is not an exhaustive state-dependent reachability proof.
  Authors remain responsible for meaningful exits and fallback rules; M3 tooling
  is deliberately not implemented here.
- Text Enter acknowledgements are coarse. Graphical/audio bindings, presentation
  resume positions and application save envelopes are future host concerns.
- Checkpoints replay semantic events rather than store private mutable engine
  internals. Restore accepts at most 65,536 events; long sessions beyond that
  need a future versioned snapshot/compaction design. Consecutive observations
  coalesce, but actions and semantic cue acknowledgements remain recorded.
- Plan conditions are snapshots at plan construction; action conditions are
  rechecked at selection. Time-based scheduled content should use explicit
  `at_ms`, rather than expect a hidden rule evaluation loop.
- Generated IDs use a versioned deterministic content hash; explicit authored IDs
  are preferred for media whose identity must survive content edits. Changing
  checkpoint or identity derivation requires explicit version/migration review.
- Host timestamps are trusted monotonic observations; this is not a network
  fairness protocol or an untrusted savegame integrity scheme.

These are bounded M2 choices, not new architectural dependency directions. No
accepted ADR has been modified.
