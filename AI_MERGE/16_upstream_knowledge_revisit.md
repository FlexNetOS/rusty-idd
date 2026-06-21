# Upstream Knowledge Revisit - Full-Feature Adopt-First Pass

Date: 2026-06-21
Branch: `integration/full-feature-upstream-revisit`
OpenSpec change: `openspec/changes/revisit-upstream-knowledge-integration`
ADR: `adr/0005-full-feature-upstream-knowledge-integration.md`

## Purpose

This note records the upstream-as-is evidence before any Rusty IDD consolidation
or implementation cut for the `codegraph-rust` and `repomix-rs` integrations.

The correction for this pass is that Rusty IDD is the lifecycle/product workflow
for OpenSpec plans, ADRs, specs, tasks, implementation, validation, and archive
state. The build/merge process is only one execution path inside that workflow.

## Verified Upstream Pins

| Repo | URL | Current HEAD on 2026-06-21 | Local mirror |
|---|---|---|---|
| CodeGraph Rust | `https://github.com/Jakedismo/codegraph-rust.git` | `ce5bf27a2978983a9089d177447f296e4c6521bb` | `third_party/upstream/codegraph-rust` |
| repomix-rs | `https://github.com/sopaco/repomix-rs.git` | `946df10d48c669ca3a99f757ffd2c6fa35844e62` | `third_party/upstream/repomix-rs` |

The current network HEADs match the revisions already recorded in
`third_party/upstream/UPSTREAMS.md`.

## Upstream Native Commands

### CodeGraph Rust

Discovered command surfaces:

- `Makefile`: `build`, `build-release`, `test`, `lint`, `fmt`,
  `fmt-check`, `check`, `ci`, `audit`, `outdated`, `doc`,
  `build-mcp-autoagents`, `build-mcp-http`, `run-http-server`,
  benchmark and deployment targets.
- `Cargo.toml`: full workspace with parser, vector, graph, MCP, daemon,
  AutoAgents, AI, cloud/vector feature groups, and multi-language
  `tree-sitter` dependencies.
- README: indexing tiers, LSP prerequisites, architecture boundary rules,
  agentic tools, MCP/server usage, and default Rig agent architecture.

Native diagnostics run against a fresh temp clone at
`/tmp/rusty-idd-upstream-codegraph`:

| Command | Result | Evidence |
|---|---|---|
| `make ci` | Failed | upstream `cargo fmt --all -- --check` diff in `compression.rs`, `lib.rs`, and `graph_functions.rs` |
| `cargo build --workspace` | Passed | full workspace compiled |
| `cargo test --workspace` | Failed | `_scalar_result` is defined but `scalar_result` is asserted in `crates/codegraph-vector/src/simd_ops.rs` |
| `cargo clippy -p codegraph-core -- -A clippy::all -A warnings` | Passed | no issues |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Failed | custom build command failure for upstream `objc_exception v0.1.2` path |
| `cargo doc --workspace --no-deps` | Passed with warnings | rustdoc invalid HTML/bare URL/broken intra-doc warnings |
| `make audit` | Failed | `cargo audit` reported upstream vulnerabilities including `bytes 1.11.0`; full report in local rtk tee output |
| `make outdated` | Blocked | `cargo-outdated` is not installed in the parent-managed toolchain |

Runtime/tool assumptions:

- LSP-enabled CodeGraph indexing tiers require `rust-analyzer`, `node`,
  `typescript-language-server`, `pyright-langserver`, `gopls`, `jdtls`, and
  `clangd` depending on language.
- MCP/server/daemon/cloud/vector targets exist upstream and are valuable
  system surfaces, but remain out of Rusty IDD default workflows unless
  explicitly feature-gated and justified.
- `cargo-outdated` is an optional diagnostic gap. It must be added through
  parent `meta` / `envctl` if this repo makes outdated checks a required gate.

Valuable upstream surfaces:

- Multi-language `tree-sitter` parser registry and query assets.
- Rust graph extractor, symbol/import/call/type/trait/impl metadata, and
  complexity output.
- Indexing tiers, LSP prerequisite model, architecture boundary rules, and
  agentic tool contracts.
- MCP, daemon, AutoAgents, Rig, cloud, embeddings, graph, vector, and watcher
  surfaces as feature-gated system capability source material.

Retained cut boundary:

- Rusty IDD default knowledge paths continue to avoid host-service management
  and MCP transport unless feature-gated.
- `crates/core` remains std-only.
- Any future CodeGraph consolidation must first isolate the specific compile,
  audit, platform, or scope friction and keep rollback to the mirrored upstream.

### repomix-rs

Discovered command surfaces:

- `Cargo.toml`: workspace crates `core`, `config`, `cli`, `mcp`, `shared`.
- `README.md`: CLI, async library API, tree-sitter compression, token counting,
  security scanning, git-aware output, config layering, MCP tools, npm package
  surface, and remote repository packing.
- `AGENTS.md`: MindMesh knowledge, repomix context, codegraph, and RTK agent
  workflow; explicitly says not to run `codegraph install` or `rtk init`.
