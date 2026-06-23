# Rusty IDD Codex Environment

This repository uses Codex-native control surfaces to make agent work repeatable:
repo guidance, skills, project config, rules, hooks, custom agents, and
generated knowledge artifacts. The harness follows Rusty IDD's intent-driven
flow; `AI_MERGE/` is a tool/evidence surface, not Rusty IDD's main intent or
control plane.

The always-on harness must stay small. Stage-specific capability belongs in
Rusty IDD workflow packages generated with `rusty-idd harness package`, not in a
growing `.codex`, `.claude`, `.kimi`, or `.agents` toolbox.

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
| Workflow hooks | `.codex/hooks.json` | Runs Rusty IDD workflow checks before/after write-capable tool use and at turn/subagent stop. |
| Custom agents | `.codex/agents/*.toml` | Read-heavy explorer/gap/verifier agents and one writer agent. |
| Model loop config | `.codex/loops/rusty-idd-model-loop.toml` | Defines read-only, design-first Codex passes across model roles. |
| Codex env CLI | `rusty-idd codex env-check` | Rust-native checks for required artifacts and forbidden regressions. |
| Workflow check CLI | `rusty-idd codex workflow-check` | Rust-native checks for active change, task-card evidence, validation evidence, and PR handoff evidence. |
| Runtime audit CLI | `rusty-idd codex runtime-audit` | Rust-native proof that repo-local Codex hooks, agents, loops, and targets do not depend on Python. |
| System audit CLI | `rusty-idd codex system-audit` | Rust-native proof that the active Codex binary and parent-managed source-build path are Rust-first. |
| Model loop CLI | `rusty-idd codex model-loop` | Rust-native command that emits or executes exact read-only `codex exec` commands for the default loop. |
| Harness package CLI | `rusty-idd harness package` | Rust-native package creation for stage-scoped roles, contracts, tools, helpers, hooks, gates, and evidence schemas. |
| Merge tool package | `rusty-idd merge-tools show` | Rusty IDD-owned package for merge-goal inventory, planning, implementation, verification, evidence, and legacy-surface disposition. |

## Intentional Exclusions

- `AI_MERGE/` is excluded as the default intent/control-plane source. Rusty IDD
  may still read or write AI_MERGE records for audit notes, migration history,
  rollback, and merge evidence when a workflow step requires them.
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
- Plugins are a distribution surface for sharing skills, apps, and MCP bundles.
  They are not the default answer to tool overflow. Rusty IDD first creates or
  selects a stage-scoped harness package; plugin or MCP use must be declared by
  that package as a narrow feature-gated dependency.
- Provider credentials, model providers, notification commands, telemetry, and
  host tool installation remain user/admin or parent `meta`/`envctl` concerns.

## Task-Scoped Package Growth

Codex agents are responsible for the quality of their own output. When a miss,
slow path, stale artifact, weak verification, or repeated manual step shows that
additional task-specific support would materially improve accuracy or speed, the
agent must first create or select the right Rusty IDD harness package instead of
adding another always-loaded skill, hook, rule, or custom agent.

The package is the scoped unit of capability: contracts, stage tools, helpers,
hooks, validation gates, evidence schema, and agent roles. Adapter directories
such as `.codex`, `.claude`, `.kimi`, and `.agents` may launch or reference the
package, but they must not become the source of truth for package content.

Allowed package dependencies include repo skills, project rules, hooks, custom
agents, model-loop passes, local Rust helpers, plugin packaging, and
feature-gated MCP or cloud/vector helpers. Every dependency must preserve the
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

## Rusty IDD Flow

The default harness order is:

1. Capture the user goal.
2. Refresh or read `.idd/knowledge/*`.
3. Generate graph-backed plan context.
4. Create or select the stage-scoped harness package, starting with:

```bash
cargo run --bin rusty-idd -- harness package --stage scan --target .
```

5. Create or select an OpenSpec change.
6. Write proposal, spec deltas, design, ADR decisions, and tasks in schema order.
7. Use `rusty-idd spec status` and `rusty-idd spec next` to gate execution.
8. Implement only after the artifacts are ready and implementation is
   explicitly authorized.
9. Validate, regenerate deterministic artifacts, and record optional AI_MERGE
   evidence when audit or merge records are required.

For merge, migration, or repository-unification goals, use the Rusty IDD-owned
package before reading retired bridge material:

```bash
cargo run --bin rusty-idd -- merge-tools show
```

The package replaces the old active `idd-merge-idd` Claude/Gemini workflow with
Rust-native phases: inventory, plan, decide, implement, verify, and evidence.
Claude/Gemini files may exist only as thin bridge notes; they are not the source
of truth for merge execution.

## Adopt-First Policy

The default integration path is:

