# Rusty IDD + Handoff Single-Repo Architecture

## Goal

```bash
rusty-idd --goal-file [rusty-idd and handoff must be combined into a sinlge repo]
```

Task card: `KBTASK-RUSTY-IDD-HANDOFF-SINGLE-REPO-ARCHITECTURE`

OpenSpec change: `openspec/changes/plan-handoff-single-repo-architecture`

ADR: `adr/0004-handoff-outer-single-repo.md`

## Decision

Use **handoff as the outer canonical repository**, with current Rusty IDD
embedded as explicit peer workspace packages and control-plane artifacts.

This is a handoff-outer, boundary-preserving single repo. It is not a flattening
of Rusty IDD into `hf`, and it is not the stale `crates/intent-analysis`
copy/reference plan.

## Why This Shape

Handoff is the operational truth:

- `hf` owns claim, checkpoint, done, delivery, status, policy, fleet, session,
  drift, task minting, and task-card flow.
- `ledger` owns witnessed event history, redb storage, RVF overlay, export/import
  JSONL, leases, rollup provenance, and witness verification.
- `work-order` owns the `handoff.task.v1` schema, intent lock, and task-card
  contract.
- `.handoff/` owns durable execution state, fleet capsules, deliveries, packets,
  policies, hooks, tasks, and committed ledger JSONL.

Rusty IDD is the planning and validation truth:

- `rusty-idd knowledge` owns generated index, report, architecture, diagrams,
  system architecture, operating model, integration plan/status/owners/readiness,
  and plan-context artifacts.
- `rusty-idd spec` owns OpenSpec validation, archive, status, ADR, scaffold, and
  integration planning.
- `rusty-idd codex` owns repo-local Codex workflow checks, env checks, runtime
  audits, system audits, and model-loop emission.
- `.idd/`, OpenSpec, ADR, and AI_MERGE evidence own readiness and audit records.

The combined repo needs both, and handoff is the better outer repo because it
already carries the execution kernel and already embeds an older Rusty IDD
subset.

## Inventory

### Rusty IDD Current Repo

- Branch/head: `feature/handoff-single-repo-architecture` at
  `f4df9a40468b5d60eb38b196a305010fbe9253ca`.
- Workspace packages:
  - `rusty-idd-cli`
  - `rusty-idd-core`
  - `rusty-idd-knowledge`
  - `rusty-idd-merge-tools`
  - `rusty-idd-runner`
  - `rusty-idd-spec`
  - `rusty-idd-tui`
  - `codegraph-core`
  - `codegraph-parser`
  - `repomix-shared`
- Generated knowledge after this planning pass:
  - indexed source files: 139
  - graph nodes: 8649
  - graph edges: 35402
  - manifest entries: 2814
- Key current-only packages absent from handoff:
  - `crates/knowledge`
  - `crates/merge-tools`
- Key current-only workflow files absent from handoff include:
  - `crates/cli/src/commands/knowledge.rs`
  - `crates/cli/src/commands/codex.rs`
  - `crates/cli/src/commands/merge_tools.rs`
  - `crates/cli/src/commands/spec_plan_integration.rs`
  - `docs/rusty-idd/architecture-diagrams.md`
  - `docs/rusty-idd/codex-environment.md`

### Handoff Current Repo

- Branch/head: `develop` at `e920fec414e18eca677cc2484df5a0689adba65a`.
- Workspace packages:
  - `work-order`
  - `ledger`
  - `hf`
  - `rusty-idd-cli`
  - `rusty-idd-core`
  - `rusty-idd-runner`
  - `rusty-idd-spec`
  - `rusty-idd-tui`
- Handoff-only packages absent from Rusty IDD:
  - `hf`
  - `ledger`
  - `work-order`
- Handoff embeds 269 files across the Rusty IDD subset paths.
- Current Rusty IDD has 348 files across matching paths, with 79 current-only
  files and 0 handoff-only files in that subset comparison.

## Feature Matrix

