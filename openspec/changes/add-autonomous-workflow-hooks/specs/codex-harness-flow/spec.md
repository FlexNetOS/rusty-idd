## MODIFIED Requirements

### Requirement: Harness follows Rusty IDD flow
The Codex harness SHALL treat Rusty IDD as the intent-driven workflow engine and SHALL run graph-backed planning, OpenSpec artifact checks, task-card checks, validation checks, and PR handoff checks before claiming implementation work complete.

#### Scenario: Intent is captured before implementation
- **GIVEN** a user provides a goal for Rusty IDD work
- **WHEN** the Codex harness prepares agent context
- **THEN** it SHALL use Rusty IDD knowledge planning artifacts to capture the goal before any write-capable implementation pass.

#### Scenario: OpenSpec artifacts gate writes
- **GIVEN** a proposed change has not produced the required OpenSpec artifacts
- **WHEN** the harness reaches the implementation phase
- **THEN** write-capable agents SHALL refuse to implement until proposal, specs, design, ADR status, and tasks are ready according to `rusty-idd spec status`.

#### Scenario: AI_MERGE is evidence, not intent
- **GIVEN** AI_MERGE records exist for audit, migration, or merge evidence
- **WHEN** the harness reads repository context
- **THEN** it SHALL treat `AI_MERGE/` as a Rusty IDD tool/evidence surface and SHALL NOT present it as the main intent source or authoritative Rusty IDD control plane.

#### Scenario: Default loop stops before writes
- **GIVEN** the repo-local model loop runs without explicit implementation authorization
- **WHEN** it emits or executes its default passes
- **THEN** the default passes SHALL remain read-only and SHALL stop at design, artifact status, and verification outputs.

#### Scenario: Pre-hook rejects unprepared implementation
- **GIVEN** a Codex agent is about to run a write-capable tool in the Rusty IDD repository
- **WHEN** the workflow pre-hook runs
- **THEN** it SHALL require a non-main feature branch in a git worktree based on `develop`, refreshed `.idd/knowledge/plan-context.md`, a ready OpenSpec change, and task-card evidence before allowing implementation.

#### Scenario: Post-hook rejects incomplete handoff
- **GIVEN** a Codex agent has modified tracked repository files for a Rusty IDD change
- **WHEN** the workflow post-hook or turn-completion hook runs
- **THEN** it SHALL require validation evidence and PR handoff evidence that the feature branch has been pushed and configured for auto-merge into `develop`.

## ADDED Requirements

### Requirement: Autonomous workflow hooks are Rust-native
The Codex harness SHALL implement autonomous workflow hook checks through the `rusty-idd` Rust CLI and SHALL NOT introduce Python, shell script, or host-service runtime dependencies for repo-local hook enforcement.

#### Scenario: Hook command is registered
- **GIVEN** the repo-local Codex hooks are enabled
- **WHEN** Codex loads `.codex/hooks.json`
- **THEN** the hook commands SHALL invoke `cargo run --bin rusty-idd -- codex workflow-check` from the git root.

#### Scenario: Hook runtime remains repository-scoped
- **GIVEN** the workflow hooks run during a Codex session
- **WHEN** the hook evaluates repository state
- **THEN** it SHALL inspect files, git state, OpenSpec status, and handoff evidence inside the repository boundary without managing host services or installing user-global tools.
