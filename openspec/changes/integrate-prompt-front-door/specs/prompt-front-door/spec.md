## ADDED Requirements

### Requirement: Integrate Prompt front door
Rusty IDD SHALL integrate capability `capability:prompt-front-door` through the documented owner repos while preserving adopt-first evidence and deterministic validation.

#### Scenario: Adopt-first evidence is recorded
- **GIVEN** the selected integration work item lists owner repos, anchors, and adopt-first inputs
- **WHEN** implementation begins
- **THEN** native diagnostics and current owner/upstream evidence SHALL be recorded before any consolidation cut.

#### Scenario: Thin Rusty IDD boundary is implemented
- **GIVEN** upstream or owner repo behavior is proven through diagnostics
- **WHEN** Rusty IDD wires the capability
- **THEN** the local boundary SHALL be limited to DTO mapping, deterministic output, feature flags, validation, size/token policy, and CLI/API calls.

#### Scenario: Validation gates protect the integration
- **GIVEN** the implementation and generated artifacts are complete
- **WHEN** the integration is proposed for merge
- **THEN** focused tests, affected smoke tests, Rusty IDD validation, and full gates SHALL pass or the change SHALL remain unmerged.
