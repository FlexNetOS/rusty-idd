# ADR 0006: Delivery Evidence Requires Success Semantics

Status: accepted
Date: 2026-06-22

## Context

ADR 0005 requires generated artifact evidence before test evidence and requires
validation evidence before task completion, push, and PR handoff. A deep review
of the merged implementation found that the checker accepted required labels
without validating the result text behind those labels.

Marker-only evidence can accidentally or intentionally report failed, skipped,
or stale commands while still passing the workflow checker.

## Decision

Rusty IDD workflow checks will require each mandatory validation evidence
section to be success-like. The Markdown evidence format stays human-readable,
but each required entry must include language that represents a completed,
passing, clean, refreshed, or no-finding result. Entries that report failure,
skips, unknown state, missing evidence, stale artifacts, TODO placeholders, or
not-run commands are rejected.

Validation evidence must also name the active OpenSpec change. PR evidence must
name the active OpenSpec change, current branch, real PR marker, `Base: develop`,
and enabled auto-merge status. Prior-task and prior-PR evidence can remain in
Git history and AI_MERGE packages, but it cannot unlock a new active change.

The order requirement from ADR 0005 remains: generated artifacts must precede
tests.

## Consequences

- Delivery-sensitive commands cannot be unlocked by labels alone.
- Delivery-sensitive commands cannot be unlocked by validation evidence from a
  previous active change.
- Dirty-work Stop/SubagentStop handoff cannot be unlocked by PR evidence from a
  previous active change or branch.
- Evidence remains easy for humans to read and easy for agents to write.
- The workflow checker grows a small deterministic parser instead of depending
  on remote CI or shell-output parsing.
- Legitimate evidence text should use explicit success language such as
  `passed`, `completed`, `refreshed`, `clean`, `no matches`, or `0 critical`.
- Legitimate validation and PR evidence should include a `Change: <change-id>`
  line, and PR evidence should include the current feature branch.

## Rollback

Revert this ADR, the parser upgrade, and its tests. The previous marker-order
gate remains available, but rollback restores the false-positive gap.
