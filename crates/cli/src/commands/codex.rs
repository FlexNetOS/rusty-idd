use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{bail, Context};
use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Subcommand)]
pub enum CodexCommand {
    /// Check repo-local Codex environment invariants.
    EnvCheck(EnvCheckArgs),
    /// Emit or execute the dry-run-first Codex model loop.
    ModelLoop(ModelLoopArgs),
    /// Audit whether repo-local Codex runtime paths depend on Python.
    RuntimeAudit(RuntimeAuditArgs),
    /// Audit the active Codex install and parent-managed source-build path.
    SystemAudit(SystemAuditArgs),
}

#[derive(Args)]
pub struct EnvCheckArgs {
    #[arg(long, default_value = ".")]
    pub workspace: PathBuf,
}

#[derive(Args)]
pub struct ModelLoopArgs {
    #[arg(long, default_value = ".codex/loops/rusty-idd-model-loop.toml")]
    pub config: PathBuf,
    #[arg(long)]
    pub execute: bool,
    #[arg(long = "only")]
    pub only: Vec<String>,
    #[arg(long)]
    pub output_dir: Option<PathBuf>,
}

#[derive(Args)]
pub struct RuntimeAuditArgs {
    #[arg(long, default_value = ".")]
    pub workspace: PathBuf,
}

#[derive(Args)]
pub struct SystemAuditArgs {
    #[arg(long)]
    pub codex_bin: Option<PathBuf>,
    #[arg(long)]
    pub codex_source: Option<PathBuf>,
    #[arg(long)]
    pub envctl: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct LoopConfig {
    name: Option<String>,
    output_dir: Option<PathBuf>,
    passes: Vec<ModelPass>,
}

#[derive(Debug, Deserialize)]
struct ModelPass {
    name: String,
    agent: String,
    model: String,
    reasoning: String,
    sandbox: String,
    prompt: String,
}

#[derive(Debug, Deserialize)]
struct AgentConfig {
    name: Option<String>,
    description: Option<String>,
    developer_instructions: Option<String>,
}

#[derive(Debug, Serialize)]
struct LoopEntry {
    name: String,
    agent: String,
    model: String,
    reasoning: String,
    sandbox: String,
    output: String,
    command: Vec<String>,
    shell: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    exit_code: Option<i32>,
}

#[derive(Debug, Serialize)]
struct LoopManifest {
    loop_name: String,
    config: String,
    run_id: String,
    execute: bool,
    passes: Vec<LoopEntry>,
}

pub fn run(command: CodexCommand) -> i32 {
    match try_run(command) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("rusty-idd codex: {error:#}");
            1
        }
    }
}

fn try_run(command: CodexCommand) -> anyhow::Result<()> {
    match command {
        CodexCommand::EnvCheck(args) => env_check(&args.workspace),
        CodexCommand::ModelLoop(args) => model_loop(args),
        CodexCommand::RuntimeAudit(args) => runtime_audit(&args.workspace),
        CodexCommand::SystemAudit(args) => system_audit(args),
    }
}

