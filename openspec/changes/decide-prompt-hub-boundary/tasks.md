# decide-prompt-hub-boundary - Tasks

## 1. Research

- [x] 1.1 Recall ICM context and repo rules.
- [x] 1.2 Inspect current Rusty IDD generated knowledge and system graph.
- [x] 1.3 Inspect PromptHub docs, Cargo workspace, source layout, templates,
      CLI, continuity capsule, and handoff state.
- [x] 1.4 Run PromptHub native diagnostic: `rtk cargo check --workspace`.
- [x] 1.5 Record PromptHub dirty-state boundaries without mutating them.

## 2. Decision Artifacts

- [x] 2.1 Create the goal file for the boundary decision.
- [x] 2.2 Create the Rusty IDD task card.
- [x] 2.3 Create OpenSpec proposal, design, spec delta, and tasks.
- [x] 2.4 Create ADR for the PromptHub front-door boundary.
- [x] 2.5 Create AI_MERGE research evidence.

## 3. Generated Artifacts

- [x] 3.1 Generate goal-file-backed plan context.
- [x] 3.2 Refresh `.idd/knowledge/index.json` and `.idd/knowledge/report.md`.
- [x] 3.3 Refresh architecture graph and architecture diagrams.
- [x] 3.4 Refresh system architecture, operating model, integration plan,
      integration status, integration owners, and integration readiness.
- [x] 3.5 Refresh `.idd/MANIFEST.tsv`.

## 4. Validation

- [x] 4.1 Validate the OpenSpec change status.
- [x] 4.2 Run `rusty-idd spec validate --all`.
- [x] 4.3 Run Rusty IDD generated artifact checks.
- [x] 4.4 Run full `just ci` with this goal file and change id.
- [x] 4.5 Record validation evidence.

## 5. Publication

- [x] 5.1 Mark task complete after validation.
- [x] 5.2 Commit all changes.
- [x] 5.3 Push branch, create PR to `develop`, and enable auto-merge on green.
