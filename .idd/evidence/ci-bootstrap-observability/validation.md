# ci-bootstrap-observability Validation Evidence

- Change: `improve-ci-bootstrap-observability`
- Branch: `feature/ci-bootstrap-observability`

## Generated Artifacts

- `cargo run --bin rusty-idd -- knowledge refresh --workspace .`: refreshed
  `.idd/knowledge/index.json`, `report.md`, `architecture.json`, and
  `architecture.md`.
- `cargo run --bin rusty-idd -- knowledge plan-context --workspace . --out
  .idd/knowledge/plan-context.md --goal-file
  .idd/goals/improve-ci-bootstrap-observability.md`: passed.
- `cargo run --bin rusty-idd -- knowledge plan-context --workspace . --out
  .idd/knowledge/plan-context.json --goal-file
  .idd/goals/improve-ci-bootstrap-observability.md`: passed.

## Toolchain Evidence

- `source scripts/ci/envctl-rust-env.sh release`: passed with local
  meta/envctl-owned toolchains layout.
  - `META_ROOT=/home/drdave/Desktop/meta`
  - `RUSTUP_HOME=/home/drdave/Desktop/meta/.toolchains/rustup`
  - `CARGO_HOME=/home/drdave/Desktop/meta/.toolchains/cargo`
  - `rustc=/home/drdave/Desktop/meta/.toolchains/rustup/toolchains/1.96.0-x86_64-unknown-linux-gnu/bin/rustc`
  - `cargo=/home/drdave/Desktop/meta/.toolchains/rustup/toolchains/1.96.0-x86_64-unknown-linux-gnu/bin/cargo`
- `source scripts/ci/envctl-rust-env.sh ci`: passed with strict nightly
  activation and meta-owned tools.
  - `rustc=/home/drdave/Desktop/meta/.toolchains/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc`
  - `cargo=/home/drdave/Desktop/meta/.toolchains/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo`
  - `RUSTC_WRAPPER=/home/drdave/Desktop/meta/.toolchains/cargo/bin/kache`
  - `RUSTY_IDD_CACHE_ROOT=/home/drdave/Desktop/meta/.cache/rust/kache`
  - `RUSTY_IDD_LINKER_PATH=/home/drdave/Desktop/meta/.toolchains/cargo/bin/wild`
  - `RUSTFLAGS=-C linker=clang -C
    link-arg=-fuse-ld=/home/drdave/Desktop/meta/.toolchains/cargo/bin/wild`
- Cold strict bootstrap timing evidence:
  - `rustup toolchain install nightly`: 6s.
  - `rustup component add rustc-codegen-gcc`: 1s.
  - `cargo install kache`: 142s.
  - `cargo install wild-linker`: 26s.
  - `cargo-audit`: reused from
    `/home/drdave/Desktop/meta/.toolchains/cargo/bin/cargo-audit`.
- `scripts/ci/envctl-rust-audit.sh`: passed with verdict
  `meta/envctl-owned Rust toolchain contract satisfied`; `sccache fallback
  version: n/a`.

## Model Loop Evidence

- `cargo run --bin rusty-idd -- codex model-loop --only explore --only
  gap-hunt`: passed and emitted `gpt-5.5-mini` for both read-heavy passes.
- `cargo run --bin rusty-idd -- codex model-loop --only verify`: passed and
  emitted `gpt-5.5` for the final verifier pass.
- `cargo run --bin rusty-idd -- codex model-loop --only explore --execute`:
  command generation succeeded with `gpt-5.5-mini`, but live Codex execution
  failed because this ChatGPT-backed Codex account reported:
  `The 'gpt-5.5-mini' model is not supported when using Codex with a ChatGPT
  account.` The repo config was not downgraded.

## OpenSpec

- `cargo run --bin rusty-idd -- spec status
  openspec/changes/improve-ci-bootstrap-observability`: passed; proposal,
  specs, design, ADR, and tasks all present; archivable yes.

## Build

- `cargo check --workspace --locked`: passed under strict meta-owned nightly +
  `kache` + `wild` activation.

## Test

- `cargo test -p rusty-idd-cli
  codex_model_loop_dry_run_emits_codex_exec_commands -- --nocapture`: passed
  under strict meta-owned nightly + `kache` + `wild` activation.

## Lint

- `bash -n scripts/ci/envctl-rust-env.sh`: passed.
- `cargo fmt --all -- --check`: passed.
- `cargo clippy -p rusty-idd-cli --all-targets -- -D warnings`: passed under
  strict meta-owned nightly + `kache` + `wild` activation.
- `git diff --check`: passed.

## Secret Scan

- `git diff -- . ':(exclude).idd/knowledge/index.json' | rg -n
  "(AKIA[0-9A-Z]{16}|-----BEGIN [A-Z ]*PRIVATE KEY-----|ghp_[A-Za-z0-9_]{20,}|sk-[A-Za-z0-9]{20,}|xox[baprs]-|(?i)(password|secret|token)\s*[:=])"`:
  no matches.

## Manifest

- `cargo run --bin rusty-idd -- manifest --workspace . --out
  .idd/MANIFEST.tsv`: passed and wrote 3503 manifest entries.

## Rollback

Revert the workflow cache split, bootstrap timing logs, model-loop version
update, docs, OpenSpec, ADR, evidence, knowledge, and manifest changes from this
branch.
