# Harness Session Frontdoor

## Purpose
Base specification for the `harness-session-frontdoor` capability, established when its first change was archived.
## Requirements
### Requirement: Vendor session-start invokes the front door
Each vendor surface that supports a session-start hook SHALL invoke `rusty-idd next` automatically when a session begins, so the computed next-step imperative
is presented without the agent having to remember to run it. The hook SHALL run
the repo-local `rusty-idd` binary against the repository root and SHALL NOT
embed workflow logic of its own (the workflow lives in the engine).

#### Scenario: Codex session start calls the front door
- **GIVEN** the repo-local `.codex/hooks.json`
- **WHEN** a Codex session starts
- **THEN** the `SessionStart` hook runs `rusty-idd next` against the repository root
- **AND** the existing `PreToolUse`/`PostToolUse`/`Stop`/`SubagentStop` hooks are unchanged

#### Scenario: Claude Code session start calls the front door
- **GIVEN** the repo-local `.claude/settings.json`
- **WHEN** a Claude Code session starts
- **THEN** its `hooks.SessionStart` entry runs `rusty-idd next`

### Requirement: Fail-closed ADR-number collision gate
Rusty IDD SHALL detect duplicate ADR sequence numbers. `rusty-idd spec adr list --check` SHALL exit non-zero when any duplicate ADR number exists that is not in
the frozen baseline of known historical collisions, and SHALL exit zero when the
only duplicates are the accepted baseline. The four pre-existing collisions
(0002, 0004, 0005, 0006) are immutable historical artifacts and form that
baseline; they SHALL NOT cause the gate to fail.

#### Scenario: Known historical collisions pass the gate
- **GIVEN** the ADR set containing the four accepted duplicate numbers
- **WHEN** the operator runs `rusty-idd spec adr list --check`
- **THEN** the command exits zero
- **AND** it still reports the duplicates for visibility

#### Scenario: A new collision fails the gate
- **GIVEN** an ADR set with a duplicate number outside the frozen baseline
- **WHEN** the operator runs `rusty-idd spec adr list --check`
- **THEN** the command names the offending number and exits non-zero

