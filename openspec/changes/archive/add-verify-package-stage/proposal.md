# add-verify-package-stage

## Why

The `integrate-agent-harness` slice moved always-loaded tool growth into
Rusty IDD-owned task-scoped packages and shipped the first `scan` package. The
next missing stage is post-task verification.

Today, a user can ask Codex or another model to "verify" after implementation,
but that verification tends to live in the model prompt. That recreates the
same harness overflow problem: long always-loaded checklists, model-specific
behavior, and weak handoff between the original request, task plan,
implementation diff, tests, generated artifacts, graph evidence, ICM memory,
and final PR state.

Rusty IDD needs `/verify` to be a thin adapter. The real verification workflow
must live in a Rusty IDD `verify` package that any model can load after a task
is done.

## What Changes

- Add the `verify` workflow stage to Rusty IDD harness package planning.
- Define `/verify` as a lightweight Codex/model adapter that invokes Rusty IDD
  package generation instead of embedding an exhaustive checklist directly.
- Define a `verify` package contract covering exhaustive research, tests,
  cross-verification, diff review, questions, graphs, ICM recall/compare, and
  comparison against the original goal, request, tasks, and plans.
- Require typed verification evidence that records findings, commands, graph
  checks, ICM comparison, unresolved questions, and pass/fail verdict.
- Preserve the always-on harness boundary: `.codex` and peer adapters stay
  small; Rusty IDD owns the package, tools, helpers, hooks, validation gates,
  evidence schema, and future execution path.

## Capabilities

### New Capabilities

- `verify-package-stage`: Post-task verification is a Rusty IDD harness
  package stage that any model adapter can invoke after implementation.

### Modified Capabilities

- `agent-harness-workflow`: The task-scoped harness package catalog expands
  beyond `scan` to include `verify` as the post-task verification stage.

## Impact

- `.idd/goals/add-verify-package-stage.md`
- `openspec/changes/add-verify-package-stage/*`
- `adr/0011-verify-package-stage.md`
- Future implementation: `crates/cli/src/commands/harness.rs`
- Future adapter: `.codex` slash prompt or equivalent thin model prompt
- Future docs: `docs/rusty-idd/codex-environment.md`
- Future evidence: `.idd/evidence/<task>/verification.md`
