# Upstream Full Adoption Audit

## Scope

This pass corrects the PR #50 baseline by adopting both upstream repositories
as full tracked mirrors before judging which surfaces belong in Rusty IDD's
default knowledge path.

## Verified Upstream Pins

| Upstream | URL | Ref |
| --- | --- | --- |
| CodeGraph Rust | `https://github.com/Jakedismo/codegraph-rust` | `ce5bf27a2978983a9089d177447f296e4c6521bb` |
| repomix-rs | `https://github.com/sopaco/repomix-rs` | `946df10d48c669ca3a99f757ffd2c6fa35844e62` |

Search verification also found the TypeScript Repomix upstream at
`yamadashy/repomix`; that repository is not the Rust integration target for
this pass.

## Adopted Full Mirrors

- `third_party/upstream/codegraph-rust`: imported with `git archive` from the
  pinned CodeGraph commit. The mirror includes tracked scripts, docs, tests,
  fixtures, configs, CI workflows, examples, workspace manifests, lockfile,
  Python helper material, and package metadata.
- `third_party/upstream/repomix-rs`: imported with `git archive` from the
  pinned repomix-rs commit. The mirror includes tracked crates, MCP crate,
  npm packaging, scripts, MindMesh material, docs, tests, configs, CI workflows,
  workspace manifests, and lockfile.

The mirrors are not Cargo workspace members. They are the rollback and audit
baseline for future cuts.

## Native Upstream Diagnostics

Commands were run in clean detached checkouts under `/tmp` before importing or
cutting.

### CodeGraph Rust

| Command | Result | Notes |
| --- | --- | --- |
| `make ci` | Failed | Stops at `cargo fmt --all -- --check`; upstream formatting diffs in `codegraph-core`, `codegraph-graph`, and other files. |
| `cargo build --workspace` | Passed | Built 622 crates in dev profile. |
| `cargo test --workspace` | Failed | Test build error in `crates/codegraph-vector/src/simd_ops.rs`: variable is defined as `_scalar_result` but assertions reference `scalar_result`. |
| `cargo clippy -p codegraph-core -- -A clippy::all -A warnings` | Passed | Matches the local Makefile's limited lint target. |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Failed | Fails while compiling macOS clipboard dependency path on this Linux host through `objc_exception`. |
| `cargo audit --deny warnings` | Failed | 12 vulnerabilities and 15 denied warnings; notable default-path issues include `lsh-rs2` via `libsqlite3-sys`, `bytes`, `quinn-proto`, `rkyv`, `rustls-webpki`, `paste`, `atty`, `lru`, and `rand`. |
| `cargo doc --workspace --no-deps` | Passed with warnings | Warnings include invalid rustdoc HTML tags and a broken intra-doc link. |

Valuable surfaces: `codegraph-core`, `codegraph-parser`,
`RustExtractor::extract_with_edges`, complexity data, symbolic edge extraction,
language registry source, watcher source, graph/vector/MCP/server crates for
future feature-gated work, CI/security workflow evidence, config examples, and
installation/runtime docs.

Runtime assumptions: full agentic/MCP paths assume optional LSP binaries,
SurrealDB/vector/embedding dependencies, cloud/provider configuration, and host
service/server management. These remain outside Rusty IDD's default workflow.

### repomix-rs

| Command | Result | Notes |
| --- | --- | --- |
| `cargo fmt --all -- --check` | Failed | Upstream formatting diffs in tree-sitter language setup and new-language tests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy issues. |
| `cargo test --workspace` | Passed | 93 tests passed across 12 suites. |
| `cargo build --workspace` | Passed | No additional crates compiled after test/clippy. |
| `node scripts/check-npm-version.mjs` | Passed | All versions match `2.0.1`. |
| `cargo audit --deny warnings` | Failed | `number_prefix` unmaintained through `indicatif 0.17.11` in `repomix-shared` and `repomix-cli`. |
| `cargo doc --workspace --no-deps` | Passed | Generated docs for all five crates. |
| `cargo run -p repomix-cli -- --version` | Passed | Reported `repomix-rs 2.0.1`. |
| `cargo run -p repomix-cli -- --style xml --output /tmp/repomix-rs-smoke.xml --include '*.rs' --ignore 'target/**' .` | Passed | Packed 59 files, 230731 characters, and 55118 tokens. |

