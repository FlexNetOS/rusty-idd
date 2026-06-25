# Fleet-Deploy Control-Plane Goal

rusty-idd --goal-file .idd/goals/fleet-deploy-control-plane.md

Deploy the Rusty IDD full package across the entire meta fleet so every fleet
repository is governed by one deterministic, engine-owned control plane instead
of each repo carrying its own divergent prose harness. Rusty IDD does **not**
replace each repo's runtime: it **consumes** each repo's existing
vendor/provider harness (its forge loop and runtime) and installs a *minimal*
adapter surface on top, so an agent can be swapped seamlessly between repos
because the harness it sees is the same thin CLI-over-MCP front door everywhere
(`rusty-idd next`), not a repo-specific prose blob.

This preserves the Rusty IDD workflow order:

1. Goal file and graph-backed context (`knowledge plan-context`).
2. OpenSpec proposal, spec delta, design, ADR, and tasks; readiness via
   `spec status` / `spec next`.
3. Task-scoped harness package for the active stage (`harness package`).
4. Implementation only after OpenSpec is ready.
5. Generated-artifact refresh + mandatory validation before completion and push.

## Intent

Add the missing **fleet front door**: a single deterministic command that takes
the rusty-idd thin-adapter surface (already rendered for the home repo by
`rusty-idd render`) and **deploys** it into one or more *target* fleet repos —
writing the per-vendor `rusty-idd-adapter.md` and the SessionStart hook that
calls `rusty-idd next`, while never touching the target repo's own forge loop or
runtime. The result: every fleet member presents the same minimal agent harness,
backed by the one engine, so agents are portable across the fleet.

The "full package" deployed is the rusty-idd binary surface plus its rendered
thin adapters and hooks — the control plane — applied per repo. Adopting the
handoff and prompt_hub runtimes *into* rusty-idd, and retiring those standalone
repos, are sequenced follow-on goals; this slice delivers the deploy mechanism
and proves it against the home repo and at least one peer (handoff).

## Required Method

- Bind this goal with `rusty-idd knowledge plan-context --goal-file` over the
  refreshed `.idd/knowledge/*` graph artifacts.
- Bind the goal to OpenSpec change `fleet-deploy-control-plane` and drive it with
  the artifact-DAG oracle (`rusty-idd next` / `spec status` / `spec next`).
- Reuse the existing engine-owned adapter source of truth
  (`render::expected_adapter`) and the VENDORS gate; the deploy command MUST
  render byte-identical adapters to `render`, only into a target repo root.
- Record the architecture as ADR-0017, building on ADR-0010 (stage packages),
  ADR-0015 (single control plane), and the `render` / `render --check` drift
  gate, without superseding them.
- A `--check` / `--dry-run` mode that reports drift per target without writing,
  so fleet deployment is verifiable and idempotent.
- Refresh `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and validation evidence
  before completion (refresh-last → validate → manifest).

## Decision Target

Rusty IDD SHALL expose a fleet-deploy command that installs its thin-adapter
control-plane surface (per-vendor `rusty-idd-adapter.md` + SessionStart hook
calling `rusty-idd next`) into a target fleet repo without mutating that repo's
forge loop or runtime, with an idempotent `--check`/`--dry-run` drift mode, so
the same minimal agent harness is presented across the whole fleet.

## Non-Goals

- No adoption of handoff or prompt_hub runtimes into rusty-idd in this slice
  (sequenced follow-on; this slice only deploys the control-plane surface).
- No retirement, archiving, or `.meta.yaml` unregistration in this slice.
- No mutation of any target repo's forge loop, runtime, build, or generated
  artifacts — the deploy is additive (adapters + hooks) only.
- No native Rust swarm runtime; vendor runtimes still execute subagents.
- No downgrade of any existing gate, dependency, or generated artifact.
