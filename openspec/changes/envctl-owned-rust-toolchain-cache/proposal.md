# envctl-owned-rust-toolchain-cache

## Why

Rusty IDD builds currently resolve Rust through the user's rustup and Cargo
homes unless the caller happens to activate a different shell. That repeats a
known failure mode: separating compiler cache and session state from the
meta-owned environment makes builds slow, non-reproducible, and difficult for
agents to audit.

The repository already states that required tooling belongs in the parent
`meta` / `envctl` contract. This change makes the Rust toolchain/cache contract
explicit and machine-checkable.

## What Changes

- Add a Rusty IDD audit contract for meta/envctl-owned Rust toolchains.
- Require the active Rust workflow to resolve the real Cargo-executed `rustc`
  path, Cargo binary path, `RUSTUP_HOME`, `CARGO_HOME`, cache wrapper, linker,
  and cache root.
- Replace the previous mold assumption with a `wild-linker` requirement.
- Prefer `kache` as the compiler cache wrapper, allow `hurry` or `zccache`, and
  keep the checked-in CI implementation off the old `sccache` path.
- Document the parent `meta` / `envctl` ownership boundary and rollback path.

## Capabilities

### New Capabilities

- `meta-owned-rust-toolchain`: Rusty IDD can audit that Rust builds use a
  parent-managed nightly toolchain, Cargo home, cache wrapper, and linker rather
  than user-global or system-owned paths.

### Modified Capabilities

- none

## Impact

- Affected code: `rusty-idd codex system-audit`, Codex environment docs, tests.
- Affected specs: `meta-owned-rust-toolchain`.
- Affected decisions: new ADR for Rust toolchain/cache ownership.
- Parent implementation boundary: actual binary provisioning remains in
  `meta` / `envctl`; this repository records and enforces the contract.
