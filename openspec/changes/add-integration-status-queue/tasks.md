# add-integration-status-queue - Tasks

## 1. Lifecycle Artifacts

- [x] 1.1 Create the OpenSpec change.
- [x] 1.2 Add proposal, design, tasks, and spec delta before implementation.

## 2. Implementation

- [x] 2.1 Add integration status DTOs and builder.
- [x] 2.2 Add `rusty-idd knowledge integration-status`.
- [x] 2.3 Classify planned, incomplete scaffold, scaffolded,
  ready-to-archive, and archived states.
- [x] 2.4 Emit deterministic JSON and Markdown.
- [x] 2.5 Add Justfile and Makefile generation/check targets.
- [x] 2.6 Update local knowledge workflow guidance.

## 3. Validation

- [x] 3.1 Add focused library and CLI tests.
- [x] 3.2 Generate `.idd/knowledge/integration-status.*`.
- [x] 3.3 Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
- [x] 3.4 Run focused tests.
- [x] 3.5 Run full gates.
- [x] 3.6 Record evidence and rollback in `/AI_MERGE`.
