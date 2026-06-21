use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rusty_idd_knowledge::{IntegrationAutomationPlan, IntegrationWorkItem};

#[derive(Debug, Clone)]
pub struct PlanIntegrationArgs {
    pub base: PathBuf,
    pub integration_plan: Option<PathBuf>,
    pub change: Option<String>,
    pub capability: Option<String>,
    pub work_item: Option<String>,
    pub force: bool,
}

pub fn run(args: PlanIntegrationArgs) -> i32 {
    match try_run(args) {
        Ok(change_dir) => {
            println!("Created integration OpenSpec change:");
            println!("  {}", change_dir.display());
            0
        }
        Err(error) => {
            eprintln!("rusty-idd spec plan-integration: {error:#}");
            1
        }
    }
}

fn try_run(args: PlanIntegrationArgs) -> Result<PathBuf> {
    let plan_path = args
        .integration_plan
        .unwrap_or_else(|| args.base.join(".idd/knowledge/integration-plan.json"));
    let plan = read_plan(&plan_path)?;
    let item = select_work_item(
        &plan,
        args.change.as_deref(),
        args.capability.as_deref(),
        args.work_item.as_deref(),
    )?;
    write_change(&args.base, item, args.force)
}

fn read_plan(path: &Path) -> Result<IntegrationAutomationPlan> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read integration plan {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("failed to parse integration plan {}", path.display()))
}

fn select_work_item<'a>(
    plan: &'a IntegrationAutomationPlan,
    change: Option<&str>,
    capability: Option<&str>,
    work_item: Option<&str>,
) -> Result<&'a IntegrationWorkItem> {
    let selector_count = [change, capability, work_item]
        .into_iter()
        .flatten()
        .count();
    if selector_count > 1 {
        anyhow::bail!("select only one of --change, --capability, or --work-item");
    }

    let selected = if let Some(change) = change {
        plan.work_items
            .iter()
            .find(|item| item.change_id == change)
            .with_context(|| format!("no integration work item found for change {change:?}"))?
    } else if let Some(capability) = capability {
        plan.work_items
            .iter()
            .find(|item| {
                item.capability == capability || item.capability == capability_id(capability)
            })
            .with_context(|| {
                format!("no integration work item found for capability {capability:?}")
            })?
    } else if let Some(work_item) = work_item {
        plan.work_items
            .iter()
            .find(|item| item.id == work_item)
            .with_context(|| format!("no integration work item found for id {work_item:?}"))?
    } else {
        plan.work_items
            .iter()
            .min_by(|a, b| {
                a.priority
                    .cmp(&b.priority)
                    .then(a.change_id.cmp(&b.change_id))
            })
            .context("integration plan contains no work items")?
    };

    Ok(selected)
}

fn write_change(base: &Path, item: &IntegrationWorkItem, force: bool) -> Result<PathBuf> {
    let change_dir = base.join("openspec/changes").join(&item.change_id);
    let spec_dir = change_dir
        .join("specs")
        .join(capability_slug(&item.capability));
    let files = [
        (change_dir.join("proposal.md"), render_proposal(item)),
        (change_dir.join("design.md"), render_design(item)),
        (change_dir.join("tasks.md"), render_tasks(item)),
        (spec_dir.join("spec.md"), render_spec(item)),
    ];

    for (path, _) in &files {
        if path.exists() && !force {
            anyhow::bail!(
                "refusing to overwrite existing {}; pass --force to replace generated artifacts",
                path.display()
            );
        }
    }

    for (path, content) in files {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
        fs::write(&path, content).with_context(|| format!("failed to write {}", path.display()))?;
    }

    Ok(change_dir)
}

