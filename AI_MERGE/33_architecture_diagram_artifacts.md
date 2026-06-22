# Architecture Diagram Artifacts

Branch: `feature/architecture-artifacts-upgrade`
OpenSpec change: `openspec/changes/add-architecture-diagram-artifacts`
Task: `KBTASK-RUSTY-IDD-ARCHITECTURE-ARTIFACTS-UPGRADE`

## Goal

Create architecture diagrams for Rusty IDD, regenerate deterministic artifacts
against the current codebase, audit gaps, and upgrade the artifact workflow.

## Generated Artifacts

The current-code artifact pass regenerated:

- `.idd/knowledge/index.json`
- `.idd/knowledge/report.md`
- `.idd/knowledge/architecture.json`
- `.idd/knowledge/architecture.md`
- `.idd/knowledge/system-architecture.json`
- `.idd/knowledge/system-architecture.md`
- `.idd/knowledge/operating-model.json`
- `.idd/knowledge/operating-model.md`
- `.idd/knowledge/integration-plan.json`
- `.idd/knowledge/integration-plan.md`
- `.idd/knowledge/integration-status.json`
- `.idd/knowledge/integration-status.md`
- `.idd/knowledge/integration-owners.json`
- `.idd/knowledge/integration-owners.md`
- `.idd/knowledge/integration-readiness.json`
- `.idd/knowledge/integration-readiness.md`
- `.idd/knowledge/plan-context.json`
- `.idd/knowledge/plan-context.md`
- `.idd/MANIFEST.tsv`
- `AI_MERGE/validation_report.md`
- `docs/rusty-idd/architecture-diagrams.md`

## Gap Audit

| Gap | Evidence | Upgrade |
|---|---|---|
| Architecture diagrams were hand-maintained docs, not deterministic artifacts | `docs/rusty-idd/architecture-diagrams.md` existed, while `Justfile` had no diagram generator or freshness check | Added `rusty-idd knowledge diagrams`, `just diagrams`, and `just diagrams-check`; `just ci` now fails on stale diagrams |
| Agents lacked a compact current-code visual entrypoint | Existing `.idd/knowledge/architecture.*` was graph/table oriented and the diagram document could drift | Replaced the diagram doc with generated Mermaid workflow, crate-boundary, and artifact-flow views |
| Generated artifact list did not include diagrams | Knowledge skill listed `.idd/knowledge/*` artifacts but not the docs diagram surface | Updated `.agents/skills/rusty-idd-knowledge/SKILL.md` to treat diagrams as generated artifacts |
| Wider system integration gaps remain queued | `.idd/knowledge/integration-status.md` reports planned items such as `integrate-rtk-ai-foundation`, `integrate-github-agent-run-upgrades`, and `integrate-vector-runtime` | Recorded as future integration backlog; not widened into this diagram artifact slice |

## Rollback

Revert the `knowledge diagrams` CLI/API, `Justfile` recipes, generated
`docs/rusty-idd/architecture-diagrams.md`, OpenSpec/ADR/evidence additions, and
regenerate `.idd/knowledge/*` plus `.idd/MANIFEST.tsv`.
