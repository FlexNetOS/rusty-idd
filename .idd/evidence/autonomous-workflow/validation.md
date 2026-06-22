# Handoff KB Refresh Validation Evidence

- Change: `refresh-handoff-kb-upstream`
- Mirror verification: source handoff tracked file count is 550, mirror file
  count is 550, file-list diff is clean, and `third_party/upstream/handoff/.git`
  is absent.
- Source dirty state: handoff working-tree edits are recorded in
  `AI_MERGE/38_handoff_kb_refresh/handoff-source-state.md` and excluded from the
  mirror because the mirror is built from committed HEAD.
- Build: passed `RUSTY_IDD_CHANGE=refresh-handoff-kb-upstream RUSTY_IDD_GOAL_FILE=.idd/goals/refresh-handoff-kb-upstream.md rtk just ci`, including `cargo build --workspace --locked`.
- Generated artifacts: `rtk just ci` passed freshness checks for `.idd/MANIFEST.tsv`, `.idd/knowledge/*`, `docs/rusty-idd/architecture-diagrams.md`, and goal-file plan context for `refresh-handoff-kb-upstream`.
- Test: passed `cargo test --workspace --locked` through the same `rtk just ci` gate.
- Lint: passed `cargo fmt --all -- --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` through the same `rtk just ci` gate.
- Audit: passed `cargo audit --deny warnings` through the same `rtk just ci` gate.
- Secret scan: changed-file scan for private key, AWS, Google API, GitHub, and OpenAI token patterns returned no matches.
- Manifest: refreshed `.idd/MANIFEST.tsv` after validation evidence updates and verified through the `rtk just ci` manifest freshness gate.

## Rollback Path

Revert ADR 0008, `openspec/changes/refresh-handoff-kb-upstream`,
`.idd/goals/refresh-handoff-kb-upstream.md`,
`AI_MERGE/38_handoff_kb_refresh/`, the refreshed handoff mirror and upstream
registry pin, and regenerated `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and
`docs/rusty-idd/architecture-diagrams.md`.
