# Harness `next --json` Goal

rusty-idd --goal-file .idd/goals/harness-next-json.md

Give the harness control-plane front door (`rusty-idd next`, ADR-0015) a
machine-readable mode so non-interactive vendor adapters can consume the next
imperative as structured data instead of scraping human text. This is the brick
that the vendor-adapter `render` / drift-gate (backlog 4.1) and hook wiring (4.2)
depend on: an adapter calls `rusty-idd next --json` and acts on fields, not prose.

This preserves the Rusty IDD workflow order: goal -> graph context -> OpenSpec
(spec delta + design + tasks) -> implementation after ready -> validation refresh.

## Intent

Add `--json` to `rusty-idd next`, emitting a stable object: the active change,
its artifact-DAG status, the single next ready artifact, archivability, and the
one scoped command to run next. Reuse the existing `spec_status` snapshot so the
JSON cannot disagree with `spec status --json`.

## Decision Target

`rusty-idd next --json` SHALL print a single deterministic JSON object describing
the active change and the next action, and SHALL exit non-zero (with no stdout
JSON) when the active-change pointer is dangling, so adapters fail closed.

## Non-Goals

- No change to the default human output of `rusty-idd next`.
- No vendor rendering / drift gate in this slice (that is backlog 4.1).
- No new architectural decision (ADR-0015 already governs the front door).
