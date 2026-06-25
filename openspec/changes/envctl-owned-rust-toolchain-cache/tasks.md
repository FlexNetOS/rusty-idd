# envctl-owned-rust-toolchain-cache - Tasks

## 1. Workflow Artifacts

- [x] 1.1 Create the goal file under `.idd/goals`.
- [x] 1.2 Add OpenSpec proposal, spec delta, design, and task list.
- [x] 1.3 Add ADR 0009 for meta/envctl Rust toolchain ownership.
- [x] 1.4 Generate plan context with `rusty-idd knowledge plan-context --goal-file`.

## 2. Audit Implementation

- [x] 2.1 Extend `rusty-idd codex system-audit` with an opt-in strict Rust
  toolchain/cache audit.
- [x] 2.2 Report actual Cargo-executed compiler path, Cargo binary path,
  wrapper, linker, toolchain channel, homes, and cache root.
- [x] 2.3 Reject user-global and system-owned Rust/cache paths under strict
  audit.
- [x] 2.4 Replace mold-specific envctl audit wording with `wild-linker`.

## 3. Documentation And Verification

- [x] 3.1 Document the meta/envctl Rust toolchain/cache contract.
- [x] 3.2 Add focused tests for compliant and non-compliant Rust toolchain audit
  cases.
- [x] 3.3 Run focused tests and Rusty IDD validation.
- [x] 3.4 Refresh deterministic knowledge and manifest artifacts.

## 4. CI Implementation

- [x] 4.1 Add a tracked CI bootstrap that materializes `RUSTUP_HOME`,
  `CARGO_HOME`, compiler cache, and Rust binaries under the meta/envctl root.
- [x] 4.2 Replace workflow Rust setup/cache actions with the tracked bootstrap
  and explicit meta-owned cache paths.
- [x] 4.3 Run the strict Rust toolchain audit in primary CI and promotion
  verification.
- [x] 4.4 Reject `sccache` in the CI bootstrap path so kache/hurry/zccache stay
  the active implementation target.
