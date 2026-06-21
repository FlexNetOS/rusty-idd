# System Architecture Peer Graph

Date: 2026-06-21
Branch: `integration/system-architecture-peer-graph`
OpenSpec change: `openspec/changes/add-system-architecture-peer-graph`

## Purpose

This note records the next Rusty IDD workflow upgrade after the repo-local
architecture graph. The goal is to map the wider meta peer-repo system so Rusty
IDD can plan integrations against the real system layout instead of treating one
repo as the whole world.

## Implementation

- Added `rusty-idd knowledge system-architecture --workspace <repo> --system-root <root> --out <file>`.
- Added `SystemArchitectureGraph` DTOs in `crates/knowledge`.
- Discovery order:
  - Prefer `meta project list --json` from the system root.
  - Fall back to immediate child git repo discovery.
- Enrichment:
  - repo path, remote URL, branch, head, dirty state, tags, markers, local
    architecture artifact presence, and integration roles.
  - current Rusty IDD workspace branch/head/dirty state is intentionally not
    baked into the artifact so the generated file does not become stale after
    commit or merge.
- Added `just system-architecture` to generate:
  - `.idd/knowledge/system-architecture.json`
  - `.idd/knowledge/system-architecture.md`

## Current Meta Evidence

Generated against `/home/drdave/Desktop/meta`:

- Discovery source: `meta project list --json`
- Repos: 65
- Roles: 13
- Edges: 196
- Dirty peer repos recorded as evidence: 9
- Repos with local `.idd/knowledge/architecture.json`: 1

Key roles now represented:

- Rusty IDD control plane
- Fleet handoff
- Coordination and domain surface
- Domain upgrade surface
- Parser/runtime surface
- Toolchain provider
- Spec producer
- Meta control plane
- Capability hub
- Agent environment
- Knowledge and memory
- Documentation and knowledge
- Rust code surface

## Scope Boundary

This change is read-only. It does not start MCP servers, daemons, host services,
or mutate peer repos. It records those surfaces as system architecture roles so
future Rusty IDD tasks can decide where feature-gated integration belongs.

## Evidence

- `cargo test -p rusty-idd-knowledge system_architecture_graph_maps_peer_repo_roles --locked`:
  passed.
- `cargo test -p rusty-idd-cli system_architecture_cli_discovers_peer_repos_without_meta --locked`:
  passed.
- `just system-architecture`: passed.
- `cargo run --bin rusty-idd -- knowledge refresh --workspace .`: passed.
- `cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv`:
  passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed with 0
  critical and 0 warnings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed structurally with
  existing non-failing "Purpose section is too brief" warnings.
- `just knowledge-check`: passed.
- `just manifest-check`: passed.
- `just ci`: passed.

## Rollback

- Revert the system architecture DTOs and CLI command from `crates/knowledge`
  and `crates/cli`.
- Remove `.idd/knowledge/system-architecture.json` and
  `.idd/knowledge/system-architecture.md`.
- Re-run `rusty-idd knowledge refresh --workspace .` and
  `rusty-idd manifest --workspace . --out .idd/MANIFEST.tsv`.
