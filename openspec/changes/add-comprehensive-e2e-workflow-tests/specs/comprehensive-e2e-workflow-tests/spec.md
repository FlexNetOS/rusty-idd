## ADDED Requirements

### Requirement: Comprehensive E2E workflow tests
Rusty IDD SHALL provide end-to-end tests that cover goal-file intake, generated
artifact validation, task completion evidence, and PR push handoff evidence.

#### Scenario: Goal-file workflow creates validation context
- **GIVEN** a tracked goal file describes a Rusty IDD workflow goal
- **WHEN** `rusty-idd knowledge plan-context --goal-file` is run
- **THEN** the generated planning context SHALL preserve the goal text and
  bind it to the active OpenSpec change.

#### Scenario: Tests run after generated artifacts
- **GIVEN** generated artifacts have been refreshed for a Rusty IDD task
- **WHEN** the task is being completed
- **THEN** a successful test command result SHALL be required after the
  generated artifact refresh.

#### Scenario: Push handoff requires tests
- **GIVEN** an autonomous branch is ready to push for PR handoff
- **WHEN** the workflow handoff evidence is checked
- **THEN** a successful test command result SHALL be required before push or PR
  handoff is considered ready.

#### Scenario: Missing test evidence blocks completion
- **GIVEN** build, validation, manifest, and diagram evidence exists
- **WHEN** no successful test evidence exists after generated artifact refresh
- **THEN** Rusty IDD SHALL report the workflow as incomplete rather than ready.
