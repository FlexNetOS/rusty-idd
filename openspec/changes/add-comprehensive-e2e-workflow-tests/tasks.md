# add-comprehensive-e2e-workflow-tests - Tasks

## 1. Goal and Planning

- [x] 1.1 Create the tracked comprehensive E2E test suite goal file.
- [x] 1.2 Generate `.idd/knowledge/plan-context.md` with `--goal-file`.
- [x] 1.3 Generate `.idd/knowledge/plan-context.json` with `--goal-file`.
- [x] 1.4 Run Rusty IDD scan artifacts for this repository.
- [x] 1.5 Run the Rusty IDD plan workflow into the E2E evidence workspace.

## 2. OpenSpec and Decisions

- [x] 2.1 Add proposal, spec delta, design, and task artifacts.
- [x] 2.2 Add ADR for mandatory post-artifact test evidence.
- [x] 2.3 Verify OpenSpec change readiness with `rusty-idd spec status`.

## 3. Workflow Test Implementation

- [x] 3.1 Identify the current task-completion and push-handoff evidence checks.
- [x] 3.2 Add or update tests for mandatory post-artifact test evidence.
- [x] 3.3 Tighten implementation only where tests prove the workflow permits a
  task completion or push without required test evidence.

## 4. Regenerate and Validate

- [x] 4.1 Refresh `.idd/knowledge` artifacts.
- [x] 4.2 Regenerate architecture diagrams.
- [x] 4.3 Regenerate `.idd/MANIFEST.tsv`.
- [x] 4.4 Run OpenSpec status, validation, manifest, diagram, build/test/lint,
  audit, and secret-scan gates.
- [x] 4.5 Record final validation, migration, rollback, and PR handoff evidence.