fn env_check(workspace: &Path) -> anyhow::Result<()> {
    let root = canonical_workspace(workspace)?;
    let mut failures = Vec::new();

    for rel in [
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
        "AI_MERGE/14_upstream_full_adoption.md",
        "third_party/upstream/UPSTREAMS.md",
        "third_party/upstream/codegraph-rust/Cargo.toml",
        "third_party/upstream/repomix-rs/Cargo.toml",
    ] {
        if !root.join(rel).exists() {
            failures.push(format!(
                "missing required Codex environment artifact: {rel}"
            ));
        }
    }

    if root.join("crates/external/codegraph").exists() {
        failures
            .push("obsolete fake codegraph shim exists at crates/external/codegraph".to_string());
    }
    for rel in [
        ".codex/hooks/rusty_idd_stop_check.py",
        ".codex/scripts/rusty_idd_model_loop.py",
    ] {
        if root.join(rel).exists() {
            failures.push(format!(
                "obsolete Python Codex environment tool exists: {rel}"
            ));
        }
    }

    validate_json(&root.join(".codex/hooks.json"), &mut failures);
    for rel in [
        ".codex/config.toml",
        ".codex/agents/rusty-idd-explorer.toml",
        ".codex/agents/rusty-idd-implementer.toml",
        ".codex/agents/rusty-idd-verifier.toml",
        ".codex/agents/rusty-idd-gap-hunter.toml",
        ".codex/loops/rusty-idd-model-loop.toml",
    ] {
        validate_toml(&root.join(rel), &mut failures);
    }

    for (rel, needles) in [
        (
            ".codex/hooks.json",
            &[
                "git rev-parse --show-toplevel",
                "--manifest-path",
                "codex env-check",
                "--workspace",
                "\"timeout\": 180",
            ][..],
        ),
        (
            "AGENTS.md",
            &[
                "Adopt first, cut after evidence",
                "The agent owns output quality",
                "Upgrade only",
                "Treat stale or orphaned work as unfinished",
                "Tooling required to run this repo must be tracked",
                "Host service and process management is out of scope",
            ][..],
        ),
        (
            "adr/0004-knowledge-direct-crate-integration.md",
            &[
                "full upstream snapshots",
                "adopt the upstream parser/core",
                "Cut audit-denied",
            ][..],
        ),
        (
            "AI_MERGE/12_knowledge_deep_audit.md",
            &["Audit Cuts", "local Rust AST semantic pass were removed"][..],
        ),
        (
            "docs/rusty-idd/codex-environment.md",
            &[
                "Agent-Owned Tool Growth",
                "Upgrade-Only Gap Handling",
                "meta` / `envctl`",
                "Multi-Model Loop",
                "envctl",
                "toolchain",
                ".codex/rules",
            ][..],
        ),
        (
            "AI_MERGE/13_codex_environment.md",
            &[
                "rusty-idd codex env-check",
                "rusty-idd codex model-loop",
                "rusty-idd codex runtime-audit",
                "rusty-idd codex system-audit",
                "Codex owns its output quality",
                "stale or orphaned work",
                "Missing binaries needed for this repo",
            ][..],
        ),
        (
            "AI_MERGE/14_upstream_full_adoption.md",
            &[
                "ce5bf27a2978983a9089d177447f296e4c6521bb",
                "946df10d48c669ca3a99f757ffd2c6fa35844e62",
                "Native Upstream Diagnostics",
                "Consolidation Cuts Kept",
                "PR #50 Baseline Comparison",
            ][..],
        ),
        (
            "third_party/upstream/UPSTREAMS.md",
            &[
                "Jakedismo/codegraph-rust",
                "sopaco/repomix-rs",
                "ce5bf27a2978983a9089d177447f296e4c6521bb",
                "946df10d48c669ca3a99f757ffd2c6fa35844e62",
            ][..],
        ),
    ] {
        let path = root.join(rel);
        if !path.exists() {
            continue;
        }
        let text = read_text(&path)?;
        for needle in needles {
            if !text.contains(needle) {
                failures.push(format!(
                    "{rel} does not contain expected invariant text: {needle}"
                ));
            }
        }
    }

    for (rel, pattern) in [
        ("Cargo.lock", "lsh-rs2"),
        ("Cargo.lock", "dotenv 0.15"),
        ("crates/external/codegraph-parser/Cargo.toml", "lsh-rs2"),
        ("crates/external/codegraph-core/Cargo.toml", "dotenv ="),
    ] {
        let path = root.join(rel);
        if path.exists() && read_text(&path)?.contains(pattern) {
            failures.push(format!(
                "{rel} contains forbidden default-path dependency marker: {pattern}"
            ));
        }
    }

    if !failures.is_empty() {
        eprintln!("Rusty IDD Codex invariant check failed:");
        for failure in &failures {
            eprintln!("- {failure}");
        }
        bail!("{} invariant check(s) failed", failures.len());
    }

    println!("Rusty IDD Codex invariant check passed.");
    Ok(())
}

