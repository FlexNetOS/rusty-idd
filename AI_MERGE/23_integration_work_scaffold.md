# Integration Work Scaffold

Date: 2026-06-21
Branch: `integration/scaffold-integration-work`
OpenSpec change: `openspec/changes/scaffold-integration-work`

## Purpose

Rusty IDD already generates detailed architecture graphs, system operating
models, and an ordered integration automation plan. This change closes the next
automation gap by turning an integration work item directly into OpenSpec
lifecycle artifacts.

## Implementation

- Added `rusty-idd spec plan-integration`.
- The command reads `.idd/knowledge/integration-plan.json` by default.
- It selects a work item by:
  - first priority item when no selector is passed
  - `--change <change_id>`
  - `--capability <capability>` with or without `capability:`
  - `--work-item <work:id>`
- It generates:
  - `proposal.md`
  - `design.md`
  - `tasks.md`
  - `specs/<capability>/spec.md`
- It preserves owner repos, anchors, adopt-first inputs, validation gates,
  implementation boundary, and rollback.
- It refuses to overwrite existing files unless `--force` is passed.
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md` so future agents use
  this command as the bridge from integration-plan output to OpenSpec work.

## Scope Boundary

This command only writes Rusty IDD OpenSpec artifacts. It does not mutate peer
repos, start MCP servers, manage daemons, install tools, or run host services.
Generated implementation still follows adopt-first TDD and the owner-repo
validation path.

## Evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test -p rusty-idd-cli plan_integration_creates_openspec_artifacts_from_integration_plan --locked`:
  passed.
- `cargo test -p rusty-idd-cli --test spec_scaffold_cli --locked`: passed.
- `cargo run --bin rusty-idd -- spec validate --changes`: passed
  structurally with existing non-failing short-purpose warnings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed structurally with
  existing non-failing short-purpose warnings.
- Real-plan smoke:
  `cargo run --bin rusty-idd -- spec plan-integration --base /tmp/... --integration-plan .idd/knowledge/integration-plan.json --change integrate-github-agent-run-upgrades`:
  passed and generated the expected OpenSpec change in `/tmp`.
- `just knowledge`, `just system-architecture`, `just operating-model`,
  `just integration-plan`, `just plan-context`, and `just manifest`: passed.
- `just ci`: passed.
- `make ci`: passed.
- `cargo test --workspace --all-features --locked`: passed, 621 passed and 3
  ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`:
  passed.
- `cargo audit --deny warnings`: passed.

## Rollback

- Remove `crates/cli/src/commands/spec_plan_integration.rs`.
- Remove the `spec plan-integration` subcommand wiring.
- Remove the focused CLI test.
- Revert the local skill update.
- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
