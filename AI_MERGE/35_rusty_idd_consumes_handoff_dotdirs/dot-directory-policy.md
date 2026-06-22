# Dot-Directory Policy And Migration Evidence

## Inventory Summary

### Rusty IDD

Observed tracked dot-directory surfaces:

| Surface | Tracked Count | Primary Meaning |
|---|---:|---|
| `.idd` | 29 | Rusty IDD generated control-plane artifacts |
| `.github` | 12 | CI and delivery gates |
| `.codex` | 8 | Codex hooks and repo-local agent policy |
| `.handoff` | 4 | lightweight adopted/compatibility handoff evidence |
| `.githooks` | 3 | local Git hook support |
| `.agents` | 3 | reusable Rusty IDD skills |
| `.claude` | 2 | legacy/source-material agent surface |
| `.vscode` | 1 | editor helper state |

Rusty IDD current generated knowledge before this planning package reported
139 indexed files, 8649 graph nodes, and more than 35400 graph edges.

### `meta/handoff`

Observed packages:

- `work-order`
- `ledger`
- `hf`
- `rusty-idd-cli`
- `rusty-idd-core`
- `rusty-idd-runner`
- `rusty-idd-spec`
- `rusty-idd-tui`

Observed tracked dot-directory surfaces:

| Surface | Tracked Count | Primary Meaning |
|---|---:|---|
| `.handoff` | 113 | task cards, fleet, hooks, decisions, policies, packets, context, deliveries, ledger text evidence |
| `.claude` | 31 | Claude/harness behavior and skills |
| `.idea` | 9 | IDE project and idea/editor state |
| `.github` | 9 | CI and delivery gates |
| `.githooks` | 3 | local Git hook support |

The handoff repository had unrelated local modifications during inventory, so
this planning pass treated it as read-only evidence.

### Parent `meta`

Relevant parent surfaces:

- `.kb`: planning and backlog source; task cards can be minted from KB docs.
- `.handoff`: workspace continuity layer and rendered handoff task/packet state.
- `.idea`: IDE project metadata and idea/editor surface.
- `.claude`, `.codex`, `.agents`, `.github`: agent, enforcement, skill, and CI
  surfaces across the workspace.

### `meta/harness_hub` Lineage

`harness-loop-init` lays down `.handoff/loop` and can defer to the handoff kernel
when `hf` is present. The kernel-backed loop renders packets and task state from
handoff ledger mechanics. In the corrected Rusty IDD architecture, that lineage
is compatibility evidence and migration source material, not the canonical
current control plane.

## Authority Classes

| Authority Class | Surfaces | Rule |
|---|---|---|
| Canonical control plane | `.idd`, OpenSpec, `adr`, generated knowledge, manifest, validation | owns current intent, readiness, decision, and validation state |
| Adopted runtime evidence | `.handoff`, handoff ledger JSONL/events, task cards, packets, deliveries, fleet views | preserved through typed Rusty IDD adapters before cleanup |
| Workspace knowledge input | `.kb` | feeds planning and task discovery, but cannot bypass `.idd`/OpenSpec gates |
| Idea/editor input | `.idea` | may hold concepts and IDE metadata; must be promoted before implementation |
| Compatibility source material | `.claude`, `meta/harness_hub`, `.handoff/loop` | source for behavior adoption and risk discovery only |
| Enforcement and reuse | `.codex`, `.agents`, `.githooks` | enforces or teaches workflow, but does not approve product requirements |
| Remote delivery gate | `.github` | validates and protects branches after local readiness |
| Local cache/tool state | untracked caches, binary ledgers, lock files, editor caches | no workflow authority |

## State Precedence

1. Git-tracked Rusty IDD source and canonical planning artifacts.
2. `.idd` goals, knowledge, plan-context, manifest, and validation.
3. OpenSpec changes and accepted ADRs.
4. Adopted handoff ledger/task/packet/delivery/fleet evidence through typed
   adapters.
5. `.kb` planning and backlog source material.
6. `.idea` concept and editor input.
7. `.claude`, `harness_hub`, and `.handoff/loop` compatibility traces.
8. Local caches, binary state, locks, and historical relay prose.

## First Implementation Slice

Adopt the complete tracked `meta/handoff` surface into Rusty IDD as an upstream
reference or migration source, excluding:

- `.git` metadata;
- untracked local runtime outputs;
- local lock files;
- binary cache state such as redb ledger files unless a future ADR explicitly
  promotes them.

This slice should add:

- an import inventory tied to the source commit;
- an adapter-boundary map for `hf`, `ledger`, and `work-order`;
- parity targets for task mint, claim, checkpoint, done, delivery, fleet,
  policy, drift, and ledger JSONL export/import;
- manifest and validation evidence.

It should not delete or refactor handoff behavior.

## Migration Phases

1. Planning package: current change.
2. Adopt-first mirror/reference: complete tracked handoff surface preserved.
3. Typed adapters: Rusty IDD owns task, ledger, delivery, fleet, policy, and
   drift boundaries.
4. Parity gates: compare handoff-native behavior to Rusty IDD adapter behavior.
5. Dot-directory normalization: validators prove which surface owns each state
   class.
6. Compatibility retirement: freeze or retire legacy `.claude`,
   `harness_hub`, and `.handoff/loop` traces only after parity.

## Rollback

Revert this planning package. No handoff source is moved and no state migration
is performed in this change.

## Risks

| Risk | Mitigation |
|---|---|
| Dot directories continue to multiply as peer authorities | require authority class before adding or promoting a dot-directory surface |
| Handoff behavior is guessed or cherry-picked | adopt tracked handoff whole before cutting behavior |
| Binary ledger/cache state becomes canonical | keep canonical evidence in tracked text, typed events, generated artifacts, and adapter validation |
| Legacy harness traces override current Rusty IDD intent | enforce state precedence in ADR, docs, and validators |
| Planning artifacts drift from generated graphs | refresh `.idd/knowledge/*`, diagrams, manifest, and OpenSpec status before merge |
