use std::path::Path;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

fn run_ok(args: &[&str], cwd: &Path) -> String {
    let out = Command::new(bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run rusty-idd");
    assert!(
        out.status.success(),
        "command should succeed: {:?}\nstdout={}\nstderr={}",
        args,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn run_fail(args: &[&str], cwd: &Path) -> (String, String) {
    let out = Command::new(bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run rusty-idd");
    assert!(
        !out.status.success(),
        "command should fail: {:?}\nstdout={}\nstderr={}",
        args,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

#[test]
fn harness_scan_package_markdown_is_stage_scoped() {
    let root = tempfile::tempdir().unwrap();
    let out = run_ok(
        &[
            "harness", "package", "--stage", "scan", "--target", ".", "--format", "markdown",
        ],
        root.path(),
    );

    assert!(out.contains("# scan-stage scoped Rust agent swarm package"));
    assert!(out.contains("- Stage: `scan`"));
    assert!(out.contains("## Agent Team"));
    assert!(out.contains("scan-orchestrator"));
    assert!(out.contains("## Contracts"));
    assert!(out.contains("adapter-boundary-contract"));
    assert!(out.contains("## Tools"));
    assert!(out.contains("rusty-idd scan"));
    assert!(out.contains("no-default-mcp-contract"));
    assert!(!out.contains("mcp server"));
}

#[test]
fn harness_scan_package_json_declares_evidence_schema_and_adapters() {
    let root = tempfile::tempdir().unwrap();
    let out = run_ok(
        &[
            "harness", "package", "--stage", "scan", "--target", ".", "--format", "json",
        ],
        root.path(),
    );
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();

    assert_eq!(json["stage"], "scan");
    assert_eq!(json["evidence_schema"][0]["name"], "inventory");
    assert!(json["adapter_boundary"]
        .as_array()
        .unwrap()
        .iter()
        .any(|entry| entry["name"] == ".codex"));
    assert!(json["tools"]
        .as_array()
        .unwrap()
        .iter()
        .all(|entry| !entry["name"].as_str().unwrap().contains("mcp")));
}

#[test]
fn harness_package_rejects_missing_target() {
    let root = tempfile::tempdir().unwrap();
    let (_stdout, stderr) = run_fail(
        &[
            "harness", "package", "--stage", "scan", "--target", "missing",
        ],
        root.path(),
    );

    assert!(stderr.contains("package target does not exist"));
}
