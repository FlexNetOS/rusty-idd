# align-envctl-toolchain-contract - Tasks

## 1. Artifact Flow

- [x] 1.1 Record the self-generated goal file.
- [x] 1.2 Create OpenSpec proposal, design, spec delta, and tasks.
- [x] 1.3 Create ADR 0013 for envctl toolchain layouts.
- [x] 1.4 Record task and validation evidence.

## 2. Implementation

- [x] 2.1 Add `RUSTY_IDD_RUST_LAYOUT` selection to
  `scripts/ci/envctl-rust-env.sh`.
- [x] 2.2 Preserve GitHub Actions default isolated `.env/rust` layout.
- [x] 2.3 Add local `.toolchains` auto-selection when envctl Rust state exists.
- [x] 2.4 Print the selected layout in activation output.
- [x] 2.5 Document the local and CI layout split.

## 3. Validation

- [x] 3.1 Run local `toolchains` layout activation.
- [x] 3.2 Run Rusty IDD strict system-audit with actual meta-owned compiler
  paths where local tools allow it.
- [x] 3.3 Refresh `.idd/knowledge/*`.
- [x] 3.4 Verify OpenSpec status.
- [x] 3.5 Run Rusty IDD validation.
- [x] 3.6 Refresh `.idd/MANIFEST.tsv`.
- [x] 3.7 Commit, push, open PR, and monitor CI.
