# Agent Workflow

## Recommended sequence

1. Run `idd plan` against the two repos.
2. Commit the generated `AI_MERGE`, `.idd`, `.github`, `AGENTS.md`, and `SECURITY.md` files to an integration branch.
3. Create one issue per generated or hand-written task.
4. Assign one issue to one coding agent.
5. Require each PR to update the affected control-plane document.
6. Require CI, `idd validate`, and manifest refresh.
7. Merge only through the integration branch.
1. Capture intent through Rusty IDD graph-backed context.
2. Create or select an OpenSpec change.
3. Write proposal, specs, design, ADR decisions, and tasks in schema order.
4. Assign one ready task set to one coding agent.
5. Require each PR to update the affected Rusty IDD artifacts.
6. Require CI, `rusty-idd validate`, knowledge refresh, and manifest refresh.
7. Record AI_MERGE evidence only when audit, migration, rollback, or merge records are required.

## Agent instruction block

Paste this into agent prompts or GitHub issue bodies:

```text
Use AGENTS.md and AI_MERGE as the source of truth. Do not perform broad cleanup.
Implement only this task. Keep old code until parity tests prove the migration.
Use AGENTS.md, .idd/knowledge, OpenSpec, ADRs, and tasks.md as the source of truth. Do not perform broad cleanup.
Implement only after OpenSpec status shows the change is ready. Keep old code until parity tests prove the migration.
Update the env/secrets contract if configuration behavior changes. Do not commit secrets.
Return a small PR with build/test/lint/validation evidence and rollback notes.
Refresh .idd/MANIFEST.tsv if generated control-plane files change.
```

## Anti-drift rules

- One issue means one narrow intent.
- One branch means one reviewable PR.
- Do not let multiple agents mutate the same files at the same time.
- Do not use a cloud coding agent task as a multi-repo transaction.
- If the task is too large, split it through `AI_MERGE/08_agent_queue.md`.
- If the task is too large, split it through OpenSpec tasks and use `AI_MERGE/08_agent_queue.md` only as supporting queue evidence when needed.
