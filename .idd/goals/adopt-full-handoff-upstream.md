# Adopt Full Handoff Upstream Into Rusty IDD

```bash
rusty-idd --goal-file .idd/goals/adopt-full-handoff-upstream.md
```

## Intent

Deep-review the Rusty IDD handoff dot-directory architecture and close the
implementation gap: the complete tracked `meta/handoff` repository must be
present inside Rusty IDD as an adopt-first upstream/reference source.

## Required Method

- Track the work through `KBTASK-RUSTY-IDD-ADOPT-FULL-HANDOFF`.
- Use a fresh worktree from `origin/develop`.
- Review the merged dot-directory architecture, ADR 0005, and AI_MERGE graph
  evidence before importing.
- Import `meta/handoff` from a pinned Git commit with `git archive`.
- Preserve every tracked handoff file, including tracked dot directories.
- Exclude Git metadata, untracked runtime output, local locks, binary cache
  state, and uncommitted source-checkout changes.
- Record any excluded dirty/untracked source state as migration evidence.
- Add OpenSpec, ADR, evidence, generated knowledge, architecture diagrams, and
  manifest updates.

## Validation Target

Rusty IDD must contain a complete tracked-file mirror of handoff under
`third_party/upstream/handoff`, with import evidence proving no tracked handoff
file was left behind.
