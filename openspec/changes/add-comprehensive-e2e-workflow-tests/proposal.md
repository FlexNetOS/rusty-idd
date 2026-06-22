# add-comprehensive-e2e-workflow-tests

## Why

Rusty IDD already generates goal-bound knowledge, OpenSpec, ADR, diagram,
manifest, and validation artifacts, but the autonomous completion path needs a
stronger end-to-end contract: after artifact creation, tests must be mandatory
before task completion and before PR push handoff.

The owner goal is:

```bash
rusty-idd --goal-file [comprehensive E2E test suite for the entire rusty-idd code base and workflow | must include test after all artifact creation for validation | test must be manadory before every task complete and push]
```

## What Changes

- Add a goal-file-backed Rusty IDD change for comprehensive E2E workflow tests.
- Run Rusty IDD scan and plan artifacts for the repository before
  implementation.
- Extend the Rusty IDD workflow contract so task completion and PR push handoff
  require test evidence after generated artifact refresh.
- Add focused tests for the mandatory gate behavior and generated-artifact
  validation path.
- Refresh knowledge, architecture diagrams, manifest, OpenSpec, ADR, and
  evidence artifacts.

## Capabilities

### New Capabilities

- `comprehensive-e2e-workflow-tests`: validate the Rusty IDD workflow from
  goal-file intake through artifact refresh, mandatory test evidence, task
  completion, and PR handoff.

### Modified Capabilities

- `codex-harness-flow`: require test evidence before task completion and push
  handoff.
- `graph-context-planning`: keep goal-file plan context current for generated
  validation.

## Impact

- Rusty IDD CLI workflow validation.
- `.idd/knowledge/*`
- `.idd/MANIFEST.tsv`
- `docs/rusty-idd/architecture-diagrams.md`
- `openspec/changes/add-comprehensive-e2e-workflow-tests/*`
- `adr/`
- `AI_MERGE/35_e2e_test_suite/*`
