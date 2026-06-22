# Integration Automation Plan

- System root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees`
- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/prompt_hub`
- Source model: `.idd/knowledge/operating-model.json`
- Work items: 19

## Work Items

| Priority | Work Item | Capability | Status | Owners | Adopt First |
|---:|---|---|---|---|---|
| 10 | `Integrate IDD and spec engine` | `capability:idd-spec-engine` | partial | repo:handoff, repo:rusty-idd |  |
| 20 | `Integrate Central and fleet handoff` | `capability:fleet-handoff` | partial | repo:handoff, repo:prompt-hub, repo:rusty-idd, repo:weave |  |
| 30 | `Integrate Agent communication layer` | `capability:agent-communication` | partial | repo:atc, repo:handoff, repo:mcp-hub, repo:weave |  |
| 40 | `Integrate Environment and vault relay` | `capability:env-vault-relay` | partial | repo:envctl, repo:vault-hub, repo:yazelix | /run/media/drdave/COGNITUM, Cognitum vault on Pi Zero |
| 50 | `Integrate Prompt front door` | `capability:prompt-front-door` | partial | repo:prompt-hub | github.com/f/prompts.chat, github.com/f/ai-prompt |
| 60 | `Integrate RTK AI foundation` | `capability:rtk-ai-foundation` | partial | repo:grit, repo:icm, repo:rtk-tokenkill, repo:vox |  |
| 70 | `Integrate GitHub agent-run upgrades` | `capability:github-agent-run-upgrades` | partial | repo:grit, repo:yazelix | Beads mandatory for code contributors through Yazelix, github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca, github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b |
| 80 | `Integrate Parser and terminal runtime` | `capability:parser-runtime` | partial | repo:rusty-idd, repo:tool-hub, repo:yazelix |  |
| 90 | `Integrate Vector and agentic runtime` | `capability:vector-runtime` | partial | repo:database-hub, repo:icm, repo:obsidian-mind, repo:ruvector |  |
| 100 | `Integrate User front door` | `capability:user-front-door` | partial | repo:lifeos, repo:prompt-hub, repo:ruvector | goose-like chat integration |
| 110 | `Integrate Digital twin simulation` | `capability:digital-twin-simulation` | partial | repo:teri |  |
| 120 | `Integrate Network engineering and control` | `capability:network-engineering` | partial | repo:lane, repo:network-control, repo:network-hub |  |
| 130 | `Integrate Distributed device fabric` | `capability:distributed-device-fabric` | partial | repo:envctl, repo:network-control, repo:oh-my-pi |  |
| 140 | `Integrate Lua and AR interface automation` | `capability:lua-ar-interface` | partial | repo:lifeos, repo:oh-my-pi, repo:yazelix |  |
| 150 | `Integrate Personal media and home automation` | `capability:personal-automation` | partial | repo:lifeos, repo:oh-my-pi |  |
| 500 | `Integrate Agent harness runtime` | `capability:agent-harness` | partial | repo:agent, repo:agent-skills, repo:archon, repo:atc, repo:claude-code, repo:claude-plugin, repo:claude-plugins, repo:codex, repo:copilot-plugin, repo:ecc, repo:flexnetos-runner, repo:harness-hub, repo:hermes-agent, repo:icm, repo:kasetto, repo:n8n, repo:obscura, repo:oh-my-claudecode, repo:oh-my-pi, repo:prompt-hub, repo:rtk-tokenkill, repo:ruflo, repo:ruvector |  |
| 500 | `Integrate Board reasoning layer` | `capability:board-reasoning` | partial | repo:flexnetos-brain, repo:flexnetos-wiki, repo:icm, repo:my-wiki, repo:obsidian-mind |  |
| 500 | `Integrate Domain upgrade path` | `capability:domain-upgrade` | partial | repo:obscura, repo:weave |  |
| 500 | `Integrate Meta peer repo control` | `capability:meta-peer-control` | partial | repo:loop-cli, repo:loop-lib, repo:meta-cli, repo:meta-core, repo:meta-dashboard-cli, repo:meta-git-cli, repo:meta-git-lib, repo:meta-mcp, repo:meta-plugin-api, repo:meta-plugin-protocol, repo:meta-project-cli, repo:meta-rust-cli |  |

## Gates

- `cargo fmt --all -- --check`
- `cargo test --workspace --all-features --locked`
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `cargo audit --deny warnings`
- `cargo run --bin rusty-idd -- validate --workspace .`
- `cargo run --bin rusty-idd -- spec validate --all`
- `just ci`
- `make ci`
- `affected CLI smoke tests`

## Rollback Pattern

- Revert the integration slice commit or PR.
- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
- Re-run focused tests plus full Rusty IDD gates.

## Findings

- 8 adopt-first inputs preserved from operating-model anchors
- integration plan derived 19 work items from 19 operating capabilities
