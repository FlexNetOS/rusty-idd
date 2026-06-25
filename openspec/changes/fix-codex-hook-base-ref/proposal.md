# fix-codex-hook-base-ref

## Why

The Codex Stop hook can falsely require PR/automerge evidence on a clean feature
worktree created from `origin/develop` when the local `develop` branch is stale.
That blocks normal agent turns and makes hook failures look like missing
handoff evidence instead of a base-ref selection bug.

## What Changes

- Prefer `origin/develop` as the workflow hook base when the ref exists.
- Keep local `develop` as the fallback for offline and fixture repositories.
- Add regression coverage for the stale-local-`develop` case.

## Impact

- `rusty-idd codex workflow-check --phase stop` no longer demands PR evidence
  for branches that are clean relative to `origin/develop`.
- Existing dirty-work and commits-beyond-base evidence enforcement remains.
