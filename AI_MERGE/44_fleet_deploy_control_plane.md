# 44 — Fleet-deploy control plane (`rusty-idd deploy`, ADR-0017)

Evidence note for `feat/fleet-deploy-rusty-idd` — Phase 1 of the fleet-deploy
goal. Adds the deploy front door that installs the thin-adapter control-plane
surface into a *target* fleet repo, so the whole fleet presents one minimal agent
harness backed by the one engine. This is **Option-3 first**: build + deploy the
mechanism; no repo adoption or retirement in this slice.

## What landed

- **`rusty-idd deploy --target <repo> [--vendor <name> | --all] [--check|--dry-run]`**
  (`crates/cli/src/commands/deploy.rs`). Writes each targeted vendor's
  `rusty-idd-adapter.md` (reusing `render::expected_adapter` + the shared
  `VENDORS` set — one source of truth, byte-identical to `render`) plus a
  SessionStart hook calling the front door.
- **Deployed hook uses the installed `rusty-idd` on PATH**, resolving the repo
  root at runtime (`sh -lc 'root="$(git rev-parse --show-toplevel)"; exec
  rusty-idd next --base "$root"'`) — a peer repo is not the rusty-idd cargo
  workspace, so it cannot `cargo run` it. Hook-capable vendors: `codex`
  (`.codex/hooks.json`), `claude` (`.claude/settings.json`); `agents`/`devin`
  get the adapter doc only.
- **Additive + idempotent.** Hook merge appends the SessionStart entry iff a
  front-door hook is absent (semantic match on the shared `next --base` marker,
  so the home repo's `cargo run … next` hook also counts and is never
  duplicated), preserving every other key (`$comment`) and hook phase. Never
  modifies/deletes the target's forge loop, runtime, build, source, or generated
  artifacts.
- **Fail-closed `--check`/`--dry-run` drift gate.** Adapter drift = missing or
  byte-different; hook drift = canonical front-door entry absent (semantic, so
  JSON key ordering never causes false drift). Exits non-zero on drift, writes
  nothing; clean target exits zero.
- **Enforcement:** `deploy-check` Justfile recipe + a CI step
  (`deploy --target . --all --check`) alongside render/adr gates; wired into the
  `just ci` aggregate. ADR-0017 records the decision.

## Rusty IDD flow (dogfooded)

Goal `.idd/goals/fleet-deploy-control-plane.md` → `knowledge plan-context` bind →
OpenSpec change `fleet-deploy-control-plane` (proposal/spec/design/tasks) +
ADR-0017 → `rusty-idd next` drove the artifact DAG to 5/5 → implement → validate.

## Verification evidence

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no issues.
- `cargo test --workspace --locked` — 686 passed, 3 ignored (+17 deploy tests:
  10 integration in `deploy_cli.rs`, 7 unit).
- `rusty-idd spec validate --all` — 139/139 (new `fleet-deploy` delta VALID).
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- `render --all --check`, `spec adr list --check` (4 frozen baseline dups),
  `deploy --target . --all --check` (3 vendors in sync) — all green.
- knowledge + manifest refreshed, self-stable (3556 entries), **0 contamination**
  (clean worktree regen also removed 35,998 stale sibling-`.worktrees` refs that
  had accumulated in the committed `index.json`).

## Scope boundary (what is NOT in this slice)

Adopting the handoff + prompt_hub runtimes into rusty-idd, deploying the full
package across the live fleet, reorganizing, and retiring the standalone repos
are the sequenced follow-on phases (2–6). This slice delivers and self-gates the
deploy mechanism only.
