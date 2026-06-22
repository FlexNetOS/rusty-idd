# Graph Planning Context

- Change: `harden-e2e-validation-evidence`
- Goal: # E2E Review Upgrade Goal

Run a deep research code-review loop over the comprehensive E2E workflow test
suite and Codex workflow-check work. Gap-hunt the merged implementation, apply
upgrades, and raise the tests to professional code-grade quality.

The upgrade must preserve the Rusty IDD workflow order:

1. Goal file and graph-backed context.
2. Claimed task card.
3. OpenSpec proposal, design, spec delta, tasks, and ADR.
4. Review evidence.
5. Test-first implementation upgrades.
6. Generated artifact refresh.
7. Mandatory validation before task completion, push, PR, and merge.

Professional-grade means validation evidence must not pass from marker words
alone. The workflow checker must reject failed, placeholder, skipped, or
misordered validation entries for build, generated artifacts, tests, lint,
secret scan, and manifest evidence.
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/e2e-review-upgrades`
- Source graph: 139 files, 8693 nodes, 35695 edges via `codegraph-rust`
- Context package: 293 files, 695065 tokens via `repomix-rs`

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
| `cli` | crate | 24 | 863 | 3806 | crates/cli/src/commands/codex.rs, crates/cli/src/commands/core.rs, crates/cli/src/commands/knowledge.rs, crates/cli/src/commands/merge_tools.rs, crates/cli/src/commands/mod.rs, crates/cli/src/commands/run.rs, crates/cli/src/commands/spec.rs, crates/cli/src/commands/spec_adr.rs, crates/cli/src/commands/spec_archive.rs, crates/cli/src/commands/spec_plan_integration.rs, crates/cli/src/commands/spec_scaffold.rs, crates/cli/src/commands/spec_status.rs |
| `codegraph-parser` | external_crate | 29 | 1060 | 7485 | crates/external/codegraph-parser/src/complexity.rs, crates/external/codegraph-parser/src/diff.rs, crates/external/codegraph-parser/src/edge.rs, crates/external/codegraph-parser/src/fast_io.rs, crates/external/codegraph-parser/src/fast_ml/enhancer.rs, crates/external/codegraph-parser/src/fast_ml/mod.rs, crates/external/codegraph-parser/src/fast_ml/pattern_matcher.rs, crates/external/codegraph-parser/src/fast_ml/symbol_resolver.rs, crates/external/codegraph-parser/src/file_collect.rs, crates/external/codegraph-parser/src/integration_tests.rs, crates/external/codegraph-parser/src/language.rs, crates/external/codegraph-parser/src/languages/cpp.rs |
| `core` | crate | 12 | 448 | 2470 | crates/core/src/cli.rs, crates/core/src/env_contract.rs, crates/core/src/fs_utils.rs, crates/core/src/lib.rs, crates/core/src/manifest.rs, crates/core/src/model.rs, crates/core/src/planner.rs, crates/core/src/scanner.rs, crates/core/src/templates.rs, crates/core/src/validation.rs, crates/core/tests/smoke.rs, crates/core/tests/template_agent_surface.rs |
| `spec` | crate | 24 | 461 | 1957 | crates/spec/src/adr/mod.rs, crates/spec/src/archive/mod.rs, crates/spec/src/lib.rs, crates/spec/src/model/block.rs, crates/spec/src/model/delta.rs, crates/spec/src/model/merge.rs, crates/spec/src/model/mod.rs, crates/spec/src/model/requirement.rs, crates/spec/src/model/spec.rs, crates/spec/src/parse/common.rs, crates/spec/src/parse/delta_parser.rs, crates/spec/src/parse/emit.rs |
| `codegraph-core` | external_crate | 39 | 1918 | 9070 | crates/external/codegraph-core/benches/core_micro.rs, crates/external/codegraph-core/src/advanced_config.rs, crates/external/codegraph-core/src/arena.rs, crates/external/codegraph-core/src/buffer_pool.rs, crates/external/codegraph-core/src/cli_config.rs, crates/external/codegraph-core/src/compression.rs, crates/external/codegraph-core/src/config.rs, crates/external/codegraph-core/src/config_manager.rs, crates/external/codegraph-core/src/embedding_config.rs, crates/external/codegraph-core/src/error.rs, crates/external/codegraph-core/src/incremental/mod.rs, crates/external/codegraph-core/src/incremental/updater.rs |
| `runner` | crate | 4 | 770 | 3135 | crates/runner/src/config.rs, crates/runner/src/data.rs, crates/runner/src/lib.rs, crates/runner/src/runner.rs |
| `merge-tools` | crate | 1 | 45 | 235 | crates/merge-tools/src/lib.rs |
| `knowledge` | crate | 1 | 639 | 4653 | crates/knowledge/src/lib.rs |
| `tui` | crate | 3 | 1006 | 3985 | crates/tui/src/app.rs, crates/tui/src/lib.rs, crates/tui/src/ui.rs |
| `repomix-shared` | external_crate | 2 | 11 | 34 | crates/external/repomix-shared/src/lib.rs, crates/external/repomix-shared/src/types.rs |

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
- `Spec producer`: Produces intent or prompt artifacts that Rusty IDD can turn into OpenSpec
- `Toolchain provider`: Provides parent-managed tools instead of user-global installs

## System Repos

| Repo | Branch | Dirty | Roles | Architecture |
|---|---|---|---|---|
| `obscura` | `` | false | role:agent-environment, role:domain-upgrade-surface |  |
| `codex` | `` | false | role:agent-environment |  |
| `yazelix` | `` | false | role:parser-runtime-surface, role:toolchain-provider |  |
| `claude-code` | `` | false | role:agent-environment |  |
| `oh-my-claudecode` | `` | false | role:agent-environment |  |
| `rusty-idd` | `` | false | role:idd-control-plane |  |
| `lane` | `` | false |  |  |
| `prompt_hub` | `` | false | role:agent-environment, role:capability-hub, role:spec-producer |  |
| `handoff` | `` | false | role:coordination-domain-surface, role:fleet-handoff |  |
| `Archon` | `` | false | role:agent-environment |  |
| `ECC` | `` | false | role:agent-environment |  |
| `agent` | `` | false | role:agent-environment |  |
| `commands` | `` | false | role:capability-hub |  |
| `kasetto` | `` | false | role:agent-environment |  |
| `loop_cli` | `` | false | role:meta-control-plane |  |
| `loop_lib` | `` | false | role:meta-control-plane |  |
| `network_hub` | `` | false | role:capability-hub |  |
| `oh-my-pi` | `` | false | role:agent-environment |  |
| `ruflo` | `` | false | role:agent-environment |  |
| `ruvector` | `` | false | role:agent-environment |  |

## Operating Layers

- `Agent runtime`: Agent harnesses, execution workers, and automation runtimes (2 capabilities, 25 repos)
- `Coordination and communication`: Agent communication, orchestration, and cross-agent continuity (3 capabilities, 6 repos)
- `Environment and security`: Vault, key relay, certificates, and parent-managed toolchains (1 capabilities, 3 repos)
- `Executive control plane`: Company-level command, OpenSpec, handoff, and repo governance (2 capabilities, 14 repos)
- `Front door experience`: Prompt, chat, LifeOS, and operator-facing user experience surfaces (2 capabilities, 3 repos)
- `Governance and reasoning`: Board-style reasoning, strategy, and policy without direct execution (1 capabilities, 5 repos)
- `Infrastructure and device fabric`: Network control plus distributed device compute, storage, inference, and memory (2 capabilities, 5 repos)
- `Interface automation`: AR-glasses workflow, local automation, media, and home interfaces (2 capabilities, 3 repos)
- `Knowledge and runtime`: Memory, vector/progress databases, inference, training, and runtime state (1 capabilities, 4 repos)
- `Simulation and validation`: Digital twin simulation and high-fidelity failure space for agents (1 capabilities, 1 repos)
- `Toolchain and parser runtime`: Tree-sitter, Lua, terminal/runtime, parser, and toolchain surfaces (2 capabilities, 7 repos)

## Operating Capabilities

| Capability | Layer | Status | Repos | Anchors |
|---|---|---|---|---|
| `IDD and spec engine` | `layer:executive-control-plane` | partial | repo:handoff, repo:rusty-idd | Rusty IDD built into handoff |
| `GitHub agent-run upgrades` | `layer:agent-runtime` | partial | repo:grit, repo:yazelix | GRIT from rtk-ai, Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| `Agent harness runtime` | `layer:agent-runtime` | partial | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:ruvector | harness-agent-rs rust port |
| `Central and fleet handoff` | `layer:coordination-communication` | partial | repo:handoff, repo:rusty-idd, repo:weave | handoff central and fleet design |
| `Network engineering and control` | `layer:infrastructure-device-fabric` | partial | repo:lane, repo:network-control, repo:network-hub | lane merges into network-manager |
| `Meta peer repo control` | `layer:executive-control-plane` | partial | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli | meta peer repo system |
| `Digital twin simulation` | `layer:simulation-validation` | partial | repo:teri | Teri digital twin simulator |
| `Parser and terminal runtime` | `layer:toolchain-parser-runtime` | partial | repo:rusty-idd, repo:tool-hub, repo:yazelix | tree-sitter via Yazelix, Yazelix default terminal, nushell, Lua, Ghostty, Zellij |
| `Lua and AR interface automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi, repo:yazelix | Lua required for AR glasses workflow, Brilliant Labs Noa style Rust-native agent UX |
| `RTK AI foundation` | `layer:toolchain-parser-runtime` | partial | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox | RTK from rtk-ai, ICM from rtk-ai, VOX from rtk-ai, GRIT from rtk-ai |
| `Environment and vault relay` | `layer:environment-security` | partial | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| `Prompt front door` | `layer:front-door-experience` | partial | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt, prompt_hub front door to handoff and rusty-idd |
| `User front door` | `layer:front-door-experience` | partial | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration, LifeOS front door |
| `Distributed device fabric` | `layer:infrastructure-device-fabric` | partial | repo:envctl, repo:network-control, repo:oh-my-pi | user devices for distributed compute storage inference memory |
| `Domain upgrade path` | `layer:coordination-communication` | partial | repo:obscura, repo:weave | weave plus Obscura domain upgrades |
| `Vector and agentic runtime` | `layer:knowledge-runtime` | partial | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector | meta-ruvector full agentic system |
| `Board reasoning layer` | `layer:governance-reasoning` | partial | repo:flexnetos-brain, repo:flexnetos-wiki, repo:icm, repo:my-wiki, repo:obsidian-mind | company hierarchy board layer |
| `Agent communication layer` | `layer:coordination-communication` | partial | repo:atc, repo:handoff, repo:mcp-hub, repo:weave | weave agent communication layer |

