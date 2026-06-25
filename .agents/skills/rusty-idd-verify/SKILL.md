---
name: rusty-idd-verify
description: Use to verify a completed Rusty IDD task before PR/merge. A thin adapter — it loads the engine-owned verify package and obeys it; it does NOT carry the verification checklist itself.
---

# Rusty IDD Verify

Use this skill (the `/verify` adapter) after implementation, before opening or
merging a PR. It is intentionally minimal: the verification workflow — roles,
contracts, tools, gates, and evidence schema — lives in the Rusty IDD engine,
not in this prompt (ADR-0010, ADR-0011, ADR-0015).

## Rule

Do not embed a verification checklist here. Load the package and follow it.

## Workflow

1. Generate the verify package from the engine:

   ```bash
   rusty-idd harness package --stage verify --target . \
     --goal-file <goal> --task-file <task-card> --plan-file <plan>
   ```

   (`--format json` for programmatic consumption; omit the file flags when a
   given input does not exist.)

2. Obey the package contract it returns:
   - work the `agent_team` roles and `contracts`;
   - run only the package's `tools` (git diff/log/status, `rusty-idd validate`,
     `manifest`, `knowledge refresh`/`plan-context`, `spec status`, focused
     build/test/lint, ICM recall/context-compare);
   - cross-check the implementation against the original request, goal, task
     card, OpenSpec tasks, plan, diff, tests, generated artifacts, knowledge
     graph, and ICM memory;
   - satisfy every `validation_gate` and populate every `evidence_schema` field
     before emitting a pass/fail verdict.

3. Emit the typed verification evidence the package declares (findings, commands
   run, diff summary, test results, graph/knowledge comparison, ICM comparison,
   unanswered questions, pass/fail verdict with rollback risk). Hand it to the
   PR/merge evidence.

## Missing capability

If verification needs something the package does not provide, that is a **missing
Rusty IDD package capability**. Report it and extend the engine's verify package
(`crates/cli/src/commands/harness.rs`) through the normal Rusty IDD flow. Do NOT
solve it by growing an always-loaded prompt or hand-authored checklist in this or
any adapter directory — that is the token black hole this design removes.
