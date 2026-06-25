# Harness Control-Plane — Rusty IDD Flow Evidence

Change: `openspec/changes/harness-control-plane` · ADR-0015 · branch
`feature/harness-control-plane` · base `develop` · PR #107.

This change was (re)driven through the full Rusty IDD workflow from step 1, per
`AGENTS.md` Rusty IDD Workflow Rules 1–9.

## Workflow steps (run end to end, via the CLI)

1. **Goal** — `.idd/goals/harness-control-plane.md` (intent + decision target).
2. **Oracle / state** — `rusty-idd next` (front door) reports the active change
   and the next ready artifact.
3. **Graph context** — `rusty-idd knowledge refresh --workspace .`
   (`architecture.{json,md}`, `index.json`, `report.md`).
4. **Goal binding** — `rusty-idd knowledge plan-context --workspace . --out
   .idd/knowledge/plan-context.{json,md} --change harness-control-plane
   --goal-file .idd/goals/harness-control-plane.md` (plan-context now references
   the goal).
5. **Diagrams** — `rusty-idd knowledge diagrams --workspace . --out
   docs/rusty-idd/architecture-diagrams.md` (as-is, dogfooded: 141 files / 8832
   nodes / 36415 edges).
6. **OpenSpec change + readiness** — proposal, capability spec, design (as-is +
   to-be graphs), tasks; `rusty-idd spec status ./openspec/changes/harness-control-plane`
   → **5/5 artifacts, archivable**.
7. **ADR** — `adr/0015-harness-control-plane.md` (unifies ADR-0001/0002/0010).
8. **Stage package** — `rusty-idd harness package --stage scan --target .`
   captured at `openspec/changes/harness-control-plane/assets/harness-package-scan.md`.
9. **Implementation** — `crates/cli/src/commands/next.rs` (front door) +
   `next_artifact_id` helper in `spec_status.rs`; CLI wiring; `tests/next_cli.rs`.
10. **Validation refresh** — `knowledge refresh` (run last), `validate
    --workspace .`, `manifest`.

## Required PR Evidence

- **Build:** `cargo build -p rusty-idd-cli` — ok.
- **Test:** `cargo test --workspace --locked` — 644 passed, 3 ignored;
  `cargo test -p rusty-idd-cli` — 69 passed (incl. `next_cli` 3 + unit).
- **Lint/typecheck:** `cargo clippy --workspace --all-targets --all-features --
  -D warnings` — no issues; `cargo fmt --all --check` — clean.
- **Secret scan:** no secrets added; new files are Rust/markdown/JSON artifacts.
- **IDD validation:** `rusty-idd validate --workspace .` — 0 critical, 0 warning.
- **Spec validation:** `rusty-idd spec validate --all` — 140/140 pass (only the
  repo's known brief-purpose warnings).
- **Migration note:** old path = static prose harness re-read per session across
  `.claude`/`.codex`/`.agents`/`.devin`; new path = `rusty-idd next` front door
  computes the one imperative from the artifact DAG, vendor dirs are thin
  adapters (ADR-0015, generalizing ADR-0010). Purely additive; `spec status` /
  `spec next` behaviour unchanged.
- **Rollback:** revert `commands/next.rs` + the CLI wiring + this change; the
  spec/knowledge engines are untouched.
- **Manifest:** `.idd/MANIFEST.tsv` regenerated (3514 entries; self-stable across
  runs; no `.worktrees`/`.idd-bak` contamination).

## Notes

- The cross-repo `system-architecture` / `operating-model` / `integration-*`
  artifacts scan the parent meta workspace (`--system-root ..`) and are
  maintained as meta-level deterministic artifacts; they are left unchanged here
  to keep this feature PR free of environment-specific cross-repo churn.
- merge-tools verify reports only `.worktrees/**` foreign manifests locally
  (sibling worktrees absent in CI's clean checkout) — 0 real findings.
