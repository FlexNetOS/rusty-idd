# upgrade-codex-harness-rusty-idd-flow

## Why

The repo-local Codex harness currently frames Rusty IDD work as an AI_MERGE
or merge-execution process. That breaks the intended Rusty IDD flow, where user
intent enters through graph-backed planning and OpenSpec artifacts before any
implementation or merge work.

## What Changes

- Update the `.codex` harness, agents, loop prompts, invariant checks, and
  related instruction docs to follow the Rusty IDD workflow order.
- Treat `AI_MERGE/` as an optional tool and evidence surface that Rusty IDD may
  read or write, not as the main intent source or authoritative control plane.
- Gate implementation behind graph/context artifacts and OpenSpec status rather
  than jumping from exploration directly to writes.
- Supersede the prior ADR that made `AI_MERGE/` authoritative for all
  integration operations.

## Capabilities

### New Capabilities
- `codex-harness-flow`: Codex harness execution follows the Rusty IDD
  intent-driven workflow before implementation.

### Modified Capabilities
- `idd-spec-engine`: harness-facing status and documentation must present
  OpenSpec and graph artifacts as the primary lifecycle.

## Impact

- `.codex/agents/*.toml`
- `.codex/loops/rusty-idd-model-loop.toml`
- `crates/cli/src/commands/codex.rs`
- `AGENTS.md`
- `.agents/skills/rusty-idd-codex-rust-env/SKILL.md`
- `.agents/skills/rusty-idd-knowledge/SKILL.md`
- `docs/rusty-idd/codex-environment.md`
- `AI_MERGE/13_codex_environment.md` as historical/evidence documentation only
- `adr/0006-ai-merge-as-rusty-idd-tool.md`
