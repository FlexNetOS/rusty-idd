---
name: rusty-idd-knowledge
description: "Use when investigating Rusty IDD with generated knowledge artifacts, querying .idd/knowledge indexes and reports, or preparing bounded repo context packs for agents."
---

# rusty-idd Knowledge Skill

Use this skill when investigating this repository with the generated `rusty-idd knowledge`
artifacts or when preparing compact context for an agent.

## Workflow

1. Start with `.idd/knowledge/report.md` and `.idd/knowledge/architecture.md`.
   - Check the workspace fingerprint, file count, token count, top files, hotspots, and findings.
   - Use the architecture graph to map repo components, integration surfaces, OpenSpec
     stages, and evidence paths before deciding which files to edit.
   - If the report is missing or stale, run:
     `rusty-idd knowledge refresh --workspace .`

2. Query the graph index before rescanning source manually.
   - The index uses the vendored CodeGraph tree-sitter registry for supported
     Rust, TypeScript/JavaScript, Python, Go, Java, C/C++, Swift, Ruby, and PHP
     files. Treat missing languages as explicit parser-surface gaps, not proof
     that tree-sitter is absent from the system.
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

4. Generate a fresh architecture graph when integration boundaries are unclear.
   - Markdown for reading:
     `rusty-idd knowledge architecture --workspace . --out /tmp/rusty-idd-architecture.md`
   - JSON for tooling:
     `rusty-idd knowledge architecture --workspace . --out /tmp/rusty-idd-architecture.json`
   - The graph combines CodeGraph-backed source structure with repomix-backed context
     package metrics and maps both to Rusty IDD automation stages.

5. Generate a system graph when the task crosses repo boundaries.
   - Use the parent meta workspace when working from this checkout:
     `rusty-idd knowledge system-architecture --workspace . --system-root .. --out /tmp/rusty-idd-system-architecture.md`
   - The system graph prefers `meta project list --json` and falls back to immediate
     child git repo discovery.
   - Use it to map Rusty IDD to handoff, weave, Obscura, Yazelix, envctl,
     prompt/meta producers, hubs, and agent environment repos before planning a
     cross-repo integration.
   - The command is read-only: it records peer repo state but does not start
     MCP servers, daemons, host services, or mutate peer repos.

6. Read compactly.
   - Grep or slice generated packs and indexes instead of dumping the full file.
   - Treat `.idd/knowledge/index.json`, `.idd/knowledge/report.md`,
     `.idd/knowledge/architecture.json`, `.idd/knowledge/architecture.md`,
     `.idd/knowledge/system-architecture.json`, and
     `.idd/knowledge/system-architecture.md`
     as durable control-plane artifacts; keep them deterministic and bounded.

7. Stay in-process.
   - Do not start MCP servers, daemons, or host services for this workflow.
   - MCP, daemon, domain, vector, SurrealDB, and cloud/provider integrations may
     exist in the wider meta system, but they are feature-gated or external
     surfaces rather than default knowledge behavior.

## Validation

After changing source or control-plane files, refresh artifacts and run:

```bash
cargo run --bin rusty-idd -- knowledge refresh --workspace .
cargo run --bin rusty-idd -- knowledge architecture --workspace . --out /tmp/rusty-idd-architecture.md
cargo run --bin rusty-idd -- knowledge system-architecture --workspace . --system-root .. --out /tmp/rusty-idd-system-architecture.md
cargo run --bin rusty-idd -- validate --workspace .
```
