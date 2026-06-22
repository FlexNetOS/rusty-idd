# 0002. Portable template agent surface

- Status: accepted
- Date: 2026-06-21

## Context

The intent-driven template includes canonical OpenCode command files and
canonical reusable skill files, but the goal requires a portable
`intent-driven-template/.agent` surface that can be loaded by any agent. Moving
or rewriting the existing files would risk breaking established OpenCode and
`.agents` consumers.

## Decision

Rusty IDD will keep `.agents/skills` and `.opencode/commands` as canonical
template sources and add `intent-driven-template/.agent` as a thin portable
front door. The portable layer will contain a manifest, command wrappers, and
skill wrappers that point agents to the canonical files. C4 diagram creation is
added as a first-class portable command because it is a named user workflow and
does not currently have an OpenCode command file.

## Consequences

Generic agents can discover skills and commands from one stable directory
without needing OpenCode conventions. Existing OpenCode and `.agents` behavior
is preserved. The trade-off is that wrapper files must stay aligned with
canonical files; focused tests cover the high-value C4 path and command/skill
references.
