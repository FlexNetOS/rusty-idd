# Codex Hook Base Ref Validation Evidence

- Change: `fix-codex-hook-base-ref`
- Branch: `fix/codex-hook-issues`
- Build: passed `cargo check -p rusty-idd-cli --locked`.
- Generated artifacts: refreshed `.idd/knowledge/index.json`,
  `.idd/knowledge/report.md`, `.idd/knowledge/architecture.json`,
  `.idd/knowledge/architecture.md`, `.idd/knowledge/plan-context.json`,
  `.idd/knowledge/plan-context.md`, `.idd/MANIFEST.tsv`, and
  `AI_MERGE/validation_report.md` before final tests.
- Test: passed `cargo test -p rusty-idd-cli --locked`; result was 60 passed.
- Lint: passed `cargo fmt --all -- --check`,
  `cargo clippy -p rusty-idd-cli --all-targets --locked -- -D warnings`, and
  `git diff --check`.
- Audit: passed `cargo audit --deny warnings` with 1137 loaded RustSec
  advisories.
- Secret scan: passed; `rg` scan for API keys, secrets, tokens, passwords, and
  private keys outside `target/**`, `Cargo.lock`, and `third_party/**` returned
  no matches.
- Manifest: refreshed `.idd/MANIFEST.tsv` after deterministic artifact updates.
- Rusty IDD validation: passed `rusty-idd validate --workspace .` with 0
  critical and 0 warning.
- Codex invariant check: passed `rusty-idd codex env-check --workspace .`.
- Codex runtime audit: passed `rusty-idd codex runtime-audit --workspace .`;
  live Codex Python commands and obsolete Python hook files were both 0.

## Rollback Path

Revert `openspec/changes/fix-codex-hook-base-ref`,
`.idd/goals/fix-codex-hook-base-ref.md`, the Codex workflow checker base-ref
selection changes, the focused regression test, and regenerated
`.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and `AI_MERGE/validation_report.md`.
