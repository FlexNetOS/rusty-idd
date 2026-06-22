# ADR 0004: Adopt Grit As-Is as an Upstream Reference

Status: accepted
Date: 2026-06-22

## Context

Rusty IDD needs a durable local reference for Grit so future agent-run and
integration planning can be grounded in a complete repository snapshot. The
owner explicitly constrained this slice to adoption only: no refactor, no code
tweaks, no downgrades, no cherry-picking, and no partial file selection.

Rusty IDD already preserves full upstream mirrors for selected dependencies in
`third_party/upstream/`. Those mirrors are review and rollback references, not
workspace members or live implementation code.

## Decision

Rusty IDD will adopt Grit as a full tracked-file upstream mirror at
`third_party/upstream/grit`, pinned to commit
`57b60842d71145c271b994bb7a8c33c3bca42dfe`.

The mirror is imported from `git archive` so tracked dotfiles, workflows,
scripts, docs, tests, examples, assets, nested projects, manifests, and
lockfiles are preserved while Git metadata and untracked local outputs are
excluded. Rusty IDD will not modify Grit source code in this adoption slice.

## Consequences

- Future Rusty IDD work can use Grit as local graph, scan, plan, and rollback
  evidence.
- The repository grows by the full tracked Grit snapshot, including binary
  assets and benchmark fixtures.
- Any future cut, refactor, dependency change, or direct runtime integration
  must cite new evidence and use a separate OpenSpec change.
- The mirror stays outside the Cargo workspace unless a future ADR changes that
  boundary.
