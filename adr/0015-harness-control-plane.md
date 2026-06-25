# 0015. Rusty IDD is the single harness control plane

- Status: accepted
- Date: 2026-06-23

## Context

The repository carries multiple AI-agent surfaces — `.claude`, `.codex`,
`.devin`, `.agents`, plus `CLAUDE.md` / `AGENTS.md` and many rule files. Each is
hand-authored prose/TOML, each overlaps the others, and each drifts. Every model
that enters the repo must independently (1) discover those files, (2) synthesize
the workflow order from prose, and (3) choose to follow it over its drive to
finish. All three are lossy, for every model — and the corpus is re-injected each
session, making it a token black hole that spends context on selection instead
of the current step.

Three in-force ADRs already point at the cure but each covers only a slice:

- **ADR-0001** — the harness *follows* Rusty IDD flow; `AI_MERGE/` is evidence,
  not the control plane (scoped to the Codex harness).
- **ADR-0002** — vendor surfaces are a *thin portable front door* over canonical
  sources (scoped to the template).
- **ADR-0010** — *Rusty IDD owns task-scoped agent harness packages; `.codex`,
  `.claude`, `.kimi`, `.agents` are adapters that must invoke Rusty IDD package
  generation, not grow into always-loaded toolboxes* (first slice: `scan`).

The engine already provides the hard parts: the artifact-DAG **oracle**
(`spec status` / `spec next`), graph/diagram **generation** (`knowledge`), and
the first **stage package** (`harness package`). What was missing is a single
**front door** that turns repository state into one deterministic next
imperative the thin adapters can call.

## Decision

Rusty IDD is the single harness control plane, and the vendor directories are
minimal adapters. Concretely:

1. **`rusty-idd next` is the front door.** It resolves the active change from
   `.idd/workflow/active-change` and prints the artifact-DAG status, the single
   next ready artifact, and one scoped command to produce it. Vendor surfaces
   obtain workflow direction by invoking `rusty-idd next`, not by carrying a
   static prose harness.
2. **One oracle.** The next-step computation is the spec engine's artifact-DAG
   schema (`rusty_idd_spec::schema`); `rusty-idd next` and `rusty-idd spec next`
   are the same computation and cannot disagree.
3. **Source of truth is Git-tracked text** under `.idd/`, `openspec/`, and
   `adr/`. State precedence is Git > the artifact DAG > vendor adapters.
4. **Vendor adapters stay minimal** (generalizing ADR-0010 from `.codex`-only to
   all surfaces). Enforcing thinness by rendering + drift-checking the adapters
   (`rusty-idd render` / `render --check`) is the next slice.
5. **Hybrid runtime.** Rusty IDD owns definitions, the determinism front door,
   gates, and headless execution (`runner`); vendor runtimes remain the
   executors of interactive subagents. Rusty IDD does not reimplement a vendor
   agent runtime.

This ADR unifies and extends ADR-0001, ADR-0002, and ADR-0010; it does not
supersede them — they remain in force as the slices this decision composes.

## Consequences

- **Easier:** one command (`rusty-idd next`) answers "what do I do now" for any
  model; determinism comes from the engine, not from each model's reading of
  prose; per-session context cost is bounded to the active step.
- **Easier:** new workflow rules attach to the engine (DAG, stage packages),
  inheriting the oracle and gates instead of accreting in prose files.
- **Harder / follow-up:** vendor adapters are not yet *enforced* thin — this
  decision ships the front door; the `render` + drift gate and the hook wiring
  are tracked follow-up changes. Until then, adapters can still drift.
- **Neutral:** `rusty-idd next` is advisory (read-only) in this slice;
  fail-closed enforcement continues to live in the push/CI gates (preflight,
  ADR-0005 test-evidence) and will be tightened as adapters are rendered.
- **Debt noted:** a duplicate `0002` ADR number (autonomous-hooks vs
  portable-template) should be reconciled as adapters are formalized.
