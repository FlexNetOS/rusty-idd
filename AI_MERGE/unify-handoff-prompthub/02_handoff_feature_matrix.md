# Feature Matrix

This matrix is generated from structural signals. Treat it as a starting point, then refine it with explicit product intent.

| Capability | Repo A Signal | Repo B Signal | Default Decision | Migration Action |
|---|---|---|---|---|
| Rust native core | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Node/TypeScript UI or tooling | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Python tooling | yes | no | Keep Repo A implementation unless tests fail | Wrap behind stable interface |
| GitHub Actions CI | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Environment contract | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Secret references | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Nix, mise, or direnv toolchain | no | no | Create only if required by product intent | No action yet |
| Agent control files | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Security policy files | yes | no | Keep Repo A implementation unless tests fail | Wrap behind stable interface |

## Shared Paths

| Path | Repo A | Repo B | Risk |
|---|---|---|---|
| `.claude/agent-guard.toml` | yes | yes | naming/API collision |
| `.claude/rules/meta-destructive-commands.md` | yes | yes | naming/API collision |
| `.claude/settings.json` | yes | yes | naming/API collision |
| `.gitattributes` | yes | yes | naming/API collision |
| `.githooks/commit-msg` | yes | yes | naming/API collision |
| `.githooks/pre-commit` | yes | yes | naming/API collision |
| `.githooks/pre-push` | yes | yes | naming/API collision |
| `.github/workflows/ci.yml` | yes | yes | naming/API collision |
| `.github/workflows/promote-verify.yml` | yes | yes | naming/API collision |
| `.github/workflows/release.yml` | yes | yes | naming/API collision |
| `.github/workflows/semantic-pr-title.yml` | yes | yes | naming/API collision |
| `.gitignore` | yes | yes | naming/API collision |
| `.handoff/context/capsule.json` | yes | yes | naming/API collision |
| `.release-please-manifest.json` | yes | yes | naming/API collision |
| `AGENTS.md` | yes | yes | naming/API collision |
| `CLAUDE.md` | yes | yes | naming/API collision |
| `CONTRIBUTING.md` | yes | yes | naming/API collision |
| `Cargo.lock` | yes | yes | naming/API collision |
| `Cargo.toml` | yes | yes | naming/API collision |
| `Makefile` | yes | yes | naming/API collision |
| `VERSION` | yes | yes | naming/API collision |
| `commitlint.config.cjs` | yes | yes | naming/API collision |
| `crates/cli/Cargo.toml` | yes | yes | naming/API collision |
| `crates/cli/src/commands/core.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/mod.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/run.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/spec.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/spec_adr.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/spec_archive.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/spec_scaffold.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/spec_status.rs` | yes | yes | naming/API collision |
| `crates/cli/src/commands/tui.rs` | yes | yes | naming/API collision |
| `crates/cli/src/lib.rs` | yes | yes | naming/API collision |
| `crates/cli/src/main.rs` | yes | yes | naming/API collision |
| `crates/cli/tests/archive_cli.rs` | yes | yes | naming/API collision |
| `crates/cli/tests/run_cli.rs` | yes | yes | naming/API collision |
| `crates/cli/tests/spec_adr_cli.rs` | yes | yes | naming/API collision |
| `crates/cli/tests/spec_cli.rs` | yes | yes | naming/API collision |
| `crates/cli/tests/spec_scaffold_cli.rs` | yes | yes | naming/API collision |
| `crates/cli/tests/spec_status_cli.rs` | yes | yes | naming/API collision |
| `crates/core/.gitignore` | yes | yes | naming/API collision |
| `crates/core/Cargo.toml` | yes | yes | naming/API collision |
| `crates/core/LICENSE` | yes | yes | naming/API collision |
| `crates/core/README.md` | yes | yes | naming/API collision |
| `crates/core/docs/AGENT_WORKFLOW.md` | yes | yes | naming/API collision |
| `crates/core/docs/ARCHITECTURE.md` | yes | yes | naming/API collision |
| `crates/core/docs/audits/GAP_AUDIT_v2.md` | yes | yes | naming/API collision |
| `crates/core/examples/README.md` | yes | yes | naming/API collision |
| `crates/core/rust-toolchain.toml` | yes | yes | naming/API collision |
| `crates/core/scripts/package.sh` | yes | yes | naming/API collision |
| `crates/core/src/cli.rs` | yes | yes | naming/API collision |
| `crates/core/src/env_contract.rs` | yes | yes | naming/API collision |
| `crates/core/src/fs_utils.rs` | yes | yes | naming/API collision |
| `crates/core/src/lib.rs` | yes | yes | naming/API collision |
| `crates/core/src/manifest.rs` | yes | yes | naming/API collision |
| `crates/core/src/model.rs` | yes | yes | naming/API collision |
| `crates/core/src/planner.rs` | yes | yes | naming/API collision |
| `crates/core/src/scanner.rs` | yes | yes | naming/API collision |
| `crates/core/src/templates.rs` | yes | yes | naming/API collision |
| `crates/core/src/validation.rs` | yes | yes | naming/API collision |
| `crates/core/tests/smoke.rs` | yes | yes | naming/API collision |
| `crates/runner/Cargo.toml` | yes | yes | naming/API collision |
| `crates/runner/src/config.rs` | yes | yes | naming/API collision |
| `crates/runner/src/data.rs` | yes | yes | naming/API collision |
| `crates/runner/src/lib.rs` | yes | yes | naming/API collision |
| `crates/runner/src/runner.rs` | yes | yes | naming/API collision |
| `crates/spec/Cargo.toml` | yes | yes | naming/API collision |
| `crates/spec/src/adr/mod.rs` | yes | yes | naming/API collision |
| `crates/spec/src/archive/mod.rs` | yes | yes | naming/API collision |
| `crates/spec/src/lib.rs` | yes | yes | naming/API collision |
| `crates/spec/src/model/block.rs` | yes | yes | naming/API collision |
| `crates/spec/src/model/delta.rs` | yes | yes | naming/API collision |
| `crates/spec/src/model/merge.rs` | yes | yes | naming/API collision |
| `crates/spec/src/model/mod.rs` | yes | yes | naming/API collision |
| `crates/spec/src/model/requirement.rs` | yes | yes | naming/API collision |
| `crates/spec/src/model/spec.rs` | yes | yes | naming/API collision |
| `crates/spec/src/parse/common.rs` | yes | yes | naming/API collision |
| `crates/spec/src/parse/delta_parser.rs` | yes | yes | naming/API collision |
| `crates/spec/src/parse/emit.rs` | yes | yes | naming/API collision |
| `crates/spec/src/parse/mod.rs` | yes | yes | naming/API collision |
| `crates/spec/src/parse/spec_parser.rs` | yes | yes | naming/API collision |
| `crates/spec/src/scaffold/mod.rs` | yes | yes | naming/API collision |
| `crates/spec/src/schema/graph.rs` | yes | yes | naming/API collision |
| `crates/spec/src/schema/mod.rs` | yes | yes | naming/API collision |
| `crates/spec/src/validate/mod.rs` | yes | yes | naming/API collision |
| `crates/spec/src/validate/report.rs` | yes | yes | naming/API collision |
| `crates/spec/src/validate/rules.rs` | yes | yes | naming/API collision |
| `crates/spec/tests/archive_direct.rs` | yes | yes | naming/API collision |
| `crates/spec/tests/archive_golden.rs` | yes | yes | naming/API collision |
| `crates/spec/tests/parse_emit_direct.rs` | yes | yes | naming/API collision |
| `crates/spec/tests/validate_golden.rs` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/apply.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/archive.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/bulk-archive.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/continue.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/explore.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/ff.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/new.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/onboard.md` | yes | yes | naming/API collision |
| `crates/tui/.claude/commands/opsx/propose.md` | yes | yes | naming/API collision |
