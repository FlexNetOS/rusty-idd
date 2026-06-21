# add-graph-context-planning

## Why

Rusty IDD now generates repo-local architecture graphs and parent meta system
graphs, but the automated workflow still expects agents to read those artifacts
manually before planning. That leaves a gap between graph generation and real
integration planning.

This change adds a graph-context planning artifact that consumes the generated
graphs and emits a bounded planning packet for OpenSpec/spec/design/task work.

## What Changes

- Add a `rusty-idd knowledge plan-context` command.
- Read `.idd/knowledge/architecture.json` and optional
  `.idd/knowledge/system-architecture.json`.
- Accept goal text or a goal file.
- Emit Markdown or JSON planning context with:
  - automation stages
  - integration surfaces
  - repo components
  - system roles
  - relevant repos
  - graph-backed planning guidance
  - suggested OpenSpec artifact order

## Capabilities

### New Capabilities

- `graph-context-planning`: produce a deterministic graph-backed planning packet
  for a Rusty IDD change or goal.

### Modified Capabilities

- `knowledge`: generated graph artifacts can now be consumed directly by the
  planning workflow instead of only being inspected manually.

## Impact

- `crates/knowledge`
- `crates/cli`
- `.agents/skills/rusty-idd-knowledge`
- `.idd/knowledge`
- `/AI_MERGE`
