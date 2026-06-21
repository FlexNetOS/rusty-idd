# Integration Automation Plan

Date: 2026-06-21
Branch: `integration/operating-model-integration-plan`
OpenSpec change: `openspec/changes/add-integration-automation-plan`

## Purpose

Rusty IDD can now generate detailed architecture, system, peer, planning, and
operating-model graphs. This change turns that descriptive operating model into
an ordered integration automation plan: deterministic work items that can become
OpenSpec slices across Rusty IDD, handoff, weave, envctl, prompt_hub,
ruvector, Yazelix, GRIT/Beads, and other system repos.

## Implementation

- Added `IntegrationAutomationPlan` DTOs in `crates/knowledge`.
- Added `rusty-idd knowledge integration-plan`.
- The command consumes `.idd/knowledge/operating-model.json` by default.
- Added deterministic JSON and Markdown outputs:
  - `.idd/knowledge/integration-plan.json`
  - `.idd/knowledge/integration-plan.md`
- Graph planning context now reads `.idd/knowledge/integration-plan.json` by
  default and includes selected work items.
- Added `just integration-plan` and `just integration-plan-check`.
- Added `make integration-plan` and `make integration-plan-check`.
- Added integration-plan freshness checks to both `just ci` and `make ci`.
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md`.

## Scope Boundary

The integration plan is read-only. It does not mutate peer repos, install
tools, start services, select a canonical Beads implementation, or open
cross-repo PRs. It preserves anchors and owner repos so the next Rusty IDD
slice can adopt upstream/current surfaces first.

## Evidence

- `cargo test -p rusty-idd-knowledge integration_automation_plan_orders_operating_capability_work --locked`:
  passed.
- `cargo test -p rusty-idd-knowledge graph_planning_context_preserves_peer_architecture_summary --locked`:
  passed.
- `cargo test -p rusty-idd-cli system_architecture_cli_discovers_peer_repos_without_meta --locked`:
  passed.
- `cargo run --bin rusty-idd -- knowledge refresh --workspace .`: passed.
- `just system-architecture`: passed.
- `just operating-model`: passed.
- `just integration-plan`: passed.
- `just plan-context`: passed.
- `cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv`:
  passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed with 0
  critical and 0 warnings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed structurally with
  existing non-failing "Purpose section is too brief" warnings.
- `just knowledge-check`: passed.
- `just operating-model-check`: passed.
- `just integration-plan-check`: passed.
- `just plan-context-check`: passed.
- `just manifest-check`: passed.
- `make integration-plan-check`: passed.
- `just ci`: passed.
- `make ci`: passed.
- `cargo test --workspace --all-features --locked`: passed, 620 passed and 3
  ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`:
  passed.
- `cargo audit --deny warnings`: passed.

Generated integration plan evidence:

- 19 ordered integration work items.
- 8 adopt-first inputs preserved from operating-model anchors.
- First priorities:
  - `integrate-idd-spec-engine`
  - `integrate-fleet-handoff`
  - `integrate-agent-communication`
  - `integrate-env-vault-relay`
  - `integrate-prompt-front-door`
  - `integrate-github-agent-run-upgrades`
- Plan context includes 12 selected integration work items in priority order.

## Rollback

- Remove `IntegrationAutomationPlan` DTOs and `build_integration_automation_plan`.
- Remove the CLI `knowledge integration-plan` subcommand.
- Delete `.idd/knowledge/integration-plan.json` and `.md`.
- Remove integration-plan targets/checks from Justfile and Makefile.
- Re-run knowledge, system-architecture, operating-model, plan-context, and
  manifest generation.
