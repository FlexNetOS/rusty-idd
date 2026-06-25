# harness-session-frontdoor — Tasks

## 1. Session-start wiring (4.2)

- [x] 1.1 Add a `SessionStart` hook to `.codex/hooks.json` running `rusty-idd next --base "$root"`
- [x] 1.2 Create `.claude/settings.json` with a `hooks.SessionStart` entry running `rusty-idd next`
- [x] 1.3 Keep all existing Codex hooks (`PreToolUse`/`PostToolUse`/`Stop`/`SubagentStop`) unchanged

## 2. ADR collision gate (4.5)

- [x] 2.1 Add `--check` to `spec adr list`: group by number, fail closed on any non-baseline duplicate
- [x] 2.2 Encode the frozen baseline `ACCEPTED_DUPLICATE_ADRS = [2,4,5,6]` with a clear comment
- [x] 2.3 Author `adr/0016-adr-ledger-reconciliation.md` (records collisions, slug-canonical rule)
- [x] 2.4 Keep default `spec adr list` output unchanged

## 3. Tests

- [x] 3.1 ADR `--check` passes on the real repo ADR set (baseline-only duplicates)
- [x] 3.2 ADR `--check` fails closed on a synthetic non-baseline duplicate, naming the number
- [x] 3.3 `.codex/hooks.json` parses and its `SessionStart` invokes `rusty-idd next`
- [x] 3.4 `.claude/settings.json` parses and its `SessionStart` invokes `rusty-idd next`

## 4. Enforcement

- [x] 4.1 Add an ADR-collision gate step to `.github/workflows/ci.yml`
- [x] 4.2 Add an `adr-check` Justfile recipe (and include it in the `ci` recipe)

## 5. Verification gates

- [x] 5.1 `cargo test --workspace`; `fmt --check`; `clippy --all-features -D warnings`
- [x] 5.2 `spec validate --all`; `validate --workspace .` 0/0
- [x] 5.3 refresh `.idd/knowledge/*` + `MANIFEST.tsv` (refresh-last, validate→manifest)
- [x] 5.4 Live `rusty-idd next` drives this change to archivable
