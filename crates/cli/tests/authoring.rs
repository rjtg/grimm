use grimm_cli::authoring::inspect;
use grimm_semantic::authoring::{Report, Severity, StoryIndex};
use std::process::Command;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/authoring/{name}.yaml",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}
fn report(name: &str) -> Report {
    inspect(&format!("{name}.yaml"), &fixture(name))
}

#[test]
fn compiler_errors_keep_ids_paths_and_both_conflicting_locations() {
    for name in [
        "normalized-actions",
        "generated-collision",
        "explicit-collision",
    ] {
        let yaml = fixture(name);
        let result = report(name);
        assert!(result.has_errors(), "{result:?}");
        let d = &result.diagnostics[0];
        assert!(
            d.message.contains("duplicate") || d.message.contains("collid"),
            "{d:?}"
        );
        assert_eq!(d.source.file, format!("{name}.yaml"));
        assert_eq!(d.related.len(), 1, "{d:?}");
        assert_ne!(d.source.line, d.related[0].line, "{d:?}");
        for source in std::iter::once(&d.source).chain(&d.related) {
            assert!(source.line.is_some(), "{source:?}\n{yaml}");
            assert!(source.column.is_some());
        }
        assert!(!d.suggestion.is_empty());
        assert!(result.index.is_none());
    }
}

#[test]
fn references_are_checked_by_the_runtime_compiler() {
    for (name, id) in [
        ("missing-action", "room.missing"),
        ("missing-variable", "global.missing"),
        ("missing-target", "absent"),
        ("unreachable", "unused"),
    ] {
        let result = report(name);
        assert!(result.has_errors());
        assert!(result.diagnostics[0].message.contains(id), "{result:?}");
        assert!(result.diagnostics[0].source.line.is_some(), "{result:?}");
    }
}

#[test]
fn structural_traps_and_no_exit_fail_but_cycles_and_bad_endings_are_valid() {
    for (name, code) in [
        ("trapped-region", "no_terminal_path"),
        ("no-exit", "no_exit"),
        ("constant-false", "unreachable_context"),
    ] {
        let result = report(name);
        assert!(
            grimm_engine::scene::SceneEngine::new(serde_yaml_ng::from_str(&fixture(name)).unwrap())
                .is_ok(),
            "authoring lint must not silently change runtime compatibility"
        );
        assert!(result.has_errors(), "{result:?}");
        assert!(
            result.diagnostics.iter().any(|d| d.code == code),
            "{result:?}"
        );
        assert!(
            result.index.is_some(),
            "structural lint retains the compiled inventory"
        );
    }
    for name in ["cycle-exit", "valid-endings", "conditional-exit"] {
        let result = report(name);
        assert!(!result.has_errors(), "{result:?}");
    }
    let result = report("conditional-exit");
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.code == "state_dependent" && d.severity == Severity::Info)
    );
    let result = report("constant-false");
    assert!(
        result
            .diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning)
    );
}

#[test]
fn index_is_sorted_repeatable_and_distinguishes_cues_content_and_actions() {
    let a = report("valid-endings");
    assert_eq!(a, report("valid-endings"));
    let index = a.index.unwrap();
    assert_eq!(
        index
            .contexts
            .iter()
            .map(|c| c.id.as_str())
            .collect::<Vec<_>>(),
        ["failure", "room", "success"]
    );
    let room = &index.contexts[1];
    assert_eq!(room.actions[0].id, "room.leave");
    assert_eq!(room.actions[0].uses.len(), 1);
    assert!(room.actions[0].uses[0].line.is_some());
    assert!(room.cues.iter().any(|c| c.cue_id == "room.welcome"
        && !c.generated
        && c.narrative_id.as_deref() == Some("room.shared_greeting")));
    assert_eq!(room.transitions.len(), 2);
    assert!(index.contexts[0].cues[0].generated);
    assert_eq!(
        index.contexts[0].narratives[0].narrative_id,
        index.contexts[2].narratives[0].narrative_id
    );
    let json = serde_json::to_string(&index).unwrap();
    assert!(!json.contains("__run_"));
    assert_eq!(index, serde_json::from_str::<StoryIndex>(&json).unwrap());
}

