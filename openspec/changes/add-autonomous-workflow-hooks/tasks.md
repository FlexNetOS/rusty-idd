# add-autonomous-workflow-hooks - Tasks

## 1. Rusty IDD Artifact Gate

- [x] 1.1 Create/claim the tracked task card for autonomous workflow hooks and record task evidence.
- [x] 1.2 Refresh `.idd/knowledge/plan-context.md` for this goal.
- [x] 1.3 Create proposal, spec delta, design, ADR, and tasks before implementation.
- [x] 1.4 Verify `rusty-idd spec status openspec/changes/add-autonomous-workflow-hooks` reports the change ready.
- [x] 1.5 Record the active OpenSpec change pointer for hook enforcement.

## 2. Hook Enforcement

- [x] 2.1 Add `rusty-idd codex workflow-check --phase <pre-tool|post-tool|stop>`.
- [x] 2.2 Validate feature worktree and `develop` ancestry before write-capable implementation.
- [x] 2.3 Validate plan context, OpenSpec readiness, and task-card evidence before implementation.
- [x] 2.4 Validate dirty-work completion evidence: tests, manifest/knowledge refresh, PR push, and auto-merge evidence.
- [x] 2.5 Keep runtime audit Rust-native with no Python hook/script dependency.

## 3. Hook Registration and Documentation

- [x] 3.1 Register PreToolUse, PostToolUse, Stop, and SubagentStop hooks in `.codex/hooks.json`.
- [x] 3.2 Update Codex environment docs and repo skill guidance with the enforced autonomous path.
- [x] 3.3 Add focused CLI tests for passing and failing workflow-check states.

## 4. Validation and Delivery

- [x] 4.1 Run focused CLI tests for Codex workflow checks.
- [x] 4.2 Run `rusty-idd codex env-check --workspace .`.
- [x] 4.3 Run `rusty-idd codex runtime-audit --workspace .`.
- [x] 4.4 Refresh `.idd/MANIFEST.tsv` and required knowledge artifacts or record why unchanged.
- [x] 4.5 Checkpoint the task card with validation evidence.
- [x] 4.6 Commit, push the feature branch, open a PR into `develop`, and enable auto-merge.
- [ ] 4.7 Record PR/automerge evidence and mark the tracked task done.
