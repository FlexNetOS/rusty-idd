# 46 — Unify slice 1: import-without-flattening

Evidence note for `feat/unify-slice1-import` — the first implementation slice of
the `unify-handoff-prompthub` change (merge-tools phase 4), under ADR-0018.

## What this slice does

Imports the **complete working tree** of both owned repos into rusty-idd under
`imports/`, **first-class in the code graph**, without flattening into `crates/`
(reconciliation is later slices). Faithful adopt: byte-for-byte, nothing stripped.

- `imports/handoff/` — full working tree (excludes only `.git/`).
- `imports/prompt_hub/` — full working tree (excludes only `.git/`).

### Why the complete working tree (not `git archive` tracked-state)

The owner directive is explicit: *"keep all the dotfolders and files. you need
them. this is why the current gitignore policy needs an upgrade."* A `git archive`
import keeps only **tracked** files and silently drops the durable artifacts that
are gitignored in the source repos but are nonetheless real, owned meta state:

- handoff `.kb/.cache/*` (GitKB code-intel index), `.kb/workspaces/*`,
  `.kb/config.toml`, `.handoff/ledger.db` + `.handoff/ledger.db.rvf` (witnessed
  binary ledger), `.grit/`, `.idea/workspace.xml`, `_workspace/`, `_workspace_prev/`.
- prompt_hub `.kb/.cache/*`, `prompthub.db`, `.idea/workspace.xml`,
  `.claude/settings.local.json`.

These are imported via `git add -f imports/` so the **nested `.gitignore` files
carried inside each adopted repo** (which ignore `.kb/.cache`, `target/`, db
files, etc. in their *own* repo context) do not strip them here. Empirically the
imported `.kb/.cache` does **not** churn on disk — the live GitKB daemon manages
only rusty-idd's *own* `.kb`, not these adopted subtrees — so the snapshot is a
stable static import and the validate workspace fingerprint stays clean.

### Gitignore-policy upgrade (required to hold the import)

`.gitignore` rewritten so rusty-idd's own local-artifact patterns are **anchored
to the repo root** (`/target`, `/.worktrees/`, `/.idd/runs/`, `/.idea/`,
`/.vscode/`) and therefore never strip an adopted repo's identically-named
dotfolders under `imports/`. A trailing `!imports/**` safety net re-includes
anything the universal-junk patterns would otherwise exclude there. The
`*.idd-bak-*` rotating-backup pattern stays unanchored (runtime litter at any
depth) but is also covered by the `!imports/**` net.

## Code graph is first-class over the imports

`imports/` is **indexed** by the code graph (68,973 `imports/` symbol references;
index.json 18M → 47M). It is NOT excluded — the earlier instinct to exclude it was
reverted. (Contrast: `third_party/upstream/` third-party mirrors stay excluded.)

## Engine accommodation: path-scoped secret allowlist (not a blanket skip)

`crates/core/src/validation.rs`: the own-repo secret-hygiene scan now reads
`.idd/secret-allowlist.txt` (one path substring per line) and exempts only the
**named** placeholder files from secret-pattern findings — it does **not** skip
`imports/` wholesale (the earlier blanket-skip instinct was reverted at owner
direction: *"Allowlist the place holders"*). Two prompt_hub files are listed:
`src/privacy.rs` (a secret-DETECTION module whose own provider token-detection
regexes trip the scanner) and `src/context_gatherer.rs` (detection patterns +
fixtures). Verified: **no real secrets** in imports (no `-----BEGIN` keys, no
live tokens) — every match is a detection regex or fake test value. The rest of
`imports/` stays under the live secret gate, and all of it stays first-class in
the code graph.

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
- `cargo test --workspace --locked` — 687 passed, 3 ignored (adds
  `secret_allowlist_exempts_listed_placeholder_files`).
- `rusty-idd spec validate --all` — 143/143.
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- `render --all --check`, `spec adr list --check`, `deploy --target . --all
  --check` — green.
- knowledge + manifest refreshed, self-stable, 0 `.worktrees` contamination.