#[test]
fn source_locations_work_for_flow_yaml_quoted_names_and_comments() {
    let yaml =
        fixture("normalized-actions").replace("id: Open Door", "id: 'Open Door' # quoted name");
    let result = inspect("quoted.yaml", &yaml);
    let d = &result.diagnostics[0];
    let first = d.related[0].line.unwrap();
    let second = d.source.line.unwrap();
    assert!(yaml.lines().nth(first - 1).unwrap().contains("'Open Door'"));
    assert!(yaml.lines().nth(second - 1).unwrap().contains("open-door"));
    let malformed = inspect("syntax.yaml", "contexts: [");
    assert_eq!(malformed.diagnostics[0].code, "invalid_yaml");
    assert!(malformed.diagnostics[0].source.line.is_some());
}

#[test]
fn authored_examples_export_indices_without_renderer_assets() {
    for path in [
        "examples/locked-room/story.yaml",
        "examples/locked-room/detective.yaml",
    ] {
        let yaml = std::fs::read_to_string(format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR")))
            .unwrap();
        let result = inspect(path, &yaml);
        assert!(!result.has_errors(), "{result:?}");
        let index = result.index.unwrap();
        assert!(index.contexts.iter().all(|c| c.source.line.is_some()));
        assert!(
            index
                .contexts
                .iter()
                .flat_map(|c| &c.cues)
                .all(|cue| cue.source.line.is_some())
        );
    }
}

#[test]
fn cli_json_and_exit_codes_are_usable_in_ci() {
    let path = |name: &str| {
        format!(
            "{}/tests/fixtures/authoring/{name}.yaml",
            env!("CARGO_MANIFEST_DIR")
        )
    };
    let index = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .args(["index-scene", &path("valid-endings")])
        .output()
        .unwrap();
    assert!(
        index.status.success(),
        "{}",
        String::from_utf8_lossy(&index.stderr)
    );
    serde_json::from_slice::<StoryIndex>(&index.stdout).unwrap();
    for name in ["normalized-actions", "trapped-region", "missing-target"] {
        let output = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
            .args(["check-scene", &path(name), "--json"])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1));
        assert!(
            serde_json::from_slice::<Report>(&output.stdout)
                .unwrap()
                .has_errors()
        );
    }
    let output = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .args(["check-scene", &path("normalized-actions")])
        .output()
        .unwrap();
    let text = String::from_utf8(output.stderr).unwrap();
    assert!(text.contains("error[invalid_scene]"));
    assert!(text.contains("related declaration:"));
    assert!(text.contains("help:"));
}

