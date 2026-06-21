# Integration Status Queue

Date: 2026-06-21
Branch: `integration/integration-status-queue`
OpenSpec change: `openspec/changes/add-integration-status-queue`

## Purpose

Rusty IDD can now generate an integration plan and scaffold a selected work
item into OpenSpec artifacts. This change adds the missing queue state: a
deterministic report that joins integration work items to OpenSpec change and
archive state.

## Implementation

- Added `IntegrationStatusReport`, `IntegrationStatusCounts`, and
  `IntegrationWorkStatus` DTOs in `crates/knowledge`.
- Added `rusty-idd knowledge integration-status`.
- The command consumes `.idd/knowledge/integration-plan.json` by default.
- It reports each work item as:
  - `planned`
  - `incomplete-scaffold`
  - `scaffolded`
  - `ready-to-archive`
  - `archived`
- It identifies the next planned work item deterministically by priority.
- Added deterministic JSON and Markdown outputs:
  - `.idd/knowledge/integration-status.json`
  - `.idd/knowledge/integration-status.md`
- Added `just integration-status` / `integration-status-check`.
- Added `make integration-status` / `integration-status-check`.
- Added the status check to `just ci` and `make ci`.
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md`.

## Scope Boundary

The status queue is read-only. It does not mutate OpenSpec changes, archive
completed changes, create new integration work, mutate peer repos, start host
services, or run daemons/MCP servers.

## Evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test -p rusty-idd-knowledge integration_status_reports_queue_state_from_openspec_changes --locked`:
  passed.
- `cargo test -p rusty-idd-cli system_architecture_cli_discovers_peer_repos_without_meta --locked`:
  passed.
- `cargo run --bin rusty-idd -- spec validate --all`: passed structurally with
  existing non-failing short-purpose warnings.
- `just integration-status-check`: passed.
- `make integration-status-check`: passed.
- `just knowledge`, `just system-architecture`, `just operating-model`,
  `just integration-plan`, `just integration-status`, `just plan-context`, and
  `just manifest`: passed.
- Generated queue evidence:
  - 19 total work items.
  - 19 planned items.
  - 0 incomplete-scaffold, scaffolded, ready-to-archive, or archived items.
  - Next planned work item: `integrate-idd-spec-engine`.
- `just ci`: passed.
- `make ci`: passed.
- `cargo test --workspace --all-features --locked`: passed, 622 passed and 3
  ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`:
  passed.
- `cargo audit --deny warnings`: passed.

## Rollback

- Remove the integration status DTOs and builder.
- Remove the `knowledge integration-status` subcommand.
- Remove integration-status targets/checks from Justfile and Makefile.
- Delete `.idd/knowledge/integration-status.json` and `.md`.
- Revert the local skill update.
- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
