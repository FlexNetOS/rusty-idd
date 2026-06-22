## ADDED Requirements

### Requirement: Workflow stages create scoped harness packages
Rusty IDD SHALL create or select a task-scoped harness package for each
workflow stage instead of requiring agents to load a broad, always-visible tool
surface.

#### Scenario: Goal advances to scan package
- **GIVEN** a user goal has been created
- **WHEN** Rusty IDD routes the goal into the scan stage for a target path
- **THEN** it SHALL produce a scan-stage harness package containing only the
  scan target, scan contracts, scan agent roles, scan helpers, scan hooks, scan
  tools, validation gates, and scan evidence schema.

#### Scenario: Package replaces ad hoc skill creation
- **GIVEN** an agent needs task-specific implementation support
- **WHEN** the support maps to a known Rusty IDD workflow stage
- **THEN** the always-on adapter SHALL invoke Rusty IDD package generation
  rather than asking the agent to create another repo-local skill by hand.

### Requirement: Agent directories remain minimal adapters
Rusty IDD SHALL treat `.codex`, `.claude`, `.kimi`, `.agents`, and equivalent
agent directories as runtime adapters or compatibility views, not the
authoritative source of workflow capabilities.

#### Scenario: Codex adapter needs a package
- **GIVEN** Codex is operating in a repository with Rusty IDD support
- **WHEN** Codex reaches a workflow stage that needs task-specific tools
- **THEN** the Codex adapter SHALL call the Rusty IDD harness package surface
  and load only that package's bounded context.

#### Scenario: Default package avoids MCP sprawl
- **GIVEN** the scan-stage package is generated
- **WHEN** the package declares its tools
- **THEN** MCP servers SHALL NOT appear in the default tool list unless a
  package-specific feature gate and contract explain why they are required.

### Requirement: Harness packages hand off typed evidence
Each Rusty IDD harness package SHALL declare the evidence schema required to
handoff from its workflow stage to the next stage.

#### Scenario: Scan evidence is declared
- **GIVEN** the scan-stage harness package is generated
- **WHEN** an agent reads its evidence schema
- **THEN** it SHALL include inventory, graph/context, risk, validation, and
  next-stage recommendation outputs.
