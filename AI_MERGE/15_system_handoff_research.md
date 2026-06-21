# AI Merge 15: System Handoff Research

Date: 2026-06-21
Branch: `docs/system-handoff-research`
Baseline: `develop` at `4c9987f`

## Purpose

This note records the corrected split between Rusty IDD's product role and the
build/merge process, then captures the current peer-repo evidence for the
system direction the owner clarified.

The immediate correction is that PR #50/#52 knowledge-integration notes were a
local Rusty IDD slice, not the current architecture for tree-sitter,
domain/daemon coordination, or the cross-repo agentic OS direction.

## Corrected Split

### 1. Build and merge process

The build/merge process is the repo hygiene and delivery path:

- adopt-first integration strategy for upstream surfaces;
- branch authority, TDD cuts, rollback notes, and PR evidence;
- `just ci`, `make ci`, workspace tests, docs, audit, validation, smoke tests;
- commit, PR, checks, merge to `develop`, and clean worktree state.

This is necessary infrastructure, but it is not the whole Rusty IDD product.

### 2. What Rusty IDD does

Rusty IDD is the Rust-native intent lifecycle binary. Its current documented
product surface is:

- `rusty-idd scan`, `plan`, `task`, `validate`, and `manifest` for the IDD
  control plane;
- `rusty-idd spec validate`, `archive`, `show`, `sync`, `status`, `next`,
  `adr`, `scaffold`, and `new` for the OpenSpec lifecycle;
- `rusty-idd run <change>` for headless task execution;
- `rusty-idd tui` for the task/OpenSpec terminal UI;
- `rusty-idd knowledge` for generated repo knowledge packs.

Rusty IDD therefore owns the OpenSpec-style lifecycle for plans, ADRs, specs,
tasks, implementation, validation, and automation. Future work should express
tree-sitter updates, domain/daemon coordination, merge execution, and system
handoff integration through that lifecycle instead of treating Rusty IDD as only
a merge process.

## Verified Peer-Revisions

These are the revisions used for this research pass:

| Repo | Local state | Current evidence revision |
|---|---|---|
| `rusty-idd` | clean `develop` baseline plus this docs branch | `4c9987f` |
| `meta/handoff` | branch `fix/windows-ledger-path-and-promote-checkout`, dirty `.idea/handoff.iml` unrelated | `91d430b` |
| `weave` | local `develop` behind `origin/develop` by 4 | `origin/develop` `d67bfaf` |
| `obscura` | local `main` matches `origin/main` | `d8c8487` |
| `yazelix` | local `main` behind `origin/main` by 197 | `origin/main` `9283900` |
| `yazelix-helix` | cloned only under `/tmp` for pinned child-input verification | `5133772` |

## Findings

### Rusty IDD is the lifecycle engine

The Rusty IDD README defines one Rust-native binary unifying the IDD control
plane, the OpenSpec lifecycle engine, and task execution UI. The proposal places
it at the terminus of `user-request -> prompt_hub -> weave+rtk -> rusty-idd`.

That means the durable workflow for future system work should be:

1. capture the system-level intent and constraints;
2. update or create a system ADR when the cross-repo architecture changes;
3. create per-repo designs/specs/tasks for the affected repos;
4. execute implementation through the runner/CLI path;
5. validate, archive/sync, regenerate knowledge, and record merge evidence.

### Handoff is the continuity and fleet execution kernel

`meta/handoff` defines a two-plane model:

- planning lives in git-kb `.kb/`;
- execution truth lives in `.handoff/` plus per-repo and central fleet ledgers.

Its `hf` CLI handles resume, intake, dispatch, claim, checkpoint, sync, drift,
policy checks, fleet status/render, ship, review verdicts, handoff packets, and
sessions. `WorkOrder` is the `handoff.task.v1` envelope, with intent locks and
deterministic synthesis from prompt bundles.

The correct Rusty IDD/handoff relationship is therefore not "copy Rusty IDD into
handoff." It is a system contract:

- Rusty IDD owns intent lifecycle artifacts and validation;
- handoff owns durable session continuity, claims, leases, checkpoints, packets,
  and fleet rollup;
