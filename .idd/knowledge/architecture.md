# Architecture Graph

- Workspace fingerprint: `fnv1a64:bd3fd81990e795ba`
- Source graph provider: `codegraph-rust`
- Source graph: 147 files, 9062 nodes, 37154 edges
- Source languages: rust
- Context provider: `repomix-rs`
- Context package: 385 files, 758530 tokens

## Automation Stages

| Stage | Purpose | Surfaces |
|---|---|---|
| `Goal intake and bounded context` | turn repo state and user goal into bounded agent context | surface:repomix-rs, surface:openspec |
| `Architecture mapping` | map source structure, integrations, and impact before implementation | surface:codegraph-rust, surface:repomix-rs |
| `Specification and decisions` | convert architecture map into spec deltas, design, ADRs, and tasks | surface:openspec, surface:audit-manifest |
| `Implementation` | apply graph-informed, spec-backed code changes | surface:codegraph-rust, surface:openspec |
| `Validation and regeneration` | run gates and refresh deterministic control-plane artifacts | surface:codegraph-rust, surface:repomix-rs, surface:audit-manifest |
| `Handoff and merge evidence` | record evidence, rollback, and merge-ready traceability | surface:audit-manifest, surface:openspec |

## Integration Surfaces

| Surface | Kind | Scope | Capabilities |
|---|---|---|---|
| `CodeGraph Rust` | architecture_graph | in-process knowledge indexing | multi-language tree-sitter registry, symbol/import/call/type graph extraction, impact and hotspot evidence |
| `repomix-rs` | context_package | bounded context packing and token policy | compressed context packs, token accounting and top-file metrics, security and suspicious-file signals, git-aware context options |
| `Rusty IDD OpenSpec lifecycle` | lifecycle_control_plane | proposal, spec, design, ADR, task, validation, archive | goal intake, spec delta tracking, ordered implementation tasks, validation and merge evidence |
| `Audit and manifest evidence` | evidence_control_plane | deterministic generated control-plane artifacts | AI_MERGE audit records, ADR traceability, .idd manifest baseline, knowledge artifact freshness |

## Components

| Component | Kind | Files | Nodes | Edges | Languages |
|---|---|---:|---:|---:|---|
| `cli` | crate | 32 | 1228 | 5328 | Rust, rust |
| `core` | crate | 12 | 448 | 2479 | Rust, rust |
| `knowledge` | crate | 1 | 639 | 4653 | Rust, rust |
| `merge-tools` | crate | 1 | 45 | 234 | Rust, rust |
| `runner` | crate | 4 | 770 | 3135 | Rust, rust |
| `spec` | crate | 24 | 461 | 1970 | Rust, rust |
| `tui` | crate | 3 | 1006 | 3985 | Rust, rust |
| `codegraph-core` | external_crate | 39 | 1918 | 9070 | Rust, rust |
| `codegraph-parser` | external_crate | 29 | 1060 | 7485 | Rust, rust |
| `repomix-shared` | external_crate | 2 | 11 | 34 | Rust, rust |

## Edges