## Integration Work

| Priority | Work Item | Change | Owners | Adopt First |
|---:|---|---|---|---|
| 10 | `Integrate IDD and spec engine` | `integrate-idd-spec-engine` | repo:handoff, repo:rusty-idd |  |
| 20 | `Integrate Central and fleet handoff` | `integrate-fleet-handoff` | repo:handoff, repo:rusty-idd, repo:weave |  |
| 30 | `Integrate Agent communication layer` | `integrate-agent-communication` | repo:atc, repo:handoff, repo:mcp-hub, repo:weave |  |
| 60 | `Integrate RTK AI foundation` | `integrate-rtk-ai-foundation` | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox |  |
| 70 | `Integrate GitHub agent-run upgrades` | `integrate-github-agent-run-upgrades` | repo:grit, repo:yazelix | Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| 80 | `Integrate Parser and terminal runtime` | `integrate-parser-runtime` | repo:rusty-idd, repo:tool-hub, repo:yazelix |  |
| 90 | `Integrate Vector and agentic runtime` | `integrate-vector-runtime` | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector |  |
| 110 | `Integrate Digital twin simulation` | `integrate-digital-twin-simulation` | repo:teri |  |
| 120 | `Integrate Network engineering and control` | `integrate-network-engineering` | repo:lane, repo:network-control, repo:network-hub |  |
| 140 | `Integrate Lua and AR interface automation` | `integrate-lua-ar-interface` | repo:lifeos, repo:oh-my-pi, repo:yazelix |  |
| 500 | `Integrate Agent harness runtime` | `integrate-agent-harness` | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:ruvector |  |
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

- system context selected 12 roles and 20 repos from 65 discovered repos
- operating context selected 11 layers and 18 capabilities from 19 generated capabilities
- integration context selected 12 work items from 19 generated work items
