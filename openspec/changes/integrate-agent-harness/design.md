# integrate-agent-harness - Design

## Context

Rusty IDD already treats graph context, OpenSpec, validation, and evidence as
the automation spine. The agent harness still exposes many reusable surfaces at
once: skills, hooks, subagents, rules, model loops, bridge directories, and
upstream harness references. That makes the default prompt/tool surface too
large and pushes agents toward tool browsing instead of workflow execution.

The target architecture is task-scoped. Rusty IDD owns a package catalog of
workflow stages. When a goal advances to a stage, Rusty IDD creates or selects a
stage package for the target. The package defines the bounded context and
execution contract for a Rust agent swarm. Agent-specific directories are thin
launch adapters into that package.

## Goals / Non-Goals

**Goals:**

- Define a Rust-owned harness package model for task-scoped workflow stages.
- Ship the first package for the `scan` stage.
- Add a CLI command that emits the package contract for a target and stage.
- Keep `.codex`, `.claude`, `.kimi`, and `.agents` minimal by routing through
  Rusty IDD package generation.
- Make the package output typed and deterministic enough for tests, hooks, and
  future workflow routing.

**Non-Goals:**

- Replacing Codex, Claude, Kimi, or their runtime adapters.
- Standing up MCP servers as the default tool selection strategy.
- Implementing a full multi-process swarm executor in this first slice.
- Removing historical harness or upstream reference material before parity
  evidence exists.

## Decisions

- Introduce `rusty-idd harness package --stage <stage> --target <path>` as the
  first operator-facing edge. The command starts as deterministic package
  generation, not process spawning.
- Represent package content as typed Rust data rendered to JSON or Markdown.
- The first supported stage is `scan`.
- A scan package declares:
  - `target`: the repo or path being scanned;
  - `agent_team`: stage-specific roles;
  - `contracts`: inventory, secret/config, graph/context, and evidence
    contracts;
  - `tools`: Rusty IDD-native scan, knowledge, manifest, validation, and
    OpenSpec status commands;
  - `helpers`: narrowly scoped helper surfaces;
  - `hooks`: stage-gate names, not global hook sprawl;
  - `evidence_schema`: required outputs for handoff to the next stage.
- MCP is absent from the default scan package. A future package may declare a
  feature-gated external tool only when the stage contract proves why it is
  needed.

## Risks / Trade-offs

- A package command without an executor can feel modest. The benefit is that it
  creates the contract and test surface first, keeping the migration safe.
- Stage packages can become another static catalog if unchecked. Mitigation:
  package generation takes a stage and target, and tests assert bounded content.
- Adapter directories still exist. Mitigation: documentation and invariant
  checks define them as adapters, while the Rust command owns package content.

## Migration Plan

1. Create this OpenSpec package and ADR before implementation.
2. Add a Rust-owned harness package module and CLI command for scan packages.
3. Document the minimal adapter boundary for `.codex`, `.claude`, `.kimi`, and
   `.agents`.
4. Add focused tests for scan package output and bounded tool selection.
5. Refresh knowledge and manifest artifacts.
6. Validate, commit, push, open a PR to `develop`, and merge when green.

## Open Questions

- Should later slices materialize packages to `.idd/harness/packages/<stage>.json`,
  or should commands continue to emit package contracts on demand until an
  executor exists?
