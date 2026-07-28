# Odysseus front-door evaluation

Research date: 2026-06-26. Candidate: `github.com/pewdiepie-archdaemon/odysseus`.

## Verdict
**QUALIFY / sandbox-adopt. Do not make it the canonical LifeOS front door yet.** It is promising as a local-first AI workspace panel, but the strict-upgrade path is a reversible envctl-managed service behind LifeOS, not a source merge or direct replacement. [O1][O3][O4][O5]

## What Odysseus provides today
Odysseus describes itself as a self-hosted workspace for chat, agents, research, documents, email, notes, calendar, and local model workflows. Its feature list includes local/API models, tools, MCP, files, shell, skills, memory, deep research, compare, documents, email, tasks/calendar, uploads, web search, sessions, and 2FA. [O1]

## Runtime and dependencies
The Python requirements include FastAPI/Uvicorn, Pydantic, SQLAlchemy, ChromaDB client, fastembed, MCP, crypto/bcrypt, calendar/email support, testing, and research/report rendering dependencies. [O2]

Docker Compose builds the app, persists `data` and `logs`, exposes Odysseus on APP_BIND/APP_PORT defaults of 127.0.0.1:7000, and composes ChromaDB, SearXNG, and ntfy with local binds. It also mounts the Docker socket for Cookbook/model-serving workflows, which is a privileged seam requiring explicit gating. [O3]

## License and legal risk
The README declares AGPL-3.0-or-later; the LICENSE is the GNU AGPL v3. AGPL network-copyleft means LifeOS should avoid code merging until a license decision exists. A separate process, reverse proxy, iframe/WebView, or API bridge is the safer first prototype boundary. [O1][O5]

## Security risk
Odysseus warns not to run as public unauthenticated service, requires auth for network-accessible deployment, asks operators to keep raw ChromaDB/SearXNG/ntfy/Ollama/vLLM/model APIs internal-only, and lists shell/Python/file/email/MCP/API/task/skill/memory/settings/token/model serving as admin-grade privileged functions. [O4]

## Strict-upgrade adoption plan
1. envctl component: clone or download a pinned upstream ref, never floating `latest`. [E3][O1]
2. Run local-only: bind Odysseus and bundled raw services to `127.0.0.1`; disable localhost bypass outside dev. [O3][O4]
3. Data boundary: envctl-managed data/log/cache directories under meta-local state or documented app-data path, with backup/restore tests. [E1][O3]
4. LifeOS adapter: route `/ai` to the local service while LifeOS owns shell navigation, status, and settings. [L1][O1]
5. Auth/secrets: use envctl secret stack/keyring/env contract, never commit `.env`, data, logs, auth files, API keys, tokens, or uploaded personal docs. [E1][O4]
6. Observability: publish health/status/events into weave/handoff so planning and resume loops can see service state. [W2][H2]
7. Promotion gates: license review, reproducible dependency lock, secret scan, tool privilege audit, backup/restore, rollback, and LifeOS UX fit. [O2][O4][O5]


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