fn runtime_audit(workspace: &Path) -> anyhow::Result<()> {
    let root = canonical_workspace(workspace)?;
    let mut audit = RuntimePythonAudit::default();

    scan_dir(&root, &root, &mut audit)?;

    println!("Codex runtime audit");
    println!("- Live Codex Python commands: {}", audit.live_runtime.len());
    for finding in &audit.live_runtime {
        println!("  - {}: {}", finding.path, finding.detail);
    }
    println!(
        "- Obsolete Python Codex tool files: {}",
        audit.obsolete_tools.len()
    );
    for finding in &audit.obsolete_tools {
        println!("  - {}", finding.path);
    }
    println!(
        "- Parser/language support references: {}",
        audit.parser_support
    );
    println!("- Test fixture references: {}", audit.test_fixtures);
    println!(
        "- Rust audit implementation references: {}",
        audit.audit_implementation
    );
    println!(
        "- Core inventory support references: {}",
        audit.core_inventory
    );
    println!(
        "- Legacy bridge material references: {}",
        audit.legacy_bridge
    );
    println!("- Policy/documentation references: {}", audit.policy_docs);
    println!("- Other repo references: {}", audit.other_repo);
    for (path, count) in &audit.other_files {
        println!("  - {path}: {count}");
    }
    println!("- Ignored generated paths: target/**, .git/**, .idd/**");

    if !audit.live_runtime.is_empty() || !audit.obsolete_tools.is_empty() {
        bail!(
            "Codex runtime audit found {} live Python runtime command(s) and {} obsolete Python tool file(s)",
            audit.live_runtime.len(),
            audit.obsolete_tools.len()
        );
    }

    println!("Verdict: repo-local Codex runtime is Rust-native; Python remains only as language support, fixtures, policy text, or documentation.");
    Ok(())
}

fn system_audit(args: SystemAuditArgs) -> anyhow::Result<()> {
    let codex_bin = match args.codex_bin {
        Some(path) => path,
        None => find_on_path("codex").context("locate codex on PATH")?,
    };
    let resolved_codex = codex_bin
        .canonicalize()
        .with_context(|| format!("resolve codex binary {}", codex_bin.display()))?;
    let binary_kind = binary_kind(&resolved_codex)?;

    println!("Codex system audit");
    println!("- codex binary: {}", codex_bin.display());
    println!("- resolved codex binary: {}", resolved_codex.display());
    println!("- binary kind: {binary_kind}");
    println!(
        "- active runtime verdict: {}",
        if binary_kind == "ELF native executable" {
            "Rust-native binary path"
        } else {
            "not proven Rust-native"
        }
    );

    if let Some(source) = args.codex_source {
        let source = source
            .canonicalize()
            .with_context(|| format!("resolve Codex source root {}", source.display()))?;
        let source_audit = audit_codex_source_root(&source)?;
        println!("- Codex source root: {}", source.display());
        println!(
            "  - upstream Python developer tooling references: {}",
            source_audit.dev_python
        );
        println!(
            "  - upstream Python package/runtime references: {}",
            source_audit.package_python
        );
        println!(
            "  - source Rust workspace present: {}",
            source_audit.rust_workspace
        );
    }

    if let Some(envctl) = args.envctl {
        let envctl = envctl
            .canonicalize()
            .with_context(|| format!("resolve envctl root {}", envctl.display()))?;
        let envctl_audit = audit_envctl_codex_component(&envctl)?;
        println!("- envctl root: {}", envctl.display());
        println!("  - codex source-build path: {}", envctl_audit.source_build);
        println!(
            "  - direct Cargo codex build: {}",
            envctl_audit.direct_cargo_build
        );
        println!(
            "  - high-parallel Cargo jobs: {}",
            envctl_audit.parallel_jobs
        );
        println!("  - mold linker path: {}", envctl_audit.mold_linker);
        println!("  - Bun fallback only: {}", envctl_audit.bun_fallback);
        println!(
            "  - Python in codex component: {}",
            envctl_audit.python_mentions
        );
    }

    if binary_kind != "ELF native executable" {
        bail!("active codex binary is not a native ELF executable");
    }

    println!("Verdict: active Codex execution is Rust-native; Python is upstream developer/package tooling unless an envctl fallback installs the Bun package.");
    Ok(())
}

