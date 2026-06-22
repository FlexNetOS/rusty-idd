use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

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

fn run_fail_with_stdin(args: &[&str], cwd: &Path, stdin: &str) -> (String, String) {
    let mut child = Command::new(bin())
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run rusty-idd");
    child
        .stdin
        .as_mut()
        .expect("open stdin")
        .write_all(stdin.as_bytes())
        .expect("write stdin");
    let out = child.wait_with_output().expect("wait for rusty-idd");
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

fn git_ok(cwd: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run git");
    assert!(
        out.status.success(),
        "git {:?} should succeed\nstdout={}\nstderr={}",
        args,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

const STOP_HOOK_JSON: &str = r#"{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash|apply_patch",
        "hooks": [
          {
            "type": "command",
            "command": "sh -lc 'root=\"$(git rev-parse --show-toplevel)\"; exec cargo run --quiet --manifest-path \"$root/Cargo.toml\" --bin rusty-idd -- codex workflow-check --workspace \"$root\" --phase pre-tool'",
            "timeout": 180,
            "statusMessage": "Checking Rusty IDD workflow before tool use"
          }
        ]
      }
    ],
    "PostToolUse": [
      {
        "matcher": "Bash|apply_patch",
        "hooks": [
          {
            "type": "command",
            "command": "sh -lc 'root=\"$(git rev-parse --show-toplevel)\"; exec cargo run --quiet --manifest-path \"$root/Cargo.toml\" --bin rusty-idd -- codex workflow-check --workspace \"$root\" --phase post-tool'",
            "timeout": 180,
            "statusMessage": "Checking Rusty IDD workflow after tool use"
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "sh -lc 'root=\"$(git rev-parse --show-toplevel)\"; exec cargo run --quiet --manifest-path \"$root/Cargo.toml\" --bin rusty-idd -- codex workflow-check --workspace \"$root\" --phase stop'",
            "timeout": 180,
            "statusMessage": "Checking Rusty IDD workflow handoff"
          }
        ]
      },
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
    ],
    "SubagentStop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": "sh -lc 'root=\"$(git rev-parse --show-toplevel)\"; exec cargo run --quiet --manifest-path \"$root/Cargo.toml\" --bin rusty-idd -- codex workflow-check --workspace \"$root\" --phase stop'",
            "timeout": 180,
            "statusMessage": "Checking Rusty IDD subagent workflow handoff"
          }
        ]
      }
    ]
  }
}
"#;

