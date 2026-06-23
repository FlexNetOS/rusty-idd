# Graph Planning Context

- Goal: # add-self-upgrade-governor

## Step 1: Run This Goal Through Rusty IDD

```bash
rusty-idd --goal-file .idd/goals/add-self-upgrade-governor.md
```

If the active CLI surface requires a subcommand for goal binding, use the same
goal file as the input to the Rusty IDD planning command:

```bash
rusty-idd knowledge plan-context \
  --workspace . \
  --out .idd/knowledge/plan-context.md \
  --goal-file .idd/goals/add-self-upgrade-governor.md
```

## Goal

Create a first-class Rusty IDD self-upgrade governor workflow from the approved
brainstorm below. The always-on harness must remain small. Rusty IDD must own
the task-scoped packages, candidate-goal generation, lifecycle gates,
verification, publishing, and learning loop.

## Approved Brainstorm To Execute

Yes. The path is not "one infinite agent with every tool loaded." It is a
bounded renewable Rusty IDD loop: each cycle discovers work, writes one narrow
goal, generates the right task-scoped package, runs it through OpenSpec,
executes one PR, verifies hard, merges, then starts the next cycle from the new
repo truth.

The repo already has several pieces:

- `rusty-idd knowledge` gives graph/context artifacts.
- `rusty-idd spec status/next` gives lifecycle gates.
- `rusty-idd run` can drive OpenSpec tasks.
- `.codex/loops/rusty-idd-model-loop.toml` already defines read-only
  explore/gap/verify passes.
- `.codex/agents/*` already splits explorer, gap-hunter, verifier,
  implementer roles.
- `codex workflow-check` already enforces active-change, validation, and PR
  evidence.
- `merge-tools` is already a Rusty IDD-owned package model for one workflow
  family.

What is missing is the self-upgrade governor.

## Core Idea

Rusty IDD should own a first-class command family something like:

```bash
rusty-idd self-upgrade scan
rusty-idd self-upgrade propose
rusty-idd self-upgrade goal
rusty-idd self-upgrade package
rusty-idd self-upgrade run
rusty-idd self-upgrade verify
rusty-idd self-upgrade publish
rusty-idd self-upgrade next
```

The always-on harness stays tiny. It only says: "Ask Rusty IDD for the next
scoped package." Rusty IDD does the real routing.

The loop should look like this:

```text
repo truth
  -> scan
  -> opportunity graph
  -> candidate goals
  -> architecture/design reasoning
  -> goal file
  -> OpenSpec change
  -> task-scoped package
  -> implementation
  -> exhaustive verify
  -> PR/merge
  -> ICM + knowledge refresh
  -> next goal
```

## How Rusty IDD Writes Its Own Goals

It should not let a model free-write arbitrary goals directly into execution.
Instead, use a typed pipeline:

```text
Finding
  -> Opportunity
  -> Hypothesis
  -> CandidateGoal
  -> GoalReview
  -> ApprovedGoal
  -> OpenSpecChange
  -> Package
```

Example:

```text
Finding:
  "The verify package exists as docs/goal artifacts but has no first-class CLI package command."

Opportunity:
  "Promote verify package from documented workflow to executable Rusty IDD package."

Hypothesis:
  "A first-class verify package reduces copy/paste prompts and improves post-task quality."

CandidateGoal:
  "Add `rusty-idd harness package --stage verify` that emits contracts, agents, tools, evidence schema, and gates."

GoalReview:
  risk: medium
  blast_radius: cli + docs + tests
  package: verify
  requires_human: false if docs/CLI only; true if changing merge policy

ApprovedGoal:
  saved under `.idd/goals/...`

OpenSpecChange:
  proposal/design/spec/ADR/tasks generated in order.
```

That gives self-authored goals without letting the loop become sloppy.

## The Endless Loop Should Be Two Loops

Split it:

```text
1. Discovery Loop: endless, read-only, cheap
2. Delivery Loop: finite, write-capable, one goal/PR at a time
```

The discovery loop can run forever because it only produces ranked candidate
goals. The delivery loop must always terminate:

- one active goal
- one worktree
- one OpenSpec change
- one PR
- one merge or one blocked handoff
- no hidden background mutation

That prevents agent soup.

## Package Types To Add First

Create Rusty IDD-owned packages in this order:

1. `scan` package

   Finds stale artifacts, missing specs, workflow gaps, CI drift, code
   hotspots, orphaned work, toolchain risk.

