# 41 — Verify-stage harness package

Evidence note for the `add-verify-package-stage` change (ADR-0011; advances
`harness-control-plane` backlog 4.3, "additional stage packages"). Adds the
`verify` stage to `rusty-idd harness package`, so post-task verification is an
engine-owned package — not an always-loaded prose checklist in each adapter.

## What landed

1. **`rusty-idd harness package --stage verify`** (`crates/cli/src/commands/harness.rs`):
   - new `Verify` variant on `HarnessStage`; `verify_package(...)` assembles the
     verify-scoped agent team, contracts, tools, helpers, hooks, validation
     gates, evidence schema, and adapter boundary.
   - new `--goal-file` / `--task-file` / `--plan-file` inputs bound into the
     package (optional, JSON-visible via `skip_serializing_if`, rendered in
     Markdown); `scan` keeps them `None`.
   - emits both JSON and Markdown (the existing `--format`).
2. **Cross-checks built into the package** (tasks 2.5/2.6): `goal-comparator` +
   `original-request-contract` + `task-plan-contract` + `goal-matched`/
   `tasks-satisfied` gates; `icm-checker` + `graph-checker` +
   `icm-comparison-contract` + `graph-contract` + `icm-recall-context-compare`
   tool + `graph-icm-checked` gate.
3. **Thin `/verify` adapter** (`.agents/skills/rusty-idd-verify/SKILL.md`): loads
   the package and obeys it; carries no checklist of its own, and documents that
   a missing verification capability must be reported as a missing Rusty IDD
   package capability — never solved by always-loaded prompt growth (tasks 3.1–3.3).

## Verification evidence

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no issues.
- `cargo test --workspace --locked` — 665 passed, 0 failed (+3 verify tests over
  the 662 baseline: markdown stage-scoped, JSON evidence/adapters, goal/task/plan
  binding).
- `rusty-idd spec validate --all` — 153/153, 0 failed.
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- `.idd/knowledge/*` refreshed, then `.idd/MANIFEST.tsv` (3541 entries);
  re-validate stays 0/0, manifest self-stable, 0 `.worktrees`/`.idd-bak`
  contamination.
- `rusty-idd spec status openspec/changes/add-verify-package-stage` — archivable.

## Flow

Artifacts (proposal/design/spec/ADR-0011/tasks) and the goal were already
authored in a prior pass; this pass implemented §2–§4 after the DAG was ready,
then refreshed validation. Active-change pointer set to
`add-verify-package-stage`.
