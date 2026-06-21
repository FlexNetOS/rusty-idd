## ADDED Requirements

### Requirement: Merge goals use a Rusty IDD package
Rusty IDD SHALL expose a reusable merge-goal package that consolidates retired
merge bridge contracts into Rust-owned workflow data.

#### Scenario: Package is available from the CLI
- **GIVEN** an agent needs to plan a merge, migration, or repository-unification goal
- **WHEN** it runs `rusty-idd merge-tools show`
- **THEN** the CLI SHALL describe the inventory, plan, decide, implement, verify, and evidence phases.

#### Scenario: Legacy bridge surfaces are dispositioned
- **GIVEN** legacy `.claude`, `.gemini`, `_workspace`, `AI_MERGE`, or ADR material exists in history
- **WHEN** the package is rendered
- **THEN** it SHALL identify the surface disposition and the Rusty IDD replacement.

#### Scenario: Active ADR set is current-only
- **GIVEN** the repository ADR directory is read by `rusty-idd spec adr list`
- **WHEN** the Codex harness flow cleanup is complete
- **THEN** only the Codex harness Rusty IDD flow ADR SHALL be active in `adr/`.
