# 0002. Autonomous workflow hooks enforce Rusty IDD gates

- Status: accepted
- Date: 2026-06-21

## Context

ADR-0001 makes the Codex harness follow Rusty IDD flow, but the enforcement
surface is still mostly instructional. A Codex agent can begin implementation
after reading intent and plan context without first creating the change-specific
OpenSpec artifacts or task-card evidence. That failure mode is exactly what the
Rusty IDD workflow is meant to prevent.

## Decision

Rusty IDD will enforce Codex autonomous workflow gates with repo-local,
Rust-native lifecycle hooks. The hooks run `rusty-idd codex workflow-check`
from `.codex/hooks.json` and verify the expected progression:

1. feature worktree and branch are based on `develop`;
2. graph-backed plan context exists for the goal;
3. an OpenSpec change has proposal, specs, design, ADR, and tasks ready;
4. task-card claim/checkpoint evidence exists before completion;
5. validation and PR/automerge evidence exist before final handoff.

The hooks must not introduce Python hooks, host service management, daemon
control, or user-global tool installation.

## Consequences

- `.codex/hooks.json` becomes an active workflow gate, not just an environment
  invariant check.
- `rusty-idd codex env-check` continues to validate static environment
  invariants; `rusty-idd codex workflow-check` validates change-specific
  autonomous workflow state.
- Final PR creation and auto-merge still happen through GitHub tooling; hooks
  verify local evidence and the delivery flow verifies the live PR state.
