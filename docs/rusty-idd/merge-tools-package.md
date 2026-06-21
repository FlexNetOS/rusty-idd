# Rusty IDD Merge Tool Package

Rusty IDD now owns merge, migration, and repository-unification workflow as a
packageable Rust surface instead of an active `idd-merge-idd` Claude/Gemini
bridge. The package is exposed through:

```bash
cargo run --bin rusty-idd -- merge-tools show
```

## Deprecated merge content scan

The deprecated merge content was concentrated in these surfaces:

| Surface | Finding | Disposition |
| --- | --- | --- |
| `.claude/agents/*` and `.claude/skills/*merge*` | Active bridge prompts still taught the old `idd-merge-idd` loop. | Retired as active workflow; useful contracts consolidated into `crates/merge-tools`. |
| `.gemini/agents/*` and `.gemini/skills/*merge*` | Mirror of the same old bridge prompts. | Retired as active workflow; `GEMINI.md` is a bridge note only. |
| `_workspace/{backlog,loop_state,HANDOFF}.md` | Run-local continuity state from the old loop. | Removed from active tracked state; OpenSpec tasks and `.idd/knowledge/*` carry current state. |
| `AI_MERGE/` | Historical audit and merge evidence. | Kept only as evidence/history; Rusty IDD remains the workflow source. |
| `adr/0001..0006` | Mixed legacy architecture decisions, several pre-dating current workflow ownership. | Summarized here and replaced by a single active ADR for Codex harness flow. |
| `research_ai_autopilot_merge.md` and `idd-merge-workspace.code-workspace` | Bootstrap research and local editor state for the old merge era. | Retired from active repo state. |

## Package boundary

`crates/merge-tools` defines the reusable merge package as data and verification:

- phases: inventory, plan, decide, implement, verify, evidence;
- gates: Rusty IDD knowledge, OpenSpec status, build/test/lint/validation, and
  manifest refresh;
- legacy-surface dispositions for Claude/Gemini bridge files, `_workspace`,
  `AI_MERGE`, and retired ADRs;
- `rusty-idd merge-tools verify` replaces the deleted bridge drift scripts in CI.

The crate intentionally does not run agents, spawn daemons, or mutate peer repos.
It is a Rust-owned workflow package that CLI, docs, and future automation can
reuse without resurrecting the old bridge as the control plane.

## Legacy decision summary

The previous active ADR directory contained decisions for the early unified CLI,
TUI inclusion, AI_MERGE control-plane authority, direct crate knowledge
integration, upstream knowledge integration, and AI_MERGE as evidence. Those
decisions are now represented as implementation history in docs and source:

- CLI, TUI, spec, knowledge, and merge-tool package boundaries live in
  `Cargo.toml`, `README.md`, and `docs/rusty-idd/design.md`.
- Direct crate knowledge integration remains embodied in `crates/knowledge` and
  the vendored CodeGraph/repomix crates.
- AI_MERGE is retained as optional audit evidence, not as the authoritative
  workflow.
- The active ADR directory is intentionally reduced to the single active ADR
  for the Codex harness flow.

## Active bridge rule

Claude and Gemini repo files are compatibility notes only. They must point back
to `AGENTS.md`, `.agents/skills`, `.idd/knowledge/*`, OpenSpec status, and
`rusty-idd merge-tools show`. They must not reintroduce `idd-merge-idd` as the
source of truth.

## Rollback

If a removed bridge surface is needed for evidence, recover it from Git history
and port only the required contract into `crates/merge-tools` or a Rusty
IDD-owned skill. Do not restore retired bridge prompts as the active workflow.
