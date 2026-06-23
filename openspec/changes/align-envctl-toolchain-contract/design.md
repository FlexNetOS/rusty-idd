# align-envctl-toolchain-contract - Design

## Context

Rusty IDD now has a self-upgrade governor goal. Its first generated goal comes
from a concrete environment audit: CI proved the strict `.env/rust` layout, but
the workstation envctl contract is `.toolchains`.

The parent envctl ADR and `envctl env --toolchains` define
`CARGO_HOME=$META_ROOT/.toolchains/cargo`. The local `~/.cargo/bin` entries are
symlinks into that meta-owned tree, not independent user-global state.

## Goals

- Preserve CI's isolated strict nightly/kache/wild path.
- Support local envctl `.toolchains` as a first-class meta-owned layout.
- Surface the selected layout in toolchain output.
- Keep all mutable Rust state under the meta root.
- Avoid sccache fallback.

## Non-Goals

- Do not migrate CI cache paths in this slice.
- Do not install or remove local Rust tools.
- Do not require a live envctl/weave session to validate this script.
- Do not change the Rusty IDD compiler backend policy.

## Decisions

### Layout Selector

`scripts/ci/envctl-rust-env.sh` accepts `RUSTY_IDD_RUST_LAYOUT`:

- `isolated` or `env`: `$META_ROOT/.env/rust/{rustup,cargo,bin}`;
- `toolchains`: `$META_ROOT/.toolchains/{rustup,cargo}` with
  `RUSTY_IDD_RUST_BIN=$CARGO_HOME/bin`.

If unset, GitHub Actions selects `isolated`. Local runs select `toolchains`
when `$META_ROOT/.toolchains/cargo/bin/rustup` and
`$META_ROOT/.toolchains/rustup` exist, otherwise `isolated`.

### Audit Output

The script prints `layout: <layout>` alongside `META_ROOT`, `RUSTUP_HOME`,
`CARGO_HOME`, actual `rustc path`, actual `cargo path`, wrapper, and flags.

### Contract Boundary

Both supported layouts are valid only when they resolve under `META_ROOT`.
User-global-looking symlinks are acceptable only when their real paths resolve
under meta. Real user-global or system-depth Rust homes remain invalid for Rusty
IDD validation.

## Risks and Trade-Offs

- Local `toolchains` mode may install missing optional tools into the envctl
  cargo prefix if strict `ci` mode is run locally. This remains meta-owned and
  explicit through the selected mode.
- CI and local paths are now different by design. Documentation and audit output
  mitigate ambiguity.

## Migration Plan

1. Add the layout selector and output.
2. Update documentation.
3. Validate `release` mode locally with `RUSTY_IDD_RUST_LAYOUT=toolchains`
   because Rust 1.96.0 is already installed there.
4. Let CI continue proving strict `isolated` mode.

## Open Questions

- Should envctl later expose `RUSTUP_HOME` directly in `envctl env --toolchains`
  to avoid Rusty IDD inferring `$META_ROOT/.toolchains/rustup`?
- Should CI eventually use `.toolchains` too, or remain isolated for cache
  clarity?
