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

    run_ok(
        &[
            "knowledge",
            "plan-context",
            "--workspace",
            ".",
            "--out",
            "plan-context.md",
            "--change",
            "demo-change",
            "--goal",
            "Use CodeGraph and repomix for planning",
            "--architecture",
            "architecture.json",
        ],
        root.path(),
    );
    let plan_context = fs::read_to_string(root.path().join("plan-context.md")).unwrap();
    assert!(plan_context.contains("# Graph Planning Context"));
    assert!(plan_context.contains("demo-change"));
    assert!(plan_context.contains("CodeGraph Rust"));

    run_ok(&["knowledge", "refresh", "--workspace", "."], root.path());
    assert!(root.path().join(".idd/knowledge/index.json").exists());
    assert!(root.path().join(".idd/knowledge/report.md").exists());
    assert!(root
        .path()
        .join(".idd/knowledge/architecture.json")
        .exists());
    assert!(root.path().join(".idd/knowledge/architecture.md").exists());
}

#[test]
fn system_architecture_cli_discovers_peer_repos_without_meta() {
    let system = tempfile::tempdir().unwrap();
    let rusty = system.path().join("rusty-idd");
    let handoff = system.path().join("handoff");
    fs::create_dir_all(&rusty).unwrap();
    fs::create_dir_all(&handoff).unwrap();

    run_git(&["init"], &rusty);
    run_git(&["init"], &handoff);
    fs::write(
        rusty.join("Cargo.toml"),
        "[package]\nname = \"rusty-idd\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();
    fs::create_dir_all(handoff.join(".handoff")).unwrap();
    fs::create_dir_all(handoff.join(".idd/knowledge")).unwrap();
    fs::create_dir_all(handoff.join("src")).unwrap();
    fs::write(
        handoff.join("src/lib.rs"),
        "pub struct FleetHandoff;\npub fn sync() -> FleetHandoff { FleetHandoff }\n",
    )
    .unwrap();

    run_ok(
        &[
            "knowledge",
            "architecture",
            "--workspace",
            "../handoff",
            "--out",
            "../handoff/.idd/knowledge/architecture.json",
        ],
        &rusty,
    );

    run_ok(
        &[
            "knowledge",
            "system-architecture",
            "--workspace",
            ".",
            "--system-root",
            "..",
            "--out",
            "system-architecture.json",
        ],
        &rusty,
    );
    let graph = fs::read_to_string(rusty.join("system-architecture.json")).unwrap();
    assert!(graph.contains("\"discovery_source\": \"filesystem git discovery\""));
    assert!(graph.contains("\"name\": \"rusty-idd\""));
    assert!(graph.contains("\"name\": \"handoff\""));
    assert!(graph.contains("role:idd-control-plane"));
    assert!(graph.contains("role:fleet-handoff"));
    assert!(graph.contains("\"local_architecture\""));
    assert!(graph.contains("\"top_components\""));

    run_ok(
        &[
            "knowledge",
            "operating-model",
            "--workspace",
            ".",
            "--system-architecture",
            "system-architecture.json",
            "--out",
            "operating-model.json",
        ],
        &rusty,
    );
    let model = fs::read_to_string(rusty.join("operating-model.json")).unwrap();
    assert!(model.contains("\"capability:idd-spec-engine\""));
    assert!(model.contains("\"capability:fleet-handoff\""));
    assert!(model.contains("\"capability:agent-communication\""));

    run_ok(
        &[
            "knowledge",
            "integration-plan",
            "--workspace",
            ".",
            "--operating-model",
            "operating-model.json",
            "--out",
            "integration-plan.json",
        ],
        &rusty,
    );
    let plan = fs::read_to_string(rusty.join("integration-plan.json")).unwrap();
    assert!(plan.contains("\"work_items\""));
    assert!(plan.contains("\"integrate-idd-spec-engine\""));

    run_ok(
        &[
            "knowledge",
            "integration-status",
            "--workspace",
            ".",
            "--integration-plan",
            "integration-plan.json",
            "--out",
            "integration-status.json",
        ],
        &rusty,
    );
    let status = fs::read_to_string(rusty.join("integration-status.json")).unwrap();
    assert!(status.contains("\"next_change_id\""));
    assert!(status.contains("\"integrate-idd-spec-engine\""));
    assert!(status.contains("\"planned\""));

    run_ok(
        &[
            "knowledge",
            "integration-owners",
            "--workspace",
            ".",
            "--integration-plan",
            "integration-plan.json",
            "--system-architecture",
            "system-architecture.json",
            "--change",
            "integrate-fleet-handoff",
            "--out",
            "integration-owners.json",
        ],
        &rusty,
    );
    let owners = fs::read_to_string(rusty.join("integration-owners.json")).unwrap();
    assert!(owners.contains("\"change_id\": \"integrate-fleet-handoff\""));
    assert!(owners.contains("\"owner_surfaces\""));
    assert!(owners.contains("\"repo:handoff\""));

    run_ok(
        &[
            "knowledge",
            "integration-owners",
            "--workspace",
            ".",
            "--integration-plan",
            "integration-plan.json",
            "--system-architecture",
            "system-architecture.json",
            "--next",
            "--out",
            "integration-owners-queue-head.json",
        ],
        &rusty,
    );
    let queue_head = fs::read_to_string(rusty.join("integration-owners-queue-head.json")).unwrap();
    assert!(queue_head.contains("\"next\": true"));
    assert!(queue_head.contains("\"change_id\": \"integrate-idd-spec-engine\""));

    run_ok(
        &[
            "knowledge",
            "integration-owners",
            "--workspace",
            ".",
            "--integration-plan",
            "integration-plan.json",
            "--system-architecture",
            "system-architecture.json",
            "--next-planned",
            "--out",
            "integration-owners-next.json",
        ],
        &rusty,
    );
    let next_owners = fs::read_to_string(rusty.join("integration-owners-next.json")).unwrap();
    assert!(next_owners.contains("\"next_planned\": true"));
    assert!(next_owners.contains("\"change_id\": \"integrate-idd-spec-engine\""));

    run_ok(
        &[
            "knowledge",
            "integration-readiness",
            "--workspace",
            ".",
            "--integration-plan",
            "integration-plan.json",
            "--system-architecture",
            "system-architecture.json",
            "--change",
            "integrate-fleet-handoff",
            "--out",
            "integration-readiness.json",
        ],
        &rusty,
    );
    let readiness = fs::read_to_string(rusty.join("integration-readiness.json")).unwrap();
    assert!(readiness.contains("\"change_id\": \"integrate-fleet-handoff\""));
    assert!(readiness.contains("\"tool_requirements\""));
    assert!(readiness.contains("\"native_diagnostics\""));
}

fn run_git(args: &[&str], cwd: &Path) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git command should succeed: {:?}\nstdout={}\nstderr={}",
        args,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
