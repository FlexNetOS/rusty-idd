# 48 — Unify slice: absorb handoff `work-order` crate

First Phase-5 reconciliation slice (merge-tools phase 4, task 3.5) under ADR-0018:
migrate a handoff-unique crate from the faithful `imports/` snapshot into
rusty-idd's `crates/` as a real workspace member.

## What this slice does

Migrates handoff's `work-order` crate (the `handoff.task.v1` work-order envelope
+ SwarmBundle→WorkOrder seam) into `crates/work-order` and adds it to the
workspace members. `work-order` is the foundation of the handoff continuity
kernel (hf → ledger → work-order).

- Source: `imports/handoff/work-order/` (faithful snapshot, left intact —
  deprecate-before-remove; dedup happens when the whole handoff tree migrates).
- Target: `crates/work-order/` (new workspace member).
- Manifest edits: `edition` and `license` set **explicitly** (`2021`,
  `Apache-2.0 OR MIT`) instead of `*.workspace = true`, so the migration into
  rusty-idd's MIT/edition-2021 workspace preserves handoff's original
  more-permissive license — faithful adopt, no silent relicense.

## Why work-order first (dependency ground truth)

Mapped the handoff-unique crates' dependency graphs:

| crate | clean to absorb? | blocker |
|-------|------------------|---------|
| `work-order` | **yes** | only serde / serde_json / blake3 / schemars — fully self-contained |
| `ledger` | no | default features `v2` → redb-store → RuVector `rvf-crypto`/`rvf-runtime`/`rvf-index`/`rvf-types` (cross-repo `../../RuVector/...` path deps) |
| `hf` | no | requires `ledger` + **non-optional** `ruvector-verified` + `ruvector-domain-expansion` (RuVector) + optional `envctl-secrets-engine` |

So `work-order` is the only handoff-unique crate absorbable without first deciding
a **RuVector strategy** (vendor the `rvf-*` crates into rusty-idd vs. keep them as
meta-workspace path deps vs. publish + version-depend). That decision gates the
ledger/hf kernel slices and is recorded as the next blocker.

The shared crates (`rusty-idd-core/-spec/-runner/-cli/-tui`) carry **identical
package names** in handoff (prior-poor-merge residue), so they need per-crate
reconciliation (slice 3.4), not absorption.

## Parity

`work-order` was copied verbatim; its **22 unit tests pass unchanged** in the
rusty-idd workspace — that IS the parity evidence (same tests, same behavior).

## Verification

- `cargo build --workspace` green; `cargo test --workspace --locked` 710 passed
  (+22 from work-order), 3 ignored.
- `cargo fmt --all --check` clean; `cargo clippy --workspace --all-targets
  --all-features -D warnings` no issues.
- `rusty-idd merge-tools verify` passed (60 crate manifests, 54 src trees).
- `validate --workspace .` 0/0; `manifest` STABLE; 0 `.worktrees` contamination.

## Not in this slice

`ledger` + `hf` (blocked on the RuVector strategy decision), the 5 shared-crate
reconciliations (3.4), prompt_hub crates (3.6, MSRV 1.91 / `--all-features`), and
removing the `imports/handoff/work-order` snapshot (deprecate-before-remove).
