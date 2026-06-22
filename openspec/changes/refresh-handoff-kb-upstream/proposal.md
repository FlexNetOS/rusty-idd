# refresh-handoff-kb-upstream

## Why

Final mirror proof showed `meta/handoff` advanced after the previous adoption
PRs. The current committed handoff HEAD tracks 550 files and adds `.kb/*`
knowledge state. Rusty IDD must carry the entire current tracked handoff repo,
not an older partial pin.

## What Changes

- Refresh `third_party/upstream/handoff` from handoff commit
  `6365c12fc38f5d7247d81f9fdbd3a55817797904`.
- Update `third_party/upstream/UPSTREAMS.md` to the new handoff pin and tracked
  file count.
- Add ADR 0008 documenting the `.kb` refresh and dirty-source exclusion.
- Refresh `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, architecture diagrams, and
  AI_MERGE evidence.

## Impact

- Affected specs: `handoff-upstream-adoption`
- Affected code: no runtime Rust code; upstream mirror and generated control
  artifacts only
