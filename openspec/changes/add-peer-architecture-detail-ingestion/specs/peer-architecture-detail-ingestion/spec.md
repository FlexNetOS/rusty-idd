## ADDED Requirements

### Requirement: Ingest peer architecture details
Rusty IDD SHALL ingest bounded details from peer `.idd/knowledge/architecture.json`
artifacts when generating the system architecture graph.

#### Scenario: Peer architecture graph is available
- **GIVEN** a peer repo contains `.idd/knowledge/architecture.json`
- **WHEN** the system architecture graph is generated
- **THEN** the peer repo entry SHALL include a bounded architecture summary with
  source graph metrics, languages, top components, and integration surfaces.

#### Scenario: Planning context includes peer details
- **GIVEN** the system architecture graph includes peer architecture summaries
- **WHEN** graph planning context is generated
- **THEN** relevant peer repo entries SHALL preserve those summaries for
  planning and integration boundary decisions.

#### Scenario: Bad peer graph does not block system graph generation
- **GIVEN** a peer repo has an unreadable or invalid architecture artifact
- **WHEN** the system architecture graph is generated
- **THEN** Rusty IDD SHALL record a finding for that peer
- **AND** system graph generation SHALL continue for other repos.
