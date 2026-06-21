# integrate-fleet-handoff

## Why

Rusty IDD selected `work:integrate-fleet-handoff` from the integration automation plan to move `Integrate Central and fleet handoff` from `partial` toward implemented system capability.

This change preserves the graph-backed owners, anchors, adopt-first inputs, validation gates, and rollback path before implementation begins.

## What Changes

- Implement integration work item `work:integrate-fleet-handoff`.
- Keep the implementation boundary: Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input.
- Use TDD consolidation: adopt current upstream/owner surfaces first, run native diagnostics, then cut only evidenced friction.

## Capabilities

### New Capabilities

- `fleet-handoff`: Integrate Central and fleet handoff

### Modified Capabilities

- `integration-automation-plan`: this work item is now executing through OpenSpec artifacts.

## Impact

- Owner repos:
  - `repo:agent`
  - `repo:ecc`
  - `repo:envctl`
  - `repo:flexnetos-runner`
  - `repo:github-org`
  - `repo:handoff`
  - `repo:harness-hub`
  - `repo:lane`
  - `repo:lifeos`
  - `repo:network-control`
  - `repo:prompt-hub`
  - `repo:rusty-idd`
  - `repo:weave`
- Anchors:
  - `handoff central and fleet design`
