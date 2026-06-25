# Harness Control Plane

## Purpose
Base specification for the `harness-control-plane` capability, established when its first change was archived.
## Requirements
### Requirement: Single front-door imperative
Rusty IDD SHALL expose one command, `rusty-idd next`, that prints the single
next imperative for the active change, computed from repository state. Vendor
agent surfaces (`.claude`, `.codex`, `.agents`, `.kimi`, …) SHALL obtain their
workflow direction by invoking this command rather than carrying a static,
always-loaded prose harness. The command SHALL additionally support a `--json`
mode that emits one deterministic JSON object so non-interactive adapters can act
on structured fields instead of parsing human text.

#### Scenario: Active change has unfinished artifacts
- **GIVEN** `.idd/workflow/active-change` names a change whose artifact DAG is incomplete
- **WHEN** the operator runs `rusty-idd next`
- **THEN** the command prints the active change, its artifact-DAG status, the single next ready artifact, and one scoped command to produce it
- **AND** exits `0`

#### Scenario: Machine-readable mode emits a deterministic object
- **GIVEN** `.idd/workflow/active-change` names an existing change
- **WHEN** the operator runs `rusty-idd next --json`
- **THEN** stdout is a single JSON object with the active change, artifact status, the next ready artifact (or null), archivability, and the scoped next command
- **AND** the same invocation produces byte-identical output on repeated runs over an unchanged tree

#### Scenario: No active change is set
- **GIVEN** `.idd/workflow/active-change` is absent or empty
- **WHEN** the operator runs `rusty-idd next` (text or `--json`)
- **THEN** the command reports that no change is active and exits `0`

#### Scenario: Active change pointer is dangling fails closed
- **GIVEN** `.idd/workflow/active-change` names a change with no directory under `openspec/changes/`
- **WHEN** the operator runs `rusty-idd next --json`
- **THEN** no JSON object is written to stdout and the command exits non-zero

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

