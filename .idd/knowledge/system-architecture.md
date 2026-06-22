# System Architecture Graph

- System root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees`
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/e2e-test-suite`
- Discovery source: `meta project list --json`
- Repos: 65
- Roles: 12
- Edges: 144

## Roles

| Role | Purpose | Repos |
|---|---|---|
| `Agent environment` | Supports agent runtime, skills, prompts, or execution environment | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:hermes-agent, repo:icm, repo:kasetto, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:ruvector |
| `Capability hub` | Groups domain capability repos used by the wider system | repo:commands, repo:database-hub, repo:flow-hub, repo:harness-hub, repo:hooks-hub, repo:mcp-hub, repo:network-hub, repo:plugin-hub, repo:prompt-hub, repo:template-hub, repo:tool-hub, repo:vault-hub |
| `Coordination and domain surface` | Provides orchestration, MCP, and domain-adjacent system coordination surfaces | repo:atc, repo:handoff, repo:mcp-hub, repo:weave |
| `Documentation and knowledge` | Stores documentation and wiki surfaces | repo:flexnetos-brain, repo:flexnetos-wiki, repo:my-wiki, repo:obsidian-mind |
| `Domain upgrade surface` | Contributes domain behavior through weave plus Obscura upgrade paths | repo:obscura |
| `Fleet handoff` | Carries central and fleet handoff state for cross-repo agent continuity | repo:handoff |
| `Rusty IDD control plane` | Owns OpenSpec, ADR, task, validation, manifest, and graph-driven implementation workflow | repo:rusty-idd |
| `Knowledge and memory` | Stores memory or knowledge surfaces used by agents | repo:icm, repo:obsidian-mind |
| `Meta control plane` | Provides parent meta workspace inventory and execution surfaces | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli |
| `Parser/runtime surface` | Carries parser/runtime support such as tree-sitter through Yazelix | repo:yazelix |
| `Spec producer` | Produces intent or prompt artifacts that Rusty IDD can turn into OpenSpec | repo:prompt-hub |
| `Toolchain provider` | Provides parent-managed tools instead of user-global installs | repo:envctl, repo:yazelix |

## Repos

