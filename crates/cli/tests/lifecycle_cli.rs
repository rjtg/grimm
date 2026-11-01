//! Real CLI boundary saves and preparation; manual clocks avoid timing races.
use std::{
    io::Write,
    process::{Command, Stdio},
};
fn play(commands: &str, extra: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_grimm-cli"))
        .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
        .args([
            "play-scene",
            "crates/cli/tests/fixtures/lifecycle/story.yaml",
            "crates/cli/tests/fixtures/lifecycle/text.yaml",
            "--lifecycle",
            "crates/cli/tests/fixtures/lifecycle/scenes.yaml",
            "--manual-clock",
            "--auto-reading",
        ])
        .args(extra)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(commands.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn stdout(output: &std::process::Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout.clone()).unwrap()
}
#[test]
fn real_cli_only_saves_at_boundary_and_loads_without_replaying_outgoing_media() {
    let path = std::env::temp_dir().join(format!("grimm-boundary-cli-{}.json", std::process::id()));
    let input = format!(
        ":save {}\n:time 10\n@room.travel\n:present 5000\n:save {}\n:continue\n:time 100\n@hall.finish\n:continue\n",
        path.display(),
        path.display()
    );
    let out = stdout(&play(&input, &[]));
    assert!(
        out.contains("checkpoints require a stable scene boundary"),
        "{out}"
    );
    assert!(out.contains("Saved"), "{out}");
    assert!(out.contains("Leaving the room behind"), "{out}");
    assert!(out.contains("The hall is quiet"), "{out}");
    let checkpoint: grimm_semantic::lifecycle::BoundaryCheckpoint =
        serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(checkpoint.world.scope.context_id, "hall");
    let restored = stdout(&play(
        ":continue\n:time 100\n@hall.finish\n:continue\n",
        &["--load", path.to_str().unwrap()],
    ));
    assert!(!restored.contains("Leaving the room behind"), "{restored}");
    assert!(!restored.contains("A familiar room"), "{restored}");
    assert!(restored.contains("The hall is quiet"), "{restored}");
    std::fs::remove_file(path).unwrap();
}
#[test]
fn real_cli_reentry_prepares_the_original_scene_again() {
    let out = stdout(&play(
        ":time 10\n@room.travel\n:present 5000\n:continue\n:time 100\n@hall.return\n:continue\n:quit\n",
        &[],
    ));
    assert_eq!(out.matches("A familiar room").count(), 2, "{out}");
    assert_eq!(out.matches("The hall is quiet").count(), 1, "{out}");
    assert!(!out.contains("Action rejected"), "{out}");
}
