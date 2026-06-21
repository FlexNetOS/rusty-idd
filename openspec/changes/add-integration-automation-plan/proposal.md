# add-integration-automation-plan

## Why

Rusty IDD now generates repo architecture, system architecture, peer
architecture summaries, graph planning context, and an operating model for the
agentic company system. The next gap is execution ordering: the operating model
shows capabilities and anchors, but Rusty IDD does not yet turn those into an
ordered, OpenSpec-ready integration backlog.

Rusty IDD needs a deterministic artifact that converts operating-model
capabilities into integration work items with owners, anchors, validation gates,
and rollback guidance.

## What Changes

- Add `rusty-idd knowledge integration-plan`.
- Consume `.idd/knowledge/operating-model.json` by default.
- Generate `.idd/knowledge/integration-plan.json` and
  `.idd/knowledge/integration-plan.md`.
- Produce ordered work items for partial, external, or missing capabilities.
- Carry selected integration work into graph planning context.
- Add Justfile and Makefile freshness checks.

## Capabilities

### New Capabilities

- `integration-automation-plan`: converts operating-model capabilities into
  deterministic OpenSpec-ready integration tasks.

### Modified Capabilities

- `graph-context-planning`: planning packets include selected integration work
  when the integration-plan artifact is present.

## Impact

- `crates/knowledge`
- `crates/cli`
- `.idd/knowledge`
- `.idd/MANIFEST.tsv`
- `.agents/skills/rusty-idd-knowledge`
- `/AI_MERGE`