| Repo | Branch | Dirty | Tags | Roles | Markers |
|---|---|---|---|---|---|
| `Archon` | `` | false | ai, forked, harness | role:agent-environment |  |
| `ECC` | `` | false | ai, forked | role:agent-environment |  |
| `agent` | `` | false | ai, framework | role:agent-environment |  |
| `agent-skills` | `` | false | agent-env, ai | role:agent-environment |  |
| `assets` | `` | false | assets |  |  |
| `atc` | `` | false | ai, orchestration, rust | role:agent-environment, role:coordination-domain-surface |  |
| `claude-code` | `` | false | ai, forked | role:agent-environment |  |
| `claude-plugin` | `` | false | ai, plugin | role:agent-environment |  |
| `claude-plugins` | `` | false | ai | role:agent-environment |  |
| `codex` | `` | false | ai, forked | role:agent-environment |  |
| `commands` | `` | false | commands, hub | role:capability-hub |  |
| `copilot-plugin` | `` | false | ai, plugin | role:agent-environment |  |
| `database_hub` | `` | false | database, hub | role:capability-hub |  |
| `envctl` | `` | false | env, tools | role:toolchain-provider |  |
| `flexnetos_brain` | `` | false | data, docs | role:documentation-knowledge |  |
| `flexnetos_github_app` | `` | false | github-app, ops |  |  |
| `flexnetos_runner` | `` | false | ops, runner |  |  |
| `flexnetos_wiki` | `` | false | docs | role:documentation-knowledge |  |
| `flow_hub` | `` | false | flow, hub | role:capability-hub |  |
| `github_org` | `` | false | ci, org |  |  |
| `grit` | `` | false | untriaged |  |  |
| `handoff` | `` | false | handoff, orchestration | role:coordination-domain-surface, role:fleet-handoff |  |
| `harness_hub` | `` | false | harness, hub | role:capability-hub |  |
| `hermes-agent` | `` | false | agents, ai, untriaged | role:agent-environment |  |
| `hooks_hub` | `` | false | hooks, hub | role:capability-hub |  |
| `icm` | `` | false | ai, memory | role:agent-environment, role:knowledge-memory |  |
| `kasetto` | `` | false | agent-env, forked, tools | role:agent-environment |  |
| `lane` | `` | false | tools, workflow |  |  |
| `lifeos` | `` | false | untriaged |  |  |
| `loop_cli` | `` | false | canon | role:meta-control-plane |  |
| `loop_lib` | `` | false | canon | role:meta-control-plane |  |
| `mcp_hub` | `` | false | hub, mcp, meta | role:capability-hub, role:coordination-domain-surface |  |
| `meta-plugins` | `` | false | tools |  |  |
| `meta_cli` | `` | false | canon | role:meta-control-plane |  |
| `meta_core` | `` | false | canon | role:meta-control-plane |  |
| `meta_dashboard_cli` | `` | false |  | role:meta-control-plane |  |
| `meta_git_cli` | `` | false | canon | role:meta-control-plane |  |
| `meta_git_lib` | `` | false | canon | role:meta-control-plane |  |
| `meta_mcp` | `` | false |  | role:meta-control-plane |  |
| `meta_plugin_api` | `` | false | canon | role:meta-control-plane |  |
| `meta_plugin_protocol` | `` | false | canon | role:meta-control-plane |  |
| `meta_project_cli` | `` | false | canon | role:meta-control-plane |  |
| `meta_rust_cli` | `` | false | canon | role:meta-control-plane |  |
| `my-wiki` | `` | false | docs, wiki | role:documentation-knowledge |  |
| `n8n` | `` | false | automation, forked |  |  |
| `network-control` | `` | false | network, tools |  |  |
| `network_hub` | `` | false | hub, network | role:capability-hub |  |
| `obscura` | `` | false | ai, browser, network | role:agent-environment, role:domain-upgrade-surface |  |
| `obsidian-mind` | `` | false | docs, knowledge | role:documentation-knowledge, role:knowledge-memory |  |
| `oh-my-claudecode` | `` | false | ai, forked | role:agent-environment |  |
| `oh-my-pi` | `` | false | ai, forked | role:agent-environment |  |
| `plugin_hub` | `` | false | hub, plugins | role:capability-hub |  |
| `prompt_hub` | `` | false | ai, prompts | role:agent-environment, role:capability-hub, role:spec-producer |  |
| `rtk-tokenkill` | `` | false | ai, optimization, tools | role:agent-environment |  |
| `ruflo` | `` | false | ai, forked, rust, wasm | role:agent-environment |  |
| `rusty-idd` | `` | false | idd, tools | role:idd-control-plane |  |
| `ruvector` | `` | false | ai, crates-only, forked, rust, wasm | role:agent-environment |  |
| `shimmy` | `` | false | forked, untriaged |  |  |
| `template_hub` | `` | false | hub, templates | role:capability-hub |  |
| `teri` | `` | false | forked, untriaged |  |  |
| `tool_hub` | `` | false | hub, tools | role:capability-hub |  |
| `vault_hub` | `` | false | hub, vault | role:capability-hub |  |
| `vox` | `` | false | forked, tools, voice |  |  |
| `weave` | `` | false | mcp, orchestration | role:coordination-domain-surface |  |
| `yazelix` | `` | false | env, forked, terminal | role:parser-runtime-surface, role:toolchain-provider |  |

## Peer Architecture Summaries

No parsed peer architecture summaries.

## Edges

