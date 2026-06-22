# Handoff Full Adoption Evidence

Goal file: `.idd/goals/adopt-full-handoff-upstream.md`

OpenSpec change: `openspec/changes/adopt-full-handoff-upstream`

ADR: `adr/0006-adopt-handoff-as-upstream-reference.md`

Task card: `KBTASK-RUSTY-IDD-ADOPT-FULL-HANDOFF`

## Decision

Rusty IDD now contains the complete tracked `meta/handoff` repository as an
adopt-first upstream/reference mirror at `third_party/upstream/handoff`.

The mirror is pinned to source commit
`7be85fcea3c2454fc3470fc929860afb7ea9864b` and includes all 533 tracked handoff
files. It is not a Cargo workspace member and does not execute as part of
default Rusty IDD builds.

## Gap Hunt Result

The merged dot-directory architecture was correct but incomplete for the
owner's current requirement. It defined the policy and graphs, but Rusty IDD did
not yet contain the full handoff repo. This change closes that gap by importing
the tracked handoff baseline and recording proof that no tracked file was left
behind.

## Evidence Files

- `mirror-verification.md`: source count, mirror count, and top-level surface
  breakdown.
- `handoff-tracked-files.md`: complete tracked source file list.
- `handoff-source-state.md`: clean source checkout state recorded before import.

## Migration Note

Old path: `meta/handoff` sibling repository only.

New path: `third_party/upstream/handoff` full tracked mirror inside Rusty IDD.

Behavior is not moved yet. The next implementation slice should add typed
Rusty IDD adapters for `hf`, `ledger`, `work-order`, and durable `.handoff`
evidence semantics.

## Rollback

Revert this PR. The mirror is a reference snapshot, not a runtime state owner,
so rollback is a branch revert with no data migration.
