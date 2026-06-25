# 0014. CI Bootstrap Cache Observability

- Status: accepted
- Date: 2026-06-23

## Context

PR #103 and PR #104 proved that Rusty IDD CI uses the correct parent
meta/envctl-owned strict Rust surface. They also showed that the full `rust`
job can spend 11-14 minutes in bootstrap and full validation. The existing
cache key couples parent Rust toolchain/tool state to workspace `Cargo.lock`
changes by caching `.env/rust`, `.cache/rust`, and `target` together.

The repo also still emits `gpt-5.4-mini` for a cheap read-heavy model-loop pass
even though the current preferred cheap model target is `gpt-5.5-mini`.

## Decision

Split CI caches into:

1. parent meta/envctl Rust toolchain and tool state, keyed by bootstrap scripts;
2. workspace `target`, keyed by `Cargo.lock` plus bootstrap scripts.

Add elapsed-time bootstrap logs for rustup, codegen component setup, cache
wrapper setup, wild-linker setup, and cargo-audit setup.

Resolve the parent meta root to a canonical absolute path before Actions cache
restore/save so the cache action does not reject parent-owned `.env/rust` and
`.cache/rust` paths that were previously expressed through `../meta`.

Update the `explore` and `gap-hunt` model-loop passes to emit `gpt-5.5-mini`,
while keeping write-capable implementation outside the default read-only loop.

## Consequences

- Parent Rust tool/toolchain cache reuse is less sensitive to workspace
  dependency churn.
- CI logs can identify the precise strict bootstrap span that consumed time.
- The strict contract remains nightly + `rustc_codegen_gcc` + `wild` + `kache`
  under meta/envctl ownership.
- `sccache` remains excluded from the checked-in CI bootstrap.

## Rollback

Revert the workflow cache split, timed logging helper, model-loop model update,
and generated artifacts. No host services, user-global toolchains, or system
paths are mutated by this decision.
