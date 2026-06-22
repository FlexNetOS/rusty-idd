# adopt-full-handoff-upstream - Design

## Context

The merged architecture package established that Rusty IDD owns the canonical
control plane and consumes handoff whole. Handoff's useful runtime semantics
must be preserved before any adapter, consolidation, cleanup, or retirement
work. The prior package also stated that the first implementation slice should
import or mirror the complete tracked `meta/handoff` surface as an
upstream/reference source.

## Goals

- Preserve the complete tracked handoff repository inside Rusty IDD.
- Keep the mirror outside the Cargo workspace and outside default build
  membership.
- Make the import reproducible by pinning the exact source commit and tracked
  file count.
- Include tracked dotfiles and dot directories from handoff.
- Record source-checkout dirty/untracked state without committing that
  uncommitted state into the upstream mirror.
- Leave future behavior migration to typed Rusty IDD adapters.

## Non-Goals

- No handoff behavior is refactored in this slice.
- No handoff package is added to the Rusty IDD Cargo workspace.
- No untracked `.git`, lock files, local runtime outputs, binary caches, or
  source-checkout dirty changes are promoted into the mirror.
- No deletion of existing Rusty IDD or handoff compatibility surfaces.

## Import Boundary

The mirror is created with:

```bash
git -C /home/drdave/Desktop/meta/handoff archive --format=tar HEAD \
  | tar -x -C third_party/upstream/handoff
```

This preserves every file tracked by the pinned source commit and excludes Git
metadata and untracked local state. That is the right boundary for a durable
adoption baseline because it is reproducible from Git, reviewable in Rusty IDD,
and does not silently convert another repository's local work-in-progress into
Rusty IDD source.

## Source State

At import time, the handoff checkout had local modifications to `Cargo.lock`,
`hf/src/main.rs`, and `scripts/handoff-loop-init.sh`, plus untracked
`.claude/skills/session-relay-*` files. Those are recorded as evidence in
`AI_MERGE/36_handoff_full_adoption/handoff-source-state.md` and are excluded
from the mirror. If those changes become committed handoff behavior, a future
adoption refresh should update the mirror to the new handoff commit.

## Review Findings

The gap hunt found these implementation gaps after ADR 0005:

| Gap | Resolution In This Change |
|---|---|
| No full handoff mirror existed in Rusty IDD | add `third_party/upstream/handoff` |
| No pinned handoff import record existed | update `third_party/upstream/UPSTREAMS.md` and ADR 0006 |
| No tracked-file proof existed | add tracked-file inventory and mirror verification evidence |
| Source checkout dirty state was previously only described | capture exact dirty/untracked state at import time |
| Future adapters lacked a local source baseline | preserve `hf`, `ledger`, `work-order`, `.handoff`, `.claude`, `.idea`, `.github`, docs, scripts, and embedded Rusty IDD subset |

## Next Adapter Gaps

After this adoption, the remaining gaps are typed Rusty IDD adapters and parity
tests for:

- task mint, claim, checkpoint, done, delivery;
- ledger JSONL export/import and witness semantics;
- fleet status and packet rendering;
- policy and drift gates;
- harness-loop compatibility inputs;
- dot-directory ownership validators.

Those are intentionally not implemented in this mirror slice.

## Rollback

Revert this change. Since the mirror is an adoption baseline and is not a
workspace member, rollback removes the mirror and evidence without data
migration.
