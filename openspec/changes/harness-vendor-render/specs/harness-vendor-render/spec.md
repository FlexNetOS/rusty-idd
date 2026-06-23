## ADDED Requirements

### Requirement: Engine-owned vendor adapter generation

Rusty IDD SHALL generate vendor adapter files from one engine-owned template
(the source of truth). `rusty-idd render` SHALL write a deterministic, minimal
adapter into each targeted vendor directory; the adapter SHALL point agents at
`rusty-idd next` and SHALL NOT contain workflow rules of its own.

#### Scenario: Render writes a deterministic adapter
- **GIVEN** a vendor directory such as `.claude`
- **WHEN** the operator runs `rusty-idd render --vendor claude`
- **THEN** a generated adapter file is written under `.claude/` that references `rusty-idd next`
- **AND** running render again over the unchanged tree produces byte-identical content

#### Scenario: Render all known vendors
- **GIVEN** the known vendor set (claude, codex, agents, devin)
- **WHEN** the operator runs `rusty-idd render --all`
- **THEN** each existing vendor directory receives its generated adapter

### Requirement: Fail-closed drift gate

`rusty-idd render --check` SHALL regenerate the expected adapter content in
memory and compare it to what is on disk. If any targeted adapter is missing or
differs from the engine output, the command SHALL report the drift and exit
non-zero. When all adapters match, it SHALL exit zero and write nothing.

#### Scenario: Matching adapters pass the gate
- **GIVEN** adapters previously written by `rusty-idd render --all`
- **WHEN** the operator runs `rusty-idd render --all --check`
- **THEN** the command exits zero and modifies no files

#### Scenario: Hand-edited adapter fails the gate
- **GIVEN** a vendor adapter whose content was hand-edited away from the engine output
- **WHEN** the operator runs `rusty-idd render --check`
- **THEN** the command names the drifted adapter and exits non-zero

#### Scenario: Missing adapter fails the gate
- **GIVEN** a known vendor directory with no adapter file
- **WHEN** the operator runs `rusty-idd render --check`
- **THEN** the command reports the missing adapter and exits non-zero
