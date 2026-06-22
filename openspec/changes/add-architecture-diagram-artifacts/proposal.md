# add-architecture-diagram-artifacts

## Why

Rusty IDD can generate knowledge, architecture, operating-model, integration,
readiness, manifest, and validation artifacts, but the architecture diagrams
used by agents are still a hand-maintained documentation surface. That creates a
gap between the current code graph and the diagrams agents consult before
planning upgrades.

## What Changes

- Add a deterministic architecture diagram artifact generated from the current
  Rusty IDD knowledge graph.
- Expose the diagram generation through the `rusty-idd knowledge` CLI and the
  repo `Justfile`.
- Regenerate every deterministic Rusty IDD artifact against the current
  codebase.
- Record a gap report and upgrade evidence for the generated artifact surface.

## Capabilities

### New Capabilities

- `architecture-diagram-artifacts`: generate current Mermaid architecture
  diagrams from the Rusty IDD architecture graph.

### Modified Capabilities

- `knowledge`: include diagram generation as a first-class generated artifact
  path alongside architecture, system architecture, operating model,
  integration, readiness, plan-context, and manifest artifacts.

## Impact

- `crates/knowledge`
- `crates/cli`
- `docs/rusty-idd/architecture-diagrams.md`
- `.idd/knowledge`
- `.idd/MANIFEST.tsv`
- `Justfile`
- `.agents/skills/rusty-idd-knowledge`
- `AI_MERGE`
