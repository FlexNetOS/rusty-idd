# self-upgrade-governor Validation Evidence

- Change: `add-self-upgrade-governor`
- Branch: `feature/self-upgrade-governor-goal`

## Generated Artifacts

- Passed: `.idd/knowledge/index.json`, `.idd/knowledge/report.md`,
  `.idd/knowledge/architecture.json`, and `.idd/knowledge/architecture.md`
  refreshed with the Rusty IDD binary at
  `/home/drdave/Desktop/meta/rusty-idd/.worktrees/ci-envctl-rust-toolchain/target/debug/rusty-idd`.
- Passed: `.idd/knowledge/plan-context.md` and
  `.idd/knowledge/plan-context.json` regenerated from
  `.idd/goals/add-self-upgrade-governor.md`.
- Passed: `.idd/MANIFEST.tsv` refreshed with 3485 entries.

## OpenSpec

- Passed: `rusty-idd spec status openspec/changes/add-self-upgrade-governor`
  reported 5/5 artifacts done and archivable.

## Build

- Not run locally in this shell: `rtk cargo` failed because `cargo` was not
  available, and `scripts/ci/envctl-rust-env.sh ci` failed because `rustup` was
  not available on `PATH` to materialize the meta-owned toolchain. No
  user-global or system-depth fallback was used.
- Artifact generation used the already-built Rusty IDD binary from the develop
  worktree listed above.

## Test

- Passed: `rusty-idd validate --workspace .` completed with `0 critical, 0
  warning`.

## Lint

- Passed: `git diff --check`.

## Secret Scan

- Passed: Rusty IDD validation reported `0 critical, 0 warning`; this change
  adds goal, OpenSpec, ADR, evidence, knowledge, and manifest artifacts only.

## Manifest

- Passed: `.idd/MANIFEST.tsv` refreshed.

## Rollback

Revert the goal file, OpenSpec change directory, ADR, evidence directory,
refreshed generated artifacts, and manifest changes from this branch.
