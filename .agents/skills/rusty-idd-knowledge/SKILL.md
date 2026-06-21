---
name: rusty-idd-knowledge
description: "Use when investigating Rusty IDD with generated knowledge artifacts, querying .idd/knowledge indexes and reports, or preparing bounded repo context packs for agents."
---

# rusty-idd Knowledge Skill

Use this skill when investigating this repository with the generated `rusty-idd knowledge`
artifacts or when preparing compact context for an agent.

## Workflow

1. Start with `.idd/knowledge/report.md`.
   - Check the workspace fingerprint, file count, token count, top files, hotspots, and findings.
   - If the report is missing or stale, run:
     `rusty-idd knowledge refresh --workspace .`

2. Query the graph index before rescanning source manually.
   - Symbol lookup:
     `rusty-idd knowledge query --index .idd/knowledge/index.json --symbol NAME`
   - File lookup:
     `rusty-idd knowledge query --index .idd/knowledge/index.json --file PATH`
   - Impact lookup:
     `rusty-idd knowledge query --index .idd/knowledge/index.json --impact NODE_ID`

3. Use focused pack outputs for temporary context.
   - Prefer `/tmp` for exploratory packs:
     `rusty-idd knowledge pack --workspace . --out /tmp/rusty-idd-pack.xml --style xml --compress`
   - Use `--include` and `--ignore` to narrow the bundle.
   - Use `--remove-comments`, `--remove-empty-lines`, `--line-numbers`, `--truncate-base64`,
     `--git-diff`, and `--git-log` only when they directly help the task.
   - Do not commit ad hoc pack files.

4. Read compactly.
   - Grep or slice generated packs and indexes instead of dumping the full file.
   - Treat `.idd/knowledge/index.json` and `.idd/knowledge/report.md` as durable control-plane
     artifacts; keep them deterministic and bounded.

5. Stay in-process.
   - Do not start MCP servers, daemons, or host services for this workflow.
   - Optional vector search, SurrealDB, and cloud provider integrations are feature-gated future
     surfaces, not default behavior.

## Validation

After changing source or control-plane files, refresh artifacts and run:

```bash
cargo run --bin rusty-idd -- knowledge refresh --workspace .
cargo run --bin rusty-idd -- validate --workspace .
```
