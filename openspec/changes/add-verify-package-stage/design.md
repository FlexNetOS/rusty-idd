# add-verify-package-stage - Design

## Context

Rusty IDD now has a task-scoped harness package surface. The first package
slice, `scan`, proved the correct ownership boundary: Rusty IDD creates or
selects a package, while agent directories stay minimal adapters.

Verification needs the same treatment. A `/verify` prompt should work for
Codex or any model, but it should not be the source of truth for verification
logic. The prompt should delegate to Rusty IDD's `verify` package, then follow
the package contract.

## Goals

- Define `verify` as a Rusty IDD harness package stage.
- Make `/verify` a small adapter that loads the package and obeys it.
- Make verification exhaustive across goal, request, tasks, plans, diffs,
  tests, generated artifacts, graphs, ICM memory, evidence, questions, and
  rollback risk.
- Produce a typed verification evidence schema that can be validated and handed
  off to PR/merge evidence.

## Non-Goals

- Do not turn `.codex` into the verification engine.
- Do not add default MCP/server surfaces to verification.
- Do not replace the existing Rusty IDD validation command in this planning
  slice.
- Do not implement a full executor until the package contract is reviewed and
  stable.

## Package Shape

The target invocation is:

```bash
rusty-idd harness package \
  --stage verify \
  --target . \
  --goal-file <goal.md> \
  --task-file <task.md> \
  --plan-file <plan.md> \
  --format markdown
```

The package should expose:

- `agent_team`: verifier, diff-auditor, test-runner, evidence-checker,
  goal-comparator, graph-checker, icm-checker, risk-reviewer.
- `contracts`: original request contract, goal contract, task/plan contract,
  implementation diff contract, test contract, graph contract, ICM comparison
  contract, evidence contract, adapter-boundary contract.
- `tools`: `git status`, `git diff`, `git log`, `rusty-idd validate`,
  `rusty-idd manifest`, `rusty-idd knowledge refresh`,
  `rusty-idd knowledge plan-context`, `rusty-idd spec status`, focused
  build/test/lint gates, ICM recall/context compare.
- `helpers`: goal normalization, task checklist extraction, changed-file
  classifier, risk matrix builder, original-request comparator, queue/question
  extractor.
- `hooks`: pre-verify snapshot, generated-artifact freshness check, evidence
  write check, stop-phase workflow check.
- `validation_gates`: goal matched, tasks satisfied, tests appropriate, diff
  reviewed, generated artifacts fresh, graph/ICM checked, evidence complete,
  unresolved questions explicit, rollback path present.
- `evidence_schema`: findings, commands run, diff summary, test results,
  graph/knowledge comparison, ICM comparison, unanswered questions, pass/fail
  verdict, rollback risk.

## Adapter Shape

`/verify` should contain only enough text to delegate:

```text
Run post-task verification through Rusty IDD, not through this prompt body.

1. Load the Rusty IDD verify package:
   rusty-idd harness package --stage verify --target . --goal-file <goal> --task-file <task> --plan-file <plan> --format markdown

2. Follow that package exactly.
3. Produce a verification report with pass/fail findings, evidence, unresolved questions, and rollback risk.
4. Do not invent extra always-loaded .codex skills/tools. If capability is missing, report the missing Rusty IDD package capability.
```

## Execution Strategy

1. Add package planning artifacts first: goal, OpenSpec proposal/design/spec,
   tasks, ADR, plan-context, evidence, and manifest.
2. Implement `--stage verify` in the existing `harness package` command after
   artifacts are ready.
3. Add focused tests for the package shape and adapter boundary.
4. Add the minimal `/verify` prompt only after the Rust package exists.
5. Refresh knowledge, manifest, validation, and PR evidence before merge.

## Risks

- The package can become a giant checklist. Mitigation: keep the exhaustive
  workflow in Rusty IDD package content, not in every adapter prompt.
- Verification can be expensive. Mitigation: the package should define focused
  first-pass gates and broaden based on diff blast radius.
- ICM comparisons can become noisy. Mitigation: the package should require
  relevant recall queries and explicit compare notes, not broad memory dumps.
