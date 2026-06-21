use std::fs;
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
fn knowledge_commands_cover_index_pack_report_query_and_refresh() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("src")).unwrap();
    fs::write(
        root.path().join("src/lib.rs"),
        "pub struct Widget;\npub fn build_widget() -> Widget { Widget }\n",
    )
    .unwrap();
    fs::write(root.path().join("notes.py"), "print('context')\n").unwrap();
    fs::write(
        root.path().join("ignored.ts"),
        "export const ignored = true;\n",
    )
    .unwrap();

    run_ok(
        &[
            "knowledge",
            "index",
            "--workspace",
            ".",
            "--out",
            "knowledge.json",
        ],
        root.path(),
    );
    assert!(root.path().join("knowledge.json").exists());

    let query = run_ok(
        &[
            "knowledge",
            "query",
            "--index",
            "knowledge.json",
            "--symbol",
            "build_widget",
        ],
        root.path(),
    );
    assert!(query.contains("build_widget"));

    run_ok(
        &[
            "knowledge",
            "pack",
            "--workspace",
            ".",
            "--out",
            "pack.md",
            "--style",
            "markdown",
            "--remove-empty-lines",
            "--truncate-base64",
            "--top-files-length",
            "5",
            "--ignore",
            "ignored.ts",
        ],
        root.path(),
    );
    let pack = fs::read_to_string(root.path().join("pack.md")).unwrap();
    assert!(pack.contains("notes.py"));
    assert!(!pack.contains("export const ignored"));

    run_ok(
        &[
            "knowledge",
            "report",
            "--workspace",
            ".",
            "--out",
            "report.md",
        ],
        root.path(),
    );
    assert!(fs::read_to_string(root.path().join("report.md"))
        .unwrap()
        .contains("# Knowledge Report"));

    run_ok(
        &[
            "knowledge",
            "architecture",
            "--workspace",
            ".",
            "--out",
            "architecture.json",
        ],
        root.path(),
    );
    let architecture = fs::read_to_string(root.path().join("architecture.json")).unwrap();
    assert!(architecture.contains("\"provider\": \"codegraph-rust\""));
    assert!(architecture.contains("\"provider\": \"repomix-rs\""));

    run_ok(&["knowledge", "refresh", "--workspace", "."], root.path());
    assert!(root.path().join(".idd/knowledge/index.json").exists());
    assert!(root.path().join(".idd/knowledge/report.md").exists());
    assert!(root
        .path()
        .join(".idd/knowledge/architecture.json")
        .exists());
    assert!(root.path().join(".idd/knowledge/architecture.md").exists());
}
