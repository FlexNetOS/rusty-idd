## ADDED Requirements

### Requirement: Meta-owned Rust toolchain audit

Rusty IDD SHALL audit the Rust compiler and cache surface used for repository
builds without accepting user-global or system-owned Rust state as compliant.

#### Scenario: Actual compiler path is reported

- **GIVEN** Rusty IDD audits an active Rust build environment
- **WHEN** the audit runs
- **THEN** it SHALL report the actual Cargo-executed `rustc` path and Cargo
  binary path rather than only the selected channel label.

#### Scenario: User-global Rust homes are rejected

- **GIVEN** `RUSTUP_HOME` or `CARGO_HOME` resolves under the user's home
  directory outside the parent `meta` root
- **WHEN** the Rust toolchain audit runs in strict mode
- **THEN** it SHALL fail and identify the offending path.

#### Scenario: Nightly backend surface is required

- **GIVEN** the repository's meta-owned Rust workflow is active
- **WHEN** the audit checks the selected toolchain
- **THEN** it SHALL require a nightly Rust toolchain and record
  `rustc_codegen_gcc` as the required runtime backend surface.

#### Scenario: Cache wrapper stays meta-owned

- **GIVEN** a Rust compiler cache wrapper is configured
- **WHEN** the audit classifies it
- **THEN** it SHALL accept `kache`, `hurry`, or `zccache` only when the wrapper
  path and cache root are owned by the parent `meta` / `envctl` environment.
- **AND** it SHALL classify `sccache` as an accepted last-resort fallback only
  when the contract records version `0.15.0` or newer and a Unix-domain socket
  transport instead of TCP loopback for daemon communication.

#### Scenario: Wild linker replaces mold

- **GIVEN** a fast linker is configured for Linux Rust builds
- **WHEN** the audit checks linker policy
- **THEN** it SHALL require a parent-managed `wild` / `wild-linker` path and
  SHALL NOT treat mold as the compliant linker for this workflow.

#### Scenario: Parent provisioning boundary is preserved

- **GIVEN** the required Rust toolchain or cache tool is missing
- **WHEN** an agent works in this repository
- **THEN** the agent SHALL NOT install it into user-global or system paths from
  Rusty IDD.
- **AND** the missing tool SHALL be routed to the parent `meta` / `envctl`
  provisioning contract.

#### Scenario: CI bootstrap uses the envctl cache target

- **GIVEN** GitHub CI materializes the Rusty IDD Rust environment
- **WHEN** the checked-in bootstrap selects a compiler cache wrapper
- **THEN** it SHALL use `kache`, `hurry`, or `zccache` under the parent
  meta/envctl root.
- **AND** it SHALL fail rather than falling back to `sccache`.

#### Scenario: Workflows expose the actual compiler surface

- **GIVEN** primary CI or promotion verification runs Rust gates
- **WHEN** the strict Rust toolchain audit runs
- **THEN** it SHALL pass explicit `rustc`, `cargo`, `RUSTUP_HOME`,
  `CARGO_HOME`, cache wrapper, cache root, linker, toolchain, and backend
  values derived from the active CI environment.
