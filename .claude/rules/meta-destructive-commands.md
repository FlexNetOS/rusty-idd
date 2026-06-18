# Destructive Command Guard

This repository treats source, control-plane records, and generated audit files as reviewable state.

Agents must not run destructive cleanup commands unless the user explicitly requested the cleanup and the current worktree has been inspected for uncommitted or unmerged work.

Forbidden by default:

- `git reset --hard`
- `git clean -fd` or broader variants
- `git branch -D`
- `rm -rf` against repository paths
- force-pushes without an explicit lease and task-specific approval

When cleanup is requested, verify and record:

- Active branch.
- `git status --short --branch`.
- `git worktree list --porcelain`.
- Whether each stale worktree contains commits not reachable from `develop` or `main`.
