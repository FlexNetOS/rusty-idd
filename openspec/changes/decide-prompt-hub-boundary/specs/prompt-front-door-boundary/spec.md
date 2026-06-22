## ADDED Requirements

### Requirement: PromptHub produces goal artifacts for Rusty IDD
PromptHub SHALL be treated as a front-door/spec-producer that emits durable
goal artifacts for Rusty IDD to consume.

#### Scenario: Goal artifact feeds Rusty IDD
- **GIVEN** PromptHub has transformed user intent into a rendered goal artifact
- **WHEN** Rusty IDD runs goal-file planning for that artifact
- **THEN** Rusty IDD SHALL generate graph-backed plan context and keep ownership
  of OpenSpec, ADR, task, validation, manifest, and merge evidence state.

#### Scenario: PromptHub lifecycle authority remains bounded
- **GIVEN** PromptHub stores prompts, templates, audit records, search indexes,
  and swarm handoff templates
- **WHEN** those outputs are used for Rusty IDD work
- **THEN** PromptHub SHALL NOT own `.idd`, OpenSpec, ADR, manifest, or Rusty IDD
  validation state.

#### Scenario: Rusty IDD integration stays thin
- **GIVEN** Rusty IDD consumes a PromptHub-produced goal artifact
- **WHEN** the integration is implemented
- **THEN** Rusty IDD SHALL limit local integration to deterministic goal-file
  parsing, provenance validation, size/token policy, generated context, and
  workflow evidence.

### Requirement: PromptHub boundary evidence is refreshed
Rusty IDD SHALL record source evidence for the PromptHub boundary decision
before implementation.

#### Scenario: Local PromptHub source is researched
- **GIVEN** `/home/drdave/Desktop/meta/prompt_hub` is available locally
- **WHEN** the boundary decision is recorded
- **THEN** evidence SHALL cite PromptHub workspace members, core library
  surfaces, CLI surfaces, templates, continuity capsule, dirty-state note, and a
  native diagnostic result.
