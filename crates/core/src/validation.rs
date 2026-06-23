use crate::fs_utils::{
    read_to_string_lossy, relative_path, stable_walk, write_string_preserving_existing,
};
use crate::manifest::workspace_fingerprint;
use crate::model::{FindingSeverity, ValidationFinding};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const MAX_SCAN_BYTES: u64 = 512 * 1024;

pub fn validate_workspace(path: impl AsRef<Path>) -> Result<Vec<ValidationFinding>, String> {
    let root = path.as_ref();
    if !root.exists() || !root.is_dir() {
        return Err(format!(
            "workspace path is not a directory: {}",
            root.display()
        ));
    }

    let mut findings = Vec::new();
    require_file(root, "AGENTS.md", &mut findings);
    require_file(root, ".env.schema.example.json", &mut findings);
    require_file(root, ".env.contract.yaml", &mut findings);
    require_file(root, ".github/copilot-instructions.md", &mut findings);
    require_file(root, ".github/pull_request_template.md", &mut findings);
    require_file(root, ".github/ISSUE_TEMPLATE/idd-task.yml", &mut findings);
    require_file(root, ".github/CODEOWNERS", &mut findings);
    require_file(root, ".github/dependabot.yml", &mut findings);
    require_file(root, ".github/workflows/ci.yml", &mut findings);
    require_file(root, ".github/workflows/promote-verify.yml", &mut findings);
    require_file(
        root,
        ".github/workflows/semantic-pr-title.yml",
        &mut findings,
    );
    require_file(root, ".github/workflows/on-push-main.yml", &mut findings);
    require_file(root, ".github/workflows/release.yml", &mut findings);
    require_file(root, ".idd/LOCK.md", &mut findings);
    require_file(root, ".idd/MANIFEST.tsv", &mut findings);
    require_file(root, "VERSION", &mut findings);
    require_file(root, "CONTRIBUTING.md", &mut findings);
    require_file(root, "Makefile", &mut findings);
    require_file(root, "Justfile", &mut findings);
    require_file(root, "commitlint.config.cjs", &mut findings);
    require_file(root, "renovate.json", &mut findings);
    require_file(root, "release-please-config.json", &mut findings);
    require_file(root, ".release-please-manifest.json", &mut findings);
    require_file(root, ".claude/agent-guard.toml", &mut findings);
    require_file(
        root,
        ".claude/rules/meta-destructive-commands.md",
        &mut findings,
    );
    require_file(root, "AI_MERGE/04_merge_plan.md", &mut findings);
    require_file(
        root,
        "AI_MERGE/03_env_and_secret_contracts.md",
        &mut findings,
    );
    require_file(root, "AI_MERGE/08_agent_queue.md", &mut findings);
    require_file(root, "AI_MERGE/10_parity_test_plan.md", &mut findings);
    require_file(
        root,
        "crates/external/codegraph-core/LICENSE-MIT",
        &mut findings,
    );
    require_file(
        root,
        "crates/external/codegraph-core/LICENSE-APACHE",
        &mut findings,
    );
    require_file(
        root,
        "crates/external/codegraph-parser/LICENSE-MIT",
        &mut findings,
    );
    require_file(
        root,
        "crates/external/codegraph-parser/LICENSE-APACHE",
        &mut findings,
    );
    require_file(
        root,
        "crates/external/repomix-shared/LICENSE-MIT",
        &mut findings,
    );

    let secret_allowlist = load_secret_allowlist(root);
    for abs in stable_walk(root).map_err(|e| format!("walk failed: {e}"))? {
        let rel = relative_path(root, &abs);
        if validation_scan_should_skip(&rel) {
            continue;
        }
        flag_committed_env_file(&rel, &mut findings);
        let content = read_to_string_lossy(&abs, MAX_SCAN_BYTES).unwrap_or_default();
        if content.is_empty() {
            continue;
        }
        if !is_secret_allowlisted(&rel, &secret_allowlist) {
            scan_secret_patterns(&rel, &content, &mut findings);
        }
        if is_github_workflow(&rel) {
            scan_workflow_risks(&rel, &content, &mut findings);
            scan_workflow_policy(&rel, &content, &mut findings);
        }
        if rel == ".idd/MANIFEST.tsv" {
            scan_manifest_policy(&rel, &content, &mut findings);
        }
    }

    scan_knowledge_staleness(root, &mut findings);

    findings.sort_by(|a, b| b.severity.cmp(&a.severity).then(a.file.cmp(&b.file)));
    Ok(findings)
}

