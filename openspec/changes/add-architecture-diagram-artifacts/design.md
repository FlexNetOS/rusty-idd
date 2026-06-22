# add-architecture-diagram-artifacts - Design

## Context

Rusty IDD's knowledge layer already emits machine-readable and human-readable
architecture artifacts from CodeGraph and repomix data. Agents also need compact
architecture diagrams, but the existing diagram document is a manual artifact.
Manual diagrams drift as crates, stages, and integration surfaces change.

## Goals / Non-Goals

**Goals:**

- Generate architecture diagrams deterministically from the current architecture
  graph.
- Keep the diagram generator in `crates/knowledge`, with CLI wiring in
  `crates/cli`.
- Emit Mermaid Markdown so diagrams are reviewable in GitHub and useful in
  agent context.
- Preserve `crates/core` as the low-dependency core boundary.
- Refresh all deterministic artifacts after implementation.

**Non-Goals:**

- Add a graph database, MCP server, daemon, or hosted diagram renderer.
- Replace the existing architecture graph JSON/Markdown artifacts.
- Mutate peer repos while generating diagrams.

## Decisions

- Add `build_architecture_diagrams` in `crates/knowledge` so diagrams reuse the
  same `ArchitectureGraph` DTOs as `.idd/knowledge/architecture.*`.
- Add `rusty-idd knowledge diagrams --workspace <path> --out <path>` as the CLI
  surface.
- Add a `diagrams` recipe and include it in `just ci` freshness checks.
- Generate `docs/rusty-idd/architecture-diagrams.md` from current graph data and
  include gap notes in `AI_MERGE`.

## Risks / Trade-offs

- Mermaid diagrams can become too dense if every graph edge is rendered. The
  generator should keep the first version bounded to lifecycle, crate
  dependency, and artifact-flow views.
- The generated document replaces a richer hand-authored explanation with a
  deterministic view. Longer narrative belongs in design docs and AI_MERGE
  evidence, not in a generated artifact.

## Migration Plan

1. Add OpenSpec and ADR records.
2. Add deterministic diagram generation in the knowledge crate.
3. Wire the CLI and `Justfile` recipes.
4. Regenerate all Rusty IDD artifacts.
5. Record gap and validation evidence.
