## ADDED Requirements

### Requirement: Preserve handoff as a complete upstream reference
Rusty IDD SHALL preserve `meta/handoff` as a complete tracked-file upstream
reference before adapter, consolidation, cleanup, or retirement work depends on
handoff behavior.

#### Scenario: Import includes every tracked handoff file
- **GIVEN** `meta/handoff` is available as a Git repository
- **WHEN** Rusty IDD adopts the handoff upstream reference
- **THEN** the mirror SHALL be created from the pinned source commit's tracked
  files.
- **AND** the mirror SHALL include tracked dotfiles, tracked dot directories,
  workflows, scripts, docs, tests, nested crates, manifests, lockfiles, task
  cards, fleet capsules, policies, packets, and ledger text evidence.
- **AND** the mirror SHALL exclude only Git metadata and untracked local state.

#### Scenario: Adoption does not rewrite handoff
- **GIVEN** the handoff mirror is an upstream adoption baseline
- **WHEN** the adoption slice is implemented
- **THEN** Rusty IDD SHALL NOT refactor, trim, cherry-pick, downgrade, or edit
  handoff source code in the mirror.
- **AND** any future cut from the upstream surface SHALL require separate
  evidence, OpenSpec artifacts, and validation.

### Requirement: Record source checkout state at import time
Rusty IDD SHALL record whether the source handoff checkout was clean or dirty
when the upstream mirror was imported.

#### Scenario: Source handoff checkout has local changes
- **GIVEN** the source checkout contains modified or untracked files
- **WHEN** the mirror imports the pinned tracked commit
- **THEN** Rusty IDD SHALL record the dirty and untracked source state as
  evidence.
- **AND** Rusty IDD SHALL NOT silently promote those uncommitted source changes
  into the mirror.

### Requirement: Regenerate Rusty IDD control-plane artifacts after handoff adoption
Rusty IDD SHALL refresh generated control-plane artifacts after the handoff
mirror is present.

#### Scenario: Knowledge graph includes handoff mirror
- **GIVEN** `third_party/upstream/handoff` has been added
- **WHEN** Rusty IDD knowledge, diagram, plan-context, and manifest commands are
  rerun
- **THEN** `.idd/knowledge/*`, `docs/rusty-idd/architecture-diagrams.md`, and
  `.idd/MANIFEST.tsv` SHALL reflect the adopted handoff baseline.
