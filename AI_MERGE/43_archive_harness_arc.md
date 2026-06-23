# 43 — Archive the harness control-plane arc + fix new-capability archiving

Evidence note for `chore/archive-harness-arc`. Closes the Rusty IDD flow loop
for the five completed harness changes by archiving them — and fixes a gap in
the archive tooling discovered while doing so.

## The gap (found by closing the loop)

The front door (`rusty-idd next`) ends a completed change with the imperative
`rusty-idd spec archive …`. But `spec archive` **failed for every new-capability
change**: it read the base spec at `openspec/specs/<cap>/spec.md` and aborted
with "failed to read base spec … No such file" when the capability had no base
yet — which is always true for a change that ADDs a brand-new capability. So the
final step the front door points at was broken for exactly the changes this arc
produced. A broken step in the determinism loop is a real gap.

## The fix

`crates/cli/src/commands/spec_archive.rs`:
- On a **NotFound** base spec, synthesize a minimal titled seed
  (`seed_base_spec` / `title_case`: `# Harness Control Plane` + `## Purpose` +
  `## Requirements`) so the delta's `## ADDED Requirements` merge into a valid,
  newly-created base spec. Any non-NotFound IO error still aborts. A
  MODIFIED/REMOVED op against the empty seed still errors correctly (you cannot
  modify what does not exist) — proven by the existing `archive_aborts_*` test.
- Create the capability's base-spec parent dir before writing (idempotent for
  existing capabilities).
- Tests: `title_case_handles_kebab_and_snake`,
  `seeded_base_parses_to_titled_empty_spec` (unit) and
  `archive_creates_base_for_new_capability` (integration).

## Archived changes (loop closed)

`harness-control-plane`, `harness-next-json` (merged as a MODIFIED into the
`harness-control-plane` base), `harness-vendor-render`, `harness-session-frontdoor`,
and `add-verify-package-stage` — moved to `openspec/changes/archive/`, their delta
specs merged into base specs under `openspec/specs/`. The active-change pointer is
cleared (no active change); `rusty-idd next` reports cleanly (exit 0).

## Verification evidence

- `cargo fmt --all -- --check` — clean.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — no issues.
- `cargo test --workspace --locked` — 669 passed, 0 failed (+3 archive tests).
- `rusty-idd spec validate --all` — 135/135 (deltas merged to base, changes archived).
- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- knowledge + manifest refreshed, self-stable (3547 entries), 0 contamination.
- `render --all --check` + `spec adr list --check` — both green.
