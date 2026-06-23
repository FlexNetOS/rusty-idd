# add-verify-package-stage - Tasks

## 1. Rusty IDD Artifact Flow

- [x] 1.1 Create a fresh feature worktree from `origin/develop`.
- [x] 1.2 Recall ICM context for `/verify`, harness packages, and tool overflow.
- [x] 1.3 Create `.idd/goals/add-verify-package-stage.md` from the approved
  strategy response.
- [x] 1.4 Create proposal, design, spec delta, ADR, and task artifacts before
  implementation.
- [x] 1.5 Generate graph-backed plan context for the new goal.
- [x] 1.6 Record the active OpenSpec change pointer.
- [x] 1.7 Verify `rusty-idd spec status openspec/changes/add-verify-package-stage`.

## 2. Verify-Stage Package Slice

- [x] 2.1 Extend `rusty-idd harness package` to accept `--stage verify`.
- [x] 2.2 Add `--goal-file`, `--task-file`, and `--plan-file` inputs for
  verification package generation.
- [x] 2.3 Emit JSON and Markdown package formats for the verify stage.
- [x] 2.4 Include only verify-scoped roles, contracts, tools, helpers, hooks,
  validation gates, and evidence schema.
- [x] 2.5 Ensure the package compares implementation output against the
  original request, goal, task card, OpenSpec tasks, and plan.
- [x] 2.6 Ensure the package requires ICM recall/context comparison and graph
  or knowledge artifact comparison.

## 3. Thin `/verify` Adapter

- [x] 3.1 Add a minimal `/verify` adapter prompt for Codex or equivalent model
  surfaces.
- [x] 3.2 Ensure the adapter delegates to Rusty IDD package generation instead
  of embedding the full checklist.
- [x] 3.3 Document that missing verification capability must be reported as a
  missing Rusty IDD package capability, not solved with ad hoc always-loaded
  prompt growth.

## 4. Validation and Delivery

- [x] 4.1 Add focused tests for verify package output.
- [x] 4.2 Run focused CLI tests for harness package generation.
- [x] 4.3 Run formatting, clippy, and relevant workspace checks.
- [x] 4.4 Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
- [x] 4.5 Run Rusty IDD validation and OpenSpec status.
- [x] 4.6 Record validation and PR evidence.
- [ ] 4.7 Commit, push, open a PR to `develop`, and enable auto-merge when
  green.
