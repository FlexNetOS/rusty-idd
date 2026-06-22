# ADR-0003: Generated Architecture Diagrams

## Status

Accepted

## Context

Rusty IDD already treats knowledge, architecture, system architecture, operating
model, integration, readiness, manifest, and validation outputs as deterministic
control-plane artifacts. Architecture diagrams are useful agent context, but a
manual diagram document can drift from the current code graph and artifact
surface.

## Decision

Architecture diagrams for Rusty IDD will be generated from the same knowledge
architecture graph used by `.idd/knowledge/architecture.json`. The generator
lives in `crates/knowledge`, the CLI surface lives under `rusty-idd knowledge`,
and repository freshness checks compare the checked-in diagram file against
fresh output.

## Consequences

- Diagram changes become reviewable generated diffs.
- Agents can rely on diagrams as current codebase context.
- Narrative design discussion remains in OpenSpec, ADR, and AI_MERGE evidence
  rather than inside the generated diagram artifact.
