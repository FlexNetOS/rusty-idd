# scaffold-integration-work

## Why

Rusty IDD now produces a deterministic integration automation plan, but the
next execution step is still manual: an agent must read a work item and then
hand-write the OpenSpec proposal, design, tasks, and spec delta. That leaves a
gap between detailed system graphs and real automated integration.

Rusty IDD should consume its own integration-plan artifact and create the next
OpenSpec change in the expected artifact order, preserving owners, adopt-first
inputs, validation gates, and rollback before implementation begins.

## What Changes

- Add a CLI command that reads `.idd/knowledge/integration-plan.json`.
- Select a work item by change id, capability id, work-item id, or first
  priority item by default.
- Generate an OpenSpec change directory with proposal, design, tasks, and spec
  delta files.
- Refuse to overwrite existing files unless explicitly forced.
- Record generated output paths and preserve the original work-item evidence.

## Capabilities

### New Capabilities

- `integration-work-scaffolding`: turns an integration-plan work item into
  executable OpenSpec lifecycle artifacts.

### Modified Capabilities

- `integration-automation-plan`: work items become direct inputs to OpenSpec
  change generation, not only human-readable planning rows.

## Impact

- `crates/cli`
- `crates/knowledge`
- `.agents/skills/rusty-idd-knowledge`
- `.idd/knowledge`
- `.idd/MANIFEST.tsv`
- `/AI_MERGE`
