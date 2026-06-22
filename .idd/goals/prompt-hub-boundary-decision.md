# Prompt Hub Boundary Decision Goal

rusty-idd --goal-file .idd/goals/prompt-hub-boundary-decision.md

Decide the integration boundary between local `prompt_hub` and Rusty IDD after
deep source research of `/home/drdave/Desktop/meta/prompt_hub`.

The core question is whether Rusty IDD should consume `prompt_hub`, or whether
`prompt_hub` should consume Rusty IDD.

## Research Inputs

- PromptHub local repo: `/home/drdave/Desktop/meta/prompt_hub`
- Rusty IDD worktree: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/prompt_hub`
- Existing Rusty IDD system graph classification of `prompt_hub` as a
  front-door/spec-producer capability.
- PromptHub source crates:
  - `prompt-hub`: core library for prompts, templates, RBAC, audit, search,
    swarm handoff, and vibe-coding flow.
  - `prompthub`: CLI for prompt management and intent-like commands.
  - `prompthub-server`: HTTP API around the core hub.

## Decision Target

Rusty IDD SHALL consume `prompt_hub` as a front-door/spec-producer through a
durable goal artifact contract.

`prompt_hub` SHOULD NOT embed, import, or own Rusty IDD's OpenSpec lifecycle,
task gating, ADR archive, generated knowledge artifacts, or validation rules.
Its role is to transform a user request into a high-quality goal, prompt bundle,
handoff packet, or goal-file candidate that Rusty IDD can then ingest.

The preferred execution contract is:

1. `prompt_hub` stores, renders, audits, and improves the user intent/prompt.
2. `prompt_hub` emits a goal artifact suitable for `rusty-idd --goal-file`
   style intake, with enough metadata for provenance and rollback.
3. Rusty IDD consumes that goal file through its graph-backed planning,
   OpenSpec, ADR, task, implementation, validation, manifest, and PR evidence
   workflow.

## Required Artifacts

- Goal-file-backed plan context.
- OpenSpec proposal, design, task list, and spec delta for the boundary.
- ADR recording the ownership decision.
- AI_MERGE research note with source evidence from prompt_hub and Rusty IDD.
- Refreshed `.idd/knowledge/*`, architecture diagrams, and `.idd/MANIFEST.tsv`.
- Validation evidence proving the artifacts were regenerated and checked.

## Non-Goals

- Do not vendor `prompt_hub` into Rusty IDD in this slice.
- Do not make `prompt_hub` depend on Rusty IDD crates in this slice.
- Do not start prompt_hub server, daemons, MCP servers, or host services.
- Do not mutate the existing dirty state in `/home/drdave/Desktop/meta/prompt_hub`.
