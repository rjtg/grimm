//! Exercise actual noncanonical terminal input, not only redirected lines.
#[cfg(unix)]
#[test]
fn terminal_keys_deadlines_commands_and_restoration() {
    let output = std::process::Command::new("python3")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/terminal_test.py"
        ))
        .arg(env!("CARGO_BIN_EXE_grimm-cli"))
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../stories/the-silent-signal.yaml"
        ))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(unix)]
#[test]
fn scene_tui_automatic_text_inspector_resize_menu_and_cleanup() {
    let output = std::process::Command::new("python3")
        .arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/scene_tui_test.py"
        ))
        .arg(env!("CARGO_BIN_EXE_grimm-cli"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