- meta coordinates peer repos as independent repos, not as a monorepo collapse.

The handoff doc `docs/INTEGRATION-RUSTY-IDD.md` still records a historical
`COPY + REFERENCE` plan. Treat it as stale research until a future task replaces
it with a proper Rusty IDD OpenSpec change and a handoff-side ADR/spec.

### Tree-sitter is active through the Yazelix stack

The prior Rusty IDD knowledge notes treated tree-sitter as a postponed or cut
surface. That is stale.

Current Yazelix `origin/main` consumes a child-input stack, including
`yazelix-helix`, `yazelix-zellij`, and `yazelix-zellij-pane-orchestrator`.
The pinned `yazelix-helix` revision carries the tree-sitter surface directly:

- tree-sitter grammar lock/cache docs;
- `grammar_sources.lock.json` grammar source locking;
- `helix-term/build.rs` grammar fetch/compile path;
- syntax AST documentation;
- incremental tree-sitter syntax updates;
- tree-sitter text-object and selection commands.

Rusty IDD must not preserve a blanket "no tree-sitter" assumption. Future parser
work should inspect the pinned Yazelix/Helix contract and upgrade forward from
that evidence.

### Domains and web automation are active through weave plus Obscura

The prior Rusty IDD notes also over-generalized MCP/server/daemon/domain cuts.
That was valid only for the narrow PR #50/#52 local knowledge slice.

Current `weave` `origin/develop` has:

- full CLI/MCP parity for jobs, leases, permissions, schedules, memory,
  orchestrator, and governed web;
- a poll-only durable job board with fenced `attempt_id` updates;
- lease resources with TTL and path conflict checks;
- default-off `--features obscura` governed web access;
- deny-by-default Obscura operation allow-lists;
- optional domain allow-lists plus SSRF/internal-host guards;
- permission/lease/job auditing around browser operations.

Current `obscura` is a Rust headless browser workspace with CDP domains, an MCP
server, V8-backed JS execution, DOM/runtime/network layers, and docs for adding
CDP domains or Web APIs. It is a real system surface, but weave keeps it behind
feature flags and a subprocess/runtime boundary rather than linking it into the
default mesh binary.

Rusty IDD should preserve its `crates/core` std-only default, but it must stop
treating domains/daemons/web as nonexistent. Those surfaces belong in explicit
system ADRs, per-repo specs, and feature-gated integration plans.

## Implications

- Rusty IDD needs a general system ADR layer in addition to per-repo design/spec
  artifacts.
- Meta remains the peer-repo coordinator. The point is not a monorepo; it is a
  coherent system goal spanning independent repos.
- The Rusty IDD/handoff unification should be designed as an artifact and
  workflow contract before code movement.
- The next implementation task should not start by copying crates or files. It
  should start with an OpenSpec change that names the system ADR, the per-repo
  specs, the acceptance criteria, and the verification path.

## Docs Corrected In This Pass

- `adr/0004-knowledge-direct-crate-integration.md`
- `AI_MERGE/11_integration_research_audit_roadmap.md`
- `AI_MERGE/12_knowledge_deep_audit.md`
- `AI_MERGE/14_upstream_full_adoption.md`

Each correction marks the old statements as PR #50/#52 local-slice evidence, not
current whole-system architecture.

## Next OpenSpec Task Shape

Suggested next change id:

`unify-rusty-idd-handoff-system-lifecycle`

Artifacts to produce:

- system ADR: Rusty IDD system lifecycle plus handoff fleet continuity;
- per-repo Rusty IDD spec: system ADR registry, per-repo design/spec/task
  generation, knowledge refresh, validation gates;
- per-repo handoff spec: replace copy/reference Rusty IDD plan with CLI/artifact
  contract and fleet packet integration;
- per-repo weave/obscura spec: feature-gated domain/web automation contract;
- per-repo Yazelix spec: tree-sitter/Helix child-input parser contract;
- tasks: one TDD slice per boundary, with rollback and generated-knowledge
  refresh after each source/control-plane change.

