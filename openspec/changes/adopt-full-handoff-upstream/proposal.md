# adopt-full-handoff-upstream

## Why

ADR 0005 corrected the combined architecture: Rusty IDD is canonical, and
`meta/handoff` is consumed whole through adopt-first migration. The planning PR
created the control-plane model and graphs, but it intentionally did not move
handoff source. The gap is now concrete: Rusty IDD still lacks the complete
tracked handoff repository as a local, graph-visible, reviewable adoption
baseline.

The owner has clarified that the entire handoff repo is needed in Rusty IDD.
This change implements the first adoption slice without refactoring, trimming,
cherry-picking, or flattening handoff behavior.

## What Changes

- Add a goal file for full handoff adoption.
- Add a complete tracked-file mirror at `third_party/upstream/handoff`, pinned
  to the current handoff commit.
- Update `third_party/upstream/UPSTREAMS.md` with the handoff pin, file count,
  import method, and boundary.
- Add ADR 0006 to record the actual adoption decision and source-state boundary.
- Add AI_MERGE evidence with mirror verification, tracked-file inventory,
  source dirty-state evidence, rollback, and next adapter gaps.
- Regenerate `.idd/knowledge/*`, `docs/rusty-idd/architecture-diagrams.md`, and
  `.idd/MANIFEST.tsv` after the mirror is present.

## Capabilities

### New Capabilities

- `handoff-upstream-adoption`: preserve the full tracked `meta/handoff`
  repository as Rusty IDD's local upstream/reference baseline.

### Modified Capabilities

- `handoff-adoption-into-rusty-idd`: implement the first adopt-first phase that
  ADR 0005 and the dot-directory policy required.

## Impact

- `.idd/goals/adopt-full-handoff-upstream.md`
- `third_party/upstream/handoff`
- `third_party/upstream/UPSTREAMS.md`
- `openspec/changes/adopt-full-handoff-upstream`
- `adr/0006-adopt-handoff-as-upstream-reference.md`
- `AI_MERGE/36_handoff_full_adoption/`
- generated `.idd/knowledge/*`
- generated `docs/rusty-idd/architecture-diagrams.md`
- generated `.idd/MANIFEST.tsv`
