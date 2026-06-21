## ADDED Requirements

### Requirement: Adopt upstream knowledge repos before consolidation
Rusty IDD SHALL pin the current upstream revisions for `codegraph-rust` and
`repomix-rs`, preserve each upstream repository as an intact tracked integration
surface, and run upstream-native diagnostics before any local consolidation or
cut.

#### Scenario: Current upstream revision is verified
- **GIVEN** the repository is preparing a knowledge integration change
- **WHEN** the upstream repos are evaluated
- **THEN** the exact upstream URL and git revision for each repo SHALL be
  recorded in `/AI_MERGE`
- **AND** any difference from the tracked mirror revision SHALL be resolved by
  adopting the newer upstream surface or explicitly recording why that revision
  is held.

#### Scenario: Native diagnostics precede cuts
- **GIVEN** an upstream repo has Make, Just, Cargo, CI, package, script, or
  documentation-declared commands
- **WHEN** Rusty IDD evaluates whether to cut or adapt an upstream surface
- **THEN** those native commands SHALL be run or documented as blocked before
  any local consolidation is made
- **AND** failures SHALL be recorded with the command, exit result, required
  tools, runtime assumptions, and rollback path.

#### Scenario: Toolchain gaps are repo-managed
- **GIVEN** an upstream-native command requires a missing tool
- **WHEN** the tool is needed for Rusty IDD integration
- **THEN** the tool SHALL be added or repaired through the parent `meta` /
  `envctl` toolchain surface or an already tracked repo-local equivalent
- **AND** the change SHALL NOT install binaries into user-global paths.

### Requirement: Preserve full-feature knowledge capabilities
Rusty IDD SHALL preserve proven upstream capabilities through thin local
boundaries and SHALL NOT downgrade `tree-sitter`, domain, daemon, packaging,
fixture, script, or generated-asset surfaces to simplify the integration.

#### Scenario: Tree-sitter assumptions are corrected
- **GIVEN** the system uses `tree-sitter` through the Yazelix stack
- **WHEN** Rusty IDD documents or wires parser behavior
- **THEN** the documentation and implementation SHALL treat `tree-sitter` as an
  active system surface
- **AND** any Rusty IDD default exclusion SHALL be scoped to a specific feature
  boundary instead of described as a system-wide absence.

#### Scenario: Domains and daemon surfaces are scoped correctly
- **GIVEN** domains are handled through weave plus Obscura upgrades
- **WHEN** Rusty IDD documents or integrates host-service, daemon, MCP, or
  domain-adjacent behavior
- **THEN** default workflows SHALL keep host-service management out unless
  explicitly feature-gated and justified
- **AND** documentation SHALL distinguish default Rusty IDD workflow scope from
  system-wide capability availability.

#### Scenario: Thin adapter boundaries preserve upstream output
- **GIVEN** an upstream surface produces graph, pack, token, fixture, or
  diagnostic output used by Rusty IDD
- **WHEN** Rusty IDD exposes that surface through CLI or API calls
- **THEN** the local boundary SHALL provide deterministic DTO mapping,
  validation, size or token policy, feature flags, and reproducible output
- **AND** tests SHALL compare behavior before retaining any consolidation cut.

### Requirement: Rusty IDD lifecycle drives implementation order
Rusty IDD SHALL use its OpenSpec lifecycle artifacts as executable planning
inputs for repo and system goals before implementation, validation, and
archival.

#### Scenario: Artifacts are produced before implementation
- **GIVEN** a non-trivial integration task begins
- **WHEN** the task changes architecture, specs, tool surfaces, or upstream
  contracts
- **THEN** Rusty IDD SHALL produce or update proposal, spec, design, ADR, and
  task artifacts before implementation proceeds
- **AND** `rusty-idd spec status`, `rusty-idd spec next`, and the headless
  runner SHALL be used where applicable to drive the ordered workflow.

#### Scenario: Consolidation is test-driven
- **GIVEN** upstream diagnostics have identified an evidenced cut
- **WHEN** local consolidation is attempted
- **THEN** the cut SHALL be made as one TDD step
- **AND** targeted upstream plus Rusty IDD tests SHALL be run before the cut is
  retained
- **AND** rollback SHALL be recorded in the audit trail.
