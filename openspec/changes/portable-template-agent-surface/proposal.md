# portable-template-agent-surface

## Why

The intent-driven template already includes reusable skills under
`intent-driven-template/.agents/skills` and OpenCode commands under
`intent-driven-template/.opencode/commands`, but the portable agent entrypoint
named by the goal, `intent-driven-template/.agent`, does not exist. Agents that
look for a generic `.agent` surface cannot discover the template skills or run
commands such as creating C4 diagrams without knowing OpenCode-specific paths.

This change makes the template usable by any file-reading agent while preserving
the existing OpenCode and `.agents` surfaces as the canonical implementation.

## What Changes

- Add an agent-agnostic `intent-driven-template/.agent` entrypoint with a
  manifest, README, command wrappers, and skill wrappers.
- Add a portable `create-c4-diagram` command that explicitly loads the existing
  C4 skill and templates.
- Mirror the existing OPSX commands through portable `.agent/commands` wrappers
  so agents that do not understand OpenCode command directories can still use
  the lifecycle commands.
- Teach Rusty IDD inventory scanning to classify `.agent`, `.agents`, and
  `.opencode` command/skill surfaces as agent-control files.
- Add focused tests proving the portable C4 command, skill wrapper, canonical C4
  skill, and templates exist and are connected.

## Capabilities

### New Capabilities
- `template-agent-surface`: Portable template agent entrypoint, commands, and
  skill loading for any agent.

### Modified Capabilities
- `agent-control-inventory`: Rusty IDD classifies portable agent template
  surfaces as agent-control evidence.

## Impact

- Affected template files:
  - `intent-driven-template/.agent/**`
  - `intent-driven-template/README.md`
  - `intent-driven-template/INSTALL_TEMPLATE.md`
- Affected Rust code:
  - `crates/core/src/scanner.rs`
  - `crates/core/tests/template_agent_surface.rs`
- No new dependencies, secrets, host services, or runtime daemons.
