# scaffold-integration-work - Design

## Context

The integration automation plan already contains enough structured data to
seed an OpenSpec change: `change_id`, capability, layer, owner repos, anchors,
adopt-first inputs, implementation boundary, validation gates, and rollback.
Keeping this as report-only output forces agents to repeat the same conversion
manually for every system integration slice.

## Goals / Non-Goals

**Goals:**

- Convert a selected integration work item into proposal, design, tasks, and
  spec delta files.
- Keep generation deterministic and reviewable.
- Refuse overwrites by default.
- Preserve all work-item evidence in the generated artifacts.
- Keep peer repos read-only; this command only writes Rusty IDD OpenSpec files.

**Non-Goals:**

- Running the integration implementation automatically.
- Mutating peer repos, starting daemons, or managing host services.
- Deciding which upstream capability to cut or downgrade.
- Replacing the existing `spec new` stub workflow.

## Decisions

- Add the command under `rusty-idd spec` because it writes OpenSpec lifecycle
  artifacts.
- Read `.idd/knowledge/integration-plan.json` by default, with an override for
  temporary or test plans.
- Default to the lowest-priority-number work item when no selector is given.
- Support selectors for `change_id`, capability id, and work-item id.
- Use the integration work item's validation and rollback fields verbatim in
  generated tasks and design.
- Generate a spec delta under
  `openspec/changes/<change>/specs/<capability>/spec.md`.

## Risks / Trade-offs

- Auto-generated text can become noisy. The command keeps generated content
  direct and evidence-backed instead of trying to author implementation prose.
- A default first-work-item selector can produce the wrong slice if the backlog
  is stale. The generated change is still reviewable, and overwrite refusal
  prevents accidental replacement.

## Migration Plan

1. Add the CLI renderer and tests.
2. Generate a sample OpenSpec change in a temp test workspace.
3. Update knowledge workflow guidance.
4. Refresh `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
5. Run focused tests and full Rusty IDD gates.

## Open Questions

- Whether future work should add a follow-up command that runs the generated
  task gates automatically in owner repos through the meta peer-repo layer.
