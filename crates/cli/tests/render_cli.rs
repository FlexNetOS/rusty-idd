//! Integration tests for `rusty-idd render` — vendor-adapter generation and the
//! fail-closed drift gate (ADR-0010/0015), via the compiled binary.

use std::path::Path;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

fn run(args: &[&str], base: &Path) -> (i32, String, String) {
    let out = Command::new(bin())
        .args(args)
        .args(["--base"])
        .arg(base)
        .output()
        .expect("run rusty-idd render");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Create the known vendor dirs in a temp base.
fn base_with_vendors() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    for d in [".claude", ".codex", ".agents", ".devin"] {
        std::fs::create_dir_all(root.path().join(d)).unwrap();
    }
    root
}

#[test]
fn render_all_then_check_passes_and_is_deterministic() {
    let root = base_with_vendors();
    let (code, _o, _e) = run(&["render", "--all"], root.path());
    assert_eq!(code, 0);

    let p = root.path().join(".claude/rusty-idd-adapter.md");
    let first = std::fs::read_to_string(&p).unwrap();
    assert!(first.contains("rusty-idd next"));

    // re-render: byte-identical (deterministic, idempotent)
    run(&["render", "--all"], root.path());
    assert_eq!(first, std::fs::read_to_string(&p).unwrap());

    // the gate passes on freshly-rendered adapters
    let (code, out, _e) = run(&["render", "--all", "--check"], root.path());
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("in sync"), "{out}");
}

#[test]
fn check_fails_closed_on_hand_edit() {
    let root = base_with_vendors();
    run(&["render", "--all"], root.path());
    // tamper with one adapter
    std::fs::write(
        root.path().join(".codex/rusty-idd-adapter.md"),
        "hand-edited workflow prose\n",
    )
    .unwrap();

    let (code, _o, err) = run(&["render", "--all", "--check"], root.path());
    assert_eq!(code, 1);
    assert!(err.contains("drifted"), "{err}");
    assert!(err.contains(".codex"), "{err}");
}

#[test]
fn check_fails_closed_on_missing_adapter() {
    let root = base_with_vendors();
    // render only claude, then check all -> the others are missing
    run(&["render", "--vendor", "claude"], root.path());
    let (code, _o, err) = run(&["render", "--all", "--check"], root.path());
    assert_eq!(code, 1);
    assert!(err.contains("missing"), "{err}");
}

#[test]
fn unknown_vendor_is_rejected() {
    let root = base_with_vendors();
    let (code, _o, err) = run(&["render", "--vendor", "bogus"], root.path());
    assert_eq!(code, 2);
    assert!(err.contains("unknown vendor"), "{err}");
}