Valuable surfaces: `repomix-core`, `repomix-config`,
`RepomixConfig::load`, `PackOptions`, output splitting/header/instruction
fields, git diff/log options, tree-sitter compression, suspicious-file data,
MCP tool shapes, npm packaging, and version-alignment script.

Runtime assumptions: git-aware paths require `git`; MCP mode is stdio server
runtime; clipboard/UI/logger paths pull dependencies not needed by Rusty IDD's
headless default knowledge path.

## Consolidation Cuts Kept

The full mirrors prove that the current default build cannot simply point at
every upstream crate as-is without violating Rusty IDD's boundaries and gates.
These cuts remain deliberate:

- Keep `crates/core` std-only; no upstream knowledge dependency crosses into
  `crates/core`.
- Keep MCP/server/daemon/vector/cloud/provider paths out of default Rusty IDD
  workflows. Rollback path: wire from `third_party/upstream/codegraph-rust`
  behind a reviewed feature such as `knowledge-vector`,
  `knowledge-surrealdb`, or `knowledge-cloud`.
- Keep the consolidated CodeGraph crates in `crates/external` as the default
  parser boundary. Rollback path: refresh from
  `third_party/upstream/codegraph-rust/crates/codegraph-core` and
  `third_party/upstream/codegraph-rust/crates/codegraph-parser`, then re-run the
  targeted parser tests and full gates.
- Keep `lsh-rs2` out of the default parser path because native upstream audit
  still denies its `libsqlite3-sys` and old `rand` dependency chain.
- Keep CodeGraph default jemalloc disabled and dotenv loading absent in the
  default path. Rollback path: feature-gate those upstream defaults and rerun
  audit.
- Keep the tree-sitter line aligned in the default workspace. The pinned
  CodeGraph and repomix-rs upstreams use incompatible newer tree-sitter lines
  for direct simultaneous path dependency use, and Cargo permits only one
  native `links = "tree-sitter"` package in this workspace.
- Keep the minimal local `repomix-shared` DTO crate while `repomix-core` and
  `repomix-config` remain on the latest published compatible crates. The
  pinned repomix-rs `2.0.1` source is mirrored, but the crates.io surface
  currently exposes `2.0.0`; switching directly to the mirrored `2.0.1`
  workspace would also reintroduce the `number_prefix` audit denial through
  `indicatif`.
- Exclude `third_party/upstream/**` from default `rusty-idd knowledge` indexing
  and report packing. The mirrors are audit source, not default graph input.
  Rollback path: include a specific mirrored upstream path explicitly in a
  feature or command designed for upstream audits.
- Exclude `third_party/upstream/**` from repo-local validation policy scans
  while keeping it in `.idd/MANIFEST.tsv`. The mirrored upstreams contain test
  fixtures and example configs with intentionally secret-shaped strings; mutating
  them would violate the as-is adoption requirement. Rollback path: add an
  upstream-audit-specific scanner that reports findings against the mirror
  without blocking default Rusty IDD validation.
- Exclude `third_party/upstream/**` from repo-native drift-check crate and
  foreign-manifest discovery while keeping the mirror tracked and in the
  manifest. The mirrors contain upstream crate layouts, prompt markdown, Python
  metadata, and tree-sitter query assets that are valid upstream source but not
  first-party Rusty IDD drift. Rollback path: remove the drift-check exclusion
  only if the mirrors are converted from audit input into first-party workspace
  crates with their non-Rust assets ported or feature-gated.

## TDD Verification During Consolidation

- Added a focused knowledge test proving default indexing skips full upstream
  mirrors while still indexing local Rust source.
- Added Codex environment invariants so required full-upstream metadata and
  this audit note are checked with `rusty-idd codex env-check`.
- Refreshed `.idd/knowledge/*` and `.idd/MANIFEST.tsv` after source and
  control-plane changes.
- Updated both `.gemini` and `.claude` merge-verification drift gates so full
  upstream mirrors remain adopted as-is without being misclassified as
  first-party Rust-native drift.

## PR #50 Baseline Comparison

PR #50 established direct crate use, but it treated selected vendored crates as
the effective adoption surface. This pass adds the missing step: exact full
upstream mirrors are now tracked first, native upstream diagnostics are recorded
before cuts, and every retained cut has a rollback path to the full mirror.
