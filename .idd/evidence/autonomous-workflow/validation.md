# Envctl-Owned Rust Toolchain Validation Evidence

- Change: `envctl-owned-rust-toolchain-cache`
- Branch: `feature/envctl-owned-rust-toolchain-cache`
- Build: passed `cargo check --workspace --locked`.
- Generated artifacts: refreshed `.idd/knowledge/index.json`,
  `.idd/knowledge/report.md`, `.idd/knowledge/architecture.json`,
  `.idd/knowledge/architecture.md`, `.idd/knowledge/plan-context.json`,
  `.idd/knowledge/plan-context.md`, `.idd/MANIFEST.tsv`,
  `docs/rusty-idd/architecture-diagrams.md`, and
  `AI_MERGE/validation_report.md` before final tests.
- Test: passed `cargo test -p rusty-idd-cli --locked` after generated artifact
  refresh; result was 59 passed.
- Lint: passed `cargo fmt --all -- --check`,
  `cargo clippy -p rusty-idd-cli --all-targets --locked -- -D warnings`, and
  `git diff --check`.
- Audit: passed `cargo audit --deny warnings`.
- Secret scan: branch diff scan for AWS keys, private-key headers, GitHub PATs,
  OpenAI-style keys, and Slack tokens returned no matches. Whole-tree scan only
  reported known upstream repomix secretlint fixture strings under
  `third_party/upstream/repomix-rs`.
- Manifest: refreshed `.idd/MANIFEST.tsv` after deterministic artifact updates.
- Rusty IDD validation: passed `rusty-idd validate --workspace .` with 0
  critical and 0 warning.
- Codex invariant check: passed `rusty-idd codex env-check --workspace .`.

## Rollback Path

Revert ADR 0009, `openspec/changes/envctl-owned-rust-toolchain-cache`,
`.idd/goals/envctl-owned-rust-toolchain-cache.md`, the `codex system-audit`
Rust toolchain audit changes, the Codex environment documentation update, the
focused tests, and regenerated `.idd/knowledge/*`, `.idd/MANIFEST.tsv`,
`AI_MERGE/validation_report.md`, and `docs/rusty-idd/architecture-diagrams.md`.
