# AGENTS.md

## First command

Run:

```bash
hf resume
```

## North Star (canon — two levels)

This repo carries TWO canonical layers; read both before planning:

**1. Kernel doctrine (local, authoritative for *how* this kernel governs change):**
[`NORTH-STAR.md`](NORTH-STAR.md) — a local-first, auditable, reversible, model-native
agentic OS where **every agent action increases verified capability without corrupting the
baseline**: Integrity · Reversibility · Capability Gain (no promotion without all three).
CECCA/NOA is the executive kernel; the **Gold World** is the protected baseline; failures
compress into evidence. The keystone ADR is [`docs/adr-0001-flexnetos-autopilot-keystone.md`](docs/adr-0001-flexnetos-autopilot-keystone.md).

**2. Fleet vision (the *why/where* — meta root):** **NO HUMAN IN THE LOOP — a multi-provider
agentic autopilot; the user gives direction, the system builds/verifies/delivers/operates;
`NEEDS-HUMAN` is a scaffold replaced by a model with the human's skillset; end-state = a
single-person conglomerate run by the system.**

- **Fleet vision:** `../NORTH-STAR.md` (mission, laws, planes, build-order, steward rubric)
- **Architecture:** `../ARCHITECTURE-TRUTH.md` (the 5 planes, verified vs code)
- **Runbook:** `../RUVECTOR-RUNBOOK.md` (the agentic pipeline + build-out)

The packet's North Star derives from `.handoff/context/capsule.json` (`northstar`), which
points here — not a hardcoded string (ADR-0006).

## Mission

Maintain this repository through the Continuity Ledger Kernel (`.handoff`) protocol. The repo is the source of truth. Chat history is not authoritative.

## Agent navigation surfaces

The canonical local CLI is `hf`; the tool/front-door surface is `hf-mcp`.

- `hf resume` is still the first read path and the ledger remains authoritative.
- `hf-mcp` is a strict MCP stdio bridge over the same CLI, not a second source of truth.
  Every tool argument schema rejects unknown properties and the server validates the allowlist
  before dispatching to `hf`, so an agent typo must fail closed instead of silently running a
  narrower action.
- Current MCP coverage includes status/resume/claim/checkpoint/done/test/ship, prompt_hub intake
  and dispatch, delivery get/list, policy/gatekeeper checks, drift/handoff, version, lease, schema,
  session start/end/reap, and fleet status/sync/render. If a CLI verb is added or its flags change,
  update `hf/src/bin/hf-mcp.rs`, docs, and tests in the same task.

## Knowledge base — the planning plane (`.kb`, ADR-0003 / ADR-0018 D7)

handoff fully adopts the FlexNetOS agent guide (`../.kb/AGENTS.md`) with its OWN durable
`.kb/`. Detect KB state and load context BEFORE planning non-trivial work:

```bash
git kb list --path context/      # PATH A (empty) / PATH B (populated) / PATH C (resume)
git kb checkout --path context/  # load the seven context documents
git kb board                     # task kanban
```

- **Create-first discipline:** for a non-trivial feature/bug, create the kb document
  FIRST (`git kb create task|incident …`) — the document IS your plan — then explore.
  Trivial typo/one-liner changes are exempt.
- **Traceability:** commit messages reference the task (`Implements [[tasks/<slug>]]` /
  the HFTASK id); tasks link incidents/specs; children link parents.
- **Two-way seam (ONE-WAY authority — kb never overrides execution truth):**
  `hf task mint --from-kb <slug>` mints a witnessed card IN; `hf claim`/`checkpoint`/
  `done`/`release` mirror the transition back OUT (active/+progress/completed/backlog).
  `hf status` / `hf resume` (the ledger) is authoritative; the kb informs the plan.
- **Residency:** `.kb/store/**` (durable text) is committed; `.kb/.cache/` (binary
  cache) + `.kb/workspaces/` (ephemeral) + `.kb/config.toml` are gitignored.

See `.claude/rules/knowledge-management.md` for the full lifecycle.

## Hard rules

- Do not edit files without a task claim.
- Do not write outside claimed path scope.
- Do not run a parallel write session against overlapping paths.
- Do not mark a task complete without tests or an explicit waiver.
- Do not stop without `hf checkpoint` and `hf handoff`.
- Do not make architecture changes without an ADR.
- Do not treat `.handoff/packets/latest.md` as more authoritative than Git, the ledger, or task cards.

### Fail-closed law (the FAIL-OPEN ban — L7)

A guard, loader, or evidence-check that **cannot confirm its precondition must STOP,
not proceed.** In any continuity-gating path (card load, ledger read, status
derivation, completion-evidence, lock acquisition, policy gate) the following
fail-OPEN patterns are **banned** — each must instead fail closed with a *surfaced
diagnostic*:

- a silent `if let Ok(_) { … }` / `match … { Ok => …, Err => /* skip */ }` that drops
  the error case without surfacing it (e.g. silently skipping an unparseable card);
- `.ok()?` on a card or ledger read (swallows the failure and short-circuits as if
  empty);
- `unwrap_or_default()` feeding a status/derivation (an empty default reported as
  truth — `current_statuses()` returning `{}` must be a FAIL, not "nothing done");
- **exit 0 ⇒ pass** — treating a zero exit / empty result / zero rows / `None`
  runner as evidence the criterion was met (require the *positive* count or artifact);
- **retry-then-quietly-give-up** — exhausting a retry cap and proceeding as if the
  operation succeeded (the stale-lock wedge), instead of surfacing the wall.

