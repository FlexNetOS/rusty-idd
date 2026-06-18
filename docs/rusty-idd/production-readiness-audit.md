# rusty-idd Production Readiness Audit

Date: 2026-06-18

Scope: current `develop` worktree, current local files, and the visible `origin/develop` delta. The local `.git` directory is read-only in this sandbox, so the branch could not be fast-forwarded, but the remote delta was inspected.

## Executive Summary

rusty-idd is close to a production-ready unified CLI, but production readiness is not yet proven by the current control plane. The strongest functional gap found during this audit was the TUI runner data layer still depending on the external Node/OpenSpec CLI for active change listing and status. That gap has been fixed in this pass: `crates/runner/src/data.rs` now reads the local OpenSpec file layout directly.

Follow-up control-plane hardening addressed the highest-risk release and audit-baseline gaps:

- Promotion CI now runs Gemini and Claude drift checks as separate named steps.
- The legacy duplicate `idd-ci.yml` workflow was removed and the generator now emits `ci.yml`.
- CI runs on `main` and `develop`, and enforces `rusty-idd validate` plus manifest diff checks.
- `.idd/MANIFEST.tsv` excludes generated backups, local workspaces, editor state, and worktrees.
- `rusty-idd validate` now catches duplicate workflow keys, stale workflow policy, and polluted manifests.
- Release governance files now exist for semantic PR titles, release-please, Renovate/Dependabot, CODEOWNERS, local hooks, and release assets.

The remaining proof item is branch-level: verify the local `develop`, `origin/develop`, and `main` promotion state in a writable git checkout and clean stale worktrees only after proving their commits are reachable.

## Architecture Diagram

```text
              +--------------------+
              |    rusty-idd CLI   |
              |  crates/cli        |
              +---------+----------+
                        |
        +---------------+----------------+
        |               |                |
+-------v------+ +------v-------+ +------v-------+
| core         | | spec         | | runner       |
| scan/plan/   | | validate/    | | run/apply/   |
| validate/    | | archive/     | | task status  |
| manifest     | | scaffold     | | data layer   |
+-------+------+ +------+-------+ +------+-------+
        |               |                |
        |               |                v
        |               |        +-------+-------+
        |               |        | tui           |
        |               |        | ratatui app   |
        |               |        +---------------+
        |
        v
+-------+------------------------------------------------+
| Control plane: AGENTS.md, AI_MERGE, .idd/MANIFEST.tsv, |
| GitHub workflows, SECURITY.md, OpenSpec changes/specs  |
+--------------------------------------------------------+
```

## Runtime Data Diagram

Before this audit:

```text
TUI -> runner::data -> external openspec CLI -> Node/OpenSpec runtime
```

After this audit:

```text
TUI -> runner::data -> openspec/changes/
                    -> tasks.md checkbox counts
                    -> proposal.md / design.md / tasks.md / specs/**/*.md
```

This removes a production runtime dependency while preserving the existing `ChangeListOutput` and `ChangeStatusOutput` shapes consumed by the TUI.

## CI And Release Gate Diagram

```text
PR / push
  |
  v
.github/workflows/ci.yml
  |-- pinned Rust toolchain
  |-- drift check
  |-- cargo build --workspace --locked
  |-- cargo test --workspace --locked
  |-- cargo fmt --all -- --check
  |-- cargo clippy --workspace --all-targets --all-features -- -D warnings
  `-- cargo audit --deny warnings

develop -> main PR
  |
  v
.github/workflows/promote-verify.yml
  |-- clean merge into main
  |-- drift check     [GAP: duplicate run key]
  |-- build/test/fmt/clippy
  `-- cargo audit
```

## User Story For Gap Hunting

As the integration maintainer, I want a repeatable production-readiness audit that exercises rusty-idd as an installed, Rust-native control plane, so that any hidden Node dependency, stale CI path, weak validator, untracked control-plane drift, or host-specific test assumption is found before a promotion PR reaches `main`.

Acceptance criteria:

- The audit runs from a clean, up-to-date integration branch.
- The audit proves `rusty-idd tui` can list active changes without `openspec` on PATH.
- The audit fails when a workflow has duplicate YAML keys or weaker duplicate gates.
- The audit fails when generated `.idd-bak-*`, `_workspace` logs, or local worktrees enter the production manifest.
- The audit records build, test, lint, secret/supply-chain, migration, rollback, and manifest evidence in `AI_MERGE`.

## Gap Register

