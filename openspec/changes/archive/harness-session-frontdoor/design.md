# Design — harness-session-frontdoor

## Context

The control-plane arc landed three pieces: `rusty-idd next` (the imperative,
ADR-0015), `next --json` (machine-readable), and `render` + `--check` (thin
adapters enforced, ADR-0010). What is missing is *invocation*: the adapters
point at `rusty-idd next`, but no surface runs it for the agent. The
determinism loop only closes when session start triggers the front door.

A second, smaller debt: the ADR ledger has four duplicate-numbered pairs. The
allocator (`spec adr next`) returns `max+1`, so it never *re-issues* a used
number — but two changes that both read the next number before either committed
both got the same value. The fix is detection, not a smarter allocator.

## Decisions

### Session-start wiring uses each vendor's native hook, not a wrapper

- **Codex**: append a `SessionStart` array to the existing `.codex/hooks.json`
  alongside `PreToolUse`/`PostToolUse`/`Stop`/`SubagentStop`. Same command shape
  as the other hooks: `sh -lc 'root="$(git rev-parse --show-toplevel)"; exec
  cargo run --quiet --manifest-path "$root/Cargo.toml" --bin rusty-idd -- next
  --base "$root"'`. `next` takes `--base` (not `--workspace`); using `--base`
  keeps the invocation correct regardless of the session's CWD.
- **Claude Code**: create `.claude/settings.json` with
  `hooks.SessionStart[].hooks[]` of `type: command` running the same `rusty-idd
  next` invocation. The third-party handoff `.claude/settings.json` is the
  reference shape for the JSON structure.
- **`.agents` / `.devin`**: no standard session-start hook mechanism exists in
  their current configs, so they keep their thin adapter only (the adapter
  already instructs running `rusty-idd next`). Adopting a hook there is a future
  change if/when those surfaces grow one.

Rationale: the hooks are *data*, not engine logic — they only shell out to the
engine. This keeps vendor dirs thin (the `render --check` gate still governs the
adapter markdown; the hook config is the one piece of vendor-native wiring that
cannot be a generated markdown file).

### ADR collision detection mirrors the cargo-audit baseline pattern

- Add a `--check` flag to `spec adr list`. It groups ADRs by number; any number
  with >1 ADR is a collision.
- A `const ACCEPTED_DUPLICATE_ADRS: &[u32] = &[2, 4, 5, 6];` frozen baseline
  encodes the four immutable historical collisions. A collision at a baseline
  number is reported but does not fail; a collision at any other number fails
  closed (exit 1). This is exactly the `.cargo/audit.toml` philosophy already in
  this repo: known-accepted exceptions are frozen, anything new fails.
- Default `spec adr list` behavior (no `--check`) is unchanged.

### ADR-0016 reconciles, it does not renumber

ADRs are immutable once accepted (supersede, don't edit). Renumbering a colliding
ADR would change its identity and cascade through every `ADR-000N` citation.
Instead, ADR-0016:
- records the four collisions as frozen historical artifacts;
- establishes that ADRs are canonically referenced by **slug** (filename), not by
  bare number;
- points at the new `--check` gate as the recurrence guard.

## Risks / Trade-offs

- A `SessionStart` hook that runs `cargo run` adds first-session latency while the
  binary builds. Mitigated by `--quiet` and the workspace target cache; the
  existing workflow-check hooks already pay this cost, so it is not new.
- Creating `.claude/settings.json` in this repo is additive and repo-local; it
  does not affect the user's global Claude config and only fires on *new*
  sessions.

## Migration

None. Additive config + one new CLI flag + one new ADR.
