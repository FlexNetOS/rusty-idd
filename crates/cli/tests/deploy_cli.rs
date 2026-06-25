//! Integration tests for `rusty-idd deploy` (ADR-0017): deploy the thin-adapter
//! control-plane surface into a target repo, idempotently and additively, with a
//! fail-closed `--check` drift gate. Exercised through the compiled binary.

use std::path::Path;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(bin())
        .args(args)
        .output()
        .expect("run rusty-idd")
}

fn ok(args: &[&str]) -> String {
    let out = run(args);
    assert!(
        out.status.success(),
        "command should succeed: {:?}\nstdout={}\nstderr={}",
        args,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn mkdirs(root: &Path, rels: &[&str]) {
    for r in rels {
        std::fs::create_dir_all(root.join(r)).unwrap();
    }
}

fn adapter(root: &Path, vendor_dir: &str) -> String {
    std::fs::read_to_string(root.join(vendor_dir).join("rusty-idd-adapter.md")).unwrap()
}

#[test]
fn deploy_writes_adapter_and_session_start_hook() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".claude", ".codex"]);

    ok(&[
        "deploy",
        "--target",
        target.path().to_str().unwrap(),
        "--all",
    ]);

    // Adapters written and point at the front door.
    for vd in [".claude", ".codex"] {
        let a = adapter(target.path(), vd);
        assert!(a.contains("rusty-idd next"), "{vd} adapter points at next");
        assert!(a.contains("THIN ADAPTER"), "{vd} adapter is thin");
    }

    // SessionStart hook installed in both hook-capable configs.
    let codex: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(target.path().join(".codex/hooks.json")).unwrap(),
    )
    .unwrap();
    let claude: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(target.path().join(".claude/settings.json")).unwrap(),
    )
    .unwrap();
    for cfg in [&codex, &claude] {
        let entries = cfg["hooks"]["SessionStart"].as_array().unwrap();
        let cmd = entries[0]["hooks"][0]["command"].as_str().unwrap();
        assert!(
            cmd.contains("rusty-idd next --base"),
            "hook calls front door: {cmd}"
        );
        assert!(
            cmd.contains("git rev-parse --show-toplevel"),
            "hook self-resolves root"
        );
        assert!(
            !cmd.contains("cargo run"),
            "deployed hook uses PATH binary, not cargo run"
        );
    }
}

#[test]
fn deploy_adapter_is_byte_identical_to_render() {
    // Deploy into one tree, render into another; the adapter bytes must match
    // (single source of truth).
    let dep = tempfile::tempdir().unwrap();
    let ren = tempfile::tempdir().unwrap();
    mkdirs(dep.path(), &[".claude"]);
    mkdirs(ren.path(), &[".claude"]);

    ok(&[
        "deploy",
        "--target",
        dep.path().to_str().unwrap(),
        "--vendor",
        "claude",
    ]);
    ok(&[
        "render",
        "--vendor",
        "claude",
        "--base",
        ren.path().to_str().unwrap(),
    ]);

    assert_eq!(
        adapter(dep.path(), ".claude"),
        adapter(ren.path(), ".claude"),
        "deploy and render must emit byte-identical adapters"
    );
}

#[test]
fn deploy_all_only_targets_existing_vendor_dirs() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".claude"]); // no .codex / .agents / .devin

    ok(&[
        "deploy",
        "--target",
        target.path().to_str().unwrap(),
        "--all",
    ]);

    assert!(target.path().join(".claude/rusty-idd-adapter.md").is_file());
    assert!(
        !target.path().join(".codex").exists(),
        "absent vendor dir not created"
    );
    assert!(!target.path().join(".agents").exists());
}

