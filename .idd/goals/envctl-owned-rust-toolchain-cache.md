# envctl-owned Rust toolchain and build cache

Rusty IDD Rust builds must not depend on user-global or system-depth Rust
state. The active Rust toolchain, Cargo home, compiler cache, and linker must be
owned by the parent `meta` / `envctl` environment and must be auditable from
Rusty IDD.

Target toolchain and build acceleration stack:

- nightly Rust toolchain, installed under a meta/envctl-owned `RUSTUP_HOME`
  rather than `~/.rustup`.
- meta/envctl-owned `CARGO_HOME` rather than `~/.cargo`.
- `rustc_codegen_gcc` available as a runtime backend surface for the nightly
  toolchain.
- `wild-linker` as the meta-owned Linux linker path, replacing the previous
  mold assumption.
- a meta-owned compiler cache path, preferring `kache`, then `hurry` or
  `zccache`. The checked-in CI path must not fall back to `sccache`.

Rusty IDD must expose an audit that reports the actual Cargo-executed compiler
path, wrapper, linker, home directories, and cache root, and rejects user-global
or system-owned paths for this repo workflow.
