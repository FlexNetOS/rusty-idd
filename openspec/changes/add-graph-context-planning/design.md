# add-graph-context-planning - Design

## Context

PR #55 added repo architecture graphs. PR #56 added system architecture graphs.
The next workflow gap is consumption: Rusty IDD needs a deterministic, bounded
artifact that turns those graphs into a planning packet for OpenSpec changes.

## Goals / Non-Goals

**Goals:**

- Consume existing architecture graph artifacts.
- Keep output deterministic and bounded.
- Support Markdown for humans/agents and JSON for future automation.
- Include enough graph context to guide proposal, spec, design, ADR, tasks, and
  implementation order.
- Keep this in `crates/knowledge` and the CLI layer.

**Non-Goals:**

- Generating a complete OpenSpec change automatically in this slice.
- Starting services or mutating peer repos.
- Adding LLM/provider calls.
- Replacing the existing OpenSpec lifecycle engine.

## Decisions

- Add `knowledge plan-context` rather than folding this into `spec new`.
- Treat missing system graph as non-fatal; repo-local graph is the required
  input.
- Filter system repos by role/name/tag/marker matches against the goal text.
- Include all automation stages and surfaces from the repo architecture graph.
- Include top components and matching system repos/roles to keep the artifact
  bounded.

## Risks / Trade-offs

- The first relevance model is deterministic keyword matching, not semantic
  search. This is acceptable because it is explainable and can later be upgraded
  behind the same DTO boundary.
- The command can only consume graphs that already exist. The skill and docs
  must continue to instruct agents to refresh graphs first.

## Migration Plan

1. Add OpenSpec artifacts.
2. Add planning-context DTOs and renderers.
3. Add CLI command and tests.
4. Generate a planning-context artifact for the active graph workflow goal.
5. Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
6. Validate and merge.
