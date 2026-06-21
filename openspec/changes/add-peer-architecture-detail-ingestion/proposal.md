# add-peer-architecture-detail-ingestion

## Why

Rusty IDD now generates repo-local architecture graphs, parent meta system
graphs, and graph-backed planning context. The system graph still treats peer
repo architecture artifacts as a boolean: present or absent.

To move toward detailed full-system architecture, Rusty IDD needs to ingest
available peer `.idd/knowledge/architecture.json` files and summarize their
component, surface, language, and source-graph details inside the system graph
and planning context.

## What Changes

- Extend `SystemRepo` with a bounded peer architecture summary.
- Read peer `.idd/knowledge/architecture.json` files when they exist.
- Include source graph metrics, languages, top components, and integration
  surfaces from peer architecture artifacts.
- Render peer architecture summaries in system architecture Markdown.
- Include peer architecture summaries in graph planning context.

## Capabilities

### New Capabilities

- `peer-architecture-detail-ingestion`: system graph generation consumes
  available peer architecture graph details instead of only recording artifact
  presence.

### Modified Capabilities

- `system-architecture-peer-graph`: peer repos with local architecture graphs
  now carry bounded architecture summaries.
- `graph-context-planning`: planning context can use peer architecture detail
  when available.

## Impact

- `crates/knowledge`
- `.idd/knowledge`
- `.agents/skills/rusty-idd-knowledge`
- `/AI_MERGE`
