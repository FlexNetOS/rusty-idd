# fleet-deploy-control-plane — Tasks

## 1. Implementation

- [x] 1.1 Expose `render::expected_adapter` + the `VENDORS` table as `pub(crate)` (single source of truth)
- [x] 1.2 `commands::deploy` planner: pure fn computing desired surface per vendor (adapter bytes + canonical SessionStart hook entry)
- [x] 1.3 Canonical deploy hook command `const` (PATH binary, runtime root resolution: `rusty-idd next --base "$root"`)
- [x] 1.4 Hook merge for codex (`.codex/hooks.json`) + claude (`.claude/settings.json`): append SessionStart entry iff absent, preserve all other keys/phases; create minimal file if missing
- [x] 1.5 `rusty-idd deploy --target <path> [--vendor <name> | --all] [--check|--dry-run]` write + check modes
- [x] 1.6 Additive guarantee: only adapter docs + hook entry written; never delete/modify target runtime/build/source
- [x] 1.7 Wire into CLI enum / dispatch / module tree / lib docs

## 2. Tests

- [x] 2.1 deploy into a temp target writes adapter (byte-identical to `render`) + SessionStart hook
- [x] 2.2 `--all` deploys only existing vendor dirs; does not create absent ones
- [x] 2.3 hook merge preserves pre-existing PreToolUse/Stop entries and `$comment`
- [x] 2.4 idempotent: second deploy is byte-identical, reports no changes
- [x] 2.5 `--check` passes on in-sync target (exit 0, no writes)
- [x] 2.6 `--check` fails closed on missing/drifted adapter and on absent hook entry (exit 1, names it)
- [x] 2.7 additive: a sentinel runtime file in the target is untouched after deploy

## 3. Enforcement / Rollout

- [x] 3.1 Deploy to home repo: adapters match `render` (no drift), hook present — `deploy --target . --all --check` exits 0
- [x] 3.2 Prove deploy against a peer-shaped target — covered by temp-peer integration tests (byte-identical adapter, additive runtime untouched); live fleet deploy to handoff is Phase 4
- [x] 3.3 Add a `deploy-check` Justfile recipe (home repo self-check) + CI step (ADR-0017)

## 4. Verification gates

- [x] 4.1 `cargo test --workspace --locked` (686 passed, +17 deploy); `fmt --check`; `clippy --workspace --all-targets --all-features -D warnings` clean
- [x] 4.2 `spec validate --all` 139/139; `validate --workspace .` 0 critical / 0 warning
- [x] 4.3 `render --all --check` + `spec adr list --check` + `deploy --target . --all --check` green
- [x] 4.4 refresh `.idd/knowledge/*` + `MANIFEST.tsv` (refresh-last → validate → manifest); 0 contamination (clean regen also dropped 35,998 sibling-worktree refs from index.json)
- [x] 4.5 AI_MERGE evidence note (`AI_MERGE/44_fleet_deploy_control_plane.md`)
