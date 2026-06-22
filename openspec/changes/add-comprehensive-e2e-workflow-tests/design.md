# add-comprehensive-e2e-workflow-tests - Design

## Context

Rusty IDD coordinates autonomous work through goal intake, graph-backed context,
OpenSpec readiness, ADR decisions, generated tasks, validation, and PR handoff
evidence. Existing gates cover build, test, validation, manifest, knowledge,
diagrams, lint, audit, and Codex environment checks, but the task-completion and
push-handoff contract must explicitly require tests after generated artifact
creation.

## Goals / Non-Goals

**Goals:**

- Exercise the end-to-end Rusty IDD workflow through CLI-level tests.
- Prove goal-file planning works before implementation.
- Prove generated artifact validation runs before task completion.
- Prove test evidence is mandatory before task completion and before push.
- Keep the full repo CI path as the final validation target.

**Non-Goals:**

- Replace existing validation commands.
- Downgrade or skip build, lint, audit, manifest, diagram, or knowledge gates.
- Start daemons, host services, MCP servers, or user-global installs.
- Refactor unrelated CLI or knowledge internals.

## Design

The change tightens the existing Rusty IDD workflow checker and tests instead
of introducing a second validation surface.

1. Goal-file intake remains bound through `rusty-idd knowledge plan-context
   --goal-file`.
2. Generated artifacts are refreshed before the final validation pass.
3. Task completion evidence must include a successful test command result after
   generated artifact creation.
4. Push or PR handoff evidence must include the same mandatory test gate.
5. CLI tests create isolated fixture workspaces so E2E coverage does not depend
   on dirty local state.

## Validation Strategy

- Add focused unit or CLI tests for workflow-check evidence requirements.
- Run `cargo test --workspace --locked` as the required test gate.
- Run `just ci` with `RUSTY_IDD_CHANGE` and `RUSTY_IDD_GOAL_FILE` set to this
  change and goal file.
- Record command evidence in `AI_MERGE/35_e2e_test_suite/validation.md`.

## Rollback

Revert the workflow-check contract and this OpenSpec change. Existing build,
test, lint, audit, manifest, knowledge, diagram, and validation commands remain
available because this change only strengthens required evidence for task
completion and push handoff.
