# adopt-grit-full-integration - Design

## Context

Grit is a peer repository in the meta workspace used for agent-run/session
coordination, lock-store behavior, backends, benchmarks, and test projects.
Rusty IDD needs a stable upstream reference so later graph planning can reason
about Grit from local evidence without mutating the Grit repository or relying
on partial notes.

Existing Rusty IDD upstream mirrors live in `third_party/upstream/`, are marked
as vendored for GitHub linguist, and are documented in `third_party/upstream/UPSTREAMS.md`.
Those mirrors are not Cargo workspace members; they are adoption baselines for
audits, rollback, and future consolidation work.

## Goals

- Adopt Grit as a full tracked-file upstream snapshot.
- Keep the adoption read-only with respect to Grit behavior.
- Preserve scan, plan, OpenSpec, ADR, task, evidence, knowledge, diagram, and
  manifest artifacts.
- Make the import reproducible from the pinned Grit commit.

## Non-Goals

- No Grit code changes.
- No Rusty IDD runtime refactor.
- No dependency downgrade, feature cut, or compatibility shim.
- No host-service, daemon, or user-global tool installation.
- No attempt to make the mirror a Cargo workspace member.

## Decisions

1. Import Grit with `git archive` from the clean local `FlexNetOS/grit` checkout
   at commit `57b60842d71145c271b994bb7a8c33c3bca42dfe`.
2. Store the mirror at `third_party/upstream/grit`, matching the existing
   upstream mirror convention.
3. Preserve tracked dotfiles and directories from Grit, while excluding Git
   metadata and untracked local outputs by relying on Git's tracked-file archive.
4. Keep generated scan and plan artifacts under
   `AI_MERGE/34_grit_full_integration/` so `rusty-idd plan` can emit its full
   workspace without overwriting Rusty IDD root control files.
5. Raise the Rusty IDD generated-artifact pack ceiling from 160k to 400k tokens.
   This applies to deterministic report/architecture generation only; the
   default ad hoc `knowledge pack --max-tokens` behavior remains conservative.
6. Exclude the generated `docs/rusty-idd/architecture-diagrams.md` artifact from
   the workspace fingerprint used by knowledge staleness checks. Without this,
   `knowledge refresh -> diagrams -> validate` can make the knowledge index look
   stale immediately.
7. Regenerate knowledge, system architecture, operating model, integration
   plan/status/owners/readiness, diagrams, validation, and manifest after the
   mirror is present.

## Alternatives Considered

- **Submodule**: rejected for this slice because Rusty IDD's existing upstream
  mirrors are tracked snapshots and the owner asked for a full-depth adoption
  artifact, not a moving external pointer.
- **Sparse copy**: rejected because it would violate the full features,
  dotfiles, directories, and no cherry-picking constraints.
- **Direct code integration**: deferred because this slice is the as-is
  adoption baseline; any code integration requires a separate evidenced change.

## Risks / Trade-offs

- The mirror increases repository size, especially through binary assets and
  benchmark fixtures. This is accepted because the task requires a full
  tracked-file adoption baseline.
- The generated control-plane artifact set now exceeds the old 160k internal
  pack ceiling. The ceiling is raised only for deterministic generated reports,
  not for arbitrary user-requested context packs.
- Future contributors could mistake the mirror for live Rusty IDD code. The
  ledger and ADR explicitly keep it outside the Cargo workspace and identify it
  as a reference surface.
- The scanner and manifest intentionally skip local build/cache folders. That
  does not weaken the adoption boundary because `git archive` imports tracked
  upstream files, not generated local outputs.

## Migration Plan

1. Generate the goal-file planning context.
2. Run Rusty IDD scan and plan commands against Rusty IDD and Grit.
3. Import the full tracked Grit snapshot into `third_party/upstream/grit`.
4. Update the upstream mirror ledger and adoption evidence.
5. Refresh generated knowledge, diagrams, validation, and manifest artifacts.

## Rollback

Revert `third_party/upstream/grit`, the Grit row in
`third_party/upstream/UPSTREAMS.md`, the
`openspec/changes/adopt-grit-full-integration` directory,
`adr/0004-adopt-grit-as-is-upstream-reference.md`,
`AI_MERGE/34_grit_full_integration/`, `.idd/goals/grit-full-integration.md`,
the generated-artifact ceiling/fingerprint fixes, and the regenerated
`.idd`/diagram/manifest artifacts from this change.

## Open Questions

None for this adoption slice. Runtime coupling between Rusty IDD and Grit is
future work and must start from this pinned mirror plus fresh native diagnostics.
