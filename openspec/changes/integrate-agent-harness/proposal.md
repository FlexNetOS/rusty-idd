# integrate-agent-harness

## Why

Rusty IDD's current agent harness surface has grown around always-visible
agent directories such as `.codex`, `.claude`, `.kimi`, and `.agents`. MCP
servers attempted to make more tools reachable, but that still leaves agents
with a broad tool universe, large prompt context, and weak task-specific
selection.

The workflow needs the opposite shape: after a goal is created, Rusty IDD
selects the next workflow stage and assembles a stage-scoped Rust package for
that target. The always-on harness stays small. The stage package carries only
the contracts, tools, helpers, hooks, skills, agent roles, and evidence schema
needed for that exact work slice.

## What Changes

- Add a Rusty IDD-owned `agent-harness` workflow package model.
- Add the first vertical slice for the `scan` stage so Rusty IDD can describe
  the scoped agent team, tools, contracts, validation gates, and typed evidence
  required for scan work.
- Add a general package-creation CLI surface that creates or selects the right
  task-scoped package instead of asking Codex to create another ad hoc skill.
- Document that `.codex`, `.claude`, `.kimi`, and similar directories are
  minimal adapters/runtime views, not the authoritative harness brain.
- Keep MCP out of the default solution for tool overflow; feature-gated MCP can
  still exist only when a stage package explicitly declares it.

## Capabilities

### New Capabilities

- `agent-harness-workflow`: Rusty IDD routes workflow stages to scoped Rust
  agent swarm packages with typed contracts and evidence.

### Modified Capabilities

- `codex-harness-flow`: Codex adapters invoke Rusty IDD package generation
  rather than growing always-loaded repo skill/tool surfaces.

## Impact

- `crates/cli/src/commands/*`
- `crates/core/src/*` or a new Rust-owned harness package module
- `docs/rusty-idd/codex-environment.md`
- `.agents/skills/rusty-idd-codex-rust-env/SKILL.md`
- `AGENTS.md`
- `openspec/changes/integrate-agent-harness/*`
- `adr/0010-task-scoped-agent-harness-packages.md`