| Priority | Gap | Evidence | Production impact | Plan |
|---|---|---|---|---|
| P0 | Promotion workflow duplicate `run:` key | `.github/workflows/promote-verify.yml` drift-check step had two `run:` entries | YAML keeps only one command; intended drift gate can be skipped silently | Fixed: split Gemini and Claude drift checks into separate named steps; validator now flags duplicate workflow keys |
| P0 | Legacy CI workflow drift | `.github/workflows/idd-ci.yml` used `@stable` and weaker commands than `.github/workflows/ci.yml` | Conflicting CI signals and unpinned release behavior | Fixed: deleted `idd-ci.yml`; generator now emits pinned `.github/workflows/ci.yml` |
| P0 | External OpenSpec runtime dependency in TUI | `runner::data` spawned `openspec list/status` before this audit | Violated Rust-native/no-Node runtime goal | Fixed in this pass; add end-to-end no-`openspec` PATH test |
| P0 | Host-dependent runner test | `test_implementation_loop_writes_task_header` launched default `claude` when present | Tests could hang on developer/CI hosts with real agent binaries installed | Fixed in this pass with empty command fixture |
| P1 | Manifest baseline includes generated/local artifacts | `.idd/MANIFEST.tsv` included `.idd-bak-*`, `_workspace` logs, `.devin/config.local.json` | Audit baseline changes due local execution debris | Fixed: manifest walk excludes local execution artifacts; validator fails polluted manifests |
| P1 | Validator misses release-critical structure | `rusty-idd validate --workspace .` returned 0 warnings despite CI duplicate key and manifest pollution | False green local validation | Fixed: validator checks duplicate YAML keys, stale workflow policy, manifest hygiene, and required release governance files |
| P1 | Checkout freshness not enforceable here | `develop...origin/develop [behind 2]`; fast-forward blocked by read-only `.git` | Local audit can lag integration branch | Run final verification in a writable git checkout; include branch freshness in release checklist |
| P2 | Historical TUI OpenSpec records mention old CLI dependency | archived `crates/tui/openspec/changes/archive/**` docs still describe `openspec` subprocesses | Historical docs can confuse agents if not distinguished from live specs | Leave archive immutable; ensure live specs/config/README state current filesystem behavior |
| P2 | Supply-chain accepted-risk baseline needs periodic review | `cargo audit --deny warnings` passed in this audit with the current advisory DB | Future advisories can change the risk posture | Keep the fail-closed audit gate; update `security-advisories.md` only if output changes |

## Production Readiness Plan

### Phase 0: Seal Release Gates

1. Done: `promote-verify.yml` splits Gemini and Claude drift checks into separate named steps.
2. Done: obsolete `idd-ci.yml` was removed; generated workspaces now emit `.github/workflows/ci.yml`.
3. Done: `rusty-idd validate` detects duplicate workflow keys and stale workflow policy.

Exit evidence: `rusty-idd validate --workspace .` reports the duplicated-key fixture in a test and returns clean on the repo.

### Phase 1: Lock Rust-Native Runtime Behavior

1. Keep runner/TUI active change discovery filesystem-backed.
2. Add an integration test that clears `PATH` or shadows `openspec` and proves active list/status still works.
3. Keep Node/Bun only as a development oracle in `scripts/oracle-sync.sh`, not as a production runtime dependency.

Exit evidence: runner tests pass and `rg "Command::new(\"openspec\")|openspec_command" crates/runner/src crates/tui/src` returns no active code hits.

### Phase 2: Clean The Audit Baseline

1. Done: production manifest excludes `.idd-bak-*`, `_workspace`, `.devin`, `.worktrees`, `.vscode`, and the self-referential `.idd/MANIFEST.tsv`.
2. Done: tests cover transient artifact exclusion and idempotent manifest generation.
3. Done: `.idd/MANIFEST.tsv` was regenerated after the exclude policy was implemented.

Exit evidence: manifest contains source, docs, specs, CI, and control-plane files, but excludes transient local execution state.

### Phase 3: Complete Verification Evidence

1. Run `cargo build --workspace --locked`.
2. Run `cargo test --workspace --locked` after isolating process-spawn tests from host tools.
3. Run `cargo fmt --all -- --check`.
4. Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
5. Run `cargo audit --deny warnings`.
6. Run `rusty-idd validate --workspace .`.

Exit evidence: paste exact command results into the relevant `AI_MERGE` record.

### Phase 4: Release And Operations

1. Document install paths: `cargo install rusty-idd-cli`, source build, and local TUI launch.
2. Add a release checklist covering crates.io publish, GitHub release artifact, rollback path, and advisory baseline review.
3. Add a minimal smoke script that initializes a temp OpenSpec workspace, lists changes through the TUI data layer, validates specs, and runs manifest generation.

Exit evidence: release candidate can be installed and smoke-tested without Node/OpenSpec installed.

## Changes Made During This Audit

- Replaced `runner::data::list_changes` and `get_change_status` subprocess calls with filesystem-backed discovery.
- Added unit tests for active change discovery and artifact status.
- Completed the `retry_on_failure` config test coverage and loop wiring from the visible `origin/develop` fix that could not be fast-forwarded through `.git`.
- Made `test_implementation_loop_writes_task_header` deterministic by avoiding a real host `claude` launch.
- Updated live TUI specs, context, and README to match the no-external-OpenSpec runtime behavior.

## Verification So Far

```text
cargo build --workspace --locked
result: passed

cargo test --workspace --locked
result: passed

cargo fmt --all -- --check
result: passed

cargo clippy --workspace --all-targets --all-features -- -D warnings
result: passed

cargo audit --deny warnings
result: passed

cargo run --bin rusty-idd -- validate --workspace .
result: 0 critical, 0 warning

cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv
result: wrote 412 manifest entries

sha256sum .idd/MANIFEST.tsv before and after serial regeneration
result: unchanged during the final idempotence check

rg -n "idd-bak|_workspace/|\.devin/|\.worktrees/|\.vscode/|\.idd/MANIFEST.tsv|\.github/workflows/idd-ci\.yml" .idd/MANIFEST.tsv
result: no matches
```