fn render_proposal(item: &IntegrationWorkItem) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", item.change_id));
    out.push_str("## Why\n\n");
    out.push_str(&format!(
        "Rusty IDD selected `{}` from the integration automation plan to move `{}` from `{}` toward implemented system capability.\n\n",
        item.id, item.title, item.status
    ));
    out.push_str("This change preserves the graph-backed owners, anchors, adopt-first inputs, validation gates, and rollback path before implementation begins.\n\n");
    out.push_str("## What Changes\n\n");
    out.push_str(&format!(
        "- Implement integration work item `{}`.\n",
        item.id
    ));
    out.push_str(&format!(
        "- Keep the implementation boundary: {}.\n",
        item.implementation_boundary
    ));
    out.push_str("- Use TDD consolidation: adopt current upstream/owner surfaces first, run native diagnostics, then cut only evidenced friction.\n\n");
    out.push_str("## Capabilities\n\n");
    out.push_str("### New Capabilities\n\n");
    out.push_str(&format!(
        "- `{}`: {}\n\n",
        capability_slug(&item.capability),
        item.title
    ));
    out.push_str("### Modified Capabilities\n\n");
    out.push_str("- `integration-automation-plan`: this work item is now executing through OpenSpec artifacts.\n\n");
    out.push_str("## Impact\n\n");
    out.push_str("- Owner repos:\n");
    push_markdown_list(&mut out, &item.owner_repos, "  ");
    if !item.anchors.is_empty() {
        out.push_str("- Anchors:\n");
        push_markdown_list(&mut out, &item.anchors, "  ");
    }
    if !item.adopt_first_inputs.is_empty() {
        out.push_str("- Adopt-first inputs:\n");
        push_markdown_list(&mut out, &item.adopt_first_inputs, "  ");
    }
    out
}

fn render_design(item: &IntegrationWorkItem) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {} - Design\n\n", item.change_id));
    out.push_str("## Context\n\n");
    out.push_str(&format!(
        "Integration work item `{}` maps capability `{}` in layer `{}` to owner repos and validation gates generated by Rusty IDD.\n\n",
        item.id, item.capability, item.layer
    ));
    out.push_str("## Goals / Non-Goals\n\n");
    out.push_str("**Goals:**\n\n");
    out.push_str("- Adopt the current owner/upstream surfaces before cutting local behavior.\n");
    out.push_str("- Keep Rusty IDD glue thin: DTO mapping, deterministic output, feature flags, validation, and CLI/API calls.\n");
    out.push_str("- Preserve `crates/core` as std-only.\n\n");
    out.push_str("**Non-Goals:**\n\n");
    out.push_str("- Starting host services, daemons, MCP servers, or vault relays from default Rusty IDD workflows.\n");
    out.push_str(
        "- Downgrading a working upstream or owner-repo capability to simplify implementation.\n\n",
    );
    out.push_str("## Decisions\n\n");
    out.push_str(&format!(
        "- Implementation boundary: {}.\n",
        item.implementation_boundary
    ));
    out.push_str("- Every consolidation cut must cite failing build, audit, platform, or scope evidence and include rollback.\n");
    if !item.adopt_first_inputs.is_empty() {
        out.push_str("- Adopt-first inputs:\n");
        push_markdown_list(&mut out, &item.adopt_first_inputs, "  ");
    }
    out.push_str("\n## Risks / Trade-offs\n\n");
    out.push_str("- Cross-repo ownership may require separate PRs in owner repos; this Rusty IDD change records the execution contract first.\n");
    out.push_str("- Native diagnostics may fail for upstream reasons; failures are evidence, not permission to pre-filter.\n\n");
    out.push_str("## Migration Plan\n\n");
    out.push_str(
        "1. Re-read current owner repo docs, scripts, CI, and generated architecture artifacts.\n",
    );
    out.push_str(
        "2. Adopt the relevant current surfaces as-is or reference pinned upstream mirrors.\n",
    );
    out.push_str("3. Run native diagnostics before cuts.\n");
    out.push_str("4. Apply the thinnest Rusty IDD boundary and focused tests.\n");
    out.push_str("5. Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.\n");
    out.push_str("6. Run full Rusty IDD gates and owner-repo smoke tests.\n\n");
    out.push_str("## Open Questions\n\n");
    out.push_str(
        "- Which owner repo owns the first implementation PR if multiple repos are listed?\n",
    );
    out
}

