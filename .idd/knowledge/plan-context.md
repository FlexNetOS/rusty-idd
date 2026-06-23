# Graph Planning Context

- Change: `harness-session-frontdoor`
- Goal: # Harness Session Front Door + ADR Ledger Hygiene Goal

rusty-idd --goal-file .idd/goals/harness-session-frontdoor.md

The front door is built but nothing invokes it. `rusty-idd next` (ADR-0015) and
the thin vendor adapters (`render`, ADR-0010) exist and are CI-enforced, yet no
vendor surface actually *calls* `rusty-idd next` at session start — agents must
remember to run it. Close that gap: wire the `SessionStart` hook of each vendor
surface that supports one to invoke `rusty-idd next`, so the computed
next-step imperative is presented automatically at the start of every session
(backlog item 4.2 of harness-control-plane).

While here, reconcile the ADR ledger: parallel changes each allocated the same
ADR number before committing, producing duplicate-numbered ADRs at 0002, 0004,
0005, and 0006 (backlog item 4.5). ADRs are immutable once accepted, so the
existing files are frozen as historical artifacts; reconciliation is a new ADR
plus an engine collision detector that prevents recurrence.

This preserves the Rusty IDD workflow order: goal -> graph context -> OpenSpec
(spec delta + design + tasks) -> ADR -> implementation after ready -> validation
refresh.

## Intent

- Add a `SessionStart` hook to `.codex/hooks.json` that runs `rusty-idd next`
  (same `sh -lc 'root=...; cargo run ... --bin rusty-idd -- next --base "$root"'`
  shape as the existing workflow-check hooks).
- Create `.claude/settings.json` with a `hooks.SessionStart` entry that runs
  `rusty-idd next`, so Claude Code surfaces the imperative on session start.
- Add a fail-closed ADR-number collision detector (`rusty-idd spec adr list
  --check`, mirroring `render --check`) that exits non-zero on any NEW duplicate
  number beyond a frozen baseline of the four known historical collisions; wire
  it into CI so the bug cannot recur.
- Author ADR-0016 recording the historical collisions as frozen and establishing
  slug-canonical ADR referencing going forward.

## Decision Target

Vendor surfaces SHALL invoke `rusty-idd next` automatically at session start via
their native `SessionStart` hook, not by relying on the agent to remember. The
ADR ledger SHALL be collision-checked in CI against a frozen baseline of the four
accepted historical duplicates; new collisions SHALL fail closed.

## Non-Goals

- No change to `rusty-idd next` behavior or output (it already exists, ADR-0015).
- No renumbering or editing of the existing duplicate-numbered ADRs (immutable;
  reconciled by ADR + slug-canonical rule, not by mutation).
- No additional stage packages (impl/validation/handoff swarms) — that is backlog
  4.3 (the separate add-verify-package-stage change), out of scope here.
