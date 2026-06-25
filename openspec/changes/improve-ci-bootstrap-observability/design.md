# improve-ci-bootstrap-observability - Design

## Context

PR #103 and #104 proved the strict CI Rust surface:

- isolated meta-owned layout under `/home/runner/work/rusty-idd/meta/.env/rust`;
- nightly `rustc` and `cargo` under that meta root;
- `RUSTC_WRAPPER` set to meta-owned `kache`;
- `RUSTFLAGS` using parent-managed `wild`;
- strict audit passing with `rustc_codegen_gcc`.

The slow path is not an incorrect compiler. The slow path is bootstrap and full
gate cost. The current workflow restores `.env/rust`, `.cache/rust`, and
`target` together under a key that includes `Cargo.lock`. Toolchain and tool
state should survive workspace lockfile churn because `kache`, `wild`, and
`cargo-audit` provisioning depends on the bootstrap scripts and selected tool
versions, not the application dependency graph.

The previous parent meta cache path also used a raw `../meta` expression. GitHub
Actions cache rejects `..` path patterns, so CI must resolve the parent meta root
to a canonical absolute path before restore/save.

## Decisions

### Split Cache Layers

Use one cache for envctl Rust state:

- `$META_ROOT/.env/rust`
- `$META_ROOT/.cache/rust`

Key it on OS plus `scripts/ci/envctl-rust-env.sh` and
`scripts/ci/envctl-rust-audit.sh`.

Use a separate cache for workspace `target`, keyed on OS, `Cargo.lock`, and the
same bootstrap scripts. This preserves compile artifact invalidation when the
workspace dependency graph changes without forcing the parent tool cache to
miss.

Apply the same canonical-root split to CI, promotion verification, and release
build workflows.

### Timed Bootstrap Spans

Add a small Bash helper that wraps bootstrap commands with elapsed seconds and
GitHub log groups when `GITHUB_ACTIONS` is set. Apply it to:

- rustup toolchain install;
- `rustc-codegen-gcc` / preview component attempts;
- cargo tool installs for `kache`, `wild-linker`, and `cargo-audit`.

When a tool already exists on PATH, report the resolved path and skip install.

### Model-Loop Cheap Pass

Upgrade the read-only `explore` and `gap-hunt` passes to `gpt-5.5-mini` so
read-heavy scanning and gap discovery use the current cheaper model target. Keep
high-reasoning `verify` on `gpt-5.5`.

## Risks and Trade-Offs

- Splitting caches adds workflow YAML lines, but the ownership boundary becomes
  clearer and the cache miss domain is smaller.
- Timed spans improve diagnosis but do not by themselves make a cold tool
  install fast.
- `gpt-5.5-mini` availability remains a user/admin Codex configuration concern;
  Rusty IDD only emits the requested model name.

## Migration Plan

1. Create Rusty IDD goal and OpenSpec artifacts.
2. Split CI and promotion caches.
3. Add timed bootstrap logging.
4. Update model-loop config, docs, and tests.
5. Refresh knowledge and manifest.
6. Push a PR and let CI prove strict bootstrap still passes.

## Rollback

Revert this branch. The prior single-cache workflow and older model-loop pass
choices return without mutating host toolchains or user-global state.
