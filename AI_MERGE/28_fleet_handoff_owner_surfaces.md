# Fleet Handoff Owner Surfaces

Branch: `integration/fleet-handoff-owner-surfaces`
OpenSpec change: `openspec/changes/integrate-fleet-handoff`

## Purpose

The fleet-handoff integration item spans 13 owner repos. Before this change,
Rusty IDD could list the owners in the integration plan, but it did not provide
a deterministic handoff artifact that joined the selected work item to current
peer repo state, evidence paths, and native diagnostic commands.

This slice adds the missing read-only automation boundary before any
cross-repo consolidation work.

## Changed

- Added `rusty-idd knowledge integration-owners`.
- The command selects exactly one integration work item by `--change`,
  `--capability`, or `--work-item`.
- The command consumes:
  - `.idd/knowledge/integration-plan.json`
  - `.idd/knowledge/system-architecture.json`
- The report emits:
  - selected `IntegrationWorkItem`
  - owner repo join state
  - branch/head/dirty evidence
  - repo markers and roles
  - local architecture summaries when present
  - evidence paths
  - native diagnostic command candidates
  - missing owner repo findings
- Added durable fleet-handoff artifacts:
  - `.idd/knowledge/integration-owners.json`
  - `.idd/knowledge/integration-owners.md`
- Added `integration-owners` and `integration-owners-check` to both `Justfile`
  and `Makefile`.
- Added `integration-owners-check` to `just ci` and `make ci`.
- Updated `.agents/skills/rusty-idd-knowledge/SKILL.md` so future runs generate
  owner surfaces between integration-status queue selection and implementation.
- Raised the internal knowledge report and architecture pack ceilings to
  160,000 tokens after regeneration exposed a 123,510-token pack. The public
  `knowledge pack` default remains 120,000 tokens.

## Fleet Handoff Evidence

Generated command:

```bash
cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --change integrate-fleet-handoff --out .idd/knowledge/integration-owners.json
cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --change integrate-fleet-handoff --out .idd/knowledge/integration-owners.md
```

Result:

- Owner repos resolved: 13 / 13.
- Missing owner repos: 0.
- Owner repos reporting dirty state from the existing system graph: 3
  (`repo:envctl`, `repo:handoff`, `repo:prompt-hub`).
- `repo:rusty-idd` publishes parsed local architecture detail.
- Native diagnostic candidates were generated for Rust, Node, Make, Just, and
  git state surfaces where markers exist.

## Boundary

- Read-only command.
- No peer repo mutation.
- No MCP/server/daemon/host-service start.
- No `crates/core` dependency changes.
- Peer repo diagnostics are candidates recorded for the next execution step;
  they are not executed by this command.
- Size/token policy remains explicit: durable knowledge graph/report generation
  has a larger internal budget, while ad hoc pack generation keeps the existing
  default.

## Focused Validation

- `cargo fmt --all -- --check`: passed.
- `cargo test -p rusty-idd-knowledge integration_owner_surfaces_join_work_item_to_system_repos --locked`: passed.
- `cargo test -p rusty-idd-cli --test knowledge_cli system_architecture_cli_discovers_peer_repos_without_meta --locked`: passed.
- `just integration-owners-check`: passed.
- `make integration-owners-check`: passed.
- `cargo run --bin rusty-idd -- knowledge integration-owners --help`: passed.
- `cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --change integrate-fleet-handoff --out /tmp/rusty-idd-fleet-owner-smoke.json`: passed.
- `cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --capability capability:fleet-handoff --out /tmp/rusty-idd-fleet-owner-smoke.md`: passed.

## Full Validation

- `cargo test --workspace --all-features --locked`: passed, 626 passed and 3 ignored.
- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`: passed.
- `cargo audit --deny warnings`: passed.
- `cargo run --bin rusty-idd -- validate --workspace .`: passed after sequential knowledge refresh; 0 critical and 0 warning findings.
- `cargo run --bin rusty-idd -- spec validate --all`: passed, 51 items passed and 0 failed.
- `just ci`: passed with `integration-owners-check` included.
- `make ci`: passed with `integration-owners-check` included.
- `cargo run --bin rusty-idd -- spec archive openspec/changes/integrate-fleet-handoff --yes`: passed.
- Post-archive queue state: `integrate-fleet-handoff` archived, archived count is 2, and next planned item is `integrate-agent-communication`.

## Validation Notes

- The first `rusty-idd validate` run was executed concurrently with other gates
  and reported stale `.idd/knowledge` artifacts. A sequential `just knowledge`
  followed by `cargo run --bin rusty-idd -- validate --workspace .` passed with
  no findings.
- The first `just ci` run failed at `plan-context-check` because plan-context
  was stale after the latest source/control-plane edits. Regenerating
  plan-context and manifest fixed it; the next `just ci` run passed.

## Rollback

1. Revert `knowledge integration-owners` DTOs, builder, CLI command, and tests.
2. Remove `.idd/knowledge/integration-owners.json` and `.md`.
3. Remove `integration-owners` targets and checks from `Justfile` and
   `Makefile`.
4. Re-run knowledge, integration-plan, integration-status, plan-context, and
   manifest generation.
5. Re-run focused tests plus full gates.
