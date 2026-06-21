## ADDED Requirements

### Requirement: Report integration execution status
Rusty IDD SHALL report execution status for integration automation work items
by joining the integration plan to OpenSpec change state.

#### Scenario: Planned work item has no change
- **GIVEN** an integration work item has no matching OpenSpec change directory
  and no matching archived change
- **WHEN** `rusty-idd knowledge integration-status` runs
- **THEN** Rusty IDD SHALL report the item as `planned`.

#### Scenario: Scaffolded work item has generated artifacts
- **GIVEN** an OpenSpec change directory contains `proposal.md`, `design.md`,
  `tasks.md`, and at least one `specs/**/spec.md`
- **WHEN** integration status is generated
- **THEN** Rusty IDD SHALL report the item as `scaffolded` unless all task
  checkboxes are complete.

#### Scenario: Completed task list is ready to archive
- **GIVEN** a scaffolded OpenSpec change has no unchecked task markers in
  `tasks.md`
- **WHEN** integration status is generated
- **THEN** Rusty IDD SHALL report the item as `ready-to-archive`.

#### Scenario: Archived work item is detected
- **GIVEN** `openspec/changes/archive/<change_id>` exists for a work item
- **WHEN** integration status is generated
- **THEN** Rusty IDD SHALL report the item as `archived`.

#### Scenario: Next work item is deterministic
- **GIVEN** multiple planned integration work items exist
- **WHEN** integration status is generated
- **THEN** Rusty IDD SHALL identify the lowest-priority planned work item as
  the next work item.
