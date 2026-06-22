# Full Handoff Adoption Validation Evidence

- Change: `adopt-full-handoff-upstream`
- Build: passed `RUSTY_IDD_CHANGE=adopt-full-handoff-upstream RUSTY_IDD_GOAL_FILE=.idd/goals/adopt-full-handoff-upstream.md rtk just ci`, including `cargo build --workspace --locked`.
- Generated artifacts: refreshed and compared `.idd/knowledge/*`, `docs/rusty-idd/architecture-diagrams.md`, `.idd/MANIFEST.tsv`, OpenSpec artifacts, ADR, AI_MERGE evidence, task evidence, and goal-file-backed `.idd/knowledge/plan-context.{json,md}` before the successful test gate.
- Test: passed `cargo test --workspace --locked` through the successful `rtk just ci` gate after generated artifacts.
- Lint: passed `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` through the successful `rtk just ci` gate.
- Secret scan: changed-file scan for private key, AWS, Google API, GitHub, and OpenAI token patterns returned no matches.
- Manifest: refreshed `.idd/MANIFEST.tsv` and passed the `rtk just ci` manifest freshness comparison.
- Spec status: passed `rtk cargo run --quiet --bin rusty-idd -- spec status openspec/changes/adopt-full-handoff-upstream`; change is archivable.
- Spec validate: passed `rtk cargo run --quiet --bin rusty-idd -- spec validate --all` with 98 passed and 0 failed.
- Workflow stop check: passed `rtk cargo run --quiet --bin rusty-idd -- codex workflow-check --workspace . --phase stop --change adopt-full-handoff-upstream`.
- Mirror verification: source handoff tracked file count is 533, mirror file count is 533, file-list diff is clean, and `third_party/upstream/handoff/.git` is absent.
- Follow-up mirror correction: final trunk proof found the committed mirror had only 524 files because tracked handoff `.idea/*` files were ignored by Rusty IDD; the fix force-added the nine tracked `.idea` files and excluded source-local `.idea/workspace.xml`.

## Rollback Path

Revert ADR 0006, the `adopt-full-handoff-upstream` OpenSpec package, `.idd/goals/adopt-full-handoff-upstream.md`, `AI_MERGE/36_handoff_full_adoption/`, the `third_party/upstream/UPSTREAMS.md` handoff row, `third_party/upstream/handoff/`, and regenerated `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and `docs/rusty-idd/architecture-diagrams.md`.
