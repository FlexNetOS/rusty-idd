# harness-vendor-render — Tasks

## 1. Implementation

- [ ] 1.1 `commands::render` with engine-owned adapter template + `expected_adapter(vendor)`
- [ ] 1.2 `rusty-idd render [--vendor <name> | --all] [--check] [--base <dir>]`
- [ ] 1.3 Write mode creates the adapter; `--check` fails closed on missing/drift
- [ ] 1.4 Wire into CLI enum / dispatch / module tree / lib docs

## 2. Tests

- [ ] 2.1 render writes a deterministic adapter (byte-identical on re-run)
- [ ] 2.2 render --all over a temp tree, then --check passes
- [ ] 2.3 hand-edited adapter → --check exits non-zero, names it
- [ ] 2.4 missing adapter → --check exits non-zero

## 3. Enforcement

- [ ] 3.1 Render adapters into `.claude` / `.codex` / `.agents` / `.devin`
- [ ] 3.2 Add `rusty-idd render --all --check` to `.github/workflows/ci.yml`
- [ ] 3.3 Add a `render-check` Justfile recipe

## 4. Verification gates

- [ ] 4.1 `cargo test --workspace`; `fmt --check`; `clippy --all-features -D warnings`
- [ ] 4.2 `spec validate --all`; `validate --workspace .` 0/0
- [ ] 4.3 refresh `.idd/knowledge/*` + `MANIFEST.tsv` (refresh-last, validate→manifest)
