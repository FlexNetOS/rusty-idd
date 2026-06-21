## ADDED Requirements

### Requirement: Generate graph-backed planning context
Rusty IDD SHALL generate a deterministic planning context from the repo
architecture graph and optional system architecture graph.

#### Scenario: Repo graph is consumed
- **GIVEN** `.idd/knowledge/architecture.json` exists
- **WHEN** graph planning context is generated
- **THEN** Rusty IDD SHALL include automation stages, integration surfaces,
  source graph metrics, context package metrics, and bounded component context.

#### Scenario: System graph enriches planning
- **GIVEN** `.idd/knowledge/system-architecture.json` exists
- **WHEN** graph planning context is generated
- **THEN** Rusty IDD SHALL include relevant system roles and repos selected by
  deterministic goal and graph metadata matching.

#### Scenario: Missing system graph is non-fatal
- **GIVEN** the repo architecture graph exists but the system graph does not
- **WHEN** graph planning context is generated
- **THEN** Rusty IDD SHALL still emit a repo-local planning context
- **AND** it SHALL record a finding that system graph context is unavailable.
