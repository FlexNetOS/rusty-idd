# add-autonomous-workflow-hooks

## Why

Rusty IDD already documents the autonomous workflow order, but the repo-local
Codex hook surface only runs a final environment invariant check. That lets an
agent drift into implementation before creating or selecting the tracked task,
knowledge context, OpenSpec artifacts, validation evidence, and PR handoff that
the workflow requires.

## What Changes

- Add Rust-native pre/post Codex workflow checks that make the autonomous Rusty
  IDD workflow executable instead of advisory.
- Register the checks in `.codex/hooks.json` on the supported Codex lifecycle
  events that bracket tool execution and turn completion.
- Require implementation work to happen from a feature worktree based on
  `develop`, with graph-backed plan context, OpenSpec readiness, task-card
  evidence, validation evidence, and PR/automerge handoff evidence.
- Document the enforced path and its rollback boundary.

## Capabilities

### New Capabilities

- `autonomous-workflow-hooks`: Codex hooks enforce the full Rusty IDD
  autonomous workflow from goal intake through PR handoff.

### Modified Capabilities

- `codex-harness-flow`: harness-facing workflow checks become lifecycle hook
  gates, not just instructions.

## Impact

- `.codex/hooks.json`
- `crates/cli/src/commands/codex.rs`
- `crates/cli/tests/codex_cli.rs`
- `docs/rusty-idd/codex-environment.md`
- `.agents/skills/rusty-idd-codex-rust-env/SKILL.md`
- `adr/0002-autonomous-workflow-hooks.md`