- No new vendor surfaces; `.agents`/`.devin` have no standard SessionStart hook
  mechanism and keep their existing thin adapter only.
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd`
- Source graph: 145 files, 8965 nodes, 36839 edges via `codegraph-rust`
- Context package: 376 files, 753083 tokens via `repomix-rs`

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
| `cli` | crate | 30 | 1131 | 5013 | crates/cli/src/commands/codex.rs, crates/cli/src/commands/core.rs, crates/cli/src/commands/harness.rs, crates/cli/src/commands/knowledge.rs, crates/cli/src/commands/merge_tools.rs, crates/cli/src/commands/mod.rs, crates/cli/src/commands/next.rs, crates/cli/src/commands/render.rs, crates/cli/src/commands/run.rs, crates/cli/src/commands/spec.rs, crates/cli/src/commands/spec_adr.rs, crates/cli/src/commands/spec_archive.rs |
| `codegraph-core` | external_crate | 39 | 1918 | 9070 | crates/external/codegraph-core/benches/core_micro.rs, crates/external/codegraph-core/src/advanced_config.rs, crates/external/codegraph-core/src/arena.rs, crates/external/codegraph-core/src/buffer_pool.rs, crates/external/codegraph-core/src/cli_config.rs, crates/external/codegraph-core/src/compression.rs, crates/external/codegraph-core/src/config.rs, crates/external/codegraph-core/src/config_manager.rs, crates/external/codegraph-core/src/embedding_config.rs, crates/external/codegraph-core/src/error.rs, crates/external/codegraph-core/src/incremental/mod.rs, crates/external/codegraph-core/src/incremental/updater.rs |
| `codegraph-parser` | external_crate | 29 | 1060 | 7485 | crates/external/codegraph-parser/src/complexity.rs, crates/external/codegraph-parser/src/diff.rs, crates/external/codegraph-parser/src/edge.rs, crates/external/codegraph-parser/src/fast_io.rs, crates/external/codegraph-parser/src/fast_ml/enhancer.rs, crates/external/codegraph-parser/src/fast_ml/mod.rs, crates/external/codegraph-parser/src/fast_ml/pattern_matcher.rs, crates/external/codegraph-parser/src/fast_ml/symbol_resolver.rs, crates/external/codegraph-parser/src/file_collect.rs, crates/external/codegraph-parser/src/integration_tests.rs, crates/external/codegraph-parser/src/language.rs, crates/external/codegraph-parser/src/languages/cpp.rs |
| `core` | crate | 12 | 448 | 2479 | crates/core/src/cli.rs, crates/core/src/env_contract.rs, crates/core/src/fs_utils.rs, crates/core/src/lib.rs, crates/core/src/manifest.rs, crates/core/src/model.rs, crates/core/src/planner.rs, crates/core/src/scanner.rs, crates/core/src/templates.rs, crates/core/src/validation.rs, crates/core/tests/smoke.rs, crates/core/tests/template_agent_surface.rs |
| `spec` | crate | 24 | 461 | 1967 | crates/spec/src/adr/mod.rs, crates/spec/src/archive/mod.rs, crates/spec/src/lib.rs, crates/spec/src/model/block.rs, crates/spec/src/model/delta.rs, crates/spec/src/model/merge.rs, crates/spec/src/model/mod.rs, crates/spec/src/model/requirement.rs, crates/spec/src/model/spec.rs, crates/spec/src/parse/common.rs, crates/spec/src/parse/delta_parser.rs, crates/spec/src/parse/emit.rs |
| `runner` | crate | 4 | 770 | 3135 | crates/runner/src/config.rs, crates/runner/src/data.rs, crates/runner/src/lib.rs, crates/runner/src/runner.rs |
| `repomix-shared` | external_crate | 2 | 11 | 34 | crates/external/repomix-shared/src/lib.rs, crates/external/repomix-shared/src/types.rs |
| `knowledge` | crate | 1 | 639 | 4653 | crates/knowledge/src/lib.rs |
| `tui` | crate | 3 | 1006 | 3985 | crates/tui/src/app.rs, crates/tui/src/lib.rs, crates/tui/src/ui.rs |
| `merge-tools` | crate | 1 | 45 | 234 | crates/merge-tools/src/lib.rs |

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
| `ruvector` | `develop` | true | role:agent-environment, role:fleet-handoff, role:rust-code-surface |  |
| `prompt_hub` | `main` | true | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer |  |
| `weave` | `develop` | false | role:agent-environment, role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface |  |
| `meta_git_lib` | `chore/indicatif-0.18-rustsec-2025-0119` | false | role:fleet-handoff, role:meta-control-plane, role:rust-code-surface |  |
| `lifeos` | `main` | false | role:fleet-handoff, role:rust-code-surface |  |
| `agent` | `main` | false | role:agent-environment, role:fleet-handoff, role:rust-code-surface |  |
| `lane` | `main` | false | role:fleet-handoff, role:rust-code-surface |  |
| `meta_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `network-control` | `develop` | false | role:fleet-handoff, role:rust-code-surface |  |
| `envctl` | `master` | true | role:fleet-handoff, role:rust-code-surface, role:toolchain-provider |  |
| `handoff` | `develop` | true | role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface |  |
| `ECC` | `main` | false | role:agent-environment, role:fleet-handoff |  |
| `flexnetos_runner` | `chore/handoff-tier-a-pilot` | false | role:fleet-handoff, role:rust-code-surface |  |
| `github_org` | `fix/autonomous-feature-develop-approval` | false | role:agent-environment, role:fleet-handoff |  |
| `loop_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `loop_lib` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `meta_core` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `meta_git_cli` | `feat/dep-upgrades` | false | role:meta-control-plane, role:rust-code-surface |  |
| `meta_plugin_api` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |

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
| `IDD and spec engine` | `layer:executive-control-plane` | partial | repo:handoff, repo:rusty-idd | Rusty IDD built into handoff |
| `Agent harness runtime` | `layer:agent-runtime` | partial | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave | harness-agent-rs rust port |
| `Central and fleet handoff` | `layer:coordination-communication` | partial | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave | handoff central and fleet design |
| `Prompt front door` | `layer:front-door-experience` | partial | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt, prompt_hub front door to handoff and rusty-idd |
| `Lua and AR interface automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi, repo:yazelix | Lua required for AR glasses workflow, Brilliant Labs Noa style Rust-native agent UX |
| `Digital twin simulation` | `layer:simulation-validation` | partial | repo:teri | Teri digital twin simulator |
| `GitHub agent-run upgrades` | `layer:agent-runtime` | partial | repo:grit, repo:yazelix | GRIT from rtk-ai, Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| `Vector and agentic runtime` | `layer:knowledge-runtime` | partial | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector | meta-ruvector full agentic system |
| `User front door` | `layer:front-door-experience` | partial | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration, LifeOS front door |
| `Meta peer repo control` | `layer:executive-control-plane` | partial | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli | meta peer repo system |
| `Parser and terminal runtime` | `layer:toolchain-parser-runtime` | partial | repo:rusty-idd, repo:tool-hub, repo:yazelix | tree-sitter via Yazelix, Yazelix default terminal, nushell, Lua, Ghostty, Zellij |
| `RTK AI foundation` | `layer:toolchain-parser-runtime` | partial | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox | RTK from rtk-ai, ICM from rtk-ai, VOX from rtk-ai, GRIT from rtk-ai |
| `Environment and vault relay` | `layer:environment-security` | partial | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| `Network engineering and control` | `layer:infrastructure-device-fabric` | partial | repo:lane, repo:network-control, repo:network-hub | lane merges into network-manager |
| `Personal media and home automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi | personal life media TV home automation |
| `Agent communication layer` | `layer:coordination-communication` | partial | repo:atc, repo:handoff, repo:mcp-hub, repo:weave | weave agent communication layer |
| `Distributed device fabric` | `layer:infrastructure-device-fabric` | partial | repo:envctl, repo:network-control, repo:oh-my-pi | user devices for distributed compute storage inference memory |
| `Domain upgrade path` | `layer:coordination-communication` | partial | repo:obscura, repo:weave | weave plus Obscura domain upgrades |

## Integration Work

| Priority | Work Item | Change | Owners | Adopt First |
|---:|---|---|---|---|
| 10 | `Integrate IDD and spec engine` | `integrate-idd-spec-engine` | repo:handoff, repo:rusty-idd |  |
| 20 | `Integrate Central and fleet handoff` | `integrate-fleet-handoff` | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave |  |
| 30 | `Integrate Agent communication layer` | `integrate-agent-communication` | repo:atc, repo:handoff, repo:mcp-hub, repo:weave |  |
| 50 | `Integrate Prompt front door` | `integrate-prompt-front-door` | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt |
| 70 | `Integrate GitHub agent-run upgrades` | `integrate-github-agent-run-upgrades` | repo:grit, repo:yazelix | Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| 80 | `Integrate Parser and terminal runtime` | `integrate-parser-runtime` | repo:rusty-idd, repo:tool-hub, repo:yazelix |  |
| 90 | `Integrate Vector and agentic runtime` | `integrate-vector-runtime` | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector |  |
| 100 | `Integrate User front door` | `integrate-user-front-door` | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration |
| 120 | `Integrate Network engineering and control` | `integrate-network-engineering` | repo:lane, repo:network-control, repo:network-hub |  |
| 130 | `Integrate Distributed device fabric` | `integrate-distributed-device-fabric` | repo:envctl, repo:network-control, repo:oh-my-pi |  |
| 140 | `Integrate Lua and AR interface automation` | `integrate-lua-ar-interface` | repo:lifeos, repo:oh-my-pi, repo:yazelix |  |
| 500 | `Integrate Agent harness runtime` | `integrate-agent-harness` | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave |  |

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
