# Autonomous Workflow Hooks Evidence

## Change

- OpenSpec change: `openspec/changes/add-autonomous-workflow-hooks`
- ADR: `adr/0002-autonomous-workflow-hooks.md`
- Branch: `feature/autonomous-workflow-hooks-v2`
- Base: `develop`
- Task card: `KBTASK-RUSTY-IDD-AUTONOMOUS-WORKFLOW-HOOKS`

## Evidence Summary

- Build: passed through `rtk just ci`.
- Test: passed through `rtk just ci`; focused Codex workflow tests passed.
- Lint/typecheck: `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed through `rtk just ci`.
- Format: `rtk cargo fmt --all --check` passed.
- Secret scan: changed-file secret-pattern scan returned no matches.
- Supply-chain audit: `rtk cargo audit --deny warnings` passed after sandbox escalation for the advisory DB lock.
- Manifest: `.idd/MANIFEST.tsv` refreshed after knowledge artifacts.
- Migration: Stop-only invariant hook upgraded to pre/post/stop workflow gates plus the existing Stop invariant.
- Rollback: revert this PR to restore the prior Stop-only `codex env-check` hook behavior.

## PR Handoff

Pending until the feature branch is pushed, a PR is opened against `develop`,
and auto-merge is enabled.
