# queue-aware-plan-integration - Design

## Context

The integration status queue derives state from existing OpenSpec directories:
planned items have no active or archived change, scaffolded items have an active
change, and archived items live under `openspec/changes/archive`.

`spec plan-integration` already reads the integration plan and writes OpenSpec
artifacts. It should use the same filesystem state for default selection,
without introducing a mutable queue file or changing explicit selectors.

## Goals / Non-Goals

**Goals:**

- Default no-selector scaffolding selects the lowest-priority planned item.
- Existing active and archived changes are skipped by default.
- Explicit selectors remain exact and continue to use overwrite protection.
- The command remains read/write scoped to Rusty IDD OpenSpec artifacts only.

**Non-Goals:**

- Automatically archiving completed changes.
- Running implementation tasks.
- Mutating peer repos or starting services.
- Adding a new queue database.

## Decisions

- Treat `openspec/changes/<change_id>` and
  `openspec/changes/archive/<change_id>` as occupied queue slots.
- Apply occupied-slot filtering only when no selector is provided.
- Preserve existing `--force` behavior for explicit and default writes; `--force`
  can overwrite the selected target after selection.
- Return a clear error when all integration work items are already scaffolded or
  archived.

## Risks / Trade-offs

- A partially hand-created directory occupies the queue slot. That is intentional
  because it prevents duplicate automation and lets `integration-status` report
  the incomplete scaffold.

## Migration Plan

1. Add focused CLI tests around default queue advancement.
2. Update the selection logic.
3. Refresh generated artifacts.
4. Run focused and full gates.

## Open Questions

- Whether a future command should combine status, scaffold, and runner execution
  into one dry-run-first automation loop.
