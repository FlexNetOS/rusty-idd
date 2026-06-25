# 0010. Task-scoped agent harness packages

- Status: accepted
- Date: 2026-06-22

## Context

Rusty IDD has accumulated agent-facing surfaces in `.codex`, `.claude`,
`.agents`, upstream harness mirrors, hooks, skills, and model-loop definitions.
MCP servers can expose additional capability, but they do not solve the core
problem: agents still face an oversized tool universe and spend context on
selection rather than the current workflow stage.

The owner clarified that the desired model is stage-scoped. After a goal is
created, Rusty IDD should route work to a Rust agent swarm for the exact stage,
starting with scan. That swarm needs a bounded package: target, roles,
contracts, helpers, tools, hooks, validation gates, and evidence schema for the
stage.

## Decision

Rusty IDD owns task-scoped agent harness packages. The always-on agent harness
is minimal and provides a general package-creation capability. Agent-specific
directories such as `.codex`, `.claude`, `.kimi`, and `.agents` are adapters or
compatibility views. They must invoke Rusty IDD package generation instead of
growing into broad always-loaded toolboxes.

The first package slice is the `scan` stage. It is Rust-owned, deterministic,
and typed so it can be tested, validated, and later executed by a Rust swarm
runner.

MCP is not the default solution for tool overflow. A package may use external
servers only when a package-specific contract and feature gate justify them.

## Consequences

- Harness growth moves into Rusty IDD workflow packages, not ad hoc skill files.
- `.codex` remains a thin adapter to Rusty IDD package generation.
- Future stage packages can add implementation, validation, and handoff swarms
  without increasing always-loaded context.
- Validation can assert package scope and evidence contracts directly.
