# System Operating Model Graph

Date: 2026-06-21
Branch: `integration/system-operating-model-graph`
OpenSpec change: `openspec/changes/add-system-operating-model-graph`

## Purpose

Rusty IDD now has repo architecture, system architecture, graph planning
context, and bounded peer architecture summaries. This change adds the next
planning layer: a generated operating model for the wider agentic company
system.

The operating model maps the discovered meta repo fleet to company/system
layers and capabilities such as board reasoning, Rusty IDD plus handoff, weave
communication, envctl/vault relay, prompt front door, ruvector runtime,
LifeOS/front-door UX, Teri simulation, network control, parser/runtime, Lua/AR,
distributed device fabric, Yazelix terminal/runtime, RTK AI foundation, and
GitHub agent-run upgrades through GRIT and Beads.

## Implementation

- Added `SystemOperatingModel` DTOs in `crates/knowledge`.
- Added `rusty-idd knowledge operating-model`.
- The command consumes `.idd/knowledge/system-architecture.json` by default.
- Added deterministic JSON and Markdown outputs:
  - `.idd/knowledge/operating-model.json`
  - `.idd/knowledge/operating-model.md`
- Added `just operating-model` and `just operating-model-check`.
- Added `operating-model-check` to `just ci`.
- Added `make operating-model` and `make operating-model-check`.
- Added `operating-model-check` to `make ci`.
- Graph planning context now reads `.idd/knowledge/operating-model.json` by
  default and includes selected operating layers/capabilities.
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md`.
- Verified Beads upstream candidates and recorded exact HEAD anchors:
  - `github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca`
  - `github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b`

## Scope Boundary

The operating model is read-only. It does not mutate peer repos, start services,
invoke MCP, generate missing peer knowledge artifacts, run model runtimes, or
claim that external anchors are implemented. External or future anchors are
recorded as findings.

## Evidence

- `cargo test -p rusty-idd-knowledge system_operating_model_maps_agentic_company_capabilities --locked`:
  passed.
- `cargo test -p rusty-idd-knowledge graph_planning_context_preserves_peer_architecture_summary --locked`:
  passed.
- `cargo test -p rusty-idd-cli system_architecture_cli_discovers_peer_repos_without_meta --locked`:
  passed.
- `cargo run --bin rusty-idd -- knowledge refresh --workspace .`: passed.
- `just system-architecture`: passed.
- `just operating-model`: passed.
- `just plan-context`: passed.
- `cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv`:
  passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed with 0
  critical and 0 warnings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed structurally with
  existing non-failing "Purpose section is too brief" warnings.
- `just knowledge-check`: passed.
- `just operating-model-check`: passed.
- `just plan-context-check`: passed.
- `just manifest-check`: passed.
- `make operating-model-check`: passed.
- `just ci`: passed.
- `make ci`: passed.
- `cargo test --workspace --all-features --locked`: passed, 619 passed and 3
  ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`:
  passed.
- `cargo audit --deny warnings`: passed.

Generated operating model evidence:

- 11 operating layers.
- 19 operating capabilities.
- 162 operating edges.
- `capability:parser-runtime` maps `rusty-idd`, `tool-hub`, and `yazelix` and
  records Yazelix, nushell, Lua, Ghostty, and Zellij anchors.
- `capability:rtk-ai-foundation` maps `grit`, `icm`, `rtk-tokenkill`, and
  `vox`.
- `capability:github-agent-run-upgrades` maps `grit` and `yazelix` and records
  both Beads upstream HEAD anchors.
- Graph planning context includes 18 selected operating capabilities.

## Rollback

- Remove the `SystemOperatingModel` DTOs and `build_system_operating_model`.
- Remove the CLI `knowledge operating-model` subcommand.
- Delete `.idd/knowledge/operating-model.json` and `.md`.
- Remove `operating-model` and `operating-model-check` from `Justfile`.
- Re-run knowledge, system-architecture, plan-context, and manifest generation.
