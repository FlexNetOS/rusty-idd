## ADDED Requirements

### Requirement: Portable Agent Template Surface
Feature: Intent-driven template agent portability
Rule: Agents SHALL be able to discover reusable skills and runnable commands
without requiring OpenCode-specific directory knowledge.

#### Scenario: Agent discovers portable skills
- **GIVEN** a repository includes `intent-driven-template/.agent`
- **WHEN** an agent reads the portable manifest or skill directory
- **THEN** it SHALL discover the intent-driven skills by stable name
- **AND** each portable skill entry SHALL point to a loadable canonical skill
  file in `intent-driven-template/.agents/skills`.

#### Scenario: Agent runs C4 diagram command
- **GIVEN** an agent needs to create C4 diagrams from the template
- **WHEN** it reads `intent-driven-template/.agent/commands/create-c4-diagram.md`
- **THEN** the command SHALL instruct the agent to load the C4 diagram skill
- **AND** it SHALL identify the C4 templates file used to create ASCII or
  Mermaid diagrams.

#### Scenario: Agent uses existing OPSX lifecycle commands
- **GIVEN** an agent only knows to inspect `intent-driven-template/.agent`
- **WHEN** it lists portable commands
- **THEN** it SHALL find command wrappers for the existing OPSX lifecycle
  commands
- **AND** each wrapper SHALL point back to the canonical OpenCode command
  document.

### Requirement: Agent Control Inventory Classification
Feature: Rusty IDD control-plane evidence
Rule: Agent-facing template surfaces SHALL be classified as agent-control files.

#### Scenario: Scanner classifies portable agent files
- **GIVEN** a repository contains `.agent`, `.agents`, or `.opencode` agent
  command and skill files
- **WHEN** Rusty IDD scans the repository inventory
- **THEN** those files SHALL be categorized as `agent-control`
- **AND** they SHALL be included in the inventory's agent file list.
