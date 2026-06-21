# IDD Spec Engine Automation Boundary

Date: 2026-06-21
Branch: `integration/idd-spec-engine-automation`
OpenSpec change: `openspec/changes/integrate-idd-spec-engine`

## Purpose

The integration queue selected `integrate-idd-spec-engine` as the next planned
work item. Rusty IDD already exposes human-readable OpenSpec lifecycle status,
but automated handoff/fleet workflows need a deterministic machine-readable
state surface before a runner can safely decide what to do next.

This change adds the thin Rusty IDD boundary for that capability:
`rusty-idd spec status --json <change_dir>`.

## Adopt-First Evidence

- Rusty IDD owner surface:
  - Current branch started from clean `develop`.
  - `AGENTS.md` requires adopt-first, OpenSpec artifacts, `.idd/MANIFEST.tsv`
    freshness, and PR evidence.
  - `AI_MERGE/23_integration_work_scaffold.md` shows the existing
    `spec plan-integration` bridge from generated integration work to OpenSpec
    artifacts.
  - `AI_MERGE/24_integration_status_queue.md` shows the existing deterministic
    integration queue and status classifications.
- Handoff owner surface:
  - Local peer repo: `/home/drdave/Desktop/meta/handoff`.
  - Current HEAD: `91d430b496602aea1dcc7149ce301a0be791d31c`.
  - Current branch: `fix/windows-ledger-path-and-promote-checkout`.
  - Existing dirty file before this slice: `.idea/handoff.iml`; this Rusty IDD
    slice did not modify handoff.
  - `AGENTS.md` says the repo is governed through the Continuity Ledger Kernel,
    `hf resume`, task claims, and handoff checkpoints.
  - `hf resume`: passed and reported 55/55 tasks done with 88 verified events.

## Native Diagnostics

- `cargo fmt --all -- --check` in Rusty IDD: passed.
- `cargo test -p rusty-idd-cli --test spec_status_cli --locked`: passed,
  5 tests.
- `hf resume` in handoff: passed.
- `make fmt-check` in handoff: failed before formatting because
  `Cargo.toml` references missing workspace member `cli`.
- `make test` in handoff: failed before tests because `Cargo.toml` references
  missing workspace member `cli`.

The handoff failures are owner-repo evidence for a future handoff repair slice;
they are not a reason to add handoff code, host services, or daemon management
to Rusty IDD's default workflow.

## Implementation

- Added `--json` to `rusty-idd spec status`.
- Reused the existing spec schema and filesystem artifact detection.
- Added a deterministic JSON snapshot containing:
  - change name and path
  - schema name and version
  - ordered artifact statuses
  - done count and total
  - archivability
  - next ready artifact
- Preserved the existing human `spec status` output by routing both modes
  through the same snapshot builder.
- Added focused CLI test coverage for the automation snapshot.

## Scope Boundary

This change does not mutate handoff, start services, run MCP servers, install
tools, alter vault relay behavior, or change the OpenSpec schema. It only adds
a deterministic CLI/API output shape for the already-proven spec status
surface.

## Validation

- `cargo fmt --all -- --check`: passed.
- `cargo test -p rusty-idd-cli --test spec_status_cli --locked`: passed,
  5 tests.
- `cargo run --bin rusty-idd -- spec status --json openspec/changes/integrate-idd-spec-engine`:
  passed and emitted deterministic JSON with schema, artifact status,
  archivability, and next-ready fields.
- `cargo run --bin rusty-idd -- spec status openspec/changes/integrate-idd-spec-engine`:
  passed and preserved human-readable status output.
- `cargo run --bin rusty-idd -- spec validate --changes`: passed, 49 items,
  0 failed; existing brief-purpose warnings remain.
- `just knowledge`, `just system-architecture`, `just operating-model`,
  `just integration-plan`, `just integration-status`, `just plan-context`, and
  `just manifest`: passed.
- `just knowledge-check`, `just operating-model-check`,
  `just integration-plan-check`, `just integration-status-check`,
  `just plan-context-check`, and `just manifest-check`: passed.
- `just ci`: passed.
- `make ci`: passed.
- `cargo test --workspace --all-features --locked`: passed, 625 tests,
  3 ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`:
  passed.
- `cargo audit --deny warnings`: passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed,
  0 critical and 0 warning.
- `cargo run --bin rusty-idd -- spec validate --all`: passed, 50 items,
  0 failed; existing brief-purpose warnings remain.

## Rollback

- Remove the `--json` flag from `SpecCommand::Status`.
- Revert `crates/cli/src/commands/spec_status.rs` to direct human rendering.
- Remove the focused JSON status CLI test.
- Delete `openspec/changes/integrate-idd-spec-engine`.
- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