2. `goal` package

   Converts findings into candidate goals with risk score, blast radius, owner
   boundary, evidence, and suggested OpenSpec slug.

3. `design` package

   Forces architecture reasoning before implementation. Reads ADRs, OpenSpec,
   `.idd/knowledge`, current code, and prior ICM.

4. `implement` package

   The only write-capable package. Must require ready OpenSpec status.

5. `verify` package

   Exhaustive post-task verifier: original request vs goal vs plan vs diff vs
   tests vs graph vs ICM vs PR evidence.

6. `publish` package

   Commit, push, PR, CI wait with useful parallel work, merge, sync, cleanup.

7. `learn` package

   Stores durable ICM lessons, updates knowledge artifacts, feeds the next
   discovery cycle.

## The Self-Upgrade Governor

This is the missing component. Name it something like `crates/self-upgrade` or
`crates/governor`.

It owns:

```text
Queue:
  candidate goals, approved goals, blocked goals, completed goals

Policy:
  what can run automatically
  what requires user approval
  max risk per cycle
  max file blast radius
  max session duration
  max parallel agents

Scoring:
  correctness impact
  workflow friction removed
  compile/test speed impact
  verification quality
  token/context savings
  user-stated priority

State:
  last scan
  last completed PR
  active worktree
  active change
  current package
  verification result
```

## Important Safety Rule

Do not build a true unbounded write loop. Build an endless read/recommend loop
plus a bounded approve/run/publish loop.

Auto-run can be allowed for low-risk categories:

```text
Allowed auto goals:
  stale generated artifact refresh
  docs/spec consistency repair
  missing validation evidence
  narrow CLI package emission
  test fixture repair
  workflow prompt/package scaffolding

Require approval:
  dependency upgrades
  architecture boundary changes
  toolchain changes
  auth/secrets/env behavior
  deletion/removal
  cross-repo mutation
  CI policy changes
```

## Preferred Path

Start with one vertical slice:

```text
Goal: Add Rusty IDD self-upgrade discovery package.

It should:
  1. scan repo truth
  2. produce candidate goals
  3. rank them
  4. write no code by default
  5. emit a `rusty-idd --goal-file ...` ready artifact
  6. route the chosen candidate into the existing OpenSpec flow
```

Then the next goal can be generated by the new system itself: "Promote candidate
goal into OpenSpec scaffolding."

That is the bootstrap moment. After that, Rusty IDD starts feeding itself
clean, scoped goals instead of relying on giant always-loaded harness prompts.

The north star:

```text
Codex asks: "What package do I need for this goal?"
Rusty IDD answers with a scoped package.
The package produces evidence.
The evidence produces the next goal.
The loop continues, but every write is still reviewable, typed, gated, and PR-shaped.
```

That is how Rusty IDD gets full-auto self-upgrade without turning the harness
into a token furnace.

## First Downstream Test Target

After this goal-artifact pass is complete, the first test target must be Rusty
IDD feature integrations and automations:

- What is the real integration between Rusty IDD, handoff, and prompt_hub?
- Where is the autonomous flow?
- How are the handoff kernel, handoff CLI, and prompt_hub CLI integrated?
- What is the directory structure?
- How do other repos initiate, build the proper directory structure, and sync?

