# adopt-grit-full-integration

## Why

Rusty IDD needs a durable, graph-visible reference for Grit so future agent-run,
session, lock-store, cloud-backend, and benchmark work can be planned from real
repo evidence instead of memory or partial summaries. The Grit repository has
just been synced and merged upstream, making this the right point to preserve an
as-is baseline before any later consolidation or implementation work.

The owner constraint for this slice is strict adoption: scan and document the
full Grit surface, including tracked dotfiles and nested projects, but do not
refactor, trim, cherry-pick, downgrade, or change Grit code.

## What Changes

- Add a Rusty IDD goal file for the Grit integration and generate graph planning
  context from `rusty-idd knowledge plan-context --goal-file`.
- Run the Rusty IDD scan and plan workflow for Rusty IDD plus Grit and preserve
  the emitted artifacts under `AI_MERGE/34_grit_full_integration/`.
- Add a full tracked-file upstream mirror at `third_party/upstream/grit`, pinned
  to the current `FlexNetOS/grit` commit.
- Update the upstream mirror ledger with the Grit URL, ref, file count, import
  method, and local boundary.
- Add OpenSpec, ADR, task, evidence, knowledge, architecture diagram, and
  manifest artifacts for the adoption.
- Upgrade Rusty IDD generated-artifact handling so the full Grit adoption can be
  represented without trimming evidence: raise the internal generated-report
  pack ceiling and exclude generated architecture diagrams from the knowledge
  freshness fingerprint.
- Regenerate deterministic `.idd/knowledge/*`, `docs/rusty-idd/architecture-diagrams.md`,
  and `.idd/MANIFEST.tsv` artifacts after the mirror is present.

## Capabilities

### New Capabilities

- `upstream-grit-integration`: preserve and expose a full as-is Grit upstream
  snapshot as a Rusty IDD planning, diagnostics, and rollback reference.

### Modified Capabilities

- None.

## Impact

- `.idd/goals/grit-full-integration.md`
- `third_party/upstream/grit`
- `third_party/upstream/UPSTREAMS.md`
- `openspec/changes/adopt-grit-full-integration`
- `adr/0004-adopt-grit-as-is-upstream-reference.md`
- `AI_MERGE/34_grit_full_integration/`
- `crates/core/src/manifest.rs`
- `crates/knowledge/src/lib.rs`
- `.idd/knowledge/*`
- `docs/rusty-idd/architecture-diagrams.md`
- `.idd/MANIFEST.tsv`
