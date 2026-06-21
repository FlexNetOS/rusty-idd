# 0001. Codex harness follows Rusty IDD flow

- Status: accepted
- Date: 2026-06-21

## Context

The repository still carried active `idd-merge-idd` bridge material in
Claude/Gemini prompts, `_workspace` loop state, and legacy ADRs. That material
was useful during the original merge, but it now competes with Rusty IDD's
designed workflow: user intent, graph-backed context, OpenSpec artifacts,
implementation gating, validation, and evidence.

The active ADR directory must describe current architectural commitments only.
Historical merge-era decisions are preserved as implementation history in
documentation and the merge-tools package rather than as active decisions.

## Decision

The Codex harness follows Rusty IDD flow:

1. capture the user goal;
2. read or refresh `.idd/knowledge/*`;
3. bind the goal with OpenSpec proposal, spec deltas, design, ADR, and tasks;
4. implement only after OpenSpec status is ready;
5. validate, regenerate deterministic artifacts, and record evidence.

Merge, migration, and repository-unification goals use the `merge-tools package`
(`rusty-idd merge-tools show`) for reusable workflow phases and legacy-surface
disposition. `AI_MERGE/` remains optional evidence/history, not the active
control plane.

This is the single active ADR after clearing the legacy ADR set.

## Consequences

- `adr/` contains only the current Codex harness decision.
- Legacy merge decisions are summarized in `docs/rusty-idd/merge-tools-package.md`.
- Claude/Gemini bridge files must not reintroduce `idd-merge-idd` as an active
  workflow.
- Future durable decisions should add a new ADR only when the current active
  architecture changes, not to preserve retired bridge history.
