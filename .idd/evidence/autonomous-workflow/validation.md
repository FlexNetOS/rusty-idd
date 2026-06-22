# PromptHub Boundary Validation Evidence

- Change: `decide-prompt-hub-boundary`
- Goal file: `.idd/goals/prompt-hub-boundary-decision.md`
- Branch: `feature/prompt_hub`
- Build: `RUSTY_IDD_CHANGE=decide-prompt-hub-boundary RUSTY_IDD_GOAL_FILE=.idd/goals/prompt-hub-boundary-decision.md rtk just ci` completed `cargo build --workspace --locked` successfully.
- Generated artifacts: refreshed `.idd/knowledge/*`,
  `docs/rusty-idd/architecture-diagrams.md`, `.idd/MANIFEST.tsv`, OpenSpec
  artifacts, ADR, task evidence, AI_MERGE research evidence, and goal-file-backed
  `.idd/knowledge/plan-context.{json,md}` before the successful test gate.
- Test: the same successful `rtk just ci` completed
  `cargo test --workspace --locked`.
- Lint: the same successful `rtk just ci` completed
  `cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Secret scan: changed-file scan for private key, AWS, GitHub, Slack, and
  OpenAI token patterns returned no matches.
- Manifest: the same successful `rtk just ci` completed `manifest-check`
  after `.idd/MANIFEST.tsv` regeneration.
- Knowledge artifacts: the same successful `rtk just ci` completed freshness
  checks for knowledge index, report, architecture graph, architecture diagrams,
  system/operating/integration artifacts, and goal-file plan context.
- Spec status:
  `rtk cargo run --bin rusty-idd -- spec status openspec/changes/decide-prompt-hub-boundary`
  passed and reported the change archivable.
- Spec validate: `rtk cargo run --bin rusty-idd -- spec validate --all` passed
  with 97 valid items and 0 failed.
- Rusty IDD validation: the same successful `rtk just ci` completed
  `rusty-idd validate --workspace .` with 0 critical and 0 warning.
- Runtime audit: the same successful `rtk just ci` completed
  `rusty-idd codex runtime-audit`.
- Env check: the same successful `rtk just ci` completed
  `rusty-idd codex env-check`.
- Supply-chain audit: the same successful `rtk just ci` completed
  `cargo audit --deny warnings`.
- Diff check: `rtk git diff --check` passed.
- PromptHub native diagnostic: `rtk cargo check --workspace` passed in
  `/home/drdave/Desktop/meta/prompt_hub`.
- Workflow pre-hook:
  `RUSTY_IDD_CHANGE=decide-prompt-hub-boundary RUSTY_IDD_GOAL_FILE=.idd/goals/prompt-hub-boundary-decision.md rtk cargo run --bin rusty-idd -- codex workflow-check --workspace . --phase pre-tool --change decide-prompt-hub-boundary`
  passed.
- Workflow post-hook:
  `RUSTY_IDD_CHANGE=decide-prompt-hub-boundary RUSTY_IDD_GOAL_FILE=.idd/goals/prompt-hub-boundary-decision.md rtk cargo run --bin rusty-idd -- codex workflow-check --workspace . --phase post-tool --change decide-prompt-hub-boundary`
  passed.

## Rollback Path

Revert the PromptHub boundary goal file, OpenSpec change, ADR 0007, task card,
AI_MERGE research note, validation evidence, active-change update, and refreshed
generated artifacts. Then rerun Rusty IDD knowledge refresh, plan context,
manifest, `spec validate --all`, and `just ci`.
