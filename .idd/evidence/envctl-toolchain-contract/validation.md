# envctl-toolchain-contract Validation Evidence

- Change: `align-envctl-toolchain-contract`
- Branch: `feature/envctl-toolchain-contract`

## Generated Artifacts

- Passed: `cargo run --bin rusty-idd -- knowledge refresh --workspace .`
  refreshed `.idd/knowledge/index.json`, `.idd/knowledge/report.md`,
  `.idd/knowledge/architecture.json`, and `.idd/knowledge/architecture.md`.
- Passed: `cargo run --bin rusty-idd -- knowledge plan-context --workspace .
  --out .idd/knowledge/plan-context.md --goal-file
  .idd/goals/align-envctl-toolchain-contract.md`.
- Passed: `cargo run --bin rusty-idd -- knowledge plan-context --workspace .
  --out .idd/knowledge/plan-context.json --goal-file
  .idd/goals/align-envctl-toolchain-contract.md`.
- Passed: `cargo run --bin rusty-idd -- manifest --workspace . --out
  .idd/MANIFEST.tsv`; wrote 3494 manifest entries.

## Toolchain Evidence

- Passed: `source scripts/ci/envctl-rust-env.sh release` selected the local
  envctl `toolchains` layout.
- `META_ROOT=/home/drdave/Desktop/meta`.
- `RUSTUP_HOME=/home/drdave/Desktop/meta/.toolchains/rustup`.
- `CARGO_HOME=/home/drdave/Desktop/meta/.toolchains/cargo`.
- `RUSTUP_TOOLCHAIN=1.96.0`.
- Actual Cargo-executed `rustc`:
  `/home/drdave/Desktop/meta/.toolchains/rustup/toolchains/1.96.0-x86_64-unknown-linux-gnu/bin/rustc`.
- Actual Cargo-executed `cargo`:
  `/home/drdave/Desktop/meta/.toolchains/rustup/toolchains/1.96.0-x86_64-unknown-linux-gnu/bin/cargo`.
- Local strict cache/linker tools are not currently installed in `.toolchains`
  (`kache` and `wild` were absent), so this branch adds
  `codex_system_audit_accepts_envctl_toolchains_rust_layout` to prove the
  strict `.toolchains` audit shape without installing missing tools.
- No `sccache` fallback was introduced.

## OpenSpec

- Passed: `cargo run --bin rusty-idd -- spec status
  openspec/changes/align-envctl-toolchain-contract`.
- Result: `Archivable: yes (5/5 artifacts done)`.

## Build

- Passed: `cargo check -p rusty-idd-cli --locked`.

## Test

- Passed: `cargo test -p rusty-idd-cli
  codex_system_audit_accepts_envctl_toolchains_rust_layout -- --nocapture`.

## Lint

- Passed: `cargo fmt --all -- --check`.
- Passed: `bash -n scripts/ci/envctl-rust-env.sh`.
- Passed: `cargo clippy -p rusty-idd-cli --all-targets -- -D warnings`.

## Secret Scan

- Passed: scanned the working diff for common private keys and token patterns;
  no matches.

## Manifest

- Passed: `.idd/MANIFEST.tsv` refreshed with the current goal, OpenSpec, ADR,
  evidence, source, docs, and generated knowledge artifacts.

## Rollback

Revert the script, docs, goal, OpenSpec change, ADR, evidence, generated
knowledge, and manifest changes from this branch.
