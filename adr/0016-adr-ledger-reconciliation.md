# 0016. ADR ledger reconciliation: frozen collisions, slug-canonical references, fail-closed gate

- Status: accepted
- Date: 2026-06-23

## Context

The ADR ledger carries four duplicate sequence numbers:

- **0002** — "Autonomous workflow hooks enforce Rusty IDD gates" and "Portable
  template agent surface"
- **0004** — "Adopt Grit As-Is as an Upstream Reference" and "Handoff-Outer
  Single Repository For Rusty IDD And Handoff"
- **0005** — "Rusty IDD Consumes Handoff And Governs Dot Directories" and
  "Require Post-Artifact Test Evidence Before Completion and Push"
- **0006** — "Delivery Evidence Requires Success Semantics" and "Adopt Handoff
  As A Full Upstream Reference"

These arose from concurrency, not a buggy allocator: `rusty-idd spec adr next`
returns `max(number) + 1`, so it never re-issues a *used* number. But several
changes were authored in parallel; each read the next free number before any of
them had committed, so two changes legitimately saw the same value and both used
it. ADR-0015 already recorded the 0002 case as debt to reconcile; this ADR
reconciles all four and prevents the next one.

ADRs are immutable once accepted (supersede, don't edit). Renumbering one ADR of
each colliding pair would change its identity and cascade through every
`ADR-000N` citation across docs, specs, and code comments — an edit, not a
supersession. That is not an acceptable fix.

## Decision

1. **The four existing collisions (0002, 0004, 0005, 0006) are frozen historical
   artifacts.** The colliding ADR files are left exactly as accepted; none is
   renumbered, edited, or superseded by this reconciliation.
2. **ADRs are canonically referenced by slug, not by bare number.** Where a
   reference could be ambiguous (a colliding number), cite the filename slug
   (e.g. `0002-portable-template-agent-surface`) rather than `ADR-0002`. New,
   non-colliding ADRs may still be cited by number.
3. **A fail-closed collision gate prevents recurrence.** `rusty-idd spec adr
   list --check` groups ADRs by number and exits non-zero on any duplicate
   outside a frozen baseline of the four numbers above (encoded as
   `ACCEPTED_DUPLICATE_ADRS` in the CLI). It is wired into CI. Any *new*
   collision fails the build; the four accepted ones are reported but do not
   fail. This mirrors the `.cargo/audit.toml` baseline philosophy already used
   for supply-chain advisories: known-accepted exceptions are frozen, anything
   new fails closed.
4. **Adding to the baseline requires an ADR.** A number may only join
   `ACCEPTED_DUPLICATE_ADRS` alongside an ADR explaining why the collision is an
   accepted immutable artifact. The default remedy for a new collision is to
   give the new ADR the next free number from `spec adr next`.

This ADR does not supersede any existing ADR; it adds a referencing convention
and a recurrence guard.

## Consequences

- **Easier:** the ledger's known collisions are documented in one place and can
  no longer silently grow; CI catches the next concurrent-allocation mistake.
- **Easier:** slug-canonical references are unambiguous for the colliding
  numbers without rewriting history.
- **Neutral:** the four historical collisions remain; tooling and humans
  disambiguate by slug. `spec adr list --all` continues to show every ADR with
  its number for full visibility.
- **Trade-off:** the baseline is a hardcoded constant, so adding an accepted
  collision is a deliberate code change plus an ADR — by design, to keep the
  exception list auditable.
