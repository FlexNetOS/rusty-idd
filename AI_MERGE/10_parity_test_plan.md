# Parity Test Plan

## Goal

Prove that migrated behavior matches or intentionally improves old behavior before deleting old code.

## Required test classes

| Class | Purpose | Required before deletion? |
|---|---|---|
| Golden input/output tests | Compare old and new behavior on fixed fixtures | yes |
| Contract tests | Verify canonical interface compatibility | yes |
| Env resolution tests | Verify config/secret precedence | yes |
| CI workflow dry-read | Verify workflow references expected secrets/vars | yes |
| Rollback smoke test | Prove old path can be restored or feature flag disabled | yes |

## Deletion rule

No old implementation is deleted until parity tests pass and the migration note names the replacement path.
