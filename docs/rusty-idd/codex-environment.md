# Rusty IDD Codex Environment

This repository uses Codex-native control surfaces to make agent work repeatable:
repo guidance, skills, project config, rules, hooks, custom agents, and
generated knowledge artifacts.

The setup was checked against the current Codex manual fetched on 2026-06-21.
The manual does not define a canonical "7 layer" or "8 layer" Codex model.
Configuration precedence has six layers: CLI overrides, trusted project
`.codex/config.toml`, selected profile config, user config, system config, and
built-in defaults. Repo behavior is broader than config precedence; the
project-control surfaces below are the intentionally tracked Rusty IDD build.

## Surfaces

| Surface | Path | Purpose |
| --- | --- | --- |
| Project instructions | `AGENTS.md` | Durable repo rules and the adopt-first integration policy. |
| Knowledge workflow | `.agents/skills/rusty-idd-knowledge/SKILL.md` | Use generated graph/report artifacts before broad rescans. |
| Adopt-first workflow | `.agents/skills/rusty-idd-adopt-first/SKILL.md` | Adopt upstream/current state before cutting. |
| Codex env workflow | `.agents/skills/rusty-idd-codex-rust-env/SKILL.md` | Operate skills, hooks, agents, knowledge, and gates. |
| Project config | `.codex/config.toml` | Enables hooks and caps subagent fan-out. |
| Project rules | `.codex/rules/default.rules` | Blocks raw host/process management and prompts for untracked tool installs. |
| Stop hook | `.codex/hooks.json` | Runs the Rusty IDD invariant checker from the git root when a turn stops. |
| Custom agents | `.codex/agents/*.toml` | Read-heavy explorer/gap/verifier agents and one writer agent. |
| Model loop config | `.codex/loops/rusty-idd-model-loop.toml` | Defines dry-run-first Codex passes across model roles. |
| Codex env CLI | `rusty-idd codex env-check` | Rust-native checks for required artifacts and forbidden regressions. |
| Runtime audit CLI | `rusty-idd codex runtime-audit` | Rust-native proof that repo-local Codex hooks, agents, loops, and targets do not depend on Python. |
| System audit CLI | `rusty-idd codex system-audit` | Rust-native proof that the active Codex binary and parent-managed source-build path are Rust-first. |
| Model loop CLI | `rusty-idd codex model-loop` | Rust-native command that emits or executes exact `codex exec` commands. |

## Intentional Exclusions

- MCP is excluded from the default knowledge integration path; this feature uses
  direct Rust crate integration to avoid context rot and transport overhead.
  MCP may still be added as a narrow helper surface when evidence shows it will
  improve Codex output accuracy, speed, verification, or repeatability.
- `tree-sitter` is active in the Rusty IDD knowledge path through the vendored
  CodeGraph parser registry, and active in the wider system through Yazelix.
  Default workflow exclusions only apply to host-service, daemon, or transport
  management, not to parser availability.
- Custom prompts are deprecated by Codex in favor of skills and are not
  repo-shared through `.codex`.
- Memories are user/app state, not the authoritative repository rule surface.
- Plugins are a distribution surface for sharing skills, apps, and MCP bundles;
  this repo keeps local skills directly under `.agents/skills` until packaging
  is justified by reuse, installability, or output-quality evidence.
- Provider credentials, model providers, notification commands, telemetry, and
  host tool installation remain user/admin or parent `meta`/`envctl` concerns.

## Agent-Owned Tool Growth

Codex agents are responsible for the quality of their own output. When a miss,
slow path, stale artifact, weak verification, or repeated manual step shows that
an additional Codex surface would materially improve accuracy or speed, the
agent must decide and add the narrowest tracked tool rather than waiting for the
user to name it.

Allowed growth surfaces include repo skills, project rules, hooks, custom
agents, model-loop passes, local Rust helpers, plugin packaging, and
feature-gated MCP or cloud/vector helpers. Every addition must preserve the
repo boundary, avoid user-global installs, document the reason, and pass the
relevant gates.

## Upgrade-Only Gap Handling

Treat stale or orphaned work as unfinished unless there is evidence that it is
an intentionally local ignored artifact. Do not downgrade a working surface,
dependency, model, action, skill, hook, or generated artifact to make a task
easier. Finish the stale/orphaned work, document the decision, regenerate the
affected artifacts, and verify the result.

Missing command-line tools are not fixed with user-global installs. If a gate
needs a tool such as `just`, add or repair the tracked parent `meta` / `envctl`
tool surface first, or use an existing repo-local equivalent and record the
managed-tooling gap.

## Adopt-First Policy

The default integration path is:

1. Inventory the current repo and upstream surface.
2. Adopt or vendor the useful surface intact enough to compile.
3. Build and audit that adopted baseline.
4. Cut only concrete friction: compile conflicts, audit-denied dependencies,
   incompatible runtime versions, or out-of-scope daemon/host surfaces.
