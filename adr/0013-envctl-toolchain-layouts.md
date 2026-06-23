# ADR 0013: Envctl Rust Toolchain Layouts

- **Status:** Accepted
- **Date:** 2026-06-23
- **Change:** `align-envctl-toolchain-contract`
- **Builds On:** `adr/0009-envctl-owned-rust-toolchain-cache.md`
- **Builds On:** `adr/0012-self-upgrade-governor.md`

## Context

The self-upgrade governor generated a candidate goal from a real audit gap. CI
proved Rusty IDD's strict envctl Rust contract under `$META_ROOT/.env/rust`, but
the workstation envctl contract uses `$META_ROOT/.toolchains`.

On this workstation, `~/.cargo/bin/cargo`, `rustup`, and `rustc` resolve through
symlinks to `$META_ROOT/.toolchains/cargo/bin/rustup`. The active compiler and
Cargo paths resolve under `$META_ROOT/.toolchains/rustup`. `envctl env
--toolchains` emits `CARGO_HOME=$META_ROOT/.toolchains/cargo`.

## Decision

Rusty IDD recognizes two meta-owned Rust layouts:

- `isolated`: CI/bootstrap layout under `$META_ROOT/.env/rust`.
- `toolchains`: workstation envctl layout under
  `$META_ROOT/.toolchains/{cargo,rustup}`.

GitHub Actions defaults to `isolated`. Local activation defaults to
`toolchains` when the envctl-managed rustup binary and rustup home exist.
Operators may override with `RUSTY_IDD_RUST_LAYOUT`.

Rusty IDD audits must continue to report actual Cargo-executed `rustc` and
`cargo` paths plus wrapper, cache, linker, and backend details.

## Consequences

- Local validation no longer misclassifies meta-owned symlinks as user-global
  Rust installs.
- CI retains strict isolated nightly + kache + wild + rustc_codegen_gcc
  behavior.
- The selected layout becomes visible evidence in activation output.
- Real user-global or system-depth Rust homes remain invalid.

## Rollback

Remove the layout selector and return to the previous `.env/rust`-only helper.
CI would remain valid, but local envctl workstation validation would again need
manual environment setup or would risk misleading path reports.
