# add-integration-status-queue

## Why

Rusty IDD can generate an integration plan and scaffold an OpenSpec change from
a work item, but it does not yet report integration backlog state. Agents still
have to manually compare `.idd/knowledge/integration-plan.json` against
`openspec/changes/*` to know what is planned, already scaffolded, ready to
archive, archived, or next.

The workflow needs a deterministic queue artifact so integration automation can
advance without rediscovering state or creating duplicate OpenSpec changes.

## What Changes

- Add an integration status command that reads the integration plan.
- Join each work item to OpenSpec change/archive state.
- Report planned, incomplete scaffold, scaffolded, ready-to-archive, and
  archived statuses.
- Emit deterministic JSON and Markdown queue artifacts.
- Identify the next unscaffolded integration work item.

## Capabilities

### New Capabilities

- `integration-status-queue`: exposes integration backlog execution state from
  Rusty IDD artifacts and OpenSpec state.

### Modified Capabilities

- `integration-automation-plan`: generated work items can be tracked through
  scaffold, execution, and archive states.

## Impact

- `crates/knowledge`
- `crates/cli`
- `.agents/skills/rusty-idd-knowledge`
- `.idd/knowledge`
- `.idd/MANIFEST.tsv`
- `/AI_MERGE`
