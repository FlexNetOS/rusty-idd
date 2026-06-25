# 0018. Unify handoff + prompt_hub into rusty-idd (handoff canonical base)

- Status: accepted
- Date: 2026-06-23

## Context

handoff and prompt_hub are FlexNetOS-owned repos that must be unified into one
rusty-idd product so the standalone repos can later be retired without losing
capability, knowledge, or witnessed-ledger state. The `rusty-idd merge-tools`
INVENTORY phase established the ground truth (evidence under
`AI_MERGE/unify-handoff-prompthub/`):

- **handoff shares 293 relative paths with rusty-idd** (`crates/core,spec,runner,
  cli,tui`). This is the residue of an **earlier poor merge attempt** that pulled
  handoff's structure into rusty-idd incompletely — **handoff is NOT a fork of
  rusty-idd; handoff is the canonical base.**
- **prompt_hub shares only 18 (meta) paths** and is independent.

Prior handoff-adoption ADRs (0004 handoff-outer-single-repo, 0005 consumes-handoff-
dotdirs, 0006 adopt-handoff-as-upstream-reference, 0008 refresh-handoff-kb) treated
handoff as an upstream reference mirror under `third_party/upstream/handoff`. That
reference-mirror posture is insufficient for the unification goal (it never made
handoff first-class code) and the prior code-level merge attempt failed. This ADR
records the active unification decision; the prior ADRs remain accepted historical
inputs (not superseded edits).

## Decision

1. **handoff is the canonical base.** Where a shared file diverges, handoff's
   implementation is authoritative; rusty-idd's genuine forward additions (the
   `deploy`/`next`/`render` control plane, ADR-0015/0017, spec-engine work) are
   reconciled **onto** handoff — **upgrade-only**, losing no capability from either
   side. handoff also contributes its unique `hf`/`ledger`/`work-order` crates and
   the `.handoff` witnessed continuity kernel.
2. **prompt_hub is integrated additively** — its distinct crates are absorbed
   without renaming existing crates; only shared meta files reconcile.
3. **The unification follows the merge-tools workflow** (inventory → plan →
   decide → implement → verify → evidence) as **parity-tested vertical slices**;
   no big-bang flatten; deprecate-before-remove (behavior preserved until parity).
4. **Absorbed code is first-class in the code graph / `.kb`.** Imported code is
   indexed, not excluded. (Owned-and-unified code is treated differently from
   `third_party/upstream/` third-party mirrors, which stay excluded.)
5. **Faithful adoption.** Each repo's complete current state is imported intact,
   including its `.kb` knowledge base and `.handoff` witnessed ledgers; nothing is
   stripped during adoption. Any later removal is a separate evidence-backed cut.
6. **Import staging under `imports/`**, migrated into unified `crates/` by slices;
   `imports/` removed only after parity passes.
7. **AI_MERGE is excluded from the knowledge code-context pack.** It is an evidence
   surface (AGENTS.md), and the pack was at its token budget; excluding `AI_MERGE/**`
   keeps `knowledge refresh` healthy as merge evidence grows. The code graph still
   indexes all code.

## Consequences

- One unified rusty-idd absorbs both owned repos as first-class, code-graph-indexed
  code, enabling the standalone repos' later retirement without capability/knowledge
  loss.
- The 293-path handoff reconciliation is done incrementally (one subsystem per
  parity-tested slice), not as a mega-PR — directly avoiding the prior failure mode.
- prompt_hub's MSRV (1.91/1.96) and `--all-features` constraints are resolved at its
  membership slice as evidenced merge work, upgrade-only.
- This ADR governs the unification; implementation lands as subsequent narrow PRs
  under change `unify-handoff-prompthub`.
