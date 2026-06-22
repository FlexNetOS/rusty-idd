# ADR 0005: Rusty IDD Consumes Handoff And Governs Dot Directories

- **Status:** Accepted
- **Date:** 2026-06-22
- **Change:** `supersede-handoff-dotdir-ownership`
- **Supersedes:** `adr/0004-handoff-outer-single-repo.md`

## Context

The previous handoff single-repo planning decision selected handoff as the outer
canonical repository. The owner clarified that this was the wrong framing:
`meta/handoff` contains central management code and a `.handoff` directory
created by `/harness:handoff-loop-init`, with lineage through
`meta/harness_hub` and `.claude` harness surfaces. That lineage is useful
evidence and compatibility material, but it is not the desired foundation for
the combined product.

Rusty IDD already owns the intent-driven control plane: goal files, generated
knowledge, plan context, OpenSpec, ADRs, manifest, validation, Codex workflow
checks, and migration evidence. Handoff owns useful runtime mechanics: task
cards, claims, checkpoints, done, delivery, fleet state, policy, drift, and
ledger export/import behavior. The corrected architecture needs both without
allowing `.idd`, `.handoff`, `.kb`, `.idea`, `.claude`, `.codex`, `.agents`,
`.github`, and local tool state to become competing sources of truth.

## Decision

Rusty IDD is the canonical repository and workflow engine for the combined
Rusty IDD plus handoff architecture. Rusty IDD consumes `meta/handoff` whole by
adopt-first migration, preserving handoff's tracked source, contracts, and
runtime semantics before any cuts or refactors.

Dot directories are authority-classified:

| Surface | Authority Class | Rule |
|---|---|---|
| `.idd/` | canonical control plane | owns goals, knowledge, plan context, manifest, and validation evidence |
| OpenSpec + `adr/` | canonical planning and decision record | owns change readiness, requirements, immutable decisions, and supersession |
| `.handoff/` | adopted runtime/evidence surface | contributes task, claim, checkpoint, delivery, fleet, and ledger evidence through adapters |
| `.kb/` | workspace knowledge/backlog input | feeds task discovery and planning, but does not replace `.idd` readiness |
| `.idea/` | idea/editor workspace | captures concepts and editor metadata; ideas must graduate into `.idd`, OpenSpec, and ADR |
| `.claude/` and `meta/harness_hub` traces | compatibility source material | useful behavior may be adopted, but these surfaces do not own current intent |
| `.codex/` | agent enforcement | owns Codex hooks, workflow checks, and local execution policy |
| `.agents/` | reusable agent skills | supplies reusable workflow instructions and skills |
| `.github/` | remote delivery gate | owns CI, PR policy, and protected-branch evidence |
| local cache/editor dot dirs | no workflow authority | ignored or tool-owned; never used as durable planning truth |

State precedence in the combined repo is:

1. Git-tracked Rusty IDD source and canonical planning artifacts.
2. `.idd` goal, knowledge, plan-context, manifest, and validation artifacts.
3. OpenSpec proposal/design/spec/tasks and accepted ADRs.
4. Adopted handoff task-card, ledger-event, packet, delivery, fleet, and policy
   evidence through typed Rusty IDD adapters.
5. `.kb` workspace planning and backlog source material.
6. `.idea` concept/editor input.
7. `.claude`, `harness_hub`, and `.handoff/loop` compatibility traces.
8. Local caches, binary ledgers, lock files, editor state, and prose relay files
   that have not been promoted into canonical artifacts.

The first implementation slice after this planning change should import or
mirror the complete tracked `meta/handoff` surface as an upstream/reference
snapshot, excluding Git metadata and untracked runtime caches. Handoff behavior
then moves behind Rusty IDD-owned typed adapters for task cards, claims,
checkpoints, done, delivery, fleet state, policy, drift, and ledger JSONL
compatibility. Only after adapter parity and validation pass may duplicate or
legacy harness surfaces be retired.

## Consequences

- `adr/0004-handoff-outer-single-repo.md` remains historical but is superseded.
- Rusty IDD remains the product identity, planning engine, validation engine,
  and repository authority.
- Handoff is not flattened or guessed from memory. It is consumed whole first,
  then cut only where compile, audit, security, or scope evidence requires it.
- `.handoff` remains valuable evidence, but it no longer outranks `.idd`,
  OpenSpec, or ADRs for current workflow intent.
- Binary `ledger.db`, local lock files, editor caches, and untracked runtime
  outputs are not canonical. Durable handoff evidence must be tracked text,
  typed events, generated packets, or adapter-validated records.
- The repository gains a clear dot-directory model that future agents can use
  before creating more state surfaces.

## Rollback

Revert the planning package and this ADR. No handoff source is moved by this
decision, so rollback is a branch revert rather than a data migration. The
superseded ADR remains available as historical context.
