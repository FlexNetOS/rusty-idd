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

#[test]
fn merge_tools_show_exposes_canonical_package() {
    let root = tempfile::tempdir().unwrap();
    let out = run_ok(&["merge-tools", "show"], root.path());

    assert!(out.contains("Rusty IDD Merge Tool Package"));
    assert!(out.contains("rusty-idd spec status"));
    assert!(out.contains("AI_MERGE is evidence only"));
}

#[test]
fn merge_tools_legacy_lists_retired_bridge_surfaces() {
    let root = tempfile::tempdir().unwrap();
    let out = run_ok(&["merge-tools", "legacy"], root.path());

    assert!(out.contains(".claude/agents"));
    assert!(out.contains("retired active bridge material"));
}

#[test]
fn merge_tools_verify_passes_on_minimal_rust_workspace() {
    let root = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(root.path().join("crates/core/src")).unwrap();
    std::fs::write(root.path().join("crates/core/src/lib.rs"), "").unwrap();
    std::fs::write(
        root.path().join("crates/core/Cargo.toml"),
        "[package]\nname = \"rusty-idd-core\"\n\n[dependencies]\n",
    )
    .unwrap();

    let out = run_ok(
        &[
            "merge-tools",
            "verify",
            "--workspace",
            root.path().to_str().unwrap(),
        ],
        root.path(),
    );

    assert!(out.contains("Merge tools verification passed"));
}
