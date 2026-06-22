## ADDED Requirements

### Requirement: Rusty IDD shall own canonical workflow state
Rusty IDD SHALL be the canonical owner of combined Rusty IDD plus handoff
workflow state.

#### Scenario: Goal and planning state are canonical in `.idd`
- **GIVEN** a user intent requires handoff, task, fleet, or ledger behavior
- **WHEN** the work is planned in the combined architecture
- **THEN** the durable intent SHALL start from `.idd/goals`, generated knowledge
  context, OpenSpec artifacts, ADRs, and validation evidence.
- **AND** `.handoff`, `.kb`, `.idea`, `.claude`, and tool-specific dot
  directories SHALL NOT replace Rusty IDD readiness gates.

### Requirement: Dot directories shall have explicit authority classes
Rusty IDD SHALL classify dot directories by authority before consuming or
generating them.

#### Scenario: Agent sees multiple dot directories
- **GIVEN** a repository contains `.idd`, `.handoff`, `.kb`, `.idea`, `.claude`,
  `.codex`, `.agents`, `.github`, and local tool dot directories
- **WHEN** an agent plans or implements work
- **THEN** the agent SHALL distinguish canonical control-plane state,
  adopted runtime/evidence state, workspace knowledge input, idea intake,
  compatibility source material, enforcement policy, reusable skills, remote CI,
  and local cache/editor state.

### Requirement: Handoff shall be adopted into Rusty IDD whole
Rusty IDD SHALL consume `meta/handoff` as an adopt-first source before cutting or
flattening its behavior.

#### Scenario: Handoff runtime semantics are migrated
- **GIVEN** handoff has task-card, claim, checkpoint, done, delivery, fleet, and
  ledger behavior
- **WHEN** Rusty IDD consumes handoff
- **THEN** those semantics SHALL be inventoried, preserved, and represented by a
  Rusty IDD-owned adapter before duplicate legacy surfaces are retired.

### Requirement: Legacy harness traces shall remain compatibility input
Rusty IDD SHALL treat `.handoff` harness-loop traces and `.claude`/`harness_hub`
material as compatibility input until parity is proven.

#### Scenario: Legacy harness state conflicts with Rusty IDD state
- **GIVEN** `.handoff` or `.claude` material conflicts with `.idd` goal,
  OpenSpec, ADR, or validation state
- **WHEN** the combined workflow chooses an authoritative state
- **THEN** `.idd` and OpenSpec SHALL win for current intent.
- **AND** legacy state SHALL be recorded as migration evidence rather than
  silently deleted or promoted.

### Requirement: Visual graphs shall be generated or checked as planning evidence
The planning package SHALL include visual graph artifacts for ownership,
lifecycle, adoption, compatibility, and repository layout.

#### Scenario: Agent needs visual layout before implementation
- **GIVEN** a future agent must implement Rusty IDD consuming handoff
- **WHEN** the agent reads the planning package
- **THEN** it SHALL find graph artifacts showing dot-directory ownership,
  intent-to-evidence flow, handoff adoption phases, compatibility retirement,
  and the target repository layout.