pub fn write_validation_report(
    workspace: impl AsRef<Path>,
    report_path: impl AsRef<Path>,
) -> Result<Vec<ValidationFinding>, String> {
    let findings = validate_workspace(workspace)?;
    let mut out =
        String::from("# IDD Validation Report\n\n| Severity | File | Finding |\n|---|---|---|\n");
    if findings.is_empty() {
        out.push_str("| info | _workspace_ | No findings |\n");
    } else {
        for finding in &findings {
            out.push_str(&format!(
                "| {} | `{}` | {} |\n",
                finding.severity, finding.file, finding.message
            ));
        }
    }
    write_string_preserving_existing(report_path.as_ref(), &out)
        .map_err(|e| format!("failed to write validation report: {e}"))?;
    Ok(findings)
}

fn require_file(root: &Path, rel: &str, findings: &mut Vec<ValidationFinding>) {
    if !root.join(rel).exists() {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Warning,
            file: rel.to_string(),
            message: "required IDD/GitHub control-plane file is missing".to_string(),
        });
    }
}

fn validation_scan_should_skip(rel: &str) -> bool {
    rel.contains(".git/") || rel.starts_with("target/") || rel.starts_with("third_party/upstream/")
}

/// Load secret-scan allowlist entries from `.idd/secret-allowlist.txt` (one path
/// substring per line; `#` comments and blanks ignored). A file whose relative
/// path contains any entry is exempt from secret-pattern findings — used to
/// allowlist placeholder/detection-regex/test-fixture matches (e.g. a secret-
/// DETECTION module's own regexes), NOT to skip scanning for real secrets
/// elsewhere. Returns an empty list if the file is absent.
fn load_secret_allowlist(root: &Path) -> Vec<String> {
    std::fs::read_to_string(root.join(".idd/secret-allowlist.txt"))
        .map(|s| {
            s.lines()
                .map(str::trim)
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn is_secret_allowlisted(rel: &str, allowlist: &[String]) -> bool {
    allowlist.iter().any(|entry| rel.contains(entry.as_str()))
}

fn flag_committed_env_file(file: &str, findings: &mut Vec<ValidationFinding>) {
    let lower = file.to_ascii_lowercase();
    let name = Path::new(file)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(file)
        .to_ascii_lowercase();

    let allowed = name == ".env.example"
        || name == "env.example"
        || name == ".env.schema.example.json"
        || name == ".env.contract.yaml"
        || lower.ends_with(".sample")
        || lower.ends_with(".template");

    if name.starts_with(".env") && !allowed {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: file.to_string(),
            message: "committed dotenv file detected; keep real .env files local or encrypted"
                .to_string(),
        });
    }
}

fn scan_secret_patterns(file: &str, content: &str, findings: &mut Vec<ValidationFinding>) {
    let lower_file = file.to_ascii_lowercase();
    let is_allowed_example = lower_file.ends_with(".env.example")
        || lower_file.ends_with("env.schema.example.json")
        || lower_file.contains("/examples/")
        || lower_file.starts_with("examples/")
        || lower_file.ends_with(".md");

    for (line_no, line) in content.lines().enumerate() {
        let lower = line.to_ascii_lowercase();
        let critical = [
            ["-----begin ", "private key-----"].concat(),
            ["aws_secret", "_access_key="].concat(),
            ["gh", "p_"].concat(),
            ["github", "_pat_"].concat(),
            ["xo", "xb-"].concat(),
            ["sk", "-proj-"].concat(),
            ["sk", "-live-"].concat(),
            ["-----begin open", "ssh private key-----"].concat(),
        ];
        if critical
            .iter()
            .any(|needle| lower.contains(needle.as_str()))
            && !is_allowed_example
        {
            findings.push(ValidationFinding {
                severity: FindingSeverity::Critical,
                file: file.to_string(),
                message: format!("possible committed secret near line {}", line_no + 1),
            });
        }

        let warning = [
            "=drowssap".chars().rev().collect::<String>(),
            "=yek_ipa".chars().rev().collect::<String>(),
            "=yekipa".chars().rev().collect::<String>(),
            "=terces".chars().rev().collect::<String>(),
            "=nekot".chars().rev().collect::<String>(),
            "=terces_tneilc".chars().rev().collect::<String>(),
        ];
        if warning.iter().any(|needle| lower.contains(needle.as_str())) && !is_allowed_example {
            findings.push(ValidationFinding {
                severity: FindingSeverity::Warning,
                file: file.to_string(),
                message: format!("secret-like assignment near line {}", line_no + 1),
            });
        }
    }
}

fn scan_workflow_risks(file: &str, content: &str, findings: &mut Vec<ValidationFinding>) {
    scan_duplicate_yaml_keys(file, content, findings);

    for (line_no, line) in content.lines().enumerate() {
        let lower = line.trim().to_ascii_lowercase();
        if lower.starts_with("pull_request_target:") || lower == "pull_request_target" {
            findings.push(ValidationFinding {
                severity: FindingSeverity::Warning,
                file: file.to_string(),
                message: format!(
                    "pull_request_target requires explicit threat review near line {}",
                    line_no + 1
                ),
            });
        }
        if lower.contains("permissions:") && lower.contains("write-all") {
            findings.push(ValidationFinding {
                severity: FindingSeverity::Critical,
                file: file.to_string(),
                message: format!(
                    "workflow uses write-all permissions near line {}",
                    line_no + 1
                ),
            });
        }
        if lower.starts_with("if:") && lower.contains("secrets.") {
            findings.push(ValidationFinding {
                severity: FindingSeverity::Warning,
                file: file.to_string(),
                message: format!(
                    "workflow references secrets directly in if conditional near line {}",
                    line_no + 1
                ),
            });
        }
    }
}

fn scan_duplicate_yaml_keys(file: &str, content: &str, findings: &mut Vec<ValidationFinding>) {
    let mut seen_by_indent: BTreeMap<usize, BTreeSet<String>> = BTreeMap::new();
    let mut block_scalar_indent = None;

    for (line_no, raw_line) in content.lines().enumerate() {
        let trimmed = raw_line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = raw_line.len() - raw_line.trim_start_matches(' ').len();
        if let Some(block_indent) = block_scalar_indent {
            if indent > block_indent {
                continue;
            }
            block_scalar_indent = None;
        }

        seen_by_indent.retain(|scope_indent, _| *scope_indent <= indent);

        let (scope_indent, body) = if let Some(item) = trimmed.strip_prefix("- ") {
            seen_by_indent.retain(|scope_indent, _| *scope_indent <= indent);
            (indent + 2, item.trim())
        } else {
            (indent, trimmed)
        };

        let Some((key, value)) = body.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if key.is_empty()
            || !key
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
        {
            continue;
        }

        let seen = seen_by_indent.entry(scope_indent).or_default();
        if !seen.insert(key.to_string()) {
            findings.push(ValidationFinding {
                severity: FindingSeverity::Critical,
                file: file.to_string(),
                message: format!("duplicate YAML key `{key}` near line {}", line_no + 1),
            });
        }

        let value = value.trim();
        if value == "|" || value == ">" || value == "|-" || value == ">-" {
            block_scalar_indent = Some(scope_indent);
        }
    }
}

fn scan_workflow_policy(file: &str, content: &str, findings: &mut Vec<ValidationFinding>) {
    let compact = content
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join("\n");

    if file.ends_with("/idd-ci.yml") {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: file.to_string(),
            message: "legacy idd-ci workflow must be consolidated into .github/workflows/ci.yml"
                .to_string(),
        });
    }

    if content.contains("dtolnay/rust-toolchain@") {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: file.to_string(),
            message:
                "workflow must use scripts/ci/envctl-rust-env.sh instead of dtolnay/rust-toolchain"
                    .to_string(),
        });
    }

    if content.contains("Swatinem/rust-cache@") {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: file.to_string(),
            message:
                "workflow must use explicit meta-owned cache paths instead of Swatinem/rust-cache"
                    .to_string(),
        });
    }

    if file.ends_with("/ci.yml") {
        require_workflow_contains(
            file,
            &compact,
            "branches: [main, develop]",
            "primary CI must run on both main and develop pushes",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "scripts/ci/envctl-rust-env.sh ci",
            "primary CI must materialize the envctl-owned Rust toolchain/cache",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "scripts/ci/envctl-rust-audit.sh",
            "primary CI must audit the actual envctl-owned Rust compiler/cache surface",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "cargo run --bin rusty-idd -- merge-tools verify --workspace .",
            "primary CI must run the merge-tools verification gate",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "cargo run --bin rusty-idd -- validate --workspace .",
            "primary CI must run rusty-idd validate",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "git diff --exit-code -- .idd/MANIFEST.tsv",
            "primary CI must fail when manifest generation changes .idd/MANIFEST.tsv",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "cargo audit --deny warnings",
            "primary CI must deny new cargo-audit warnings",
            findings,
        );
    }

    if file.ends_with("/promote-verify.yml") {
        require_workflow_contains(
            file,
            content,
            "scripts/ci/envctl-rust-env.sh promote",
            "promotion verification must materialize the envctl-owned Rust toolchain/cache",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "scripts/ci/envctl-rust-audit.sh",
            "promotion verification must audit the actual envctl-owned Rust compiler/cache surface",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "cargo run --bin rusty-idd -- merge-tools verify --workspace .",
            "promotion verification must run the merge-tools verification gate",
            findings,
        );
        require_workflow_contains(
            file,
            content,
            "cargo audit --deny warnings",
            "promotion verification must deny new cargo-audit warnings",
            findings,
        );
    }

    if file.ends_with("/release.yml") {
        require_workflow_contains(
            file,
            content,
            "scripts/ci/envctl-rust-env.sh release",
            "release workflow must materialize the envctl-owned Rust toolchain",
            findings,
        );
    }
}

