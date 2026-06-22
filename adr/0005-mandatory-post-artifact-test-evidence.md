# ADR 0005: Require Post-Artifact Test Evidence Before Completion and Push

Status: accepted
Date: 2026-06-22

## Context

Rusty IDD agents generate and refresh knowledge, diagram, manifest, OpenSpec,
ADR, task, and evidence artifacts as part of the implementation workflow. A
task can look complete if those artifacts exist, even when tests have not run
after the generated state changed.

The owner goal requires a comprehensive end-to-end test suite for the entire
Rusty IDD code base and workflow, with tests after artifact creation, and tests
mandatory before every task complete and push.

## Decision

Rusty IDD workflow checks will require validation evidence that explicitly lists
generated artifact refresh before test evidence. Commands that represent push or
task-completion handoff, including `git push`, PR handoff commands, `hf done`,
and equivalent task-completion commands, must have validation evidence before
the command proceeds.

Stop-phase delivery checks will continue to require validation evidence and PR
or auto-merge evidence when local work requires delivery.

## Consequences

- Agents cannot claim workflow completion from generated artifacts alone.
- The evidence file must preserve the ordering: generated artifacts first,
  tests second.
- Push and task-completion commands fail early when validation evidence is
  missing.
- Existing build, lint, audit, manifest, knowledge, diagram, and validation
  gates remain in force.
