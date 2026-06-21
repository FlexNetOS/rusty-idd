## ADDED Requirements

### Requirement: Generate integration automation plan
Rusty IDD SHALL generate a deterministic integration automation plan from the
system operating model.

#### Scenario: Operating model is available
- **GIVEN** `.idd/knowledge/operating-model.json` exists
- **WHEN** `rusty-idd knowledge integration-plan` runs
- **THEN** Rusty IDD SHALL output ordered work items with capability, layer,
  owner repos, anchors, gates, rollback, and OpenSpec change identifiers.

#### Scenario: Partial and external capabilities become work items
- **GIVEN** an operating capability has `partial`, `external`, or `missing`
  status
- **WHEN** the integration plan is generated
- **THEN** Rusty IDD SHALL create a work item that preserves repo owners and
  unresolved anchors.

#### Scenario: Planning context includes integration work
- **GIVEN** `.idd/knowledge/integration-plan.json` exists
- **WHEN** graph planning context is generated
- **THEN** Rusty IDD SHALL include selected integration work items in the
  planning packet.

#### Scenario: Read-only generation
- **GIVEN** peer repos are represented in the operating model
- **WHEN** the integration plan is generated
- **THEN** Rusty IDD SHALL NOT mutate peer repos or start host services.
