# ADR-0007: PromptHub Feeds Rusty IDD Through Goal Artifacts

## Status

Accepted

## Context

PromptHub is a local Rust workspace at `/home/drdave/Desktop/meta/prompt_hub`.
It provides a prompt management core library, CLI, and HTTP API for LLM agent
swarms. It includes templates, search, RBAC, audit logging, swarm handoff
helpers, and vibe-coding intent flow.

Rusty IDD owns intent-driven development workflow state: graph-backed context,
OpenSpec proposal/design/spec/tasks, ADRs, implementation gates, validation,
manifest refresh, and merge evidence.

The local Rusty IDD system graph already classifies PromptHub as a
`spec-producer`, and PromptHub's own continuity capsule identifies its plane as
`front-door`.

## Decision

Rusty IDD consumes PromptHub outputs. PromptHub does not consume or own Rusty
IDD lifecycle internals.

The integration boundary is a durable goal artifact contract:

1. PromptHub captures, stores, searches, renders, audits, and improves user
   prompts or intents.
2. PromptHub emits a goal file or equivalent prompt artifact with provenance.
3. Rusty IDD consumes the artifact through goal-file planning and owns the
   downstream OpenSpec, ADR, task, validation, manifest, and PR evidence flow.

Future PromptHub conveniences may invoke Rusty IDD as an external CLI, but they
must not embed Rusty IDD crates or write `.idd`/OpenSpec lifecycle state as the
authority.

## Consequences

- Rusty IDD stays the workflow control plane.
- PromptHub stays the user-facing front door and prompt product surface.
- The first integration can be file-contract based and deterministic.
- Future API integration remains possible after the file contract is validated.
- PromptHub source and docs can evolve independently without forcing Rusty IDD
  to vendor PromptHub.

## Validation

- PromptHub native diagnostic: `rtk cargo check --workspace`.
- Rusty IDD plan context generated from
  `.idd/goals/prompt-hub-boundary-decision.md`.
- Rusty IDD knowledge, architecture, diagram, system, integration, manifest, and
  validation artifacts refreshed under the active OpenSpec change.
