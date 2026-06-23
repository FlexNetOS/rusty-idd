//! Integration tests for `rusty-idd next` — the harness control-plane front
//! door (ADR-0015), exercised through the compiled binary.

use std::path::Path;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

fn write(path: &Path, contents: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, contents).unwrap();
}

fn run_next(base: &Path) -> (i32, String, String) {
    let out = Command::new(bin())
        .args(["next", "--base"])
        .arg(base)
        .output()
        .expect("run rusty-idd next");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn next_with_no_active_change_guides_the_user() {
    let root = tempfile::tempdir().unwrap();
    let (code, out, _err) = run_next(root.path());
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("no active change"), "{out}");
    assert!(out.contains("rusty-idd spec new"), "{out}");
}

#[test]
fn next_routes_to_the_active_changes_imperative() {
    let root = tempfile::tempdir().unwrap();
    // active pointer -> a change that has only a proposal so far.
    write(
        &root.path().join(".idd/workflow/active-change"),
        "demo-change\n",
    );
    write(
        &root.path().join("openspec/changes/demo-change/proposal.md"),
        "# demo-change\n",
    );

    let (code, out, _err) = run_next(root.path());
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("Active change: demo-change"), "{out}");
    // the artifact-DAG oracle drives the imperative: proposal done -> specs next.
    assert!(out.contains("[x] proposal"), "{out}");
    assert!(out.contains("Next: specs"), "{out}");
    // and the scoped, token-cheap next action references the scaffold command.
    assert!(
        out.contains("rusty-idd spec scaffold specs --change demo-change"),
        "{out}"
    );
}

#[test]
fn next_errors_when_active_change_dir_is_missing() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root.path().join(".idd/workflow/active-change"),
        "ghost-change\n",
    );
    let (code, _out, err) = run_next(root.path());
    assert_eq!(code, 1);
    assert!(err.contains("has no directory"), "{err}");
}

fn run_next_json(base: &Path) -> (i32, String, String) {
    let out = Command::new(bin())
        .args(["next", "--json", "--base"])
        .arg(base)
        .output()
        .expect("run rusty-idd next --json");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn next_json_emits_fields_for_a_partial_change() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root.path().join(".idd/workflow/active-change"),
        "demo-change\n",
    );
    write(
        &root.path().join("openspec/changes/demo-change/proposal.md"),
        "# demo-change\n",
    );

    let (code, out, _err) = run_next_json(root.path());
    assert_eq!(code, 0, "{out}");
    let v: serde_json::Value = serde_json::from_str(&out).expect("valid JSON");
    assert_eq!(v["active_change"], "demo-change");
    assert_eq!(v["change"]["next"], "specs");
    assert_eq!(v["change"]["archivable"], false);
    assert_eq!(
        v["next_command"],
        "rusty-idd spec scaffold specs --change demo-change"
    );
}

#[test]
fn next_json_is_deterministic_across_runs() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root.path().join(".idd/workflow/active-change"),
        "demo-change\n",
    );
    write(
        &root.path().join("openspec/changes/demo-change/proposal.md"),
        "# demo-change\n",
    );
    let (_c1, a, _) = run_next_json(root.path());
    let (_c2, b, _) = run_next_json(root.path());
    assert_eq!(
        a, b,
        "next --json must be byte-identical over an unchanged tree"
    );
}

#[test]
fn next_json_fails_closed_on_dangling_pointer() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root.path().join(".idd/workflow/active-change"),
        "ghost-change\n",
    );
    let (code, out, err) = run_next_json(root.path());
    assert_eq!(code, 1);
    assert!(
        out.trim().is_empty(),
        "no stdout JSON on dangling pointer: {out}"
    );
    assert!(err.contains("has no directory"), "{err}");
}
