# HFTASK-0071 — Verify (runtime evidence)

## Evidence table
| Gate | Command | Result |
|------|---------|--------|
| Build | `cargo build -p hf` | green |
| Clippy (CI-exact) | `cargo clippy --workspace --all-targets -- -D warnings` | No issues found |
| Format | `cargo fmt --all --check` | clean |
| Workspace tests | `cargo test --workspace` | **677 passed, 0 failed** (28 suites, 141.8s) |
| Unit test | `cargo test -p hf direction_block` | 1 passed |
| Checkpoint | `hf checkpoint HFTASK-0071 "..."` | witnessed (≥1) |
| hf test | `hf test HFTASK-0071` | PASS — count-verified (see below) |
| Drift | `hf drift` | clean after test_result recorded |
| Live drive | `hf resume` | Next Action block renders (pasted below) |

## hf test HFTASK-0071 (count-verified)
```
hf test: HFTASK-0071 -> PASS (1 command(s) green, 677 test(s) executed, witnessed)
```
POSITIVE executed count (677), not exit-0-only — the fail-closed tests-ran>0 gate is satisfied.

## hf drift (post-test)
```
hf drift: clean — no intent, scope, evidence, or dependency drift
```

## Rendered Next Action block (live `hf resume`)
```
## 0. Next Action / Direction
- **Next safe task:** HFTASK-0071 — ADR-0018 D4: more direction from handoff (next-action in hf resume + packet)
- **Next command:** `hf checkpoint HFTASK-0071`
- **Why it is next:** resume the in-progress task (status Claimed) before starting any new work.
- **Cycle / context budget:** context — wrap at ~50% of the context window (cycle_flush=4 caps a runaway cycle); this session is at cycle 0/4.
- **Ready to ship:** no (`hf ship` once the cycle is full / context budget hit).
- **Blocking walls:** none.
```
(HFTASK-0071 is itself Claimed, so the block correctly steers to resume-via-checkpoint. For a
fresh backlog card the renderer emits `hf claim <ID>` + a deps-satisfied rationale, asserted by
the `direction_block_emits_next_action_command_budget_and_walls` unit test.)

## Scope
- Diff: only `hf/src/main.rs` (+247). `Cargo.lock` skew reverted, never staged.