| Capability | Rusty IDD current | Handoff current | Target owner |
|---|---|---|---|
| Intent goal file and plan context | yes | partial/stale subset | Rusty IDD package inside handoff |
| OpenSpec lifecycle | yes | embedded older subset | Rusty IDD package inside handoff |
| ADR lifecycle | yes | docs ADRs and embedded older subset | both, with root policy in handoff |
| Knowledge graph/index/report | yes | no | Rusty IDD package inside handoff |
| Architecture diagrams | yes | no | Rusty IDD package inside handoff |
| Codex workflow checks | yes | no | Rusty IDD package inside handoff |
| Merge-tool package | yes | no | Rusty IDD package inside handoff |
| Task card schema | no | yes | handoff `work-order` |
| Claims/checkpoints/done | no | yes | handoff `hf` |
| Ledger/witness chain | no | yes | handoff `ledger` |
| Delivery endpoint | no | yes | handoff `hf` |
| Fleet status/render/sync | no | yes | handoff `hf` |
| Session hooks/relay | partial via Codex hooks | yes | handoff owns runtime hooks; Rusty IDD owns planning hooks |

## Contract Map

```mermaid
flowchart LR
    goal["Goal / goal-file"] --> plan["Rusty IDD plan-context"]
    plan --> spec["OpenSpec proposal/spec/design/tasks"]
    spec --> adr["ADR decision"]
    adr --> task["handoff WorkOrder task card"]
    task --> claim["hf claim + lease"]
    claim --> checkpoint["hf checkpoint"]
    checkpoint --> validate["Rusty IDD + handoff gates"]
    validate --> done["hf done + delivery"]
    done --> evidence["AI_MERGE / .idd / .handoff evidence"]
```

| Contract | Producer | Consumer | Preservation Rule |
|---|---|---|---|
| Goal file | Rusty IDD | plan-context, OpenSpec | durable under `.idd/goals` |
| Plan context | Rusty IDD knowledge | OpenSpec/ADR/task planning | generated and freshness-checked |
| OpenSpec change | Rusty IDD spec | implementation gates | proposal/spec/design/tasks before writes |
| ADR | Rusty IDD spec + repo docs | migration and rollback | immutable accepted decisions |
| WorkOrder | handoff `work-order` | `hf` and fleet | schema-valid `handoff.task.v1` |
| Claim/lease | handoff `hf`/`ledger` | agents and policy gates | witnessed in ledger |
| Checkpoint | handoff `hf`/`ledger` | resume/handoff/done | witnessed in ledger and packets |
| Done/delivery | handoff `hf`/`delivery` | prompt/front-door/fleet | delivery artifact plus ledger event |
| Manifest | Rusty IDD core | validation/CI | deterministic `.idd/MANIFEST.tsv` |

## Candidate Comparison

| Shape | Benefits | Rejected Risks / Notes |
|---|---|---|
| Rusty IDD embedded in handoff | Matches existing handoff repo shape; preserves `hf`, ledger, and fleet truth; satisfies single-repo goal with least execution-state churn | Selected only if Rusty IDD remains modular peer packages, not absorbed into `hf` |
| Handoff embedded in Rusty IDD | Makes planning engine outermost; simpler for `.idd`/OpenSpec defaults | Rejected because witnessed execution state would be subordinate to planning code and more likely to be guessed or flattened |
| New peer-only repo | Cleanest theoretical boundary | Rejected because it creates a third coordination surface and delays consolidation |

## Stale/Current Evidence Reconciliation

`handoff/docs/INTEGRATION-RUSTY-IDD.md` says to copy Rusty IDD into
`crates/intent-analysis` and call it by subprocess. Later Rusty IDD evidence
corrected that as stale while the repos were independent. The new owner goal now
requires one repo, but the stale copy/reference mechanism is still not the right
implementation path.

The corrected implementation path is adopt-first:

1. bring current Rusty IDD forward intact enough in handoff to build and
   diagnose;
2. wire current Rusty IDD crates as workspace peers;
3. cut only duplicate/stale surfaces after parity tests pass.

## Migration Phases

### Phase 0: Planning Package

This PR. Produce the goal file, OpenSpec change, generated artifacts, ADR, and
evidence. Do not move handoff code.

### Phase 1: Adopt Current Rusty IDD Into Handoff

In the handoff repo, import current Rusty IDD surfaces with provenance:

- upgrade existing `crates/cli`, `crates/core`, `crates/runner`, `crates/spec`,
  and `crates/tui` to match current Rusty IDD;
- add `crates/knowledge`;
- add `crates/merge-tools`;
- add required external compatibility crates or upstream pins;
- keep `hf`, `ledger`, and `work-order` untouched except for explicit wiring.

### Phase 2: Root Control-Plane Unification

Add root `.idd`, OpenSpec, ADR, AI_MERGE, generated diagrams, and manifest
freshness to handoff. Handoff root becomes the combined repo root for planning
and witnessed execution.

