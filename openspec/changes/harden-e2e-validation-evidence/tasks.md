# harden-e2e-validation-evidence - Tasks

## 1. Review and Planning

- [x] 1.1 Create the review-upgrade goal file.
- [x] 1.2 Claim a repo-local handoff task card before implementation.
- [x] 1.3 Add OpenSpec proposal, design, spec delta, and tasks.
- [x] 1.4 Add ADR for validation evidence success semantics.
- [x] 1.5 Generate goal-file-backed plan context for this review change.

## 2. Gap Hunt

- [x] 2.1 Review PR #82 workflow-check implementation and tests.
- [x] 2.2 Identify false-positive validation evidence as the primary quality gap.
- [x] 2.3 Record review findings in AI_MERGE evidence.

## 3. Implementation

- [x] 3.1 Add failing tests for failed, skipped, stale, unknown, and placeholder evidence.
- [x] 3.2 Add delivery-sensitive hook tests for `git push`, `gh pr create`, `gh pr merge`, and task completion.
- [x] 3.3 Upgrade validation evidence parsing while preserving Markdown evidence.
- [x] 3.4 Keep successful validation evidence passing.
- [x] 3.5 Require validation evidence to name the active OpenSpec change.
- [x] 3.6 Require PR evidence to name the active OpenSpec change and current branch.

## 4. Regenerate and Validate

- [x] 4.1 Refresh `.idd/knowledge` artifacts and architecture diagrams.
- [x] 4.2 Regenerate `.idd/MANIFEST.tsv`.
- [x] 4.3 Run focused Codex CLI tests.
- [x] 4.4 Run `just ci` with the review goal file and active change.
- [x] 4.5 Run OpenSpec status/validate, secret scan, and workflow-check gates.
- [ ] 4.6 Record validation, rollback, PR, and completion evidence.
