# Agent Communication

## Purpose
Define the Rusty IDD automation contract for keeping owner-surface evidence
aligned with the active agent communication integration queue item.
## Requirements
### Requirement: Integrate Agent communication layer
Rusty IDD SHALL integrate capability `capability:agent-communication` through the documented owner repos while preserving adopt-first evidence and deterministic validation.

#### Scenario: Adopt-first evidence is recorded
- **GIVEN** the selected integration work item lists owner repos, anchors, and adopt-first inputs
- **WHEN** implementation begins
- **THEN** Rusty IDD SHALL generate deterministic owner-surface artifacts for the current integration queue head before any consolidation cut.
- **AND** the artifacts SHALL record current owner repo state, evidence paths, and native diagnostic command candidates.

#### Scenario: Owner-surface selection follows active queue head
- **GIVEN** archived or active OpenSpec integration changes exist
- **WHEN** `rusty-idd knowledge integration-owners --next` runs
- **THEN** Rusty IDD SHALL select the highest-priority integration work item whose OpenSpec state is not archived.

#### Scenario: Planned-only owner-surface selection skips active work
- **GIVEN** active or scaffolded OpenSpec integration changes exist
- **WHEN** `rusty-idd knowledge integration-owners --next-planned` runs
- **THEN** Rusty IDD SHALL select the highest-priority integration work item whose OpenSpec state is still planned.

#### Scenario: Thin Rusty IDD boundary is implemented
- **GIVEN** upstream or owner repo behavior is proven through diagnostics
- **WHEN** Rusty IDD wires the capability
- **THEN** the local boundary SHALL be limited to DTO mapping, deterministic output, feature flags, validation, size/token policy, and CLI/API calls.

#### Scenario: Validation gates protect the integration
- **GIVEN** the implementation and generated artifacts are complete
- **WHEN** the integration is proposed for merge
- **THEN** focused tests, affected smoke tests, Rusty IDD validation, and full gates SHALL pass or the change SHALL remain unmerged.
