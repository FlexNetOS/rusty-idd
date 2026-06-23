# Graph Planning Context

- Goal: # improve-ci-bootstrap-observability

## Goal

Continue the Rusty IDD self-upgrade loop from the envctl toolchain contract
evidence. PR #103 and PR #104 proved the strict CI Rust path is correct, but
the full `rust` job spent 11-14 minutes in strict bootstrap and full gates.

Improve the CI bootstrap contract without weakening the required toolchain:
nightly Rust, `rustc_codegen_gcc`, parent-managed `wild-linker`, and
meta/envctl-owned `kache` remain required. Do not introduce `sccache`.

## Required Change

- Split GitHub Actions caching so meta-owned Rust toolchain/tool state is cached
  independently from workspace `target` artifacts.
- Keep all Rust mutable state under the parent meta/envctl root.
- Add grouped/timed bootstrap logging around rustup, codegen component, cache
  wrapper, wild-linker, and cargo-audit installation checks.
- Update the Rusty IDD model-loop cheap read-heavy passes from the stale cheap
  model surface to `gpt-5.5-mini`, then run the model-loop surface as workflow
  evidence.

## Evidence

- PR #103 CI full `rust` job passed in 11m10s.
- PR #104 CI full `rust` job passed in 14m9s.
- PR #104 strict CI environment reported:
  - `layout: isolated`
  - `RUSTUP_HOME=/home/runner/work/rusty-idd/meta/.env/rust/rustup`
  - `CARGO_HOME=/home/runner/work/rusty-idd/meta/.env/rust/cargo`
  - `rustc=/home/runner/work/rusty-idd/meta/.env/rust/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc`
  - `cargo=/home/runner/work/rusty-idd/meta/.env/rust/rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/cargo`
  - `RUSTC_WRAPPER=/home/runner/work/rusty-idd/meta/.env/rust/bin/kache`
  - `RUSTFLAGS=-C linker=clang -C link-arg=-fuse-ld=/home/runner/work/rusty-idd/meta/.env/rust/bin/wild`

## Success Criteria

- CI, promotion, and release workflows have separate envctl Rust tool/cache
  cache keys that do not depend on `Cargo.lock`.
- Workspace `target` cache remains keyed by `Cargo.lock`.
- GitHub Actions cache paths use canonical parent meta paths rather than
  rejected `../meta` patterns.
- `scripts/ci/envctl-rust-env.sh` reports timed install/skip spans for strict
  bootstrap stages.
- The repo-local model loop emits `gpt-5.5-mini` for the cheap read-heavy
  `explore` and `gap-hunt` passes.
