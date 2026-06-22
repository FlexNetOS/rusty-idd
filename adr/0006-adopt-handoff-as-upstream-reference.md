# ADR 0006: Adopt Handoff As A Full Upstream Reference

- **Status:** Accepted
- **Date:** 2026-06-22
- **Change:** `adopt-full-handoff-upstream`
- **Builds On:** `adr/0005-rusty-idd-consumes-handoff-dotdirs.md`

## Context

ADR 0005 decided that Rusty IDD is the canonical product and control plane, and
that Rusty IDD consumes `meta/handoff` whole through adopt-first migration. That
decision intentionally left the first implementation slice for later: import or
mirror the complete tracked `meta/handoff` surface before any adapter or cleanup
work.

The owner has now clarified the gap directly: the entire handoff repo is needed
in Rusty IDD. Rusty IDD already preserves full upstream mirrors under
`third_party/upstream/` for adoption, audit, graph, diagnostics, and rollback
evidence. Handoff should follow that existing pattern.

At import time, the source handoff checkout had local modified and untracked
files. Those are not a reproducible Git commit, so they are recorded as source
state evidence rather than silently promoted into the Rusty IDD mirror.

## Decision

Rusty IDD adopts `meta/handoff` as a full tracked-file upstream mirror at
`third_party/upstream/handoff`, pinned to commit
`7be85fcea3c2454fc3470fc929860afb7ea9864b`.

The mirror is imported from `git archive` so tracked dotfiles, dot directories,
workflows, scripts, docs, tests, nested crates, manifests, lockfiles, task
cards, fleet capsules, policies, packets, ledger text evidence, and embedded
Rusty IDD subset files are preserved. Git metadata, untracked source-checkout
files, local lock files, binary cache state, and uncommitted dirty changes are
excluded from the mirror.

The mirror is not a Cargo workspace member. It is a Rusty IDD planning,
diagnostics, parity, and rollback reference. Future handoff behavior migration
must add typed Rusty IDD adapters and parity tests before cutting or retiring
any upstream handoff surface.

## Consequences

- Rusty IDD now contains the complete tracked handoff source baseline.
- Future adapter work can graph, inspect, test, and compare against local
  handoff source without relying on memory or a sibling checkout.
- The repository grows by the full tracked handoff snapshot, including tracked
  dot directories and handoff evidence files.
- Source handoff dirty/untracked state remains visible in AI_MERGE evidence but
  is not treated as canonical.
- Any future refresh to include later handoff commits requires a new evidence
  update tied to the newer source commit.

## Rollback

Revert this adoption PR. Since the mirror is not part of the Cargo workspace and
does not own runtime state, rollback removes the upstream reference and evidence
without data migration.
