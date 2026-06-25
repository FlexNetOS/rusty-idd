# Harness Control-Plane Goal

rusty-idd --goal-file .idd/goals/harness-control-plane.md

Make Rusty IDD the single harness control plane so every AI runtime follows one
deterministic, engine-owned workflow instead of re-deriving it from prose spread
across `.claude`, `.codex`, `.agents`, `.devin`, `CLAUDE.md`, and `AGENTS.md`.
The vendor directories stay minimal adapters; the rules, gates, oracle, stage
packages, and workflow are baked into the Rusty IDD engine and served on demand.

This preserves the Rusty IDD workflow order:

1. Goal file and graph-backed context (`knowledge plan-context`).
2. OpenSpec proposal, spec delta, design, ADR, and tasks; readiness via
   `spec status` / `spec next`.
3. Task-scoped harness package for the active stage (`harness package`).
4. Implementation only after OpenSpec is ready.
5. Generated-artifact refresh + mandatory validation before completion and push.

## Intent

Add the missing **front door**: a single command that turns repository state
into one deterministic next imperative, reusing the existing artifact-DAG oracle
so vendor surfaces become thin adapters that call the engine rather than carrying
an always-loaded prose harness (the per-session token black hole).

## Required Method

- Bind this goal with `rusty-idd knowledge plan-context --goal-file` over the
  refreshed `.idd/knowledge/*` graph artifacts.
- Bind the goal to OpenSpec change `harness-control-plane` and drive it with the
  artifact-DAG oracle (`rusty-idd next` / `spec status` / `spec next`).
- Record the architecture as ADR-0015, unifying ADR-0001 (flow), ADR-0002 (thin
  front door), and ADR-0010 (stage packages) without superseding them.
- Generate the architecture graphs with the engine (`knowledge diagrams`),
  dogfooded, plus authored to-be graphs.
- Refresh `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and validation evidence
  before completion.

## Decision Target

Rusty IDD SHALL expose `rusty-idd next` as the harness control-plane front door,
backed by exactly one artifact-DAG oracle, with token-scoped output. Vendor
agent directories SHALL be thin adapters that invoke it.

## Non-Goals

- No native Rust swarm runtime in this slice (vendor runtimes still execute
  interactive subagents).
- No vendor-adapter rendering / drift gate in this slice (tracked follow-up).
- No new workflow-stage packages beyond the existing `scan`.
- No downgrade of any existing gate, dependency, or generated artifact.
