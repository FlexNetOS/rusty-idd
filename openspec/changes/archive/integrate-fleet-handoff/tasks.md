# integrate-fleet-handoff - Tasks

## 1. Adopt-First Evidence

- [x] 1.1 Re-read owner repo docs, scripts, CI, package metadata, generated architecture artifacts, and relevant ADR/AI_MERGE notes.
- [x] 1.2 Pin or verify every upstream/current owner surface used by this slice.
- [x] 1.3 Record native build/test/diagnostic command candidates before cutting peer-repo behavior.

## 2. Implementation

- [x] 2.1 Add the thinnest Rusty IDD boundary for this capability.
- [x] 2.2 Preserve deterministic output, validation, size/token policy, and feature flags.
- [x] 2.3 Keep `crates/core` std-only.
- [x] 2.4 Record every consolidation cut with evidence and rollback.

## 3. Validation

- [x] 3.1 `cargo fmt --all -- --check`
- [x] 3.2 `cargo test --workspace --all-features --locked`
- [x] 3.3 `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- [x] 3.4 `cargo audit --deny warnings`
- [x] 3.5 `cargo run --bin rusty-idd -- validate --workspace .`
- [x] 3.6 `cargo run --bin rusty-idd -- spec validate --all`
- [x] 3.7 `just ci`
- [x] 3.8 `make ci`
- [x] 3.9 `affected CLI smoke tests`
- [x] 3.10 Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
- [x] 3.11 Record evidence in `/AI_MERGE`.

## Rollback

- Revert the OpenSpec change and generated artifacts for this integration slice
- Re-run rusty-idd knowledge refresh, system-architecture, operating-model, plan-context, and manifest
- Re-run focused owner-repo tests plus Rusty IDD gates
