---
name: create-c4-diagram
description: Create an ASCII or Mermaid C4-style diagram from code or design context.
skills:
  - c4-diagrams
---

# create-c4-diagram

Use this command when a user asks to create, draw, update, or explain a C4
diagram.

## Load

1. Load the portable skill wrapper at
   `intent-driven-template/.agent/skills/c4-diagrams/SKILL.md`.
2. Load the canonical skill at
   `intent-driven-template/.agents/skills/c4-diagrams/SKILL.md`.
3. Use templates from
   `intent-driven-template/.agents/skills/c4-diagrams/templates.md`.

## Inputs

- Purpose: existing code, new system, or design review.
- Format: ASCII or Mermaid.
- Rigor: strict C4, lightweight C4-inspired, or hybrid.

If the user already provided these choices, honor them. If choices are missing
and the agent has permission to proceed autonomously, use Mermaid and
lightweight C4-inspired rigor, then list the assumptions.

## Steps

1. Inspect enough context to identify actors, system boundary, containers,
   external systems, and uncertain areas.
2. Choose the smallest useful C4 level set.
3. Produce the diagram in the requested format using plain Mermaid `flowchart`
   or `sequenceDiagram` syntax when Mermaid is selected.
4. Include 3-6 concise notes covering boundaries, responsibilities,
   assumptions, and open questions.

## Output

Return the diagram first, followed by notes. Do not invent deployment details;
mark them as assumptions or open questions.