fn require_workflow_contains(
    file: &str,
    content: &str,
    needle: &str,
    message: &str,
    findings: &mut Vec<ValidationFinding>,
) {
    if !content.contains(needle) {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: file.to_string(),
            message: message.to_string(),
        });
    }
}

fn scan_manifest_policy(file: &str, content: &str, findings: &mut Vec<ValidationFinding>) {
    for (line_no, line) in content.lines().enumerate().skip(1) {
        let path = line.split('\t').next().unwrap_or_default();
        if manifest_path_is_local_artifact(path) {
            findings.push(ValidationFinding {
                severity: FindingSeverity::Critical,
                file: file.to_string(),
                message: format!(
                    "manifest includes local/generated artifact `{path}` near line {}",
                    line_no + 1
                ),
            });
        }
    }
}

fn manifest_path_is_local_artifact(path: &str) -> bool {
    path.contains(".idd-bak-")
        || path.starts_with("_workspace/")
        || path.starts_with(".devin/")
        || path.starts_with(".worktrees/")
        || path.starts_with(".idd/runs/")
        || path.starts_with(".vscode/")
        || path == ".github/workflows/idd-ci.yml"
}

fn scan_knowledge_staleness(root: &Path, findings: &mut Vec<ValidationFinding>) {
    let knowledge_dir = root.join(".idd/knowledge");
    let index = knowledge_dir.join("index.json");
    let report = knowledge_dir.join("report.md");
    if !knowledge_dir.exists() && !index.exists() && !report.exists() {
        return;
    }

    if !index.exists() {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: ".idd/knowledge/index.json".to_string(),
            message: "knowledge index is missing; run `rusty-idd knowledge refresh --workspace .`"
                .to_string(),
        });
        return;
    }
    if !report.exists() {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: ".idd/knowledge/report.md".to_string(),
            message: "knowledge report is missing; run `rusty-idd knowledge refresh --workspace .`"
                .to_string(),
        });
        return;
    }

    let Ok(fingerprint) = workspace_fingerprint(root) else {
        return;
    };
    let index_content = std::fs::read_to_string(&index).unwrap_or_default();
    let report_content = std::fs::read_to_string(&report).unwrap_or_default();

    if !index_content.contains(&fingerprint) {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: ".idd/knowledge/index.json".to_string(),
            message: "knowledge index is stale; run `rusty-idd knowledge refresh --workspace .`"
                .to_string(),
        });
    }
    if !report_content.contains(&fingerprint) {
        findings.push(ValidationFinding {
            severity: FindingSeverity::Critical,
            file: ".idd/knowledge/report.md".to_string(),
            message: "knowledge report is stale; run `rusty-idd knowledge refresh --workspace .`"
                .to_string(),
        });
    }
}

