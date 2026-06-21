# revisit-upstream-knowledge-integration

## Why

PR #50 and the follow-up PRs established a baseline direct-knowledge path, but
the next pass must use Rusty IDD as the product workflow instead of only a
merge checklist. Rusty IDD consumes OpenSpec plans, ADRs, specs, tasks, and
implementation evidence to automate repo work, while the merge process is only
one execution mode inside that lifecycle.

The previous upstream pass also left several statements too narrow or stale:
`tree-sitter` is active in the wider system through the Yazelix stack, domains
are handled through weave plus Obscura upgrades, and host-service or daemon
surfaces are not blanket-discarded. They are excluded from default Rusty IDD
workflows unless feature-gated and justified.

This change revisits the `codegraph-rust` and `repomix-rs` integrations with an
adopt-first rule: pin the current upstream revisions, build and diagnose the
native upstream repos as-is, then cut only evidenced friction while preserving
capabilities through thin Rusty IDD boundaries.

## What Changes

- Add a full OpenSpec/ADR/task trail for the upstream revisit before
  implementation.
- Re-verify the current upstream repos and exact revisions for `codegraph-rust`
  and `repomix-rs`.
- Run upstream-native build, test, lint, documentation, audit, smoke, and
  diagnostic commands as discovered from each upstream repo.
- Record build/test results, failures, required tools, assumptions, generated
  assets, feature flags, valuable surfaces, cuts, and rollback paths in
  `/AI_MERGE`.
- Correct stale Rusty IDD documentation and local rules around `tree-sitter`,
  domains, daemon surfaces, and the difference between Rusty IDD's product
  lifecycle and a merge process.
- Implement only strict-upgrade fixes: no feature downgrades, no local
  replacement before upstream behavior is proven unsuitable, and no dependency
  simplification that removes a working capability.
- Keep `crates/core` std-only and move heavier knowledge/tool surfaces behind
  adapter or CLI feature boundaries.
- Add or repair missing toolchain requirements through the parent `meta` /
  `envctl` path instead of user-global installs.

## Capabilities

### New Capabilities
- `upstream-knowledge-integration`: adopt-first direct integration of current
  `codegraph-rust` and `repomix-rs` surfaces with Rusty IDD's automated
  OpenSpec lifecycle and evidence trail.

### Modified Capabilities
- `base`: Rusty IDD workflow artifacts are now treated as executable product
  inputs for repo and system goals, not just documentation generated around a
  merge.

## Impact

- `crates/knowledge`, `crates/cli`, and external adapter crates.
- `third_party/upstream/codegraph-rust` and
  `third_party/upstream/repomix-rs`.
- `openspec/changes`, `adr`, `AI_MERGE`, `.idd/knowledge`, and
  `.idd/MANIFEST.tsv`.
- Parent-managed toolchain definitions under `meta` / `envctl` if required by
  native upstream commands.
