# Graph Planning Context

- Change: `unify-handoff-prompthub`
- Goal: # Unify handoff + prompt_hub into rusty-idd Goal

rusty-idd --goal-file .idd/goals/unify-handoff-prompthub.md

Unify the FlexNetOS-owned `handoff` and `prompt_hub` repositories into rusty-idd
as first-class parts of one product, following the Rusty IDD **merge-tools**
workflow (`rusty-idd merge-tools show`) and the AGENTS.md North Star: *unify by
preserving working behavior, making contracts explicit, and merging only through
reviewable, test-backed increments.*

These are NOT third-party upstreams, and the direction of truth matters:
**`handoff` is the CANONICAL BASE — it is NOT a fork of rusty-idd.** The 293
relative paths `handoff` shares with rusty-idd (`crates/core,spec,runner,cli,tui`)
are the residue of an **earlier POOR merge attempt** that pulled handoff's
structure into rusty-idd incompletely. So where a shared file diverges, **handoff
is authoritative**; rusty-idd's genuine *forward* additions (e.g. the Phase-1
`deploy` front door, ADR-0017, the `next`/`render` control plane) are reconciled
*onto* handoff's canonical base — upgrade-only, never losing a capability from
either side. handoff also contributes its unique `hf` / `ledger` / `work-order`
crates and its `.handoff` witnessed continuity kernel. `prompt_hub` is, by
contrast, **independent** (only 18 shared meta-file paths): `prompt-hub` lib +
`prompthub` CLI + `prompthub-server` + its `prompt-loop` harness + `.kb`, folded
in additively. The unified rusty-idd's **code graph and `.kb` knowledge base must
span all of this as first-class code** — the consolidation is worthless if
rusty-idd is blind to the code it absorbs.

A prior poor merge attempt already failed here (and I repeated its mistakes in
this session before resetting). This goal MUST learn from that: handoff canonical,
inventory-before-flatten, faithful adopt, parity-tested vertical slices.

## Required Method — the merge-tools 6 phases (do them in order)

1. **inventory** (FIRST, before any flattening — Operating Rule 1): generate, for
   both repos, the `RepoInventory`, feature matrix, env/secret contract, and
   legacy-surface inventory, plus the divergence map of handoff↔rusty-idd shared
   crates. Gates: `rusty-idd scan`, `rusty-idd knowledge refresh`, no untracked
   secret material.
2. **plan**: bind this goal with `rusty-idd knowledge plan-context` and create ONE
   OpenSpec change (proposal, spec deltas, design, tasks). Gates: `spec status` /
   `spec next`.
3. **decide**: one active ADR for the unification architecture + migration note;
   summarize prior merge decisions as inputs, don't resurrect them.
4. **implement**: apply ONE narrow vertical slice at a time, adopt-first,
   preserving old behavior until parity is proven (no stubs, no downgrades, core
   crate stays zero-dep). Reconcile the crate-name collisions and toolchain/feature
   constraints as evidenced merge work, not as guesses.
5. **verify**: `cargo build/test/fmt/clippy --workspace` + `rusty-idd validate
   --workspace .`; refresh `.idd/knowledge/*` + `MANIFEST.tsv`.
6. **evidence**: PR evidence bundle, migration note (old path → new path),
   rollback path, manifest state, AI_MERGE audit note.

## Hard Constraints (lessons already paid for)

- **Adopt as-is, faithfully** (Codex Rule 5): bring the complete current state
  forward intact — including `.kb` knowledge bases and `.handoff` witnessed
  ledgers (handoff `.handoff/ledger.db` + `.rvf`, prompt_hub `.handoff/ledger.db`).
  Do NOT strip binaries, db/onnx/sarif, or knowledge-base state during adoption;
  any cut happens later, only with recorded evidence.
- **Code graph is first-class**: the adopted code MUST be indexed in rusty-idd's
  code graph / `.kb`, not excluded. Do not hide absorbed code from code
  intelligence.
- **Upgrade only, never downgrade** a working surface, dep, or generated artifact.
- **Test-backed increments**: narrow PRs, one vertical slice each; preserve
  behavior until parity tests pass; deprecate before remove.

## Decision Target

rusty-idd SHALL absorb handoff and prompt_hub as first-class, code-graph-indexed
parts of one unified product, via the merge-tools workflow, with explicit
inventories/contracts produced before any flattening and behavior preserved
through test-backed increments — so the standalone repos can subsequently be
unregistered and archived without losing capability, knowledge, or ledger state.

## Non-Goals (this unification goal)

- Live fleet deployment, repo unregistration, and archiving are the sequenced
  follow-on phases; this goal delivers the unification itself.
