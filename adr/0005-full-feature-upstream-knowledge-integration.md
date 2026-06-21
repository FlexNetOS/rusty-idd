# 0005. Full Feature Upstream Knowledge Integration

- Status: accepted
- Date: 2026-06-21
- Related: ADR-0004

## Context

ADR 0004 captured the PR #52 local knowledge slice: keep `crates/core` std-only,
prefer direct crate or adapter boundaries over MCP transport for default
knowledge paths, and adopt upstream before cutting. That decision remains valid
for the local slice, but the broader system needs a clearer rule for future
work.

Rusty IDD consumes OpenSpec proposals, specs, ADRs, tasks, implementation
records, and validation evidence as product inputs. A merge process is one
workflow inside that lifecycle, not the lifecycle itself.

The wider system also invalidates stale assumptions from the first pass:
`tree-sitter` is active through Yazelix, domains are handled through weave plus
Obscura upgrades, and daemon or MCP surfaces may exist in the system even when
they are excluded from default Rusty IDD workflows.

## Decision

Rusty IDD knowledge integrations SHALL use a full-feature adopt-first strategy:

- Verify and pin current upstream repo revisions before implementation.
- Preserve each upstream repo as an intact tracked integration surface before
  judging which parts are useful.
- Run each upstream repo's native build, test, lint, docs, audit, smoke, and
  diagnostic commands as discovered from the upstream repo before cutting.
- Record results, failures, required tools, runtime assumptions, generated
  assets, feature flags, valuable surfaces, cuts, and rollback paths in
  `/AI_MERGE`.
- Preserve proven upstream behavior through the thinnest local boundary:
  deterministic DTO mapping, validation, size/token policy, feature flags, and
  CLI/API surfaces.
- Keep `crates/core` std-only.
- Keep host-service, daemon, MCP, and fleet management out of default Rusty IDD
  workflows unless explicitly feature-gated and justified.
- Provision missing required tools through parent `meta` / `envctl` or tracked
  repo-local surfaces, not user-global installation.

## Consequences

This raises the evidence bar before any consolidation. It may keep more
upstream material tracked and make diagnostics slower, but it prevents silent
feature loss from premature cherry-picking.

Future Rusty IDD work must distinguish system availability from default workflow
scope. `tree-sitter` and domain capabilities cannot be described as absent from
the system; they can only be scoped out of a specific default path with
evidence.

ADR 0004 remains a record of the PR #52 local-slice decision. ADR 0005 governs
future full-feature upstream knowledge integration work.
