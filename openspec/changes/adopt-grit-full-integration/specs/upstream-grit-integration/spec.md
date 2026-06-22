## ADDED Requirements

### Requirement: Preserve Grit as an upstream reference
Rusty IDD SHALL preserve Grit as a full as-is upstream reference before any
future implementation, refactor, or consolidation work depends on it.

#### Scenario: Import uses the current tracked upstream surface
- **GIVEN** Grit is available as a clean Git repository
- **WHEN** Rusty IDD adopts the Grit integration reference
- **THEN** the mirror SHALL be created from the current tracked files at the
  pinned upstream commit
- **AND** the mirror SHALL include tracked dotfiles, workflows, scripts, docs,
  tests, examples, nested projects, assets, manifests, and lockfiles
- **AND** the mirror SHALL exclude only Git metadata and generated local build
  outputs that are not tracked by Grit.

#### Scenario: Adoption does not rewrite Grit
- **GIVEN** the Grit mirror is an upstream adoption baseline
- **WHEN** the adoption slice is implemented
- **THEN** Rusty IDD SHALL NOT refactor, trim, cherry-pick, downgrade, or edit
  Grit source code
- **AND** any future cut from the upstream surface SHALL require separate
  evidence and a new OpenSpec change.

### Requirement: Generate Rusty IDD evidence for Grit integration
Rusty IDD SHALL generate planning and evidence artifacts that make the Grit
integration reviewable and reproducible.

#### Scenario: Goal-file planning context is generated
- **GIVEN** a tracked goal file describes the Grit integration intent
- **WHEN** `rusty-idd knowledge plan-context --goal-file` is run
- **THEN** Rusty IDD SHALL emit Markdown and JSON planning context artifacts
  that bind the goal to the current knowledge graph.

#### Scenario: Scan and plan artifacts are preserved
- **GIVEN** Rusty IDD is planning the Grit adoption
- **WHEN** `rusty-idd scan` and `rusty-idd plan` are run against Rusty IDD and
  Grit
- **THEN** the emitted inventories, feature matrix, contracts, merge plan, risk
  register, task stubs, and plan manifest SHALL be preserved as review evidence.

### Requirement: Regenerate control-plane artifacts after adoption
Rusty IDD SHALL refresh generated control-plane artifacts after the Grit mirror
is present.

#### Scenario: Knowledge and diagrams include the adoption baseline
- **GIVEN** the Grit mirror has been added under `third_party/upstream/grit`
- **WHEN** the Rusty IDD knowledge and diagram commands are rerun
- **THEN** `.idd/knowledge/*` and `docs/rusty-idd/architecture-diagrams.md`
  SHALL reflect the updated workspace graph and context package
- **AND** `.idd/MANIFEST.tsv` SHALL include the adopted Grit artifacts.
