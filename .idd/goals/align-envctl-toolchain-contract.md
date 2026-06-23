# align-envctl-toolchain-contract

## Step 1: Run This Goal Through Rusty IDD

```bash
rusty-idd --goal-file .idd/goals/align-envctl-toolchain-contract.md
```

If the active CLI surface requires a subcommand for goal binding, use:

```bash
rusty-idd knowledge plan-context \
  --workspace . \
  --out .idd/knowledge/plan-context.md \
  --goal-file .idd/goals/align-envctl-toolchain-contract.md
```

## Goal

Align Rusty IDD local Rust validation with the parent `meta` / `envctl`
toolchain ownership contract.

## Evidence

The self-upgrade loop discovered a contract mismatch:

- CI successfully uses `scripts/ci/envctl-rust-env.sh` with isolated
  `$META_ROOT/.env/rust/{rustup,cargo,bin}` plus kache, wild, and
  `rustc_codegen_gcc`.
- The local workstation's envctl-owned Rust toolchain lives under
  `$META_ROOT/.toolchains/{cargo,rustup}`.
- `~/.cargo/bin/{cargo,rustup,rustc}` are not true user-global installs here;
  they are symlinks into `$META_ROOT/.toolchains/cargo/bin/rustup`.
- `envctl env --toolchains` emits
  `CARGO_HOME=$META_ROOT/.toolchains/cargo`.
- The previous artifact pass incorrectly described local `cargo/rustup` as
  unavailable rather than diagnosing the `.toolchains` versus `.env/rust`
  contract split.

## Required Change

Rusty IDD must support both envctl-owned layouts:

- `isolated`: CI/bootstrap layout under `$META_ROOT/.env/rust`.
- `toolchains`: workstation envctl layout under
  `$META_ROOT/.toolchains/{cargo,rustup}`.

GitHub Actions must keep the strict isolated layout by default. Local validation
must prefer the envctl-managed `.toolchains` layout when it exists, without
using real user-global or system-depth Rust state.

## Success Criteria

- `scripts/ci/envctl-rust-env.sh` reports the selected layout.
- GitHub Actions remains on `.env/rust` unless explicitly overridden.
- Local workstation validation can run with `.toolchains` and reports actual
  `rustc` and `cargo` paths under the meta root.
- Documentation explains both layouts and the envctl ownership boundary.
- OpenSpec, ADR, evidence, generated knowledge, and manifest artifacts are
  updated.
