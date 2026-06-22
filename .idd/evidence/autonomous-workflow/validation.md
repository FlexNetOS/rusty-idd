# Rusty IDD + Handoff Single-Repo Planning Validation Evidence

- Build: `RUSTY_IDD_CHANGE=plan-handoff-single-repo-architecture RUSTY_IDD_GOAL_FILE=.idd/goals/rusty-idd-handoff-single-repo.md rtk just ci` completed `cargo build --workspace --locked`.
- Test: the same `rtk just ci` completed `cargo test --workspace --locked`.
- Lint: the same `rtk just ci` completed `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Format: the same `rtk just ci` completed `cargo fmt --all -- --check`.
- Manifest: the same `rtk just ci` completed `manifest-check`; final generation wrote 2814 manifest entries.
- Knowledge artifacts: the same `rtk just ci` completed `knowledge-check`, `diagrams-check`, `operating-model-check`, `integration-plan-check`, `integration-status-check`, `integration-owners-check`, `integration-readiness-check`, and goal-file-backed `plan-context-check`.
- Spec status: `cargo run --quiet --bin rusty-idd -- spec status openspec/changes/plan-handoff-single-repo-architecture` reported all 5 artifacts done and ready to archive.
- Spec validate: `cargo run --quiet --bin rusty-idd -- spec validate --all` reported 75 passed, 0 failed.
- Rusty IDD validation: `cargo run --quiet --bin rusty-idd -- validate --workspace .` reported 0 critical and 0 warning.
- Runtime audit: the same `rtk just ci` completed `rusty-idd codex runtime-audit`.
- Env check: the same `rtk just ci` completed `rusty-idd codex env-check`.
- Model loop: the same `rtk just ci` completed `rusty-idd codex model-loop`.
- Supply-chain audit: the same `rtk just ci` completed `cargo audit --deny warnings`, loading 1134 advisories and scanning 496 crate dependencies.
- Diff check: `git diff --check` passed.
- Workflow post-hook: `cargo run --quiet --bin rusty-idd -- codex workflow-check --workspace . --phase post-tool --change plan-handoff-single-repo-architecture` passed.
- Secret scan: changed-file scan for private key, AWS, GitHub, Slack, and OpenAI token patterns returned `secret_scan:no_matches`.

## Gate Fix Evidence

The first `rtk just ci` run failed in `plan-context-check` because the recipe
embedded the Markdown goal in shell double quotes. Backticked strings inside the
goal were interpreted as commands before reaching `rusty-idd`. The `Justfile`
now accepts `RUSTY_IDD_GOAL_FILE` and passes `--goal-file` through the check,
preserving the required goal-file workflow without shell re-interpretation.

## Migration Note

Old path: Rusty IDD and handoff planning lived in separate repo-level control
planes, and handoff carried an older embedded Rusty IDD subset.

New path: this planning package records handoff as the outer canonical repo,
with current Rusty IDD embedded as peer workspace packages and root planning
artifacts during the implementation phases.

## Rollback Path

Revert this planning change set. No handoff code has moved in this PR, and no
runtime data migration is required.
