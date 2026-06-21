# add-integration-status-queue - Design

## Context

`rusty-idd spec plan-integration` turns a selected integration work item into
OpenSpec artifacts. The missing control-plane layer is queue state: which work
items have no change yet, which have generated artifacts, which have complete
tasks and are ready to archive, and which have already moved to the archive.

The queue should be derived from existing artifacts rather than introducing a
new mutable database.

## Goals / Non-Goals

**Goals:**

- Read `.idd/knowledge/integration-plan.json`.
- Inspect `openspec/changes/<change_id>` and
  `openspec/changes/archive/<change_id>`.
- Classify each work item deterministically.
- Emit JSON and Markdown.
- Identify the next planned work item for `spec plan-integration`.

**Non-Goals:**

- Running implementation tasks.
- Mutating OpenSpec changes or archive directories.
- Starting daemons, MCP servers, host services, or peer-repo jobs.
- Replacing `spec status` for per-change artifact DAG checks.

## Decisions

- Add this under `rusty-idd knowledge` because it produces a durable
  `.idd/knowledge` artifact from generated knowledge plus repo state.
- Treat a change as artifact-complete when `proposal.md`, `design.md`,
  `tasks.md`, and at least one `specs/**/spec.md` exist.
- Treat a change as ready to archive when artifact-complete and `tasks.md`
  contains no unchecked task markers (`- [ ]`).
- Treat an archived change as highest precedence when
  `openspec/changes/archive/<change_id>` exists.
- Treat the next work item as the lowest-priority item with `planned` status.

## Risks / Trade-offs

- Task checkbox parsing is intentionally simple. It is deterministic and
  matches Rusty IDD's current task style, but future richer task syntax may
  require an parser upgrade.
- Older changes may remain unarchived even after merge. This command reports
  that state; it does not rewrite history or archive automatically.

## Migration Plan

1. Add DTOs and status builder.
2. Add CLI command and tests.
3. Add generation/check targets.
4. Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
5. Run focused tests and full gates.

## Open Questions

- Whether future work should let `spec plan-integration` call this queue and
  automatically skip already scaffolded work by default.
