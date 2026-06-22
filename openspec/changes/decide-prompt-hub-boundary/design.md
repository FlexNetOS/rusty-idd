# decide-prompt-hub-boundary - Design

## Context

PromptHub is a local Rust workspace at `/home/drdave/Desktop/meta/prompt_hub`.
It contains a prompt management library, CLI, and HTTP API. Its own continuity
capsule identifies the repo plane as `front-door` and its north star as:

> A non-technical user makes any request; prompt_hub transforms, communicates,
> and delivers it as intended.

Rusty IDD is the intent-driven workflow engine. It owns goal-file planning,
knowledge graphs, OpenSpec changes, ADRs, task cards, validation, manifest
refresh, and PR evidence.

## Goals / Non-Goals

**Goals:**

- Decide the direction of consumption between PromptHub and Rusty IDD.
- Keep PromptHub's current prompt/product surface intact.
- Keep Rusty IDD's lifecycle authority intact.
- Define a durable file/CLI artifact contract for goal intake.
- Produce enough evidence for future implementation without re-researching the
  same boundary.

**Non-Goals:**

- Do not vendor PromptHub into Rusty IDD.
- Do not make PromptHub link Rusty IDD crates.
- Do not start the PromptHub server or host services.
- Do not modify PromptHub's dirty local state.

## Research Summary

PromptHub evidence:

- Workspace members: `prompt-hub`, `prompthub`, `prompthub-server`.
- Core surfaces: prompt models, libsql storage, RBAC, audit logging, search,
  templates, swarm handoff, quality gates, fallback, and vibe coding.
- CLI surfaces: `init`, `add`, `get`, `search`, `gather`, `vibe`, `preview`,
  `deploy`, `feedback`, `budget`, `quota`, and related prompt operations.
- Built-in templates: orchestrator, architect, implementer, critic, reviewer,
  and handoff templates.
- Native diagnostic run: `rtk cargo check --workspace` completed successfully.
- Existing dirty state in PromptHub was not modified:
  - deleted `.output.txt`
  - untracked `worktrees/`

Rusty IDD evidence:

- Generated system graph already classifies PromptHub as:
  - `role:agent-environment`
  - `role:capability-hub`
  - `role:spec-producer`
- Rusty IDD maps PromptHub through the edge:
  - `repo:rusty-idd consumes_spec_intent_from role:spec-producer`
- Existing `integrate-prompt-front-door` is complete and archivable, but it does
  not answer this exact ownership question.

## Decision

Rusty IDD SHALL consume PromptHub outputs through a durable goal-artifact
contract.

PromptHub SHOULD produce, store, render, audit, and improve prompt/intention
artifacts. Rusty IDD SHOULD consume those artifacts through `--goal-file`
planning and then own the downstream lifecycle:

1. knowledge graph refresh,
2. plan context,
3. OpenSpec proposal/design/spec/tasks,
4. ADR,
5. implementation,
6. validation,
7. manifest refresh,
8. PR and merge evidence.

PromptHub SHOULD NOT consume Rusty IDD as a crate or own `.idd`, OpenSpec, ADR,
manifest, or validation state. If PromptHub needs to launch Rusty IDD in a
future slice, it should do so as an external CLI invocation that writes or
passes a goal artifact, not by embedding Rusty IDD's lifecycle internals.

## Contract

The first stable contract is file based:

- Input owner: PromptHub.
- Artifact: Markdown goal file with front matter or explicit metadata.
- Minimum fields:
  - title,
  - source prompt id or provenance,
  - user intent,
  - rendered goal,
  - constraints,
  - acceptance criteria,
  - rollback or cancellation notes,
  - PromptHub audit reference when available.
- Consumer: Rusty IDD `knowledge plan-context --goal-file` and follow-on
  OpenSpec workflow.

## Risks / Trade-offs

- File contract is less rich than an API integration, but it is deterministic,
  auditable, easy to diff, and compatible with current Rusty IDD planning.
- A future PromptHub CLI wrapper around Rusty IDD may be useful, but it must not
  obscure Rusty IDD's ownership of generated workflow state.
- PromptHub docs are partly stale; source, Cargo metadata, handoff capsule, and
  native diagnostics are stronger evidence.

## Validation Plan

1. Generate plan context from the new goal file.
2. Refresh Rusty IDD knowledge, diagrams, system architecture, operating model,
   integration plan/status/owners/readiness, and manifest.
3. Validate OpenSpec and Rusty IDD artifacts.
4. Run full Rusty IDD `just ci`.
5. Record validation evidence.
