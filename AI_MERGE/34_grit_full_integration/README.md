# Grit Full Integration Evidence

OpenSpec change: `openspec/changes/adopt-grit-full-integration`

Goal file: `.idd/goals/grit-full-integration.md`

Grit source: `/home/drdave/Desktop/meta/grit`

Pinned commit: `57b60842d71145c271b994bb7a8c33c3bca42dfe`

## Boundary

This is an adopt-first slice. Grit is preserved as an upstream reference under
`third_party/upstream/grit` without source edits, refactors, downgrades,
feature cuts, or cherry-picked subsets.

The Rusty IDD plan workspace is intentionally nested under this evidence
directory so the `rusty-idd plan` command can emit its complete workspace
without overwriting root Rusty IDD control files.

## Artifacts

- `00_grit_inventory.{md,json}`: direct `rusty-idd scan` output for Grit.
- `01_rusty_idd_inventory_before_adoption.{md,json}`: direct `rusty-idd scan`
  output for Rusty IDD before the Grit mirror was imported.
- `plan-workspace/`: direct `rusty-idd plan` output for Rusty IDD plus Grit.
- `adoption-evidence.md`: verification, validation, migration, and rollback
  evidence for the adoption.
