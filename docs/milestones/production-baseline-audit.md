# WHITEOUT production baseline audit — 2026-11-01

The baseline is main `065d207` (PR #20). Inspection covered crate dependencies,
scene serialization/compiler, cue feedback/renderer, shared host/TUI, save/replay,
WHITEOUT story/text/playthroughs and integration/PTY tests. M4/M4.5 are complete;
no specific unfinished M4.5 acceptance criterion was found. The initial workspace
run passed 178 tests. M4.6's independent proposal was not evidence of missing
M4.5 functionality.

| Feature | Classification | Repository / behavioral evidence |
| --- | --- | --- |
| Authored graph/state/serialization | Complete | semantic scene/story models; engine compiler/validation; schema-1 archives, schema-2 examples, scenes/authoring/round-trip tests |
| Stable semantic IDs | Complete | normalization/collision diagnostics, M3 source-aware index, cue/action IDs independent of asset paths |
| Optional presentation durations and ACKs | Complete | semantic and presentation cues, adapters, CuePlayer and SceneSession; scene_timing/scenes tests |
| Ordered/concurrent cue execution | Complete | full REPLACE, at_ms, after_cue, retained identity and cancellation; fake-clock overlap/eligibility/deadline tests |
| Choice rules/input/UI | Complete (text); graphical implementation Missing | engine-authoritative conditional offers, duration/cutoff/effects, stale-input checks; structured TUI, terminal/PTY regressions |
| M4 text game | Complete | canonical nine-context WHITEOUT, alternative generator/radio routes, all three endings, cold exposure, archived actual CLI inputs |
| M4.5 time/UI/save | Complete | independent running scene clock, relative/absolute/reveal/extend/unset deadlines, automatic text, read-only inspector, legacy replay and real PTY checks |
| Explicit scene lifecycle/preload | Complete for text on feature branch; graphics Missing | M4.6 envelope/Ready/Begin/End/Stable and scoped actual text resolution; lifecycle/resource/CLI/PTY tests |
| Replaceable backing world storage | Complete on feature branch | atomic typed provider, injected rejection/stale revision/overflow/local reset tests, engine remains sole rule authority |
| Stable-boundary checkpoints | Complete opt-in on feature branch | history/world validation, transactional restore and fresh revision; legacy saves unchanged, CLI boundary round-trip and no replay tests |
| Renderer registry/catalog | Partially implemented | actual text exporter and scoped preparation, all-path catalog diagnostics; selected graphical renderer/exporter/import still missing |
| Screenplay production | Missing at audited baseline | existing text bindings are not a screenplay/staging model; first M5a fragment is the next increment |
| Visual identities/base assets/graphics engine | Missing | no canonical actor bible, production asset pipeline, sprite backend or reviewed graphics selection |
| Renderer-specific cueables | Partially implemented foundations | semantic playback exists; physical movement/camera/timelines/audio production missing and belongs to renderer |
| Authoring studio | Partially implemented CLI foundation | M3 index/lint and M4.6 catalog comparison; no integrated screenplay/asset preview or visual editor |

No relevant feature is labelled **Implemented but not tested**: implemented
runtime claims have automated evidence. Text preparation is not classified as
complete graphical loading. No exhaustive reachability or graphics capability
claim follows from these tests.

M4.6 design integration and the final 205-test verification are recorded in the
[integration ledger](m4-6-integration.md). The [canonical roadmap](../roadmap.md)
tracks later named phases without changing existing milestone numbers.