- `scripts/check-npm-version.mjs`: Bun/Node version consistency diagnostic.
- `npm/repomix-rs/package.json`: npm package version `2.0.1`, Node `>=18`,
  optional platform package versions.

Native diagnostics run against a fresh temp clone at
`/tmp/rusty-idd-upstream-repomix`:

| Command | Result | Evidence |
|---|---|---|
| `cargo fmt --all -- --check` | Failed | upstream fmt diff in `tree_sitter/languages.rs` and `new_languages_test.rs` |
| `cargo build --workspace` | Passed | workspace compiled |
| `cargo test --workspace` | Passed | 93 tests passed |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Passed | no issues |
| `bun scripts/check-npm-version.mjs --expected 2.0.1` | Passed | all Cargo/npm versions match |
| `cargo doc --workspace --no-deps` | Passed | docs generated |
| `cargo audit --deny warnings` | Failed | `number_prefix 0.4.0` unmaintained via `indicatif 0.17.11` |
| `cargo run -p repomix-cli -- --version` | Passed | `repomix-rs 2.0.1` |
| `cargo run -p repomix-cli -- --style markdown --compress --output /tmp/rusty-idd-repomix-smoke.md .` | Passed | packed 90 files, 28,519 tokens, generated compressed markdown output |

Runtime/tool assumptions:

- Git-aware features require `git` on `PATH`.
- Node `>=18` or Bun is required for npm version diagnostics and npm package
  workflows.
- Tree-sitter compression is active and proven by the compressed CLI smoke.
- MCP server exists upstream but remains feature-gated/out of default Rusty IDD
  workflows unless explicitly justified.

Published crate surface:

- `cargo search` shows `repomix-core`, `repomix-config`, and `repomix-shared`
  at `2.0.0` on crates.io.
- Upstream source and npm package are `2.0.1`.
- This keeps the prior compatibility split alive: the `2.0.1` mirror remains
  the source/rollback baseline while local crate wiring must use the latest
  compatible audited surface available to Rusty IDD.

Valuable upstream surfaces:

- Tree-sitter compression across the upstream language set.
- Token accounting and top-file metrics.
- Security scan and suspicious file outputs.
- Git diffs/logs/change-frequency sort and remote repo packing.
- Async `pack`, `pack_directory`, `pack_with_config`, and `pack_with_options`
  library API.
- MCP tools as feature-gated system capability source material.

Retained cut boundary:

- Rusty IDD may keep a thin local DTO compatibility crate only if replacing it
  with the upstream crate surface would downgrade compatibility, audit status,
  or deterministic CLI/API behavior.
- Any future replacement of the shim with path or published upstream crates must
  be a TDD step with behavior comparison and rollback to the mirrored upstream.

## Corrections From Prior Statements

- `tree-sitter` is active in the system through Yazelix and through the upstream
  parser/compression repos. It is not accurate to describe it as absent.
- Domains are active system capability through weave plus Obscura upgrades.
  Rusty IDD default workflows can scope domain/daemon behavior out, but cannot
  describe the system as lacking those surfaces.
- MCP, daemon, server, cloud, and host-service surfaces are not discarded. They
  are feature-gated or external-system surfaces unless Rusty IDD explicitly
  takes ownership for a workflow.
- Rusty IDD's actual job is to consume and automate plans, ADRs, specs, tasks,
  implementation, validation, and archive/handoff artifacts. Build/merge is an
  execution process, not the whole product.

## TDD Consolidation Step

Cut retained:

- `crates/knowledge` no longer calls `RustExtractor` directly. It now collects
  source files through CodeGraph's `LanguageRegistry`, creates the matching
  parser, and dispatches through CodeGraph's `extract_for_language` for every
  compatible registered language.

Evidence:

- `cargo test -p codegraph-parser registered_languages_use_supported_versions --locked`: passed.
- `cargo test -p rusty-idd-knowledge indexes_multiple_tree_sitter_languages_through_codegraph_dispatch --locked`: passed.
- `cargo test -p rusty-idd-knowledge --locked`: passed.

Behavior preserved:

- The local DTO boundary remains the same: deterministic node/edge IDs,
  `KnowledgeIndex`, `FileSummary`, `KnowledgeNode`, `KnowledgeEdge`,
  import summaries, hotspots, parse failures, and pack summaries.
- The repomix pack path remains direct crate integration with deterministic JSON
  timestamps disabled, token budget enforcement, explicit output files, and
  upstream compression options.
- `crates/core` was not touched and remains std-only.

Rollback:

- Revert `crates/knowledge/src/lib.rs` to the previous Rust-only
  `RustExtractor` call.
- Re-run `cargo test -p rusty-idd-knowledge --locked`.
- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.

## Rollback

- Revert local code/doc changes from this branch.
- Keep upstream mirrors at the verified pins above.
- Re-run `rusty-idd knowledge refresh` and manifest generation after rollback.
- Use this note plus `AI_MERGE/14_upstream_full_adoption.md` as the evidence
  baseline for any narrower reattempt.
