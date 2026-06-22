# supersede-handoff-dotdir-ownership

## Why

The previous single-repo planning decision selected handoff as the outer
canonical repository for Rusty IDD and handoff. The owner clarified that this
misidentified the handoff surface: `meta/handoff` contains central management
code plus a `.handoff` directory created by `/harness:handoff-loop-init`, tracing
back to `.claude` harness material in `meta/harness_hub`. That legacy harness
trace is not the desired foundation.

Rusty IDD must therefore own the combined workflow. It should consume
`meta/handoff` whole as an adopted capability while defining how `.idd`,
`.handoff`, `.kb`, `.idea`, and related dot directories compose without creating
competing control planes.

## What Changes

- Add a new Rusty IDD goal file for the corrected architecture.
- Add OpenSpec planning artifacts for Rusty IDD consuming handoff and governing
  dot-directory ownership.
- Add a new ADR that supersedes the handoff-outer ADR.
- Add graph/evidence artifacts that visualize dot-directory ownership,
  lifecycle flow, migration/adoption, compatibility, and repository layout.
- Regenerate `.idd/knowledge/*`, `docs/rusty-idd/architecture-diagrams.md`, and
  `.idd/MANIFEST.tsv` after the planning artifacts are present.

## Capabilities

### New Capabilities

- `dot-directory-control-plane-governance`: define canonical, adopted,
  compatibility, editor, agent, CI, and local-cache dot-directory roles.
- `handoff-adoption-into-rusty-idd`: plan Rusty IDD consuming `meta/handoff`
  whole while preserving useful handoff semantics.

### Modified Capabilities

- `single-repo-architecture-planning`: supersede the handoff-outer repository
  decision with Rusty IDD as canonical owner.

## Impact

- `.idd/goals/rusty-idd-consumes-handoff-dotdirs.md`
- `openspec/changes/supersede-handoff-dotdir-ownership`
- `adr/0005-rusty-idd-consumes-handoff-dotdirs.md`
- `AI_MERGE/35_rusty_idd_consumes_handoff_dotdirs/`
- generated `.idd/knowledge/*`
- generated `docs/rusty-idd/architecture-diagrams.md`
- generated `.idd/MANIFEST.tsv`
