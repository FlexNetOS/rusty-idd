---
name: rusty-idd-codex-rust-env
description: "Use when setting up, auditing, or repairing the Rusty IDD Codex environment: repo skills, hooks, custom agents, knowledge artifacts, Rust gates, and multi-agent workflow execution."
---

# Rusty IDD Codex Rust Environment

Use this skill to operate the repo-local Codex environment.

## Source Order

1. Current repo files are authoritative for implementation.
2. For Codex product behavior, use the current Codex manual through the `openai-docs` skill.
3. Use ICM before work because this repo requires persistent memory recall.

## Environment Surfaces

- `AGENTS.md`: durable repository rules.
- `.codex/config.toml`: project-scoped Codex settings.
- `.codex/rules/*.rules`: project-scoped exec policy.
- `.codex/hooks.json`: lifecycle hook registration.
- `.codex/agents/*.toml`: project custom subagents.
- `.codex/loops/*.toml`: dry-run-first multi-model loop definitions.
- `rusty-idd codex env-check`: Rust-native invariant checker.
- `rusty-idd codex runtime-audit`: Rust-native audit proving the repo-local Codex runtime
  is not using Python hooks/scripts.
- `rusty-idd codex system-audit`: Rust-native audit proving the active Codex binary and
  parent-managed source-build path are Rust-first, while classifying upstream Python tooling.
- `rusty-idd codex model-loop`: Rust-native model-loop command generator/runner.
- `.agents/skills/*/SKILL.md`: repo-scoped workflows.
- `.idd/knowledge/index.json` and `.idd/knowledge/report.md`: compact codebase knowledge.

## Standard Loop

1. Recall ICM context.
2. Read `.idd/knowledge/report.md`.
3. Query symbols/files/impact before rescanning source.
4. Use subagents only when explicitly useful:
   - `rusty-idd-explorer` for read-heavy mapping.
   - `rusty-idd-gap-hunter` for omissions.
   - `rusty-idd-verifier` for evidence and gates.
   - keep one writer, usually `rusty-idd-implementer`.
5. Apply the adopt-first workflow for integrations.
6. Own tool-surface growth when it improves output:
   - Add or update skills, rules, hooks, custom agents, model-loop passes,
     plugin packaging, MCP configuration, or local Rust helpers when evidence
     shows they would materially improve accuracy, speed, verification, or
     repeatability.
   - Keep the addition narrow, tracked, documented, and verified.
   - Do not wait for the user to request the exact tool when the need is clear
     from prior misses or current audit evidence.
7. Upgrade only:
   - Prefer the latest stable, more capable tracked path when improving tools,
     dependencies, models, actions, skills, hooks, or generated artifacts.
   - Never downgrade a working surface to make a task easier unless concrete
     build, audit, compatibility, or owner-boundary evidence requires a scoped
     hold.
8. Treat stale or orphaned work as unfinished by default:
   - If a stale artifact, orphaned file, skipped TODO, ignored output, or
     disconnected tool surface appears, either prove it is intentionally local
     and ignored or finish it.
   - Finishing means documenting the decision, regenerating affected artifacts,
     and running the relevant gates.
9. For multi-model loop work, start with a dry run:
   `cargo run --bin rusty-idd -- codex model-loop`
   Inspect the emitted `codex exec` commands before using `--execute`.
10. Verify with focused gates, then broaden.
11. Refresh generated artifacts and run the Codex invariant check.

## Verification

```bash
just codex-env-check
just codex-runtime-audit
cargo run --bin rusty-idd -- codex system-audit
cargo run --bin rusty-idd -- codex model-loop
just knowledge
just manifest
just validate
```

Use `just ci` before claiming a complete repo-wide change.

## Boundaries

- Do not run host service management commands.
- Do not install binaries into user/system locations from this repo workflow.
- Do not add required tools to user-global settings. Required external tools belong in the parent
  `meta`/`envctl` tool contract.
- Do not commit ad hoc context packs.
- Do not widen daemon or host-service surfaces from this repo.
- MCP, vector, cloud, plugin, and provider surfaces may be added only when the
  agent has evidence that they improve output accuracy, speed, verification, or
  repeatability, and only with a narrow feature gate or documented boundary.
