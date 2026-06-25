# Harness Session Front Door + ADR Ledger Hygiene Goal

rusty-idd --goal-file .idd/goals/harness-session-frontdoor.md

The front door is built but nothing invokes it. `rusty-idd next` (ADR-0015) and
the thin vendor adapters (`render`, ADR-0010) exist and are CI-enforced, yet no
vendor surface actually *calls* `rusty-idd next` at session start — agents must
remember to run it. Close that gap: wire the `SessionStart` hook of each vendor
surface that supports one to invoke `rusty-idd next`, so the computed
next-step imperative is presented automatically at the start of every session
(backlog item 4.2 of harness-control-plane).

While here, reconcile the ADR ledger: parallel changes each allocated the same
ADR number before committing, producing duplicate-numbered ADRs at 0002, 0004,
0005, and 0006 (backlog item 4.5). ADRs are immutable once accepted, so the
existing files are frozen as historical artifacts; reconciliation is a new ADR
plus an engine collision detector that prevents recurrence.

This preserves the Rusty IDD workflow order: goal -> graph context -> OpenSpec
(spec delta + design + tasks) -> ADR -> implementation after ready -> validation
refresh.

## Intent

- Add a `SessionStart` hook to `.codex/hooks.json` that runs `rusty-idd next`
  (same `sh -lc 'root=...; cargo run ... --bin rusty-idd -- next --base "$root"'`
  shape as the existing workflow-check hooks).
- Create `.claude/settings.json` with a `hooks.SessionStart` entry that runs
  `rusty-idd next`, so Claude Code surfaces the imperative on session start.
- Add a fail-closed ADR-number collision detector (`rusty-idd spec adr list
  --check`, mirroring `render --check`) that exits non-zero on any NEW duplicate
  number beyond a frozen baseline of the four known historical collisions; wire
  it into CI so the bug cannot recur.
- Author ADR-0016 recording the historical collisions as frozen and establishing
  slug-canonical ADR referencing going forward.

## Decision Target

Vendor surfaces SHALL invoke `rusty-idd next` automatically at session start via
their native `SessionStart` hook, not by relying on the agent to remember. The
ADR ledger SHALL be collision-checked in CI against a frozen baseline of the four
accepted historical duplicates; new collisions SHALL fail closed.

## Non-Goals

- No change to `rusty-idd next` behavior or output (it already exists, ADR-0015).
- No renumbering or editing of the existing duplicate-numbered ADRs (immutable;
  reconciled by ADR + slug-canonical rule, not by mutation).
- No additional stage packages (impl/validation/handoff swarms) — that is backlog
  4.3 (the separate add-verify-package-stage change), out of scope here.
- No new vendor surfaces; `.agents`/`.devin` have no standard SessionStart hook
  mechanism and keep their existing thin adapter only.
