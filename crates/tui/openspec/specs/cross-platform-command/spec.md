## REMOVED Requirements

### Requirement: Platform-aware openspec command construction
The function that constructed a `std::process::Command` for invoking the external `openspec` CLI is removed.

**Reason**: Active change listing and artifact status are now derived from local OpenSpec files owned by rusty-idd, so the TUI no longer needs a Node/OpenSpec runtime binary on PATH.
**Migration**: `list_changes()` reads `openspec/changes/`; `get_change_status()` checks generated artifact files under the selected change directory.

### Requirement: All openspec CLI calls use the shared command constructor
The shared `openspec_command()` constructor requirement is removed because no active TUI data path shells out to the external `openspec` CLI.

**Reason**: The Rust-native lifecycle engine and filesystem-backed TUI data layer replace the legacy subprocess contract.
**Migration**: Tests should verify local file discovery and artifact-status detection rather than command construction.

## REMOVED Requirements

### Requirement: Platform-aware claude command construction
The `claude_command()` function that constructs a hardcoded `Command::new("claude")` is removed. The runner now constructs commands from the configurable command template instead.

**Reason**: Replaced by the config-driven command template system in the `tui-configuration` capability.
**Migration**: The runner uses `TuiConfig.command` with whitespace splitting instead of `claude_command()`.