#[test]
fn effective_id_sets_survive_declaration_reordering_and_match_runtime_ids() {
    use grimm_semantic::scene::SceneStory;
    let original = fixture("valid-endings");
    let mut story: SceneStory = serde_yaml_ng::from_str(&original).unwrap();
    story.contexts.reverse();
    story.contexts.iter_mut().for_each(|context| {
        context.actions.reverse();
        context.cues.reverse();
    });
    let reordered = serde_yaml_ng::to_string(&story).unwrap();
    let ids = |report: Report| {
        report
            .index
            .unwrap()
            .contexts
            .into_iter()
            .map(|c| {
                (
                    c.id,
                    c.actions.into_iter().map(|a| a.id).collect::<Vec<_>>(),
                    c.cues
                        .into_iter()
                        .map(|cue| (cue.cue_id, cue.generated, cue.narrative_id))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        ids(inspect("original.yaml", &original)),
        ids(inspect("reordered.yaml", &reordered))
    );
    let engine = grimm_engine::scene::SceneEngine::new(story).unwrap();
    let index = inspect("reordered.yaml", &reordered).index.unwrap();
    let room = index.contexts.iter().find(|c| c.id == "room").unwrap();
    for cue in engine.output().cues {
        assert!(room.cues.iter().any(|indexed| indexed.cue_id == cue.cue_id));
    }
}

#[test]
fn typed_rules_branches_timers_and_transitions_have_actionable_source_diagnostics() {
    let base = fixture("valid-endings");
    for (replacement, expected) in [
        (
            "effects: [{op: set, var: global.ready, value: 1}]\n        outcomes: [{goto: success}]",
            "type",
        ),
        (
            "when: {op: eq, left: {var: global.ready}, right: 1}\n        outcomes: [{goto: success}]",
            "type",
        ),
        ("outcomes: []", "ELSE"),
        (
            "outcomes: [{goto: success, cues: [{kind: narrative, narrative_id: exit, at_ms: 0}]}]",
            "at_ms",
        ),
    ] {
        let yaml = base.replace("outcomes: [{goto: success}]", replacement);
        let result = inspect("rules.yaml", &yaml);
        assert!(result.has_errors(), "{result:?}");
        let d = &result.diagnostics[0];
        assert!(d.message.contains(expected), "{d:?}");
        assert!(d.source.line.is_some(), "{d:?}");
        assert!(!d.suggestion.is_empty());
    }
    for invalid in ["start_ms: 0", "start_ms: 5, extend_ms: 1"] {
        let yaml = base.replace(
            "{cue_id: welcome, kind: narrative, narrative_id: shared_greeting}",
            &format!("{{kind: timer, {invalid}}}"),
        );
        let result = inspect("timer.yaml", &yaml);
        assert!(result.has_errors());
        assert!(result.diagnostics[0].message.contains("timer"));
        assert!(result.diagnostics[0].source.line.is_some());
    }
    let invalid_id = inspect("unicode.yaml", &base.replace("id: leave", "id: öffnen"));
    assert!(invalid_id.diagnostics[0].message.contains("ASCII"));
}

#[test]
fn namespace_and_context_collisions_report_both_declarations() {
    let base = fixture("valid-endings");
    for yaml in [
        base.replace("id: failure", "id: SUCCESS"),
        base.replace("id: success\n", "id: success\n    namespace: SAME\n")
            .replace("id: failure\n", "id: failure\n    namespace: same\n"),
    ] {
        let result = inspect("duplicate-context.yaml", &yaml);
        let d = &result.diagnostics[0];
        assert!(result.has_errors());
        assert_eq!(d.related.len(), 1, "{d:?}");
        assert_ne!(d.source.line, d.related[0].line);
        assert!(d.source.line.is_some() && d.related[0].line.is_some());
    }
}

#[test]
fn duplicate_if_guards_retain_both_branch_sources() {
    let yaml = fixture("valid-endings").replace("outcomes: [{goto: success}]", "outcomes:\n          - when: {op: eq, left: {var: global.ready}, right: true}\n            goto: success\n          - when: {op: eq, left: {var: global.ready}, right: true}\n            goto: success\n          - goto: failure");
    let report = inspect("branches.yaml", &yaml);
    let d = &report.diagnostics[0];
    assert!(d.message.contains("duplicate IF guard"), "{d:?}");
    assert_eq!(d.related.len(), 1);
    assert_ne!(d.source.line, d.related[0].line);
    assert!(d.source.line.is_some() && d.related[0].line.is_some());
}

#[test]
fn declared_exits_do_not_hide_a_plan_that_never_offers_an_attempt() {
    let report = report("no-offers");
    assert!(report.has_errors());
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.code == "no_entry_progress")
    );
    // Scheduled offers remain possible; this does not attempt a timing solver.
    let timed = fixture("valid-endings").replace(
        "kind: actions, available_actions",
        "at_ms: 1000, kind: actions, available_actions",
    );
    assert!(!inspect("scheduled.yaml", &timed).has_errors());
    // An authored timeout is a legitimate way to progress without any offers.
    let timeout = fixture("no-offers")
        .replace(
            "    actions:\n",
            "    timeout: {goto: failure}\n    actions:\n",
        )
        .replace(
            "      - {cue_id: welcome",
            "      - {kind: timer, start_ms: 1000}\n      - {cue_id: welcome",
        );
    assert!(!inspect("timeout.yaml", &timeout).has_errors());
}