#[derive(Default)]
struct CodexSourceAudit {
    dev_python: usize,
    package_python: usize,
    rust_workspace: bool,
}

#[derive(Default)]
struct EnvctlCodexAudit {
    source_build: bool,
    direct_cargo_build: bool,
    parallel_jobs: bool,
    mold_linker: bool,
    bun_fallback: bool,
    python_mentions: usize,
}

fn audit_codex_source_root(root: &Path) -> anyhow::Result<CodexSourceAudit> {
    let mut audit = CodexSourceAudit {
        rust_workspace: root.join("codex-rs/Cargo.toml").exists(),
        ..CodexSourceAudit::default()
    };
    for rel in [
        "justfile",
        "scripts/format.py",
        "scripts/just-shell.py",
        ".github/scripts",
        "tools/argument-comment-lint",
    ] {
        audit.dev_python += count_python_markers_under(&root.join(rel))?;
    }
    for rel in [
        "codex-cli/scripts/build_npm_package.py",
        "scripts/codex_package",
        "sdk/python-runtime",
    ] {
        audit.package_python += count_python_markers_under(&root.join(rel))?;
    }
    Ok(audit)
}

fn audit_envctl_codex_component(root: &Path) -> anyhow::Result<EnvctlCodexAudit> {
    let path = root.join("manifest/ai-clis.toml");
    let text = read_text(&path)?;
    let codex = extract_component_block(&text, "codex-cli").unwrap_or_default();
    Ok(EnvctlCodexAudit {
        source_build: codex.contains("codex/codex-rs/cli"),
        direct_cargo_build: codex.contains("cargo build --release -p codex-cli"),
        parallel_jobs: codex.contains("CODEX_CARGO_JOBS")
            && codex.contains("CARGO_BUILD_JOBS")
            && codex.contains("--jobs"),
        mold_linker: codex.contains("mold") && codex.contains("fuse-ld=mold"),
        bun_fallback: codex.contains("bun install -g @openai/codex"),
        python_mentions: codex
            .lines()
            .filter(|line| contains_python_marker(line))
            .count(),
    })
}

fn extract_component_block(text: &str, id: &str) -> Option<String> {
    let mut blocks = text.split("[[component]]");
    blocks.find_map(|block| {
        block
            .contains(&format!("id = \"{id}\""))
            .then(|| block.to_string())
    })
}

fn count_python_markers_under(path: &Path) -> anyhow::Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    if path.is_file() {
        let text = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(_) => return Ok(0),
        };
        return Ok(python_marker_count(&path.display().to_string(), &text));
    }
    let mut count = 0;
    for entry in fs::read_dir(path).with_context(|| format!("read dir {}", path.display()))? {
        let entry = entry?;
        let child = entry.path();
        if child.is_dir() || child.is_file() {
            count += count_python_markers_under(&child)?;
        }
    }
    Ok(count)
}

fn binary_kind(path: &Path) -> anyhow::Result<&'static str> {
    let bytes = fs::read(path).with_context(|| format!("read binary header {}", path.display()))?;
    if bytes.starts_with(b"\x7fELF") {
        Ok("ELF native executable")
    } else if bytes.starts_with(b"#!/") {
        Ok("script")
    } else {
        Ok("unknown")
    }
}

fn find_on_path(command: &str) -> anyhow::Result<PathBuf> {
    let path_var = std::env::var_os("PATH").context("PATH is not set")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(command);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    bail!("{command} not found on PATH")
}

