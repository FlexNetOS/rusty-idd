# Agent Queue

This queue serializes work for GitHub cloud agents, local agents, and human contributors.

| Order | Task | Branch | Agent | Status | Blocking files | Notes |
|---:|---|---|---|---|---|---|
| 1 | Import repositories under `/imports` | `idd/imports` | TBD | queued | `/imports` | No flattening |
| 2 | Normalize env/secrets contract | `idd/env-secrets` | TBD | queued | config, CI | No secret values |
| 3 | Create canonical interfaces | `idd/interfaces` | TBD | queued | crates/apps | Preserve old behavior |
| 4 | Add parity tests | `idd/parity-tests` | TBD | queued | tests | Required before deletion |
| 5 | Migrate first vertical slice | `idd/vertical-slice-001` | TBD | queued | TBD | Small PR |
| 6 | Final dedupe cleanup | `idd/final-cleanup` | TBD | blocked | TBD | Only after parity passes |

Only one row may be marked `active` at a time when it touches overlapping files.
