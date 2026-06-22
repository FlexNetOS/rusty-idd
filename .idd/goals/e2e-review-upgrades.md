# E2E Review Upgrade Goal

Run a deep research code-review loop over the comprehensive E2E workflow test
suite and Codex workflow-check work. Gap-hunt the merged implementation, apply
upgrades, and raise the tests to professional code-grade quality.

The upgrade must preserve the Rusty IDD workflow order:

1. Goal file and graph-backed context.
2. Claimed task card.
3. OpenSpec proposal, design, spec delta, tasks, and ADR.
4. Review evidence.
5. Test-first implementation upgrades.
6. Generated artifact refresh.
7. Mandatory validation before task completion, push, PR, and merge.

Professional-grade means validation evidence must not pass from marker words
alone. The workflow checker must reject failed, placeholder, skipped, or
misordered validation entries for build, generated artifacts, tests, lint,
secret scan, and manifest evidence.
