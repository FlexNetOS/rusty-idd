# harden-e2e-validation-evidence

## Why

The comprehensive E2E workflow test suite merged in PR #82 made test evidence
mandatory after generated artifact refresh, before task completion, push, and PR
handoff. The review loop found that the current checker still accepts marker
words such as `Build:` and `Test:` without validating whether those entries
represent successful evidence.

That is too weak for a professional code-grade workflow suite. A failed or
placeholder validation line must not unlock `git push`, PR handoff, or task
completion.

## What Changes

- Add a goal-file-backed review upgrade change for the E2E workflow suite.
- Record the review gap in AI_MERGE evidence.
- Strengthen validation evidence parsing so each required evidence section must
  be present, ordered, and success-like.
- Reject failed, placeholder, skipped, missing, or unknown validation results.
- Require validation evidence to name the active OpenSpec change so prior-task
  evidence cannot unlock a new push, PR, merge, or task completion.
- Require PR evidence to name the active OpenSpec change and feature branch so
  prior PR evidence cannot satisfy dirty-work Stop/SubagentStop handoff.
- Extend CLI tests to cover false-positive evidence for stop, push, PR, and
  task-completion paths.
- Refresh generated knowledge, diagram, and manifest artifacts.

## Capabilities

### Modified Capabilities

- `codex-harness-flow`: requires success-like validation evidence before
  delivery-sensitive commands and active-change-bound PR evidence for dirty-work
  Stop/SubagentStop handoff.
- `comprehensive-e2e-workflow-tests`: covers negative evidence cases instead of
  only marker presence and ordering.

## Impact

- `crates/cli/src/commands/codex.rs`
- `crates/cli/tests/codex_cli.rs`
- `.idd/knowledge/*`
- `.idd/MANIFEST.tsv`
- `docs/rusty-idd/architecture-diagrams.md`
- `openspec/changes/harden-e2e-validation-evidence/*`
- `adr/0006-validation-evidence-success-semantics.md`
- `AI_MERGE/36_e2e_review_upgrades/*`