5. Record every cut in an ADR or `AI_MERGE` note.
6. Refresh `.idd/knowledge` and `.idd/MANIFEST.tsv`.

This prevents the failure mode where an agent cuts too much early, then rebuilds
local substitutes from guesses.

## Subagent Workflow

Use subagents explicitly, not automatically.

- `rusty-idd-explorer`: read-only repository and upstream inventory.
- `rusty-idd-gap-hunter`: read-only omission and regression search.
- `rusty-idd-verifier`: read-only evidence and gate verification.
- `rusty-idd-implementer`: the single writer for implementation slices.

Keep parallel work read-heavy unless one integration branch/worktree owner
coordinates writes.

## Multi-Model Loop

The repo includes a dry-run-first model loop definition:

```bash
cargo run --bin rusty-idd -- codex model-loop
```

The runner reads `.codex/loops/rusty-idd-model-loop.toml` and emits JSON lines
containing the exact `codex exec` commands for each pass. It defaults to dry-run
mode and writes its manifest under `/tmp/rusty-idd-codex-loop`.

Use `--execute` only when you intentionally want to run the loop:

```bash
cargo run --bin rusty-idd -- codex model-loop --execute
```

The default passes are:

- `explore`: fast read-only scan with `gpt-5.4-mini`.
- `gap-hunt`: high-reasoning read-only audit with `gpt-5.5`.
- `verify`: high-reasoning read-only verification with `gpt-5.5`.
- `implement`: single workspace-write pass with `gpt-5.5`.

Provider credentials, model availability, and account policy stay in user/admin
Codex config. The repository only defines the workflow shape.

## Runtime Language Audit

The repo-local Codex hot path is intentionally Rust-first:

```bash
cargo run --bin rusty-idd -- codex runtime-audit
```

The audit classifies Python mentions instead of treating every occurrence as
runtime evidence. A clean result means:

- `.codex/hooks.json`, `.codex/agents`, `.codex/loops`, `Justfile`, and
  `Makefile` do not call Python.
- obsolete `.codex/hooks/*.py` and `.codex/scripts/*.py` tools are absent.
- remaining Python references are parser/language support, test fixtures,
  policy text, or documentation.

Generated dependency build output under `target/**` is ignored because it is not
repo-owned Codex control-plane code.

## Stop Hook

The Stop hook is git-root anchored so it works even when Codex starts from a
subdirectory:

```bash
sh -lc 'root="$(git rev-parse --show-toplevel)"; exec cargo run --quiet --manifest-path "$root/Cargo.toml" --bin rusty-idd -- codex env-check --workspace "$root"'
```

The hook timeout is 180 seconds. That keeps normal stops bounded while allowing
Cargo to wait briefly on build locks or rebuild the small CLI slice when needed.
If Codex reports that the hook is skipped instead of failed, review and trust the
changed hook definition with `/hooks`; Codex records trust by hook hash.

## System Language Audit

Use the system audit when the question is broader than this repo:

```bash
cargo run --bin rusty-idd -- codex system-audit \
  --codex-source ../codex \
  --envctl ../envctl
```

The audit checks the active `codex` binary first. On this workstation, the
managed path resolves to the source-built Rust executable under
`../codex/codex-rs/target/release/codex`. The parent `envctl` component builds
that executable directly with Cargo, high-parallel jobs, release incremental
artifacts, LTO disabled for local workstation builds, and `mold` when present.

Python still appears in upstream Codex source material, but the audit classifies
it as developer/package tooling:

- upstream `justfile` uses Python as its recipe shell and calls
  `scripts/format.py`;
- upstream CI/helper scripts under `.github/scripts` and
  `tools/argument-comment-lint` use Python;
- upstream npm/Python package publishing surfaces use Python scripts and a
  wheel-only `sdk/python-runtime`.

Those surfaces do not contradict the active runtime being Rust-native.

## Tool Installation

Do not install required tools into user-global settings from this repository.
Shared tools such as `just` must be added to the parent `meta`/`envctl` tool
contract so they are tracked, reproducible, and available per repo. This repo
therefore exposes both Make and Rust-native command paths; `just` targets are
convenience wrappers only when the parent toolchain provides `just`.

## Commands

```bash
just codex-env-check
cargo run --bin rusty-idd -- codex runtime-audit
cargo run --bin rusty-idd -- codex system-audit
cargo run --bin rusty-idd -- codex model-loop
just knowledge
just manifest
just validate
```

Use the full gate when claiming repo-wide completion:

```bash
just ci
```

## Boundaries

This is a repository environment, not host management. Do not use raw
`systemctl`, daemon kills, user binary installs, or other OS service operations
to make repository tasks pass.
