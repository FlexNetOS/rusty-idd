# integrate-agent-harness - Tasks

## 1. Rusty IDD Artifact Flow

- [x] 1.1 Create a fresh feature worktree from `origin/develop`.
- [x] 1.2 Recall ICM context for task-scoped harness packages and tool overflow.
- [x] 1.3 Refresh `.idd/knowledge/plan-context.md` for this goal.
- [x] 1.4 Create proposal, design, spec delta, ADR, and tasks before implementation.
- [x] 1.5 Record the active OpenSpec change pointer for hook enforcement.
- [x] 1.6 Verify `rusty-idd spec status openspec/changes/integrate-agent-harness`.

## 2. Scan-Stage Package Slice

- [x] 2.1 Add Rust-owned harness package data structures.
- [x] 2.2 Add `rusty-idd harness package --stage scan --target <path>`.
- [x] 2.3 Emit JSON and Markdown package formats.
- [x] 2.4 Ensure the scan package includes only stage-scoped contracts, roles,
  hooks, helpers, tools, and evidence schema.
- [x] 2.5 Add focused CLI/unit tests for scan package output.

## 3. Minimal Adapter Boundary

- [x] 3.1 Update docs and skills so `.codex`, `.claude`, `.kimi`, and `.agents`
  are described as thin adapters/views.
- [x] 3.2 Document package generation as the replacement for ad hoc skill
  creation when a process needs proper implementation support.
- [x] 3.3 Keep MCP out of the default package and document the feature-gated
  exception rule.

## 4. Validation and Delivery

- [x] 4.1 Run focused tests for the new harness package command.
- [x] 4.2 Run formatting and relevant Rust checks.
- [x] 4.3 Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
- [x] 4.4 Run OpenSpec status and Rusty IDD validation.
- [x] 4.5 Record validation evidence.
- [x] 4.6 Commit, push, open a PR to `develop`, and enable auto-merge when green.
