## MODIFIED Requirements

### Requirement: Single-repo architecture shall use Rusty IDD as the canonical control plane
The single-repo architecture for Rusty IDD and handoff SHALL make Rusty IDD the
canonical product, planning engine, validation engine, and workflow control
plane, with handoff consumed as a whole adopted capability.

#### Scenario: Corrected owner intent supersedes handoff-outer architecture
- **GIVEN** prior ADR `adr/0004-handoff-outer-single-repo.md` selected handoff
  as the outer repository
- **WHEN** the owner clarifies that handoff contains central management plus a
  `.handoff` harness-derived surface that is not the desired foundation
- **THEN** a new ADR SHALL supersede the handoff-outer decision.
- **AND** future implementation SHALL migrate handoff into Rusty IDD rather than
  embedding Rusty IDD under handoff.
