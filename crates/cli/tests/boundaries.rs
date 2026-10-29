//! Inspect Cargo's own dependency model, including dev/build/optional/target edges.
use std::collections::BTreeSet;
use std::process::Command;

use serde_json::Value;

const PACKAGES: &[&str] = &[
    "grimm-semantic",
    "grimm-presentation",
    "grimm-engine",
    "grimm-adapters",
    "grimm-cli-renderer",
    "grimm-cli",
];

fn allowed(package: &str, dependency: &str, kind: &str) -> bool {
    match (package, kind) {
        ("grimm-semantic" | "grimm-presentation", "normal") => dependency == "serde",
        ("grimm-semantic" | "grimm-presentation", "dev") => dependency == "serde_json",
        ("grimm-engine", "normal") => dependency == "grimm-semantic",
        ("grimm-adapters", "normal") => {
            matches!(dependency, "grimm-semantic" | "grimm-presentation")
        }
        ("grimm-cli-renderer", "normal") => dependency == "grimm-presentation",
        ("grimm-cli", "normal") => matches!(
            dependency,
            "grimm-engine"
                | "grimm-adapters"
                | "grimm-cli-renderer"
                | "grimm-presentation"
                | "grimm-semantic"
                | "serde_json"
                | "serde_yaml_ng"
        ),
        _ => false,
    }
}

#[test]
fn cargo_dependencies_preserve_architecture() {
    let output = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--locked",
            "--offline",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: Value = serde_json::from_slice(&output.stdout).unwrap();
    let packages = metadata["packages"].as_array().unwrap();
    let names: BTreeSet<_> = packages
        .iter()
        .map(|package| package["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        PACKAGES.iter().copied().collect(),
        "New workspace crates require a boundary policy."
    );
    for package in packages {
        let name = package["name"].as_str().unwrap();
        for dependency in package["dependencies"].as_array().unwrap() {
            let target = dependency["name"].as_str().unwrap();
            let kind = dependency["kind"].as_str().unwrap_or("normal");
            assert!(
                allowed(name, target, kind),
                "Forbidden {kind} dependency: {name} -> {target}"
            );
            assert!(
                dependency["rename"].is_null(),
                "Dependency aliases require boundary review"
            );
            if PACKAGES.contains(&target) {
                let target_package = packages
                    .iter()
                    .find(|package| package["name"] == target)
                    .unwrap();
                let manifest =
                    std::path::Path::new(target_package["manifest_path"].as_str().unwrap());
                assert_eq!(
                    dependency["path"].as_str().map(std::path::Path::new),
                    manifest.parent(),
                    "Internal edges must resolve to workspace crates"
                );
            }
        }
    }
}

#[test]
fn policy_rejects_reverse_edges_and_external_engine_dependencies() {
    for kind in ["normal", "dev", "build"] {
        for dependency in [
            "grimm-adapters",
            "grimm-presentation",
            "grimm-cli-renderer",
            "bevy",
            "reqwest",
            "serde_json",
            "serde_yaml_ng",
        ] {
            assert!(!allowed("grimm-engine", dependency, kind));
        }
        assert!(!allowed("grimm-cli-renderer", "grimm-semantic", kind));
        assert!(!allowed("grimm-adapters", "grimm-engine", kind));
        assert!(!allowed("grimm-semantic", "grimm-engine", kind));
    }
}
