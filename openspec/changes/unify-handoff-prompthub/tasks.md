# unify-handoff-prompthub — Tasks

## 1. Inventory (merge-tools phase 1) — DONE

- [x] 1.1 `rusty-idd scan` RepoInventory for handoff (592) + prompt_hub (420)
- [x] 1.2 `rusty-idd plan` per unification: feature matrix, env/secret contracts, merge plan, conflict register, parity plan, 5-slice tasks
- [x] 1.3 Distilled evidence committed under `AI_MERGE/unify-handoff-prompthub/` (+ REGENERATE.md)
- [x] 1.4 Ground truth recorded: handoff canonical (293 shared, prior poor merge); prompt_hub additive (18 shared)

## 2. Plan + Decide (merge-tools phases 2–3) — THIS CHANGE

- [x] 2.1 Goal bound via `knowledge plan-context`
- [x] 2.2 OpenSpec change: proposal, `repo-unification` spec delta, design, tasks
- [x] 2.3 ADR-0018 (unification architecture: handoff canonical, code-graph-first, faithful adopt, parity slices)
- [x] 2.4 Engine fix: exclude `AI_MERGE/**` evidence surface from the knowledge code-context pack (refresh stays within budget; code graph unchanged)
- [ ] 2.5 Validate + manifest + checkpoint with owner before any implementation slice

## 3. Implement — vertical slices (merge-tools phase 4, SUBSEQUENT PRs)

Each slice is its own narrow PR with parity tests; behavior preserved until parity.

- [x] 3.1 Slice: import-without-flattening — `imports/handoff/` + `imports/prompt_hub/` **complete working trees** via `git add -f` (faithful byte-for-byte; ALL dotfolders/files kept incl. `.kb/.cache`, `.kb/workspaces`, `.handoff/ledger.db`+`.rvf`, `.grit/`, `prompthub.db`, `.idea`), code-graph-indexed first-class. Gitignore-policy upgrade: root-anchored own-artifact patterns + `!imports/**` net. Secret scan uses a path-scoped allowlist (`.idd/secret-allowlist.txt`, placeholders only — NOT a blanket skip); verified no real secrets. [AI_MERGE/46]
- [ ] 3.2 Slice: normalize env/secret contracts (one SecretProvider + env order)
- [ ] 3.3 Slice: canonical interfaces for the 293 handoff-base shared paths + prompt_hub crate seams
- [ ] 3.4 Slices: reconcile shared subsystems one at a time (core → spec → runner → cli → tui), handoff base + rusty-idd forward additions, parity-tested
- [ ] 3.5 Slices: absorb handoff-unique crates (`hf`, `ledger`, `work-order`, `.handoff` kernel)
- [ ] 3.6 Slices: absorb prompt_hub crates additively (resolve MSRV / `--all-features` at membership)

## 4. Verify + Evidence (merge-tools phases 5–6, per slice)

- [ ] 4.1 Per slice: `cargo build/test/fmt/clippy --workspace`; `rusty-idd validate --workspace .`
- [ ] 4.2 Per slice: parity tests green before dedup; deprecate-before-remove
- [ ] 4.3 Refresh `.idd/knowledge/*` + `MANIFEST.tsv`; code graph spans absorbed code; 0 contamination
- [ ] 4.4 Per slice: AI_MERGE migration note (old path → new path) + rollback path

## 5. This change's verification gates

- [x] 5.1 `cargo test --workspace --locked` 686 passed; `fmt --check` clean; `clippy --workspace --all-targets --all-features -D warnings` no issues (engine pack fix)
- [x] 5.2 `spec validate --all` 143/143; `validate --workspace .` 0 critical / 0 warning
- [x] 5.3 `render --all --check` + `spec adr list --check` + `deploy --target . --all --check` green
- [x] 5.4 refresh `.idd/knowledge/*` + `MANIFEST.tsv` (refresh-last → validate → manifest); 0 contamination
- [x] 5.5 AI_MERGE evidence note (`AI_MERGE/45_unify_handoff_prompthub_plan.md`)
