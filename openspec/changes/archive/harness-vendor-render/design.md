# harness-vendor-render — Design

## Context

ADR-0010/ADR-0015 make vendor dirs thin adapters; `rusty-idd next` (+ `--json`)
is the front door. Nothing yet *enforces* thinness. This slice adds generation +
a drift gate so the invariant is checked, not just documented.

## Goals / Non-Goals

**Goals:**
- One engine-owned template = the source of truth for adapter content.
- `rusty-idd render` writes the adapter; `render --check` fails closed on drift.
- Deterministic output; CI + Justfile run the gate.

**Non-Goals:**
- No rewrite of existing vendor hooks (backlog 4.2); adapter is an additive thin
  pointer.
- No new vendor surfaces; target the known set (claude, codex, agents, devin).
- No new ADR (ADR-0010 + ADR-0015 govern this).

## Decisions

1. **Template is a `const` in the engine** with a single `{vendor}` substitution.
   Adapter path: `<vendor-dir>/rusty-idd-adapter.md`. The body tells agents to
   run `rusty-idd next` / `next --json` and to keep no workflow prose here.
2. **Known vendor set is fixed** in code: `claude → .claude`, `codex → .codex`,
   `agents → .agents`, `devin → .devin`. `--vendor <name>` targets one; `--all`
   targets the set.
3. **`render` (write)** vs **`render --check` (compare)** share one
   `expected_adapter(vendor)` function so write and check can never disagree.
4. **Drift = missing OR different.** `--check` collects all drifted/missing
   adapters, prints them, exits 1; clean → exit 0, no writes.
5. **Scope to existing dirs for `--all`/`--check`** so the gate does not demand
   adapters in vendor dirs a repo does not use; an explicit `--vendor` may create
   its dir.
6. **CI gate:** add `rusty-idd render --all --check` to `ci.yml` and a Justfile
   recipe, alongside the manifest/validate gates.

## Risks / Trade-offs

- Adding a CI gate means every future PR must keep adapters in sync — that is the
  intent. This PR commits the rendered adapters so the gate passes from the start.
- A fixed vendor set in code is simple but must be extended in code when a new
  surface is adopted; acceptable (adoption is a deliberate, reviewed act).

## Migration Plan

- Additive: a new `rusty-idd-adapter.md` per vendor dir; existing files
  untouched. Rollback = remove `render` + the adapters + the CI step.

## Open Questions

- Whether to later fold existing hook config (`.codex/hooks.json`) into the
  rendered set (backlog 4.2). Out of scope here.
