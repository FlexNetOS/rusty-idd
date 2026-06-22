# Rusty IDD And Handoff Single-Repo Goal

```bash
rusty-idd --goal-file [rusty-idd and handoff must be combined into a sinlge repo]
```

## Intent

Plan the architecture for combining `rusty-idd` and `handoff` into a single
repository without guessing the ownership boundary first.

The planning pass must decide whether Rusty IDD should be embedded inside
`handoff`, whether `handoff` should be embedded inside `rusty-idd`, or whether a
single repository should preserve both as peer crates under one workspace.

## Required Method

- Start from a fresh Rusty IDD worktree based on `develop`.
- Track the work through a handoff task card.
- Generate Rusty IDD knowledge, system, operating-model, integration, readiness,
  plan-context, OpenSpec, ADR, and evidence artifacts before implementation.
- Deep scan both current repositories and their integration surfaces.
- Prefer the repository shape that preserves working behavior and reduces
  long-term context rot, build drift, and duplicated control-plane state.
- Do not move code during this planning pass unless the OpenSpec decision and
  evidence explicitly prove a narrow migration slice is ready.

## Validation Target

The final planning artifact must identify the recommended target repository
shape, the migration phases, validation gates, rollback path, and the smallest
first implementation slice.
