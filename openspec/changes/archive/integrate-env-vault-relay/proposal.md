# integrate-env-vault-relay

## Why

Rusty IDD selected `work:integrate-env-vault-relay` from the integration automation plan to move `Integrate Environment and vault relay` from `partial` toward implemented system capability.

This change preserves the graph-backed owners, anchors, adopt-first inputs, validation gates, and rollback path before implementation begins.

## What Changes

- Implement integration work item `work:integrate-env-vault-relay`.
- Keep the implementation boundary: Feature-gate host/vault behavior; keep default Rusty IDD generation read-only.
- Use TDD consolidation: adopt current upstream/owner surfaces first, run native diagnostics, then cut only evidenced friction.

## Capabilities

### New Capabilities

- `env-vault-relay`: Integrate Environment and vault relay

### Modified Capabilities

- `integration-automation-plan`: this work item is now executing through OpenSpec artifacts.

## Impact

- Owner repos:
  - `repo:envctl`
  - `repo:vault-hub`
  - `repo:yazelix`
- Anchors:
  - `/run/media/drdave/COGNITUM`
  - `Cognitum vault on Pi Zero`
- Adopt-first inputs:
  - `/run/media/drdave/COGNITUM`
  - `Cognitum vault on Pi Zero`
