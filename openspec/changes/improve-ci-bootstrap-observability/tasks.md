# improve-ci-bootstrap-observability - Tasks

## 1. Artifact Flow

- [x] 1.1 Record the self-generated goal file.
- [x] 1.2 Generate the scan-stage package evidence.
- [x] 1.3 Create OpenSpec proposal, design, spec delta, and tasks.
- [x] 1.4 Create ADR 0014 for CI bootstrap cache and model-loop policy.
- [x] 1.5 Record task, validation, and PR evidence placeholders.

## 2. Implementation

- [x] 2.1 Split CI envctl Rust tool/cache cache from workspace target cache.
- [x] 2.2 Apply the same cache split to promotion verification.
- [x] 2.3 Apply the same cache split to release builds.
- [x] 2.4 Add timed/grouped bootstrap spans to `envctl-rust-env.sh`.
- [x] 2.5 Update generated CI template cache shape.
- [x] 2.6 Update model-loop cheap read-heavy passes to `gpt-5.5-mini`.
- [x] 2.7 Update docs and CLI tests for the model-loop/cache policy.

## 3. Validation

- [x] 3.1 Run model-loop dry-run and cheap read-only pass evidence.
- [x] 3.2 Run focused tests.
- [x] 3.3 Verify OpenSpec status.
- [x] 3.4 Refresh `.idd/knowledge/*`.
- [x] 3.5 Run Rusty IDD validation.
- [x] 3.6 Refresh `.idd/MANIFEST.tsv`.
- [ ] 3.7 Commit, push, open PR, and monitor CI.