- OpenSpec, ADR, evidence, generated knowledge, and manifest artifacts are
  refreshed.
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/ci-bootstrap-observability`
- Source graph: 141 files, 8832 nodes, 36415 edges via `codegraph-rust`
- Context package: 356 files, 741052 tokens via `repomix-rs`

## Automation Order

- `Goal intake and bounded context`: turn repo state and user goal into bounded agent context (surface:repomix-rs, surface:openspec)
- `Architecture mapping`: map source structure, integrations, and impact before implementation (surface:codegraph-rust, surface:repomix-rs)
- `Specification and decisions`: convert architecture map into spec deltas, design, ADRs, and tasks (surface:openspec, surface:audit-manifest)
- `Implementation`: apply graph-informed, spec-backed code changes (surface:codegraph-rust, surface:openspec)
- `Validation and regeneration`: run gates and refresh deterministic control-plane artifacts (surface:codegraph-rust, surface:repomix-rs, surface:audit-manifest)
- `Handoff and merge evidence`: record evidence, rollback, and merge-ready traceability (surface:audit-manifest, surface:openspec)

## Integration Surfaces

- `CodeGraph Rust` [architecture_graph]: in-process knowledge indexing. Capabilities: multi-language tree-sitter registry, symbol/import/call/type graph extraction, impact and hotspot evidence
- `repomix-rs` [context_package]: bounded context packing and token policy. Capabilities: compressed context packs, token accounting and top-file metrics, security and suspicious-file signals, git-aware context options
- `Rusty IDD OpenSpec lifecycle` [lifecycle_control_plane]: proposal, spec, design, ADR, task, validation, archive. Capabilities: goal intake, spec delta tracking, ordered implementation tasks, validation and merge evidence
- `Audit and manifest evidence` [evidence_control_plane]: deterministic generated control-plane artifacts. Capabilities: AI_MERGE audit records, ADR traceability, .idd manifest baseline, knowledge artifact freshness

## Focus Components

| Component | Kind | Files | Nodes | Edges | Evidence |
|---|---|---:|---:|---:|---|
| `cli` | crate | 26 | 1000 | 4547 | crates/cli/src/commands/codex.rs, crates/cli/src/commands/core.rs, crates/cli/src/commands/harness.rs, crates/cli/src/commands/knowledge.rs, crates/cli/src/commands/merge_tools.rs, crates/cli/src/commands/mod.rs, crates/cli/src/commands/run.rs, crates/cli/src/commands/spec.rs, crates/cli/src/commands/spec_adr.rs, crates/cli/src/commands/spec_archive.rs, crates/cli/src/commands/spec_plan_integration.rs, crates/cli/src/commands/spec_scaffold.rs |
| `core` | crate | 12 | 448 | 2479 | crates/core/src/cli.rs, crates/core/src/env_contract.rs, crates/core/src/fs_utils.rs, crates/core/src/lib.rs, crates/core/src/manifest.rs, crates/core/src/model.rs, crates/core/src/planner.rs, crates/core/src/scanner.rs, crates/core/src/templates.rs, crates/core/src/validation.rs, crates/core/tests/smoke.rs, crates/core/tests/template_agent_surface.rs |
| `spec` | crate | 24 | 461 | 1969 | crates/spec/src/adr/mod.rs, crates/spec/src/archive/mod.rs, crates/spec/src/lib.rs, crates/spec/src/model/block.rs, crates/spec/src/model/delta.rs, crates/spec/src/model/merge.rs, crates/spec/src/model/mod.rs, crates/spec/src/model/requirement.rs, crates/spec/src/model/spec.rs, crates/spec/src/parse/common.rs, crates/spec/src/parse/delta_parser.rs, crates/spec/src/parse/emit.rs |
| `codegraph-core` | external_crate | 39 | 1918 | 9070 | crates/external/codegraph-core/benches/core_micro.rs, crates/external/codegraph-core/src/advanced_config.rs, crates/external/codegraph-core/src/arena.rs, crates/external/codegraph-core/src/buffer_pool.rs, crates/external/codegraph-core/src/cli_config.rs, crates/external/codegraph-core/src/compression.rs, crates/external/codegraph-core/src/config.rs, crates/external/codegraph-core/src/config_manager.rs, crates/external/codegraph-core/src/embedding_config.rs, crates/external/codegraph-core/src/error.rs, crates/external/codegraph-core/src/incremental/mod.rs, crates/external/codegraph-core/src/incremental/updater.rs |
| `runner` | crate | 4 | 770 | 3135 | crates/runner/src/config.rs, crates/runner/src/data.rs, crates/runner/src/lib.rs, crates/runner/src/runner.rs |
| `repomix-shared` | external_crate | 2 | 11 | 34 | crates/external/repomix-shared/src/lib.rs, crates/external/repomix-shared/src/types.rs |
| `knowledge` | crate | 1 | 639 | 4653 | crates/knowledge/src/lib.rs |
| `merge-tools` | crate | 1 | 45 | 234 | crates/merge-tools/src/lib.rs |
| `codegraph-parser` | external_crate | 29 | 1060 | 7485 | crates/external/codegraph-parser/src/complexity.rs, crates/external/codegraph-parser/src/diff.rs, crates/external/codegraph-parser/src/edge.rs, crates/external/codegraph-parser/src/fast_io.rs, crates/external/codegraph-parser/src/fast_ml/enhancer.rs, crates/external/codegraph-parser/src/fast_ml/mod.rs, crates/external/codegraph-parser/src/fast_ml/pattern_matcher.rs, crates/external/codegraph-parser/src/fast_ml/symbol_resolver.rs, crates/external/codegraph-parser/src/file_collect.rs, crates/external/codegraph-parser/src/integration_tests.rs, crates/external/codegraph-parser/src/language.rs, crates/external/codegraph-parser/src/languages/cpp.rs |
| `tui` | crate | 3 | 1006 | 3985 | crates/tui/src/app.rs, crates/tui/src/lib.rs, crates/tui/src/ui.rs |

## System Roles

- `Agent environment`: Supports agent runtime, skills, prompts, or execution environment
- `Capability hub`: Groups domain capability repos used by the wider system
- `Coordination and domain surface`: Provides orchestration, MCP, and domain-adjacent system coordination surfaces
- `Documentation and knowledge`: Stores documentation and wiki surfaces
- `Domain upgrade surface`: Contributes domain behavior through weave plus Obscura upgrade paths
- `Fleet handoff`: Carries central and fleet handoff state for cross-repo agent continuity
- `Rusty IDD control plane`: Owns OpenSpec, ADR, task, validation, manifest, and graph-driven implementation workflow
- `Knowledge and memory`: Stores memory or knowledge surfaces used by agents
- `Meta control plane`: Provides parent meta workspace inventory and execution surfaces
- `Parser/runtime surface`: Carries parser/runtime support such as tree-sitter through Yazelix
- `Rust code surface`: Contains Rust source that can be indexed by CodeGraph-backed Rusty IDD knowledge
- `Spec producer`: Produces intent or prompt artifacts that Rusty IDD can turn into OpenSpec
- `Toolchain provider`: Provides parent-managed tools instead of user-global installs

## System Repos

| Repo | Branch | Dirty | Roles | Architecture |
|---|---|---|---|---|
| `rusty-idd` | `` | false | role:agent-environment, role:fleet-handoff, role:idd-control-plane, role:rust-code-surface | 139 files, 8697 nodes, 35719 edges; 720698 tokens; surfaces 4; top: codegraph-core, codegraph-parser, knowledge |
| `envctl` | `master` | true | role:fleet-handoff, role:rust-code-surface, role:toolchain-provider |  |
| `ruvector` | `develop` | true | role:agent-environment, role:fleet-handoff, role:rust-code-surface |  |
| `agent` | `main` | false | role:agent-environment, role:fleet-handoff, role:rust-code-surface |  |
| `obscura` | `main` | false | role:agent-environment, role:domain-upgrade-surface, role:rust-code-surface |  |
| `kasetto` | `main` | false | role:agent-environment, role:rust-code-surface |  |
| `lane` | `main` | false | role:fleet-handoff, role:rust-code-surface |  |
| `yazelix` | `main` | false | role:parser-runtime-surface, role:toolchain-provider |  |
| `prompt_hub` | `main` | true | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer |  |
| `weave` | `develop` | false | role:agent-environment, role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface |  |
| `icm` | `fix/containment-claude-p-recursion` | false | role:agent-environment, role:knowledge-memory, role:rust-code-surface |  |
| `flexnetos_runner` | `chore/handoff-tier-a-pilot` | false | role:fleet-handoff, role:rust-code-surface |  |
| `network-control` | `develop` | false | role:fleet-handoff, role:rust-code-surface |  |
| `oh-my-pi` | `main` | false | role:agent-environment, role:rust-code-surface |  |
| `rtk-tokenkill` | `develop` | false | role:agent-environment, role:rust-code-surface |  |
| `atc` | `main` | false | role:agent-environment, role:coordination-domain-surface, role:rust-code-surface |  |
| `meta_git_lib` | `chore/indicatif-0.18-rustsec-2025-0119` | false | role:fleet-handoff, role:meta-control-plane, role:rust-code-surface |  |
| `ECC` | `main` | false | role:agent-environment, role:fleet-handoff |  |
| `loop_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `loop_lib` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |

## Operating Layers

- `Agent runtime`: Agent harnesses, execution workers, and automation runtimes (2 capabilities, 28 repos)
- `Coordination and communication`: Agent communication, orchestration, and cross-agent continuity (3 capabilities, 19 repos)
- `Environment and security`: Vault, key relay, certificates, and parent-managed toolchains (1 capabilities, 3 repos)
- `Executive control plane`: Company-level command, OpenSpec, handoff, and repo governance (2 capabilities, 14 repos)
- `Front door experience`: Prompt, chat, LifeOS, and operator-facing user experience surfaces (2 capabilities, 3 repos)
- `Infrastructure and device fabric`: Network control plus distributed device compute, storage, inference, and memory (2 capabilities, 5 repos)
- `Interface automation`: AR-glasses workflow, local automation, media, and home interfaces (2 capabilities, 3 repos)
- `Knowledge and runtime`: Memory, vector/progress databases, inference, training, and runtime state (1 capabilities, 4 repos)
- `Simulation and validation`: Digital twin simulation and high-fidelity failure space for agents (1 capabilities, 1 repos)
- `Toolchain and parser runtime`: Tree-sitter, Lua, terminal/runtime, parser, and toolchain surfaces (2 capabilities, 7 repos)

## Operating Capabilities

| Capability | Layer | Status | Repos | Anchors |
|---|---|---|---|---|
| `Central and fleet handoff` | `layer:coordination-communication` | partial | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave | handoff central and fleet design |
| `Environment and vault relay` | `layer:environment-security` | partial | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| `Agent harness runtime` | `layer:agent-runtime` | partial | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave | harness-agent-rs rust port |
| `Meta peer repo control` | `layer:executive-control-plane` | partial | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli | meta peer repo system |
| `GitHub agent-run upgrades` | `layer:agent-runtime` | partial | repo:grit, repo:yazelix | GRIT from rtk-ai, Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| `IDD and spec engine` | `layer:executive-control-plane` | partial | repo:handoff, repo:rusty-idd | Rusty IDD built into handoff |
| `Lua and AR interface automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi, repo:yazelix | Lua required for AR glasses workflow, Brilliant Labs Noa style Rust-native agent UX |
| `Parser and terminal runtime` | `layer:toolchain-parser-runtime` | partial | repo:rusty-idd, repo:tool-hub, repo:yazelix | tree-sitter via Yazelix, Yazelix default terminal, nushell, Lua, Ghostty, Zellij |
| `Distributed device fabric` | `layer:infrastructure-device-fabric` | partial | repo:envctl, repo:network-control, repo:oh-my-pi | user devices for distributed compute storage inference memory |
| `RTK AI foundation` | `layer:toolchain-parser-runtime` | partial | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox | RTK from rtk-ai, ICM from rtk-ai, VOX from rtk-ai, GRIT from rtk-ai |
| `Vector and agentic runtime` | `layer:knowledge-runtime` | partial | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector | meta-ruvector full agentic system |
| `Prompt front door` | `layer:front-door-experience` | partial | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt, prompt_hub front door to handoff and rusty-idd |
| `Personal media and home automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi | personal life media TV home automation |
| `Digital twin simulation` | `layer:simulation-validation` | partial | repo:teri | Teri digital twin simulator |
| `User front door` | `layer:front-door-experience` | partial | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration, LifeOS front door |
| `Network engineering and control` | `layer:infrastructure-device-fabric` | partial | repo:lane, repo:network-control, repo:network-hub | lane merges into network-manager |
| `Agent communication layer` | `layer:coordination-communication` | partial | repo:atc, repo:handoff, repo:mcp-hub, repo:weave | weave agent communication layer |
| `Domain upgrade path` | `layer:coordination-communication` | partial | repo:obscura, repo:weave | weave plus Obscura domain upgrades |

