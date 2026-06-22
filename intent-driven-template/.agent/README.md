# Portable Agent Entry Point

This directory is the agent-agnostic front door for the intent-driven template.
It is for agents that can read files but do not know OpenCode-specific
conventions.

Start with `manifest.json`, then load the command or skill named by the user.
Canonical skill bodies live in `../.agents/skills`. Canonical OpenCode command
instructions live in `../.opencode/commands`. Files in this directory are thin
portable wrappers so every agent has a stable place to begin.

## Common Commands

- `commands/create-c4-diagram.md`: create ASCII or Mermaid C4-style diagrams.
- `commands/opsx-new.md`: start a new OpenSpec change.
- `commands/opsx-propose.md`: create all proposal artifacts before
  implementation.
- `commands/opsx-apply.md`: implement an approved change.
- `commands/opsx-verify.md`: verify implementation against artifacts.
- `commands/opsx-archive.md`: archive a completed change.

## Skill Loading

Each `skills/*/SKILL.md` wrapper points to its canonical skill. If an agent can
follow relative paths, it should load the canonical file directly. If it cannot,
the wrapper names the required source path and the expected workflow.