#[derive(Default)]
struct RuntimePythonAudit {
    live_runtime: Vec<RuntimeFinding>,
    obsolete_tools: Vec<RuntimeFinding>,
    parser_support: usize,
    test_fixtures: usize,
    audit_implementation: usize,
    core_inventory: usize,
    legacy_bridge: usize,
    policy_docs: usize,
    other_repo: usize,
    other_files: BTreeMap<String, usize>,
}

struct RuntimeFinding {
    path: String,
    detail: String,
}

fn scan_dir(root: &Path, dir: &Path, audit: &mut RuntimePythonAudit) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("read dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        if file_name == ".git" || file_name == "target" || file_name == ".idd" {
            continue;
        }
        if path.is_dir() {
            scan_dir(root, &path, audit)?;
            continue;
        }
        if path.is_file() {
            scan_file(root, &path, audit)?;
        }
    }
    Ok(())
}

fn scan_file(root: &Path, path: &Path, audit: &mut RuntimePythonAudit) -> anyhow::Result<()> {
    let rel = rel_path(root, path);
    let rel_str = rel.display().to_string();

    if is_obsolete_python_tool(&rel_str) {
        audit.obsolete_tools.push(RuntimeFinding {
            path: rel_str,
            detail: "repo-local Codex hook/script should be Rust-native".to_string(),
        });
        return Ok(());
    }

    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return Ok(()),
    };
    let mentions = python_marker_count(&rel_str, &text);
    if mentions == 0 {
        return Ok(());
    }

    if is_live_runtime_surface(&rel_str) {
        let live_mentions = live_python_lines(&text);
        audit
            .live_runtime
            .extend(live_mentions.into_iter().map(|line| RuntimeFinding {
                path: rel_str.clone(),
                detail: line,
            }));
    } else if is_parser_support(&rel_str) {
        audit.parser_support += mentions;
    } else if is_test_fixture(&rel_str) {
        audit.test_fixtures += mentions;
    } else if is_audit_implementation(&rel_str) {
        audit.audit_implementation += mentions;
    } else if is_core_inventory_support(&rel_str) {
        audit.core_inventory += mentions;
    } else if is_legacy_bridge_material(&rel_str) {
        audit.legacy_bridge += mentions;
    } else if is_policy_or_doc(&rel_str) {
        audit.policy_docs += mentions;
    } else {
        audit.other_repo += mentions;
        *audit.other_files.entry(rel_str).or_default() += mentions;
    }

    Ok(())
}

fn live_python_lines(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && contains_python_marker(line))
        .map(|line| line.to_string())
        .collect()
}

fn python_marker_count(path: &str, text: &str) -> usize {
    let path_count = usize::from(path.ends_with(".py"));
    path_count
        + text
            .lines()
            .filter(|line| contains_python_marker(line))
            .count()
}

fn contains_python_marker(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("python")
        || lower.contains("python3")
        || lower.contains(".py")
        || lower.contains("pip ")
        || lower.contains("\"pip\"")
        || lower.contains("pytest")
        || lower.contains(".venv")
        || lower.contains("venv/")
}

fn is_obsolete_python_tool(rel: &str) -> bool {
    rel.starts_with(".codex/hooks/") && rel.ends_with(".py")
        || rel.starts_with(".codex/scripts/") && rel.ends_with(".py")
}

fn is_live_runtime_surface(rel: &str) -> bool {
    rel == ".codex/hooks.json"
        || rel.starts_with(".codex/loops/")
        || rel.starts_with(".codex/agents/")
        || rel == "Justfile"
        || rel == "Makefile"
}

fn is_parser_support(rel: &str) -> bool {
    rel.starts_with("crates/external/codegraph-")
        || rel == "Cargo.lock"
        || rel == "Cargo.toml"
        || rel.starts_with("crates/core/src/fs_utils.rs")
        || rel.starts_with("crates/core/src/scanner.rs")
        || rel.starts_with("crates/core/src/model.rs")
}

