# Repository Instructions for AI Coding Agents

Follow `AGENTS.md` first. Treat Rusty IDD as the intent-driven workflow engine.
`AI_MERGE/` is an audit and evidence tool that Rusty IDD may use, not the main
intent source.

Preferred workflow:

1. Read the user goal and active OpenSpec change completely.
2. Inspect `.idd/knowledge/report.md`, `.idd/knowledge/architecture.md`, and `.idd/knowledge/plan-context.md`.
3. Verify `rusty-idd spec status <change>` before editing.
4. Make the smallest behavior-preserving change authorized by `tasks.md`.
5. Refresh `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and validation artifacts.
6. Update `AI_MERGE/` only when audit, migration, rollback, or merge evidence is required.
7. Never commit secret values.

Do not perform broad cleanup, style-only rewrites, dependency swaps, or folder flattening unless the task explicitly says so.
