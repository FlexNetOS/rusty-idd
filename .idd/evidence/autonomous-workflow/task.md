# Envctl Rust Toolchain CI Task Evidence

- Task: `KBTASK-RUSTY-IDD-ENVCTL-RUST-CI`
- Change: `envctl-owned-rust-toolchain-cache`
- Goal file: `.idd/goals/envctl-owned-rust-toolchain-cache.md`
- Branch: `fix/ci-envctl-rust-toolchain`
- Claim: `claim recorded by this Rusty IDD worktree owner for the active CI implementation slice`

This task moves GitHub workflow Rust setup away from Rust action/cache shims and
onto the Rusty IDD envctl-owned toolchain/cache contract. The CI path reports
the actual Cargo-executed compiler, uses meta-local Rust homes, requires the
nightly `rustc_codegen_gcc` surface, uses `wild`, and selects kache/hurry/zccache
without falling back to sccache.