1. Inventory the current repo and upstream surface.
2. Adopt or vendor the useful surface intact enough to compile.
3. Build and audit that adopted baseline.
4. Cut only concrete friction: compile conflicts, audit-denied dependencies,
   incompatible runtime versions, or out-of-scope daemon/host surfaces.
5. Record every durable boundary decision in an ADR and every audit/merge
   evidence item in the appropriate evidence surface, including AI_MERGE when
   the workflow calls for it.
6. Refresh `.idd/knowledge` and `.idd/MANIFEST.tsv`.

This prevents the failure mode where an agent cuts too much early, then rebuilds
local substitutes from guesses.

## Subagent Workflow

Use subagents explicitly, not automatically.

- `rusty-idd-explorer`: read-only repository and upstream inventory.
- `rusty-idd-gap-hunter`: read-only omission and regression search.
- `rusty-idd-verifier`: read-only evidence and gate verification.
- `rusty-idd-implementer`: the single writer for implementation slices after
  OpenSpec status shows the change is ready.

Keep parallel work read-heavy unless one integration branch/worktree owner
coordinates writes.

## Multi-Model Loop

The repo includes a read-only, design-first model loop definition:

```bash
cargo run --bin rusty-idd -- codex model-loop
```

The runner reads `.codex/loops/rusty-idd-model-loop.toml` and emits JSON lines
containing the exact `codex exec` commands for each pass. It defaults to dry-run
mode and writes non-deterministic run manifests under
`.idd/runs/rusty-idd-codex-loop`, which is ignored as local execution evidence.

Use `--execute` only when you intentionally want to run the loop:

```bash
cargo run --bin rusty-idd -- codex model-loop --execute
```

The default passes are read-only. Cheap read-heavy discovery uses
`gpt-5.5-mini`; the final verification pass stays on `gpt-5.5` for higher
confidence before implementation or handoff.

- `explore`: fast read-only scan with `gpt-5.5-mini`.
- `gap-hunt`: focused read-only audit with `gpt-5.5-mini`.
- `verify`: high-reasoning read-only verification with `gpt-5.5`.

Write-capable implementation is intentionally outside the default loop. Use a
separate explicitly authorized implementation pass only after OpenSpec artifacts
are ready.

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

## Autonomous Workflow Hooks

The hook surface is git-root anchored so it works even when Codex starts from a
subdirectory. PreToolUse and PostToolUse run around Bash/apply_patch-style tool
use, while Stop and SubagentStop run before a turn ends.

The workflow hook command is:

```bash
sh -lc 'root="$(git rev-parse --show-toplevel)"; exec cargo run --quiet --manifest-path "$root/Cargo.toml" --bin rusty-idd -- codex workflow-check --workspace "$root" --phase pre-tool'
```

For write-capable tool use, `codex workflow-check` requires:

- a feature branch in a worktree based on `develop`;
- `.idd/knowledge/plan-context.md`;
- `.idd/workflow/active-change` or `RUSTY_IDD_CHANGE`;
- a ready OpenSpec change with proposal, specs, design, ADR, and tasks;
- task-card evidence in `.handoff/tasks/*.task.json` or
  `.idd/evidence/autonomous-workflow/task.md`.

For push or task-completion commands such as `git push`, `gh pr create`,
`gh pr merge`, `hf done`, `handoff done`, `task complete`, or `task done`, the
hook requires validation evidence before the command proceeds.

At Stop/SubagentStop, once the branch has dirty work or commits beyond
`develop`, the hook also requires:

- `.idd/evidence/autonomous-workflow/validation.md` with build, generated
  artifacts, test, lint, secret-scan, and manifest results, with `Test:` listed
  after `Generated artifacts:`. Each required result must be success-like
  (`passed`, `completed`, `refreshed`, `clean`, `no matches`, `0 critical`, or
  similar); failed, skipped, stale, missing, unknown, placeholder, or not-run
  evidence does not satisfy the gate. The validation evidence must also include
  `Change: <active-change>` so older task evidence cannot unlock a new push,
  PR, merge, or task-completion command;
- `.idd/evidence/autonomous-workflow/pr.md` with `Change: <active-change>`,
  `Branch: <current-branch>`, the PR, `Base: develop`, and enabled auto-merge
  status.

The invariant Stop hook remains registered as a second Stop handler:

```bash
sh -lc 'root="$(git rev-parse --show-toplevel)"; exec cargo run --quiet --manifest-path "$root/Cargo.toml" --bin rusty-idd -- codex env-check --workspace "$root"'
```

Each hook timeout is 180 seconds. That keeps normal stops bounded while allowing
Cargo to wait briefly on build locks or rebuild the small CLI slice when needed.
If Codex reports that a hook is skipped instead of failed, review and trust the
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
artifacts, LTO disabled for local workstation builds, and the parent-managed
Rust linker/cache contract when present.

