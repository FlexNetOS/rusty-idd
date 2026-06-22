# PromptHub Boundary Research

## Scope

Research local PromptHub at `/home/drdave/Desktop/meta/prompt_hub` and decide
whether Rusty IDD should consume PromptHub or PromptHub should consume Rusty
IDD.

## Local State

PromptHub was inspected in place. Existing local dirty state was observed and
not modified:

- `.output.txt` is deleted in the PromptHub checkout.
- `worktrees/` is untracked in the PromptHub checkout.

Rusty IDD work occurred in:

- `/home/drdave/Desktop/meta/rusty-idd/.worktrees/prompt_hub`
- branch `feature/prompt_hub`

## PromptHub Evidence

PromptHub is a Rust workspace with three members:

- `prompt-hub`: core prompt management library.
- `prompthub`: CLI binary.
- `prompthub-server`: Axum HTTP API server.

Core library surfaces found:

- prompt models and generation parameters,
- libsql storage and migrations,
- RBAC and agent identity,
- audit logging,
- prompt injection sanitization,
- FTS and hybrid search,
- template rendering,
- built-in role templates,
- swarm handoff template generation,
- quality gates,
- rollback,
- feedback/learning and vibe-coding flow.

CLI surfaces found:

- `init`, `add`, `get`, `list`, `search`, `update`, `rollback`, `audit`,
  `export`, `import`, `gather`, `vibe`, `preview`, `cost`, `deploy`,
  `feedback`, `budget`, `quota`, and related prompt operations.

Built-in templates found:

- `base_orchestrator`,
- `base_architect`,
- `base_implementer`,
- `base_critic`,
- `base_reviewer`,
- `handoff_standard`,
- `env_state_convergence`.

Continuity evidence:

- `.handoff/context/capsule.json` names PromptHub's plane as `front-door`.
- The capsule north star says a non-technical user makes any request and
  PromptHub transforms, communicates, and delivers it as intended.
- `.handoff/README.md` marks PromptHub as a fleet member with committed
  text-only continuity state.

Native diagnostic:

```bash
rtk cargo check --workspace
```

Result: passed; all three PromptHub crates compiled.

## Rusty IDD Evidence

Rusty IDD generated system architecture already classifies PromptHub as:

- `role:agent-environment`,
- `role:capability-hub`,
- `role:spec-producer`.

The system graph includes:

- `repo:rusty-idd consumes_spec_intent_from role:spec-producer`,
- `repo:rusty-idd maps_for_automation role:spec-producer`.

Existing OpenSpec change `integrate-prompt-front-door` is complete and
archivable, but it does not answer the exact ownership direction question.

## Decision

Rusty IDD should consume PromptHub outputs. PromptHub should not consume Rusty
IDD internals.

The correct boundary is:

1. PromptHub owns prompt/intention capture, rendering, search, audit, and
   front-door product behavior.
2. PromptHub emits a goal file or equivalent durable artifact with provenance.
3. Rusty IDD consumes that artifact through goal-file planning and owns
   generated context, OpenSpec, ADR, task, validation, manifest, and PR evidence.

## Rationale

This preserves both ownership boundaries:

- PromptHub remains the front door.
- Rusty IDD remains the workflow engine.

It also keeps integration thin and deterministic. A file contract can be tested,
diffed, audited, and rolled back before any richer API or CLI wrapper is added.

## Rollback

If this boundary proves wrong, revert the OpenSpec change, ADR, goal file,
task card, AI_MERGE note, and regenerated artifacts. Then rerun Rusty IDD
knowledge refresh, plan context, manifest, and validation.
