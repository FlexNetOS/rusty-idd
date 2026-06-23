# align-envctl-toolchain-contract

## Why

The self-upgrade governor produced a new goal from validation evidence: Rusty
IDD's CI helper uses `$META_ROOT/.env/rust`, while envctl's workstation
contract uses `$META_ROOT/.toolchains`. Both paths are meta-owned, but treating
them as one contract caused local validation to misreport the environment.

Rusty IDD needs an explicit layout boundary so agents report actual compiler
paths correctly and do not mistake meta-owned symlinked tools for user-global
state.

## What Changes

- Add an explicit Rust layout selector to `scripts/ci/envctl-rust-env.sh`.
- Keep GitHub Actions defaulting to the isolated `.env/rust` layout.
- Let local runs default to the envctl `.toolchains` layout when
  `$META_ROOT/.toolchains/cargo/bin/rustup` exists.
- Document the two supported meta-owned layouts.
- Record this as the first self-generated goal from the governor loop.

## Capabilities

### New

- `envctl-toolchain-layout`: Rusty IDD recognizes CI isolated and local
  envctl-toolchains Rust layouts as meta-owned.

### Modified

- `codex-harness-flow`: Rust/Codex environment audits must report the selected
  layout and the actual Cargo-executed compiler path.

## Impact

- Affected artifacts:
  - `.idd/goals/align-envctl-toolchain-contract.md`
  - `openspec/changes/align-envctl-toolchain-contract/*`
  - `adr/0013-envctl-toolchain-layouts.md`
  - `scripts/ci/envctl-rust-env.sh`
  - `docs/rusty-idd/codex-environment.md`
  - `.idd/evidence/envctl-toolchain-contract/*`
  - `.idd/knowledge/*`
  - `.idd/MANIFEST.tsv`
- No user-global Rust install is added.
- No sccache fallback is introduced.
