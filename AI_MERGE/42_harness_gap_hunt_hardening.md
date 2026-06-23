# 42 — Harness gap-hunt hardening

Evidence note for `fix/harness-gap-hunt-hardening`. After the control-plane arc
landed (PRs #109–#112), an adversarial gap-hunt (skeptic agents trying to
*refute* each slice against the actual code/spec) found three real, evidence-
backed gaps. This change applies the upgrades; all three are conformance fixes
that bring code in line with existing specs/design — no behavior change beyond
closing the gaps.

## Gaps found and fixed

1. **ADR collision gate was presence-only, not count-bounded**
   (`crates/cli/src/commands/spec_adr.rs`). A *third* file at a frozen baseline
   number (e.g. a new `0002-*.md` beside the two historical ones) was classified
   "accepted baseline" and passed CI silently. Fix: `ACCEPTED_DUPLICATE_ADRS` is
   now `&[(u32, usize)]` pinning each baseline number to its exact count `(2,2),
   (4,2),(5,2),(6,2)`; a count beyond the baseline is tagged `EXCEEDS BASELINE`
   and fails closed. New integration test
   `third_file_at_baseline_number_exceeds_baseline_and_fails`.

2. **Session-hook tests were too weak to catch a `--base` regression**
   (`crates/cli/tests/vendor_hooks.rs`). `invokes_next` matched only the `next`
   substring, so swapping `--base`→`--workspace` (a clap error at every session
   start) or dropping the flag (silent default to `.`) would pass tests. Fix:
   added `next_uses_base` and asserted it for both the `.codex` and `.claude`
   SessionStart commands.

3. **Verify package was missing the `rollback-risk` evidence field**
   (`crates/cli/src/commands/harness.rs`). `design.md` lists 9 `evidence_schema`
   fields; the code had 8 (rollback risk was folded into `pass-fail-verdict`).
   Fix: added the standalone `rollback-risk` entry; strengthened the verify JSON
   test to assert it.

## Verification evidence

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no issues.
- `cargo test --workspace --locked` — 666 passed, 0 failed (+1 ADR test).
- `rusty-idd spec validate --all` — 153/153.
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- knowledge + manifest refreshed, self-stable (3542 entries), 0 contamination.
- `rusty-idd spec adr list --check` — 4 baseline duplicates at pinned count, exit 0.
