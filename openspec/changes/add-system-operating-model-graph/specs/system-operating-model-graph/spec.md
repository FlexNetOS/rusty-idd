## ADDED Requirements

### Requirement: Generate system operating model graph
Rusty IDD SHALL generate a deterministic operating-model graph from the
system architecture graph.

#### Scenario: System graph is available
- **GIVEN** `.idd/knowledge/system-architecture.json` exists
- **WHEN** `rusty-idd knowledge operating-model` runs
- **THEN** Rusty IDD SHALL output layers, capabilities, repo mappings, edges,
  and findings in JSON or Markdown.

#### Scenario: Agentic company capabilities are mapped
- **GIVEN** the system graph includes repos such as `rusty-idd`, `handoff`,
  `weave`, `envctl`, `prompt_hub`, `ruvector`, `lifeos`, `teri`, `lane`, and
  network repos
- **WHEN** the operating model is generated
- **THEN** the output SHALL map those repos to agentic-company capabilities
  including IDD/spec engine, fleet handoff, agent communication, environment
  relay, prompt front door, vector/runtime, user front door, simulation, and
  network control.

#### Scenario: External or missing anchors are explicit
- **GIVEN** the target system includes anchors that are external or not yet
  represented by repo evidence
- **WHEN** the operating model is generated
- **THEN** Rusty IDD SHALL record those anchors as findings rather than
  implying implementation exists.

#### Scenario: RTK AI and Yazelix foundation surfaces are visible
- **GIVEN** the operating model includes toolchain and agent-run capabilities
- **WHEN** the operating model is generated
- **THEN** the output SHALL include Yazelix terminal/runtime context, RTK AI
  foundation context, and Beads upstream anchors for contributor workflows.

#### Scenario: Planning context includes operating model
- **GIVEN** `.idd/knowledge/operating-model.json` exists
- **WHEN** graph planning context is generated
- **THEN** Rusty IDD SHALL include selected operating layers and capabilities
  in the planning packet.

#### Scenario: Read-only generation
- **GIVEN** peer repos are discovered by the system graph
- **WHEN** the operating model is generated
- **THEN** Rusty IDD SHALL read only the system graph and SHALL NOT mutate peer
  repos or start host services.
