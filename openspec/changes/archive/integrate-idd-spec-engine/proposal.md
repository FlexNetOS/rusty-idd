# integrate-idd-spec-engine

## Why

Rusty IDD selected `work:integrate-idd-spec-engine` from the integration automation plan to move `Integrate IDD and spec engine` from `partial` toward implemented system capability.

This change preserves the graph-backed owners, anchors, adopt-first inputs, validation gates, and rollback path before implementation begins.

## What Changes

- Implement integration work item `work:integrate-idd-spec-engine`.
- Keep the implementation boundary: Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input.
- Use TDD consolidation: adopt current upstream/owner surfaces first, run native diagnostics, then cut only evidenced friction.
- Add a deterministic `rusty-idd spec status --json <change_dir>` boundary so
  automated handoff and future runner workflows can consume OpenSpec lifecycle
  state without scraping human text.

## Capabilities

### New Capabilities

- `idd-spec-engine`: Integrate IDD and spec engine
- `spec-status-json`: machine-readable OpenSpec lifecycle status for automated
  orchestration.

### Modified Capabilities

- `integration-automation-plan`: this work item is now executing through OpenSpec artifacts.

## Impact

- Owner repos:
  - `repo:handoff`
  - `repo:rusty-idd`
- Anchors:
  - `Rusty IDD built into handoff`
