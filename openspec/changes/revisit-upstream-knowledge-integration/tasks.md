# revisit-upstream-knowledge-integration - Tasks

## 1. Lifecycle Artifacts

- [x] 1.1 Create the OpenSpec change.
- [x] 1.2 Add the spec delta for upstream knowledge integration.
- [x] 1.3 Add the design record.
- [x] 1.4 Add ADR 0005 for the full-feature upstream strategy.
- [x] 1.5 Validate artifact order with `rusty-idd spec status` and
  `rusty-idd spec next`.

## 2. Upstream Adoption Diagnostics

- [x] 2.1 Verify current upstream URLs and exact git revisions for
  `codegraph-rust` and `repomix-rs`.
- [x] 2.2 Compare verified upstream revisions against tracked mirrors.
- [x] 2.3 Run native `codegraph-rust` build, test, lint, docs, audit, smoke, and
  diagnostic commands as discovered from upstream metadata.
- [x] 2.4 Run native `repomix-rs` build, test, lint, docs, audit, smoke, and
  diagnostic commands as discovered from upstream metadata.
- [x] 2.5 Record commands, results, failures, required tools, runtime
  assumptions, generated assets, feature flags, valuable surfaces, cuts, and
  rollback in `/AI_MERGE`.

## 3. Strict-Upgrade Implementation

- [x] 3.1 Audit current Rusty IDD code and docs for missed or misstated
  `repomix-rs`, `codegraph-rust`, `tree-sitter`, domains, daemon, and MCP
  surfaces.
- [x] 3.2 Assess required toolchain support through parent `meta` / `envctl` or
  tracked repo-local equivalents; record optional upstream diagnostic gaps
  without user-global installs.
- [x] 3.3 Implement thin-boundary code fixes with tests while keeping
  `crates/core` std-only.
- [x] 3.4 Apply any consolidation cut as one TDD step with targeted upstream and
  Rusty IDD tests plus documented rollback.

## 4. Regeneration, Validation, and Merge

- [x] 4.1 Refresh `.idd/knowledge/*`.
- [x] 4.2 Refresh `.idd/MANIFEST.tsv`.
- [x] 4.3 Run focused tests for changed code paths and smoke-test affected CLI
  surfaces.
- [x] 4.4 Run full gates: `just ci`, `make ci`,
  `cargo test --workspace --all-features --locked`, strict docs,
  `cargo audit --deny warnings`, and
  `rusty-idd validate --workspace .`.
- [ ] 4.5 Commit, push, open PR, wait for checks, merge to `develop`, and clean
  branch/worktree state.
