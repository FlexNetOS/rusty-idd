# Rusty IDD Consumes Handoff Dot-Directory Architecture

Goal file: `.idd/goals/rusty-idd-consumes-handoff-dotdirs.md`

OpenSpec change: `openspec/changes/supersede-handoff-dotdir-ownership`

ADR: `adr/0005-rusty-idd-consumes-handoff-dotdirs.md`

Task card: `KBTASK-RUSTY-IDD-CONSUMES-HANDOFF-DOTDIRS`

## Decision

Rusty IDD is the canonical product and workflow engine. It consumes
`meta/handoff` whole through adopt-first migration. Handoff does not become the
outer repository for Rusty IDD.

The `.handoff` directory and harness-loop lineage remain valuable evidence and
compatibility material, but `.idd`, OpenSpec, ADR, manifest, generated
knowledge, and validation are the canonical planning and readiness surfaces.

## Evidence Files

- `dot-directory-policy.md`: inventory, state precedence, migration phases,
  first implementation slice, rollback, and risks.
- `graphs.md`: visual ownership, lifecycle, adoption, compatibility, state
  precedence, and target layout graphs.

## Validation Target

This package is planning-only. It moves no handoff source code. Validation must
refresh `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, OpenSpec status, and Rusty IDD
workflow checks after the planning artifacts are present.
