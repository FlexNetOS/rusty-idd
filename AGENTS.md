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
6. Merge, migration, and repository-unification goals use `rusty-idd merge-tools show` for the reusable Rusty IDD merge package; retired `idd-merge-idd` bridge material is not the active workflow.
7. Harness capabilities are task-scoped workflow packages. After a goal enters a workflow stage, use `rusty-idd harness package --stage <stage> --target <path>` to create or select the bounded package for that stage instead of growing always-loaded agent directories.
8. Implementation follows tasks only after the OpenSpec artifacts are ready.
9. Validation must refresh deterministic artifacts: `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, OpenSpec status, and Rusty IDD validation.

## Codex Environment Rules

1. Use `.idd/knowledge/report.md`, `.idd/knowledge/architecture.md`, `.idd/knowledge/plan-context.md`, and OpenSpec status before broad manual rescans or edits.
2. Use repo-local Codex surfaces intentionally:
   - `AGENTS.md` for durable repo rules.
   - `.agents/skills` for reusable workflow instructions, not stage package authority.
   - `.codex/agents` for project-scoped subagent adapters.
   - `.codex/rules` for project-scoped exec policy.
   - `.codex/hooks.json` and `.codex/hooks` for lifecycle checks.
   - `rusty-idd harness package` for task-scoped tools, contracts, helpers, hooks, roles, validation gates, and evidence schemas.
3. Keep `.codex`, `.claude`, `.kimi`, and similar harness directories minimal. They are runtime adapters or compatibility views; Rusty IDD workflow packages are the source of truth for stage-specific capability.
4. The default Codex harness is read-only and design-first. A write-capable implementation pass requires explicit authorization and ready OpenSpec artifacts.
5. Adopt first, cut after evidence. For crate, tool, agent, skill, hook, or repo integrations, first bring the upstream/current surface forward intactly enough to diagnose it, then cut only evidenced compile, audit, security, or scope friction.
6. Do not replace an upstream capability with local guesses before proving the upstream surface cannot be used. If a cut is required, record the reason in an ADR or evidence note and keep the adapter thin.
7. Upgrade only. Never downgrade a working surface, dependency, action, model, agent, skill, or generated artifact to simplify a task.
8. Treat stale or orphaned work as unfinished by default unless evidence proves it is intentionally local and ignored.
9. Tooling required to run this repo must be tracked and provisioned through the parent `meta` / `envctl` path first. Do not install missing binaries into user-global state to make a gate pass.
10. Parallel subagents are for read-heavy exploration, verification, and gap hunting unless a single integration branch/worktree owner coordinates writes.
11. Host service and process management is out of scope for this repo. Do not use raw `systemctl`, daemon kills, or binary installation as a way to make repository work pass.

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
