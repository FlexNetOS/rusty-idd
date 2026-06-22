## ADDED Requirements

### Requirement: Produce a single-repo architecture decision package
Rusty IDD SHALL produce an evidence-backed architecture decision package before
combining the Rusty IDD and handoff repositories.

#### Scenario: Decision compares target repository shapes
- **GIVEN** Rusty IDD and handoff are candidates for single-repository
  consolidation
- **WHEN** the architecture planning change is executed
- **THEN** the decision package SHALL compare embedding Rusty IDD in handoff,
  embedding handoff in Rusty IDD, and preserving both as peer packages in one
  repository.
- **AND** the selected shape SHALL cite generated artifacts, source scans,
  validation gates, and rollback constraints.

#### Scenario: Planning precedes code movement
- **GIVEN** repository consolidation can delete or obscure ownership boundaries
- **WHEN** the planning change is in progress
- **THEN** the change SHALL NOT move source code between repositories until
  inventories, contract maps, OpenSpec artifacts, ADR, and evidence records are
  ready.

### Requirement: Preserve witnessed handoff semantics
The combined repository architecture SHALL preserve handoff task-card, claim,
checkpoint, done, delivery, and fleet pickup semantics.

#### Scenario: Handoff remains authoritative for witnessed task state
- **GIVEN** handoff task cards and ledger state are used to track autonomous
  work
- **WHEN** Rusty IDD and handoff are combined into one repository
- **THEN** handoff task state semantics SHALL remain explicit and testable.
- **AND** Rusty IDD planning artifacts SHALL reference that task state rather
  than replacing it with untracked local notes.

### Requirement: Preserve Rusty IDD intent and validation semantics
The combined repository architecture SHALL preserve Rusty IDD intent, knowledge,
OpenSpec, ADR, validation, and evidence semantics.

#### Scenario: Rusty IDD remains authoritative for planning readiness
- **GIVEN** Rusty IDD gates implementation through generated knowledge,
  OpenSpec readiness, validation, and evidence
- **WHEN** handoff is combined with Rusty IDD in one repository
- **THEN** Rusty IDD planning readiness SHALL remain explicit, generated, and
  CI-checkable.
