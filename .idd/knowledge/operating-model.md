# System Operating Model

- System root: `/home/drdave/Desktop/meta`
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd`
- Source graph: `.idd/knowledge/system-architecture.json`
- Layers: 11
- Capabilities: 19
- Edges: 162

## Layers

| Layer | Purpose | Capabilities | Repos |
|---|---|---:|---:|
| `Agent runtime` | Agent harnesses, execution workers, and automation runtimes | 2 | 27 |
| `Coordination and communication` | Agent communication, orchestration, and cross-agent continuity | 3 | 17 |
| `Environment and security` | Vault, key relay, certificates, and parent-managed toolchains | 1 | 3 |
| `Executive control plane` | Company-level command, OpenSpec, handoff, and repo governance | 2 | 14 |
| `Front door experience` | Prompt, chat, LifeOS, and operator-facing user experience surfaces | 2 | 3 |
| `Governance and reasoning` | Board-style reasoning, strategy, and policy without direct execution | 1 | 5 |
| `Infrastructure and device fabric` | Network control plus distributed device compute, storage, inference, and memory | 2 | 5 |
| `Interface automation` | AR-glasses workflow, local automation, media, and home interfaces | 2 | 3 |
| `Knowledge and runtime` | Memory, vector/progress databases, inference, training, and runtime state | 1 | 4 |
| `Simulation and validation` | Digital twin simulation and high-fidelity failure space for agents | 1 | 1 |
| `Toolchain and parser runtime` | Tree-sitter, Lua, terminal/runtime, parser, and toolchain surfaces | 2 | 7 |

## Capabilities

| Capability | Layer | Status | Repos | Anchors |
|---|---|---|---|---|
| `Agent communication layer` | `layer:coordination-communication` | partial | repo:atc, repo:handoff, repo:mcp-hub, repo:weave | weave agent communication layer |
| `Agent harness runtime` | `layer:agent-runtime` | partial | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:github-org, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:rusty-idd, repo:ruvector, repo:weave | harness-agent-rs rust port |
| `Board reasoning layer` | `layer:governance-reasoning` | partial | repo:flexnetos-brain, repo:flexnetos-wiki, repo:icm, repo:my-wiki, repo:obsidian-mind | company hierarchy board layer |
| `Digital twin simulation` | `layer:simulation-validation` | partial | repo:teri | Teri digital twin simulator |
| `Distributed device fabric` | `layer:infrastructure-device-fabric` | partial | repo:envctl, repo:network-control, repo:oh-my-pi | user devices for distributed compute storage inference memory |
| `Domain upgrade path` | `layer:coordination-communication` | partial | repo:obscura, repo:weave | weave plus Obscura domain upgrades |
| `Environment and vault relay` | `layer:environment-security` | partial | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| `Central and fleet handoff` | `layer:coordination-communication` | partial | repo:agent, repo:ecc, repo:envctl, repo:flexnetos-runner, repo:github-org, repo:handoff, repo:harness-hub, repo:lane, repo:lifeos, repo:network-control, repo:prompt-hub, repo:rusty-idd, repo:teri, repo:weave | handoff central and fleet design |
| `GitHub agent-run upgrades` | `layer:agent-runtime` | partial | repo:grit, repo:yazelix | GRIT from rtk-ai, Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| `IDD and spec engine` | `layer:executive-control-plane` | partial | repo:handoff, repo:rusty-idd | Rusty IDD built into handoff |
| `Lua and AR interface automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi, repo:yazelix | Lua required for AR glasses workflow, Brilliant Labs Noa style Rust-native agent UX |
| `Meta peer repo control` | `layer:executive-control-plane` | partial | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli | meta peer repo system |
| `Network engineering and control` | `layer:infrastructure-device-fabric` | partial | repo:lane, repo:network-control, repo:network-hub | lane merges into network-manager |
| `Parser and terminal runtime` | `layer:toolchain-parser-runtime` | partial | repo:rusty-idd, repo:tool-hub, repo:yazelix | tree-sitter via Yazelix, Yazelix default terminal, nushell, Lua, Ghostty, Zellij |
| `Personal media and home automation` | `layer:interface-automation` | partial | repo:lifeos, repo:oh-my-pi | personal life media TV home automation |
| `Prompt front door` | `layer:front-door-experience` | partial | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt, prompt_hub front door to handoff and rusty-idd |
| `RTK AI foundation` | `layer:toolchain-parser-runtime` | partial | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox | RTK from rtk-ai, ICM from rtk-ai, VOX from rtk-ai, GRIT from rtk-ai |
| `User front door` | `layer:front-door-experience` | partial | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration, LifeOS front door |
| `Vector and agentic runtime` | `layer:knowledge-runtime` | partial | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector | meta-ruvector full agentic system |

