# HFTASK-0071 — Implementation (ADR-0018 D4: more direction from handoff)

## Objective (locked, byte-exact)
`hf resume` + the rendered packet emit EXPLICIT next-action direction — the single next safe
task, the exact next command, the cycle/context-budget state, and the blocking walls — so a
fresh agent needs zero archaeology.

## Files changed
- `hf/src/main.rs` (+247 lines) — the ONLY changed file. Diff is exactly within `handoff/**`.

### What changed in `hf/src/main.rs`
1. **New pure fn `direction_block(tasks, replay, next, policy, sess) -> String`** (after
   `machine_summary`). Renders the `## 0. Next Action / Direction` block, every field derived
   from the witnessed ledger / policy / session counter — never hardcoded:
   - **Next safe task** — reuses the existing `next_safe()` result (passed in), ID + title.
   - **Exact next command** — `hf claim <ID>` for a backlog card, `hf checkpoint <ID>` for an
     in-progress (Claimed/Checkpointed/Active/Review) card (same precedence `next_safe` uses).
   - **Decision rationale** — WHY it is next: resume-in-progress, OR "deps satisfied (…),
     priority Pn" for a fresh backlog card.
   - **Cycle / context budget** — reads `policy.loop_cfg.{wrap_strategy, context_budget_pct,
     cycle_flush}` (`policy::Policy::load`) and the live `session::LoopSessionState.cycle`;
     emits the context-wrap rule (`~N%`) or the legacy `tasks` fixed-count rule, plus
     `cycle X/flush` and a `Ready to ship` line.
   - **Blocking walls** — any card in `Status::Blocked`, carrying `blocked_by` ids, or whose
     objective contains `NEEDS-HUMAN`, with reasons; "none" when clear.
2. **Wired into `render_packet_md`** — the block is pushed right after §3 Progress, before §4
   Remaining. Because `render_packet_md` is the SINGLE renderer shared by `hf handoff` (writes
   `packets/latest.md`) and `hf resume` Full (renders LIVE), the block appears identically in
   both — no parallel renderer.
3. **`cmd_seed` tight `test_commands` arm for `HFTASK-0071`** (additive; objective text
   untouched / byte-identical — `test_commands` is NOT part of the intent_lock):
   ```
   cargo test -p hf direction_block
   ./target/debug/hf resume | grep -q 'Next Action / Direction'
   ./target/debug/hf resume | grep -q 'Next command:'
   ./target/debug/hf resume | grep -q 'Cycle / context budget:'
   ```
4. **One unit test** `direction_block_emits_next_action_command_budget_and_walls` — asserts
   the next-action, the exact claim vs checkpoint command, the deps-satisfied rationale, the
   context-budget rule (+ live cycle), the `tasks`-strategy rule, and the wall reasons
   (Blocked + NEEDS-HUMAN), plus the "none" walls case.

## Scope confirmation
- `git status --short` ⇒ only `M hf/src/main.rs`.
- `Cargo.lock` churn (ruvector-domain-expansion 2.3.0→2.2.3 skew) reverted via
  `git checkout -- Cargo.lock` per scope discipline — never staged.
- No new dependency. No file outside `handoff/**`.

## Not done (leader's call)
- No commit / push / PR. Stopped after checkpoint + test + drift.
