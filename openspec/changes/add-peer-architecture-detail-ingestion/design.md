# add-peer-architecture-detail-ingestion - Design

## Context

The parent meta graph knows about 65 repos and role relationships, but only one
repo currently exposes `.idd/knowledge/architecture.json`. The graph records
that fact, but does not ingest the artifact. That limits the system graph to
metadata rather than detailed architecture.

## Goals / Non-Goals

**Goals:**

- Ingest peer architecture graphs where they exist.
- Keep summaries bounded and deterministic.
- Preserve read-only behavior.
- Include architecture detail in both system graph and plan context.

**Non-Goals:**

- Recursively generating architecture graphs in peer repos.
- Mutating or validating peer repos.
- Starting services or host daemons.
- Adding semantic/vector search.

## Decisions

- Add a small `PeerArchitectureSummary` DTO to `SystemRepo`.
- Summarize only counts, languages, top components, and surfaces.
- Keep full peer architecture JSON in the peer repo; do not inline it into the
  system graph.
- Treat parse failures in peer architecture JSON as findings rather than fatal
  errors, so one bad peer artifact does not block system graph generation.

## Risks / Trade-offs

- The first generated system graph will only have detail for repos that already
  publish architecture artifacts. That is still useful because the schema and
  automation path are ready for more peers as they adopt Rusty IDD knowledge.

## Migration Plan

1. Add OpenSpec artifacts.
2. Extend system graph DTOs.
3. Ingest bounded peer architecture summaries.
4. Render details in system graph and planning context.
5. Refresh artifacts and validate.
