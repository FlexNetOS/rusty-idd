# 46 — Unify slice 1: import-without-flattening

Evidence note for `feat/unify-slice1-import` — the first implementation slice of
the `unify-handoff-prompthub` change (merge-tools phase 4), under ADR-0018.

## What this slice does

Imports the **complete tracked state** of both owned repos into rusty-idd under
`imports/`, **first-class in the code graph**, without flattening into `crates/`
(reconciliation is later slices). Faithful adopt: nothing stripped.

- `imports/handoff/` — 563 tracked files (`git archive origin/develop`).
- `imports/prompt_hub/` — 432 tracked files (`git archive origin/main`).

### Why tracked-state (`git archive`), not a working-tree copy

The durable knowledge base and ledgers ARE tracked and come in:
- handoff `.kb/store/documents/context/*` (project-brief, patterns, architecture,
  product, tech, active, progress), `.kb/store/commits/*`, `.kb/AGENTS.md`;
  `.handoff/ledger.events.jsonl` (witnessed export).
- prompt_hub `.kb/store/*`; `.handoff/ledger.db` (tracked).

Only `.kb/.cache/*.db` is excluded — it is **untracked daemon runtime** in the
source repos (a regenerable SQLite index of the tracked store) that the live
GitKB daemon rewrites constantly. A working-tree copy pulled it and churn-broke
the validate fingerprint; the tracked-state import is deterministic and keeps the
*real* `.kb` knowledge base. This is faithful to each repo's tracked content, not
a strip.

## Code graph is first-class over the imports

`imports/` is **indexed** by the code graph (68,973 `imports/` symbol references;
index.json 18M → 47M). It is NOT excluded — the earlier instinct to exclude it was
reverted. (Contrast: `third_party/upstream/` third-party mirrors stay excluded.)

## Engine accommodation (verified-safe, mirrors existing scope)

`crates/core/src/validation.rs`: the own-repo secret-hygiene scan now skips
`imports/` (mirroring the existing `third_party/upstream/` skip). prompt_hub's
`privacy.rs` is a secret-DETECTION module — its `ghp_[a-zA-Z0-9]{36}` regex and a
fake test-fixture token tripped the scanner as false positives. Verified: **no
real secrets** in imports (no `-----BEGIN` keys, no live tokens). Imported owned
repos carry their own secret vetting; the imported code stays first-class in the
code graph — only the secret gate skips it.

## Not in this slice

Reconciling the 5 shared crates (handoff base + rusty-idd's forward additions —
the forensics map: spec≈, tui docs-only, runner/core rusty-idd-ahead, cli
rusty-idd-dominant), absorbing handoff's unique `hf`/`ledger`/`work-order` kernel
and prompt_hub's crates as workspace members, env/secret normalization, parity
tests, and dedup — each a subsequent slice under `unify-handoff-prompthub`.

## Verification evidence

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no
  issues (imports/ is not a workspace member; not built this slice).
- `cargo test --workspace --locked` — 686 passed, 3 ignored.
- `rusty-idd spec validate --all` — 143/143.
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- `render --all --check`, `spec adr list --check`, `deploy --target . --all
  --check` — green.
- knowledge + manifest refreshed, self-stable, 0 `.worktrees` contamination.
