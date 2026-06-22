# IDD Task: Import both repositories under /imports without flattening

## Intent

Import both repositories under /imports without flattening

## Scope

- Kind: repo-import
- Repo area: TBD
- Files expected to change: TBD
- Files forbidden to change: TBD
- Branch authority: non-authoritative unless `.idd/LOCK.md` assigns this task

## Inputs

- `/AI_MERGE/00_repo_a_inventory.md`
- `/AI_MERGE/01_repo_b_inventory.md`
- `/AI_MERGE/02_feature_matrix.md`
- `/AI_MERGE/03_env_and_secret_contracts.md`
- `/AI_MERGE/04_merge_plan.md`
- `/AI_MERGE/08_agent_queue.md`
- `/AI_MERGE/10_parity_test_plan.md`

## Required Output

- Small PR
- Updated migration notes
- Updated tests
- Updated docs if user-facing behavior changes
- Updated `.idd/MANIFEST.tsv` if generated control-plane files change

## Definition of Done

- [ ] Build passes
- [ ] Tests pass
- [ ] Lint/typecheck passes
- [ ] Secret scan has no critical findings
- [ ] Rollback path documented
- [ ] Contract map updated
- [ ] No source deletion without parity evidence

## Agent Guardrails

Do not invent provider-specific behavior. Do not remove old implementation until parity has been proven by tests or an explicit migration note. Never print secret values into logs, PR descriptions, or generated documents.
