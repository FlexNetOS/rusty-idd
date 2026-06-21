## ADDED Requirements

### Requirement: Scaffold integration work from plan
Rusty IDD SHALL create OpenSpec lifecycle artifacts from a selected integration
automation work item.

#### Scenario: Default work item is selected
- **GIVEN** `.idd/knowledge/integration-plan.json` contains ordered work items
- **WHEN** `rusty-idd spec plan-integration --base .` runs without a selector
- **THEN** Rusty IDD SHALL select the lowest-priority work item and create an
  OpenSpec change directory for that work item's `change_id`.

#### Scenario: Work item is selected explicitly
- **GIVEN** an integration automation plan contains a work item with a
  capability, work-item id, and change id
- **WHEN** the command is run with a matching selector
- **THEN** Rusty IDD SHALL generate artifacts for that exact work item.

#### Scenario: Generated artifacts preserve evidence
- **GIVEN** the selected work item contains owner repos, anchors,
  adopt-first inputs, validation gates, and rollback steps
- **WHEN** OpenSpec artifacts are generated
- **THEN** the proposal, design, tasks, and spec delta SHALL preserve that
  evidence in deterministic Markdown.

#### Scenario: Existing artifacts are protected
- **GIVEN** the target OpenSpec change file already exists
- **WHEN** the command runs without `--force`
- **THEN** Rusty IDD SHALL fail without overwriting the existing file.
