# add-system-operating-model-graph - Design

## Context

The parent meta system graph currently discovers 65 repos and assigns roles such
as `fleet-handoff`, `agent-environment`, `toolchain-provider`, and
`rust-code-surface`. That is useful for broad integration planning, but the
user's target system is an agentic company operating system with explicit
responsibility layers and capability owners.

The operating-model graph is the next bounded layer above the system graph. It
does not replace repo architecture or system architecture. It consumes them and
adds a deterministic capability taxonomy.

## Goals / Non-Goals

**Goals:**

- Encode the agentic company operating model as generated Rusty IDD knowledge.
- Map discovered repos to layers and capabilities.
- Keep unmapped/external requirements explicit.
- Preserve read-only behavior across peer repos.
- Make the artifact deterministic and checkable in CI.

**Non-Goals:**

- Mutating peer repos.
- Starting services, MCP servers, daemons, model runtimes, or vault agents.
- Claiming Cognitum, upstream prompt repos, Lua/AR, or distributed device
  surfaces are integrated before repo evidence exists.
- Building the user-facing UI or runtime scheduler in this change.

## Decisions

- Add a new `SystemOperatingModel` DTO under `crates/knowledge`.
- Use a static built-in capability taxonomy derived from the current system
  vision and repo names/roles/tags.
- Treat Yazelix as the default terminal/runtime surface for nushell, Lua,
  Ghostty, Zellij, and contributor interactions.
- Treat RTK, ICM, VOX, GRIT, and Beads as foundational RTK AI/GitHub agent-run
  surfaces. Beads candidates are pinned as external anchors until a canonical
  repo dependency is selected.
- Read the system graph from `.idd/knowledge/system-architecture.json` by
  default.
- Let graph planning context read `.idd/knowledge/operating-model.json` by
  default and carry selected layers/capabilities into the planning packet.
- Render both JSON and Markdown.
- Treat missing expected repo anchors as findings.
- Keep `crates/core` unchanged and std-only.

## Rollback

- Remove the `operating-model` CLI command and DTOs.
- Delete `.idd/knowledge/operating-model.json` and `.md`.
- Remove `operating-model` and `operating-model-check` from the Justfile.
- Re-run knowledge, system-architecture, plan-context, and manifest generation.
