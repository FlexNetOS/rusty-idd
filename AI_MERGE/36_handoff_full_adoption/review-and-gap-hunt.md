# Review And Gap Hunt

## Scope Reviewed

- ADR 0005: `adr/0005-rusty-idd-consumes-handoff-dotdirs.md`
- Dot-directory architecture: `docs/rusty-idd/dot-directory-architecture.md`
- Prior evidence package:
  `AI_MERGE/35_rusty_idd_consumes_handoff_dotdirs/`
- Handoff source checkout: `/home/drdave/Desktop/meta/handoff`
- Rusty IDD upstream mirror policy:
  `third_party/upstream/UPSTREAMS.md`
- Existing adopt-first precedent:
  `openspec/changes/adopt-grit-full-integration`

## Findings

| Finding | Severity | Resolution |
|---|---|---|
| ADR 0005 required the first implementation slice to import or mirror the complete tracked `meta/handoff` surface, but Rusty IDD did not yet contain that mirror. | high | Added `third_party/upstream/handoff` from the pinned tracked source commit. |
| There was no pinned handoff upstream record in `third_party/upstream/UPSTREAMS.md`. | high | Added the handoff URL, commit, tracked file count, and mirror path. |
| There was no tracked-file proof that the whole handoff repo was adopted. | high | Added `mirror-verification.md` and `handoff-tracked-files.md`; source tracked count and mirror count both equal 533. |
| Final post-merge proof found the mirror count was 524 because `.idea/*` was ignored locally and absent from the committed mirror even though handoff tracks nine `.idea` files. | high | Force-added only the nine tracked handoff `.idea` files; excluded source-local `.idea/workspace.xml`; reran source-vs-mirror path diff until clean. |
| The source handoff checkout needed an explicit source-state record. | medium | Recorded clean source state and imported the complete tracked `HEAD` mirror while excluding only Git metadata. |
| Future handoff behavior still lacks Rusty IDD-owned typed adapters. | medium | Left as the next slice; this adoption intentionally creates the baseline before adapter cuts. |
| The mirror contains handoff's embedded older Rusty IDD subset. | low | Preserved as source evidence; not added to Cargo workspace and not compiled by default. |

## Code Review Result

This change does not alter Rusty IDD runtime code. The primary correctness risk
is adoption completeness rather than behavior regression. The mirror was
imported with `git archive` from source commit
`7be85fcea3c2454fc3470fc929860afb7ea9864b`, and the tracked source count
matches the mirror file count exactly.

The full handoff repository is now present as source evidence, including:

- `hf`
- `ledger`
- `work-order`
- `crates/`
- `.handoff/`
- `.claude/`
- `.idea/`
- `.github/`
- `.githooks/`
- docs, scripts, schemas, task cards, fleet capsules, packets, policy, and
  ledger text evidence

## Remaining Work

The next implementation slice should add Rusty IDD-owned adapters and parity
tests for `hf`, `ledger`, `work-order`, and durable `.handoff` semantics. It
should start from `third_party/upstream/handoff`, not from memory or a sibling
checkout.
