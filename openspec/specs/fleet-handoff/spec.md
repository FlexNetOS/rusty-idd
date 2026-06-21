# Fleet Handoff

## Purpose
Define the Rusty IDD automation contract for mapping integration work items to
fleet owner repo surfaces before peer-repo implementation begins.
## Requirements
### Requirement: Integrate Central and fleet handoff
Rusty IDD SHALL integrate capability `capability:fleet-handoff` through the documented owner repos while preserving adopt-first evidence and deterministic validation.

#### Scenario: Adopt-first evidence is recorded
- **GIVEN** the selected integration work item lists owner repos, anchors, and adopt-first inputs
- **WHEN** implementation begins
- **THEN** Rusty IDD SHALL generate deterministic owner-surface artifacts before any consolidation cut.
- **AND** the artifacts SHALL record current owner repo state, evidence paths, and native diagnostic command candidates.

#### Scenario: Thin Rusty IDD boundary is implemented
- **GIVEN** upstream or owner repo behavior is proven through diagnostics
- **WHEN** Rusty IDD wires the capability
- **THEN** the local boundary SHALL be limited to DTO mapping, deterministic output, feature flags, validation, size/token policy, and CLI/API calls.

#### Scenario: Owner-surface automation is read-only
- **GIVEN** an integration work item targets multiple peer repos
- **WHEN** Rusty IDD joins the work item to the system architecture graph
- **THEN** it SHALL NOT mutate peer repos, start host services, start daemons, or require MCP/server workflows.

#### Scenario: Validation gates protect the integration
- **GIVEN** the implementation and generated artifacts are complete
- **WHEN** the integration is proposed for merge
- **THEN** focused tests, affected smoke tests, Rusty IDD validation, and full gates SHALL pass or the change SHALL remain unmerged.

