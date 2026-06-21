# Peer Architecture Detail Ingestion

Date: 2026-06-21
Branch: `integration/peer-architecture-detail-ingestion`
OpenSpec change: `openspec/changes/add-peer-architecture-detail-ingestion`

## Purpose

PR #56 made Rusty IDD aware of the parent meta repo fleet. PR #57 made that
system graph available to graph-backed OpenSpec planning. This change upgrades
the system graph from "peer architecture artifact exists" to "bounded peer
architecture details are available for automation".

## Implementation

- Added `PeerArchitectureSummary` to `SystemRepo`.
- During `knowledge system-architecture`, Rusty IDD reads peer
  `.idd/knowledge/architecture.json` artifacts when they exist.
- Summaries include:
  - CodeGraph source metrics
  - repomix context package metrics
  - top components
  - integration surfaces
- Bad or unreadable peer architecture artifacts are recorded as findings and do
  not fail system graph generation.
- System architecture Markdown now includes `Peer Architecture Summaries`.
- Graph planning context preserves selected peer architecture summaries in JSON
  and renders compact detail in the system repo table.
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md`.

## Scope Boundary

This remains a read-only graph ingestion workflow. It does not generate missing
peer architecture artifacts, mutate peer repos, start services, invoke MCP, or
add semantic/vector search. Missing peer summaries remain non-fatal until the
peer repo publishes its own Rusty IDD knowledge artifacts.

## Evidence

- `cargo fmt --all -- --check`: passed.
- `cargo test -p rusty-idd-knowledge system_architecture_graph_ingests_peer_architecture_summary --locked`: passed.
- `cargo test -p rusty-idd-knowledge graph_planning_context_preserves_peer_architecture_summary --locked`: passed.
- `cargo test -p rusty-idd-cli system_architecture_cli_discovers_peer_repos_without_meta --locked`: passed.
- `cargo run --bin rusty-idd -- knowledge refresh --workspace .`: passed.
- `just system-architecture`: passed.
- `just plan-context`: passed.
- `cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv`: passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed with 0
  critical and 0 warnings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed structurally with
  existing non-failing "Purpose section is too brief" warnings.
- `just knowledge-check`: passed.
- `just plan-context-check`: passed.
- `just manifest-check`: passed.
- `just ci`: passed.
- `make ci`: passed.
- `cargo test --workspace --all-features --locked`: passed, 618 passed and 3
  ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`:
  passed.
- `cargo audit --deny warnings`: passed.
- CLI smoke:
  `rusty-idd knowledge system-architecture --workspace . --system-root ..` and
  `rusty-idd knowledge plan-context --workspace . --change peer-architecture-detail-ingestion`
  produced non-empty Markdown outputs.

Generated system evidence:

- 65 peer repos discovered from `meta project list --json`.
- 1 repo currently exposes `.idd/knowledge/architecture.json`.
- 1 repo exposes a parsed peer architecture summary.
- The current Rusty IDD summary records 134 files, 8059 CodeGraph nodes, 32075
  CodeGraph edges, 100809 repomix tokens, 4 integration surfaces, and top
  components including `codegraph-core`, `codegraph-parser`, `tui`, `runner`,
  and `cli`.

## Rollback

- Remove the `PeerArchitectureSummary` DTOs and `SystemRepo.local_architecture`.
- Revert peer architecture parsing in system graph enrichment.
- Remove peer architecture Markdown rendering and plan-context table detail.
- Revert the OpenSpec change and skill update.
- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
