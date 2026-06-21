---
name: rusty-idd-adopt-first
description: Use for Rusty IDD integrations, vendoring, upstream crate adoption, agent/tool imports, or refactors where the safe path is to adopt the current surface first, build it, then cut only evidenced friction.
---

# Rusty IDD Adopt-First Integration

Use this skill when adding or correcting an integration in this repository.

## Rule

Adopt first, cut after evidence. Do not replace upstream behavior with local guesses before proving the upstream surface cannot be used.

## Workflow

1. Inventory the current repo surface.
   - Read `AGENTS.md`, `.idd/knowledge/plan-context.*`, the active OpenSpec
     change, in-force ADRs, and any relevant `AI_MERGE` evidence note.
   - Query `.idd/knowledge/index.json` before broad manual source reads when it exists.
   - Identify the existing ownership boundary before editing.

2. Bring the upstream or current implementation forward as intactly as practical.
   - Prefer pinned Cargo dependencies or vendored upstream crates with license material.
   - Keep local glue thin: DTO mapping, feature flags, deterministic output, and repo-specific policy.
   - Do not create fake compatibility crates unless build evidence proves there is no better path.

3. Build before cutting.
   - Run the narrowest compile/test command that exercises the adopted surface.
   - Record concrete failures before removing dependencies, modules, tests, or features.

4. Cut only evidenced friction.
   - Acceptable cuts: compile conflicts, audit-denied dependencies, incompatible runtime versions, out-of-scope daemon/host surfaces, or tests outside the chosen boundary.
   - Upgrade only. Do not downgrade a working upstream or repo surface to make the task easier; choose the latest stable or more capable tracked path unless concrete evidence requires a scoped hold.
   - Treat stale or orphaned work as unfinished by default. Either prove it is intentionally local/ignored or finish it before claiming completion.
   - Record durable boundary decisions in ADRs.
   - Record audit, migration, rollback, or merge evidence in `AI_MERGE` only
     when the Rusty IDD workflow calls for that evidence surface.

5. Verify and regenerate.
   - Run focused tests, then the relevant workspace gates.
   - Refresh `.idd/knowledge` and `.idd/MANIFEST.tsv` after source/control-plane changes.

## Checks

Useful commands:

```bash
just knowledge
just manifest
just validate
just codex-env-check
```

For the full gate:

```bash
just ci
```
