## ADDED Requirements

### Requirement: Single front-door imperative

Rusty IDD SHALL expose one command, `rusty-idd next`, that prints the single
next imperative for the active change, computed from repository state. Vendor
agent surfaces (`.claude`, `.codex`, `.agents`, `.kimi`, …) SHALL obtain their
workflow direction by invoking this command rather than carrying a static,
always-loaded prose harness.

#### Scenario: Active change has unfinished artifacts
- **GIVEN** `.idd/workflow/active-change` names a change whose artifact DAG is incomplete
- **WHEN** the operator runs `rusty-idd next`
- **THEN** the command prints the active change, its artifact-DAG status, the single next ready artifact, and one scoped command to produce it
- **AND** exits `0`

#### Scenario: No active change is set
- **GIVEN** `.idd/workflow/active-change` is absent or empty
- **WHEN** the operator runs `rusty-idd next`
- **THEN** the command prints guidance to set or create a change and exits `0`

#### Scenario: Active change pointer is dangling
- **GIVEN** `.idd/workflow/active-change` names a change with no directory under `openspec/changes/`
- **WHEN** the operator runs `rusty-idd next`
- **THEN** the command reports the dangling pointer on stderr and exits non-zero

### Requirement: One oracle, not two

The next-step computation SHALL reuse the spec engine's artifact-DAG oracle
(`rusty_idd_spec::schema` via `commands::spec_status`). `rusty-idd next` and
`rusty-idd spec next` SHALL never disagree about the next ready artifact for the
same change.

#### Scenario: Front door and spec oracle agree
- **GIVEN** any change directory with a partial set of artifacts
- **WHEN** both `rusty-idd next` and `rusty-idd spec next <change>` are run
- **THEN** both report the same next ready artifact id

### Requirement: Token-scoped direction

`rusty-idd next` SHALL emit only the active change's status plus one next action.
It SHALL NOT emit the full rule/policy/skill corpus, so per-session context cost
stays bounded regardless of how large the harness grows.

#### Scenario: Output is bounded to the current step
- **GIVEN** an active change
- **WHEN** `rusty-idd next` runs
- **THEN** its output is scoped to the active change and the single next artifact, not the entire workflow corpus