fn setup_workflow_repo() -> tempfile::TempDir {
    let root = tempfile::tempdir().unwrap();
    git_ok(root.path(), &["init"]);
    git_ok(root.path(), &["config", "user.email", "codex@example.com"]);
    git_ok(root.path(), &["config", "user.name", "Codex Test"]);
    write(&root.path().join("README.md"), "seed\n");
    git_ok(root.path(), &["add", "README.md"]);
    git_ok(root.path(), &["commit", "-m", "seed"]);
    git_ok(root.path(), &["switch", "-c", "develop"]);
    git_ok(root.path(), &["switch", "-c", "feature/workflow-hooks"]);

    write(
        &root.path().join(".idd/knowledge/plan-context.md"),
        "# Plan Context\n",
    );
    write(
        &root.path().join(".idd/workflow/active-change"),
        "add-autonomous-workflow-hooks\n",
    );
    write(
        &root
            .path()
            .join(".idd/evidence/autonomous-workflow/task.md"),
        "Task: KBTASK-RUSTY-IDD-AUTONOMOUS-WORKFLOW-HOOKS\nclaim: hf claim KBTASK-RUSTY-IDD-AUTONOMOUS-WORKFLOW-HOOKS\n",
    );
    write(
        &root
            .path()
            .join("openspec/changes/add-autonomous-workflow-hooks/proposal.md"),
        "# proposal\n",
    );
    write(
        &root
            .path()
            .join("openspec/changes/add-autonomous-workflow-hooks/design.md"),
        "# design\n",
    );
    write(
        &root
            .path()
            .join("openspec/changes/add-autonomous-workflow-hooks/tasks.md"),
        "- [ ] task\n",
    );
    write(
        &root.path().join(
            "openspec/changes/add-autonomous-workflow-hooks/specs/codex-harness-flow/spec.md",
        ),
        "## ADDED Requirements\n\n### Requirement: Test\nThe system SHALL work.\n\n#### Scenario: Works\n- **WHEN** checked\n- **THEN** it passes\n",
    );
    write(
        &root.path().join("adr/0001-test.md"),
        "# 0001. Test\n\n- Status: accepted\n",
    );
    root
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
        ".agents/skills/rusty-idd-knowledge/SKILL.md",
        ".idd/knowledge/index.json",
        ".idd/knowledge/report.md",
        ".idd/knowledge/architecture.json",
        ".idd/knowledge/architecture.md",
        ".idd/knowledge/system-architecture.json",
        ".idd/knowledge/system-architecture.md",
        ".idd/knowledge/operating-model.json",
        ".idd/knowledge/operating-model.md",
        ".idd/knowledge/integration-plan.json",
        ".idd/knowledge/integration-plan.md",
        ".idd/knowledge/integration-status.json",
        ".idd/knowledge/integration-status.md",
        ".idd/knowledge/plan-context.json",
        ".idd/knowledge/plan-context.md",
        "docs/rusty-idd/codex-environment.md",
        "docs/rusty-idd/merge-tools-package.md",
        "adr/0001-codex-harness-rusty-idd-flow.md",
        "crates/merge-tools/Cargo.toml",
        "crates/merge-tools/src/lib.rs",
        "openspec/changes/upgrade-codex-harness-rusty-idd-flow/proposal.md",
        "openspec/changes/upgrade-codex-harness-rusty-idd-flow/specs/codex-harness-flow/spec.md",
        "openspec/changes/upgrade-codex-harness-rusty-idd-flow/specs/merge-tool-package/spec.md",
        "openspec/changes/upgrade-codex-harness-rusty-idd-flow/design.md",
        "openspec/changes/upgrade-codex-harness-rusty-idd-flow/tasks.md",
        "third_party/upstream/UPSTREAMS.md",
        "third_party/upstream/codegraph-rust/Cargo.toml",
        "third_party/upstream/repomix-rs/Cargo.toml",
    ] {
        write(&root.path().join(path), "");
    }
    write(
        &root.path().join("AGENTS.md"),
        "Rusty IDD is the intent-driven workflow engine\n`AI_MERGE/` is a Rusty IDD tool and evidence surface\nBefore writes, create or select an OpenSpec change\nAdopt first, cut after evidence\nUpgrade only\nTreat stale or orphaned work as unfinished\nTooling required to run this repo must be tracked\nHost service and process management is out of scope\n",
    );
    write(
        &root.path().join("docs/rusty-idd/codex-environment.md"),
        "`AI_MERGE/` is a tool/evidence surface\nThe default harness order is\n`rusty-idd merge-tools show`\nWrite-capable implementation is intentionally outside the default loop\nUpgrade-Only Gap Handling\nmeta` / `envctl`\nMulti-Model Loop\nAutonomous Workflow Hooks\ncodex workflow-check\nenvctl\ntoolchain\n.codex/rules\n",
    );
    write(
        &root.path().join("docs/rusty-idd/merge-tools-package.md"),
        "Rusty IDD Merge Tool Package\nDeprecated merge content scan\nActive bridge rule\nsingle active ADR\n",
    );
    write(
        &root.path().join("adr/0001-codex-harness-rusty-idd-flow.md"),
        "Codex harness follows Rusty IDD flow\nsingle active ADR\nmerge-tools package\n",
    );
    write(
        &root.path().join(".codex/loops/rusty-idd-model-loop.toml"),
        r#"
name = "design-first"
description = ".idd/knowledge/plan-context.md OpenSpec Treat AI_MERGE as evidence"

[[passes]]
name = "verify"
agent = "rusty-idd-verifier"
model = "gpt-5.5"
reasoning = "high"
sandbox = "read-only"
prompt = "OpenSpec"
"#,
    );
    write(
        &root.path().join(".codex/agents/rusty-idd-implementer.toml"),
        r#"
name = "rusty-idd-implementer"
description = "Workspace-write implementer."
developer_instructions = "Before editing, verify the active OpenSpec change. Update AI_MERGE only when the workflow calls for evidence."
"#,
    );
    write(
        &root.path().join("third_party/upstream/UPSTREAMS.md"),
        "Jakedismo/codegraph-rust\nsopaco/repomix-rs\nce5bf27a2978983a9089d177447f296e4c6521bb\n946df10d48c669ca3a99f757ffd2c6fa35844e62\n",
    );
    write(&root.path().join(".codex/hooks.json"), STOP_HOOK_JSON);

    let out = run_ok(&["codex", "env-check", "--workspace", "."], root.path());
    assert!(out.contains("invariant check passed"));
}

