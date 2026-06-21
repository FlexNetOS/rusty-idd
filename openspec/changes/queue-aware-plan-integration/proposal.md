# queue-aware-plan-integration

## Why

`rusty-idd knowledge integration-status` reports the next planned integration
work item, but `rusty-idd spec plan-integration` still selects the first item
from the raw integration plan when no selector is passed. After the first work
item is scaffolded, a default run can fail on overwrite instead of advancing the
queue.

The scaffolder should use queue-aware default selection so automated runs can
continue from planned work to planned work without manual comparison.

## What Changes

- Keep explicit `--change`, `--capability`, and `--work-item` selection
  behavior unchanged.
- Change the no-selector path to skip work items that already have an active
  OpenSpec change or archived change.
- Fail clearly when no planned work item remains.
- Update workflow guidance and tests.

## Capabilities

### New Capabilities

- `queue-aware-integration-scaffolding`: default integration scaffolding
  advances to the next planned work item.

### Modified Capabilities

- `integration-work-scaffolding`: no-selector scaffolding is driven by queue
  state instead of raw plan order only.

## Impact

- `crates/cli`
- `.agents/skills/rusty-idd-knowledge`
- `.idd/knowledge`
- `.idd/MANIFEST.tsv`
- `/AI_MERGE`
