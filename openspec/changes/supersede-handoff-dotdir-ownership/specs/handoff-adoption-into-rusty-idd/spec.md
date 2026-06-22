## ADDED Requirements

### Requirement: Rusty IDD shall adopt handoff as a complete source before cutting behavior
Rusty IDD SHALL consume `meta/handoff` as a whole upstream source through an
adopt-first migration path.

#### Scenario: Handoff source is prepared for migration
- **GIVEN** `meta/handoff` contains `hf`, `ledger`, `work-order`, embedded
  Rusty IDD crates, docs, hooks, policies, and durable `.handoff` text state
- **WHEN** Rusty IDD starts the implementation migration
- **THEN** the first implementation slice SHALL preserve the tracked handoff
  surface as evidence or an upstream mirror before refactoring behavior.
- **AND** Git metadata, local lock files, binary cache state, and untracked
  runtime outputs SHALL NOT become canonical Rusty IDD state.

### Requirement: Handoff runtime semantics shall become typed Rusty IDD capabilities
Rusty IDD SHALL represent handoff task, ledger, fleet, delivery, and policy
semantics through Rusty IDD-owned adapters before legacy surfaces are retired.

#### Scenario: Agent migrates handoff behavior
- **GIVEN** handoff currently owns task minting, claim, checkpoint, done,
  delivery, policy, fleet status, drift, and ledger export/import behavior
- **WHEN** Rusty IDD integrates that behavior
- **THEN** Rusty IDD SHALL add typed adapter boundaries and parity tests before
  deleting, flattening, or replacing the original handoff implementation.

### Requirement: Combined state precedence shall be explicit
Rusty IDD SHALL publish a deterministic precedence order for combined Rusty IDD
and handoff state.

#### Scenario: Dot-directory or ledger state conflicts
- **GIVEN** `.idd`, OpenSpec, ADR, `.handoff`, `.kb`, `.idea`, `.claude`, and
  local tool state disagree about the current workflow
- **WHEN** Rusty IDD selects the authoritative state
- **THEN** Git-tracked Rusty IDD artifacts, `.idd` goal/context/manifest,
  OpenSpec, ADR, and validation evidence SHALL win for planning and readiness.
- **AND** handoff ledger events and rendered task-card evidence SHALL be treated
  as adopted runtime evidence, not as a replacement for planning readiness.