## Integration Work

| Priority | Work Item | Change | Owners | Adopt First |
|---:|---|---|---|---|
| 20 | `Integrate Central and fleet handoff` | `integrate-fleet-handoff` | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave |  |
| 40 | `Integrate Environment and vault relay` | `integrate-env-vault-relay` | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| 50 | `Integrate Prompt front door` | `integrate-prompt-front-door` | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt |
| 60 | `Integrate RTK AI foundation` | `integrate-rtk-ai-foundation` | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox |  |
| 70 | `Integrate GitHub agent-run upgrades` | `integrate-github-agent-run-upgrades` | repo:grit, repo:yazelix | Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| 80 | `Integrate Parser and terminal runtime` | `integrate-parser-runtime` | repo:rusty-idd, repo:tool-hub, repo:yazelix |  |
| 90 | `Integrate Vector and agentic runtime` | `integrate-vector-runtime` | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector |  |
| 130 | `Integrate Distributed device fabric` | `integrate-distributed-device-fabric` | repo:envctl, repo:network-control, repo:oh-my-pi |  |
| 140 | `Integrate Lua and AR interface automation` | `integrate-lua-ar-interface` | repo:lifeos, repo:oh-my-pi, repo:yazelix |  |
| 150 | `Integrate Personal media and home automation` | `integrate-personal-automation` | repo:lifeos, repo:oh-my-pi |  |
| 500 | `Integrate Agent harness runtime` | `integrate-agent-harness` | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave |  |
| 500 | `Integrate Domain upgrade path` | `integrate-domain-upgrade` | repo:obscura, repo:weave |  |

## Planning Guidance

- Use proposal.md to bind the goal to graph-backed scope before implementation
- Use specs/*/spec.md to express externally visible behavior and integration contracts
- Use design.md to map repo components, system roles, and feature-gated surfaces
- Use ADRs for durable boundary decisions such as default workflow versus system capability
- Use tasks.md to make every consolidation or integration cut a test-backed step
- Regenerate .idd/knowledge artifacts and .idd/MANIFEST.tsv after source or control-plane edits
- For cross-repo work, treat peer repo state as evidence and avoid mutating peers from this command

## Findings

- system context selected 13 roles and 20 repos from 65 discovered repos
- operating context selected 10 layers and 18 capabilities from 19 generated capabilities
- integration context selected 12 work items from 19 generated work items
