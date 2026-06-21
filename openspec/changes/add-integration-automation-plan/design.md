# add-integration-automation-plan - Design

## Context

The system operating model maps repos and anchors to agentic-company
capabilities, but future automation needs a stricter shape: ordered work items
that can become OpenSpec changes or cross-repo tasks.

This change adds a deterministic planning artifact. It is intentionally still
read-only and does not execute cross-repo work. Its job is to create the next
automation input that tells Rusty IDD which integration slices are ready, which
need upstream adoption, and which need external anchors resolved.

## Goals / Non-Goals

**Goals:**

- Convert operating capabilities into ordered integration work.
- Preserve repo owners, anchors, status, gates, and rollback guidance.
- Highlight adopt-first requirements for upstream/external anchors.
- Feed selected work items into graph planning context.
- Keep generation deterministic and checkable.

**Non-Goals:**

- Mutating peer repos.
- Installing tools.
- Starting services or model runtimes.
- Choosing a canonical Beads implementation.
- Automatically opening cross-repo PRs.

## Decisions

- Add `IntegrationAutomationPlan` DTOs in `crates/knowledge`.
- Rank work by capability status and known strategic priority.
- Generate stable change IDs from capability IDs.
- Include default gates: OpenSpec validation, artifact regeneration,
  `just ci`, `make ci`, all-features tests, strict docs, audit, and affected
  CLI smoke tests.
- Treat anchors as adopt-first inputs, not as completed integrations.

## Rollback

- Remove `knowledge integration-plan` and DTOs.
- Delete `.idd/knowledge/integration-plan.json` and `.md`.
- Remove integration-plan checks from Justfile and Makefile.
- Regenerate knowledge, operating-model, plan-context, and manifest artifacts.
