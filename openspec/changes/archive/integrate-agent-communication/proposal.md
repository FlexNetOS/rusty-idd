# integrate-agent-communication

## Why

Rusty IDD selected `work:integrate-agent-communication` from the integration automation plan to move `Integrate Agent communication layer` from `partial` toward implemented system capability.

This change preserves the graph-backed owners, anchors, adopt-first inputs, validation gates, and rollback path before implementation begins.

## What Changes

- Implement integration work item `work:integrate-agent-communication`.
- Keep the implementation boundary: Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input.
- Use TDD consolidation: adopt current upstream/owner surfaces first, run native diagnostics, then cut only evidenced friction.

## Capabilities

### New Capabilities

- `agent-communication`: Integrate Agent communication layer

### Modified Capabilities

- `integration-automation-plan`: this work item is now executing through OpenSpec artifacts.

## Impact

- Owner repos:
  - `repo:atc`
  - `repo:handoff`
  - `repo:mcp-hub`
  - `repo:weave`
- Anchors:
  - `weave agent communication layer`
