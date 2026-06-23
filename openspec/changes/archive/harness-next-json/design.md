# harness-next-json — Design

## Context

`rusty-idd next` (ADR-0015) is the front door but prints only human text. The
upcoming vendor-adapter render/drift gate (backlog 4.1) and hook wiring (4.2)
need the front door to be machine-consumable: an adapter runs `rusty-idd next
--json` and branches on fields.

## Goals / Non-Goals

**Goals:**
- A `--json` flag on `rusty-idd next` emitting one deterministic object.
- Reuse the existing `spec_status` artifact-DAG snapshot so `next --json` and
  `spec status --json` are the same data (one oracle).
- Fail closed on a dangling pointer (non-zero, no stdout JSON).

**Non-Goals:**
- No change to default human output.
- No vendor rendering / drift gate (backlog 4.1).
- No new ADR (ADR-0015 governs the front door).

## Decisions

1. **Promote the snapshot, don't duplicate it.** `spec_status` already builds a
   `StatusSnapshot` for `spec status --json`. Expose a reusable constructor and
   have `next` wrap it with the front-door fields (`active_change`,
   `next_command`). `next --json` serialises that wrapper.
2. **Determinism.** Field order is fixed by the struct; no timestamps or
   absolute paths that vary by machine. Repeated runs over an unchanged tree are
   byte-identical (the spec requires it; a test asserts it).
3. **Fail closed.** Dangling pointer → stderr message, exit 1, no stdout JSON —
   so an adapter parsing stdout gets nothing and must stop.

## Risks / Trade-offs

- Coupling `next` to `spec_status`'s snapshot shape: acceptable and desired —
  one oracle is the whole point; a shared type makes drift impossible.

## Migration Plan

- Additive flag; default behaviour unchanged. Rollback = remove the flag.

## Open Questions

- None blocking. The JSON shape may gain a `schema` version field when the first
  external adapter consumes it (backlog 4.2).