| Source | Kind | Target |
|---|---|---|
| `crate:cli` | codegraph:Calls | `crate:core` |
| `crate:cli` | codegraph:Calls | `crate:knowledge` |
| `crate:cli` | codegraph:Imports | `crate:knowledge` |
| `crate:cli` | codegraph:References | `crate:knowledge` |
| `crate:cli` | codegraph:Calls | `crate:merge-tools` |
| `crate:cli` | codegraph:Imports | `crate:merge-tools` |
| `crate:cli` | codegraph:Calls | `crate:runner` |
| `crate:cli` | codegraph:Imports | `crate:runner` |
| `crate:cli` | codegraph:Calls | `crate:spec` |
| `crate:cli` | codegraph:Imports | `crate:spec` |
| `crate:cli` | codegraph:References | `crate:spec` |
| `crate:cli` | codegraph:Calls | `external:codegraph-core` |
| `crate:cli` | codegraph:Calls | `external:codegraph-parser` |
| `crate:core` | codegraph:Calls | `crate:cli` |
| `crate:core` | codegraph:Calls | `crate:spec` |
| `crate:core` | codegraph:Calls | `external:codegraph-core` |
| `crate:knowledge` | codegraph:Calls | `crate:cli` |
| `crate:knowledge` | codegraph:Imports | `crate:cli` |
| `crate:knowledge` | codegraph:Calls | `crate:core` |
| `crate:knowledge` | codegraph:Imports | `crate:core` |
| `crate:knowledge` | codegraph:Calls | `crate:spec` |
| `crate:knowledge` | codegraph:Calls | `external:codegraph-core` |
| `crate:knowledge` | codegraph:Imports | `external:codegraph-core` |
| `crate:knowledge` | codegraph:Calls | `external:codegraph-parser` |
| `crate:knowledge` | codegraph:Imports | `external:codegraph-parser` |
| `crate:merge-tools` | codegraph:Calls | `crate:cli` |
| `crate:merge-tools` | codegraph:Calls | `crate:core` |
| `crate:merge-tools` | codegraph:Calls | `crate:spec` |
| `crate:runner` | codegraph:Calls | `crate:cli` |
| `crate:runner` | codegraph:Calls | `crate:core` |
| `crate:runner` | codegraph:Calls | `crate:spec` |
| `crate:runner` | codegraph:Calls | `external:codegraph-core` |
| `crate:spec` | codegraph:Calls | `crate:cli` |
| `crate:spec` | codegraph:Calls | `external:codegraph-core` |
| `crate:tui` | codegraph:Calls | `crate:core` |
| `crate:tui` | codegraph:Calls | `crate:runner` |
| `crate:tui` | codegraph:Imports | `crate:runner` |
| `crate:tui` | codegraph:References | `crate:runner` |
| `crate:tui` | codegraph:Calls | `crate:spec` |
| `crate:tui` | codegraph:Calls | `external:codegraph-core` |
| `external:codegraph-core` | codegraph:Calls | `crate:cli` |
| `external:codegraph-core` | codegraph:Calls | `crate:core` |
| `external:codegraph-core` | codegraph:Calls | `crate:spec` |
| `external:codegraph-core` | codegraph:References | `crate:spec` |
| `external:codegraph-parser` | codegraph:Calls | `crate:cli` |
| `external:codegraph-parser` | codegraph:Calls | `crate:core` |
| `external:codegraph-parser` | codegraph:Calls | `crate:spec` |
| `external:codegraph-parser` | codegraph:Calls | `external:codegraph-core` |
| `external:codegraph-parser` | codegraph:Imports | `external:codegraph-core` |
| `external:codegraph-parser` | codegraph:References | `external:codegraph-core` |
| `stage:architecture-map` | precedes | `stage:specification` |
| `stage:architecture-map` | uses | `surface:codegraph-rust` |
| `stage:architecture-map` | uses | `surface:repomix-rs` |
| `stage:handoff` | uses | `surface:audit-manifest` |
| `stage:handoff` | uses | `surface:openspec` |
| `stage:implementation` | precedes | `stage:validation` |
| `stage:implementation` | uses | `surface:codegraph-rust` |
| `stage:implementation` | uses | `surface:openspec` |
| `stage:intake` | precedes | `stage:architecture-map` |
| `stage:intake` | uses | `surface:openspec` |
| `stage:intake` | uses | `surface:repomix-rs` |
| `stage:specification` | precedes | `stage:implementation` |
| `stage:specification` | uses | `surface:audit-manifest` |
| `stage:specification` | uses | `surface:openspec` |
| `stage:validation` | precedes | `stage:handoff` |
| `stage:validation` | uses | `surface:audit-manifest` |
| `stage:validation` | uses | `surface:codegraph-rust` |
| `stage:validation` | uses | `surface:repomix-rs` |
| `surface:codegraph-rust` | implemented_by | `external:codegraph-parser` |
| `surface:openspec` | implemented_by | `crate:spec` |

## Findings

- CodeGraph-backed parsing completed without source failures
- repomix context package measured 385 files and 758530 tokens
