# improve-ci-bootstrap-observability

## Goal

Continue the Rusty IDD self-upgrade loop from the envctl toolchain contract
evidence. PR #103 and PR #104 proved the strict CI Rust path is correct, but
the full `rust` job spent 11-14 minutes in strict bootstrap and full gates.

Improve the CI bootstrap contract without weakening the required toolchain:
nightly Rust, `rustc_codegen_gcc`, parent-managed `wild-linker`, and
meta/envctl-owned `kache` remain required. Do not introduce `sccache`.

## Required Change

- Split GitHub Actions caching so meta-owned Rust toolchain/tool state is cached
  independently from workspace `target` artifacts.
- Keep all Rust mutable state under the parent meta/envctl root.
- Add grouped/timed bootstrap logging around rustup, codegen component, cache
  wrapper, wild-linker, and cargo-audit installation checks.
- Update the Rusty IDD model-loop cheap read-heavy passes from the stale cheap
  model surface to `gpt-5.5-mini`, then run the model-loop surface as workflow
  evidence.

## Evidence

- PR #103 CI full `rust` job passed in 11m10s.
- PR #104 CI full `rust` job passed in 14m9s.
- PR #104 strict CI environment reported:
  - `layout: isolated`
  - `RUSTUP_HOME=/home/runner/work/rusty-idd/meta/.env/rust/rustup`
  - `CARGO_HOME=/home/runner/work/rusty-idd/meta/.env/rust/cargo`
  - `rustc=/home/runner/work/rusty-idd/meta/.env/rust/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc`
  - `cargo=/home/runner/work/rusty-idd/meta/.env/rust/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo`
  - `RUSTC_WRAPPER=/home/runner/work/rusty-idd/meta/.env/rust/bin/kache`
  - `RUSTFLAGS=-C linker=clang -C link-arg=-fuse-ld=/home/runner/work/rusty-idd/meta/.env/rust/bin/wild`

## Success Criteria

- CI, promotion, and release workflows have separate envctl Rust tool/cache
  cache keys that do not depend on `Cargo.lock`.
- Workspace `target` cache remains keyed by `Cargo.lock`.
- GitHub Actions cache paths use canonical parent meta paths rather than
  rejected `../meta` patterns.
- `scripts/ci/envctl-rust-env.sh` reports timed install/skip spans for strict
  bootstrap stages.
- The repo-local model loop emits `gpt-5.5-mini` for the cheap read-heavy
  `explore` and `gap-hunt` passes.
- OpenSpec, ADR, evidence, generated knowledge, and manifest artifacts are
  refreshed.