#[test]
fn deploy_preserves_existing_hooks_and_comment() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".claude"]);
    // Pre-existing settings with a comment and a PreToolUse hook.
    std::fs::write(
        target.path().join(".claude/settings.json"),
        r#"{
  "$comment": "keep me",
  "hooks": {
    "PreToolUse": [ { "matcher": "Bash", "hooks": [ { "type": "command", "command": "echo pre" } ] } ]
  }
}"#,
    )
    .unwrap();

    ok(&[
        "deploy",
        "--target",
        target.path().to_str().unwrap(),
        "--vendor",
        "claude",
    ]);

    let cfg: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(target.path().join(".claude/settings.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(cfg["$comment"], "keep me", "comment preserved");
    assert_eq!(
        cfg["hooks"]["PreToolUse"][0]["hooks"][0]["command"], "echo pre",
        "pre-existing PreToolUse hook preserved"
    );
    assert!(
        cfg["hooks"]["SessionStart"][0]["hooks"][0]["command"]
            .as_str()
            .unwrap()
            .contains("rusty-idd next"),
        "front-door SessionStart added"
    );
}

#[test]
fn deploy_is_idempotent_and_check_passes() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".claude", ".codex"]);
    let t = target.path().to_str().unwrap();

    ok(&["deploy", "--target", t, "--all"]);
    let claude_1 = std::fs::read_to_string(target.path().join(".claude/settings.json")).unwrap();
    let adapter_1 = adapter(target.path(), ".codex");

    // Second deploy changes nothing on disk.
    ok(&["deploy", "--target", t, "--all"]);
    let claude_2 = std::fs::read_to_string(target.path().join(".claude/settings.json")).unwrap();
    let adapter_2 = adapter(target.path(), ".codex");
    assert_eq!(
        claude_1, claude_2,
        "hook config byte-identical on re-deploy"
    );
    assert_eq!(adapter_1, adapter_2, "adapter byte-identical on re-deploy");

    // --check passes on the in-sync target (no writes).
    let out = run(&["deploy", "--target", t, "--all", "--check"]);
    assert!(out.status.success(), "check should pass on in-sync target");
}

#[test]
fn check_fails_on_missing_adapter() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".claude"]);
    let t = target.path().to_str().unwrap();
    // Never deployed -> adapter + hook missing.
    let out = run(&["deploy", "--target", t, "--vendor", "claude", "--check"]);
    assert_eq!(out.status.code(), Some(1), "check fails closed");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("out of sync"), "names the drift: {stderr}");
}

#[test]
fn check_fails_on_drifted_adapter_and_dry_run_is_alias() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".claude"]);
    let t = target.path().to_str().unwrap();
    ok(&["deploy", "--target", t, "--vendor", "claude"]);
    // Hand-edit the adapter away from the engine output.
    std::fs::write(
        target.path().join(".claude/rusty-idd-adapter.md"),
        "hand edited\n",
    )
    .unwrap();

    let out = run(&["deploy", "--target", t, "--vendor", "claude", "--dry-run"]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "--dry-run is an alias for --check and fails on drift"
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("drifted adapter"));
}

#[test]
fn check_fails_when_front_door_hook_absent() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".codex"]);
    let t = target.path().to_str().unwrap();
    ok(&["deploy", "--target", t, "--vendor", "codex"]);
    // Remove only the hook config, leave the adapter in place.
    std::fs::remove_file(target.path().join(".codex/hooks.json")).unwrap();

    let out = run(&["deploy", "--target", t, "--vendor", "codex", "--check"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("hook config"));
}

#[test]
fn deploy_does_not_touch_target_runtime_files() {
    let target = tempfile::tempdir().unwrap();
    mkdirs(target.path(), &[".claude", "src"]);
    // A sentinel "forge loop / runtime" file in the target.
    let sentinel = target.path().join("src/forge_loop.rs");
    std::fs::write(
        &sentinel,
        "// the target's own runtime — must be untouched\n",
    )
    .unwrap();
    let before = std::fs::read_to_string(&sentinel).unwrap();

    ok(&[
        "deploy",
        "--target",
        target.path().to_str().unwrap(),
        "--all",
    ]);

    assert_eq!(
        std::fs::read_to_string(&sentinel).unwrap(),
        before,
        "deploy is additive: the target's runtime file is untouched"
    );
}

#[test]
fn deploy_rejects_missing_target() {
    let out = run(&["deploy", "--target", "/no/such/target/xyz", "--all"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("target repo root not found"));
}
