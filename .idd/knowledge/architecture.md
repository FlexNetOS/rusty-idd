# Architecture Graph

- Workspace fingerprint: `fnv1a64:ba941a3726f7a678`
- Source graph provider: `codegraph-rust`
- Source graph: 458 files, 30935 nodes, 113106 edges
- Source languages: javascript, python, rust
- Context provider: `repomix-rs`
- Context package: 343 files, 246727 tokens

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
| `cli` | crate | 34 | 1371 | 5852 | Rust, rust |
| `core` | crate | 12 | 458 | 2438 | Rust, rust |
| `knowledge` | crate | 1 | 639 | 4653 | Rust, rust |
| `merge-tools` | crate | 1 | 50 | 241 | Rust, rust |
| `runner` | crate | 4 | 785 | 3091 | Rust, rust |
| `spec` | crate | 24 | 461 | 1552 | Rust, rust |
| `tui` | crate | 3 | 1006 | 3985 | Rust, rust |
| `work-order` | crate | 2 | 165 | 527 | Rust, rust |
| `codegraph-core` | external_crate | 39 | 1919 | 9029 | Rust, rust |
| `codegraph-parser` | external_crate | 29 | 1060 | 7486 | Rust, rust |
| `repomix-shared` | external_crate | 2 | 11 | 34 | Rust, rust |
| `handoff-core` | repo_surface | 1 | 66 | 288 | Rust, rust |
| `handoff-drift` | repo_surface | 1 | 168 | 615 | Rust, rust |
| `handoff-fleet` | repo_surface | 1 | 188 | 941 | Rust, rust |
| `handoff-gatekeeper` | repo_surface | 1 | 73 | 337 | Rust, rust |
| `handoff-hooks` | repo_surface | 1 | 75 | 218 | Rust, rust |
| `handoff-index` | repo_surface | 1 | 92 | 329 | Rust, rust |
| `handoff-intake` | repo_surface | 1 | 65 | 201 | Rust, rust |
| `handoff-lease` | repo_surface | 1 | 49 | 161 | Rust, rust |
| `handoff-policy` | repo_surface | 3 | 128 | 309 | Rust, rust |
| `handoff-route` | repo_surface | 1 | 37 | 161 | Rust, rust |
| `handoff-schema` | repo_surface | 1 | 68 | 244 | Rust, rust |
| `handoff-secrets` | repo_surface | 1 | 28 | 72 | Rust, rust |
| `handoff-test-support` | repo_surface | 1 | 3 | 11 | Rust, rust |
| `hf` | repo_surface | 13 | 1644 | 6837 | Rust, rust |
| `imports` | repo_surface | 236 | 14297 | 55038 | JavaScript, Python, Rust, javascript, python, rust |
| `ledger` | repo_surface | 5 | 355 | 1865 | Rust, rust |
| `spike` | repo_surface | 2 | 12 | 50 | JavaScript, javascript |
| `vendor` | repo_surface | 33 | 1882 | 12332 | Rust, rust |
| `work-order` | repo_surface | 3 | 229 | 748 | Rust, rust |

## Edges

