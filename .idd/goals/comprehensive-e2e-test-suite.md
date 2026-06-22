# Comprehensive E2E Test Suite Goal

```bash
rusty-idd --goal-file [comprehensive E2E test suite for the entire rusty-idd code base and workflow | must include test after all artifact creation for validation | test must be manadory before every task complete and push]
```

## Intent

Create a comprehensive end-to-end test suite and workflow contract for the
entire Rusty IDD code base and generated workflow.

The suite must cover the Rusty IDD CLI path from goal-file intake through graph
artifacts, OpenSpec readiness, generated diagrams, manifest validation, task
completion evidence, and PR handoff gates.

## Required Method

- Start from the latest Rusty IDD `develop` branch in a fresh feature worktree.
- Generate Rusty IDD plan context through `rusty-idd knowledge plan-context
  --goal-file`.
- Run the Rusty IDD scan and plan workflow before implementation writes.
- Update OpenSpec proposal, spec delta, design, tasks, ADR, and evidence
  records before claiming implementation complete.
- Add or update tests so generated artifacts are validated after creation.
- Make the test gate mandatory before any task is marked complete.
- Make the test gate mandatory before any branch is pushed for PR handoff.
- Refresh `.idd/knowledge/*`, architecture diagrams, and `.idd/MANIFEST.tsv`.
- Record validation evidence, migration notes, and rollback guidance.

## Non-Goals

- No downgrade of existing gates, tools, dependencies, or generated artifacts.
- No removal of existing validation commands.
- No host-service, daemon, MCP server, or user-global tool installation.
- No broad refactor unrelated to E2E workflow validation.