| Source | Kind | Target |
|---|---|---|
| `repo:agent` | provides | `role:agent-environment` |
| `repo:agent-skills` | provides | `role:agent-environment` |
| `repo:archon` | provides | `role:agent-environment` |
| `repo:atc` | provides | `role:agent-environment` |
| `repo:atc` | provides | `role:coordination-domain-surface` |
| `repo:claude-code` | provides | `role:agent-environment` |
| `repo:claude-plugin` | provides | `role:agent-environment` |
| `repo:claude-plugins` | provides | `role:agent-environment` |
| `repo:codex` | provides | `role:agent-environment` |
| `repo:commands` | provides | `role:capability-hub` |
| `repo:copilot-plugin` | provides | `role:agent-environment` |
| `repo:database-hub` | provides | `role:capability-hub` |
| `repo:ecc` | provides | `role:agent-environment` |
| `repo:envctl` | provides | `role:toolchain-provider` |
| `repo:flexnetos-brain` | provides | `role:documentation-knowledge` |
| `repo:flexnetos-wiki` | provides | `role:documentation-knowledge` |
| `repo:flow-hub` | provides | `role:capability-hub` |
| `repo:handoff` | provides | `role:coordination-domain-surface` |
| `repo:handoff` | provides | `role:fleet-handoff` |
| `repo:harness-hub` | provides | `role:capability-hub` |
| `repo:hermes-agent` | provides | `role:agent-environment` |
| `repo:hooks-hub` | provides | `role:capability-hub` |
| `repo:icm` | provides | `role:agent-environment` |
| `repo:icm` | provides | `role:knowledge-memory` |
| `repo:kasetto` | provides | `role:agent-environment` |
| `repo:loop-cli` | provides | `role:meta-control-plane` |
| `repo:loop-lib` | provides | `role:meta-control-plane` |
| `repo:mcp-hub` | provides | `role:capability-hub` |
| `repo:mcp-hub` | provides | `role:coordination-domain-surface` |
| `repo:meta-cli` | provides | `role:meta-control-plane` |
| `repo:meta-core` | provides | `role:meta-control-plane` |
| `repo:meta-dashboard-cli` | provides | `role:meta-control-plane` |
| `repo:meta-git-cli` | provides | `role:meta-control-plane` |
| `repo:meta-git-lib` | provides | `role:meta-control-plane` |
| `repo:meta-mcp` | provides | `role:meta-control-plane` |
| `repo:meta-plugin-api` | provides | `role:meta-control-plane` |
| `repo:meta-plugin-protocol` | provides | `role:meta-control-plane` |
| `repo:meta-project-cli` | provides | `role:meta-control-plane` |
| `repo:meta-rust-cli` | provides | `role:meta-control-plane` |
| `repo:my-wiki` | provides | `role:documentation-knowledge` |
| `repo:network-hub` | provides | `role:capability-hub` |
| `repo:obscura` | provides | `role:agent-environment` |
| `repo:obscura` | provides | `role:domain-upgrade-surface` |
| `repo:obsidian-mind` | provides | `role:documentation-knowledge` |
| `repo:obsidian-mind` | provides | `role:knowledge-memory` |
| `repo:oh-my-claudecode` | provides | `role:agent-environment` |
| `repo:oh-my-pi` | provides | `role:agent-environment` |
| `repo:plugin-hub` | provides | `role:capability-hub` |
| `repo:prompt-hub` | provides | `role:agent-environment` |
| `repo:prompt-hub` | provides | `role:capability-hub` |
| `repo:prompt-hub` | provides | `role:spec-producer` |
| `repo:rtk-tokenkill` | provides | `role:agent-environment` |
| `repo:ruflo` | provides | `role:agent-environment` |
| `repo:rusty-idd` | maps_for_automation | `role:agent-environment` |
| `repo:rusty-idd` | maps_for_automation | `role:capability-hub` |
| `repo:rusty-idd` | maps_for_automation | `role:coordination-domain-surface` |
| `repo:rusty-idd` | scopes_as_feature_gated_surface | `role:coordination-domain-surface` |
| `repo:rusty-idd` | maps_for_automation | `role:documentation-knowledge` |
| `repo:rusty-idd` | maps_for_automation | `role:domain-upgrade-surface` |
| `repo:rusty-idd` | scopes_as_feature_gated_surface | `role:domain-upgrade-surface` |
| `repo:rusty-idd` | maps_for_automation | `role:fleet-handoff` |
| `repo:rusty-idd` | uses_for_continuity | `role:fleet-handoff` |
| `repo:rusty-idd` | provides | `role:idd-control-plane` |
| `repo:rusty-idd` | maps_for_automation | `role:knowledge-memory` |
| `repo:rusty-idd` | maps_for_automation | `role:meta-control-plane` |
| `repo:rusty-idd` | uses_for_workspace_inventory | `role:meta-control-plane` |
| `repo:rusty-idd` | maps_for_automation | `role:parser-runtime-surface` |
| `repo:rusty-idd` | uses_as_parser_runtime_evidence | `role:parser-runtime-surface` |
| `repo:rusty-idd` | consumes_spec_intent_from | `role:spec-producer` |
| `repo:rusty-idd` | maps_for_automation | `role:spec-producer` |
| `repo:rusty-idd` | maps_for_automation | `role:toolchain-provider` |
| `repo:rusty-idd` | uses_for_parent_managed_tools | `role:toolchain-provider` |
| `repo:ruvector` | provides | `role:agent-environment` |
| `repo:template-hub` | provides | `role:capability-hub` |
| `repo:tool-hub` | provides | `role:capability-hub` |
| `repo:vault-hub` | provides | `role:capability-hub` |
| `repo:weave` | provides | `role:coordination-domain-surface` |
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
- 0 repos have local dirty state recorded as evidence
- 0 repos expose .idd/knowledge/architecture.json
- 0 repos expose parsed architecture summaries