### Phase 3: CLI Contract Wiring

Keep two CLIs:

- `hf` for execution state and fleet continuity;
- `rusty-idd` for planning, knowledge, OpenSpec, validation, and Codex gates.

Add contract checks that prove a Rusty IDD goal/OpenSpec task maps to a handoff
WorkOrder claim/checkpoint/done trail.

### Phase 4: De-duplicate And Retire

Only after parity:

- remove stale duplicate handoff Rusty IDD files;
- update stale docs;
- archive or freeze the standalone Rusty IDD repo as historical source, mirror,
  or package source.

## First Implementation Slice

Base repo: `handoff`.

Branch from: `develop`.

Smallest useful slice:

1. Create a handoff task card and OpenSpec/ADR package for Phase 1.
2. Import current Rusty IDD `knowledge` and `merge-tools` packages into handoff.
3. Upgrade handoff's existing Rusty IDD CLI command wiring only enough to expose
   `rusty-idd knowledge` and `rusty-idd merge-tools`.
4. Run:
   - `cargo test -p rusty-idd-cli`
   - `cargo test -p rusty-idd-knowledge`
   - `cargo test -p rusty-idd-merge-tools`
   - `cargo test -p hf`
   - handoff CI-equivalent gates
5. Leave source deletion/renaming for a later parity-proven slice.

## Risks

| Risk | Mitigation |
|---|---|
| Handoff has path dependencies on sibling RuVector/envctl | Keep CI clone layout and do not move those dependencies in Phase 1 |
| Rusty IDD generated system-root commands can be wrong from nested worktrees | Use absolute meta root during generation; later add a resolver or env-driven system root |
| Existing handoff Rusty IDD subset is stale | Adopt current Rusty IDD before deleting stale code |
| `.handoff` state is durable and can churn | Preserve JSONL committed truth and keep binary ledger cache ignored |
| Two CLIs can confuse users | Keep `hf` execution-only and `rusty-idd` planning/validation-only in docs and tests |
| Standalone Rusty IDD users may break | Keep standalone repo until handoff proves parity and publish migration notes |

## Rollback

- Revert the handoff migration PR.
- Keep standalone Rusty IDD unchanged until parity is proven.
- Re-run handoff `cargo test --workspace`, drift, fmt/clippy, and audit.
- Re-run Rusty IDD generated artifact checks before attempting the next slice.

## Validation Evidence For This Planning Change

- Created worktree:
  `/home/drdave/Desktop/meta/rusty-idd/.worktrees/handoff-single-repo-architecture`
- Created and claimed task:
  `KBTASK-RUSTY-IDD-HANDOFF-SINGLE-REPO-ARCHITECTURE`
- Generated:
  - `.idd/knowledge/index.json`
  - `.idd/knowledge/report.md`
  - `.idd/knowledge/architecture.{json,md}`
  - `.idd/knowledge/system-architecture.{json,md}`
  - `.idd/knowledge/operating-model.{json,md}`
  - `.idd/knowledge/integration-plan.{json,md}`
  - `.idd/knowledge/integration-status.{json,md}`
  - `.idd/knowledge/integration-owners.{json,md}`
  - `.idd/knowledge/integration-readiness.{json,md}`
  - `.idd/knowledge/plan-context.{json,md}`
  - `docs/rusty-idd/architecture-diagrams.md`
  - `.idd/MANIFEST.tsv`
- Corrected generation root for nested worktree:
  `--system-root /home/drdave/Desktop/meta`
- Fixed `Justfile` plan-context recipes to support `RUSTY_IDD_GOAL_FILE` so
  Markdown goal files with backticked command text are not re-interpreted by
  the shell during `just ci`.
- Validation:
  - `RUSTY_IDD_CHANGE=plan-handoff-single-repo-architecture RUSTY_IDD_GOAL_FILE=.idd/goals/rusty-idd-handoff-single-repo.md rtk just ci`
    passed build, tests, validation, freshness checks, Codex audits, fmt,
    clippy, and cargo audit.
  - `cargo run --quiet --bin rusty-idd -- spec status openspec/changes/plan-handoff-single-repo-architecture`
    reported all artifacts done and ready to archive.
  - `cargo run --quiet --bin rusty-idd -- spec validate --all` reported 75
    passed, 0 failed.
  - `git diff --check` passed.
  - Changed-file secret pattern scan returned `secret_scan:no_matches`.
