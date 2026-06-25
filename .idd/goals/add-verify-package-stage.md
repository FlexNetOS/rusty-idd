# Goal: Add Rusty IDD `/verify` Package Stage

## Step 1: Start With The Rusty IDD CLI

Create or select the task-scoped verification package from Rusty IDD, not from
an always-loaded `.codex` prompt body:

```bash
rusty-idd harness package \
  --stage verify \
  --target . \
  --goal-file <goal.md> \
  --task-file <task.md> \
  --plan-file <plan.md> \
  --format markdown
```

The CLI package is the source of truth. `/verify` for Codex or any other model
is only a minimal adapter that loads and follows this package.

## Original Strategy Response To Materialize

Using `rusty-idd-codex-rust-env` as the frame: yes, `/verify` should be a thin
Codex adapter that delegates to a Rusty IDD `verify` package. The prompt should
not contain the full checklist. It should say: "load the task-scoped Rusty IDD
verify package, then execute it against this completed task."

### Core Shape

Make this the next package stage:

```bash
rusty-idd harness package \
  --stage verify \
  --target . \
  --goal-file <goal.md> \
  --task-file <task.md> \
  --plan-file <plan.md> \
  --format markdown
```

Then `/verify` becomes a small repo prompt:

```text
Run post-task verification through Rusty IDD, not through this prompt body.

1. Load the Rusty IDD verify package:
   rusty-idd harness package --stage verify --target . --goal-file <goal> --task-file <task> --plan-file <plan> --format markdown

2. Follow that package exactly.
3. Produce a verification report with pass/fail findings, evidence, unresolved questions, and rollback risk.
4. Do not invent extra always-loaded .codex skills/tools. If capability is missing, report the missing Rusty IDD package capability.
```

### Verify Package Contents

The Rusty IDD `verify` package should include:

- `agent_team`: verifier, diff-auditor, test-runner, evidence-checker,
  goal-comparator, graph-checker, icm-checker, risk-reviewer.
- `contracts`: original request contract, plan/task contract, implementation
  diff contract, evidence contract, adapter-boundary contract.
- `tools`: `git diff`, `git status`, `rusty-idd validate`,
  `rusty-idd manifest`, `rusty-idd knowledge plan-context`,
  `rusty-idd spec status`, test/lint/build gates, ICM recall/context compare.
- `helpers`: goal normalization, task checklist extraction, changed-file
  classifier, risk matrix builder.
- `hooks`: pre-verify snapshot, post-verify evidence write, manifest freshness
  check.
- `validation_gates`: goal matched, all tasks satisfied, tests appropriate,
  generated artifacts fresh, no stale evidence, no unrelated regression,
  rollback path present.
- `evidence_schema`: findings, commands run, diff summary, test results,
  graph/knowledge comparison, ICM comparison, unanswered questions, pass/fail
  verdict.

### What `/verify` Must Check

Define verification as exhaustive but bounded:

1. Compare final work against original user goal/request.
2. Compare against task card, OpenSpec tasks, ADR/design notes, and plan.
3. Inspect full diff and classify each changed file.
4. Run focused tests, then broaden based on blast radius.
5. Run Rusty IDD gates: spec status, validate, manifest, workflow-check where
   applicable.
6. Refresh or compare knowledge graphs and report drift.
7. Query ICM for relevant prior decisions/preferences and compare against
   implementation.
8. Check PR/evidence completeness: build, test, lint, secret scan, migration
   note, rollback.
9. Produce a findings-first report: blockers, risks, missing evidence, then
   pass summary.
10. Queue unresolved questions separately instead of burying them in prose.

### Preferred Implementation Path

Implement this in two slices:

1. Add `--stage verify` to `rusty-idd harness package`.
   This gives Codex, Claude, Kimi, or any model the same task-scoped
   verification contract without expanding `.codex`.

2. Add the tiny `/verify` adapter prompt.
   The prompt only loads the package and instructs the model to obey it. No
   giant checklist in `.codex`.

Later, if useful, add:

```bash
rusty-idd verify run --target . --goal-file ... --task-file ... --plan-file ...
```

Do not start there. First make the package authoritative; then make execution
native once the package shape proves itself.

## Required Workflow Artifact Order

1. Recall ICM context for `/verify`, harness packages, tool overflow, and the
   task-scoped adapter boundary.
2. Create a fresh worktree from `origin/develop`.
3. Create this goal file under `.idd/goals/`.
4. Create or select the OpenSpec change `add-verify-package-stage`.
5. Generate graph-backed plan context for this goal.
6. Create proposal, design, spec delta, ADR, and task artifacts.
7. Record `.idd/workflow/active-change`.
8. Verify OpenSpec status.
9. Implement the Rust-owned `verify` package stage.
10. Add the thin `/verify` adapter only after package generation exists.
11. Run focused tests and broaden by blast radius.
12. Refresh `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, validation, and evidence.
13. Commit, push, open a PR to `develop`, and enable auto-merge when green.
