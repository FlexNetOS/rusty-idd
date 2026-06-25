# fix-codex-hook-base-ref - Design

## Context

Autonomous Codex workflow checks use Git state to decide whether Stop requires
delivery evidence. The original implementation compared `develop..HEAD`, which
assumes local `develop` is current. In Rusty IDD worktrees, agents commonly
branch from `origin/develop` after fetch without checking out and fast-forwarding
the local `develop` branch.

## Decision

Introduce a small base-ref resolver for the Codex workflow checker:

- use `origin/develop` when `refs/remotes/origin/develop` resolves to a commit;
- otherwise use local `develop`.

Use that resolved base ref for both ancestry validation and Stop delivery
detection.

## Alternatives Considered

- Always require local `develop` to be fast-forwarded before hook use. This
  would make hook behavior depend on unrelated local branch maintenance.
- Always use `origin/develop`. This would break local fixture repositories and
  offline workflows that intentionally do not have remotes.

## Risk

If a repository has a stale `origin/develop`, the hook will trust that stale
remote-tracking ref. The normal workflow already fetches before creating a
feature branch, and this is still more accurate than trusting local `develop`
in multi-worktree agent sessions.
