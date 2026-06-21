# Agent Communication Queue Owner Surfaces

Branch: `integration/agent-communication-next-owner-surfaces`
OpenSpec change: `openspec/changes/integrate-agent-communication`

## Purpose

The fleet-handoff owner-surface report proved the value of joining integration
work items to current peer repo state, but the durable
`.idd/knowledge/integration-owners.*` target was pinned to one archived change.
That would make the automation stale after each archive.

This slice makes owner-surface generation queue-aware for the active
agent-communication integration work.

## Changed

- Added `rusty-idd knowledge integration-owners --next`.
  - Selects the highest-priority non-archived integration work item.
  - Keeps the durable owner report on an active/scaffolded integration until it
    is archived.
- Added `rusty-idd knowledge integration-owners --next-planned`.
  - Selects the highest-priority work item that is still planned.
  - Useful when automation must deliberately skip active/scaffolded changes.
- Updated `Justfile` and `Makefile` so `integration-owners` and
  `integration-owners-check` use `--next`.
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md` to document `--next`
  as the durable artifact selector.
- Scaffolded `openspec/changes/integrate-agent-communication`.
- Regenerated `.idd/knowledge/integration-owners.json` and `.md` for
  `integrate-agent-communication`.
- Archived `integrate-agent-communication` into
  `openspec/specs/agent-communication/spec.md`.

## Agent Communication Evidence

Generated command:

```bash
cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --next --out .idd/knowledge/integration-owners.json
cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --next --out .idd/knowledge/integration-owners.md
```

Result:

- Selected change: `integrate-agent-communication`.
- Owner repos resolved: 4 / 4.
- Missing owner repos: 0.
- Owner repos:
  - `repo:atc`
  - `repo:handoff`
  - `repo:mcp-hub`
  - `repo:weave`
- Read-only peer state evidence:
  - `atc`: `main`, clean, head `831d5943b0690f77b6306d99c6d09919f5a06a88`
  - `handoff`: `feat/hftask-0058-durability-policy`, dirty, head
    `83de633516493ea7b532dfd3593c9e18327b84fe`
  - `mcp_hub`: `master`, clean, head
    `4b4d3fe815a7795700e1acaa1ff34a19c7165764`
  - `weave`: `wl056-xmachine-push`, dirty (`weave-core/src/store.rs`), head
    `5e5c34e4d83da1f3e4649720fbad0f2b8f988b7c`

## Boundary

- Read-only command.
- No peer repo mutation.
- No MCP/server/daemon/host-service start.
- No `crates/core` dependency changes.
- Peer repo diagnostics are candidates recorded for the next execution step;
  they are not executed by this command.

## Focused Validation

- `cargo fmt --all -- --check`: passed.
- `cargo test -p rusty-idd-knowledge integration_owner_surfaces_join_work_item_to_system_repos --locked`: passed.
- `cargo test -p rusty-idd-cli --test knowledge_cli system_architecture_cli_discovers_peer_repos_without_meta --locked`: passed.
- `just integration-owners`: passed.
- `just integration-owners-check`: passed.
- `make integration-owners-check`: passed.
- `cargo run --bin rusty-idd -- knowledge integration-owners --help`: passed.
- `cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --next --out /tmp/rusty-idd-owner-next-smoke.json`: passed.
- `cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --next-planned --out /tmp/rusty-idd-owner-next-planned-smoke.json`: passed.

## Full Validation

- `cargo test --workspace --all-features --locked`: passed, 626 passed and 3 ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: passed.
- `cargo audit --deny warnings`: passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed after sequential knowledge refresh; 0 critical and 0 warning findings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed, 52 items passed and 0 failed before archive.
- `just ci`: passed.
- `make ci`: passed.
- `cargo run --bin rusty-idd -- spec archive openspec/changes/integrate-agent-communication --yes`: passed.

## Post-Archive State

- `integrate-agent-communication` is archived.
- Archived work item count is 3.
- Next planned item is `integrate-env-vault-relay`.
- `integration-owners --next` now advances to `integrate-env-vault-relay`.
- Post-archive validation passed:
  - `cargo test --workspace --all-features --locked`
  - `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
  - `cargo audit --deny warnings`
  - `cargo run --bin rusty-idd -- validate --workspace .`
  - `cargo run --bin rusty-idd -- spec validate --all` (49 items passed, 0 failed)
  - `just ci`
  - `make ci`

## Rollback

1. Revert `--next` and `--next-planned` selector plumbing.
2. Restore Make/Just `integration-owners` targets to their previous explicit
   selector.
3. Remove or revert `openspec/changes/integrate-agent-communication`.
4. Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
5. Re-run focused tests plus full Rusty IDD gates.
