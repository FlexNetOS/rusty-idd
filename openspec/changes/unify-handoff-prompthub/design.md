# unify-handoff-prompthub — Design

## Context

The merge-tools INVENTORY phase proved the structure: handoff is a 293-shared-path
**canonical base** (rusty-idd's `core/spec/runner/cli/tui` are residue of a prior
poor merge from handoff); prompt_hub is **independent** (18 meta-file overlaps).
A prior poor merge failed here; this design follows the canonical
`rusty-idd merge-tools` workflow and AGENTS.md North Star (preserve behavior,
explicit contracts, reviewable test-backed increments) so it does not repeat it.

## Goals / Non-Goals

**Goals:**
- One unified rusty-idd that absorbs handoff (canonical base + unique
  `hf`/`ledger`/`work-order`/`.handoff`) and prompt_hub (additive), with the code
  graph spanning all of it as first-class code.
- Faithful adoption (no stripping; `.kb` + witnessed ledgers preserved).
- Behavior preserved via parity-tested vertical slices; upgrade-only.

**Non-Goals (this change = inventory + plan + decide only):**
- No source merged into `crates/` yet — implementation slices are subsequent PRs.
- No fleet deploy / unregister / archive (later goal phases).
- No big-bang flatten.

## Decisions

1. **handoff canonical, reconcile rusty-idd forward additions onto it.** For each
   of the 293 shared paths, handoff is the base; rusty-idd's forward work (deploy
   front door, `next`/`render`, ADR-0015/0017, the spec-engine improvements) is
   merged on top, upgrade-only. Each reconciled subsystem is a vertical slice with
   parity tests before dedup.
2. **prompt_hub additive.** Its `prompt-hub`/`prompthub`/`prompthub-server` crates
   are absorbed without renaming existing crates; only ~18 shared meta files
   reconcile. Its MSRV (1.91/1.96) + `--all-features` constraints are resolved at
   the slice that makes it a workspace member, not before.
3. **Import staging under `imports/`** (merge-tools recommended tree): slice 1
   brings each repo's complete current state into `imports/handoff/` and
   `imports/prompt_hub/` intact (faithful — `.kb` + `.handoff` ledgers included),
   **indexed by the code graph** (NOT excluded). Later slices migrate canonical
   code from `imports/` into unified `crates/`; `imports/` is removed only after
   parity passes (deprecate-before-remove).
4. **Code graph is first-class over absorbed code.** The earlier instinct to
   exclude absorbed code from the index was wrong and is rejected. `imports/` is
   indexed. (Contrast: `third_party/upstream/` mirrors remain excluded — those are
   genuine third-party references, not owned code being unified.)
5. **AI_MERGE evidence excluded from the code-context pack (engine fix).** The
   knowledge report/architecture packs were at the 800K-token budget and ingested
   the entire `AI_MERGE/**` evidence surface, so any merge-evidence addition broke
   `knowledge refresh`. Per AGENTS.md (AI_MERGE is an evidence surface), the packs
   now exclude `AI_MERGE/**`. This does NOT touch the code graph (index.json still
   indexes all code); it only keeps the bounded AI *code*-context bundle code-only.
6. **Evidence is distilled, not bloated.** The full regenerable `rusty-idd plan`
   workspaces (which also emit a 447K rusty-idd self-inventory + boilerplate) are
   not committed; the distilled analysis docs are, with `REGENERATE.md` recording
   the commands. Nothing of analytical value is lost.

## Slice Sequence (merge-tools 5-task, per source repo)

1. **import-without-flattening** — `imports/{handoff,prompt_hub}/` complete state,
   code-graph-indexed; no workspace flattening.
2. **normalize env/secret contracts** — one SecretProvider + env order.
3. **canonical interfaces** — seams for the 293 shared paths (handoff base) and
   prompt_hub crates before moving implementations.
4. **parity tests** — old==new per migrated subsystem.
5. **CI + validation** — gate unified crates; remove duplicates post-parity.

handoff slices reconcile the shared lineage (heavier); prompt_hub slices are
additive (lighter).

## Risks / Trade-offs

- **293-path reconciliation is large.** Mitigation: one subsystem per slice
  (core → spec → runner → cli → tui → hf/ledger/work-order), parity-gated; never a
  single mega-PR.
- **MSRV / `--all-features` (prompt_hub).** Mitigation: deferred to the membership
  slice, reconciled as evidenced merge work (raise MSRV / fix feature combos),
  upgrade-only.
- **Import size + nested `.git` / build caches.** Mitigation: import excludes
  `.git` (can't nest) and `target/` (regenerable build cache); everything else —
  source, `.kb`, ledgers — is faithful.

## Migration / Rollback

- Old paths (handoff/prompt_hub source) are deprecated, not deleted, until parity
  passes (AGENTS.md Rule 3). Rollback = revert the slice PR; the standalone repos
  + GitHub remotes remain authoritative until the later retirement phase.
