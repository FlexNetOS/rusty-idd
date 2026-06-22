# Goal: Fix Codex Hook Base Ref Detection

## Intent

Repair the autonomous Codex Stop hook so it compares feature branch delivery
state against the authoritative develop base available in the current worktree.

## Scope

- Prefer `origin/develop` for workflow hook ancestry and delivery checks when it
  exists.
- Fall back to local `develop` for fixture and offline repositories.
- Add a regression test for stale local `develop` with current `origin/develop`.

## Out Of Scope

- Host service management.
- Global Rust or Cargo installation changes.
- Replacing the Rust-native hook checker with a non-Rust hook runtime.
