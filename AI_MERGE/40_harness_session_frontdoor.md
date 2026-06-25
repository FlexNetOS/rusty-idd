# 40 — Harness session front door + ADR ledger reconciliation

Evidence note for the `harness-session-frontdoor` change (backlog items 4.2 and
4.5 of `harness-control-plane`). Closes the control-plane arc: the front door
(`rusty-idd next`, ADR-0015) is now actually *invoked* at session start, and the
ADR ledger's duplicate-number debt is reconciled and guarded against recurrence.

## What landed

1. **Session-start wiring (4.2).** Vendor surfaces now call `rusty-idd next`
   automatically:
   - `.codex/hooks.json` gains a `SessionStart` hook (additive — the four
     existing workflow-check gates are unchanged).
   - new `.claude/settings.json` with a `hooks.SessionStart` entry.
   Both run `cargo run … --bin rusty-idd -- next --base "$root"`.
2. **ADR collision gate (4.5).** `rusty-idd spec adr list --check` fails closed
   on any NEW duplicate ADR number; the four accepted historical collisions
   (0002/0004/0005/0006) are a frozen baseline (`ACCEPTED_DUPLICATE_ADRS`),
   reported but not failing — the `.cargo/audit.toml` baseline philosophy.
   Wired into CI (`.github/workflows/ci.yml`) and the `Justfile` (`adr-check`,
   added to the `ci` recipe alongside the previously-missing `render-check`).
3. **ADR-0016** records the collisions as frozen historical artifacts and
   establishes slug-canonical ADR referencing (no renumbering — ADRs are
   immutable). ADR-0015 had flagged the 0002 collision as debt; this closes it.

## Verification evidence

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no issues.
- `cargo test --workspace --locked` — 662 passed, 0 failed (+9 over the 653
  baseline: 4 `adr_check_cli` + 3 `vendor_hooks` + 2 `spec_adr` unit tests).
- `rusty-idd spec validate --all` — 153/153 passed, 0 failed.
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (after refresh-last).
- `.idd/knowledge/*` refreshed, then `.idd/MANIFEST.tsv` (3539 entries);
  re-validate stays 0/0, manifest is self-stable, 0 `.worktrees`/`.idd-bak`
  contamination.
- `rusty-idd render --all --check` — 4 adapters in sync.
- `rusty-idd spec adr list --check` — 4 baseline duplicates, exit 0.
- `rusty-idd next` — change reports archivable (5/5 artifacts).

## Flow

Followed the full Rusty IDD flow from step 1: goal
(`.idd/goals/harness-session-frontdoor.md`) → `knowledge plan-context` binding →
OpenSpec change (proposal + spec delta + design + tasks) → ADR-0016 →
implementation after the artifact DAG was ready → validation refresh.

## Not in scope

- Backlog 4.3 (additional stage packages: impl/validation/handoff swarms) — that
  is the separate `add-verify-package-stage` change.
- `.agents`/`.devin` have no standard session-start hook mechanism; they keep
  their thin adapter only.
