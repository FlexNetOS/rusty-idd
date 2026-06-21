# Queue-Aware Plan Integration

Date: 2026-06-21
Branch: `integration/queue-aware-plan-integration`
OpenSpec change: `openspec/changes/queue-aware-plan-integration`

## Purpose

`rusty-idd spec plan-integration` created OpenSpec artifacts from the
integration plan, but default selection still used raw plan order. After the
first item was scaffolded, repeated default runs could hit overwrite protection
instead of moving to the next planned item.

This change makes default scaffolding queue-aware: active and archived OpenSpec
changes are skipped when no selector is passed.

## Implementation

- Updated no-selector `spec plan-integration` selection.
- Default selection skips:
  - `openspec/changes/<change_id>`
  - `openspec/changes/archive/<change_id>`
- Explicit `--change`, `--capability`, and `--work-item` selectors remain
  exact and keep existing overwrite protection.
- Added focused CLI tests for:
  - default queue advancement past an existing active change
  - clear failure when no planned work remains
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md`.

## Scope Boundary

This change only affects Rusty IDD OpenSpec artifact selection. It does not run
implementation tasks, archive changes, mutate peer repos, start services, or
change the generated integration queue.

## Evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test -p rusty-idd-cli --test spec_scaffold_cli --locked`: passed,
  8 tests.
- `cargo run --bin rusty-idd -- spec validate --changes`: passed, 45 valid
  change items, 0 failed; existing brief-purpose warnings remain.
- `just knowledge`, `just system-architecture`, `just operating-model`,
  `just integration-plan`, `just integration-status`, `just plan-context`, and
  `just manifest`: passed.
- `just knowledge-check`, `just operating-model-check`,
  `just integration-plan-check`, `just integration-status-check`,
  `just plan-context-check`, and `just manifest-check`: passed.
- `just ci`: passed.
- `make ci`: passed.
- `cargo test --workspace --all-features --locked`: passed, 624 tests,
  3 ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`:
  passed.
- `cargo audit --deny warnings`: passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed,
  0 critical and 0 warning.
- `cargo run --bin rusty-idd -- spec plan-integration --help`: passed.

## Rollback

- Revert `crates/cli/src/commands/spec_plan_integration.rs`.
- Remove the queue-aware focused tests.
- Revert the local skill update.
- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
