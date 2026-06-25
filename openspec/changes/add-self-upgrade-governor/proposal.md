# add-self-upgrade-governor

## Why

Rusty IDD has graph-backed knowledge, OpenSpec lifecycle gates, a runner, Codex
model-loop adapters, workflow checks, merge-tool packages, and the newly planned
task-scoped harness package surface. Those pieces still require a user or model
to manually decide the next goal and assemble the correct package path.

The next missing capability is a Rusty IDD-owned self-upgrade governor: a
bounded renewable workflow that scans repo truth, proposes candidate goals,
routes each accepted goal through the right package, verifies the result, and
feeds the next cycle without growing always-loaded harness directories.

## What Changes

- Add a goal-file-backed design for a `self-upgrade` command family:
  `scan`, `propose`, `goal`, `package`, `run`, `verify`, `publish`, and `next`.
- Define a typed goal-generation pipeline:
  `Finding -> Opportunity -> Hypothesis -> CandidateGoal -> GoalReview ->
  ApprovedGoal -> OpenSpecChange -> Package`.
- Split automation into an endless read-only discovery loop and a finite
  write-capable delivery loop.
- Define the first package sequence: `scan`, `goal`, `design`, `implement`,
  `verify`, `publish`, and `learn`.
- Record safety policy for auto-runnable low-risk goals and owner-approved
  high-risk goals.
- Record the first downstream test target as Rusty IDD feature integrations and
  automations, without researching or implementing that downstream target in
  this change.

## Capabilities

### New

- `self-upgrade-governor`: Rusty IDD governs self-upgrade discovery, candidate
  goal generation, scoped package routing, and bounded delivery loops.

### Modified

- `agent-harness-workflow`: Harness adapters remain minimal and delegate package
  selection to Rusty IDD rather than accumulating always-loaded skills/tools.

## Impact

- Affected artifacts:
  - `.idd/goals/add-self-upgrade-governor.md`
  - `openspec/changes/add-self-upgrade-governor/*`
  - `adr/0012-self-upgrade-governor.md`
  - `.idd/evidence/self-upgrade-governor/*`
  - `.idd/knowledge/*`
  - `.idd/MANIFEST.tsv`
- No host services, daemons, MCP servers, or user-global tools are added.
- No implementation of the downstream integration/automation test target is in
  scope for this change.
