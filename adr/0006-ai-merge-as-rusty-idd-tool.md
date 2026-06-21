# 0006. AI_MERGE as Rusty IDD tool and evidence surface

- Status: accepted, supersedes ADR-0003
- Date: 2026-06-21
- Supersedes: ADR-0003

## Context

ADR-0003 designated `AI_MERGE/` and `.idd/MANIFEST.tsv` as the authoritative
control plane for integration operations. That matched the earlier merge-control
stage, but Rusty IDD has since grown into an intent-driven workflow engine with
graph-backed knowledge artifacts and an OpenSpec lifecycle.

The current workflow needs a clearer boundary:

- Rusty IDD owns intent intake, graph/context artifacts, OpenSpec proposal,
  specs, design, ADRs, tasks, implementation gating, validation, archive, and
  handoff-ready evidence.
- `AI_MERGE/` remains useful for audit notes, migration history, merge evidence,
  rollback notes, and compatibility with older merge-oriented surfaces.

## Decision

`AI_MERGE/` is no longer the authoritative Rusty IDD control plane. It is not the authoritative Rusty IDD control plane; it is a tool and evidence surface that Rusty IDD may read or write when a workflow step needs audit, migration, or merge evidence.

The authoritative workflow order for Rusty IDD is:

1. User intent or goal.
2. Graph-backed knowledge and bounded context artifacts.
3. OpenSpec proposal and capability specs.
4. Design and ADR decisions.
5. Tasks and implementation gating.
6. Validation, regenerated artifacts, and archive or handoff evidence.

## Consequences

- Codex harness prompts and invariant checks must start from Rusty IDD
  knowledge/OpenSpec artifacts rather than AI_MERGE.
- AI_MERGE records stay valuable as evidence and history, but they must not be
  presented as the main intent source.
- Older merge-oriented templates and docs may need staged upgrades so the repo
  no longer teaches agents to treat AI_MERGE as Rusty IDD itself.
