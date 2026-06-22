# ADR 0008: Refresh Handoff KB Upstream Mirror

- **Status:** Accepted
- **Date:** 2026-06-22
- **Change:** `refresh-handoff-kb-upstream`
- **Supersedes:** pin from `adr/0006-adopt-handoff-as-upstream-reference.md`

## Context

ADR 0006 adopted `meta/handoff` as a full tracked-file upstream mirror pinned to
commit `7be85fcea3c2454fc3470fc929860afb7ea9864b`. During final trunk proof,
the source handoff repository had advanced to commit
`6365c12fc38f5d7247d81f9fdbd3a55817797904` and now tracks 550 files, including
a `.kb` knowledge surface that was not present in the earlier mirror.

The handoff working tree also had local edits to `.handoff/active.md` and
`.handoff/packets/latest.md`; those edits are not part of committed upstream
HEAD and must not be imported as source truth.

## Decision

Refresh `third_party/upstream/handoff` from handoff's committed HEAD at
`6365c12fc38f5d7247d81f9fdbd3a55817797904`, preserving every tracked file and
excluding Git metadata and uncommitted working-tree edits.

The mirror remains an upstream reference outside the Cargo workspace. Future
adapter work must consume this mirror as source evidence before cutting or
rewriting handoff behavior.

## Consequences

- Rusty IDD now carries handoff's tracked `.kb` knowledge surface alongside the
  previously adopted `.handoff`, `.claude`, `.idea`, `.github`, and `.githooks`
  surfaces.
- The upstream registry pin and file count move to `6365c12` and 550 tracked
  files.
- Dirty source checkout state remains documented as evidence, not imported.
