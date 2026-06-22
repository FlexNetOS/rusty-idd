# ADR-0004: Handoff-Outer Single Repository For Rusty IDD And Handoff

- **Status:** Accepted
- **Date:** 2026-06-22
- **Change:** `plan-handoff-single-repo-architecture`
- **Task:** `KBTASK-RUSTY-IDD-HANDOFF-SINGLE-REPO-ARCHITECTURE`

## Context

The owner goal is:

```bash
rusty-idd --goal-file [rusty-idd and handoff must be combined into a sinlge repo]
```

Rusty IDD and handoff currently form one autonomous workflow across two repos:

- Rusty IDD owns intent, generated knowledge, OpenSpec, ADR, validation,
  manifest, Codex workflow gates, and merge evidence.
- Handoff owns witnessed task cards, claims, leases, checkpoints, delivery,
  fleet state, session packets, and the `hf` CLI.

The current handoff repo already embeds an older Rusty IDD subset under
`crates/cli`, `crates/core`, `crates/runner`, `crates/spec`, and `crates/tui`.
The current Rusty IDD repo has advanced beyond that subset with
`crates/knowledge`, `crates/merge-tools`, generated architecture diagrams,
Codex workflow checks, and current `.idd/knowledge` artifacts.

## Decision

Use **handoff as the outer canonical repository**, and embed current Rusty IDD as
explicit peer workspace packages inside handoff.

This is not a flattening of Rusty IDD into `hf`, and it is not the old stale
`COPY + REFERENCE` plan that copied Rusty IDD to `crates/intent-analysis`.
The target shape is:

- `hf`, `ledger`, and `work-order` remain handoff-owned packages.
- Rusty IDD packages remain Rusty IDD-owned peer packages inside the same Cargo
  workspace.
- `.handoff/` remains the witnessed execution truth.
- `.idd/`, OpenSpec, ADR, and generated knowledge remain the planning and
  validation truth.
- The first migration slices adopt current Rusty IDD intact enough to build and
  diagnose before cutting duplicate or stale handoff-embedded Rusty IDD files.

## Options Considered

### Option A: Rusty IDD Embedded In Handoff

Selected, with the important constraint that Rusty IDD remains a modular package
family and control-plane surface, not code absorbed into `hf`.

Evidence:

- Handoff already contains the `hf`, `ledger`, and `work-order` packages that
  own witnessed execution state.
- Handoff already contains an older Rusty IDD subset, so the repository shape is
  partially in place.
- The system operating model records `IDD and spec engine` as partial with
  owners `repo:handoff` and `repo:rusty-idd`, and the anchor `Rusty IDD built
  into handoff`.
- The user goal now requires one repository; using handoff as the outer repo
  preserves the execution kernel and fleet truth where it already lives.

### Option B: Handoff Embedded In Rusty IDD

Rejected.

Rusty IDD is the stronger planning and validation control plane, but handoff is
the stronger execution kernel. Moving `hf`, `.handoff`, ledger export/import,
fleet status, delivery, and task-card semantics into Rusty IDD would make the
execution truth subordinate to the planning engine and would increase risk of
replacing witnessed state semantics with local workflow guesses.

### Option C: New Peer-Only Combined Repository

Rejected for this migration.

A new outer repo would preserve boundaries but would add another coordination
surface and delay consolidation. Handoff already carries the execution kernel
and an embedded Rusty IDD subset, so using it as the outer repo is the smallest
high-fidelity path.

## Consequences

- Handoff becomes the single repository that agents enter for the combined
  Rusty IDD plus handoff workflow.
- Rusty IDD retains its product identity as crates, CLI, `.idd`, OpenSpec, ADR,
  generated knowledge, and validation gates inside the handoff repo.
- The standalone Rusty IDD repo should not be retired until handoff builds and
  validates with current Rusty IDD parity.
- Migration must preserve task-card, ledger, delivery, and fleet semantics
  exactly; these are not Rusty IDD planning notes.
- Migration must preserve Rusty IDD knowledge, OpenSpec, Codex workflow checks,
  and manifest semantics exactly; these are not handoff packet projections.

## Migration Strategy

1. Adopt current Rusty IDD into handoff intact enough to inspect and build.
2. Add missing current Rusty IDD packages to handoff: `knowledge`,
   `merge-tools`, and the required external compatibility packages.
3. Upgrade handoff's existing Rusty IDD subset to match current Rusty IDD before
   deleting or renaming anything.
4. Wire generated `.idd/knowledge`, OpenSpec, ADR, and manifest gates at the
   handoff root.
5. Add `hf` to Rusty IDD workflow evidence by contract, not by replacing the
   `hf` CLI.
6. Remove duplicate/stale Rusty IDD surfaces only after parity tests pass.
7. Retire or archive the standalone Rusty IDD repo only after handoff is the
   proven canonical source.

## Validation Gates

- Handoff: `cargo test --workspace`, `cargo test --workspace --no-default-features`,
  clippy, fmt, drift, audit, and promotion gates.
- Rusty IDD inside handoff: build, tests, OpenSpec validation, manifest check,
  knowledge freshness, diagram freshness, Codex workflow checks, and audit.
- Cross-contract: a task must be traceable from Rusty IDD goal/OpenSpec evidence
  to handoff `WorkOrder`, claim, checkpoint, done, and delivery evidence.

## Rollback

Revert the handoff migration PR. Keep the standalone Rusty IDD repo unchanged
until handoff proves parity, so rollback is a branch revert rather than a
repository reconstruction.
