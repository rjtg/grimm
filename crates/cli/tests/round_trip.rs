use std::io::Write;
use std::process::{Command, Stdio};

use grimm_adapters::{InputAdapter, OutputAdapter, TextAdapter};
use grimm_engine::{ActionError, Engine};
use grimm_presentation::RawInput;

fn demo(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{:?}", output);
    assert!(output.stderr.is_empty());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn executable_round_trip() {
    assert_eq!(
        demo("1\n"),
        "Welcome to GRIMM's architecture demo.\n1. Continue\n> The engine accepted your choice. Demo complete.\nEnding: Demo complete\n"
    );
}

#[test]
fn invalid_input_retries_and_eof_exits_without_committing() {
    let retried = demo("nonsense\n0\n2\n1\n");
    assert_eq!(
        retried.matches("Enter a displayed choice number.").count(),
        3
    );
    assert_eq!(retried.matches("Demo complete.").count(), 1);
    for input in ["", "bad\n", "\n"] {
        let output = demo(input);
        assert!(output.contains("1. Continue"));
        assert!(!output.contains("Demo complete."));
    }
}

#[test]
fn presentation_changes_and_forged_offers_cannot_commit_game_state() {
    let mut engine = Engine::default();
    let initial = engine.output();
    let mut screen = initial.clone();
    let mut adapter = TextAdapter::default();
    screen.content = vec![grimm_semantic::NarrativeContent::Narration {
        text: "Untrusted presentation text".into(),
    }];
    screen.offers[0].id = "invented".into();
    adapter.present(&screen);
    assert_eq!(engine.output(), initial);
    let forged = adapter.resolve(&RawInput { line: "1".into() }).unwrap();
    assert_eq!(engine.apply(&forged), Err(ActionError::InvalidChoice));
    assert_eq!(engine.output(), initial);

    adapter.present(&initial);
    let action = adapter.resolve(&RawInput { line: "1".into() }).unwrap();
    assert_eq!(action.choice_id, "continue");
    let completed = engine.apply(&action).unwrap();
    // Reusing an old screen still produces a proposal; only the engine rejects it.
    let stale = adapter.resolve(&RawInput { line: "1".into() }).unwrap();
    assert_eq!(engine.apply(&stale), Err(ActionError::StaleRevision));
    assert_eq!(engine.output(), completed);
    adapter.present(&completed);
    assert_eq!(adapter.resolve(&RawInput { line: "1".into() }), None);
}