fn render_tasks(item: &IntegrationWorkItem) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {} - Tasks\n\n", item.change_id));
    out.push_str("## 1. Adopt-First Evidence\n\n");
    out.push_str("- [ ] 1.1 Re-read owner repo docs, scripts, CI, package metadata, generated architecture artifacts, and relevant ADR/AI_MERGE notes.\n");
    out.push_str(
        "- [ ] 1.2 Pin or verify every upstream/current owner surface used by this slice.\n",
    );
    out.push_str(
        "- [ ] 1.3 Run native build/test/diagnostic commands before cutting local behavior.\n",
    );
    if !item.adopt_first_inputs.is_empty() {
        out.push_str("- [ ] 1.4 Adopt-first inputs to verify:\n");
        for input in &item.adopt_first_inputs {
            out.push_str(&format!("  - [ ] `{input}`\n"));
        }
    }
    out.push_str("\n## 2. Implementation\n\n");
    out.push_str("- [ ] 2.1 Add the thinnest Rusty IDD boundary for this capability.\n");
    out.push_str("- [ ] 2.2 Preserve deterministic output, validation, size/token policy, and feature flags.\n");
    out.push_str("- [ ] 2.3 Keep `crates/core` std-only.\n");
    out.push_str("- [ ] 2.4 Record every consolidation cut with evidence and rollback.\n\n");
    out.push_str("## 3. Validation\n\n");
    for (idx, gate) in item.validation.iter().enumerate() {
        out.push_str(&format!("- [ ] 3.{} `{gate}`\n", idx + 1));
    }
    out.push_str("- [ ] 3.10 Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.\n");
    out.push_str("- [ ] 3.11 Record evidence in `/AI_MERGE`.\n\n");
    out.push_str("## Rollback\n\n");
    for step in &item.rollback {
        out.push_str(&format!("- {step}\n"));
    }
    out
}

fn render_spec(item: &IntegrationWorkItem) -> String {
    let mut out = String::new();
    out.push_str("## ADDED Requirements\n\n");
    out.push_str(&format!("### Requirement: {}\n", item.title));
    out.push_str(&format!(
        "Rusty IDD SHALL integrate capability `{}` through the documented owner repos while preserving adopt-first evidence and deterministic validation.\n\n",
        item.capability
    ));
    out.push_str("#### Scenario: Adopt-first evidence is recorded\n");
    out.push_str("- **GIVEN** the selected integration work item lists owner repos, anchors, and adopt-first inputs\n");
    out.push_str("- **WHEN** implementation begins\n");
    out.push_str("- **THEN** native diagnostics and current owner/upstream evidence SHALL be recorded before any consolidation cut.\n\n");
    out.push_str("#### Scenario: Thin Rusty IDD boundary is implemented\n");
    out.push_str("- **GIVEN** upstream or owner repo behavior is proven through diagnostics\n");
    out.push_str("- **WHEN** Rusty IDD wires the capability\n");
    out.push_str("- **THEN** the local boundary SHALL be limited to DTO mapping, deterministic output, feature flags, validation, size/token policy, and CLI/API calls.\n\n");
    out.push_str("#### Scenario: Validation gates protect the integration\n");
    out.push_str("- **GIVEN** the implementation and generated artifacts are complete\n");
    out.push_str("- **WHEN** the integration is proposed for merge\n");
    out.push_str("- **THEN** focused tests, affected smoke tests, Rusty IDD validation, and full gates SHALL pass or the change SHALL remain unmerged.\n");
    out
}

fn push_markdown_list(out: &mut String, values: &[String], indent: &str) {
    if values.is_empty() {
        out.push_str(&format!("{indent}- none\n"));
    } else {
        for value in values {
            out.push_str(&format!("{indent}- `{value}`\n"));
        }
    }
}

fn capability_id(value: &str) -> String {
    if value.starts_with("capability:") {
        value.to_string()
    } else {
        format!("capability:{value}")
    }
}

fn capability_slug(value: &str) -> String {
    slug(value.strip_prefix("capability:").unwrap_or(value))
}

fn slug(value: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    while out.starts_with('-') {
        out.remove(0);
    }
    if out.is_empty() {
        "integration-work".to_string()
    } else {
        out
    }
}
