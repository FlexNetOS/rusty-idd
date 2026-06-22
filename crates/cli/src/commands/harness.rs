use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use clap::{Args, Subcommand, ValueEnum};
use serde::Serialize;

#[derive(Subcommand)]
pub enum HarnessCommand {
    /// Create or select a task-scoped harness package for a workflow stage.
    Package(PackageArgs),
}

#[derive(Args)]
pub struct PackageArgs {
    /// Workflow stage to package.
    #[arg(long, value_enum)]
    pub stage: HarnessStage,
    /// Repo or path the stage package will operate on.
    #[arg(long, default_value = ".")]
    pub target: PathBuf,
    /// Output format for the package contract.
    #[arg(long, value_enum, default_value = "markdown")]
    pub format: PackageFormat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum HarnessStage {
    Scan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum PackageFormat {
    Json,
    Markdown,
}

#[derive(Debug, Serialize)]
struct HarnessPackage {
    name: String,
    stage: &'static str,
    target: String,
    purpose: &'static str,
    agent_team: Vec<PackageEntry>,
    contracts: Vec<PackageEntry>,
    tools: Vec<PackageEntry>,
    helpers: Vec<PackageEntry>,
    hooks: Vec<PackageEntry>,
    validation_gates: Vec<PackageEntry>,
    evidence_schema: Vec<PackageEntry>,
    adapter_boundary: Vec<PackageEntry>,
}

#[derive(Debug, Serialize)]
struct PackageEntry {
    name: &'static str,
    purpose: &'static str,
}

pub fn run(command: HarnessCommand) -> i32 {
    match try_run(command) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("rusty-idd harness: {error:#}");
            1
        }
    }
}

fn try_run(command: HarnessCommand) -> anyhow::Result<()> {
    match command {
        HarnessCommand::Package(args) => {
            let package = package_for(args.stage, &args.target)?;
            match args.format {
                PackageFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&package)?);
                }
                PackageFormat::Markdown => {
                    print!("{}", render_markdown(&package));
                }
            }
        }
    }
    Ok(())
}

fn package_for(stage: HarnessStage, target: &Path) -> anyhow::Result<HarnessPackage> {
    if !target.exists() {
        bail!("package target does not exist: {}", target.display());
    }
    let target = target
        .canonicalize()
        .with_context(|| format!("resolve package target {}", target.display()))?;
    match stage {
        HarnessStage::Scan => Ok(scan_package(target)),
    }
}

fn scan_package(target: PathBuf) -> HarnessPackage {
    HarnessPackage {
        name: "scan-stage scoped Rust agent swarm package".to_string(),
        stage: "scan",
        target: target.display().to_string(),
        purpose: "Bound the scan stage to only the roles, contracts, tools, helpers, hooks, gates, and evidence needed to inventory a target and hand typed evidence to the next workflow stage.",
        agent_team: vec![
            entry("scan-orchestrator", "Owns stage routing, package scope, and evidence handoff for the scan target."),
            entry("inventory-reader", "Collects repository files, manifests, agent surfaces, and toolchain signals without writing."),
            entry("risk-classifier", "Classifies secrets, workflow drift, tool overflow, and adapter-boundary risks from scan outputs."),
        ],
        contracts: vec![
            entry("target-contract", "The scan package operates only on the declared target path."),
            entry("inventory-contract", "Capture files, package managers, languages, agent directories, and workflow control-plane surfaces."),
            entry("adapter-boundary-contract", "Treat .codex, .claude, .kimi, .agents, and peer agent directories as adapters or compatibility views, not source-of-truth toolboxes."),
            entry("no-default-mcp-contract", "Do not include MCP servers in the default scan package unless a later feature gate declares a stage-specific reason."),
        ],
        tools: vec![
            entry("rusty-idd scan", "Generate deterministic inventory for the target."),
            entry("rusty-idd knowledge plan-context", "Bind graph-backed context to the current goal before implementation."),
            entry("rusty-idd manifest", "Refresh deterministic artifact inventory after scan-related control-plane changes."),
            entry("rusty-idd validate", "Run Rusty IDD validation gates before handoff."),
            entry("rusty-idd spec status", "Verify the active OpenSpec change before later workflow stages write code."),
        ],
        helpers: vec![
            entry("bounded-context-pack", "Use generated knowledge and context reports instead of broad manual rescans."),
            entry("adapter-surface-map", "List agent directories as launch adapters and compatibility sources."),
            entry("package-scope-check", "Ensure the selected package does not load unrelated stage tools."),
        ],
        hooks: vec![
            entry("pre-scan-package-check", "Verify goal, target, active change, and package stage before scan execution."),
            entry("post-scan-evidence-check", "Require scan evidence and next-stage recommendation before handoff."),
        ],
        validation_gates: vec![
            entry("target-exists", "The declared package target must exist."),
            entry("default-tool-scope", "The package tool list must stay scan-specific and omit default MCP sprawl."),
            entry("adapter-minimality", "Adapter directories must not become the authoritative package catalog."),
            entry("typed-evidence", "The package must declare evidence fields for the next workflow stage."),
        ],
        evidence_schema: vec![
            entry("inventory", "Repository inventory, package managers, languages, manifests, and agent adapter surfaces."),
            entry("graph-context", "Knowledge graph and bounded context evidence for planning."),
            entry("risk-register", "Tool overflow, secret/config, workflow drift, and adapter-boundary findings."),
            entry("validation-summary", "Commands run, generated artifacts refreshed, and remaining gaps."),
            entry("next-stage-recommendation", "The workflow stage and scoped package that should run after scan."),
        ],
        adapter_boundary: vec![
            entry(".codex", "Thin Codex launch adapter that calls Rusty IDD package generation."),
            entry(".claude", "Compatibility/source-material view, not the active workflow package catalog."),
            entry(".kimi", "Optional runtime adapter when present, not a source-of-truth toolbox."),
            entry(".agents", "Reusable instructions remain available, but workflow packages decide what is loaded for a stage."),
        ],
    }
}

fn entry(name: &'static str, purpose: &'static str) -> PackageEntry {
    PackageEntry { name, purpose }
}

fn render_markdown(package: &HarnessPackage) -> String {
    let mut out = String::new();
    out.push_str("# ");
    out.push_str(&package.name);
    out.push_str("\n\n");
    out.push_str("- Stage: `");
    out.push_str(package.stage);
    out.push_str("`\n");
    out.push_str("- Target: `");
    out.push_str(&package.target);
    out.push_str("`\n");
    out.push_str("- Purpose: ");
    out.push_str(package.purpose);
    out.push_str("\n\n");
    render_section(&mut out, "Agent Team", &package.agent_team);
    render_section(&mut out, "Contracts", &package.contracts);
    render_section(&mut out, "Tools", &package.tools);
    render_section(&mut out, "Helpers", &package.helpers);
    render_section(&mut out, "Hooks", &package.hooks);
    render_section(&mut out, "Validation Gates", &package.validation_gates);
    render_section(&mut out, "Evidence Schema", &package.evidence_schema);
    render_section(&mut out, "Adapter Boundary", &package.adapter_boundary);
    out
}

fn render_section(out: &mut String, title: &str, entries: &[PackageEntry]) {
    out.push_str("## ");
    out.push_str(title);
    out.push_str("\n\n");
    for entry in entries {
        out.push_str("- `");
        out.push_str(entry.name);
        out.push_str("`: ");
        out.push_str(entry.purpose);
        out.push('\n');
    }
    out.push('\n');
}