## Edges

| Source | Kind | Target |
|---|---|---|
| `capability:agent-communication` | records_anchor | `anchor:weave-agent-communication-layer` |
| `capability:agent-communication` | mapped_to_repo | `repo:atc` |
| `capability:agent-communication` | mapped_to_repo | `repo:handoff` |
| `capability:agent-communication` | mapped_to_repo | `repo:mcp-hub` |
| `capability:agent-communication` | mapped_to_repo | `repo:weave` |
| `capability:agent-harness` | records_anchor | `anchor:harness-agent-rs-rust-port` |
| `capability:agent-harness` | mapped_to_repo | `repo:agent` |
| `capability:agent-harness` | mapped_to_repo | `repo:agent-skills` |
| `capability:agent-harness` | mapped_to_repo | `repo:archon` |
| `capability:agent-harness` | mapped_to_repo | `repo:atc` |
| `capability:agent-harness` | mapped_to_repo | `repo:claude-code` |
| `capability:agent-harness` | mapped_to_repo | `repo:claude-plugin` |
| `capability:agent-harness` | mapped_to_repo | `repo:claude-plugins` |
| `capability:agent-harness` | mapped_to_repo | `repo:codex` |
| `capability:agent-harness` | mapped_to_repo | `repo:copilot-plugin` |
| `capability:agent-harness` | mapped_to_repo | `repo:ecc` |
| `capability:agent-harness` | mapped_to_repo | `repo:flexnetos-runner` |
| `capability:agent-harness` | mapped_to_repo | `repo:github-org` |
| `capability:agent-harness` | mapped_to_repo | `repo:harness-hub` |
| `capability:agent-harness` | mapped_to_repo | `repo:hermes-agent` |
| `capability:agent-harness` | mapped_to_repo | `repo:icm` |
| `capability:agent-harness` | mapped_to_repo | `repo:kasetto` |
| `capability:agent-harness` | mapped_to_repo | `repo:n8n` |
| `capability:agent-harness` | mapped_to_repo | `repo:oh-my-claudecode` |
| `capability:agent-harness` | mapped_to_repo | `repo:oh-my-pi` |
| `capability:agent-harness` | mapped_to_repo | `repo:prompt-hub` |
| `capability:agent-harness` | mapped_to_repo | `repo:rtk-tokenkill` |
| `capability:agent-harness` | mapped_to_repo | `repo:ruflo` |
| `capability:agent-harness` | mapped_to_repo | `repo:rusty-idd` |
| `capability:agent-harness` | mapped_to_repo | `repo:ruvector` |
| `capability:agent-harness` | mapped_to_repo | `repo:weave` |
| `capability:board-reasoning` | records_anchor | `anchor:company-hierarchy-board-layer` |
| `capability:board-reasoning` | mapped_to_repo | `repo:flexnetos-brain` |
| `capability:board-reasoning` | mapped_to_repo | `repo:flexnetos-wiki` |
| `capability:board-reasoning` | mapped_to_repo | `repo:icm` |
| `capability:board-reasoning` | mapped_to_repo | `repo:my-wiki` |
| `capability:board-reasoning` | mapped_to_repo | `repo:obsidian-mind` |
| `capability:digital-twin-simulation` | records_anchor | `anchor:teri-digital-twin-simulator` |
| `capability:digital-twin-simulation` | mapped_to_repo | `repo:teri` |
| `capability:distributed-device-fabric` | records_anchor | `anchor:user-devices-for-distributed-compute-storage-inference-memory` |
| `capability:distributed-device-fabric` | mapped_to_repo | `repo:envctl` |
| `capability:distributed-device-fabric` | mapped_to_repo | `repo:network-control` |
| `capability:distributed-device-fabric` | mapped_to_repo | `repo:oh-my-pi` |
| `capability:domain-upgrade` | records_anchor | `anchor:weave-plus-obscura-domain-upgrades` |
| `capability:domain-upgrade` | mapped_to_repo | `repo:obscura` |
| `capability:domain-upgrade` | mapped_to_repo | `repo:weave` |
| `capability:env-vault-relay` | records_anchor | `anchor:cognitum-vault-on-pi-zero` |
| `capability:env-vault-relay` | records_anchor | `anchor:run-media-drdave-cognitum` |
| `capability:env-vault-relay` | mapped_to_repo | `repo:envctl` |
| `capability:env-vault-relay` | mapped_to_repo | `repo:vault-hub` |
| `capability:env-vault-relay` | mapped_to_repo | `repo:yazelix` |
| `capability:fleet-handoff` | records_anchor | `anchor:handoff-central-and-fleet-design` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:agent` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:ecc` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:envctl` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:flexnetos-runner` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:github-org` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:handoff` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:harness-hub` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:lane` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:lifeos` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:network-control` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:prompt-hub` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:rusty-idd` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:teri` |
| `capability:fleet-handoff` | mapped_to_repo | `repo:weave` |
| `capability:github-agent-run-upgrades` | records_anchor | `anchor:beads-mandatory-for-code-contributors-through-yazelix` |
| `capability:github-agent-run-upgrades` | records_anchor | `anchor:github-com-delightful-ai-beads-rs-d98da231d068acbadcdcd2262971c561de86132b` |
| `capability:github-agent-run-upgrades` | records_anchor | `anchor:github-com-dicklesworthstone-beads-rust-2d824a8deaa203d64326849d86f8e6d4a9c24eca` |
| `capability:github-agent-run-upgrades` | records_anchor | `anchor:grit-from-rtk-ai` |
| `capability:github-agent-run-upgrades` | mapped_to_repo | `repo:grit` |
| `capability:github-agent-run-upgrades` | mapped_to_repo | `repo:yazelix` |
| `capability:idd-spec-engine` | records_anchor | `anchor:rusty-idd-built-into-handoff` |
| `capability:idd-spec-engine` | mapped_to_repo | `repo:handoff` |
| `capability:idd-spec-engine` | mapped_to_repo | `repo:rusty-idd` |
| `capability:lua-ar-interface` | records_anchor | `anchor:brilliant-labs-noa-style-rust-native-agent-ux` |
| `capability:lua-ar-interface` | records_anchor | `anchor:lua-required-for-ar-glasses-workflow` |
| `capability:lua-ar-interface` | mapped_to_repo | `repo:lifeos` |
| `capability:lua-ar-interface` | mapped_to_repo | `repo:oh-my-pi` |
| `capability:lua-ar-interface` | mapped_to_repo | `repo:yazelix` |
| `capability:meta-peer-control` | records_anchor | `anchor:meta-peer-repo-system` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:loop-cli` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:loop-lib` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-cli` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-core` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-dashboard-cli` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-git-cli` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-git-lib` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-mcp` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-plugin-api` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-plugin-protocol` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-project-cli` |
| `capability:meta-peer-control` | mapped_to_repo | `repo:meta-rust-cli` |
| `capability:network-engineering` | records_anchor | `anchor:lane-merges-into-network-manager` |
| `capability:network-engineering` | mapped_to_repo | `repo:lane` |
| `capability:network-engineering` | mapped_to_repo | `repo:network-control` |
| `capability:network-engineering` | mapped_to_repo | `repo:network-hub` |
| `capability:parser-runtime` | records_anchor | `anchor:ghostty` |
| `capability:parser-runtime` | records_anchor | `anchor:lua` |
| `capability:parser-runtime` | records_anchor | `anchor:nushell` |
| `capability:parser-runtime` | records_anchor | `anchor:tree-sitter-via-yazelix` |
| `capability:parser-runtime` | records_anchor | `anchor:yazelix-default-terminal` |
| `capability:parser-runtime` | records_anchor | `anchor:zellij` |
| `capability:parser-runtime` | mapped_to_repo | `repo:rusty-idd` |
| `capability:parser-runtime` | mapped_to_repo | `repo:tool-hub` |
| `capability:parser-runtime` | mapped_to_repo | `repo:yazelix` |
| `capability:personal-automation` | records_anchor | `anchor:personal-life-media-tv-home-automation` |
| `capability:personal-automation` | mapped_to_repo | `repo:lifeos` |
| `capability:personal-automation` | mapped_to_repo | `repo:oh-my-pi` |
| `capability:prompt-front-door` | records_anchor | `anchor:github-com-f-ai-prompt` |
| `capability:prompt-front-door` | records_anchor | `anchor:github-com-f-prompts-chat` |
| `capability:prompt-front-door` | records_anchor | `anchor:prompt-hub-front-door-to-handoff-and-rusty-idd` |
| `capability:prompt-front-door` | mapped_to_repo | `repo:prompt-hub` |
| `capability:rtk-ai-foundation` | records_anchor | `anchor:grit-from-rtk-ai` |
| `capability:rtk-ai-foundation` | records_anchor | `anchor:icm-from-rtk-ai` |
| `capability:rtk-ai-foundation` | records_anchor | `anchor:rtk-from-rtk-ai` |
| `capability:rtk-ai-foundation` | records_anchor | `anchor:vox-from-rtk-ai` |
| `capability:rtk-ai-foundation` | mapped_to_repo | `repo:grit` |
| `capability:rtk-ai-foundation` | mapped_to_repo | `repo:icm` |
| `capability:rtk-ai-foundation` | mapped_to_repo | `repo:rtk-tokenkill` |
| `capability:rtk-ai-foundation` | mapped_to_repo | `repo:vox` |
| `capability:user-front-door` | records_anchor | `anchor:goose-like-chat-integration` |
| `capability:user-front-door` | records_anchor | `anchor:lifeos-front-door` |
| `capability:user-front-door` | mapped_to_repo | `repo:lifeos` |
| `capability:user-front-door` | mapped_to_repo | `repo:prompt-hub` |
| `capability:user-front-door` | mapped_to_repo | `repo:ruvector` |
| `capability:vector-runtime` | records_anchor | `anchor:meta-ruvector-full-agentic-system` |
| `capability:vector-runtime` | mapped_to_repo | `repo:database-hub` |
| `capability:vector-runtime` | mapped_to_repo | `repo:icm` |
| `capability:vector-runtime` | mapped_to_repo | `repo:obsidian-mind` |
| `capability:vector-runtime` | mapped_to_repo | `repo:ruvector` |
| `layer:agent-runtime` | contains_capability | `capability:agent-harness` |
| `layer:agent-runtime` | contains_capability | `capability:github-agent-run-upgrades` |
| `layer:coordination-communication` | contains_capability | `capability:agent-communication` |
| `layer:coordination-communication` | contains_capability | `capability:domain-upgrade` |
| `layer:coordination-communication` | contains_capability | `capability:fleet-handoff` |
| `layer:environment-security` | contains_capability | `capability:env-vault-relay` |
| `layer:executive-control-plane` | contains_capability | `capability:idd-spec-engine` |
| `layer:executive-control-plane` | contains_capability | `capability:meta-peer-control` |
| `layer:front-door-experience` | contains_capability | `capability:prompt-front-door` |
| `layer:front-door-experience` | contains_capability | `capability:user-front-door` |
| `layer:governance-reasoning` | contains_capability | `capability:board-reasoning` |
| `layer:infrastructure-device-fabric` | contains_capability | `capability:distributed-device-fabric` |
| `layer:infrastructure-device-fabric` | contains_capability | `capability:network-engineering` |
| `layer:interface-automation` | contains_capability | `capability:lua-ar-interface` |
| `layer:interface-automation` | contains_capability | `capability:personal-automation` |
| `layer:knowledge-runtime` | contains_capability | `capability:vector-runtime` |
| `layer:simulation-validation` | contains_capability | `capability:digital-twin-simulation` |
| `layer:toolchain-parser-runtime` | contains_capability | `capability:parser-runtime` |
| `layer:toolchain-parser-runtime` | contains_capability | `capability:rtk-ai-foundation` |
| `system:agentic-company` | contains_layer | `layer:agent-runtime` |
| `system:agentic-company` | contains_layer | `layer:coordination-communication` |
| `system:agentic-company` | contains_layer | `layer:environment-security` |
| `system:agentic-company` | contains_layer | `layer:executive-control-plane` |
| `system:agentic-company` | contains_layer | `layer:front-door-experience` |
| `system:agentic-company` | contains_layer | `layer:governance-reasoning` |
| `system:agentic-company` | contains_layer | `layer:infrastructure-device-fabric` |
| `system:agentic-company` | contains_layer | `layer:interface-automation` |
| `system:agentic-company` | contains_layer | `layer:knowledge-runtime` |
| `system:agentic-company` | contains_layer | `layer:simulation-validation` |
| `system:agentic-company` | contains_layer | `layer:toolchain-parser-runtime` |
| `system:agentic-company` | planned_by | `repo:rusty-idd` |

