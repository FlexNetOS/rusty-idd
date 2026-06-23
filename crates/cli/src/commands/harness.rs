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
    /// Goal file to bind into the package (verify stage).
    #[arg(long)]
    pub goal_file: Option<PathBuf>,
    /// Task file/card to bind into the package (verify stage).
    #[arg(long)]
    pub task_file: Option<PathBuf>,
    /// Plan file to bind into the package (verify stage).
    #[arg(long)]
    pub plan_file: Option<PathBuf>,
    /// Output format for the package contract.
    #[arg(long, value_enum, default_value = "markdown")]
    pub format: PackageFormat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum HarnessStage {
    Scan,
    Verify,
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
    /// Goal file bound into this package, if any (verify stage).
    #[serde(skip_serializing_if = "Option::is_none")]
    goal_file: Option<String>,
    /// Task file/card bound into this package, if any (verify stage).
    #[serde(skip_serializing_if = "Option::is_none")]
    task_file: Option<String>,
    /// Plan file bound into this package, if any (verify stage).
    #[serde(skip_serializing_if = "Option::is_none")]
    plan_file: Option<String>,
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
            let package = package_for(
                args.stage,
                &args.target,
                args.goal_file.as_deref(),
                args.task_file.as_deref(),
                args.plan_file.as_deref(),
            )?;
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

fn package_for(
    stage: HarnessStage,
    target: &Path,
    goal_file: Option<&Path>,
    task_file: Option<&Path>,
    plan_file: Option<&Path>,
) -> anyhow::Result<HarnessPackage> {
    if !target.exists() {
        bail!("package target does not exist: {}", target.display());
    }
    let target = target
        .canonicalize()
        .with_context(|| format!("resolve package target {}", target.display()))?;
    match stage {
        HarnessStage::Scan => Ok(scan_package(target)),
        HarnessStage::Verify => Ok(verify_package(target, goal_file, task_file, plan_file)),
    }
}

fn scan_package(target: PathBuf) -> HarnessPackage {
    HarnessPackage {
        name: "scan-stage scoped Rust agent swarm package".to_string(),
        stage: "scan",
        target: target.display().to_string(),
        goal_file: None,
        task_file: None,
        plan_file: None,
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

fn verify_package(
    target: PathBuf,
    goal_file: Option<&Path>,
    task_file: Option<&Path>,
    plan_file: Option<&Path>,
) -> HarnessPackage {
    HarnessPackage {
        name: "verify-stage scoped Rust agent swarm package".to_string(),
        stage: "verify",
        target: target.display().to_string(),
        goal_file: goal_file.map(|p| p.display().to_string()),
        task_file: task_file.map(|p| p.display().to_string()),
        plan_file: plan_file.map(|p| p.display().to_string()),
        purpose: "Bound the verify stage to exhaustively cross-verify completed implementation against the original request, goal, task card, OpenSpec tasks, plan, diff, tests, graph context, and ICM memory, then emit a typed pass/fail verdict before PR handoff.",
        agent_team: vec![
            entry("verify-orchestrator", "Owns the end-to-end verification workflow, sequences the checks, and emits the pass/fail verdict."),
            entry("diff-auditor", "Reviews git diff and classifies every changed file against the declared task scope."),
            entry("test-runner", "Runs focused build/test/lint gates scoped to the change and captures results."),
            entry("goal-comparator", "Compares completed work against the original request, goal file, OpenSpec tasks, and plan."),
            entry("graph-checker", "Compares relevant knowledge graph and context artifacts against the implementation."),
            entry("icm-checker", "Runs ICM recall queries and compares results against implementation assumptions."),
            entry("evidence-checker", "Confirms generated artifacts are fresh and every evidence field is populated."),
            entry("risk-reviewer", "Builds the rollback risk matrix and surfaces unresolved questions."),
        ],
        contracts: vec![
            entry("original-request-contract", "Verification must trace back to the original user request before a pass verdict."),
            entry("goal-contract", "Implementation must satisfy the declared goal file when one is bound."),
            entry("task-plan-contract", "Every task checklist item and plan step must be addressed or explicitly flagged."),
            entry("implementation-diff-contract", "The diff must be reviewed and each changed file classified against the task scope."),
            entry("test-contract", "Tests appropriate to the change must exist and pass; missing coverage is a finding."),
            entry("graph-contract", "Knowledge graph and context artifacts must be compared, not assumed current."),
            entry("icm-comparison-contract", "ICM recall results must be compared against the implementation and mismatches reported."),
            entry("evidence-contract", "Every evidence-schema field must be populated before a pass verdict is emitted."),
            entry("adapter-boundary-contract", "Treat .codex, .claude, .kimi, and .agents as thin adapters; this package is the verification source of truth."),
        ],
        tools: vec![
            entry("git status", "Inspect working tree state before and after verification steps."),
            entry("git diff", "Review the full implementation diff and classify changed files."),
            entry("git log", "Trace recent commits against the declared task scope."),
            entry("rusty-idd validate", "Run Rusty IDD validation gates and capture output as evidence."),
            entry("rusty-idd manifest", "Refresh the deterministic artifact inventory and confirm generated artifacts are fresh."),
            entry("rusty-idd knowledge refresh", "Ensure knowledge artifacts are current before graph comparison."),
            entry("rusty-idd knowledge plan-context", "Retrieve graph-backed context for comparison against the implementation."),
            entry("rusty-idd spec status", "Verify the active OpenSpec change status and that all spec tasks are addressed."),
            entry("focused-build-test-lint", "Language-appropriate build, test, and lint gates scoped to the changed files."),
            entry("icm-recall-context-compare", "Retrieve relevant ICM memory and compare it against implementation decisions."),
        ],
        helpers: vec![
            entry("goal-normalization", "Extract the canonical goal statement from a goal file or inline text for comparison."),
            entry("task-checklist-extraction", "Pull checklist items from the task artifact and track which are addressed."),
            entry("changed-file-classifier", "Classify changed files by scope: in-task, unexpected drift, or test-only."),
            entry("risk-matrix-builder", "Build a rollback risk matrix from diff scope, test coverage, and blast radius."),
            entry("original-request-comparator", "Compare the final implementation against the original request text and surface gaps."),
            entry("queue-question-extractor", "Surface unanswered questions or blockers for the unresolved-questions evidence field."),
        ],
        hooks: vec![
            entry("pre-verify-snapshot", "Capture a pre-verification snapshot of working tree state before any verification commands run."),
            entry("generated-artifact-freshness-check", "Assert knowledge artifacts and manifests are current before verification begins; fail if stale."),
            entry("evidence-write-check", "Assert every evidence-schema field is populated before stop-phase handoff; fail if incomplete."),
        ],
        validation_gates: vec![
            entry("goal-matched", "Implementation satisfies the declared goal; any gap is a blocking finding."),
            entry("tasks-satisfied", "All task checklist items are addressed or explicitly flagged out-of-scope with rationale."),
            entry("tests-appropriate", "Tests appropriate to the diff exist and pass; absent tests for non-trivial changes are blocking."),
            entry("diff-reviewed", "All changed files are classified and the diff is reviewed for scope drift."),
            entry("generated-artifacts-fresh", "Knowledge and manifest artifacts are not stale relative to the implementation."),
            entry("graph-icm-checked", "Graph and ICM comparisons are complete; discrepancies are noted in evidence."),
            entry("evidence-complete", "Every evidence-schema field is populated before a pass verdict is issued."),
            entry("unresolved-questions-explicit", "Any unresolved questions are listed explicitly; silently dropping them fails the gate."),
            entry("rollback-path-present", "A rollback path or risk assessment is documented in the evidence."),
        ],
        evidence_schema: vec![
            entry("findings", "Blocking and non-blocking findings with concrete file, command, or artifact references."),
            entry("commands-run", "Every command executed during verification with output summaries or exit codes."),
            entry("diff-summary", "Changed-file classification, diff scope assessment, and unexpected-drift notes."),
            entry("test-results", "Build, test, and lint gate outcomes with pass/fail status per gate."),
            entry("graph-knowledge-comparison", "Knowledge graph and context artifact comparison notes against the implementation."),
            entry("icm-comparison", "ICM recall results and comparison against implementation decisions and assumptions."),
            entry("unanswered-questions", "Explicit list of unresolved questions; an empty list is valid for a pass verdict."),
            entry("pass-fail-verdict", "Final pass or fail verdict with rollback risk level and evidence locations."),
        ],
        adapter_boundary: vec![
            entry(".codex", "Thin Codex launch adapter that calls rusty-idd harness package --stage verify and follows the package contract."),
            entry(".claude", "Compatibility view; the verify package is the source of truth for post-task verification behavior."),
            entry(".kimi", "Optional runtime adapter when present; must delegate to this package, not embed a parallel checklist."),
            entry(".agents", "Reusable instructions remain available; the verify package decides what is loaded for the verify stage."),
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
    if let Some(goal) = &package.goal_file {
        out.push_str("- Goal file: `");
        out.push_str(goal);
        out.push_str("`\n");
    }
    if let Some(task) = &package.task_file {
        out.push_str("- Task file: `");
        out.push_str(task);
        out.push_str("`\n");
    }
    if let Some(plan) = &package.plan_file {
        out.push_str("- Plan file: `");
        out.push_str(plan);
        out.push_str("`\n");
    }
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
