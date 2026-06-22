# Rusty IDD Consumes Handoff And Dot-Directory Architecture Goal

```bash
rusty-idd --goal-file .idd/goals/rusty-idd-consumes-handoff-dotdirs.md
```

## Intent

Start from the corrected architecture decision: `meta/rusty-idd` is the
canonical product and workflow engine, and it consumes `meta/handoff` whole.
Handoff does not become the outer repository for Rusty IDD.

This planning pass must explain how the dot directories work before they pile up
into an ungoverned control plane. The required surfaces include `.idd`,
`.handoff`, `.kb`, `.idea`, `.claude`, `.codex`, `.agents`, `.github`, and any
other dot directory that affects agent workflow, task state, evidence, or
repository behavior.

## Required Method

- Create a fresh Rusty IDD worktree based on `develop`.
- Track the work through a handoff task card.
- Supersede the prior handoff-outer single-repo ADR with a new ADR.
- Treat `meta/handoff` as an adoption source and future embedded capability,
  not as the canonical repo.
- Treat the `.handoff` directory produced by `/harness:handoff-loop-init` and
  tracing to `meta/harness_hub` as legacy compatibility/evidence input, not the
  authoritative control plane.
- Generate and refresh Rusty IDD knowledge, system, operating-model,
  integration, readiness, plan-context, architecture diagram, OpenSpec, ADR, and
  evidence artifacts.
- Build explicit visual graphs for dot-directory ownership, lifecycle flow,
  adoption/migration, compatibility, and repository layout.

## Validation Target

The final planning artifact must define dot-directory ownership rules,
retention/migration rules, graph-backed architecture, rollback, and the smallest
implementation slice for Rusty IDD consuming handoff whole.
