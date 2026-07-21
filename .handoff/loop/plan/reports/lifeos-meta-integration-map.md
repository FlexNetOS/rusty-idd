# LifeOS / Meta integration map

## End-state architecture
LifeOS should be the owner-facing shell: Vue 3 + Vite + Pinia + vue-router in a Tauri 2 desktop shell with a web build, six addressable workspaces, global OS surfaces, settings/profile vault, and hardware inventory. [L1][L2]

The correct end-state is not to collapse all tools into LifeOS. LifeOS should render status, launch panels, accept owner intent, and expose safe controls, while each subsystem keeps its authoritative engine: prompt_hub for prompts, rusty-idd for goals/specs/tasks, envctl for installs/env state, weave for A2A/jobs/lanes, handoff for continuity, meta-ruvector for vector/memory/agent substrate, and network-control/lane/obscura for network planes. [P1][R1][E1][W2][H2][V2][N1][N2][N3]

## Current LifeOS gap
LifeOS already has AI avatar/chat and provider routing, but the lane found chat remains lightweight: UI affordances for multimodal inputs are not durable ingestion, provider routing only covers Claude/OpenAI/Gemini, and `source` is reserved rather than used as a real tool/agent router. [L2][L3]

## Integration pattern
1. LifeOS owns navigation, auth UX, workspace aggregation, owner settings, and component status. [L1][L2]
2. Odysseus is mounted as a sandboxed `/ai` workspace panel first, not source-merged. [O1][O4][O5]
3. envctl owns install, pinning, local bind, data directory, health checks, rollback, and secrets wiring. [E1][E3]
4. weave/handoff capture events, jobs, continuity, review/status, and resume evidence. [W2][H2][H3]
5. prompt_hub/rusty-idd remain upstream of all implementation loops. [P2][R1]
6. meta-ruvector is exposed through chosen trait/API/MCP/WASM seams after inventory gating; it is not bulk-wired by name. [M2][V2]

## Missing buildable seams
- LifeOS component registry/status page backed by envctl `auto-detect/install/verify` JSON. [E1][L2]
- LifeOS `/ai` route that can embed a local-only Odysseus service while preserving LifeOS shell/state. [L1][O3]
- prompt_hub -> rusty-idd -> plan-loop -> feature-forge handoff envelope with source citations and test traceability. [P2][R1]
- weave Damian/job-lane dispatch for background scans instead of token-heavy broad MCP discovery. [W1][W2]
- handoff ledger/event projection into LifeOS notifications/To-Do/Knowledge. [H2][L2]
- meta-ruvector memory/vector API selection for LifeOS knowledge, not a 314-crate firehose. [V1][V2]


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
