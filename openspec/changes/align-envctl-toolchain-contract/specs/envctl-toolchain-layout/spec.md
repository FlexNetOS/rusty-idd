### Requirement: Rusty IDD supports meta-owned Rust layout selection
Rusty IDD SHALL distinguish CI isolated Rust state from workstation envctl
toolchain state while keeping both under the meta root.

#### Scenario: GitHub Actions uses isolated layout
- **GIVEN** `scripts/ci/envctl-rust-env.sh` runs under GitHub Actions
- **WHEN** no explicit `RUSTY_IDD_RUST_LAYOUT` is provided
- **THEN** it SHALL set `RUSTUP_HOME` and `CARGO_HOME` under
  `$META_ROOT/.env/rust`.

#### Scenario: Local envctl toolchains layout is selected
- **GIVEN** a local workstation has
  `$META_ROOT/.toolchains/cargo/bin/rustup` and
  `$META_ROOT/.toolchains/rustup`
- **WHEN** `scripts/ci/envctl-rust-env.sh` runs without an explicit layout
- **THEN** it SHALL set `RUSTUP_HOME=$META_ROOT/.toolchains/rustup`,
  `CARGO_HOME=$META_ROOT/.toolchains/cargo`, and
  `RUSTY_IDD_RUST_BIN=$CARGO_HOME/bin`.

#### Scenario: Layout is reported
- **GIVEN** Rusty IDD activates a Rust toolchain layout
- **WHEN** the activation output is printed
- **THEN** it SHALL include the selected layout and the actual `rustc` and
  `cargo` paths used by rustup.

### Requirement: User-global-looking symlinks are resolved by real path
Rusty IDD SHALL judge Rust ownership by resolved paths, not by the visible
wrapper path alone.

#### Scenario: Home cargo command resolves into meta
- **GIVEN** `~/.cargo/bin/cargo` is a symlink to a meta-owned rustup binary
- **WHEN** Rusty IDD audits the active Rust toolchain
- **THEN** the audit SHALL accept it only if the resolved compiler, cargo,
  rustup home, cargo home, cache, and linker paths are under `META_ROOT`.
