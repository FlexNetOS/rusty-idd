# Env Vault Relay

## Purpose
Define the Rusty IDD automation contract for recording environment and vault
relay readiness without making host/vault behavior part of default workflows.
## Requirements
### Requirement: Integrate Environment and vault relay
Rusty IDD SHALL integrate capability `capability:env-vault-relay` through the documented owner repos while preserving adopt-first evidence and deterministic validation.

#### Scenario: Adopt-first evidence is recorded
- **GIVEN** the selected integration work item lists owner repos, anchors, and adopt-first inputs
- **WHEN** implementation begins
- **THEN** native diagnostics and current owner/upstream evidence SHALL be recorded before any consolidation cut.
- **AND** Rusty IDD SHALL generate deterministic readiness artifacts that preserve the native diagnostic command expectations and observed toolchain assumptions.

#### Scenario: Readiness records parent-managed tools
- **GIVEN** an integration owner requires tools such as Cargo, envctl, Kasetto, Nix, Nushell, Lua, Ghostty, Zellij, Beads, or Cognitum vault access
- **WHEN** `rusty-idd knowledge integration-readiness --workspace . --next` runs
- **THEN** Rusty IDD SHALL record each required tool, the owner or anchor that requires it, and the parent `meta`/`envctl` or feature-gated runtime surface that must provide it.

#### Scenario: Readiness stays read-only
- **GIVEN** the selected work item includes host/vault anchors such as `/run/media/drdave/COGNITUM` and `Cognitum vault on Pi Zero`
- **WHEN** readiness artifacts are generated
- **THEN** Rusty IDD SHALL NOT probe the vault path, mint relay credentials, mutate peer repos, install tools globally, start services, or manage daemon state.

#### Scenario: Thin Rusty IDD boundary is implemented
- **GIVEN** upstream or owner repo behavior is proven through diagnostics
- **WHEN** Rusty IDD wires the capability
- **THEN** the local boundary SHALL be limited to DTO mapping, deterministic output, feature flags, validation, size/token policy, and CLI/API calls.

#### Scenario: Validation gates protect the integration
- **GIVEN** the implementation and generated artifacts are complete
- **WHEN** the integration is proposed for merge
- **THEN** focused tests, affected smoke tests, Rusty IDD validation, and full gates SHALL pass or the change SHALL remain unmerged.