- No big-bang flatten: no single mega-commit dumping both trees before the
  inventory/contract maps exist.
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd`
- Source graph: 149 files, 9210 nodes, 37690 edges via `codegraph-rust`
- Context package: 312 files, 179926 tokens via `repomix-rs`

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
| `cli` | crate | 34 | 1371 | 5896 | crates/cli/src/commands/codex.rs, crates/cli/src/commands/core.rs, crates/cli/src/commands/deploy.rs, crates/cli/src/commands/harness.rs, crates/cli/src/commands/knowledge.rs, crates/cli/src/commands/merge_tools.rs, crates/cli/src/commands/mod.rs, crates/cli/src/commands/next.rs, crates/cli/src/commands/render.rs, crates/cli/src/commands/run.rs, crates/cli/src/commands/spec.rs, crates/cli/src/commands/spec_adr.rs |
| `core` | crate | 12 | 448 | 2479 | crates/core/src/cli.rs, crates/core/src/env_contract.rs, crates/core/src/fs_utils.rs, crates/core/src/lib.rs, crates/core/src/manifest.rs, crates/core/src/model.rs, crates/core/src/planner.rs, crates/core/src/scanner.rs, crates/core/src/templates.rs, crates/core/src/validation.rs, crates/core/tests/smoke.rs, crates/core/tests/template_agent_surface.rs |
| `codegraph-core` | external_crate | 39 | 1918 | 9070 | crates/external/codegraph-core/benches/core_micro.rs, crates/external/codegraph-core/src/advanced_config.rs, crates/external/codegraph-core/src/arena.rs, crates/external/codegraph-core/src/buffer_pool.rs, crates/external/codegraph-core/src/cli_config.rs, crates/external/codegraph-core/src/compression.rs, crates/external/codegraph-core/src/config.rs, crates/external/codegraph-core/src/config_manager.rs, crates/external/codegraph-core/src/embedding_config.rs, crates/external/codegraph-core/src/error.rs, crates/external/codegraph-core/src/incremental/mod.rs, crates/external/codegraph-core/src/incremental/updater.rs |
| `codegraph-parser` | external_crate | 29 | 1060 | 7485 | crates/external/codegraph-parser/src/complexity.rs, crates/external/codegraph-parser/src/diff.rs, crates/external/codegraph-parser/src/edge.rs, crates/external/codegraph-parser/src/fast_io.rs, crates/external/codegraph-parser/src/fast_ml/enhancer.rs, crates/external/codegraph-parser/src/fast_ml/mod.rs, crates/external/codegraph-parser/src/fast_ml/pattern_matcher.rs, crates/external/codegraph-parser/src/fast_ml/symbol_resolver.rs, crates/external/codegraph-parser/src/file_collect.rs, crates/external/codegraph-parser/src/integration_tests.rs, crates/external/codegraph-parser/src/language.rs, crates/external/codegraph-parser/src/languages/cpp.rs |
| `spec` | crate | 24 | 461 | 1976 | crates/spec/src/adr/mod.rs, crates/spec/src/archive/mod.rs, crates/spec/src/lib.rs, crates/spec/src/model/block.rs, crates/spec/src/model/delta.rs, crates/spec/src/model/merge.rs, crates/spec/src/model/mod.rs, crates/spec/src/model/requirement.rs, crates/spec/src/model/spec.rs, crates/spec/src/parse/common.rs, crates/spec/src/parse/delta_parser.rs, crates/spec/src/parse/emit.rs |
| `repomix-shared` | external_crate | 2 | 11 | 34 | crates/external/repomix-shared/src/lib.rs, crates/external/repomix-shared/src/types.rs |
| `merge-tools` | crate | 1 | 45 | 234 | crates/merge-tools/src/lib.rs |
| `knowledge` | crate | 1 | 639 | 4653 | crates/knowledge/src/lib.rs |
| `tui` | crate | 3 | 1006 | 3985 | crates/tui/src/app.rs, crates/tui/src/lib.rs, crates/tui/src/ui.rs |
| `runner` | crate | 4 | 770 | 3135 | crates/runner/src/config.rs, crates/runner/src/data.rs, crates/runner/src/lib.rs, crates/runner/src/runner.rs |

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
| `meta_git_lib` | `chore/indicatif-0.18-rustsec-2025-0119` | false | role:fleet-handoff, role:meta-control-plane, role:rust-code-surface |  |
| `loop_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `loop_lib` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `prompt_hub` | `main` | true | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer |  |
| `meta_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `meta_core` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `meta_git_cli` | `feat/dep-upgrades` | false | role:meta-control-plane, role:rust-code-surface |  |
| `meta_project_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `meta_rust_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `envctl` | `master` | true | role:fleet-handoff, role:rust-code-surface, role:toolchain-provider |  |
| `lane` | `main` | false | role:fleet-handoff, role:rust-code-surface |  |
| `meta_plugin_protocol` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `network-control` | `develop` | false | role:fleet-handoff, role:rust-code-surface |  |
| `weave` | `develop` | false | role:agent-environment, role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface |  |
| `agent` | `main` | false | role:agent-environment, role:fleet-handoff, role:rust-code-surface |  |
| `ECC` | `main` | false | role:agent-environment, role:fleet-handoff |  |
| `flexnetos_runner` | `chore/handoff-tier-a-pilot` | false | role:fleet-handoff, role:rust-code-surface |  |
| `meta_plugin_api` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |

## Operating Layers

- `Agent runtime`: Agent harnesses, execution workers, and automation runtimes (2 capabilities, 28 repos)
- `Coordination and communication`: Agent communication, orchestration, and cross-agent continuity (3 capabilities, 19 repos)
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
| `Central and fleet handoff` | `layer:coordination-communication` | partial | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave | handoff central and fleet design |
| `IDD and spec engine` | `layer:executive-control-plane` | partial | repo:handoff, repo:rusty-idd | Rusty IDD built into handoff |
| `Agent harness runtime` | `layer:agent-runtime` | partial | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave | harness-agent-rs rust port |
| `Meta peer repo control` | `layer:executive-control-plane` | partial | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli | meta peer repo system |
| `Digital twin simulation` | `layer:simulation-validation` | partial | repo:teri | Teri digital twin simulator |
| `GitHub agent-run upgrades` | `layer:agent-runtime` | partial | repo:grit, repo:yazelix | GRIT from rtk-ai, Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| `Prompt front door` | `layer:front-door-experience` | partial | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt, prompt_hub front door to handoff and rusty-idd |
| `Network engineering and control` | `layer:infrastructure-device-fabric` | partial | repo:lane, repo:network-control, repo:network-hub | lane merges into network-manager |
| `Environment and vault relay` | `layer:environment-security` | partial | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| `Parser and terminal runtime` | `layer:toolchain-parser-runtime` | partial | repo:rusty-idd, repo:tool-hub, repo:yazelix | tree-sitter via Yazelix, Yazelix default terminal, nushell, Lua, Ghostty, Zellij |
| `Vector and agentic runtime` | `layer:knowledge-runtime` | partial | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector | meta-ruvector full agentic system |
| `Lua and AR interface automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi, repo:yazelix | Lua required for AR glasses workflow, Brilliant Labs Noa style Rust-native agent UX |
| `User front door` | `layer:front-door-experience` | partial | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration, LifeOS front door |
| `Board reasoning layer` | `layer:governance-reasoning` | partial | repo:flexnetos-brain, repo:flexnetos-wiki, repo:icm, repo:my-wiki, repo:obsidian-mind | company hierarchy board layer |
| `Distributed device fabric` | `layer:infrastructure-device-fabric` | partial | repo:envctl, repo:network-control, repo:oh-my-pi | user devices for distributed compute storage inference memory |
| `RTK AI foundation` | `layer:toolchain-parser-runtime` | partial | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox | RTK from rtk-ai, ICM from rtk-ai, VOX from rtk-ai, GRIT from rtk-ai |
| `Agent communication layer` | `layer:coordination-communication` | partial | repo:atc, repo:handoff, repo:mcp-hub, repo:weave | weave agent communication layer |
| `Domain upgrade path` | `layer:coordination-communication` | partial | repo:obscura, repo:weave | weave plus Obscura domain upgrades |