Python still appears in upstream Codex source material, but the audit classifies
it as developer/package tooling:

- upstream `justfile` uses Python as its recipe shell and calls
  `scripts/format.py`;
- upstream CI/helper scripts under `.github/scripts` and
  `tools/argument-comment-lint` use Python;
- upstream npm/Python package publishing surfaces use Python scripts and a
  wheel-only `sdk/python-runtime`.

Those surfaces do not contradict the active runtime being Rust-native.

## Rust Toolchain And Cache Ownership

Rusty IDD Rust builds must use the parent `meta` / `envctl` Rust environment,
not user-global or system-depth state. A compliant activation owns all mutable
Rust toolchain and cache state under the meta root.

Rusty IDD recognizes two meta-owned Rust layouts:

- `isolated`: CI/bootstrap state under `$META_ROOT/.env/rust`.
- `toolchains`: local workstation envctl state under
  `$META_ROOT/.toolchains/{cargo,rustup}`.

GitHub Actions defaults to `isolated`. Local activation defaults to
`toolchains` when `$META_ROOT/.toolchains/cargo/bin/rustup` and
`$META_ROOT/.toolchains/rustup` exist. Operators may override the layout with
`RUSTY_IDD_RUST_LAYOUT=isolated` or `RUSTY_IDD_RUST_LAYOUT=toolchains`.

The CI isolated layout uses:

```text
RUSTUP_HOME=$META_ROOT/.env/rust/rustup
CARGO_HOME=$META_ROOT/.env/rust/cargo
RUSTC_WRAPPER=$META_ROOT/.env/rust/bin/kache
# audit cache root: $META_ROOT/.cache/rust/kache
```

The local envctl toolchains layout uses:

```text
RUSTUP_HOME=$META_ROOT/.toolchains/rustup
CARGO_HOME=$META_ROOT/.toolchains/cargo
RUSTY_IDD_RUST_BIN=$META_ROOT/.toolchains/cargo/bin
```

User-global-looking wrappers such as `~/.cargo/bin/cargo` are acceptable only
when their resolved real path is under `META_ROOT`. Audits must report the
actual Cargo-executed `rustc` and `cargo` paths, not just the visible wrapper
or channel label.

The target compiler surface is nightly Rust with `rustc_codegen_gcc` available
as a runtime backend. The target linker is parent-managed `wild-linker`; mold is
not the compliant linker for this workflow. The compiler cache preference order
is `kache`, then `hurry` or `zccache`. The strict audit can classify
`sccache` only as a separately approved last-resort fallback at version
`0.15.0` or newer with UDS/unix socket transport, but the checked-in CI
bootstrap does not fall back to sccache.

Strict audit mode reports the actual Cargo-executed compiler path and rejects
non-meta-owned paths:

```bash
cargo run --bin rusty-idd -- codex system-audit \
  --rust-toolchain \
  --meta-root .. \
  --rust-toolchain-name "$RUSTUP_TOOLCHAIN" \
  --codegen-backend rustc_codegen_gcc \
  --cache-root "$META_ROOT/.cache/rust/kache" \
  --linker-path "$META_ROOT/.env/rust/bin/wild"
```

The parent `meta` / `envctl` layer is responsible for installing or updating
nightly Rust, `rustc_codegen_gcc`, `wild-linker`, `kache`, `hurry`, `zccache`,
or any separately approved fallback cache tool. Agents working in this
repository must not repair a missing Rust tool by writing to `~/.rustup`,
`~/.cargo`, `/usr/bin`, `/opt`, or other user/system-owned locations.

GitHub workflows call `scripts/ci/envctl-rust-env.sh` to materialize the
meta-owned `RUSTUP_HOME`, `CARGO_HOME`, cache root, `wild`, and kache-compatible
wrapper before running Cargo. Local workstation validation can call the same
script with `RUSTY_IDD_RUST_LAYOUT=toolchains` or rely on auto-detection when
the envctl `.toolchains` state exists. Primary CI and promotion verification
then call `scripts/ci/envctl-rust-audit.sh`, which reports the actual `rustc`
and `cargo` paths used by Cargo and rejects the sccache path for this
implementation.

Workflow caches must resolve the parent meta root to a canonical path before
restore/save. Do not pass raw `../meta` paths to `actions/cache`; GitHub rejects
relative parent segments, which prevents the envctl Rust cache from being saved
even though the workflow appears to request the right files. Cache parent
`.env/rust` and `.cache/rust` tool state separately from workspace `target` so
Cargo.lock changes do not invalidate the toolchain/tool cache.

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
rusty-idd spec status openspec/changes/<change>
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
