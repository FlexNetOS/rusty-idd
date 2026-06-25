# 0011. Verify package stage

- Status: accepted
- Date: 2026-06-22

## Context

Rusty IDD now owns task-scoped harness packages, with `.codex`, `.claude`,
`.kimi`, and `.agents` treated as minimal runtime adapters. The first shipped
package stage is `scan`.

The next high-value stage is post-task verification. Users need `/verify` after
tasks complete, but verification must not become another large always-loaded
prompt. It needs exhaustive research, testing, cross-verification, diff review,
question extraction, graph checks, ICM recall/compare, and comparison against
the original request, goal, task artifacts, and plans.

## Decision

Rusty IDD will own a `verify` harness package stage. Model-specific `/verify`
surfaces are thin adapters that invoke Rusty IDD package generation and follow
the package contract.

The verify package declares its own roles, contracts, tools, helpers, hooks,
validation gates, and evidence schema. The package is the source of truth for
post-task verification behavior.

## Consequences

- Verification behavior becomes portable across Codex and other model adapters.
- `.codex` can expose `/verify` without carrying the exhaustive checklist in
  always-loaded context.
- Verification evidence becomes typed and eligible for Rusty IDD validation,
  manifest refresh, and PR handoff.
- Future work can add a native `rusty-idd verify run` executor after the package
  contract is stable.
