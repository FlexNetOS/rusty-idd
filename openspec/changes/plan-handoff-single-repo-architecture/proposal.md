# plan-handoff-single-repo-architecture

## Why

Rusty IDD and handoff currently share the same operating loop but live as
separate repositories. Rusty IDD owns intent, graph/context artifacts, OpenSpec,
ADR, validation, and generated planning evidence. Handoff owns witnessed task
cards, leases, checkpoints, delivery, and fleet coordination. Agents now depend
on both surfaces for one autonomous workflow, which creates duplicated
control-plane state and cross-repo handoff friction.

The owner goal is:

```bash
rusty-idd --goal-file [rusty-idd and handoff must be combined into a sinlge repo]
```

## What Changes

- Create a generated, evidence-backed architecture plan for combining Rusty IDD
  and handoff into one repository.
- Compare three target shapes:
  - embed Rusty IDD inside handoff;
  - embed handoff inside Rusty IDD;
  - preserve both as peer packages/crates in one repository workspace.
- Deep scan current Rusty IDD and handoff code, artifacts, workflows, task
  ledgers, CLI surfaces, and validation gates before choosing.
- Record the selected architecture in an ADR and migration evidence note.
- Define a smallest first implementation slice without moving code in this
  planning-only change.

## Capabilities

### New Capabilities

- `single-repo-architecture-planning`: generate and preserve the decision
  package for the Rusty IDD plus handoff repository unification.

### Modified Capabilities

- `fleet-handoff`: clarify the long-term ownership boundary between Rusty IDD
  workflow state and handoff task/fleet state.
- `integration-automation-plan`: add migration phasing and validation gates for
  the combined repository path.

## Impact

- `rusty-idd` repository architecture artifacts
- `handoff` repository architecture and crate/task-card surfaces
- `.idd/knowledge/*`
- `.idd/MANIFEST.tsv`
- `openspec/changes/plan-handoff-single-repo-architecture/*`
- `adr/`
- `AI_MERGE/`
