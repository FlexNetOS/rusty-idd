## ADDED Requirements

### Requirement: Generate system architecture peer graph
Rusty IDD SHALL generate a read-only system architecture graph that maps the
current repo to peer repositories in a parent meta workspace.

#### Scenario: Meta project metadata is used when available
- **GIVEN** a system root with a working `meta project list --json`
- **WHEN** the system architecture graph is generated
- **THEN** Rusty IDD SHALL use project names, paths, repos, tags, and meta flags
  from the meta project metadata
- **AND** the graph SHALL record that the source was meta metadata.

#### Scenario: Filesystem discovery is available as fallback
- **GIVEN** a system root without usable meta project metadata
- **WHEN** the system architecture graph is generated
- **THEN** Rusty IDD SHALL discover immediate child git repositories from the
  filesystem
- **AND** the graph SHALL still include deterministic repo nodes and edges.

#### Scenario: System graph maps integration roles
- **GIVEN** known system repos such as `rusty-idd`, `handoff`, `weave`,
  `obscura`, `yazelix`, `envctl`, prompt/meta repos, hubs, and agent tooling
- **WHEN** the system graph is rendered
- **THEN** the graph SHALL classify their integration roles
- **AND** it SHALL connect those roles to Rusty IDD automation stages without
  starting services or mutating peer repos.
