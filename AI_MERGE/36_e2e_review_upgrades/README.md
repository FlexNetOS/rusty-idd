# E2E Review Upgrade Evidence

This package records the deep code-review and gap-hunt loop for the
comprehensive Rusty IDD E2E workflow suite.

## Review Finding

| ID | Finding | Risk | Upgrade |
|---|---|---|---|
| REV-001 | Validation evidence was marker-based and did not verify success semantics. | Failed, skipped, stale, or placeholder evidence could unlock task completion, push, or PR handoff. | Add line-oriented success/failure evidence parsing and negative CLI tests for stop, push, PR, and task completion. |
| REV-002 | Validation evidence was not bound to the active change. | Successful evidence from PR #82 could unlock `harden-e2e-validation-evidence`. | Require validation evidence to include `Change: <active-change>` and add a wrong-change negative test. |
| REV-003 | PR/automerge evidence was marker-based and not bound to the active change or branch. | Previous PR evidence could satisfy dirty-work stop delivery checks for a new branch. | Require PR evidence to name the active change, current branch, real PR marker, develop base, and enabled auto-merge. |

## Research Notes

- Reviewed `crates/cli/src/commands/codex.rs` functions
  `check_validation_evidence`, `validation_evidence_is_complete`,
  `command_requires_validation`, and delivery checks.
- Reviewed `crates/cli/tests/codex_cli.rs` workflow-check fixture and PR #82
  tests.
- Queried `.idd/knowledge/index.json` for
  `crates/cli/src/commands/codex.rs` to confirm the active symbols and keep
  the review focused.
- Ran the focused Codex CLI workflow-check tests before implementation and
  observed the expected failures for failed/skipped/TODO validation evidence.
- Executed the read-only `rusty-idd-gap-hunter` model-loop pass. It confirmed
  stale validation/PR evidence, stale manifest coverage, stale knowledge index,
  and missing active-change evidence binding as delivery blockers.
- Applied the smallest upgrade: preserve Markdown evidence, parse required
  sections, reject failure-like result text, require success-like result text,
  bind validation evidence to the active change, and bind PR evidence to the
  active change and branch.

## Focused Test Evidence

Initial focused test run failed as expected:

```bash
rtk cargo test -p rusty-idd-cli --test codex_cli codex_workflow_check --locked
```

Failures proved that failed validation evidence still passed at Stop, before
`git push`, before `gh pr create`, before `gh pr merge --auto`, and before
`hf done`.

After the parser upgrade, the same command passed:

```text
cargo test: 13 passed, 6 filtered out (1 suite, 0.09s)
```

## Scope

- Change: `harden-e2e-validation-evidence`
- Task: `KBTASK-RUSTY-IDD-E2E-REVIEW-UPGRADES`
- Branch: `feature/e2e-review-upgrades`
- Worktree: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/e2e-review-upgrades`

## Evidence Order

1. Goal and task card.
2. OpenSpec and ADR.
3. Review gap evidence.
4. Failing tests.
5. Parser upgrade.
6. Generated artifact refresh.
7. Full validation.
