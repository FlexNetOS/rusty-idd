# unify-handoff-prompthub

## Why

handoff and prompt_hub are FlexNetOS-owned components that must become first-class
parts of one rusty-idd product (so the standalone repos can later be retired
without losing capability, knowledge, or witnessed-ledger state). The
merge-tools INVENTORY phase (committed under `AI_MERGE/unify-handoff-prompthub/`)
established the ground truth:

- **handoff is the CANONICAL base — not a fork of rusty-idd.** It shares **293**
  relative paths with rusty-idd (`crates/core,spec,runner,cli,tui`) because an
  earlier *poor merge attempt* pulled handoff's structure into rusty-idd
  incompletely. Where a shared file diverges, **handoff is authoritative**;
  rusty-idd's genuine forward additions (the `deploy`/`next`/`render` control
  plane, ADR-0015/0017) are reconciled *onto* handoff — upgrade-only. handoff also
  uniquely contributes `hf`, `ledger`, `work-order`, and the `.handoff` witnessed
  continuity kernel.
- **prompt_hub is independent** — only **18** shared (meta) paths; its
  `prompt-hub` lib + `prompthub` CLI + `prompthub-server` + prompt-loop + `.kb`
  are folded in **additively**.

A prior poor merge already failed here. This change follows the canonical
`rusty-idd merge-tools` workflow (inventory → plan → decide → implement → verify
→ evidence) and AGENTS.md Operating Rule 1 (no flattening before inventories /
feature matrix / contract maps — now satisfied) so it does not repeat that
failure.

## What Changes

- A **repo-unification capability** is specified: handoff and prompt_hub are
  absorbed into rusty-idd as first-class, **code-graph-indexed** code, with
  behavior preserved through parity-tested vertical slices, handoff as the
  canonical base, and nothing stripped during adoption (knowledge bases and
  witnessed ledgers included).
- The implementation proceeds as **narrow, parity-tested vertical slices** (the
  merge-tools 5-task sequence, applied per source repo):
  1. **import-without-flattening** — bring each repo's complete current state in
     intact (faithful; `.kb` + `.handoff` ledgers included), indexed by the code
     graph; no cargo-workspace flattening yet.
  2. **normalize env/secret contracts** — one SecretProvider + env resolution
     order across the unified tree.
  3. **canonical interfaces** — define the seams before moving implementations
     (handoff canonical for the 293 shared paths; rusty-idd forward additions
     merged on).
  4. **parity tests** — prove old==new behavior before any dedup/removal.
  5. **CI + validation** — gate the unified crates.
- **Engine fix (already in this change):** the knowledge code-context *report
  pack* no longer ingests the `AI_MERGE/**` evidence surface (it was at budget
  and any evidence addition broke `knowledge refresh`). Per AGENTS.md, AI_MERGE is
  an evidence surface, not code-context; the code graph still indexes all code.

## Capabilities

### New Capabilities
- `repo-unification`: Absorb FlexNetOS-owned repos (handoff canonical base,
  prompt_hub additive) into rusty-idd as first-class code-graph-indexed code via
  the merge-tools workflow — inventory-before-flatten, faithful adopt (no
  stripping), parity-tested vertical slices, upgrade-only, behavior preserved
  until parity proven.

### Modified Capabilities
<!-- none in this planning slice; subsequent implementation slices may modify the
idd-spec-engine / harness capabilities as handoff's canonical crates reconcile
with rusty-idd's forward additions, each with its own spec delta. -->

## Impact

- New spec capability `openspec/specs/repo-unification/`.
- New `adr/0018-*` recording the unification architecture (handoff canonical base,
  merge-tools slices, code-graph-first, faithful adopt).
- Engine: `crates/knowledge/src/lib.rs` report/architecture packs exclude
  `AI_MERGE/**` (evidence surface) so `knowledge refresh` stays within budget as
  merge evidence grows.
- Evidence: `AI_MERGE/unify-handoff-prompthub/` (inventories, feature matrices,
  env/secret contracts, merge plans, conflict registers, parity plans, task
  breakdowns) + `REGENERATE.md`.
- No source from handoff/prompt_hub is merged into rusty-idd's crates in THIS
  change; it delivers the inventory + plan + decision. Implementation slices land
  as subsequent narrow PRs under this change's tasks.
