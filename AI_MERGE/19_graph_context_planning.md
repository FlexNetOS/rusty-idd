# Graph Context Planning

Date: 2026-06-21
Branch: `integration/graph-context-planning`
OpenSpec change: `openspec/changes/add-graph-context-planning`

## Purpose

PR #55 generated repo-local architecture graphs. PR #56 generated parent meta
system graphs. This change closes the next automation gap: Rusty IDD now
consumes those graph artifacts to produce a bounded planning packet for
OpenSpec proposal, spec, design, ADR, task, and implementation work.

## Implementation

- Added `rusty-idd knowledge plan-context`.
- Added `GraphPlanningContext` DTOs in `crates/knowledge`.
- Inputs:
  - required repo architecture graph, defaulting to
    `.idd/knowledge/architecture.json`
  - optional system architecture graph, defaulting to
    `.idd/knowledge/system-architecture.json`
  - optional `--goal`, `--goal-file`, and `--change`
- Outputs:
  - Markdown for humans and agents
  - JSON for future automation
- Added `just plan-context`.
- Added `just plan-context-check`.
- Added `plan-context-check` to `just ci`.

## Current Generated Context

Generated for the active goal:

- Change: `graph-context-planning`
- Focus components: 9
- System roles: 12
- System repos: 20
- Finding: selected 12 roles and 20 repos from 65 discovered repos.

The generated context includes:

- repo source graph and repomix context package metrics
- Rusty IDD automation order
- integration surfaces
- focus components selected from the architecture graph
- relevant system roles and repos selected from the meta system graph
- graph-backed planning guidance for proposal/spec/design/ADR/tasks

## Evidence

- `cargo test -p rusty-idd-knowledge graph_planning_context_consumes_repo_graph_without_system_graph --locked`:
  passed.
- `cargo test -p rusty-idd-cli knowledge_commands_cover_index_pack_report_query_and_refresh --locked`:
  passed.
- `cargo run --bin rusty-idd -- knowledge refresh --workspace .`: passed.
- `just system-architecture`: passed.
- `just plan-context`: passed.
- `cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv`:
  passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed with 0
  critical and 0 warnings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed structurally with
  existing non-failing "Purpose section is too brief" warnings.
- `just knowledge-check`: passed.
- `just plan-context-check`: passed.
- `just manifest-check`: passed.
- `just ci`: passed with `plan-context-check` included.

## Scope Boundary

This is a deterministic planning-context generator. It does not invoke an LLM,
start services, mutate peer repos, or replace the OpenSpec lifecycle engine.
The output is a graph-backed input to the lifecycle, not a replacement for
proposal/spec/design/ADR/tasks.

## Rollback

- Revert the `GraphPlanningContext` DTOs and `knowledge plan-context` command.
- Remove `.idd/knowledge/plan-context.json` and `.idd/knowledge/plan-context.md`.
- Remove `plan-context` and `plan-context-check` from `Justfile`.
- Re-run `rusty-idd knowledge refresh --workspace .` and
  `rusty-idd manifest --workspace . --out .idd/MANIFEST.tsv`.