fn is_test_fixture(rel: &str) -> bool {
    rel.contains("/tests/") || rel.ends_with("_cli.rs") || rel.contains("src/lib.rs")
}

fn is_audit_implementation(rel: &str) -> bool {
    rel == "crates/cli/src/commands/codex.rs"
}

fn is_core_inventory_support(rel: &str) -> bool {
    rel == "crates/core/src/env_contract.rs" || rel == "crates/core/src/planner.rs"
}

fn is_legacy_bridge_material(rel: &str) -> bool {
    rel.starts_with(".claude/") || rel.starts_with(".gemini/")
}

fn is_policy_or_doc(rel: &str) -> bool {
    rel == "AGENTS.md"
        || rel.starts_with(".agents/")
        || rel.starts_with(".codex/rules/")
        || rel.starts_with("docs/")
        || rel.starts_with("crates/core/docs/")
        || rel.starts_with("AI_MERGE/")
        || rel.starts_with("adr/")
        || rel == "README.md"
        || rel == "CLAUDE.md"
        || rel == "GEMINI.md"
}

fn rel_path<'a>(root: &'a Path, path: &'a Path) -> &'a Path {
    path.strip_prefix(root).unwrap_or(path)
}

fn validate_json(path: &Path, failures: &mut Vec<String>) {
    match read_text(path).and_then(|text| {
        serde_json::from_str::<serde_json::Value>(&text)
            .with_context(|| format!("parse JSON {}", path.display()))
    }) {
        Ok(_) => {}
        Err(error) => failures.push(error.to_string()),
    }
}

fn validate_toml(path: &Path, failures: &mut Vec<String>) {
    match read_text(path).and_then(|text| {
        toml::from_str::<toml::Value>(&text)
            .with_context(|| format!("parse TOML {}", path.display()))
    }) {
        Ok(_) => {}
        Err(error) => failures.push(error.to_string()),
    }
}

fn model_loop(args: ModelLoopArgs) -> anyhow::Result<()> {
    let config_path = args.config;
    let config_text = read_text(&config_path)
        .with_context(|| format!("read model loop config {}", config_path.display()))?;
    let config = toml::from_str::<LoopConfig>(&config_text)
        .with_context(|| format!("parse model loop config {}", config_path.display()))?;
    if config.passes.is_empty() {
        bail!("{} does not define any [[passes]]", config_path.display());
    }

    let selected = args
        .only
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>();
    let output_dir = args
        .output_dir
        .or(config.output_dir)
        .unwrap_or_else(|| PathBuf::from("/tmp/rusty-idd-codex-loop"));
    fs::create_dir_all(&output_dir)
        .with_context(|| format!("create model loop output dir {}", output_dir.display()))?;

    let run_id = unix_run_id()?;
    let mut manifest = LoopManifest {
        loop_name: config
            .name
            .unwrap_or_else(|| "rusty-idd-model-loop".to_string()),
        config: config_path.display().to_string(),
        run_id: run_id.clone(),
        execute: args.execute,
        passes: Vec::new(),
    };

    let mut saw_selected = selected.is_empty();
    let agent_dir = config_path
        .parent()
        .and_then(Path::parent)
        .map(|path| path.join("agents"))
        .unwrap_or_else(|| PathBuf::from(".codex/agents"));

    for pass in config.passes {
        if !selected.is_empty() && !selected.contains(&pass.name) {
            continue;
        }
        saw_selected = true;
        let output = output_dir.join(format!("{run_id}-{}.md", pass.name));
        let prompt = prompt_with_agent(&agent_dir, &pass)?;
        let command = codex_exec_command(&pass, &prompt, &output);
        let mut entry = LoopEntry {
            name: pass.name,
            agent: pass.agent,
            model: pass.model,
            reasoning: pass.reasoning,
            sandbox: pass.sandbox,
            output: output.display().to_string(),
            shell: shell_join(&command),
            command,
            exit_code: None,
        };
        println!("{}", serde_json::to_string(&entry)?);
        if args.execute {
            let status = Command::new(&entry.command[0])
                .args(&entry.command[1..])
                .status()
                .with_context(|| format!("execute {}", entry.shell))?;
            let code = status.code().unwrap_or(1);
            entry.exit_code = Some(code);
            manifest.passes.push(entry);
            if code != 0 {
                write_manifest(&output_dir, &run_id, &manifest)?;
                bail!("model loop pass failed with exit code {code}");
            }
        } else {
            manifest.passes.push(entry);
        }
    }

    if !saw_selected {
        bail!("no selected model-loop passes matched --only filters");
    }

    let manifest_path = write_manifest(&output_dir, &run_id, &manifest)?;
    println!(
        "{}",
        serde_json::to_string(&BTreeMap::from([("manifest", manifest_path)]))?
    );
    Ok(())
}

