## ADDED Requirements

### Requirement: Generate architecture diagram artifacts
Rusty IDD SHALL generate deterministic architecture diagram artifacts from the
current architecture graph.

#### Scenario: Diagrams use the architecture graph
- **GIVEN** a workspace with a generated Rusty IDD architecture graph
- **WHEN** architecture diagrams are generated
- **THEN** Rusty IDD SHALL derive the diagrams from the same graph model used by
  `.idd/knowledge/architecture.json`
- **AND** the output SHALL be deterministic Markdown containing Mermaid diagram
  blocks.

#### Scenario: Diagrams cover workflow, crates, and artifacts
- **GIVEN** Rusty IDD owns lifecycle, crate, and generated artifact surfaces
- **WHEN** architecture diagrams are generated
- **THEN** the output SHALL include diagrams for the autonomous workflow,
  crate/component relationships, and generated artifact flow.

### Requirement: Validate diagram freshness
Rusty IDD SHALL provide a repeatable repository command that verifies generated
architecture diagrams are fresh.

#### Scenario: CI checks generated diagrams
- **GIVEN** the repository contains generated architecture diagrams
- **WHEN** the repository CI recipe runs
- **THEN** it SHALL compare the checked-in diagram artifact to freshly generated
  output and fail when the artifact is stale.
