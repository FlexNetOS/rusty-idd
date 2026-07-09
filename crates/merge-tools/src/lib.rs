//! Reusable Rusty IDD merge-goal workflow package.
//!
//! This crate preserves the useful contracts from the retired `idd-merge-idd`
//! Claude/Gemini bridge material as Rusty IDD-owned data. It can also verify
//! the safety gates that replaced the old bridge drift scripts.

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergePhase {
    pub name: &'static str,
    pub purpose: &'static str,
    pub inputs: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub gates: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacySurface {
    pub path: &'static str,
    pub disposition: &'static str,
    pub replacement: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergeToolPackage {
    pub name: &'static str,
    pub phases: &'static [MergePhase],
    pub legacy_surfaces: &'static [LegacySurface],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerificationReport {
    pub workspace: PathBuf,
    pub checked_crates: usize,
    pub checked_src_trees: usize,
    pub findings: Vec<String>,
}

impl VerificationReport {
    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }
}

pub const MERGE_PHASES: &[MergePhase] = &[
    MergePhase {
        name: "inventory",
        purpose: "Make repositories, crates, contracts, env keys, and current behavior explicit before planning.",
        inputs: &["user merge goal", "repo state", ".idd/knowledge/*"],
        outputs: &["RepoInventory", "feature matrix", "env/secret contract", "legacy surface inventory"],
        gates: &["rusty-idd scan", "rusty-idd knowledge refresh", "no untracked secret material"],
    },
    MergePhase {
        name: "plan",
        purpose: "Bind the goal to graph-backed context and one OpenSpec change before implementation.",
        inputs: &["inventory outputs", ".idd/knowledge/plan-context.md"],
        outputs: &["proposal.md", "spec deltas", "design.md", "tasks.md"],
        gates: &["rusty-idd spec status", "rusty-idd spec next"],
    },
    MergePhase {
        name: "decide",
        purpose: "Record only durable active decisions; keep retired merge decisions as package evidence, not active ADRs.",
        inputs: &["design tradeoffs", "legacy decision summaries"],
        outputs: &["active ADR", "migration note"],
        gates: &["rusty-idd spec adr list adr --all", "adr/ contains only active ADRs"],
    },
    MergePhase {
        name: "implement",
        purpose: "Apply one narrow vertical slice after OpenSpec readiness, preserving old behavior until parity is proven.",
        inputs: &["ready OpenSpec change", "task list", "adopt-first inputs"],
        outputs: &["code/docs changes", "updated generated artifacts"],
        gates: &["adopt first", "cut only evidenced friction", "core crate remains zero-dependency"],
    },
    MergePhase {
        name: "verify",
        purpose: "Run boundary checks that prove the merge did not drift, leak secrets, or bypass the lifecycle.",
        inputs: &["changed tree", "declared parity or correctness gate"],
        outputs: &["build/test/lint/validation evidence", "manifest refresh"],
        gates: &[
            "cargo build --workspace",
            "cargo test --workspace --locked",
            "cargo fmt --all -- --check",
            "cargo clippy --workspace --all-targets --all-features -- -D warnings",
            "rusty-idd validate --workspace .",
        ],
    },
    MergePhase {
        name: "evidence",
        purpose: "Record PR evidence, rollback, manifest state, and optional AI_MERGE notes when audit history is required.",
        inputs: &["verification results", "migration note", "rollback path"],
        outputs: &["PR evidence bundle", ".idd/MANIFEST.tsv", "optional AI_MERGE evidence"],
        gates: &["manifest updated or justified", "AI_MERGE is evidence only"],
    },
];

pub const LEGACY_SURFACES: &[LegacySurface] = &[
    LegacySurface {
        path: ".claude/agents/* and .claude/skills/*merge*",
        disposition: "retired active bridge material",
        replacement: "Rusty IDD merge-tools package plus AGENTS.md and .agents/skills",
    },
    LegacySurface {
        path: ".gemini/agents/* and .gemini/skills/*merge*",
        disposition: "retired mirrored bridge material",
        replacement: "Rusty IDD merge-tools package plus GEMINI.md bridge note",
    },
    LegacySurface {
        path: "_workspace/{backlog,loop_state,HANDOFF}.md",
        disposition: "retired run-local continuity state",
        replacement: ".idd/knowledge/*, OpenSpec tasks, and PR evidence",
    },
    LegacySurface {
        path: "AI_MERGE/",
        disposition: "downgraded from control plane to evidence surface",
        replacement: "Rusty IDD workflow with optional AI_MERGE audit records",
    },
    LegacySurface {
        path: "adr/0001..0006 legacy merge decisions",
        disposition: "summarized as historical decision inputs",
        replacement: "single active Codex harness ADR",
    },
    LegacySurface {
        path: "research_ai_autopilot_merge.md and idd-merge-workspace.code-workspace",
        disposition: "retired research/bootstrap artifacts",
        replacement: "docs/rusty-idd/merge-tools-package.md",
    },
];

pub fn package() -> MergeToolPackage {
    MergeToolPackage {
        name: "Rusty IDD Merge Tool Package",
        phases: MERGE_PHASES,
        legacy_surfaces: LEGACY_SURFACES,
    }
}

pub fn render_markdown(package: &MergeToolPackage) -> String {
    let mut out = String::new();
    out.push_str("# ");
    out.push_str(package.name);
    out.push_str("\n\n## Phases\n\n");
    for phase in package.phases {
        out.push_str("### ");
        out.push_str(phase.name);
        out.push_str("\n\n");
        out.push_str(phase.purpose);
        out.push_str("\n\n");
        render_list(&mut out, "Inputs", phase.inputs);
        render_list(&mut out, "Outputs", phase.outputs);
        render_list(&mut out, "Gates", phase.gates);
    }
    out.push_str("## Legacy Surface Disposition\n\n");
    out.push_str("| Path | Disposition | Replacement |\n");
    out.push_str("| --- | --- | --- |\n");
    for surface in package.legacy_surfaces {
        out.push_str("| `");
        out.push_str(surface.path);
        out.push_str("` | ");
        out.push_str(surface.disposition);
        out.push_str(" | ");
        out.push_str(surface.replacement);
        out.push_str(" |\n");
    }
    out
}

pub fn verify_workspace(workspace: impl AsRef<Path>) -> Result<VerificationReport, String> {
    let root = workspace.as_ref();
    if !root.is_dir() {
        return Err(format!("workspace is not a directory: {}", root.display()));
    }

    let mut report = VerificationReport {
        workspace: root.to_path_buf(),
        checked_crates: 0,
        checked_src_trees: 0,
        findings: Vec::new(),
    };

    let files = collect_files(root)?;
    for file in &files {
        let rel = relative(root, file);
        if is_foreign_manifest(&rel) && !foreign_manifest_allowed(&rel) {
            report.findings.push(format!(
                "foreign package manifest outside allowed surfaces: {rel}"
            ));
        }
        if src_purity_enforced(&rel)
            && rel.contains("/src/")
            && file.extension().and_then(|ext| ext.to_str()) != Some("rs")
        {
            report
                .findings
                .push(format!("non-Rust source file under crate src tree: {rel}"));
        }
    }

    for manifest in files
        .iter()
        .filter(|path| path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml"))
    {
        let rel = relative(root, manifest);
        report.checked_crates += 1;
        let crate_dir = manifest
            .parent()
            .ok_or_else(|| format!("manifest has no parent: {}", manifest.display()))?;
        if crate_dir.join("src").is_dir() {
            report.checked_src_trees += 1;
        }
        if rel == "crates/core/Cargo.toml" {
            let text = fs::read_to_string(manifest)
                .map_err(|error| format!("failed to read {}: {error}", manifest.display()))?;
            if dependencies_section_has_entries(&text) {
                report
                    .findings
                    .push("crates/core/Cargo.toml has non-empty [dependencies]".to_string());
            }
        }
    }

    if report.checked_src_trees == 0 {
        report
            .findings
            .push("no crate src trees were discovered; verification is not meaningful".to_string());
    }

    Ok(report)
}

fn render_list(out: &mut String, title: &str, items: &[&str]) {
    out.push_str("**");
    out.push_str(title);
    out.push_str("**\n\n");
    for item in items {
        out.push_str("- ");
        out.push_str(item);
        out.push('\n');
    }
    out.push('\n');
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut out = Vec::new();
    collect_files_inner(root, root, &mut out)?;
    out.sort();
    Ok(out)
}

fn collect_files_inner(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|error| format!("failed to read {}: {error}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("failed to read directory entry: {error}"))?;
        let path = entry.path();
        let rel = relative(root, &path);
        if path.is_dir() {
            if should_skip_dir(&rel) {
                continue;
            }
            collect_files_inner(root, &path, out)?;
        } else if path.is_file() {
            out.push(path);
        }
    }
    Ok(())
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn should_skip_dir(rel: &str) -> bool {
    matches!(rel, ".git" | "target") || rel.starts_with(".idd/runs/")
}

fn src_purity_enforced(rel: &str) -> bool {
    rel.starts_with("crates/") && !rel.starts_with("crates/external/")
}

fn is_foreign_manifest(rel: &str) -> bool {
    let name = rel.rsplit('/').next().unwrap_or(rel);
    matches!(
        name,
        "package.json" | "pyproject.toml" | "go.mod" | "requirements.txt"
    ) || name.ends_with(".omc")
        || name.ends_with(".ecc")
}

fn foreign_manifest_allowed(rel: &str) -> bool {
    rel.starts_with("third_party/upstream/")
        // `imports/` holds faithfully-adopted complete owned repos (ADR-0018,
        // import-without-flattening). They legitimately carry their own foreign
        // manifests (package.json, go.mod, pyproject.toml, …) until later slices
        // reconcile them into `crates/` — same allowance as third-party mirrors.
        || rel.starts_with("imports/")
        || rel.starts_with(".github/")
        || rel.starts_with("docs/")
        || rel.starts_with("openspec/")
        || rel.starts_with(".agents/")
        // `spike/` arrived with the handoff-kernel unification (PR #143): handoff's
        // experimental spikes (e.g. ruvocal-mcp-bridge) carry their own JS manifests
        // by design — same import-without-flattening allowance until promoted.
        || rel.starts_with("spike/")
}

fn dependencies_section_has_entries(text: &str) -> bool {
    let mut in_dependencies = false;
    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with('[') && line.ends_with(']') {
            in_dependencies = line == "[dependencies]";
            continue;
        }
        if in_dependencies && !line.is_empty() && !line.starts_with('#') {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_includes_rusty_idd_gates_and_legacy_dispositions() {
        let package = package();
        assert_eq!(package.name, "Rusty IDD Merge Tool Package");
        assert!(package.phases.iter().any(|phase| phase.name == "verify"));
        assert!(package
            .legacy_surfaces
            .iter()
            .any(|surface| surface.path.contains(".claude")));
    }

    #[test]
    fn markdown_mentions_open_spec_and_ai_merge_boundary() {
        let markdown = render_markdown(&package());
        assert!(markdown.contains("rusty-idd spec status"));
        assert!(markdown.contains("AI_MERGE is evidence only"));
        assert!(markdown.contains("Legacy Surface Disposition"));
    }

    #[test]
    fn verify_workspace_checks_core_dependencies_and_foreign_manifests() {
        let root =
            std::env::temp_dir().join(format!("rusty-idd-merge-tools-test-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("crates/core/src")).unwrap();
        fs::write(root.join("crates/core/src/lib.rs"), "").unwrap();
        fs::write(
            root.join("crates/core/Cargo.toml"),
            "[package]\nname = \"rusty-idd-core\"\n\n[dependencies]\nserde = \"1\"\n",
        )
        .unwrap();
        fs::write(root.join("package.json"), "{}\n").unwrap();

        let report = verify_workspace(&root).unwrap();
        let _ = fs::remove_dir_all(&root);

        assert!(!report.is_clean());
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.contains("non-empty [dependencies]")));
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.contains("foreign package manifest")));
    }

    #[test]
    fn imports_surface_allows_foreign_manifests() {
        // Faithful-adopt imports (ADR-0018) carry their own foreign manifests
        // until later slices reconcile them — same allowance as third-party.
        assert!(foreign_manifest_allowed(
            "imports/handoff/spike/ruvocal-mcp-bridge/package.json"
        ));
        assert!(foreign_manifest_allowed("imports/prompt_hub/go.mod"));
        assert!(foreign_manifest_allowed(
            "third_party/upstream/foo/package.json"
        ));
        // A foreign manifest at the repo root is still disallowed.
        assert!(!foreign_manifest_allowed("package.json"));
    }
}