Every such site must emit a diagnostic to stderr (or a `hf doctor`/`hf status`
surfacing) and propagate a non-zero/Err outcome. *Absence of failure is not evidence
of success.* When in doubt, fail closed and surface — the kernel's founding promise
is witnessed + fail-closed.

## Required before stopping

```bash
hf checkpoint <ID> [note]
hf handoff
```

(`hf drift` and `hf policy check-{claim,edit,handoff}` are implemented — the
PreHandoff/TaskClaim/PreEdit hard gates. Run `hf drift` before any handoff.)

## Navigation order

0. Kernel doctrine + keystone: `NORTH-STAR.md` · `docs/adr-0001-flexnetos-autopilot-keystone.md`
1. `.handoff/active.md`
2. `.handoff/context/capsule.json`
3. `.handoff/packets/latest.md`
4. `.handoff/tasks/` (task cards) · `.handoff/decisions/` (ADRs)
5. `docs/Continuity_Ledger_Kernel_PRD.md`
6. Planning plane: `git kb checkout --path context/` (the `.kb` context documents) · `git kb board`
7. Fleet canon (the why): `../NORTH-STAR.md` · `../ARCHITECTURE-TRUTH.md` · `../RUVECTOR-RUNBOOK.md`
# AGENTS.md — rusty-idd (Intent Driven Development)

## North Star

Unify repositories by preserving working behavior, making contracts explicit, and merging only through reviewable, test-backed increments.

## Operating Rules

1. Do not flatten two repos before generating inventories, manifest, feature matrix, and contract maps.
2. Do not introduce a new secret provider unless the env/secrets contract says why.
3. Do not delete source code during migration; deprecate first, remove after parity tests pass.
4. Keep PRs narrow: one vertical slice, one migration intent, one validation path.
5. Every generated PR must update the relevant OpenSpec, `.idd`, ADR, and evidence records; use `/AI_MERGE` only when audit, migration, rollback, or merge evidence is required.
6. Never commit real secrets. Use `.env.example`, `.env.schema.example.json`, GitHub Actions secrets, OIDC, or provider references.
7. If two agents conflict, stop and update `/AI_MERGE/05_conflict_risk_register.md` before continuing.
8. Treat `.idd/MANIFEST.tsv` as the audit baseline for generated control-plane artifacts.
9. Break work into sub-59-minute tasks when using cloud coding agents with hard session limits.
10. One integration branch has authority. Other branches are research, staging, or disposable.

## Rusty IDD Workflow Rules

1. Rusty IDD is the intent-driven workflow engine: user intent, graph/context artifacts, OpenSpec proposal/spec/design/ADR/tasks, implementation gating, validation, archive, and handoff evidence.
2. `AI_MERGE/` is a Rusty IDD tool and evidence surface. Use it for audit notes, migration history, rollback, and merge evidence when the workflow calls for it; do not treat it as the main intent source or authoritative control plane.
3. Before implementation, create or refresh the relevant `.idd/knowledge/*` graph artifacts and bind the goal with `rusty-idd knowledge plan-context`.
4. Before writes, create or select an OpenSpec change and verify readiness with `rusty-idd spec status` or `rusty-idd spec next`.
5. ADR decisions live in repo-level `adr/`; accepted ADRs are immutable. Supersede with a new ADR instead of editing prior accepted decisions.
6. Implementation follows tasks only after the OpenSpec artifacts are ready.
7. Validation must refresh deterministic artifacts: `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, OpenSpec status, and Rusty IDD validation.

## Codex Environment Rules

1. Use `.idd/knowledge/report.md`, `.idd/knowledge/architecture.md`, `.idd/knowledge/plan-context.md`, and OpenSpec status before broad manual rescans or edits.
2. Use repo-local Codex surfaces intentionally:
   - `AGENTS.md` for durable repo rules.
   - `.agents/skills` for reusable workflows.
   - `.codex/agents` for project-scoped subagents.
   - `.codex/rules` for project-scoped exec policy.
   - `.codex/hooks.json` and `.codex/hooks` for lifecycle checks.
3. The default Codex harness is read-only and design-first. A write-capable implementation pass requires explicit authorization and ready OpenSpec artifacts.
4. Adopt first, cut after evidence. For crate, tool, agent, skill, hook, or repo integrations, first bring the upstream/current surface forward intactly enough to diagnose it, then cut only evidenced compile, audit, security, or scope friction.
5. Do not replace an upstream capability with local guesses before proving the upstream surface cannot be used. If a cut is required, record the reason in an ADR or evidence note and keep the adapter thin.
6. Upgrade only. Never downgrade a working surface, dependency, action, model, agent, skill, or generated artifact to simplify a task.
7. Treat stale or orphaned work as unfinished by default unless evidence proves it is intentionally local and ignored.
8. Tooling required to run this repo must be tracked and provisioned through the parent `meta` / `envctl` path first. Do not install missing binaries into user-global state to make a gate pass.
9. Parallel subagents are for read-heavy exploration, verification, and gap hunting unless a single integration branch/worktree owner coordinates writes.
10. Host service and process management is out of scope for this repo. Do not use raw `systemctl`, daemon kills, or binary installation as a way to make repository work pass.

## Required PR Evidence

- Build command result
- Test command result
- Lint/typecheck result
- Secret scan result
- Migration note explaining old path -> new path
- Rollback path
- Updated manifest or note explaining why unchanged

## Merge Authority

Parallel agents may analyze and propose branches, but only the integration branch is authoritative.