## Integration Work

| Priority | Work Item | Change | Owners | Adopt First |
|---:|---|---|---|---|
| 10 | `Integrate IDD and spec engine` | `integrate-idd-spec-engine` | repo:handoff, repo:rusty-idd |  |
| 20 | `Integrate Central and fleet handoff` | `integrate-fleet-handoff` | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave |  |
| 30 | `Integrate Agent communication layer` | `integrate-agent-communication` | repo:atc, repo:handoff, repo:mcp-hub, repo:weave |  |
| 70 | `Integrate GitHub agent-run upgrades` | `integrate-github-agent-run-upgrades` | repo:grit, repo:yazelix | Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| 80 | `Integrate Parser and terminal runtime` | `integrate-parser-runtime` | repo:rusty-idd, repo:tool-hub, repo:yazelix |  |
| 90 | `Integrate Vector and agentic runtime` | `integrate-vector-runtime` | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector |  |
| 100 | `Integrate User front door` | `integrate-user-front-door` | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration |
| 120 | `Integrate Network engineering and control` | `integrate-network-engineering` | repo:lane, repo:network-control, repo:network-hub |  |
| 130 | `Integrate Distributed device fabric` | `integrate-distributed-device-fabric` | repo:envctl, repo:network-control, repo:oh-my-pi |  |
| 140 | `Integrate Lua and AR interface automation` | `integrate-lua-ar-interface` | repo:lifeos, repo:oh-my-pi, repo:yazelix |  |
| 500 | `Integrate Agent harness runtime` | `integrate-agent-harness` | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave |  |
| 500 | `Integrate Meta peer repo control` | `integrate-meta-peer-control` | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli |  |

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
- operating context selected 11 layers and 18 capabilities from 19 generated capabilities
- integration context selected 12 work items from 19 generated work items
