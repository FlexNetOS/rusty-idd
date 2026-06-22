# harden-e2e-validation-evidence - Design

## Context

PR #82 introduced mandatory post-artifact test evidence. The implementation
checks required section labels and verifies that `Test:` appears after
`Generated artifacts:`. That proves ordering, but not success semantics.

In practice, autonomous agents could write:

```text
Build: failed
Generated artifacts: skipped
Test: failed
Lint: unknown
Secret scan: not run
Manifest: stale
```

and still satisfy the marker-based parser. The same review found that stale PR
evidence from a previous branch could satisfy dirty-work Stop handoff. The
review upgrade narrows this by requiring each validation line to carry
success-like result language and by binding validation and PR evidence to the
active OpenSpec change.

## Goals / Non-Goals

**Goals:**

- Reject failed, placeholder, skipped, stale, or unknown validation evidence.
- Reject otherwise successful-looking validation evidence for a different active
  change.
- Reject otherwise successful-looking PR/automerge evidence for a different
  active change or branch.
- Preserve the existing human-readable Markdown evidence format.
- Keep the implementation deterministic and dependency-free.
- Add negative tests for stop and delivery-sensitive command hooks.
- Keep the validation order rule: generated artifacts before tests.

**Non-Goals:**

- Introduce a new evidence file format.
- Parse shell output or depend on remote CI APIs.
- Replace the full `just ci` gate.
- Refactor unrelated Codex workflow checks.

## Design

The workflow checker keeps reading
`.idd/evidence/autonomous-workflow/validation.md`, but section validation moves
from `text.contains()` to line-oriented parsing.

For each required section label, the checker finds the first Markdown bullet or
plain line that starts with that label after optional bullet/list prefixes. The
entry is accepted only when its result text contains success-like language such
as `pass`, `passed`, `completed`, `success`, `succeeded`, `refreshed`, `clean`,
`no matches`, `0 critical`, or `0 warning`, and does not contain failure-like
language such as `fail`, `failed`, `error`, `skipped`, `not run`, `missing`,
`stale`, `unknown`, or `todo`.

The order check uses section positions instead of raw substring locations.

When `.idd/workflow/active-change` or `RUSTY_IDD_CHANGE` names an active change,
validation evidence must also contain an exact `Change: <change>` line. Backticks
around the change id are accepted for Markdown readability.

Dirty-work Stop/SubagentStop PR evidence is also parsed deterministically. It
must contain `Change: <change>`, `Branch: <current-branch>`, a real PR marker,
`Base: develop`, and an `Auto-merge:` entry that says auto-merge is enabled.
Pending, placeholder, or previous-branch PR evidence is rejected.

## Validation Strategy

- Add focused CLI tests for:
  - failed validation lines rejected at Stop;
  - placeholder/skipped validation lines rejected before `git push`;
  - same rejection before `gh pr create`;
  - same rejection before `gh pr merge --auto`;
  - same rejection before task completion;
  - otherwise successful evidence for a different active change;
  - otherwise successful PR evidence for a different active change or branch;
  - successful validation evidence still passes.
- Run focused Codex CLI tests first.
- Run full `just ci` with this goal file and change id.
- Run OpenSpec status/validate, secret scan, and workflow-check evidence gates.

## Rollback

Revert the validation parser and tests. The previous marker-order gate remains
available in PR #82, but rollback reintroduces the false-positive gap and should
only be used if this parser blocks legitimate success evidence.
