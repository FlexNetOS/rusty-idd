### Requirement: Harness package catalog supports self-upgrade stages
Rusty IDD SHALL treat self-upgrade stages as task-scoped package selections
rather than permanent additions to always-loaded harness directories.

#### Scenario: Self-upgrade stage selects a package
- **GIVEN** a self-upgrade goal enters scan, goal, design, implement, verify,
  publish, or learn
- **WHEN** the harness asks for the required capability
- **THEN** Rusty IDD SHALL select or create the bounded package for that exact
  stage, target, contracts, tools, helpers, hooks, validation gates, evidence
  schema, and agent roles.

#### Scenario: Adapter remains thin
- **GIVEN** Codex or another model surface invokes the self-upgrade workflow
- **WHEN** it needs stage-specific behavior
- **THEN** the adapter SHALL delegate package selection to Rusty IDD rather than
  embedding the full workflow in `.codex`, `.claude`, `.kimi`, or another
  always-loaded directory.
