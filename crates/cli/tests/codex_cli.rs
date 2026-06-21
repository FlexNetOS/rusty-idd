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

fn write(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

const STOP_HOOK_JSON: &str = r#"{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "sh -lc 'root=\"$(git rev-parse --show-toplevel)\"; exec cargo run --quiet --manifest-path \"$root/Cargo.toml\" --bin rusty-idd -- codex env-check --workspace \"$root\"'",
            "timeout": 180,
            "statusMessage": "Checking Rusty IDD Codex invariants"
          }
        ]
      }
    ]
  }
}
"#;

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
        ".agents/skills/rusty-idd-knowledge/SKILL.md",
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
        "rusty-idd codex env-check\nrusty-idd codex model-loop\nrusty-idd codex runtime-audit\nrusty-idd codex system-audit\nCodex owns its output quality\nstale or orphaned work\nMissing binaries needed for this repo\n",
    );
    write(&root.path().join(".codex/hooks.json"), STOP_HOOK_JSON);

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

#[test]
fn codex_runtime_audit_reports_rust_native_runtime() {
    let root = tempfile::tempdir().unwrap();
    write(&root.path().join(".codex/hooks.json"), STOP_HOOK_JSON);
    write(
        &root.path().join(".codex/loops/rusty-idd-model-loop.toml"),
        "prompt = 'Use the Rust-native verifier.'\n",
    );
    write(
        &root.path().join(".codex/rules/default.rules"),
        r#"prefix_rule(pattern = ["pip", "install"], decision = "prompt")"#,
    );
    write(
        &root
            .path()
            .join("crates/external/codegraph-parser/Cargo.toml"),
        r#"tree-sitter-python = "0.23""#,
    );
    write(
        &root.path().join("crates/knowledge/src/lib.rs"),
        r#"fs::write(root.join("script.py"), "print('ok')\n").unwrap();"#,
    );

    let out = run_ok(&["codex", "runtime-audit", "--workspace", "."], root.path());
    assert!(out.contains("Live Codex Python commands: 0"));
    assert!(out.contains("Obsolete Python Codex tool files: 0"));
    assert!(out.contains("Parser/language support references: 1"));
    assert!(out.contains("Test fixture references: 1"));
    assert!(out.contains("Policy/documentation references: 1"));
    assert!(out.contains("repo-local Codex runtime is Rust-native"));
}

#[test]
fn codex_runtime_audit_fails_on_live_python_runtime() {
    let root = tempfile::tempdir().unwrap();
    write(
        &root.path().join(".codex/hooks.json"),
        r#"{"hooks":{"Stop":[{"hooks":[{"type":"command","command":"python3 .codex/hooks/check.py"}]}]}}"#,
    );

    let (stdout, stderr) = run_fail(&["codex", "runtime-audit", "--workspace", "."], root.path());
    assert!(stdout.contains("Live Codex Python commands: 1"));
    assert!(stdout.contains("python3 .codex/hooks/check.py"));
    assert!(stderr.contains("Codex runtime audit found 1 live Python runtime command"));
}

#[test]
fn codex_system_audit_classifies_active_rust_binary_and_upstream_python_tooling() {
    let root = tempfile::tempdir().unwrap();
    let codex_bin = root.path().join("codex-bin");
    fs::write(&codex_bin, b"\x7fELFfake-codex").unwrap();

    let codex_source = root.path().join("codex");
    write(&codex_source.join("codex-rs/Cargo.toml"), "[workspace]\n");
    write(
        &codex_source.join("justfile"),
        r#"set shell := ["python3", "-c", "print('shell')"]
fmt:
    python3 ../scripts/format.py
"#,
    );
    write(
        &codex_source.join("scripts/format.py"),
        "#!/usr/bin/env python3\nprint('fmt')\n",
    );
    write(
        &codex_source.join("codex-cli/scripts/build_npm_package.py"),
        "#!/usr/bin/env python3\nprint('package')\n",
    );
    write(
        &codex_source.join("sdk/python-runtime/README.md"),
        "Python wheel runtime\n",
    );

    let envctl = root.path().join("envctl");
    write(
        &envctl.join("manifest/ai-clis.toml"),
        r#"
[[component]]
id = "codex-cli"
script = '''
if [ -d "$M/codex/codex-rs/cli" ]; then
  jobs="${CODEX_CARGO_JOBS:-$(nproc --all)}"
  export CARGO_BUILD_JOBS="$jobs"
  export CARGO_PROFILE_RELEASE_LTO="${CARGO_PROFILE_RELEASE_LTO:-false}"
  export CARGO_PROFILE_RELEASE_CODEGEN_UNITS="${CARGO_PROFILE_RELEASE_CODEGEN_UNITS:-$jobs}"
  export CARGO_PROFILE_RELEASE_INCREMENTAL="${CARGO_PROFILE_RELEASE_INCREMENTAL:-true}"
  export RUSTFLAGS="${RUSTFLAGS:-} -C link-arg=-fuse-ld=mold"
  cargo build --release -p codex-cli --jobs "$jobs" --timings --manifest-path "$M/codex/codex-rs/Cargo.toml"
else
  bun install -g @openai/codex
fi
'''

[[component]]
id = "gemini-cli"
"#,
    );

    let out = run_ok(
        &[
            "codex",
            "system-audit",
            "--codex-bin",
            codex_bin.to_str().unwrap(),
            "--codex-source",
            codex_source.to_str().unwrap(),
            "--envctl",
            envctl.to_str().unwrap(),
        ],
        root.path(),
    );

    assert!(out.contains("binary kind: ELF native executable"));
    assert!(out.contains("active runtime verdict: Rust-native binary path"));
    assert!(out.contains("source Rust workspace present: true"));
    assert!(out.contains("codex source-build path: true"));
    assert!(out.contains("direct Cargo codex build: true"));
    assert!(out.contains("high-parallel Cargo jobs: true"));
    assert!(out.contains("mold linker path: true"));
    assert!(out.contains("Bun fallback only: true"));
    assert!(out.contains("Python in codex component: 0"));
}

#[test]
fn codex_system_audit_fails_when_active_codex_is_script() {
    let root = tempfile::tempdir().unwrap();
    let codex_bin = root.path().join("codex");
    fs::write(&codex_bin, b"#!/usr/bin/env python3\nprint('codex')\n").unwrap();

    let (stdout, stderr) = run_fail(
        &[
            "codex",
            "system-audit",
            "--codex-bin",
            codex_bin.to_str().unwrap(),
        ],
        root.path(),
    );
    assert!(stdout.contains("binary kind: script"));
    assert!(stdout.contains("active runtime verdict: not proven Rust-native"));
    assert!(stderr.contains("active codex binary is not a native ELF executable"));
}
