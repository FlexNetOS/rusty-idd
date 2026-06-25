## MODIFIED Requirements

### Requirement: Meta-owned Rust toolchain audit

Rusty IDD SHALL audit and bootstrap the Rust compiler and cache surface used for
repository builds without accepting user-global or system-owned Rust state as
compliant.

#### Scenario: CI bootstrap uses the envctl cache target

- **GIVEN** GitHub CI materializes the Rusty IDD Rust environment
- **WHEN** the checked-in bootstrap selects a compiler cache wrapper
- **THEN** it SHALL use `kache`, `hurry`, or `zccache` under the parent
  meta/envctl root.
- **AND** it SHALL fail rather than falling back to `sccache`.
- **AND** it SHALL cache parent meta Rust toolchain and tool state separately
  from workspace `target` artifacts.
- **AND** its GitHub Actions cache paths SHALL use canonical parent meta paths
  that do not contain `..`.

#### Scenario: Bootstrap duration evidence is reported

- **GIVEN** strict CI bootstrap installs or verifies Rust tools
- **WHEN** rustup, codegen component, cache wrapper, linker, or cargo-audit
  setup runs
- **THEN** the workflow logs SHALL identify whether the tool was reused or
  installed and report elapsed seconds for the setup span.

#### Scenario: Workflows expose the actual compiler surface

- **GIVEN** primary CI or promotion verification runs Rust gates
- **WHEN** the strict Rust toolchain audit runs
- **THEN** it SHALL pass explicit `rustc`, `cargo`, `RUSTUP_HOME`,
  `CARGO_HOME`, cache wrapper, cache root, linker, toolchain, and backend
  values derived from the active CI environment.
