# ADR 0004: Direct crate integration for code knowledge

## Status

Accepted

## Current Correction

This ADR records the PR #52 local knowledge-integration boundary, not the full
current system architecture. Two assumptions from that pass are stale:

- Tree-sitter is now an active current dependency path through the Yazelix
  surface. Future parser work must re-evaluate the live Yazelix-backed
  tree-sitter contract instead of treating the PR #52 compatibility hold as
  authoritative.
- Domain, daemon, and cross-agent coordination surfaces are active through the
  weave + obscura upgrade path. This ADR's default-path exclusions apply only to
  the local `rusty-idd knowledge` slice that landed in PR #52; they are not a
  blanket system rule against those surfaces.

## Context

`rusty-idd knowledge` needs in-process codebase knowledge without adding MCP
servers, daemons, or long-running context transport. The corrected integration
strategy is to adopt the full upstream repositories first, run their native
diagnostics, then consolidate only the evidenced default-path boundary. The
useful default-path upstream pieces are the `codegraph-rust` parser/core crates
and the Repomix core/config crates.

The initial implementation tried to cut too early by introducing a local
`codegraph` compatibility shim plus a Rust AST reconstruction pass. That made
the integration smaller on paper, but it also replaced upstream behavior with
local guesses. The corrected direction is to adopt the upstream parser/core
crates whole first, then trim only concrete compile, audit, or runtime friction.

## Decision

Preserve full upstream snapshots as tracked mirrors:

- `third_party/upstream/codegraph-rust` at
  `ce5bf27a2978983a9089d177447f296e4c6521bb`
- `third_party/upstream/repomix-rs` at
  `946df10d48c669ca3a99f757ffd2c6fa35844e62`

The mirrors include upstream scripts, skills/docs, tests, fixtures, configs,
examples, CI, workspace files, package metadata, and lockfiles. They are not
Cargo workspace members; they are the audit baseline and rollback source for
future consolidation.

Vendor the useful upstream `codegraph-rust` crates as local workspace members:

- `crates/external/codegraph-core`
- `crates/external/codegraph-parser`

Preserve the vendored license surface in each external crate directory. The
local validation gate requires `LICENSE-MIT` and `LICENSE-APACHE` for the
CodeGraph crates and `LICENSE-MIT` for the Repomix shared DTO crate so future
audits cannot silently lose attribution material.

Use `codegraph-parser::languages::rust::RustExtractor::extract_with_edges` as
the graph source for Rust symbols, imports, calls, type references, trait/impl
metadata, and cyclomatic complexity. The `crates/knowledge` adapter adds only
Rusty IDD DTO mapping, deterministic compact local IDs, synthetic file nodes,
containment edges, and conservative symbolic edge resolution.

Disable `codegraph-core`'s jemalloc global allocator by default. In PR #52, the
tree-sitter dependency line was held to the then-compatible workspace runtime.
That hold is now superseded by the current Yazelix-backed tree-sitter
direction; future parser work must re-check the active tree-sitter contract and
upgrade forward from evidence.

Cut audit-denied or incompatible default dependencies after adoption:

- Remove `lsh-rs2` from the vendored parser default path because it pulls
  audited `libsqlite3-sys` and old `rand` advisories. Keep the resolver on its
  dependency-free string-similarity fallback.
- Remove `dotenv` loading from the vendored core default path because this
  integration reads normal process environment only and `dotenv` is
  unmaintained.
- In PR #52, filter the parser registry to expose only grammars compatible with
  the then-pinned tree-sitter runtime. Treat that as a historical integration
  cut. Current work should use the Yazelix-backed tree-sitter surface as the
  live compatibility source and preserve/upgrade broad parser support where
  evidence allows.

Patch `repomix-shared` to a minimal local DTO crate at
`crates/external/repomix-shared`. The published crate pulls logger/UI
dependencies that are not used by `repomix-core` in this integration and include
an audited unmaintained transitive dependency (`number_prefix` through
`indicatif`).

The pinned `repomix-rs` upstream source is version `2.0.1`, but the compatible
published crates.io surface available for this workspace is `2.0.0`. The full
`2.0.1` mirror remains tracked so future upgrades can refresh from source rather
than from memory. Default Rusty IDD workflows continue to use the latest
published compatible Repomix crates plus the audited local DTO patch until the
current Yazelix/tree-sitter and audit contracts are re-evaluated against the
pinned upstream source.

Persistent graph storage remains out of the default path. Future persistent
storage work must go behind the explicit `knowledge-surrealdb` feature or
another reviewed feature gate.

## Consequences

- Default builds use upstream codegraph parser/core behavior instead of a local
  parser reconstruction.
- The full upstream repositories are preserved as tracked mirrors before any
  cut is accepted, so PR #50's selected-crate baseline is no longer the only
  adoption evidence.
- The adapter preserves upstream node metadata and symbolic edges while keeping
  the public `rusty-idd knowledge` index compact and deterministic.
- Persistent graph storage, MCP servers, daemon surfaces, vector search, and
  cloud providers stayed out of the PR #52 default knowledge slice. This is not
  a system-level downgrade or exclusion; weave + obscura domain work may make
  those surfaces first-class through explicit specs and feature boundaries.
- The Repomix patch keeps the shared DTO contract but removes unused logger/UI
  dependencies from the default build.
- Vendored upstream watcher tests that depend on filesystem notification timing
  are marked ignored; they are outside the `rusty-idd knowledge` boundary.
- MCP crates remained excluded from the PR #52 local knowledge integration
  slice. Future weave/obscura work must define the correct cross-repo contract
  before deciding whether that remains true for the system path.
