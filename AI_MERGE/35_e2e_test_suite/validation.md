# Comprehensive E2E Workflow Validation Evidence

- Build: `RUSTY_IDD_CHANGE=add-comprehensive-e2e-workflow-tests RUSTY_IDD_GOAL_FILE=.idd/goals/comprehensive-e2e-test-suite.md rtk just ci` completed `cargo build --workspace --locked`.
- Generated artifacts: before the successful test gate, refreshed
  `.idd/knowledge/*`, `docs/rusty-idd/architecture-diagrams.md`,
  `.idd/MANIFEST.tsv`, OpenSpec artifacts, ADR, task evidence, and
  goal-file-backed `.idd/knowledge/plan-context.{json,md}`.
- Test: the same successful `rtk just ci` completed
  `cargo test --workspace --locked`, including the new Codex workflow E2E tests
  for post-artifact test ordering, push validation, and task-completion
  validation.
- Lint: the same successful `rtk just ci` completed
  `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Format: the same successful `rtk just ci` completed
  `cargo fmt --all -- --check`.
- Secret scan: changed-file scan for private key, AWS, GitHub, Slack, and
  OpenAI token patterns returned no matches.
- Manifest: the same successful `rtk just ci` completed `manifest-check`;
  final generation wrote 2855 manifest entries.
- Knowledge artifacts: the same successful `rtk just ci` completed
  `knowledge-check`, `diagrams-check`, `operating-model-check`,
  `integration-plan-check`, `integration-status-check`,
  `integration-owners-check`, `integration-readiness-check`, and
  goal-file-backed `plan-context-check`.
- Spec status:
  `cargo run --bin rusty-idd -- spec status openspec/changes/add-comprehensive-e2e-workflow-tests`
  reported all 5 artifacts done and ready to archive.
- Spec validate: `cargo run --bin rusty-idd -- spec validate --all` reported
  83 passed, 0 failed.
- Rusty IDD validation: the same successful `rtk just ci` completed
  `rusty-idd validate --workspace .` with 0 critical and 0 warning.
- Runtime audit: the same successful `rtk just ci` completed
  `rusty-idd codex runtime-audit`.
- Env check: the same successful `rtk just ci` completed
  `rusty-idd codex env-check`.
- Model loop: the same successful `rtk just ci` completed
  `rusty-idd codex model-loop`.
- Supply-chain audit: the same successful `rtk just ci` completed
  `cargo audit --deny warnings`, loading 1134 advisories and scanning 496 crate
  dependencies.
- Diff check: `git diff --check` passed.
- Workflow post-hook:
  `RUSTY_IDD_CHANGE=add-comprehensive-e2e-workflow-tests cargo run --bin rusty-idd -- codex workflow-check --workspace . --phase post-tool --change add-comprehensive-e2e-workflow-tests`
  passed.

## Gate Fix Evidence

The first focused E2E test run found that `hf done` was classified as read-only,
which skipped the pre-tool workflow gate. The fix classifies `hf done`,
`handoff done`, task-completion strings, and PR handoff commands as write
intents so validation evidence is required before those commands proceed.

The first full `rtk just ci` run failed because `validate` rewrote
`AI_MERGE/validation_report.md` after the manifest had been generated. The
manifest was refreshed after validation output and the full gate passed.

The second full `rtk just ci` run reached `fmt-check` and failed on rustfmt
wrapping in the new test. `cargo fmt --all` was applied, generated artifacts
were refreshed again, and the full gate passed.

## Task Evidence

- KB task: `tasks/rusty-idd-comprehensive-e2e-workflow-tests`
- KB id: `019eed57-ab2a-7431-9e11-1c97dd1393fb`
- Repo-local task card:
  `.handoff/tasks/rusty-idd-comprehensive-e2e-workflow-tests.task.json`

`rtk hf task mint --from-kb` could not resolve the parent meta `.kb` from either
the Rusty IDD feature worktree or `/home/drdave/Desktop/meta`, so the durable
repo-local `.handoff/tasks/*.task.json` card is used as the workflow-check task
evidence for this slice.

## Migration Note

Old path: workflow completion evidence allowed generated artifacts, tests, and
push handoff to be recorded as loose labels, and push/task-completion commands
could run without requiring validation evidence first.

New path: validation evidence must list generated artifacts before tests, and
push/task-completion commands require validation evidence before they proceed.

## Rollback Path

Revert ADR 0005, the `codex workflow-check` validation-evidence changes, the
new Codex CLI E2E tests, the repo-local task card, and this OpenSpec/evidence
package. Existing build, test, lint, audit, manifest, knowledge, diagram, and
validation commands remain available.
