# improve-ci-bootstrap-observability

## Why

The envctl toolchain contract is now correct, but PR #103 and PR #104 exposed a
follow-on bottleneck: the full CI `rust` job passed yet spent 11-14 minutes in
strict bootstrap and gates. The bootstrap step installs or verifies nightly
Rust, `rustc_codegen_gcc`, `kache`, `wild-linker`, and `cargo-audit`, but the
current cache key combines parent meta Rust state and workspace `target` state
under a `Cargo.lock`-dependent key.

That makes parent-managed toolchain/tool cache reuse more fragile than it needs
to be. Agents also cannot quickly see which strict bootstrap sub-step consumed
time because the script only prints the final environment summary.

## What Changes

- Split GitHub Actions cache entries for parent meta Rust state and workspace
  `target` artifacts.
- Keep envctl Rust tool/cache cache keys independent from `Cargo.lock`.
- Keep workspace `target` caches keyed by `Cargo.lock`.
- Resolve the parent meta Rust root before cache restore so Actions cache paths
  are canonical and do not include `..`.
- Add grouped/timed bootstrap logging for strict Rust setup stages.
- Update the cheap read-heavy model-loop passes to `gpt-5.5-mini`.

## Capabilities

### Modified

- `meta-owned-rust-toolchain`: CI bootstrap must preserve meta/envctl ownership
  while making tool/cache reuse and elapsed bootstrap evidence explicit.
- `codex-harness-flow`: model-loop cheap read-heavy passes use `gpt-5.5-mini`
  when available.

## Impact

- Affected files:
  - `.github/workflows/ci.yml`
  - `.github/workflows/promote-verify.yml`
  - `.github/workflows/release.yml`
  - `.codex/loops/rusty-idd-model-loop.toml`
  - `scripts/ci/envctl-rust-env.sh`
  - `crates/core/src/templates.rs`
  - `crates/cli/tests/codex_cli.rs`
  - `docs/rusty-idd/codex-environment.md`
  - OpenSpec, ADR, evidence, generated knowledge, and manifest artifacts.
- No user-global Rust state is introduced.
- No `sccache` fallback is introduced.
