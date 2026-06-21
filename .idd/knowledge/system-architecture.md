# System Architecture Graph

- System root: `/home/drdave/Desktop/meta`
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd`
- Discovery source: `meta project list --json`
- Repos: 65
- Roles: 13
- Edges: 196

## Roles

| Role | Purpose | Repos |
|---|---|---|
| `Agent environment` | Supports agent runtime, skills, prompts, or execution environment | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:envctl, repo:github-org, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave |
| `Capability hub` | Groups domain capability repos used by the wider system | repo:commands, repo:database-hub, repo:flow-hub, repo:harness-hub, repo:hooks-hub, repo:mcp-hub, repo:network-hub, repo:plugin-hub, repo:prompt-hub, repo:template-hub, repo:tool-hub, repo:vault-hub |
| `Coordination and domain surface` | Provides orchestration, MCP, and domain-adjacent system coordination surfaces | repo:atc, repo:handoff, repo:mcp-hub, repo:weave |
| `Documentation and knowledge` | Stores documentation and wiki surfaces | repo:flexnetos-brain, repo:flexnetos-wiki, repo:my-wiki, repo:obsidian-mind |
| `Domain upgrade surface` | Contributes domain behavior through weave plus Obscura upgrade paths | repo:obscura |
| `Fleet handoff` | Carries central and fleet handoff state for cross-repo agent continuity | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:weave |
| `Rusty IDD control plane` | Owns OpenSpec, ADR, task, validation, manifest, and graph-driven implementation workflow | repo:rusty-idd |
| `Knowledge and memory` | Stores memory or knowledge surfaces used by agents | repo:icm, repo:obsidian-mind |
| `Meta control plane` | Provides parent meta workspace inventory and execution surfaces | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli |
| `Parser/runtime surface` | Carries parser/runtime support such as tree-sitter through Yazelix | repo:yazelix |
| `Rust code surface` | Contains Rust source that can be indexed by CodeGraph-backed Rusty IDD knowledge | repo:agent, repo:atc, repo:envctl, repo:flexnetos-github-app, repo:flexnetos-runner, repo:grit, repo:handoff, repo:icm, repo:kasetto, repo:lane, repo:lifeos, repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli, repo:network-control, repo:obscura, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:rusty-idd, repo:ruvector, repo:shimmy, repo:teri, repo:vox, repo:weave |
| `Spec producer` | Produces intent or prompt artifacts that Rusty IDD can turn into OpenSpec | repo:prompt-hub |
| `Toolchain provider` | Provides parent-managed tools instead of user-global installs | repo:envctl, repo:yazelix |

## Repos

| Repo | Branch | Dirty | Tags | Roles | Markers |
|---|---|---|---|---|---|
| `Archon` | `dev` | false | ai, forked, harness | role:agent-environment | node, claude, github-actions |
| `ECC` | `main` | false | ai, forked | role:agent-environment, role:fleet-handoff | node, handoff, agents, claude, github-actions |
| `agent` | `main` | false | ai, framework | role:agent-environment, role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions |
| `agent-skills` | `` | false | agent-env, ai | role:agent-environment |  |
| `assets` | `` | false | assets |  |  |
| `atc` | `main` | false | ai, orchestration, rust | role:agent-environment, role:coordination-domain-surface, role:rust-code-surface | rust, node, claude, github-actions, make |
| `claude-code` | `main` | false | ai, forked | role:agent-environment | claude, github-actions |
| `claude-plugin` | `main` | true | ai, plugin | role:agent-environment |  |
| `claude-plugins` | `main` | false | ai | role:agent-environment |  |
| `codex` | `main` | true | ai, forked | role:agent-environment | node, github-actions |
| `commands` | `feat/recall-remember-speak-commands` | false | commands, hub | role:capability-hub | github-actions |
| `copilot-plugin` | `main` | true | ai, plugin | role:agent-environment |  |
| `database_hub` | `master` | false | database, hub | role:capability-hub | github-actions |
| `envctl` | `master` | true | env, tools | role:agent-environment, role:fleet-handoff, role:rust-code-surface, role:toolchain-provider | rust, handoff, agents, claude, github-actions |
| `flexnetos_brain` | `` | false | data, docs | role:documentation-knowledge |  |
| `flexnetos_github_app` | `main` | false | github-app, ops | role:rust-code-surface | rust, github-actions |
| `flexnetos_runner` | `chore/handoff-tier-a-pilot` | false | ops, runner | role:fleet-handoff, role:rust-code-surface | rust, handoff, github-actions |
| `flexnetos_wiki` | `` | false | docs | role:documentation-knowledge |  |
| `flow_hub` | `master` | false | flow, hub | role:capability-hub | github-actions |
| `github_org` | `chore/wrap-up-2026-06-21-002` | false | ci, org | role:agent-environment, role:fleet-handoff | handoff, agents, claude, github-actions, make |
| `grit` | `master` | false | untriaged | role:rust-code-surface | rust, github-actions |
| `handoff` | `fix/windows-ledger-path-and-promote-checkout` | true | handoff, orchestration | role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions, make |
| `harness_hub` | `master` | false | harness, hub | role:capability-hub, role:fleet-handoff | handoff, github-actions |
| `hermes-agent` | `main` | false | agents, ai, untriaged | role:agent-environment | node, github-actions |
| `hooks_hub` | `master` | false | hooks, hub | role:capability-hub | github-actions |
| `icm` | `fix/containment-claude-p-recursion` | false | ai, memory | role:agent-environment, role:knowledge-memory, role:rust-code-surface | rust, claude, github-actions |
| `kasetto` | `main` | false | agent-env, forked, tools | role:agent-environment, role:rust-code-surface | rust, github-actions |
| `lane` | `main` | false | tools, workflow | role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions |
| `lifeos` | `main` | false | untriaged | role:fleet-handoff, role:rust-code-surface | rust, node, openspec, handoff, claude |
| `loop_cli` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `loop_lib` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `mcp_hub` | `master` | false | hub, mcp, meta | role:capability-hub, role:coordination-domain-surface | github-actions |
| `meta-plugins` | `main` | false | tools |  | github-actions |
| `meta_cli` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, claude, github-actions |
| `meta_core` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `meta_dashboard_cli` | `master` | true |  | role:meta-control-plane, role:rust-code-surface | rust |
| `meta_git_cli` | `feat/dep-upgrades` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `meta_git_lib` | `feat/dep-upgrades` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `meta_mcp` | `main` | false |  | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `meta_plugin_api` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust |
| `meta_plugin_protocol` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `meta_project_cli` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `meta_rust_cli` | `main` | false | canon | role:meta-control-plane, role:rust-code-surface | rust, github-actions |
| `my-wiki` | `` | false | docs, wiki | role:documentation-knowledge |  |
| `n8n` | `harness/epic-d` | false | automation, forked | role:agent-environment | node, agents, claude, github-actions |
| `network-control` | `fix/handoff-remove-hand-rolled-cards` | false |  | role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions |
| `network_hub` | `master` | false | hub, network | role:capability-hub | github-actions |
| `obscura` | `main` | false | untriaged | role:domain-upgrade-surface, role:rust-code-surface | rust, github-actions |
| `obsidian-mind` | `main` | false | docs, knowledge | role:documentation-knowledge, role:knowledge-memory | claude, github-actions |
| `oh-my-claudecode` | `main` | false | ai, forked | role:agent-environment | node, github-actions |
| `oh-my-pi` | `main` | false | ai, forked | role:agent-environment, role:rust-code-surface | rust, node, github-actions |
| `plugin_hub` | `master` | false | hub, plugins | role:capability-hub | github-actions |
| `prompt_hub` | `main` | true | ai, prompts | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer | rust, handoff, claude, github-actions |
| `rtk-tokenkill` | `develop` | false | ai, optimization, tools | role:agent-environment, role:rust-code-surface | rust, claude, github-actions |
| `ruflo` | `main` | true | ai, forked, rust, wasm | role:agent-environment | node, agents, claude, github-actions |
| `rusty-idd` | `` | false | idd, tools | role:agent-environment, role:fleet-handoff, role:idd-control-plane, role:rust-code-surface | rust, openspec, idd-knowledge, handoff, agents, claude, github-actions, make, just |
| `ruvector` | `main` | false | ai, forked, rust, wasm | role:agent-environment, role:rust-code-surface | rust, node, claude, github-actions |
| `shimmy` | `feat/openai-embeddings-endpoint` | false | forked, untriaged | role:rust-code-surface | rust, github-actions, make |
| `template_hub` | `master` | false | hub, templates | role:capability-hub | github-actions |
| `teri` | `fix/keyless-envctl-gguf-guard` | false | forked, untriaged | role:rust-code-surface | rust, claude |
| `tool_hub` | `feat/stage-github-org-tool-pins` | true | hub, tools | role:capability-hub | github-actions |
| `vault_hub` | `main` | false | hub, vault | role:capability-hub |  |
| `vox` | `main` | false | forked, tools, voice | role:rust-code-surface | rust, claude, github-actions |
| `weave` | `develop` | false | mcp, orchestration | role:agent-environment, role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface | rust, handoff, agents, claude, github-actions |
| `yazelix` | `main` | false | env, forked, terminal | role:parser-runtime-surface, role:toolchain-provider | claude, github-actions |

## Peer Architecture Summaries

| Repo | Source Graph | Context Package | Surfaces | Top Components |
|---|---|---|---:|---|
| `rusty-idd` | 135 files, 8287 nodes, 33408 edges via `codegraph-rust` | 158 files, 113941 tokens via `repomix-rs` | 4 | codegraph-core, codegraph-parser, tui, knowledge, runner |

## Edges

| Source | Kind | Target |
|---|---|---|
| `repo:agent` | provides | `role:agent-environment` |
| `repo:agent` | provides | `role:fleet-handoff` |
| `repo:agent` | provides | `role:rust-code-surface` |
| `repo:agent-skills` | provides | `role:agent-environment` |
| `repo:archon` | provides | `role:agent-environment` |
| `repo:atc` | provides | `role:agent-environment` |
| `repo:atc` | provides | `role:coordination-domain-surface` |
| `repo:atc` | provides | `role:rust-code-surface` |
| `repo:claude-code` | provides | `role:agent-environment` |
| `repo:claude-plugin` | provides | `role:agent-environment` |
| `repo:claude-plugins` | provides | `role:agent-environment` |
| `repo:codex` | provides | `role:agent-environment` |
| `repo:commands` | provides | `role:capability-hub` |
| `repo:copilot-plugin` | provides | `role:agent-environment` |
| `repo:database-hub` | provides | `role:capability-hub` |
| `repo:ecc` | provides | `role:agent-environment` |
| `repo:ecc` | provides | `role:fleet-handoff` |
| `repo:envctl` | provides | `role:agent-environment` |
| `repo:envctl` | provides | `role:fleet-handoff` |
| `repo:envctl` | provides | `role:rust-code-surface` |
| `repo:envctl` | provides | `role:toolchain-provider` |
| `repo:flexnetos-brain` | provides | `role:documentation-knowledge` |
| `repo:flexnetos-github-app` | provides | `role:rust-code-surface` |
| `repo:flexnetos-runner` | provides | `role:fleet-handoff` |
| `repo:flexnetos-runner` | provides | `role:rust-code-surface` |
| `repo:flexnetos-wiki` | provides | `role:documentation-knowledge` |
| `repo:flow-hub` | provides | `role:capability-hub` |
| `repo:github-org` | provides | `role:agent-environment` |
| `repo:github-org` | provides | `role:fleet-handoff` |
| `repo:grit` | provides | `role:rust-code-surface` |
| `repo:handoff` | provides | `role:coordination-domain-surface` |
| `repo:handoff` | provides | `role:fleet-handoff` |
| `repo:handoff` | provides | `role:rust-code-surface` |
| `repo:harness-hub` | provides | `role:capability-hub` |
| `repo:harness-hub` | provides | `role:fleet-handoff` |
| `repo:hermes-agent` | provides | `role:agent-environment` |
| `repo:hooks-hub` | provides | `role:capability-hub` |
| `repo:icm` | provides | `role:agent-environment` |
| `repo:icm` | provides | `role:knowledge-memory` |
| `repo:icm` | provides | `role:rust-code-surface` |
| `repo:kasetto` | provides | `role:agent-environment` |
| `repo:kasetto` | provides | `role:rust-code-surface` |
| `repo:lane` | provides | `role:fleet-handoff` |
| `repo:lane` | provides | `role:rust-code-surface` |
| `repo:lifeos` | provides | `role:fleet-handoff` |
| `repo:lifeos` | provides | `role:rust-code-surface` |
| `repo:loop-cli` | provides | `role:meta-control-plane` |
| `repo:loop-cli` | provides | `role:rust-code-surface` |
| `repo:loop-lib` | provides | `role:meta-control-plane` |
| `repo:loop-lib` | provides | `role:rust-code-surface` |
| `repo:mcp-hub` | provides | `role:capability-hub` |
| `repo:mcp-hub` | provides | `role:coordination-domain-surface` |
| `repo:meta-cli` | provides | `role:meta-control-plane` |
| `repo:meta-cli` | provides | `role:rust-code-surface` |
| `repo:meta-core` | provides | `role:meta-control-plane` |
| `repo:meta-core` | provides | `role:rust-code-surface` |
| `repo:meta-dashboard-cli` | provides | `role:meta-control-plane` |
| `repo:meta-dashboard-cli` | provides | `role:rust-code-surface` |
| `repo:meta-git-cli` | provides | `role:meta-control-plane` |
| `repo:meta-git-cli` | provides | `role:rust-code-surface` |
| `repo:meta-git-lib` | provides | `role:meta-control-plane` |
| `repo:meta-git-lib` | provides | `role:rust-code-surface` |
| `repo:meta-mcp` | provides | `role:meta-control-plane` |
| `repo:meta-mcp` | provides | `role:rust-code-surface` |
| `repo:meta-plugin-api` | provides | `role:meta-control-plane` |
| `repo:meta-plugin-api` | provides | `role:rust-code-surface` |
| `repo:meta-plugin-protocol` | provides | `role:meta-control-plane` |
| `repo:meta-plugin-protocol` | provides | `role:rust-code-surface` |
| `repo:meta-project-cli` | provides | `role:meta-control-plane` |
| `repo:meta-project-cli` | provides | `role:rust-code-surface` |
| `repo:meta-rust-cli` | provides | `role:meta-control-plane` |
| `repo:meta-rust-cli` | provides | `role:rust-code-surface` |
| `repo:my-wiki` | provides | `role:documentation-knowledge` |
| `repo:n8n` | provides | `role:agent-environment` |
| `repo:network-control` | provides | `role:fleet-handoff` |
| `repo:network-control` | provides | `role:rust-code-surface` |
| `repo:network-hub` | provides | `role:capability-hub` |
| `repo:obscura` | provides | `role:domain-upgrade-surface` |
| `repo:obscura` | provides | `role:rust-code-surface` |
| `repo:obsidian-mind` | provides | `role:documentation-knowledge` |
| `repo:obsidian-mind` | provides | `role:knowledge-memory` |
| `repo:oh-my-claudecode` | provides | `role:agent-environment` |
| `repo:oh-my-pi` | provides | `role:agent-environment` |
| `repo:oh-my-pi` | provides | `role:rust-code-surface` |
| `repo:plugin-hub` | provides | `role:capability-hub` |
| `repo:prompt-hub` | provides | `role:agent-environment` |
| `repo:prompt-hub` | provides | `role:capability-hub` |
| `repo:prompt-hub` | provides | `role:fleet-handoff` |
| `repo:prompt-hub` | provides | `role:rust-code-surface` |
| `repo:prompt-hub` | provides | `role:spec-producer` |
| `repo:rtk-tokenkill` | provides | `role:agent-environment` |
| `repo:rtk-tokenkill` | provides | `role:rust-code-surface` |
| `repo:ruflo` | provides | `role:agent-environment` |
| `repo:rusty-idd` | publishes | `artifact:.idd/knowledge/architecture.json` |
| `repo:rusty-idd` | maps_for_automation | `role:agent-environment` |
| `repo:rusty-idd` | provides | `role:agent-environment` |
| `repo:rusty-idd` | maps_for_automation | `role:capability-hub` |
| `repo:rusty-idd` | maps_for_automation | `role:coordination-domain-surface` |
| `repo:rusty-idd` | scopes_as_feature_gated_surface | `role:coordination-domain-surface` |
| `repo:rusty-idd` | maps_for_automation | `role:documentation-knowledge` |
| `repo:rusty-idd` | maps_for_automation | `role:domain-upgrade-surface` |
| `repo:rusty-idd` | scopes_as_feature_gated_surface | `role:domain-upgrade-surface` |
| `repo:rusty-idd` | maps_for_automation | `role:fleet-handoff` |
| `repo:rusty-idd` | provides | `role:fleet-handoff` |
| `repo:rusty-idd` | uses_for_continuity | `role:fleet-handoff` |
| `repo:rusty-idd` | provides | `role:idd-control-plane` |
| `repo:rusty-idd` | maps_for_automation | `role:knowledge-memory` |
| `repo:rusty-idd` | maps_for_automation | `role:meta-control-plane` |
| `repo:rusty-idd` | uses_for_workspace_inventory | `role:meta-control-plane` |
| `repo:rusty-idd` | maps_for_automation | `role:parser-runtime-surface` |
| `repo:rusty-idd` | uses_as_parser_runtime_evidence | `role:parser-runtime-surface` |
| `repo:rusty-idd` | maps_for_automation | `role:rust-code-surface` |
| `repo:rusty-idd` | provides | `role:rust-code-surface` |
| `repo:rusty-idd` | consumes_spec_intent_from | `role:spec-producer` |
| `repo:rusty-idd` | maps_for_automation | `role:spec-producer` |
| `repo:rusty-idd` | maps_for_automation | `role:toolchain-provider` |
| `repo:rusty-idd` | uses_for_parent_managed_tools | `role:toolchain-provider` |
| `repo:ruvector` | provides | `role:agent-environment` |
| `repo:ruvector` | provides | `role:rust-code-surface` |
| `repo:shimmy` | provides | `role:rust-code-surface` |
| `repo:template-hub` | provides | `role:capability-hub` |
| `repo:teri` | provides | `role:rust-code-surface` |
| `repo:tool-hub` | provides | `role:capability-hub` |
| `repo:vault-hub` | provides | `role:capability-hub` |
| `repo:vox` | provides | `role:rust-code-surface` |
| `repo:weave` | provides | `role:agent-environment` |
| `repo:weave` | provides | `role:coordination-domain-surface` |
| `repo:weave` | provides | `role:fleet-handoff` |
| `repo:weave` | provides | `role:rust-code-surface` |
| `repo:yazelix` | provides | `role:parser-runtime-surface` |
| `repo:yazelix` | provides | `role:toolchain-provider` |
| `system:meta-workspace` | contains | `repo:agent` |
| `system:meta-workspace` | contains | `repo:agent-skills` |
| `system:meta-workspace` | contains | `repo:archon` |
| `system:meta-workspace` | contains | `repo:assets` |
| `system:meta-workspace` | contains | `repo:atc` |
| `system:meta-workspace` | contains | `repo:claude-code` |
| `system:meta-workspace` | contains | `repo:claude-plugin` |
| `system:meta-workspace` | contains | `repo:claude-plugins` |
| `system:meta-workspace` | contains | `repo:codex` |
| `system:meta-workspace` | contains | `repo:commands` |
| `system:meta-workspace` | contains | `repo:copilot-plugin` |
| `system:meta-workspace` | contains | `repo:database-hub` |
| `system:meta-workspace` | contains | `repo:ecc` |
| `system:meta-workspace` | contains | `repo:envctl` |
| `system:meta-workspace` | contains | `repo:flexnetos-brain` |
| `system:meta-workspace` | contains | `repo:flexnetos-github-app` |
| `system:meta-workspace` | contains | `repo:flexnetos-runner` |
| `system:meta-workspace` | contains | `repo:flexnetos-wiki` |
| `system:meta-workspace` | contains | `repo:flow-hub` |
| `system:meta-workspace` | contains | `repo:github-org` |
| `system:meta-workspace` | contains | `repo:grit` |
| `system:meta-workspace` | contains | `repo:handoff` |
| `system:meta-workspace` | contains | `repo:harness-hub` |
| `system:meta-workspace` | contains | `repo:hermes-agent` |
| `system:meta-workspace` | contains | `repo:hooks-hub` |
| `system:meta-workspace` | contains | `repo:icm` |
| `system:meta-workspace` | contains | `repo:kasetto` |
| `system:meta-workspace` | contains | `repo:lane` |
| `system:meta-workspace` | contains | `repo:lifeos` |
| `system:meta-workspace` | contains | `repo:loop-cli` |
| `system:meta-workspace` | contains | `repo:loop-lib` |
| `system:meta-workspace` | contains | `repo:mcp-hub` |
| `system:meta-workspace` | contains | `repo:meta-cli` |
| `system:meta-workspace` | contains | `repo:meta-core` |
| `system:meta-workspace` | contains | `repo:meta-dashboard-cli` |
| `system:meta-workspace` | contains | `repo:meta-git-cli` |
| `system:meta-workspace` | contains | `repo:meta-git-lib` |
| `system:meta-workspace` | contains | `repo:meta-mcp` |
| `system:meta-workspace` | contains | `repo:meta-plugin-api` |
| `system:meta-workspace` | contains | `repo:meta-plugin-protocol` |
| `system:meta-workspace` | contains | `repo:meta-plugins` |
| `system:meta-workspace` | contains | `repo:meta-project-cli` |
| `system:meta-workspace` | contains | `repo:meta-rust-cli` |
| `system:meta-workspace` | contains | `repo:my-wiki` |
| `system:meta-workspace` | contains | `repo:n8n` |
| `system:meta-workspace` | contains | `repo:network-control` |
| `system:meta-workspace` | contains | `repo:network-hub` |
| `system:meta-workspace` | contains | `repo:obscura` |
| `system:meta-workspace` | contains | `repo:obsidian-mind` |
| `system:meta-workspace` | contains | `repo:oh-my-claudecode` |
| `system:meta-workspace` | contains | `repo:oh-my-pi` |
| `system:meta-workspace` | contains | `repo:plugin-hub` |
| `system:meta-workspace` | contains | `repo:prompt-hub` |
| `system:meta-workspace` | contains | `repo:rtk-tokenkill` |
| `system:meta-workspace` | contains | `repo:ruflo` |
| `system:meta-workspace` | contains | `repo:rusty-idd` |
| `system:meta-workspace` | contains | `repo:ruvector` |
| `system:meta-workspace` | contains | `repo:shimmy` |
| `system:meta-workspace` | contains | `repo:template-hub` |
| `system:meta-workspace` | contains | `repo:teri` |
| `system:meta-workspace` | contains | `repo:tool-hub` |
| `system:meta-workspace` | contains | `repo:vault-hub` |
| `system:meta-workspace` | contains | `repo:vox` |
| `system:meta-workspace` | contains | `repo:weave` |
| `system:meta-workspace` | contains | `repo:yazelix` |

## Findings

- discovered 65 peer repos from meta project list --json
- 9 repos have local dirty state recorded as evidence
- 1 repos expose .idd/knowledge/architecture.json
- 1 repos expose parsed architecture summaries