fn is_github_workflow(file: &str) -> bool {
    file.starts_with(".github/workflows/") && (file.ends_with(".yml") || file.ends_with(".yaml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_secretish_lines() {
        let mut findings = Vec::new();
        let fake_secret = format!("let x = \"{}abcdef\";", ["gh", "p_"].concat());
        scan_secret_patterns("src/main.rs", &fake_secret, &mut findings);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, FindingSeverity::Critical);
    }

    #[test]
    fn flags_committed_env_file() {
        let mut findings = Vec::new();
        flag_committed_env_file(".env.production", &mut findings);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, FindingSeverity::Critical);
    }

    #[test]
    fn secret_allowlist_exempts_listed_placeholder_files() {
        // An allowlisted detection-module/test-fixture path is exempt; other
        // files are still scanned (allowlist exempts placeholders, not real
        // secrets elsewhere).
        let allow = vec!["imports/prompt_hub/prompt-hub/src/privacy.rs".to_string()];
        assert!(is_secret_allowlisted(
            "imports/prompt_hub/prompt-hub/src/privacy.rs",
            &allow
        ));
        assert!(!is_secret_allowlisted("crates/cli/src/lib.rs", &allow));
        // Empty allowlist exempts nothing.
        assert!(!is_secret_allowlisted("anything.rs", &[]));
    }

    #[test]
    fn allows_env_contract_file() {
        let mut findings = Vec::new();
        flag_committed_env_file(".env.contract.yaml", &mut findings);
        assert!(findings.is_empty());
    }

    #[test]
    fn flags_duplicate_workflow_keys() {
        let mut findings = Vec::new();
        let workflow = r#"
name: example
jobs:
  test:
    steps:
      - name: Broken
        run: echo first
        run: echo second
"#;
        scan_workflow_risks(
            ".github/workflows/promote-verify.yml",
            workflow,
            &mut findings,
        );
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, FindingSeverity::Critical);
        assert!(findings[0].message.contains("duplicate YAML key `run`"));
    }

    #[test]
    fn flags_manifest_local_artifacts() {
        let mut findings = Vec::new();
        let manifest =
            "path\tsize_bytes\tfnv1a64\n_workspace/HANDOFF.md\t1\tabc\nsrc/lib.rs\t2\tdef\n";
        scan_manifest_policy(".idd/MANIFEST.tsv", manifest, &mut findings);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, FindingSeverity::Critical);
    }

    #[test]
    fn skips_full_upstream_mirror_policy_scan() {
        assert!(validation_scan_should_skip(
            "third_party/upstream/repomix-rs/crates/core/tests/integration_test.rs"
        ));
        assert!(!validation_scan_should_skip("crates/core/src/lib.rs"));
    }

    #[test]
    fn flags_stale_knowledge_artifacts() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        std::fs::create_dir_all(root.join(".idd/knowledge")).unwrap();
        std::fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        std::fs::write(
            root.join(".idd/knowledge/index.json"),
            "{\"workspace_fingerprint\":\"old\"}\n",
        )
        .unwrap();
        std::fs::write(root.join(".idd/knowledge/report.md"), "fingerprint: old\n").unwrap();

        let mut findings = Vec::new();
        scan_knowledge_staleness(root, &mut findings);

        assert_eq!(findings.len(), 2);
        assert!(findings
            .iter()
            .all(|finding| finding.severity == FindingSeverity::Critical));
    }
}
