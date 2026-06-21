# Knowledge Deep Audit

## Current Correction

This audit predates the current Yazelix and weave + obscura direction. Treat
its exclusions as evidence for the PR #50/PR #52 local knowledge slice only.
Tree-sitter is now an active current surface through Yazelix, and
domain/daemon/fleet coordination is being driven through weave + obscura
upgrades. Future work should use Rusty IDD's OpenSpec lifecycle to replace
these older holds with system ADRs, per-repo specs, and implementation tasks.

## Scope

Reviewed the direct knowledge integration against these upstream repositories:

- `sopaco/repomix-rs`
- `Jakedismo/codegraph-rust`
- `anvanster/codegraph`

The audit specifically checked for useful capabilities that were not represented in the
`rusty-idd knowledge` boundary after the initial direct crate integration.

## Skill Decision

It is worth using the upstream skills as workflow source material, but not copying them verbatim.
The useful pieces are repomix-oriented reading habits: inspect pack metadata first, search before
reading full bundles, prefer compressed XML/Markdown packs, keep exploratory outputs in `/tmp`, and
use include/ignore plus line-number/comment/git context modes intentionally.

The raw upstream skills also reference unrelated tools and surfaces such as MindMesh, bunx-based
CodeGraph CLIs, document generation systems, and MCP-oriented workflows. Those are not part of this
feature. The local adaptation is `.agents/skills/rusty-idd-knowledge/SKILL.md`.

## Added From Audit

- Repomix project/global config layering is now used through `RepomixConfig::load(...)`.
- Pack command/API now exposes:
  - `remove_empty_lines`
  - `truncate_base64`
  - `include_empty_directories`
  - `top_files_length`
  - `split_output`
  - `header_text`
  - `instruction_file`
- Generated report packs use compressed, comment-stripped, empty-line-stripped output with a bounded
  top-files list.
- CLI and unit tests cover the expanded pack surface and project config handling.

## Post-Test Correction

The first implementation still produced shallow graph output: file containment, imports, and
pack/report summaries were present, but the index did not provide enough semantic graph signal to
justify itself over a simple skill. The corrected path is to adopt the useful upstream codegraph
crates whole first, then cut only after concrete compile/audit friction is visible.

`crates/knowledge` now uses vendored `codegraph-core` and `codegraph-parser` crates directly. Rust
symbols, imports, calls, type references, impl/trait metadata, and complexity come from
`RustExtractor::extract_with_edges`. The local layer maps those upstream results into Rusty IDD DTOs,
adds synthetic file nodes and containment edges, resolves symbolic edge targets conservatively, and
records unresolved targets explicitly.

The older fake `codegraph` shim and local Rust AST semantic pass were removed.

## Audit Cuts

After adopting the upstream codegraph crates, the full gate exposed default-path friction that was
cut deliberately:

- `lsh-rs2` was removed from the vendored parser because it pulled audited `libsqlite3-sys` and old
  `rand` advisories. The symbol resolver keeps a dependency-free string-similarity fallback.
- `dotenv` loading was removed from the vendored core because the direct Rusty IDD integration uses
  normal process environment and the crate is unmaintained.
- The parser language registry now retains only grammars compatible with the workspace-pinned
  tree-sitter runtime. Incompatible extractor source remains vendored for future compatible pins or
  feature-gated multi-language work.
- Three vendored watcher timing tests are ignored because filesystem notification timing is outside
  the `rusty-idd knowledge` boundary.
- Vendored license notices are required for the external CodeGraph and Repomix crates. This keeps
  the local audit surface explicit instead of relying on memory of the upstream source.

## Coverage Matrix

| Capability | Status | Notes |
| --- | --- | --- |
| Source collection | Covered | Local ignore-aware Rust file walk for graph indexing. |
| Tree-sitter Rust parsing | Covered | Vendored upstream `codegraph-parser` through the `crates/knowledge` boundary. |
| Symbols, imports, calls, containment | Covered | Upstream parser emits symbols and symbolic edges; local adapter adds file containment and conservative target resolution. |
| Complexity and hotspots | Covered | Upstream complexity and call-link counts feed hotspot scoring. |
| AI-ready packing | Covered | `repomix-core` direct pack integration. |
| Token and file metrics | Covered | Exposed in `PackSummary` and reports. |
| Include/ignore handling | Covered | CLI/API patterns plus repomix project/global config. |
| Compression/comment stripping/line numbers | Covered | CLI/API options. |
| Empty-line stripping/base64 truncation | Covered | Added after audit. |
| Split output/header/instruction file | Covered | Added after audit. |
| Git diff/log context | Covered | CLI/API options. |
| Suspicious/security findings | Covered | Exposed from repomix results. |
| Staleness validation | Covered | `validate` compares source/control-plane fingerprint to `.idd/knowledge/index.json`. |
| Durable local skill | Covered | `.agents/skills/rusty-idd-knowledge/SKILL.md`. |
| MCP servers/daemons | Slice excluded | Excluded only from the old direct in-process knowledge slice; current weave + obscura work must re-evaluate this as a system coordination surface. |
| Clipboard output | Intentionally excluded | Headless agent runs should write explicit files. |
| Remote repo packing | Deferred | Current public interface is workspace-local. Use `/tmp` exploratory packs if added later. |
| Vector/semantic search | Deferred | Keep behind future `knowledge-vector` feature. |
| SurrealDB persistent graph | Deferred | Keep behind future `knowledge-surrealdb` feature. |
| Cloud LLM/embedding providers | Deferred | Keep behind future `knowledge-cloud` feature. |
| Broad multi-language graph parsing | Re-evaluate | This was deferred in the old slice; current Yazelix-backed tree-sitter direction makes broad parser support an active research target. |
| LSH/vector symbol resolution | Deferred | Removed from default path after audit; revisit behind `knowledge-vector` with clean dependencies. |
| Dotenv loading | Intentionally excluded | Use process environment only; no new secret provider in the default integration. |

## Verification Commands

Run the full gate after this audit:

```bash
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo audit --deny warnings
cargo run --bin rusty-idd -- knowledge refresh --workspace .
cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv
cargo run --bin rusty-idd -- validate --workspace .
```

Latest verification completed with `cargo audit --deny warnings` clean, `cargo test --workspace
--locked` reporting 584 passed and 3 ignored vendored watcher tests, and `validate` reporting 0
critical / 0 warning.
