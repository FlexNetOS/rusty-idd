## MODIFIED Requirements

### Requirement: Codex hooks enforce workflow readiness

The Codex harness SHALL enforce Rusty IDD workflow readiness before
write-capable tool use and SHALL enforce delivery evidence at Stop only when the
current worktree has dirty files or commits beyond the authoritative develop
base.

#### Scenario: Stop hook accepts clean branch from remote develop

- **GIVEN** local `develop` is stale
- **AND** `origin/develop` resolves to the current feature branch base
- **AND** the worktree has no dirty files or commits beyond `origin/develop`
- **WHEN** the Stop workflow hook runs
- **THEN** it does not require PR/automerge delivery evidence
