## MODIFIED Requirements

### Requirement: Rusty IDD shall adopt handoff as a complete source before cutting behavior
Rusty IDD SHALL consume `meta/handoff` as a whole upstream source through an
adopt-first migration path, beginning with a complete tracked-file upstream
mirror under `third_party/upstream/handoff`.

#### Scenario: Handoff source is prepared for migration
- **GIVEN** `meta/handoff` contains `hf`, `ledger`, `work-order`, embedded
  Rusty IDD crates, docs, hooks, policies, and durable `.handoff` text state
- **WHEN** Rusty IDD starts the implementation migration
- **THEN** the first implementation slice SHALL preserve the tracked handoff
  surface as `third_party/upstream/handoff` before refactoring behavior.
- **AND** Git metadata, local lock files, binary cache state, and untracked
  runtime outputs SHALL NOT become canonical Rusty IDD state.
