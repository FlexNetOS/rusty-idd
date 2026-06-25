# add-self-upgrade-governor - Design

## Context

Rusty IDD's repo-local workflow already establishes intent, knowledge,
OpenSpec, ADR, task, validation, and evidence order. The current harness model
now points toward task-scoped packages, but the next-goal decision still happens
outside Rusty IDD.

This design promotes the approved brainstorm into a Rusty IDD-owned governor
workflow. The governor does not make the model the source of truth. It turns
repo evidence into typed candidate goals, applies policy, and routes accepted
goals into existing Rusty IDD lifecycle artifacts.

## Goals

- Keep `.codex`, `.claude`, `.kimi`, and similar harness directories minimal.
- Let Rusty IDD generate and review candidate goals from repo evidence.
- Keep endless automation read-only until a bounded delivery loop is selected.
- Ensure every write-capable cycle has one goal, one worktree, one active
  OpenSpec change, one PR, and explicit verification evidence.
- Make package selection the Rusty IDD-owned answer to "what tools/skills/hooks
  should this task load?"

## Non-Goals

- Do not implement the full self-upgrade CLI in this artifact pass.
- Do not research or implement the downstream integration target in this change.
- Do not create an unbounded write loop.
- Do not add host services, user-global installs, MCP sprawl, or daemon
  lifecycle management.

## Decisions

### Two-Loop Automation

Self-upgrade automation is split into:

- discovery loop: endless, read-only, produces ranked candidate goals;
- delivery loop: finite, write-capable, handles one approved goal through PR
  completion or blocked handoff.

### Typed Goal Pipeline

The governor represents self-authored work as:

```text
Finding
  -> Opportunity
  -> Hypothesis
  -> CandidateGoal
  -> GoalReview
  -> ApprovedGoal
  -> OpenSpecChange
  -> Package
```

This keeps model-authored goals reviewable and prevents arbitrary free-form
execution.

### Package Sequence

The first package catalog sequence is:

1. `scan`
2. `goal`
3. `design`
4. `implement`
5. `verify`
6. `publish`
7. `learn`

Each package owns the contracts, tools, helpers, hooks, validation gates,
evidence schema, and roles for its exact workflow slice.

### Safety Policy

Low-risk auto goals may include generated artifact refreshes, docs/spec
consistency repairs, missing evidence repairs, narrow package scaffolding, and
test fixture repairs.

High-risk goals require owner approval when they change dependencies,
architecture boundaries, toolchains, auth/secrets/env behavior, deletion policy,
cross-repo mutation, or CI policy.

## Migration Plan

1. Land this goal, OpenSpec, ADR, task, and evidence package.
2. Implement the first `self-upgrade scan` or `self-upgrade goal` package in a
   later narrow change.
3. Use the generated candidate goal path to create the next OpenSpec change.
4. Use the first downstream test target to evaluate real Rusty IDD integrations
   with handoff and prompt_hub only after the governor artifact flow is ready.

## Risks and Trade-Offs

- Risk: "self-upgrade" becomes an uncontrolled write loop.
  Mitigation: keep discovery endless and read-only; keep delivery finite and PR
  shaped.
- Risk: candidate goals become vague.
  Mitigation: require evidence, risk, blast radius, owner boundary, and proposed
  OpenSpec slug.
- Risk: package catalog becomes another always-loaded harness.
  Mitigation: packages are selected per stage and per target, not loaded by
  default.

## Open Questions

- Should the first implementation crate be named `self-upgrade`, `governor`, or
  live under a broader `harness` package crate?
- Should candidate-goal queue state live under `.idd/self-upgrade/` or as
  generated `.idd/knowledge/*` plus `.idd/goals/*` artifacts?
- Which exact command owns the top-level `rusty-idd --goal-file` adapter if the
  current CLI keeps goal binding under subcommands?
