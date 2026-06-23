# Regenerating the full merge workspaces

This directory holds the **distilled** merge-tools inventory/plan evidence for the
handoff + prompt_hub unification. The full, regenerable `rusty-idd plan`
workspaces (which also emit a large rusty-idd self-inventory and boilerplate
scaffolding) are intentionally NOT committed — they are reproducible on demand:

```sh
rusty-idd scan --repo ../handoff   --out 00_handoff_inventory.md   --format md
rusty-idd scan --repo ../prompt_hub --out 01_prompt_hub_inventory.md --format md
rusty-idd plan --repo-a . --repo-b ../handoff    --out /tmp/plan-handoff   --name unify-handoff
rusty-idd plan --repo-a . --repo-b ../prompt_hub --out /tmp/plan-prompthub --name unify-prompthub
```

Distilled, committed here (per unification):
- `00/01_*_inventory.{md,json}` — RepoInventory (scan)
- `02_*_feature_matrix.md` — capability + shared-path matrix
- `03_*_env_secret_contracts.{md,json}` — env/secret contract
- `04_*_merge_plan.md` — strategy, recommended tree, phases, risk read
- `05_*_conflict_risk_register.md` — conflicts + mitigations
- `06_*_gap_audit.md`, `10_*_parity_test_plan.md`, `11_*_provider_matrix.md`
- `07_*_tasks/` — the 5-slice task breakdown
