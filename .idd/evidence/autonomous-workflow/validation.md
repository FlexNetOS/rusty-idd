# Autonomous Workflow Hook Validation Evidence

- Build: `RUSTY_IDD_CHANGE=add-autonomous-workflow-hooks RUSTY_IDD_GOAL="Add pre and post hooks that enforce the full autonomous Rusty IDD workflow from a develop-based worktree through tracked task claim, OpenSpec readiness, validation, PR branch push, and auto-merge into develop." rtk just ci` passed the build step (`cargo build --workspace --locked`).
- Test: the same `rtk just ci` pass completed `cargo test --workspace --locked`; focused check `rtk cargo test -p rusty-idd-cli --test codex_cli codex_workflow_check -- --nocapture` passed 3 tests; full `rtk cargo test -p rusty-idd-cli --test codex_cli -- --nocapture` passed 9 tests.
- Lint: the same `rtk just ci` pass completed `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Format: `rtk cargo fmt --all --check` passed.
- Spec status: `rtk cargo run --quiet --bin rusty-idd -- spec status openspec/changes/add-autonomous-workflow-hooks` reported all 5 artifacts done and ready to archive.
- Spec validate: `rtk cargo run --quiet --bin rusty-idd -- spec validate --all` reported 63 passed, 0 failed.
- Runtime audit: `rtk cargo run --quiet --bin rusty-idd -- codex runtime-audit --workspace .` reported 0 live Python commands and 0 obsolete Python tool files.
- Env check: `rtk cargo run --quiet --bin rusty-idd -- codex env-check --workspace .` passed.
- Workflow pre-hook: `rtk cargo run --quiet --bin rusty-idd -- codex workflow-check --workspace . --phase pre-tool` passed.
- Workflow post-hook: `rtk cargo run --quiet --bin rusty-idd -- codex workflow-check --workspace . --phase post-tool` passed.
- Manifest: `rtk just manifest` refreshed `.idd/MANIFEST.tsv`; the final `rtk just ci` pass completed `manifest-check`.
- Knowledge artifacts: `rtk just knowledge`, `rtk just operating-model`, `rtk just integration-plan`, `rtk just integration-status`, `rtk just integration-owners`, `rtk just integration-readiness`, and `rtk just plan-context` refreshed deterministic artifacts; the final `rtk just ci` pass completed the corresponding checks.
- Secret scan: `rtk rg -n "(BEGIN (RSA|OPENSSH|EC|DSA) PRIVATE KEY|AKIA[0-9A-Z]{16}|github_pat_[A-Za-z0-9_]+|ghp_[A-Za-z0-9_]+|xox[baprs]-[A-Za-z0-9-]+|sk-[A-Za-z0-9]{20,})" <changed-files>` returned no matches.
- Supply-chain audit: the final `rtk just ci` pass reached `cargo audit` and failed only because the sandbox could not take the advisory DB lock under `~/.cargo`; rerun as `rtk cargo audit --deny warnings` with escalation passed, loading 1134 advisories and scanning 496 crate dependencies.

## Migration Note

Old path: `.codex/hooks.json` only ran `rusty-idd codex env-check` on Stop.

New path: `.codex/hooks.json` runs `rusty-idd codex workflow-check` on
PreToolUse, PostToolUse, Stop, and SubagentStop, and still runs `codex
env-check` on Stop.

## Rollback Path

Revert this change set to restore the prior Stop-only `codex env-check` hook
surface. No data migration is required; the new `.idd/evidence` and
`.idd/workflow/active-change` files are workflow evidence/pointers only.
