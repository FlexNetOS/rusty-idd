# Unify handoff + prompt_hub into rusty-idd Goal

rusty-idd --goal-file .idd/goals/unify-handoff-prompthub.md

Unify the FlexNetOS-owned `handoff` and `prompt_hub` repositories into rusty-idd
as first-class parts of one product, following the Rusty IDD **merge-tools**
workflow (`rusty-idd merge-tools show`) and the AGENTS.md North Star: *unify by
preserving working behavior, making contracts explicit, and merging only through
reviewable, test-backed increments.*

These are NOT third-party upstreams, and the direction of truth matters:
**`handoff` is the CANONICAL BASE — it is NOT a fork of rusty-idd.** The 293
relative paths `handoff` shares with rusty-idd (`crates/core,spec,runner,cli,tui`)
are the residue of an **earlier POOR merge attempt** that pulled handoff's
structure into rusty-idd incompletely. So where a shared file diverges, **handoff
is authoritative**; rusty-idd's genuine *forward* additions (e.g. the Phase-1
`deploy` front door, ADR-0017, the `next`/`render` control plane) are reconciled
*onto* handoff's canonical base — upgrade-only, never losing a capability from
either side. handoff also contributes its unique `hf` / `ledger` / `work-order`
crates and its `.handoff` witnessed continuity kernel. `prompt_hub` is, by
contrast, **independent** (only 18 shared meta-file paths): `prompt-hub` lib +
`prompthub` CLI + `prompthub-server` + its `prompt-loop` harness + `.kb`, folded
in additively. The unified rusty-idd's **code graph and `.kb` knowledge base must
span all of this as first-class code** — the consolidation is worthless if
rusty-idd is blind to the code it absorbs.

A prior poor merge attempt already failed here (and I repeated its mistakes in
this session before resetting). This goal MUST learn from that: handoff canonical,
inventory-before-flatten, faithful adopt, parity-tested vertical slices.

## Required Method — the merge-tools 6 phases (do them in order)

1. **inventory** (FIRST, before any flattening — Operating Rule 1): generate, for
   both repos, the `RepoInventory`, feature matrix, env/secret contract, and
   legacy-surface inventory, plus the divergence map of handoff↔rusty-idd shared
   crates. Gates: `rusty-idd scan`, `rusty-idd knowledge refresh`, no untracked
   secret material.
2. **plan**: bind this goal with `rusty-idd knowledge plan-context` and create ONE
   OpenSpec change (proposal, spec deltas, design, tasks). Gates: `spec status` /
   `spec next`.
3. **decide**: one active ADR for the unification architecture + migration note;
   summarize prior merge decisions as inputs, don't resurrect them.
4. **implement**: apply ONE narrow vertical slice at a time, adopt-first,
   preserving old behavior until parity is proven (no stubs, no downgrades, core
   crate stays zero-dep). Reconcile the crate-name collisions and toolchain/feature
   constraints as evidenced merge work, not as guesses.
5. **verify**: `cargo build/test/fmt/clippy --workspace` + `rusty-idd validate
   --workspace .`; refresh `.idd/knowledge/*` + `MANIFEST.tsv`.
6. **evidence**: PR evidence bundle, migration note (old path → new path),
   rollback path, manifest state, AI_MERGE audit note.

## Hard Constraints (lessons already paid for)

- **Adopt as-is, faithfully** (Codex Rule 5): bring the complete current state
  forward intact — including `.kb` knowledge bases and `.handoff` witnessed
  ledgers (handoff `.handoff/ledger.db` + `.rvf`, prompt_hub `.handoff/ledger.db`).
  Do NOT strip binaries, db/onnx/sarif, or knowledge-base state during adoption;
  any cut happens later, only with recorded evidence.
- **Code graph is first-class**: the adopted code MUST be indexed in rusty-idd's
  code graph / `.kb`, not excluded. Do not hide absorbed code from code
  intelligence.
- **Upgrade only, never downgrade** a working surface, dep, or generated artifact.
- **Test-backed increments**: narrow PRs, one vertical slice each; preserve
  behavior until parity tests pass; deprecate before remove.

## Decision Target

rusty-idd SHALL absorb handoff and prompt_hub as first-class, code-graph-indexed
parts of one unified product, via the merge-tools workflow, with explicit
inventories/contracts produced before any flattening and behavior preserved
through test-backed increments — so the standalone repos can subsequently be
unregistered and archived without losing capability, knowledge, or ledger state.

## Non-Goals (this unification goal)

- Live fleet deployment, repo unregistration, and archiving are the sequenced
  follow-on phases; this goal delivers the unification itself.
- No big-bang flatten: no single mega-commit dumping both trees before the
  inventory/contract maps exist.
