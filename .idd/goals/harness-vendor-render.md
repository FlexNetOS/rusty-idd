# Harness Vendor-Adapter Render + Drift Gate Goal

rusty-idd --goal-file .idd/goals/harness-vendor-render.md

Make ADR-0010/ADR-0015's "vendor directories are thin adapters" *enforced*, not
just declared. Add `rusty-idd render` to generate the minimal adapter for each
vendor surface (`.claude`, `.codex`, `.agents`, `.devin`) from one engine-owned
source of truth, and `rusty-idd render --check` to fail when an on-disk adapter
has drifted from what the engine would generate. Wire `render --check` as a CI
gate so vendor dirs can never silently grow back into always-loaded prose
harnesses (the token black hole).

This preserves the Rusty IDD workflow order: goal -> graph context -> OpenSpec
(spec delta + design + tasks) -> implementation after ready -> validation refresh.

## Intent

- `rusty-idd render [--vendor <name> | --all]` writes a deterministic, minimal
  adapter file into each vendor dir that points agents at `rusty-idd next`.
- `rusty-idd render --check` regenerates in memory and compares to disk; any
  missing or drifted adapter is a non-zero failure (the drift gate).
- The adapter content is generated from a single template baked into the engine
  (the source of truth); hand-edits are rejected by the gate.

## Decision Target

Rusty IDD SHALL own the vendor-adapter content and provide `render` (write) and
`render --check` (fail-closed drift gate). Vendor adapters SHALL be generated,
never hand-authored; CI SHALL run `render --check`.

## Non-Goals

- No rewrite of existing vendor hook logic in this slice (hook wiring is backlog
  4.2); the adapter is a thin pointer, added alongside existing files.
- No new vendor surfaces; render targets the existing known set.
- No new ADR (ADR-0010 + ADR-0015 already govern this).
