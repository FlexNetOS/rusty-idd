# Refresh Handoff KB Upstream Mirror

Command front door:

```bash
rusty-idd --goal-file .idd/goals/refresh-handoff-kb-upstream.md
```

## Intent

Refresh the Rusty IDD handoff upstream mirror to the current committed
`meta/handoff` HEAD after handoff added its tracked `.kb` knowledge surface.

## Acceptance

- `third_party/upstream/handoff` mirrors every tracked file from handoff commit
  `6365c12fc38f5d7247d81f9fdbd3a55817797904`.
- The mirror includes tracked dot directories, including `.agent`, `.claude`,
  `.githooks`, `.github`, `.handoff`, `.idea`, and `.kb`.
- Source-local dirty handoff working-tree files remain excluded unless committed
  upstream.
- OpenSpec, ADR, `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and AI_MERGE evidence
  are refreshed.