#[test]
fn codex_workflow_check_passes_for_ready_feature_worktree() {
    let root = setup_workflow_repo();

    let out = run_ok(
        &[
            "codex",
            "workflow-check",
            "--phase",
            "pre-tool",
            "--workspace",
            ".",
        ],
        root.path(),
    );

    assert!(out.contains("autonomous workflow check passed"));
}

#[test]
fn codex_workflow_check_rejects_missing_openspec_readiness() {
    let root = setup_workflow_repo();
    fs::remove_file(
        root.path()
            .join("openspec/changes/add-autonomous-workflow-hooks/tasks.md"),
    )
    .unwrap();

    let (_stdout, stderr) = run_fail(
        &[
            "codex",
            "workflow-check",
            "--phase",
            "pre-tool",
            "--workspace",
            ".",
        ],
        root.path(),
    );

    assert!(stderr.contains("missing OpenSpec tasks"));
}

#[test]
fn codex_workflow_check_stop_requires_delivery_evidence_for_dirty_work() {
    let root = setup_workflow_repo();
    write(&root.path().join("src/lib.rs"), "pub fn touched() {}\n");

    let (_stdout, stderr) = run_fail(
        &[
            "codex",
            "workflow-check",
            "--phase",
            "stop",
            "--workspace",
            ".",
        ],
        root.path(),
    );

    assert!(stderr.contains("missing validation evidence"));
    assert!(stderr.contains("missing PR/automerge evidence"));

    write(
        &root
            .path()
            .join(".idd/evidence/autonomous-workflow/validation.md"),
        "Build: pass\nGenerated artifacts: refreshed\nTest: pass\nLint: pass\nSecret scan: pass\nManifest: refreshed\n",
    );
    write(
        &root.path().join(".idd/evidence/autonomous-workflow/pr.md"),
        "PR: #123\nBase: develop\nauto-merge: enabled\n",
    );

    let out = run_ok(
        &[
            "codex",
            "workflow-check",
            "--phase",
            "stop",
            "--workspace",
            ".",
        ],
        root.path(),
    );
    assert!(out.contains("autonomous workflow check passed"));
}

#[test]
fn codex_workflow_check_rejects_test_before_generated_artifacts() {
    let root = setup_workflow_repo();
    write(&root.path().join("src/lib.rs"), "pub fn touched() {}\n");
    write(
        &root
            .path()
            .join(".idd/evidence/autonomous-workflow/validation.md"),
        "Build: pass\nTest: pass\nGenerated artifacts: refreshed\nLint: pass\nSecret scan: pass\nManifest: refreshed\n",
    );
    write(
        &root.path().join(".idd/evidence/autonomous-workflow/pr.md"),
        "PR: #123\nBase: develop\nauto-merge: enabled\n",
    );

    let (_stdout, stderr) = run_fail(
        &[
            "codex",
            "workflow-check",
            "--phase",
            "stop",
            "--workspace",
            ".",
        ],
        root.path(),
    );

    assert!(stderr.contains("Test after Generated artifacts"));
}

#[test]
fn codex_workflow_check_requires_validation_before_push() {
    let root = setup_workflow_repo();
    let hook_input =
        r#"{"tool_name":"Bash","tool_input":{"command":"git push origin feature/workflow-hooks"}}"#;

    let (_stdout, stderr) = run_fail_with_stdin(
        &[
            "codex",
            "workflow-check",
            "--phase",
            "pre-tool",
            "--workspace",
            ".",
        ],
        root.path(),
        hook_input,
    );

    assert!(stderr.contains("missing validation evidence"));
}

#[test]
fn codex_workflow_check_requires_validation_before_task_completion() {
    let root = setup_workflow_repo();
    let hook_input = r#"{"tool_name":"Bash","tool_input":{"command":"hf done --pr 123 KBTASK-RUSTY-IDD-AUTONOMOUS-WORKFLOW-HOOKS"}}"#;

    let (_stdout, stderr) = run_fail_with_stdin(
        &[
            "codex",
            "workflow-check",
            "--phase",
            "pre-tool",
            "--workspace",
            ".",
        ],
        root.path(),
        hook_input,
    );

    assert!(stderr.contains("missing validation evidence"));
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
