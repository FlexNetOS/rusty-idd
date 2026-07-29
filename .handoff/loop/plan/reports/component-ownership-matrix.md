# Component ownership matrix

| Component | Owns | Inputs | Outputs/APIs | Automation state | Gaps / next proof |
|---|---|---|---|---|---|
| LifeOS | Owner UI shell, workspaces, global OS surfaces, settings/vault/hardware inventory | envctl status, handoff/weave events, AI provider responses, Odysseus panel | Tauri/Vue desktop/web UI | Existing Vue/Tauri app with tests/build scripts | Durable AI chat, multimodal ingestion, tool routing, external control-plane adapters [L1][L2][L3] |
| Odysseus | Candidate AI workspace: chat/agents/research/docs/email/calendar/local models | local/API models, MCP, files, shell, ChromaDB, SearXNG, ntfy | Web app/API on local port | External fast-moving upstream | Must be sandboxed, pinned, auth-gated, license-reviewed [O1][O2][O3][O4][O5] |
| prompt_hub | Prompt source-of-truth, search, lineage, RBAC/audit, planning prompt | owner intent, repo context, feedback | CLI/server/library prompts and bundles | Existing Rust workspace and planning-engineer prompt | Bind prompts to rusty-idd changes and LifeOS intent UI [P1][P2] |
| rusty-idd | Intent/spec/goal lifecycle, OpenSpec engine, runner, manifest/validation | prompt_hub output, repo knowledge, tasks | `rusty-idd` CLI/OpenSpec state | Rust-native tool with lifecycle/runner surface | Archive active change, then use for plan-loop work definitions [R1] |
| weave | A2A session mesh, messages, asks, jobs, leases, permissions, memory, spawn/kill | agent sessions, jobs, lifecycle hooks | CLI + token-light MCP meta-tool + job board | Shipped Rust mesh | Prefer Damian/job-lane path for background scans; avoid token-heavy discovery [W1][W2] |
| handoff | Continuity ledger, claims, leases, task state, proofs, handoff packets | git state, task cards, ledger events, weave leases | ledger/events/packets/status | Full-auto doctrine accepted | Project LifeOS-visible status/watch feed and exact resume packets [H1][H2][H3] |
| envctl | Meta-local installs, components, add-repo, locks, secrets/env path authority | manifests, repo URLs, host probes | CLI/GUI, auto-detect/install/reset/lock/add-repo | Existing engine/CLI/GUI | Add Odysseus component with strict gates; expose JSON to LifeOS [E1][E2][E3] |
| meta-ruvector | Vector/memory/agent/runtime/gate substrate; 314 crate inventory | crates-only code truth | RVF, vector stores, rvAgent, MCP gates/brain, WASM/API candidates | Ledger/runbook complete; adoption strict crates-only | Select LifeOS memory/vector seam by trait/API, not crate-name guesses [M2][V1][V2] |
| network-control | Off-host fabric: Omada/switch/AP/gateway/VLAN/VPN | controller APIs, device config | `netctl` CLI/GUI JSON | Pure-Rust CLI and docs | LifeOS network workspace adapter and cross-layer weave coordination [N1] |
| lane | Local HTTPS domains, tunnels, host/network plane spine | local ports/config | local domain proxy/tunnel CLI | Rust port with local CA/proxy | Use as local service routing layer for LifeOS/Odysseus where appropriate [N2] |
| obscura | Rust headless browser/web automation engine | URLs/proxies/scripts | CLI/Docker/CDP/Puppeteer/Playwright style surface | External/forked browser engine | Govern only through weave/lane policy, not raw LifeOS exposure [N3][W2] |
| Meta CLI/canon repos | Workspace project graph, plugins, dashboard | `.meta.yaml`, repo manifests | meta commands, plugin protocol | Declared in `.meta.yaml` | Keep as substrate; do not duplicate in LifeOS [M1] |
| Hubs/repos | template/assets/flow/harness/network/tool/database/mcp/plugin/hooks/commands/vault | repo-local assets | curated collections | Mostly hub placeholders | Classify before automating; unverified claims stay out [M1] |


## Source keys
- [M1] `/home/drdave/Desktop/meta/.meta.yaml:5-212,276-430`
- [M2] `/home/drdave/Desktop/meta/.meta.yaml:292-306`
- [L1] `/home/drdave/Desktop/meta/lifeos/README.md:1-8,81-129`
- [L2] `/home/drdave/Desktop/meta/lifeos/AGENTS.md:41-73,157-174`
- [L3] `lifeos` lane read-only code graph note: `.git/gitkb/code.db` exists but `git-kb code doctor --json` reported zero indexed symbols; no indexing was run.
- [P1] `/home/drdave/Desktop/meta/prompt_hub/README.md:9-18,39-75,93-175`
- [P2] `/home/drdave/Desktop/meta/prompt_hub/prompts/README.md:95-119`
- [R1] `README.md:12-31,42-53` and `docs/rusty-idd/proposal.md:8-15,36-42`
- [W1] `/home/drdave/Desktop/meta/weave/README.md:1-18,25-42,93-176`
- [W2] `/home/drdave/Desktop/meta/weave/ARCHITECTURE.md:1-15,25-59,60-100,104-160`
- [H1] `/home/drdave/Desktop/meta/handoff/NORTH-STAR.md:15-58,61-84,87-123,159-169`
- [H2] `/home/drdave/Desktop/meta/handoff/docs/ARCHITECTURE.md:1-14,53-63,80-129,148-179`
- [H3] `/home/drdave/Desktop/meta/handoff/docs/adr-0018-full-auto-agentic-operation.md:8-19,35-71,73-152`
- [E1] `/home/drdave/Desktop/meta/envctl/README.md:1-14,15-28,46-68,108-123`
- [E2] `/home/drdave/Desktop/meta/envctl/docs/ARCHITECTURE.md:9-17,20-44,47-88,92-180`
- [E3] `/home/drdave/Desktop/meta/envctl/docs/ADD-REPO.md:1-33,35-85,109-115`
- [N1] `/home/drdave/Desktop/meta/network-control/README.md:1-31,73-98,108-144`
- [N2] `/home/drdave/Desktop/meta/lane/README.md:20-28,47-63,113-128,164-175`
- [N3] `/home/drdave/Desktop/meta/obscura/README.md:14-35,93-150`
- [V1] `/home/drdave/Desktop/meta/RUVECTOR-CRATE-LEDGER.md:1-5,37-63,120-130,144-155`
- [V2] `/home/drdave/Desktop/meta/RUVECTOR-RUNBOOK.md:7-23,33-39,40-115,116-126`
- [O1] `https://raw.githubusercontent.com/pewdiepie-archdaemon/odysseus/dev/README.md` lines 2, 9-13 as retrieved 2026-06-26
- [O2] `https://raw.githubusercontent.com/pewdiepie-archdaemon/odysseus/dev/requirements.txt` lines 0-3 as retrieved 2026-06-26
- [O3] `https://raw.githubusercontent.com/pewdiepie-archdaemon/odysseus/dev/docker-compose.yml` lines 0-9 as retrieved 2026-06-26
- [O4] `https://raw.githubusercontent.com/pewdiepie-archdaemon/odysseus/dev/SECURITY.md` lines 0-5 as retrieved 2026-06-26
- [O5] `https://raw.githubusercontent.com/pewdiepie-archdaemon/odysseus/dev/LICENSE` lines 0-5,24-28 as retrieved 2026-06-26
