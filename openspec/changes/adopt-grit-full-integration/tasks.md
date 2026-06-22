# adopt-grit-full-integration - Tasks

## 1. Goal and Planning

- [x] 1.1 Create the tracked Grit integration goal file.
- [x] 1.2 Generate `.idd/knowledge/plan-context.md` with `--goal-file`.
- [x] 1.3 Generate `.idd/knowledge/plan-context.json` with `--goal-file`.
- [x] 1.4 Run Rusty IDD scan artifacts for Grit and Rusty IDD.
- [x] 1.5 Run the Rusty IDD plan workflow into the Grit evidence workspace.

## 2. OpenSpec and Decisions

- [x] 2.1 Add proposal, spec delta, design, ADR, and task artifacts.
- [x] 2.2 Verify OpenSpec change readiness with `rusty-idd spec status`.

## 3. Adopt Grit As-Is

- [x] 3.1 Import the full tracked Grit snapshot into `third_party/upstream/grit`.
- [x] 3.2 Verify mirror file counts and tracked dotfiles against Grit.
- [x] 3.3 Update `third_party/upstream/UPSTREAMS.md`.
- [x] 3.4 Record adoption evidence, migration note, rollback path, and scan/plan
  results under `AI_MERGE/34_grit_full_integration/`.
- [x] 3.5 Apply the narrow Rusty IDD generated-artifact fixes required for the
  full-depth adoption run.

## 4. Regenerate and Validate

- [x] 4.1 Refresh `.idd/knowledge` artifacts.
- [x] 4.2 Regenerate architecture diagrams.
- [x] 4.3 Regenerate `.idd/MANIFEST.tsv`.
- [x] 4.4 Run OpenSpec status, validation, manifest, diagram, build/test/lint,
  and secret-scan gates.