## Findings

- Agent communication layer records external or future anchor: weave agent communication layer
- Agent harness runtime records external or future anchor: harness-agent-rs rust port
- Board reasoning layer records external or future anchor: company hierarchy board layer
- Central and fleet handoff records external or future anchor: handoff central and fleet design
- Digital twin simulation records external or future anchor: Teri digital twin simulator
- Distributed device fabric records external or future anchor: user devices for distributed compute storage inference memory
- Domain upgrade path records external or future anchor: weave plus Obscura domain upgrades
- Environment and vault relay records external or future anchor: /run/media/drdave/COGNITUM
- Environment and vault relay records external or future anchor: Cognitum vault on Pi Zero
- GitHub agent-run upgrades records external or future anchor: Beads mandatory for code contributors through Yazelix
- GitHub agent-run upgrades records external or future anchor: GRIT from rtk-ai
- GitHub agent-run upgrades records external or future anchor: github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca
- GitHub agent-run upgrades records external or future anchor: github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b
- IDD and spec engine records external or future anchor: Rusty IDD built into handoff
- Lua and AR interface automation records external or future anchor: Brilliant Labs Noa style Rust-native agent UX
- Lua and AR interface automation records external or future anchor: Lua required for AR glasses workflow
- Meta peer repo control records external or future anchor: meta peer repo system
- Network engineering and control records external or future anchor: lane merges into network-manager
- Parser and terminal runtime records external or future anchor: Ghostty
- Parser and terminal runtime records external or future anchor: Lua
- Parser and terminal runtime records external or future anchor: Yazelix default terminal
- Parser and terminal runtime records external or future anchor: Zellij
- Parser and terminal runtime records external or future anchor: nushell
- Parser and terminal runtime records external or future anchor: tree-sitter via Yazelix
- Personal media and home automation records external or future anchor: personal life media TV home automation
- Prompt front door records external or future anchor: github.com/f/ai-prompt
- Prompt front door records external or future anchor: github.com/f/prompts.chat
- Prompt front door records external or future anchor: prompt_hub front door to handoff and rusty-idd
- RTK AI foundation records external or future anchor: GRIT from rtk-ai
- RTK AI foundation records external or future anchor: ICM from rtk-ai
- RTK AI foundation records external or future anchor: RTK from rtk-ai
- RTK AI foundation records external or future anchor: VOX from rtk-ai
- User front door records external or future anchor: LifeOS front door
- User front door records external or future anchor: goose-like chat integration
- Vector and agentic runtime records external or future anchor: meta-ruvector full agentic system
- operating model derived from 65 repos and 13 roles in .idd/knowledge/system-architecture.json
