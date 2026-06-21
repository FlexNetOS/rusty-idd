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
        ".idd/knowledge/index.json",
        ".idd/knowledge/report.md",
        "docs/rusty-idd/codex-environment.md",
        "adr/0004-knowledge-direct-crate-integration.md",
        "AI_MERGE/12_knowledge_deep_audit.md",
        "AI_MERGE/13_codex_environment.md",
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
            &["adopt the upstream parser/core", "Cut audit-denied"][..],
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
                "Codex owns its output quality",
                "stale or orphaned work",
                "Missing binaries needed for this repo",
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
