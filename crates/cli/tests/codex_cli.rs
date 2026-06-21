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

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

#[test]
fn codex_env_check_passes_on_required_repo_artifacts() {
    let root = tempfile::tempdir().unwrap();
    for path in [
        "AGENTS.md",
        ".codex/config.toml",
        ".codex/hooks.json",
        ".codex/rules/default.rules",
        ".codex/agents/rusty-idd-explorer.toml",
        ".codex/agents/rusty-idd-implementer.toml",
        ".codex/agents/rusty-idd-verifier.toml",
        ".codex/agents/rusty-idd-gap-hunter.toml",
        ".codex/loops/rusty-idd-model-loop.toml",
        ".agents/skills/rusty-idd-adopt-first/SKILL.md",
        ".agents/skills/rusty-idd-codex-rust-env/SKILL.md",
        ".idd/knowledge/index.json",
        ".idd/knowledge/report.md",
        "docs/rusty-idd/codex-environment.md",
        "adr/0004-knowledge-direct-crate-integration.md",
        "AI_MERGE/12_knowledge_deep_audit.md",
        "AI_MERGE/13_codex_environment.md",
    ] {
        write(&root.path().join(path), "");
    }
    write(
        &root.path().join("AGENTS.md"),
        "Adopt first, cut after evidence\nThe agent owns output quality\nUpgrade only\nTreat stale or orphaned work as unfinished\nTooling required to run this repo must be tracked\nHost service and process management is out of scope\n",
    );
    write(
        &root
            .path()
            .join("adr/0004-knowledge-direct-crate-integration.md"),
        "adopt the upstream parser/core\nCut audit-denied\n",
    );
    write(
        &root.path().join("AI_MERGE/12_knowledge_deep_audit.md"),
        "Audit Cuts\nlocal Rust AST semantic pass were removed\n",
    );
    write(
        &root.path().join("docs/rusty-idd/codex-environment.md"),
        "Agent-Owned Tool Growth\nUpgrade-Only Gap Handling\nmeta` / `envctl`\nMulti-Model Loop\nenvctl\ntoolchain\n.codex/rules\n",
    );
    write(
        &root.path().join("AI_MERGE/13_codex_environment.md"),
        "rusty-idd codex env-check\nrusty-idd codex model-loop\nCodex owns its output quality\nstale or orphaned work\nMissing binaries needed for this repo\n",
    );
    write(&root.path().join(".codex/hooks.json"), "{}\n");

    let out = run_ok(&["codex", "env-check", "--workspace", "."], root.path());
    assert!(out.contains("invariant check passed"));
}

#[test]
fn codex_model_loop_dry_run_emits_codex_exec_commands() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root.path().join("loop.toml"),
        r#"
name = "test-loop"
output_dir = "/tmp/rusty-idd-test-loop"

[[passes]]
name = "explore"
agent = "rusty-idd-explorer"
model = "gpt-5.4-mini"
reasoning = "medium"
sandbox = "read-only"
prompt = "Inspect without editing."

[[passes]]
name = "implement"
agent = "rusty-idd-implementer"
model = "gpt-5.5"
reasoning = "high"
sandbox = "workspace-write"
prompt = "Implement one narrow change."
"#,
    );
    write(
        &root.path().join(".codex/agents/rusty-idd-explorer.toml"),
        r#"
name = "rusty-idd-explorer"
description = "Read-only test explorer."
developer_instructions = "Explore without editing."
"#,
    );
    write(
        &root.path().join(".codex/agents/rusty-idd-implementer.toml"),
        r#"
name = "rusty-idd-implementer"
description = "Workspace-write test implementer."
developer_instructions = "Implement narrowly."
"#,
    );
    let out_dir = root.path().join("out");
    let out = run_ok(
        &[
            "codex",
            "model-loop",
            "--config",
            "loop.toml",
            "--output-dir",
            out_dir.to_str().unwrap(),
        ],
        root.path(),
    );

    let lines = out.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains("\"name\":\"explore\""));
    assert!(lines[0].contains("\"codex\",\"exec\""));
    assert!(lines[0].contains("Explore without editing."));
    assert!(lines[1].contains("\"sandbox\":\"workspace-write\""));
    assert!(lines[2].contains("\"manifest\""));
}
