# harness-session-frontdoor

## Why

The harness front door is built but nothing invokes it. `rusty-idd next`
(ADR-0015) computes the single next-step imperative for the active change, and
the thin vendor adapters (`rusty-idd render`, ADR-0010) point agents at it and
are CI-enforced. But no vendor surface actually *calls* `rusty-idd next` at
session start — an agent only sees the imperative if it remembers to run the
command. That makes the determinism guarantee opt-in, which is exactly the
failure mode the control plane exists to remove (backlog item 4.2).

Separately, the ADR ledger has drifted: parallel changes each allocated the
same ADR number before committing, producing duplicate-numbered ADRs at 0002,
0004, 0005, and 0006 (backlog item 4.5). ADRs are immutable once accepted, so
the existing files are frozen; nothing detects the collisions or stops the next
one.

## What Changes

- Add a `SessionStart` hook to `.codex/hooks.json` that runs `rusty-idd next`,
  using the same `sh -lc 'root=...; cargo run ... --bin rusty-idd -- next'`
  shape as the existing workflow-check hooks.
- Create `.claude/settings.json` with a `hooks.SessionStart` entry that runs
  `rusty-idd next`, so Claude Code surfaces the imperative automatically.
- Add `rusty-idd spec adr list --check`: a fail-closed ADR-number collision
  detector that exits non-zero on any NEW duplicate beyond a frozen baseline of
  the four known historical collisions (mirroring the `render --check` and
  cargo-audit baseline idioms). Wire it into CI so the bug cannot recur.
- Author ADR-0016 recording the historical collisions as frozen and
  establishing slug-canonical ADR referencing going forward.

## Capabilities

### New Capabilities
- `harness-session-frontdoor`: vendor `SessionStart` hooks invoke `rusty-idd
  next` automatically, and the ADR ledger is collision-checked in CI against a
  frozen baseline.

## Impact

- `.codex/hooks.json` gains a `SessionStart` entry; new `.claude/settings.json`.
- `crates/cli/src/commands/spec_adr.rs` (or equivalent ADR command module) gains
  a `--check` collision detector + frozen baseline; `tests/` gains coverage.
- `.github/workflows/ci.yml` gains an ADR-collision gate step; `Justfile` gains
  an `adr-check` recipe.
- New `adr/0016-adr-ledger-reconciliation.md`. No removals; no new dependencies.
