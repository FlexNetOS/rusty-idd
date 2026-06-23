# harness-next-json — Tasks

## 1. Implementation

- [ ] 1.1 Expose a reusable snapshot constructor from `commands::spec_status`
- [ ] 1.2 Add `--json` to `commands::next` emitting a deterministic front-door object
- [ ] 1.3 Fail closed on dangling pointer (non-zero, no stdout JSON)
- [ ] 1.4 Keep default human output unchanged

## 2. Tests

- [ ] 2.1 `next --json` emits expected fields for a partial change
- [ ] 2.2 `next --json` is byte-identical across repeated runs (determinism)
- [ ] 2.3 dangling pointer with `--json` exits non-zero, no stdout JSON

## 3. Verification gates

- [ ] 3.1 `cargo test --workspace`; `fmt --check`; `clippy --all-features -D warnings`
- [ ] 3.2 `rusty-idd spec validate --all`; `validate --workspace .` 0/0
- [ ] 3.3 refresh `.idd/knowledge/*` + `MANIFEST.tsv` (refresh-last, validate→manifest order)
