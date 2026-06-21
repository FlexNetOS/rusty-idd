# ADR 0004: Direct crate integration for code knowledge

## Status

Accepted

## Context

`rusty-idd knowledge` needs in-process codebase knowledge without adding MCP
servers, daemons, or long-running context transport. The useful upstream pieces
are the `codegraph-rust` parser/core crates and the Repomix core/config crates.

The initial implementation tried to cut too early by introducing a local
`codegraph` compatibility shim plus a Rust AST reconstruction pass. That made
the integration smaller on paper, but it also replaced upstream behavior with
local guesses. The corrected direction is to adopt the upstream parser/core
crates whole first, then trim only concrete compile, audit, or runtime friction.

## Decision

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

Disable `codegraph-core`'s jemalloc global allocator by default. Align
tree-sitter dependencies with Repomix's `tree-sitter` major/minor line to avoid
native `links = "tree-sitter"` conflicts in the workspace.

Cut audit-denied or incompatible default dependencies after adoption:

- Remove `lsh-rs2` from the vendored parser default path because it pulls
  audited `libsqlite3-sys` and old `rand` advisories. Keep the resolver on its
  dependency-free string-similarity fallback.
- Remove `dotenv` loading from the vendored core default path because this
  integration reads normal process environment only and `dotenv` is
  unmaintained.
- Filter the parser registry to expose only grammars compatible with the
  workspace-pinned tree-sitter runtime. The incompatible extractor source stays
  vendored for future compatible pins or explicit feature work.

Patch `repomix-shared` to a minimal local DTO crate at
`crates/external/repomix-shared`. The published crate pulls logger/UI
dependencies that are not used by `repomix-core` in this integration and include
an audited unmaintained transitive dependency (`number_prefix` through
`indicatif`).

Persistent graph storage remains out of the default path. Future persistent
storage work must go behind the explicit `knowledge-surrealdb` feature or
another reviewed feature gate.

## Consequences

- Default builds use upstream codegraph parser/core behavior instead of a local
  parser reconstruction.
- The adapter preserves upstream node metadata and symbolic edges while keeping
  the public `rusty-idd knowledge` index compact and deterministic.
- Persistent graph storage, MCP servers, daemon surfaces, vector search, and
  cloud providers stay out of the default integration.
- The Repomix patch keeps the shared DTO contract but removes unused logger/UI
  dependencies from the default build.
- Vendored upstream watcher tests that depend on filesystem notification timing
  are marked ignored; they are outside the `rusty-idd knowledge` boundary.
- MCP crates remain excluded from the integration.