This goal file records that target for the next cycle. Do not research or
implement that target until the self-upgrade governor goal artifacts are
created and validated.
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/self-upgrade-governor-goal`
- Source graph: 141 files, 8824 nodes, 36372 edges via `codegraph-rust`
- Context package: 346 files, 735721 tokens via `repomix-rs`

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
| `cli` | crate | 26 | 992 | 4504 | crates/cli/src/commands/codex.rs, crates/cli/src/commands/core.rs, crates/cli/src/commands/harness.rs, crates/cli/src/commands/knowledge.rs, crates/cli/src/commands/merge_tools.rs, crates/cli/src/commands/mod.rs, crates/cli/src/commands/run.rs, crates/cli/src/commands/spec.rs, crates/cli/src/commands/spec_adr.rs, crates/cli/src/commands/spec_archive.rs, crates/cli/src/commands/spec_plan_integration.rs, crates/cli/src/commands/spec_scaffold.rs |
| `core` | crate | 12 | 448 | 2479 | crates/core/src/cli.rs, crates/core/src/env_contract.rs, crates/core/src/fs_utils.rs, crates/core/src/lib.rs, crates/core/src/manifest.rs, crates/core/src/model.rs, crates/core/src/planner.rs, crates/core/src/scanner.rs, crates/core/src/templates.rs, crates/core/src/validation.rs, crates/core/tests/smoke.rs, crates/core/tests/template_agent_surface.rs |
| `codegraph-parser` | external_crate | 29 | 1060 | 7485 | crates/external/codegraph-parser/src/complexity.rs, crates/external/codegraph-parser/src/diff.rs, crates/external/codegraph-parser/src/edge.rs, crates/external/codegraph-parser/src/fast_io.rs, crates/external/codegraph-parser/src/fast_ml/enhancer.rs, crates/external/codegraph-parser/src/fast_ml/mod.rs, crates/external/codegraph-parser/src/fast_ml/pattern_matcher.rs, crates/external/codegraph-parser/src/fast_ml/symbol_resolver.rs, crates/external/codegraph-parser/src/file_collect.rs, crates/external/codegraph-parser/src/integration_tests.rs, crates/external/codegraph-parser/src/language.rs, crates/external/codegraph-parser/src/languages/cpp.rs |
| `spec` | crate | 24 | 461 | 1969 | crates/spec/src/adr/mod.rs, crates/spec/src/archive/mod.rs, crates/spec/src/lib.rs, crates/spec/src/model/block.rs, crates/spec/src/model/delta.rs, crates/spec/src/model/merge.rs, crates/spec/src/model/mod.rs, crates/spec/src/model/requirement.rs, crates/spec/src/model/spec.rs, crates/spec/src/parse/common.rs, crates/spec/src/parse/delta_parser.rs, crates/spec/src/parse/emit.rs |
| `codegraph-core` | external_crate | 39 | 1918 | 9070 | crates/external/codegraph-core/benches/core_micro.rs, crates/external/codegraph-core/src/advanced_config.rs, crates/external/codegraph-core/src/arena.rs, crates/external/codegraph-core/src/buffer_pool.rs, crates/external/codegraph-core/src/cli_config.rs, crates/external/codegraph-core/src/compression.rs, crates/external/codegraph-core/src/config.rs, crates/external/codegraph-core/src/config_manager.rs, crates/external/codegraph-core/src/embedding_config.rs, crates/external/codegraph-core/src/error.rs, crates/external/codegraph-core/src/incremental/mod.rs, crates/external/codegraph-core/src/incremental/updater.rs |
| `merge-tools` | crate | 1 | 45 | 234 | crates/merge-tools/src/lib.rs |
| `repomix-shared` | external_crate | 2 | 11 | 34 | crates/external/repomix-shared/src/lib.rs, crates/external/repomix-shared/src/types.rs |
| `knowledge` | crate | 1 | 639 | 4653 | crates/knowledge/src/lib.rs |
| `runner` | crate | 4 | 770 | 3135 | crates/runner/src/config.rs, crates/runner/src/data.rs, crates/runner/src/lib.rs, crates/runner/src/runner.rs |
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
| `prompt_hub` | `main` | true | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer |  |
| `ruvector` | `develop` | true | role:agent-environment, role:fleet-handoff, role:rust-code-surface |  |
| `lane` | `main` | false | role:fleet-handoff, role:rust-code-surface |  |
| `envctl` | `master` | true | role:fleet-handoff, role:rust-code-surface, role:toolchain-provider |  |
| `network-control` | `develop` | false | role:fleet-handoff, role:rust-code-surface |  |
| `weave` | `develop` | false | role:agent-environment, role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface |  |
| `agent` | `main` | false | role:agent-environment, role:fleet-handoff, role:rust-code-surface |  |
| `obscura` | `main` | false | role:agent-environment, role:domain-upgrade-surface, role:rust-code-surface |  |
| `kasetto` | `main` | false | role:agent-environment, role:rust-code-surface |  |
| `rtk-tokenkill` | `develop` | false | role:agent-environment, role:rust-code-surface |  |
| `yazelix` | `main` | false | role:parser-runtime-surface, role:toolchain-provider |  |
| `icm` | `fix/containment-claude-p-recursion` | false | role:agent-environment, role:knowledge-memory, role:rust-code-surface |  |
| `meta_git_lib` | `chore/indicatif-0.18-rustsec-2025-0119` | false | role:fleet-handoff, role:meta-control-plane, role:rust-code-surface |  |
| `ECC` | `main` | false | role:agent-environment, role:fleet-handoff |  |
| `loop_cli` | `main` | false | role:meta-control-plane, role:rust-code-surface |  |
| `flexnetos_runner` | `chore/handoff-tier-a-pilot` | false | role:fleet-handoff, role:rust-code-surface |  |
| `github_org` | `fix/autonomous-feature-develop-approval` | false | role:agent-environment, role:fleet-handoff |  |
| `lifeos` | `main` | false | role:fleet-handoff, role:rust-code-surface |  |
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
| `IDD and spec engine` | `layer:executive-control-plane` | partial | repo:handoff, repo:rusty-idd | Rusty IDD built into handoff |
| `Agent harness runtime` | `layer:agent-runtime` | partial | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave | harness-agent-rs rust port |
| `Central and fleet handoff` | `layer:coordination-communication` | partial | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave | handoff central and fleet design |
| `GitHub agent-run upgrades` | `layer:agent-runtime` | partial | repo:grit, repo:yazelix | GRIT from rtk-ai, Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| `Prompt front door` | `layer:front-door-experience` | partial | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt, prompt_hub front door to handoff and rusty-idd |
| `Digital twin simulation` | `layer:simulation-validation` | partial | repo:teri | Teri digital twin simulator |
| `Meta peer repo control` | `layer:executive-control-plane` | partial | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli | meta peer repo system |
| `User front door` | `layer:front-door-experience` | partial | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration, LifeOS front door |
| `Network engineering and control` | `layer:infrastructure-device-fabric` | partial | repo:lane, repo:network-control, repo:network-hub | lane merges into network-manager |
| `Lua and AR interface automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi, repo:yazelix | Lua required for AR glasses workflow, Brilliant Labs Noa style Rust-native agent UX |
| `Vector and agentic runtime` | `layer:knowledge-runtime` | partial | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector | meta-ruvector full agentic system |
| `RTK AI foundation` | `layer:toolchain-parser-runtime` | partial | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox | RTK from rtk-ai, ICM from rtk-ai, VOX from rtk-ai, GRIT from rtk-ai |
| `Parser and terminal runtime` | `layer:toolchain-parser-runtime` | partial | repo:rusty-idd, repo:tool-hub, repo:yazelix | tree-sitter via Yazelix, Yazelix default terminal, nushell, Lua, Ghostty, Zellij |
| `Distributed device fabric` | `layer:infrastructure-device-fabric` | partial | repo:envctl, repo:network-control, repo:oh-my-pi | user devices for distributed compute storage inference memory |
| `Environment and vault relay` | `layer:environment-security` | partial | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| `Domain upgrade path` | `layer:coordination-communication` | partial | repo:obscura, repo:weave | weave plus Obscura domain upgrades |
| `Agent communication layer` | `layer:coordination-communication` | partial | repo:atc, repo:handoff, repo:mcp-hub, repo:weave | weave agent communication layer |
| `Personal media and home automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi | personal life media TV home automation |

## Integration Work

| Priority | Work Item | Change | Owners | Adopt First |
|---:|---|---|---|---|
| 20 | `Integrate Central and fleet handoff` | `integrate-fleet-handoff` | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:meta-git-lib, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:ruvector, repo:teri, repo:weave |  |
| 30 | `Integrate Agent communication layer` | `integrate-agent-communication` | repo:atc, repo:handoff, repo:mcp-hub, repo:weave |  |
| 60 | `Integrate RTK AI foundation` | `integrate-rtk-ai-foundation` | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox |  |
| 70 | `Integrate GitHub agent-run upgrades` | `integrate-github-agent-run-upgrades` | repo:grit, repo:yazelix | Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| 80 | `Integrate Parser and terminal runtime` | `integrate-parser-runtime` | repo:rusty-idd, repo:tool-hub, repo:yazelix |  |
| 90 | `Integrate Vector and agentic runtime` | `integrate-vector-runtime` | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector |  |
| 100 | `Integrate User front door` | `integrate-user-front-door` | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration |
| 120 | `Integrate Network engineering and control` | `integrate-network-engineering` | repo:lane, repo:network-control, repo:network-hub |  |
| 130 | `Integrate Distributed device fabric` | `integrate-distributed-device-fabric` | repo:envctl, repo:network-control, repo:oh-my-pi |  |
| 140 | `Integrate Lua and AR interface automation` | `integrate-lua-ar-interface` | repo:lifeos, repo:oh-my-pi, repo:yazelix |  |
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
