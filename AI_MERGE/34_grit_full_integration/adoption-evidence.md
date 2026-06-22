# Grit Full Integration Adoption Evidence

OpenSpec change: `adopt-grit-full-integration`

Goal file: `.idd/goals/grit-full-integration.md`

Grit source checkout: `/home/drdave/Desktop/meta/grit`

Pinned Grit commit: `57b60842d71145c271b994bb7a8c33c3bca42dfe`

Local mirror: `third_party/upstream/grit`

## Scan Evidence

Commands run:

- `cargo run --bin rusty-idd -- knowledge plan-context --workspace . --out .idd/knowledge/plan-context.md --goal-file .idd/goals/grit-full-integration.md`
- `cargo run --bin rusty-idd -- knowledge plan-context --workspace . --out .idd/knowledge/plan-context.json --goal-file .idd/goals/grit-full-integration.md`
- `cargo run --bin rusty-idd -- scan --repo /home/drdave/Desktop/meta/grit --out AI_MERGE/34_grit_full_integration/00_grit_inventory.md --format md`
- `cargo run --bin rusty-idd -- scan --repo /home/drdave/Desktop/meta/grit --out AI_MERGE/34_grit_full_integration/00_grit_inventory.json --format json`
- `cargo run --bin rusty-idd -- scan --repo . --out AI_MERGE/34_grit_full_integration/01_rusty_idd_inventory_before_adoption.md --format md`
- `cargo run --bin rusty-idd -- scan --repo . --out AI_MERGE/34_grit_full_integration/01_rusty_idd_inventory_before_adoption.json --format json`
- `cargo run --bin rusty-idd -- plan --repo-a . --repo-b /home/drdave/Desktop/meta/grit --out AI_MERGE/34_grit_full_integration/plan-workspace --name grit-full-integration`

Grit scan result:

- Files scanned: 100
- Languages: Rust 21, TypeScript 14, Python 6, Shell 16
- Package managers: cargo
- Workflows:
  - `.github/workflows/ci.yml`
  - `.github/workflows/next-release.yml`
  - `.github/workflows/pr-target-check.yml`
  - `.github/workflows/release-please.yml`
  - `.github/workflows/release.yml`
- Agent control files: `AGENTS.md`

Plan artifacts:

- `plan-workspace/AI_MERGE/00_repo_a_inventory.{md,json}`
- `plan-workspace/AI_MERGE/01_repo_b_inventory.{md,json}`
- `plan-workspace/AI_MERGE/02_feature_matrix.md`
- `plan-workspace/AI_MERGE/03_env_and_secret_contracts.{md,json}`
- `plan-workspace/AI_MERGE/04_merge_plan.md`
- `plan-workspace/AI_MERGE/05_conflict_risk_register.md`
- `plan-workspace/AI_MERGE/06_gap_audit_and_applied_updates.md`
- `plan-workspace/AI_MERGE/07_tasks/*.md`
- `plan-workspace/AI_MERGE/08_agent_queue.md`
- `plan-workspace/AI_MERGE/09_github_execution.md`
- `plan-workspace/AI_MERGE/10_parity_test_plan.md`
- `plan-workspace/AI_MERGE/11_provider_matrix.md`
- `plan-workspace/.idd/MANIFEST.tsv`

## Import Evidence

Import command:

```bash
git -C /home/drdave/Desktop/meta/grit archive HEAD | tar -x -C third_party/upstream/grit
```

Mirror verification:

- Source tracked files: 100
- Mirrored files: 100
- Difference between source tracked files and mirror files: none
- Mirror size: 1.4M

Tracked dotfiles preserved:

- `.github/workflows/ci.yml`
- `.github/workflows/next-release.yml`
- `.github/workflows/pr-target-check.yml`
- `.github/workflows/release-please.yml`
- `.github/workflows/release.yml`
- `.gitignore`
- `.release-please-manifest.json`
- `.rtk/filters.toml`
- `scripts/.gitignore`

## Migration Note

Old path: Grit was only available as a peer checkout at
`/home/drdave/Desktop/meta/grit`.

New path: Rusty IDD also carries a pinned, as-is upstream reference at
`third_party/upstream/grit` for planning, graphing, diagnostics, and rollback
evidence.

No Grit source code was modified. The mirror is not a Cargo workspace member.

## Rusty IDD Tooling Evidence

The full adoption run exposed two deterministic-artifact limits in Rusty IDD:

- `knowledge refresh` failed at 315,658 packed-context tokens against the old
  160,000 generated-report ceiling after the scan/plan evidence and Grit mirror
  were present.
- `validate` reported stale knowledge immediately after diagram generation
  because `docs/rusty-idd/architecture-diagrams.md` participated in the
  workspace fingerprint even though it is generated from the architecture graph.

Fixes applied:

- Raised the internal generated-artifact pack ceiling to 400,000 tokens while
  leaving the default user-facing `knowledge pack --max-tokens` value unchanged.
- Excluded `docs/rusty-idd/architecture-diagrams.md` from the knowledge
  freshness fingerprint and added a focused unit-test fixture.

## Rollback Path

Remove `third_party/upstream/grit`, remove the Grit row and local-boundary note
from `third_party/upstream/UPSTREAMS.md`, and revert the
`adopt-grit-full-integration` OpenSpec, ADR, goal, evidence, regenerated
knowledge, diagram, validation, manifest artifacts, and Rusty IDD
generated-artifact fixes from this change.

## Validation Evidence

- Develop sync: branch fast-forwarded to `origin/develop` at `f4df9a4` before
  final regeneration.
- OpenSpec status: `adopt-grit-full-integration` is archivable, 5/5 artifacts
  complete.
- Build: `cargo build --workspace --locked` passed.
- Test: `cargo test --workspace --locked` passed, 621 passed and 3 ignored.
- Lint/typecheck: `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  passed.
- Audit: `cargo audit --deny warnings` passed.
- Format: `cargo fmt --all -- --check` passed.
- Diff whitespace: `git diff --check` passed.
- Rusty IDD validation: `cargo run --bin rusty-idd -- validate --workspace .`
  passed with 0 critical and 0 warning.
- Repo-local freshness: manifest, knowledge index/report/architecture,
  goal-file plan context, and architecture diagrams regenerated into a temp
  directory and matched checked-in artifacts.
- Secret scan: changed-file scan returned no matches.
- Conflict-marker scan: changed-file scan returned no matches.
- Mirror completeness: source Grit tracked files 100, mirrored files 100,
  `comm -3` difference empty.
