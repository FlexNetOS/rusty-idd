# 45 — Unify handoff + prompt_hub: plan phase (merge-tools)

Evidence note for `feat/unify-handoff-prompthub`. After a wrong-way improvised
adoption (reset), this redoes the unification THE RIGHT WAY through the canonical
`rusty-idd merge-tools` workflow. This PR delivers the **inventory + plan +
decide** phases only — no handoff/prompt_hub source is merged into `crates/` yet.

## What this PR contains

- **Inventory phase** (committed earlier in this branch): `rusty-idd scan` +
  `rusty-idd plan` evidence under `AI_MERGE/unify-handoff-prompthub/` (inventories,
  feature matrices, env/secret contracts, merge plans, conflict registers, parity
  plans, 5-slice task breakdowns; full regenerable workspaces distilled, see
  `REGENERATE.md`).
- **Plan + decide phase**: OpenSpec change `unify-handoff-prompthub`
  (proposal, `repo-unification` spec delta, design, tasks) + **ADR-0018**.
- **Engine fix**: the knowledge code-context report/architecture packs now exclude
  the `AI_MERGE/**` evidence surface. The packs were at the 800K-token budget and
  ingested all of AI_MERGE, so any merge-evidence addition broke `knowledge
  refresh`. Per AGENTS.md, AI_MERGE is an evidence surface, not code-context; the
  **code graph (index.json) still indexes all code** — only the bounded AI
  *code*-context bundle is kept code-only.

## Ground truth established (why the prior attempt failed)

- **handoff is the CANONICAL base, NOT a fork of rusty-idd.** It shares **293**
  paths with rusty-idd (`crates/core,spec,runner,cli,tui`) — residue of a prior
  poor merge that pulled handoff's structure into rusty-idd. Where shared files
  diverge, handoff is authoritative; rusty-idd's forward additions reconcile onto
  it, upgrade-only. handoff uniquely adds `hf`/`ledger`/`work-order`/`.handoff`.
- **prompt_hub is independent** — only **18** shared meta paths; absorbed
  additively.
- Method: parity-tested vertical slices (no big-bang flatten), faithful adopt
  (`.kb` + witnessed ledgers preserved, nothing stripped), absorbed code is
  first-class in the code graph.

## Lessons applied (mistakes paid for, then corrected)

1. Don't flatten before inventory (AGENTS.md Rule 1) — inventory done first now.
2. Adopt faithfully — no stripping binaries / `.kb` / ledgers.
3. Code graph spans absorbed code — do NOT exclude it (the earlier exclusion edit
   was reverted).
4. handoff is canonical — not rusty-idd (owner correction).
5. Distil regenerable tool scaffolding instead of committing build-breaking bulk.

## Verification evidence

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no issues.
- `cargo test --workspace --locked` — 686 passed, 3 ignored.
- `rusty-idd spec validate --all` — 143/143.
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- `render --all --check`, `spec adr list --check` (4 frozen dups), `deploy --target
  . --all --check` — green.
- knowledge + manifest refreshed, self-stable (3594), 0 `.worktrees` contamination.

## Not in this PR (subsequent narrow slices)

Importing the source under `imports/`, reconciling the 293 handoff-base shared
subsystems, absorbing handoff-unique + prompt_hub crates, and parity tests — each
its own PR under change `unify-handoff-prompthub`.
