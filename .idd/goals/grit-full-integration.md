# Grit Full Integration Goal

Adopt `github.com/FlexNetOS/grit` into Rusty IDD as a full upstream integration
reference without refactoring, rewriting, trimming, or changing grit code.

The integration must:

- start from the latest Rusty IDD `develop` branch in a fresh feature worktree;
- scan grit at full tracked-file depth, including tracked dotfiles, workflows,
  tests, examples, assets, docs, scripts, nested projects, and configuration;
- generate Rusty IDD planning context through `rusty-idd knowledge plan-context
  --goal-file`;
- run the Rusty IDD scan and plan workflow before adoption writes;
- preserve grit as an as-is upstream snapshot for future Rusty IDD planning,
  diagnostics, and rollback;
- update OpenSpec, ADR, task, evidence, knowledge, diagram, and manifest
  artifacts; and
- record validation evidence and rollback guidance.

Non-goals:

- no grit source-code edits;
- no Rusty IDD runtime refactor;
- no dependency downgrade or feature cut;
- no cherry-picking partial grit files; and
- no host-service, daemon, or user-global tool installation.
