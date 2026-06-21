# revisit-upstream-knowledge-integration - Design

## Context

Rusty IDD is not only a build or merge process. It is the lifecycle engine that
turns OpenSpec proposals, specs, ADRs, task plans, implementation evidence, and
validation into automated repo work. The merge process is one execution outcome
inside that lifecycle.

PR #50 established a baseline knowledge integration and PR #52/PR #53 corrected
parts of the adopt-first strategy, but this pass must revisit the upstream
repos with the stricter rule: adopt the full current upstream surfaces first,
run their own diagnostics as-is, then cut only evidenced friction.

The wider system context also changed the assumptions that shaped the earlier
slice. `tree-sitter` is active through Yazelix. Domains are handled through
weave plus Obscura upgrades. Host-service, daemon, MCP, and fleet management
surfaces remain out of default Rusty IDD workflows, but they are not absent from
the system and must be described as feature-gated or external-system surfaces.

## Goals / Non-Goals

**Goals:**

- Re-run the Rusty IDD workflow in artifact order before implementation.
- Re-verify the current upstream `codegraph-rust` and `repomix-rs` revisions.
- Preserve full upstream mirrors and native diagnostics before consolidation.
- Correct stale statements around `tree-sitter`, domains, daemons, MCP, and
  default workflow scope.
- Keep `crates/core` std-only.
- Wire Rusty IDD through thin local boundaries: DTO mapping, deterministic
  output, validation, size/token policy, feature flags, and CLI/API surfaces.
- Add missing tools through parent `meta` / `envctl` surfaces when required.

**Non-Goals:**

- Managing host services, daemon lifecycle, or MCP servers in default Rusty IDD
  workflows.
- Installing global host tools to make a gate pass.
- Replacing proven upstream behavior with local guesses.
- Flattening the meta peer-repo system into a monorepo layout.

## Decisions

- Use a new OpenSpec change as the control record for this revisit.
- Treat ADR 0004 as the PR #52 local-slice decision, and add ADR 0005 for the
  system-wide full-feature strategy going forward.
- Continue to preserve upstream mirrors under `third_party/upstream/`.
- Default to direct crate or adapter integration for knowledge paths; use MCP,
  daemon, or host-service surfaces only behind explicit feature flags and
  documented justification.
- Treat `tree-sitter` as an active system parser surface because Yazelix carries
  it, even if Rusty IDD's default CLI path only uses a subset.
- Treat domains as active system capability through weave plus Obscura, while
  keeping Rusty IDD default workflows focused on repo artifacts and validation.

## Risks / Trade-offs

- Full upstream adoption increases repository size and validation time, but it
  prevents capability loss caused by cherry-picking before evidence.
- Native upstream diagnostics may fail for upstream reasons. Those failures are
  still useful inputs and must be recorded before local cuts.
- Adding toolchain support through `meta` / `envctl` may touch a parent repo.
  That is allowed only when the tool is required and the change is scoped.

## Migration Plan

1. Create the OpenSpec and ADR artifacts.
2. Verify current upstream revisions and compare them with tracked mirrors.
3. Run native upstream diagnostics as discovered from upstream docs and config.
4. Record the audit trail in `/AI_MERGE`.
5. Apply strict-upgrade implementation fixes in small TDD steps.
6. Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
7. Run focused tests and full gates.
8. Commit, push, PR, wait for checks, merge to `develop`, and leave the worktree
   clean.

## Open Questions

- Whether current upstream revisions introduce new required tools that are not
  already present in repo or parent-managed toolchains.
- Whether `repomix-rs` now exposes a crate surface that can replace any local
  shim without reintroducing audit or compatibility regressions.
- Whether current `codegraph-rust` parser surfaces can support broader
  `tree-sitter` behavior directly in Rusty IDD without pulling daemon or host
  service management into default workflows.
