# AGENTS.md — Intent Driven Development

## North Star

Unify repositories by preserving working behavior, making contracts explicit, and merging only through reviewable, test-backed increments.

## Operating Rules

1. Do not flatten two repos before generating inventories, manifest, feature matrix, and contract maps.
2. Do not introduce a new secret provider unless the env/secrets contract says why.
3. Do not delete source code during migration; deprecate first, remove after parity tests pass.
4. Keep PRs narrow: one vertical slice, one migration intent, one validation path.
5. Every generated PR must update the relevant `/AI_MERGE` record.
6. Never commit real secrets. Use `.env.example`, `.env.schema.example.json`, GitHub Actions secrets, OIDC, or provider references.
7. If two agents conflict, stop and update `/AI_MERGE/05_conflict_risk_register.md` before continuing.
8. Treat `.idd/MANIFEST.tsv` as the audit baseline for generated control-plane artifacts.
9. Break work into sub-59-minute tasks when using cloud coding agents with hard session limits.
10. One integration branch has authority. Other branches are research, staging, or disposable.

## Codex Environment Rules

1. Adopt first, cut after evidence. For crate, tool, agent, skill, hook, or repo integrations, first vendor or depend on the upstream/current surface as intactly as possible, build it, audit it, then cut only the concrete compile, audit, security, or scope friction that evidence exposes.
2. Do not replace an upstream capability with local guesses before proving the upstream surface cannot be used. If a cut is required, record the reason in an ADR or `/AI_MERGE` note and keep the adapter thin.
3. Use `.idd/knowledge/report.md` and `.idd/knowledge/index.json` before broad manual rescans. Refresh knowledge after source or control-plane changes that affect graph/report output.
4. Use repo-local Codex surfaces intentionally:
   - `AGENTS.md` for durable repo rules.
   - `.agents/skills` for reusable workflows.
   - `.codex/agents` for project-scoped subagents.
   - `.codex/rules` for project-scoped exec policy.
   - `.codex/hooks.json` and `.codex/hooks` for lifecycle checks.
5. The agent owns output quality and tool-surface growth. When evidence from a miss, slow path, stale artifact, weak verification, or repeated manual step shows that a Codex skill, rule, hook, agent, plugin, MCP server, or local Rust helper would materially improve accuracy or speed, decide and add the narrowest tracked tool surface instead of waiting for the user to micromanage it. Record the reason, keep it bounded, and verify it.
6. Depend less on human-in-the-loop approval for tool choice. Ask only when the change would cross repo scope, require credentials, alter host/user-global state, create a destructive path, or conflict with an explicit owner boundary.
7. Upgrade only. Never downgrade a working surface, dependency, action, model, agent, skill, or generated artifact to simplify a task; move forward to the latest stable or more capable tracked path unless concrete build, audit, compatibility, or owner-boundary evidence requires a scoped hold.
8. Treat stale or orphaned work as unfinished work by default. Before ignoring a stale artifact, orphaned file, skipped TODO, ignored generated output, or disconnected tool surface, prove it is intentionally local/ignored or finish it, document it, regenerate affected artifacts, and verify it.
9. Tooling required to run this repo must be tracked and provisioned through the parent `meta` / `envctl` path first. Do not install missing binaries into user-global state to make a gate pass; add or fix the parent-managed tool surface, or use an already tracked repo-local equivalent while recording the gap.
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
