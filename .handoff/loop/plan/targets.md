# Plan-loop targets — LifeOS / Meta front-door integration

Run date: 2026-06-26. Scope: first three planning cycles from the requested budget=3 run.

## North star
LifeOS is the unified owner-facing app/front door for work, personal, home automation, AI command, and media surfaces; the current missing seam is a durable AI chat/front-door runtime beyond the lightweight existing avatar/chat panel. [L1][L2]

## Cycles executed
1. `lifeos-meta-front-door-integration` — fleet-level architecture map for LifeOS + Odysseus candidate + prompt_hub + meta-ruvector + rusty-idd + weave + handoff + envctl + network-control/lane/obscura + remaining meta peers. [M1][L1][O1]
2. `odysseus-adoption-plan` — Odysseus current-state evaluation, license/runtime/security/dependency risk, and strict-upgrade adoption wrapper. [O1][O2][O3][O4][O5]
3. `automation-control-plane` — automation loop connecting prompt_hub -> rusty-idd -> planning-engineer -> feature-forge -> envctl install/verify -> handoff/weave continuity -> LifeOS UI. [P2][R1][E1][H3][W2]

## Major components seeded
- LifeOS: Vue/Tauri desktop/web shell, workspaces, global OS icons, settings/profile/vault/hardware inventory, lightweight AI chat surface. [L1][L2]
- Odysseus: external self-hosted AI workspace candidate for chat/agents/research/docs/email/notes/calendar/local models; evaluated as sandbox candidate, not direct canonical replacement. [O1]
- prompt_hub: prompt source-of-truth, semantic search, versioning, RBAC/audit, swarm/handoff templates, planning-engineer prompt. [P1][P2]
- meta-ruvector: crates-only adoption surface with 314-crate ledger, RVF/vector/memory/agent/runtime/gate substrate; never wire non-crate UI/runtime material into meta. [M2][V1][V2]
- rusty-idd: Rust-native intent/spec/goal lifecycle terminus of prompt_hub -> weave+rtk -> rusty-idd. [R1]
- weave: Rust-native A2A orchestration mesh, job board, leases, permissions, memory, spawn/kill, token-light MCP meta-tool and CLI parity. [W1][W2]
- handoff: continuity ledger kernel, state precedence, intent locks, ledger events, policy gates, full-auto workflow doctrine. [H1][H2][H3]
- envctl: meta-local environment manager, manifests, add-repo, reproducible install/verify/reset, no user-global installs. [E1][E2][E3]
- network-control/lane/obscura: off-host fabric, local HTTPS/tunnel/network plane, governed web/browser automation. [N1][N2][N3]


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
