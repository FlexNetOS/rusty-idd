# upgrade-architecture-graph-workflow - Design

## Context

Rusty IDD is the lifecycle controller for goal intake, OpenSpec artifacts, ADRs,
task execution, validation, and merge evidence. The knowledge layer already uses
CodeGraph for multi-language tree-sitter parsing and repomix for bounded context
packages, but the workflow does not yet expose an explicit architecture graph
that maps those details to Rusty IDD automation.

## Goals / Non-Goals

**Goals:**

- Generate a deterministic architecture graph artifact.
- Preserve the thin local boundary: DTO mapping, deterministic output, token
  policy, validation, and CLI/API access.
- Treat CodeGraph and repomix features as architecture-mapping capability.
- Keep `crates/core` std-only.
- Keep host-service, daemon, MCP, vector, cloud, and domain surfaces out of the
  default path unless feature-gated in a later change.

**Non-Goals:**

- Starting MCP servers or daemons.
- Replacing Rusty IDD's OpenSpec lifecycle with CodeGraph or repomix.
- Adding vector databases, LSP runtime management, or host service management
  to the default workflow.

## Decisions

- Add the architecture graph in `crates/knowledge`, not `crates/core`.
- Build the artifact from the existing CodeGraph-backed `KnowledgeIndex` and a
  repomix pack summary so both upstream integrations participate in generation.
- Model repo components, integration surfaces, automation stages, and edges as
  serializable DTOs.
- Add a CLI command for direct artifact generation and extend refresh to write
  `.idd/knowledge/architecture.json` and `.idd/knowledge/architecture.md`.

## Risks / Trade-offs

- Architecture mapping is initially heuristic because Rusty IDD does not yet
  own a cross-repo system graph database. The artifact must therefore record
  evidence and stage mapping without pretending to be a complete runtime graph.
- Running repomix during refresh adds work, but the output gives the automation
  loop token-budget and context-package evidence.

## Migration Plan

1. Add OpenSpec artifacts.
2. Add architecture graph DTOs and renderer.
3. Add CLI command and refresh output.
4. Add focused tests.
5. Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
6. Record implementation evidence in `/AI_MERGE`.
