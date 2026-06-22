# envctl-owned-rust-toolchain-cache - Design

## Context

The current checkout can resolve `rustc` through rustup shims and user-global
homes even when the selected channel is correct. That is insufficient for agent
work: compile time, cache reuse, and reproducibility depend on the actual
compiler executable, wrapper, linker, and cache roots.

Official Rust tooling supports the desired boundary: rustup's `RUSTUP_HOME`
controls installed toolchains and Cargo's `CARGO_HOME` controls Cargo cache
state. Cargo exposes `RUSTC`, `RUSTC_WRAPPER`, and
`RUSTC_WORKSPACE_WRAPPER` / config equivalents for the actual compiler and
wrapper path. The missing piece is a Rusty IDD contract that rejects
non-meta-owned paths and reports the exact active surface.

## Goals / Non-Goals

**Goals:**

- Make Rusty IDD able to audit a meta/envctl-owned nightly Rust stack.
- Report the exact compiler path, Cargo path, wrapper, linker, and cache root.
- Prefer `kache`, allow `hurry` or `zccache`, and keep the checked-in CI path
  off `sccache`.
- Replace mold with parent-managed `wild-linker`.
- Keep actual installation and binary provisioning in parent `meta` / `envctl`.

**Non-Goals:**

- Do not install Rust tooling into user-global or system-owned paths from this
  repository.
- Do not start, stop, or manage cache daemons from this repository.
- Do not commit toolchain or cache contents.
- Do not make `rustc_codegen_gcc` the default backend for every build in this
  slice; record and audit the runtime backend surface first.

## Decisions

1. Add a Rust toolchain section to `rusty-idd codex system-audit`, gated by an
   explicit flag so existing Codex binary audits remain usable.
2. Require a `--meta-root` for strict Rust toolchain audits. The audit treats
   paths under that root as owned and rejects `~/.rustup`, `~/.cargo`, `/usr`,
   `/opt`, and other system-owned paths.
3. Model expected tools as policy: `nightly`, `rustc_codegen_gcc`,
   `wild-linker`, and compiler cache wrapper preference order
   `kache > hurry|zccache` for the checked-in CI implementation.
4. Keep cache daemon lifecycle outside Rusty IDD. The audit can validate socket
   path policy but must not create or kill daemons.
5. Document parent `meta` / `envctl` as the only provisioning location.

## Risks / Trade-offs

- Nightly plus alternative codegen is less stable than stable LLVM Rust. The
  mitigation is audit-first adoption: record the runtime surface and keep the
  default build path reversible while parent provisioning matures.
- `kache`, `hurry`, and `zccache` are the active cache-wrapper targets. The
  contract accepts multiple modern wrappers but demands meta-owned paths.
- `rustc_codegen_gcc` may require platform-specific runtime libraries. The
  audit records the requirement but leaves installation to parent envctl.

## Migration Plan

1. Add Rusty IDD OpenSpec, ADR, docs, and tests for the meta-owned Rust
   toolchain/cache contract.
2. Extend `rusty-idd codex system-audit` with strict Rust toolchain checks.
3. Update docs and env-check invariants to mention nightly, `rustc_codegen_gcc`,
   `wild-linker`, and accepted cache wrappers.
4. Parent `meta` / `envctl` later materializes the binaries and exports the
   required environment.
5. Re-run the strict audit after envctl provisioning lands.

## Rollback

Revert this change. Since it does not install toolchains or mutate host services,
rollback is limited to removing the audit contract, docs, and generated
artifacts.

## Open Questions

- Whether parent `envctl` should select `kache`, `hurry`, or `zccache` first in
  practice after benchmark evidence. This change records the allowed order and
  keeps `kache` as the preferred default.
