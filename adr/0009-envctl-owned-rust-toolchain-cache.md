# ADR 0009: Envctl-Owned Rust Toolchain And Build Cache

- **Status:** Accepted
- **Date:** 2026-06-22
- **Change:** `envctl-owned-rust-toolchain-cache`

## Context

Rusty IDD builds were able to resolve the selected Rust channel through
user-global rustup and Cargo homes. That can make a version check look correct
while the actual compiler executable, Cargo cache, compiler wrapper, linker, and
session state remain outside the parent `meta` environment.

The owner clarified that this is the same class of mistake as separating cache
and session state from the meta-owned environment. Rust tooling must be
provisioned and activated by parent `meta` / `envctl`, not installed or cached
ad hoc in user-global or system paths by this repository.

## Decision

Rusty IDD will treat the Rust toolchain and build cache as parent-managed
runtime state owned by `meta` / `envctl`.

The compliant Rust build surface is:

1. nightly Rust toolchain under a meta/envctl-owned `RUSTUP_HOME`;
2. meta/envctl-owned `CARGO_HOME`;
3. `rustc_codegen_gcc` available as the nightly runtime backend surface;
4. parent-managed `wild-linker` for Linux fast linking, replacing mold;
5. parent-managed compiler cache wrapper, preferring `kache`, allowing `hurry`
   or `zccache`, and allowing `sccache` only as a last-resort fallback at
   version `0.15.0` or newer with UDS rather than TCP loopback daemon transport.

Rusty IDD will expose an audit that reports the exact Cargo-executed `rustc`
path, Cargo path, wrapper, linker, home directories, cache root, and selected
tool policy. Strict audit mode fails if those paths resolve to user-global or
system-owned locations.

## Consequences

- Future answers about Rust compiler state must surface executable paths and
  wrappers, not just channel labels.
- Missing Rust tools are routed to parent `meta` / `envctl`; agents must not
  repair this by running global installs from Rusty IDD.
- Build acceleration policy moves from mold/sccache assumptions to
  `wild-linker` plus modern cache wrappers.
- The audit can be implemented and tested in Rusty IDD before parent envctl
  provisioning exists.

## Rollback

Revert the audit contract and this ADR. No committed toolchain or cache content
is removed because the actual runtime state is parent-managed and untracked.
