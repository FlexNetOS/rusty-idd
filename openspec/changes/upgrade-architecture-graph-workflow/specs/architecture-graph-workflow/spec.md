## ADDED Requirements

### Requirement: Generate architecture graph artifacts
Rusty IDD SHALL generate deterministic architecture graph artifacts that map
code structure, integration surfaces, and OpenSpec automation stages.

#### Scenario: Architecture graph uses integrated tools
- **GIVEN** a workspace with source files and Rusty IDD control-plane artifacts
- **WHEN** the architecture graph is generated
- **THEN** Rusty IDD SHALL use the CodeGraph-backed knowledge index for
  structural nodes, edges, languages, and hotspots
- **AND** Rusty IDD SHALL use repomix-backed pack metrics for context package
  and token-budget evidence.

#### Scenario: Architecture graph maps automation stages
- **GIVEN** Rusty IDD owns the OpenSpec lifecycle
- **WHEN** architecture artifacts are generated
- **THEN** the graph SHALL include automation stages for intake, architecture
  mapping, specification, implementation, validation, and handoff
- **AND** the graph SHALL connect those stages to the relevant integration
  surfaces.

#### Scenario: Refresh writes durable artifacts
- **GIVEN** the user runs `rusty-idd knowledge refresh --workspace .`
- **WHEN** refresh completes
- **THEN** `.idd/knowledge/architecture.json` SHALL exist
- **AND** `.idd/knowledge/architecture.md` SHALL exist
- **AND** both artifacts SHALL be deterministic control-plane outputs.