fn prompt_with_agent(agent_dir: &Path, pass: &ModelPass) -> anyhow::Result<String> {
    let path = agent_dir.join(format!("{}.toml", pass.agent));
    let text = read_text(&path).with_context(|| {
        format!(
            "read agent config for model-loop pass '{}' at {}",
            pass.name,
            path.display()
        )
    })?;
    let agent = toml::from_str::<AgentConfig>(&text)
        .with_context(|| format!("parse agent config {}", path.display()))?;

    let mut prompt = String::new();
    prompt.push_str("You are running as the repo-local Codex agent");
    if let Some(name) = agent.name.as_deref() {
        prompt.push_str(" `");
        prompt.push_str(name);
        prompt.push('`');
    }
    prompt.push_str(".\n");
    if let Some(description) = agent.description.as_deref() {
        prompt.push_str("\nAgent description:\n");
        prompt.push_str(description.trim());
        prompt.push('\n');
    }
    if let Some(instructions) = agent.developer_instructions.as_deref() {
        prompt.push_str("\nAgent instructions:\n");
        prompt.push_str(instructions.trim());
        prompt.push('\n');
    }
    prompt.push_str("\nTask prompt:\n");
    prompt.push_str(pass.prompt.trim());
    Ok(prompt)
}

fn codex_exec_command(pass: &ModelPass, prompt: &str, output: &Path) -> Vec<String> {
    vec![
        "codex".to_string(),
        "exec".to_string(),
        "--json".to_string(),
        "--sandbox".to_string(),
        pass.sandbox.clone(),
        "--model".to_string(),
        pass.model.clone(),
        "-c".to_string(),
        format!("model_reasoning_effort=\"{}\"", pass.reasoning),
        "-o".to_string(),
        output.display().to_string(),
        prompt.to_string(),
    ]
}

fn write_manifest(
    output_dir: &Path,
    run_id: &str,
    manifest: &LoopManifest,
) -> anyhow::Result<String> {
    let path = output_dir.join(format!("{run_id}-manifest.json"));
    let content = serde_json::to_string_pretty(manifest)? + "\n";
    fs::write(&path, content).with_context(|| format!("write {}", path.display()))?;
    Ok(path.display().to_string())
}

fn canonical_workspace(workspace: &Path) -> anyhow::Result<PathBuf> {
    let candidate = if workspace == Path::new(".") {
        git_root().unwrap_or_else(|| workspace.to_path_buf())
    } else {
        workspace.to_path_buf()
    };
    candidate
        .canonicalize()
        .with_context(|| format!("workspace path does not exist: {}", candidate.display()))
}

fn git_root() -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_string()))
}

fn read_text(path: &Path) -> anyhow::Result<String> {
    fs::read_to_string(path).with_context(|| format!("read {}", path.display()))
}

fn unix_run_id() -> anyhow::Result<String> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before UNIX epoch")?
        .as_secs();
    Ok(format!("{seconds}"))
}

fn shell_join(command: &[String]) -> String {
    command
        .iter()
        .map(|part| shell_quote(part))
        .collect::<Vec<_>>()
        .join(" ")
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.' | '/' | ':' | '='))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\"'\"'"))
    }
}