| Source | Kind | Target |
|---|---|---|
| `crate:cli` | codegraph:Calls | `crate:knowledge` |
| `crate:cli` | codegraph:Imports | `crate:knowledge` |
| `crate:cli` | codegraph:References | `crate:knowledge` |
| `crate:cli` | codegraph:Calls | `crate:merge-tools` |
| `crate:cli` | codegraph:Imports | `crate:merge-tools` |
| `crate:cli` | codegraph:Calls | `external:codegraph-core` |
| `crate:cli` | codegraph:Calls | `external:codegraph-parser` |
| `crate:cli` | codegraph:Calls | `repo:imports` |
| `crate:cli` | codegraph:Imports | `repo:imports` |
| `crate:cli` | codegraph:References | `repo:imports` |
| `crate:cli` | codegraph:Calls | `repo:vendor` |
| `crate:cli` | codegraph:Imports | `repo:vendor` |
| `crate:core` | codegraph:Calls | `crate:cli` |
| `crate:core` | codegraph:Calls | `external:codegraph-core` |
| `crate:core` | codegraph:Calls | `repo:imports` |
| `crate:core` | codegraph:References | `repo:imports` |
| `crate:core` | codegraph:Calls | `repo:vendor` |
| `crate:knowledge` | codegraph:Calls | `crate:cli` |
| `crate:knowledge` | codegraph:Calls | `crate:core` |
| `crate:knowledge` | codegraph:Imports | `crate:core` |
| `crate:knowledge` | codegraph:Calls | `external:codegraph-core` |
| `crate:knowledge` | codegraph:Imports | `external:codegraph-core` |
| `crate:knowledge` | codegraph:Calls | `external:codegraph-parser` |
| `crate:knowledge` | codegraph:Imports | `external:codegraph-parser` |
| `crate:knowledge` | codegraph:Calls | `repo:imports` |
| `crate:knowledge` | codegraph:References | `repo:imports` |
| `crate:knowledge` | codegraph:Calls | `repo:vendor` |
| `crate:merge-tools` | codegraph:References | `repo:imports` |
| `crate:merge-tools` | codegraph:Calls | `repo:vendor` |
| `crate:runner` | codegraph:Calls | `crate:cli` |
| `crate:runner` | codegraph:Calls | `external:codegraph-core` |
| `crate:runner` | codegraph:Calls | `repo:imports` |
| `crate:runner` | codegraph:References | `repo:imports` |
| `crate:runner` | codegraph:Calls | `repo:vendor` |
| `crate:runner` | codegraph:References | `repo:vendor` |
| `crate:spec` | codegraph:Calls | `external:codegraph-core` |
| `crate:spec` | codegraph:Calls | `repo:imports` |
| `crate:spec` | codegraph:References | `repo:imports` |
| `crate:spec` | codegraph:Calls | `repo:vendor` |
| `crate:tui` | codegraph:Calls | `external:codegraph-core` |
| `crate:tui` | codegraph:Calls | `repo:imports` |
| `crate:tui` | codegraph:References | `repo:imports` |
| `crate:tui` | codegraph:Calls | `repo:vendor` |
| `crate:tui` | codegraph:References | `repo:vendor` |
| `crate:work-order` | codegraph:Calls | `repo:imports` |
| `crate:work-order` | codegraph:References | `repo:imports` |
| `crate:work-order` | codegraph:Calls | `repo:vendor` |
| `external:codegraph-core` | codegraph:Calls | `crate:cli` |
| `external:codegraph-core` | codegraph:Calls | `repo:imports` |
| `external:codegraph-core` | codegraph:Imports | `repo:imports` |
| `external:codegraph-core` | codegraph:References | `repo:imports` |
| `external:codegraph-core` | codegraph:Calls | `repo:vendor` |
| `external:codegraph-core` | codegraph:Imports | `repo:vendor` |
| `external:codegraph-core` | codegraph:References | `repo:vendor` |
| `external:codegraph-parser` | codegraph:Calls | `crate:cli` |
| `external:codegraph-parser` | codegraph:Calls | `external:codegraph-core` |
| `external:codegraph-parser` | codegraph:Imports | `external:codegraph-core` |
| `external:codegraph-parser` | codegraph:References | `external:codegraph-core` |
| `external:codegraph-parser` | codegraph:Calls | `repo:imports` |
| `external:codegraph-parser` | codegraph:References | `repo:imports` |
| `external:codegraph-parser` | codegraph:Calls | `repo:vendor` |
| `external:codegraph-parser` | codegraph:Imports | `repo:vendor` |
| `external:codegraph-parser` | codegraph:References | `repo:vendor` |
| `external:repomix-shared` | codegraph:References | `repo:imports` |
| `repo:handoff-core` | codegraph:Calls | `repo:imports` |
| `repo:handoff-core` | codegraph:References | `repo:imports` |
| `repo:handoff-core` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-core` | codegraph:Calls | `repo:work-order` |
| `repo:handoff-drift` | codegraph:Calls | `repo:imports` |
| `repo:handoff-drift` | codegraph:References | `repo:imports` |
| `repo:handoff-drift` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-fleet` | codegraph:Calls | `repo:handoff-core` |
| `repo:handoff-fleet` | codegraph:Calls | `repo:imports` |
| `repo:handoff-fleet` | codegraph:References | `repo:imports` |
| `repo:handoff-fleet` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-gatekeeper` | codegraph:Calls | `repo:handoff-core` |
| `repo:handoff-gatekeeper` | codegraph:Imports | `repo:handoff-core` |
| `repo:handoff-gatekeeper` | codegraph:Calls | `repo:imports` |
| `repo:handoff-gatekeeper` | codegraph:References | `repo:imports` |
| `repo:handoff-gatekeeper` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-hooks` | codegraph:Calls | `repo:imports` |
| `repo:handoff-hooks` | codegraph:References | `repo:imports` |
| `repo:handoff-hooks` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-index` | codegraph:Calls | `repo:imports` |
| `repo:handoff-index` | codegraph:References | `repo:imports` |
| `repo:handoff-index` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-intake` | codegraph:Calls | `repo:imports` |
| `repo:handoff-intake` | codegraph:References | `repo:imports` |
| `repo:handoff-intake` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-lease` | codegraph:Calls | `repo:imports` |
| `repo:handoff-lease` | codegraph:References | `repo:imports` |
| `repo:handoff-lease` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-policy` | codegraph:Calls | `repo:imports` |
| `repo:handoff-policy` | codegraph:References | `repo:imports` |
| `repo:handoff-route` | codegraph:References | `repo:imports` |
| `repo:handoff-schema` | codegraph:Calls | `repo:imports` |
| `repo:handoff-schema` | codegraph:References | `repo:imports` |
| `repo:handoff-schema` | codegraph:Calls | `repo:vendor` |
| `repo:handoff-secrets` | codegraph:References | `repo:imports` |
| `repo:handoff-test-support` | codegraph:Calls | `external:codegraph-core` |
| `repo:hf` | codegraph:Calls | `crate:cli` |
| `repo:hf` | codegraph:Calls | `external:codegraph-core` |
| `repo:hf` | codegraph:Calls | `repo:handoff-core` |
| `repo:hf` | codegraph:Imports | `repo:handoff-core` |
| `repo:hf` | codegraph:Calls | `repo:handoff-fleet` |
| `repo:hf` | codegraph:Calls | `repo:handoff-index` |
| `repo:hf` | codegraph:Calls | `repo:imports` |
| `repo:hf` | codegraph:References | `repo:imports` |
| `repo:hf` | codegraph:Calls | `repo:vendor` |
| `repo:hf` | codegraph:References | `repo:vendor` |
| `repo:imports` | codegraph:Calls | `crate:cli` |
| `repo:imports` | codegraph:Calls | `external:codegraph-core` |
| `repo:imports` | codegraph:Imports | `external:codegraph-core` |
| `repo:imports` | codegraph:References | `external:codegraph-core` |
| `repo:imports` | codegraph:Calls | `external:codegraph-parser` |
| `repo:imports` | codegraph:Calls | `repo:vendor` |
| `repo:imports` | codegraph:Imports | `repo:vendor` |
| `repo:imports` | codegraph:References | `repo:vendor` |
| `repo:ledger` | codegraph:Calls | `crate:cli` |
| `repo:ledger` | codegraph:Calls | `external:codegraph-core` |
| `repo:ledger` | codegraph:Calls | `repo:imports` |
| `repo:ledger` | codegraph:References | `repo:imports` |
| `repo:ledger` | codegraph:Calls | `repo:vendor` |
| `repo:ledger` | codegraph:References | `repo:vendor` |
| `repo:spike` | codegraph:Calls | `repo:imports` |
| `repo:vendor` | codegraph:Calls | `crate:cli` |
| `repo:vendor` | codegraph:Calls | `external:codegraph-core` |
| `repo:vendor` | codegraph:Calls | `repo:imports` |
| `repo:vendor` | codegraph:Imports | `repo:imports` |
| `repo:vendor` | codegraph:References | `repo:imports` |
| `repo:work-order` | codegraph:Calls | `external:codegraph-core` |
| `repo:work-order` | codegraph:Calls | `repo:imports` |
| `repo:work-order` | codegraph:References | `repo:imports` |
| `repo:work-order` | codegraph:Calls | `repo:vendor` |
| `repo:work-order` | codegraph:References | `repo:vendor` |
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
- repomix context package measured 343 files and 246727 tokens
