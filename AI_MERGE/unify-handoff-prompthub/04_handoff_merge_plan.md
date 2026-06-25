# Merge Plan: unify-handoff

## Strategy

Use an intent-driven integration branch. Keep both imported repos intact until canonical contracts are proven. Migrate one vertical slice per PR.

## Recommended Tree

```text
imports/repo/          # untouched Repo A import
imports/handoff/          # untouched Repo B import
crates/                    # canonical Rust crates
apps/                      # canonical runnable apps
docs/                      # user and operator docs
AI_MERGE/                  # agent-readable audit and evidence surface
.idd/                      # lock, manifest, local state
```

## Execution Phases

1. Freeze integration branch and write `.idd/LOCK.md`.
2. Import both repositories under `/imports`.
3. Normalize environment and secret contracts.
4. Create canonical interfaces before moving implementations.
5. Migrate the smallest working vertical slice.
6. Add parity tests comparing old behavior to new behavior.
7. Deprecate old paths only after tests pass.
8. Remove duplicate code in final cleanup PRs.

## Initial Risk Read

- Repo A files: `3562`
- Repo B files: `592`
- Shared path collisions: `293`
- Repo A secret/env references: `718`
- Repo B secret/env references: `62`

## Merge Gate

A PR is mergeable only when build, tests, lint/typecheck, secret scan, `idd validate`, and migration notes are complete.

## GitHub Agent Constraint

Cloud agents should be fed one repo task at a time. If the task needs two repos, import or mirror the second repo into this integration repo first, then assign a single narrow PR.
