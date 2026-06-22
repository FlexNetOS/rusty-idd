# E2E Review Upgrade Validation Evidence

- Change: `harden-e2e-validation-evidence`
- Build: `RUSTY_IDD_CHANGE=harden-e2e-validation-evidence RUSTY_IDD_GOAL_FILE=.idd/goals/e2e-review-upgrades.md rtk just ci` completed `cargo build --workspace --locked` successfully.
- Generated artifacts: refreshed `.idd/knowledge/*`,
  `docs/rusty-idd/architecture-diagrams.md`, `.idd/MANIFEST.tsv`, OpenSpec
  artifacts, ADR, task evidence, and goal-file-backed
  `.idd/knowledge/plan-context.{json,md}` before the successful test gate.
- Test: `rtk cargo test -p rusty-idd-cli --test codex_cli codex_workflow_check --locked`
  passed 14 workflow-check tests, and the successful `rtk just ci` completed
  `cargo test --workspace --locked`.
- Lint: the same successful `rtk just ci` completed
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Secret scan: changed-file scan for private key, AWS, GitHub, Slack, and
  OpenAI token patterns returned no matches.
- Manifest: the same successful `rtk just ci` completed `manifest-check`
  after `.idd/MANIFEST.tsv` regeneration.
- Spec status:
  `cargo run --bin rusty-idd -- spec status openspec/changes/harden-e2e-validation-evidence`
  passed after the OpenSpec task package was created.
- Spec validate: `cargo run --bin rusty-idd -- spec validate --all` passed.
- Rusty IDD validation: the same successful `rtk just ci` completed
  `rusty-idd validate --workspace .` with 0 critical and 0 warning.
- Runtime audit: the same successful `rtk just ci` completed
  `rusty-idd codex runtime-audit`.
- Env check: the same successful `rtk just ci` completed
  `rusty-idd codex env-check`.
- Model loop: `rusty-idd codex model-loop --execute --only gap-hunt`
  completed and recorded stale-evidence and active-change binding gaps before
  implementation.
- Supply-chain audit: the same successful `rtk just ci` completed
  `cargo audit --deny warnings`.
- Diff check: `git diff --check` passed.
- Workflow post-hook:
  `RUSTY_IDD_CHANGE=harden-e2e-validation-evidence cargo run --bin rusty-idd -- codex workflow-check --workspace . --phase post-tool --change harden-e2e-validation-evidence`
  passed.
- Delivery evidence: `codex workflow-check --phase stop` now rejects stale PR
  evidence unless it names the active change, current branch, real PR marker,
  `Base: develop`, and enabled auto-merge.

## Rollback Path

Revert ADR 0006, the `codex workflow-check` validation and PR evidence parser
changes, the new Codex CLI negative E2E tests, the
`harden-e2e-validation-evidence` OpenSpec package, the repo-local task card, and
refreshed generated artifacts.
