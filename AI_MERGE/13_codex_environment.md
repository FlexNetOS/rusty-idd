# Codex Environment Record

## Intent

Build a repo-local Codex environment for Rusty IDD that preserves current
working behavior, learns from the premature-cut failure, and gives future agents
durable skills, hooks, and subagent roles.

This file is an AI_MERGE evidence note. It is not the Rusty IDD control plane.
Rusty IDD's workflow starts with user intent, graph-backed `.idd/knowledge`
artifacts, OpenSpec proposal/spec/design/ADR/tasks, validation, and optional
AI_MERGE evidence when audit or merge records are needed.

## Implemented Surfaces

- `AGENTS.md` now records the Codex environment rules and adopt-first policy.
- `.codex/config.toml` enables project hooks and caps subagent fan-out.
- `.codex/rules/default.rules` blocks raw host/process management and prompts
  for untracked global tool installation.
- `.codex/hooks.json` registers a git-root anchored Stop hook.
- `rusty-idd codex env-check` verifies required artifacts and known regression
  markers.
- `.codex/agents/` defines read-only explorer, verifier, gap-hunter, and one
  workspace-write implementer.
- `.agents/skills/rusty-idd-adopt-first/` captures the integration workflow.
- `.agents/skills/rusty-idd-codex-rust-env/` captures the Codex/Rust operating
  workflow.
- `.codex/loops/rusty-idd-model-loop.toml` defines a read-only, design-first
  multi-model Codex loop.
- `rusty-idd codex model-loop` emits or executes exact read-only `codex exec`
  commands for that default loop.
- `rusty-idd codex runtime-audit` classifies Python mentions and fails if
  repo-local Codex hooks, agents, loops, or targets depend on Python runtime
  commands.
- `rusty-idd codex system-audit` checks the active Codex binary and optional
  parent `codex`/`envctl` roots to distinguish Rust runtime from upstream
  Python developer/package tooling.
- `docs/rusty-idd/codex-environment.md` documents the environment.

## Research Basis

Checked the current Codex manual on 2026-06-21 for:

- Agent Skills
- `AGENTS.md`
- project `.codex/config.toml`
- project `.codex/rules`
- hooks
- custom agents and subagents
- non-interactive `codex exec`

The manual does not define a canonical "7 layer" or "8 layer" Codex model.
Configuration precedence is six layers: CLI overrides, trusted project config,
selected profile config, user config, system config, and built-in defaults.
Rusty IDD's repo-local behavior build now tracks project guidance, repo skills,
project config, project rules, hooks, project custom agents, the model loop,
and generated knowledge artifacts.

Owner correction on 2026-06-21: Codex owns its output quality and must decide
when additional tool surfaces are justified by lessons learned. MCP, plugin
packaging, vector/cloud helpers, local Rust helpers, hooks, rules, skills, and
custom agents are not blocked merely because the user did not name them. The
agent should add the narrowest tracked tool when evidence shows it will improve
accuracy, speed, verification, or repeatability. Provider credentials, model
providers, notification commands, telemetry, host service management, and
user-global tool installation remain user/admin or parent `meta`/`envctl`
concerns.

Owner correction on 2026-06-20: review passes are upgrade-only. Do not downgrade
a working surface to simplify a task. Treat stale or orphaned work as unfinished
unless evidence proves it is intentionally local and ignored; finish it,
document it, regenerate affected artifacts, and verify the result.

Tooling is part of the control plane. Missing binaries needed for this repo
must be added through tracked parent `meta` / `envctl` provisioning, not by
mutating user-global state. Until the parent tool is present, use only a tracked
repo-local equivalent and keep the gap visible.

## Verification Plan

```bash
just codex-env-check
cargo run --bin rusty-idd -- codex runtime-audit
cargo run --bin rusty-idd -- codex system-audit --codex-source ../codex --envctl ../envctl
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo audit --deny warnings
cargo run --bin rusty-idd -- knowledge refresh --workspace .
cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv
cargo run --bin rusty-idd -- validate --workspace .
cargo run --bin rusty-idd -- codex model-loop
```

## Verification Results

- `cargo run --bin rusty-idd -- codex env-check`: passed.
- `make codex-env-check`: passed.
- `cargo run --bin rusty-idd -- codex env-check`: parsed hook JSON plus project config, agent, and loop TOML successfully.
- Stop hook hardening: `.codex/hooks.json` now resolves the git root, runs
  Cargo with `--manifest-path`, passes `--workspace` explicitly, and uses a
  180-second timeout to avoid false failures from subdirectory launches or
  short Cargo build-lock waits.
- `cargo run --bin rusty-idd -- codex runtime-audit`: passed and reported zero
  live Codex Python commands and zero obsolete Python Codex tool files.
- `cargo run --bin rusty-idd -- codex system-audit --codex-source ../codex --envctl ../envctl`:
  passed; active `codex` resolved to a native ELF source build under
  `../codex/codex-rs/target/release/codex`, envctl uses direct high-parallel
  Cargo source builds with mold and Bun fallback, and upstream Python was
  classified as developer/package tooling.
- `cargo fmt --all -- --check`: passed.
- `cargo build --workspace --locked`: passed.
- `cargo test --workspace --locked`: passed, 584 tests passed and 3 vendored watcher timing tests ignored.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed.
- `cargo audit --deny warnings`: passed.
- `cargo run --bin rusty-idd -- knowledge refresh --workspace .`: passed.
- `cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv`: passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed with 0 critical and 0 warning.
- `cargo run --bin rusty-idd -- codex model-loop`: passed in dry-run mode and emitted four `codex exec` pass commands.
- `make codex-model-loop`: passed in dry-run mode.
- `codex execpolicy check --rules .codex/rules/default.rules -- systemctl status env-ctl.service`: returned `forbidden`.
- `codex execpolicy check --rules .codex/rules/default.rules -- cargo install just`: returned `prompt`.

`just codex-env-check` could not run in this environment because `just` is not
installed. Per owner correction, that should be fixed through the parent
`meta`/`envctl` tool contract, not user-global installation. The equivalent Make
target and Rust-native CLI command passed.
