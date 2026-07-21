# Automation roadmap — next Feature Forge chunks

## P0: Odysseus sandbox component via envctl
Build an envctl component/drop-in for pinned Odysseus install, local-only bind, health check, auth/secrets checks, raw-port verification, backup/restore, and rollback. [E1][E3][O3][O4]

## P0: LifeOS `/ai` adapter panel
Add a LifeOS route/panel that can launch/status-check/embed a local Odysseus service while preserving LifeOS navigation and UI contracts. [L1][L2][O1]

## P0: prompt_hub -> rusty-idd plan-loop envelope
Make planning-engineer outputs directly create/select rusty-idd OpenSpec changes with target DAG, citations, tests, and handoff resume packet. [P2][R1][H2]

## P1: weave Damian/job-lane background scan runner
Use weave's job/dispatch/session surfaces as the default 5-lane planning scan transport, avoiding broad high-token MCP gateway discovery. [W1][W2]

## P1: handoff status projection into LifeOS
Expose handoff ledger state, next safe task, tests, rollback, and open risks in LifeOS Notifications/To-Do/Knowledge. [H1][H2][L2]

## P1: meta-ruvector memory/vector seam selection
Select a minimal LifeOS memory stack candidate from RVF + vector/index traits + mcp-brain/gate/rvAgent seams; avoid bulk wiring all crates. [M2][V1][V2]

## P1: network workspace adapter
LifeOS network workspace should call network-control for off-host fabric, lane for local routing/tunnels, and weave-governed obscura for web reach. [N1][N2][N3][W2]

## P2: autoresearch freshness loop
Track external dependency freshness for Odysseus, ChromaDB, SearXNG, ntfy, local model providers, and license/security changes; feed changes back to prompt_hub/rusty-idd. [O1][O2][O3][O4]


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
