## ADDED Requirements

### Requirement: Harness follows Rusty IDD flow
The Codex harness SHALL treat Rusty IDD as the intent-driven workflow engine and SHALL run graph-backed planning and OpenSpec artifact checks before any implementation pass.

#### Scenario: Intent is captured before implementation
- **GIVEN** a user provides a goal for Rusty IDD work
- **WHEN** the Codex harness prepares agent context
- **THEN** it SHALL use Rusty IDD knowledge planning artifacts to capture the goal before any write-capable implementation pass.

#### Scenario: OpenSpec artifacts gate writes
- **GIVEN** a proposed change has not produced the required OpenSpec artifacts
- **WHEN** the harness reaches the implementation phase
- **THEN** write-capable agents SHALL refuse to implement until proposal, specs, design, ADR status, and tasks are ready according to `rusty-idd spec status`.

#### Scenario: AI_MERGE is evidence, not intent
- **GIVEN** AI_MERGE records exist for audit, migration, or merge evidence
- **WHEN** the harness reads repository context
- **THEN** it SHALL treat `AI_MERGE/` as a Rusty IDD tool/evidence surface and SHALL NOT present it as the main intent source or authoritative Rusty IDD control plane.

#### Scenario: Default loop stops before writes
- **GIVEN** the repo-local model loop runs without explicit implementation authorization
- **WHEN** it emits or executes its default passes
- **THEN** the default passes SHALL remain read-only and SHALL stop at design, artifact status, and verification outputs.
