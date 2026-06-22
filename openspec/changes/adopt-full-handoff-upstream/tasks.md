# adopt-full-handoff-upstream - Tasks

## 1. Tracking And Review

- [x] 1.1 Create, mint, and claim the handoff task card.
- [x] 1.2 Create a fresh feature worktree from `origin/develop`.
- [x] 1.3 Review ADR 0005, dot-directory architecture, and AI_MERGE graph
  evidence.
- [x] 1.4 Inspect the source handoff checkout, tracked-file count, and dirty
  source state.

## 2. OpenSpec And Decision

- [x] 2.1 Create a Rusty IDD goal file for full handoff adoption.
- [x] 2.2 Add OpenSpec proposal, design, spec deltas, and tasks.
- [x] 2.3 Add ADR 0006 for the pinned handoff upstream mirror.

## 3. Adopt Handoff Whole

- [x] 3.1 Import the complete tracked handoff repository into
  `third_party/upstream/handoff`.
- [x] 3.2 Verify source tracked-file count matches mirror file count.
- [x] 3.3 Update `third_party/upstream/UPSTREAMS.md`.
- [x] 3.4 Record mirror verification, tracked-file inventory, source dirty
  state, rollback, and next adapter gaps under `AI_MERGE`.

## 4. Regenerate And Validate

- [x] 4.1 Refresh deterministic Rusty IDD knowledge and architecture artifacts.
- [x] 4.2 Regenerate architecture diagrams and `.idd/MANIFEST.tsv`.
- [x] 4.3 Run OpenSpec status and validation.
- [x] 4.4 Run full local gates, workflow checks, diff check, and secret scan.
- [ ] 4.5 Publish PR, enable auto-merge, complete the task, and sync branches.
