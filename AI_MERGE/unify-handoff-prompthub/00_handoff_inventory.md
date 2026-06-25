# Repository Inventory: handoff

- Root: `/home/drdave/Desktop/meta/handoff`
- Files scanned: `592`

## Category Counts

| Category | Count |
|---|---:|
| source | 67 |
| config | 128 |
| workflow | 11 |
| documentation | 112 |
| test | 213 |
| build | 12 |
| lockfile | 2 |
| agent-control | 3 |
| unknown | 44 |

## Languages

| Language | Files |
|---|---:|
| JavaScript | 3 |
| Nix | 1 |
| Rust | 89 |
| Shell | 13 |

## Package Managers / Toolchains

- `cargo`

## Entrypoints

- _none detected_

## Workflows

- `.github/workflows/ai-gatekeeper.yml`
- `.github/workflows/ci.yml`
- `.github/workflows/differential-drive-ci.yml`
- `.github/workflows/differential-drive.yml`
- `.github/workflows/guard-master.yml`
- `.github/workflows/notify-parent.yml`
- `.github/workflows/promote-verify.yml`
- `.github/workflows/qodana.yml`
- `.github/workflows/release.yml`
- `.github/workflows/semantic-pr-title.yml`
- `.github/workflows/sync-master.yml`

## Agent Control Files

- `.agent/skills-catalog.md`
- `.kb/AGENTS.md`
- `AGENTS.md`

## Security Files

- _none detected_

## Environment Keys Found

- `API_KEY`
- `DB_URL`
- `DOCKER_COMPOSE_ENV_FILE`
- `DOPPLER`
- `EDITOR`
- `GITHUB_TOKEN`
- `HANDOFF_LEDGER`
- `HANDOFF_LEDGER_BACKUP_DIR`
- `HF_EXE`
- `HF_LEASE_HOLDER`
- `HF_MCP_EXE`
- `HF_WEAVE_BIN`
- `HOME`
- `HOSTNAME`
- `INFISICAL`
- `NODE_ENV`
- `PARENT_REPO_PAT`
- `PORT`
- `QODANA_TOKEN`
- `SOPS`
- `VAULT_OR_OPENBAO`
- `VITE_API_URL`
- `XDG_DATA_HOME`

## Secret / Env References Found

| File | Key | Source |
|---|---|---|
| `.agent/skills-catalog.md` | `SOPS` | sops |
| `.agent/skills-catalog.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `.github/workflows/guard-master.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/notify-parent.yml` | `PARENT_REPO_PAT` | github-actions-secret |
| `.github/workflows/promote-verify.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/qodana.yml` | `QODANA_TOKEN` | github-actions-secret |
| `.github/workflows/release.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/semantic-pr-title.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/sync-master.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.handoff/decisions/ADR-0001-loop-upgrades.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `.handoff/fleet/kasetto/capsule.json` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `.handoff/fleet/obsidian-mind/capsule.json` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `.handoff/tasks/HFTASK-0013.task.json` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `crates/core/README.md` | `DOPPLER` | doppler |
| `crates/core/README.md` | `INFISICAL` | infisical |
| `crates/core/README.md` | `SOPS` | sops |
| `crates/core/README.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `crates/core/docs/audits/GAP_AUDIT_v2.md` | `DOPPLER` | doppler |
| `crates/core/docs/audits/GAP_AUDIT_v2.md` | `INFISICAL` | infisical |
| `crates/core/docs/audits/GAP_AUDIT_v2.md` | `SOPS` | sops |
| `crates/core/docs/audits/GAP_AUDIT_v2.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `crates/core/src/env_contract.rs` | `API_KEY` | process.env |
| `crates/core/src/env_contract.rs` | `DB_URL` | process.env |
| `crates/core/src/env_contract.rs` | `DOCKER_COMPOSE_ENV_FILE` | docker-compose-env-file |
| `crates/core/src/env_contract.rs` | `DOPPLER` | doppler |
| `crates/core/src/env_contract.rs` | `INFISICAL` | infisical |
| `crates/core/src/env_contract.rs` | `SOPS` | sops |
| `crates/core/src/env_contract.rs` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `crates/core/src/env_contract.rs` | `VITE_API_URL` | import.meta.env |
| `crates/core/src/model.rs` | `DOPPLER` | doppler |
| `crates/core/src/model.rs` | `INFISICAL` | infisical |
| `crates/core/src/model.rs` | `SOPS` | sops |
| `crates/core/src/model.rs` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `crates/core/src/planner.rs` | `DOPPLER` | doppler |
| `crates/core/src/planner.rs` | `INFISICAL` | infisical |
| `crates/core/src/planner.rs` | `SOPS` | sops |
| `crates/core/src/planner.rs` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `crates/core/src/templates.rs` | `INFISICAL` | infisical |
| `crates/core/src/templates.rs` | `SOPS` | sops |
| `crates/core/src/templates.rs` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `crates/core/tests/smoke.rs` | `API_KEY` | github-actions-secret |
| `crates/core/tests/smoke.rs` | `NODE_ENV` | process.env |
| `crates/tui/src/lib.rs` | `EDITOR` | std::env::var |
| `docs/adr-0001-flexnetos-autopilot-keystone.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `docs/adr-0005-needs-human-steward.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `docs/adr-0006-meta-portability.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `docs/adr-0007-flexnetos-secrets-retirement.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `docs/adr-0008-flexnetos-app-runner.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `hf/src/bin/hf-mcp.rs` | `HF_EXE` | std::env::var |
| `hf/src/lease.rs` | `HF_LEASE_HOLDER` | std::env::var |
| `hf/src/lease.rs` | `HF_WEAVE_BIN` | std::env::var |
| `hf/src/lease.rs` | `HOSTNAME` | std::env::var |
| `hf/src/main.rs` | `HANDOFF_LEDGER` | std::env::var |
| `hf/src/main.rs` | `HANDOFF_LEDGER_BACKUP_DIR` | std::env::var |
| `hf/src/main.rs` | `HOME` | std::env::var |
| `hf/src/main.rs` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `hf/src/main.rs` | `XDG_DATA_HOME` | std::env::var |
| `hf/src/policy.rs` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `hf/src/secrets.rs` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `ledger/src/v2.rs` | `HOSTNAME` | std::env::var |
| `spike/ruvocal-mcp-bridge/index.js` | `HF_MCP_EXE` | process.env |
| `spike/ruvocal-mcp-bridge/index.js` | `PORT` | process.env |

## File Index

| Path | Category | Size |
|---|---|---:|
| `.agent/skills-catalog.md` | agent-control | 320840 |
| `.claude/agent-guard.toml` | config | 4351 |
| `.claude/agents/code-omniscient-gatekeeper.md` | documentation | 5199 |
| `.claude/agents/continuity-navigator.md` | documentation | 5358 |
| `.claude/agents/doc-updater.md` | documentation | 3573 |
| `.claude/agents/fleet-steward.md` | documentation | 4563 |
| `.claude/agents/kernel-implementer.md` | documentation | 5054 |
| `.claude/agents/kernel-researcher.md` | documentation | 4207 |
| `.claude/agents/kernel-verifier.md` | documentation | 4576 |
| `.claude/agents/meta-sync-steward.md` | documentation | 6188 |
| `.claude/agents/systems-orchestrator.md` | documentation | 5299 |
| `.claude/rules/code-intelligence.md` | documentation | 909 |
| `.claude/rules/knowledge-management.md` | documentation | 1328 |
| `.claude/rules/meta-destructive-commands.md` | documentation | 918 |
| `.claude/rules/meta-workspace-discipline.md` | documentation | 1454 |
| `.claude/rules/ollama-provider.md` | documentation | 663 |
| `.claude/rules/refactoring-safety.md` | documentation | 798 |
| `.claude/settings.json` | config | 943 |
| `.claude/skills/doc-sync/SKILL.md` | documentation | 2756 |
| `.claude/skills/drift-reconcile/SKILL.md` | documentation | 5547 |
| `.claude/skills/fleet-handoff/SKILL.md` | documentation | 8592 |
| `.claude/skills/gatekeeper-review/SKILL.md` | documentation | 4506 |
| `.claude/skills/grit-coordination/SKILL.md` | documentation | 4894 |
| `.claude/skills/handoff-loop-init/SKILL.md` | documentation | 4081 |
| `.claude/skills/handoff-loop/SKILL.md` | documentation | 16545 |
| `.claude/skills/icm-memory/SKILL.md` | documentation | 3179 |
| `.claude/skills/kernel-research/SKILL.md` | documentation | 4670 |
| `.claude/skills/kernel-verify/SKILL.md` | documentation | 4502 |
| `.claude/skills/meta-kb-sync/SKILL.md` | documentation | 7088 |
| `.claude/skills/session-relay-resume/SKILL.md` | documentation | 5474 |
| `.claude/skills/session-relay-resume/scripts/verify-on-resume.template.sh` | source | 939 |
| `.claude/skills/session-relay-wrap-up/SKILL.md` | documentation | 6867 |
| `.claude/skills/session-relay/SKILL.md` | documentation | 4390 |
| `.claude/skills/systems-conduct/SKILL.md` | documentation | 4193 |
| `.claude/statusline-command.sh` | source | 775 |
| `.gitattributes` | unknown | 231 |
| `.githooks/commit-msg` | unknown | 944 |
| `.githooks/pre-commit` | unknown | 1377 |
| `.githooks/pre-push` | unknown | 1460 |
| `.github/workflows/ai-gatekeeper.yml` | workflow | 2065 |
| `.github/workflows/ci.yml` | workflow | 7240 |
| `.github/workflows/differential-drive-ci.yml` | workflow | 1731 |
| `.github/workflows/differential-drive.yml` | workflow | 1202 |
| `.github/workflows/guard-master.yml` | workflow | 2175 |
| `.github/workflows/notify-parent.yml` | workflow | 712 |
| `.github/workflows/promote-verify.yml` | workflow | 7507 |
| `.github/workflows/qodana.yml` | workflow | 1112 |
| `.github/workflows/release.yml` | workflow | 3996 |
| `.github/workflows/semantic-pr-title.yml` | workflow | 758 |
| `.github/workflows/sync-master.yml` | workflow | 4448 |
| `.gitignore` | unknown | 1907 |
| `.grit/registry.db` | unknown | 188416 |
| `.handoff/active.md` | documentation | 62 |
| `.handoff/context/capsule.json` | config | 904 |
| `.handoff/decisions/ADR-0001-loop-upgrades.md` | documentation | 73328 |
| `.handoff/decisions/TASK-0002-promote-verify-windows-envctl-stub.md` | documentation | 1108 |
| `.handoff/decisions/TASK-0003-promote-verify-audit-permission.md` | documentation | 879 |
| `.handoff/decisions/adr-0017-cognitum-gate.md` | documentation | 1427 |
| `.handoff/decisions/adr-0052-session-end-auto-sync.md` | documentation | 1931 |
| `.handoff/deliveries/handoff-buildout.delivery.json` | config | 305 |
| `.handoff/fleet/Archon/capsule.json` | config | 662 |
| `.handoff/fleet/ECC/capsule.json` | config | 817 |
| `.handoff/fleet/PILOT.toml` | config | 1505 |
| `.handoff/fleet/RuVector/capsule.json` | config | 593 |
| `.handoff/fleet/claude-code/capsule.json` | config | 757 |
| `.handoff/fleet/claude-plugins/capsule.json` | config | 678 |
| `.handoff/fleet/codex/capsule.json` | config | 707 |
| `.handoff/fleet/grit/capsule.json` | config | 758 |
| `.handoff/fleet/hermes-agent/capsule.json` | config | 840 |
| `.handoff/fleet/icm/capsule.json` | config | 722 |
| `.handoff/fleet/kasetto/capsule.json` | config | 832 |
| `.handoff/fleet/n8n/capsule.json` | config | 751 |
| `.handoff/fleet/obscura/capsule.json` | config | 705 |
| `.handoff/fleet/obsidian-mind/capsule.json` | config | 785 |
| `.handoff/fleet/oh-my-claudecode/capsule.json` | config | 741 |
| `.handoff/fleet/oh-my-pi/capsule.json` | config | 692 |
| `.handoff/fleet/rtk-tokenkill/capsule.json` | config | 860 |
| `.handoff/fleet/ruflo/capsule.json` | config | 520 |
| `.handoff/fleet/shimmy/capsule.json` | config | 771 |
| `.handoff/fleet/teri/capsule.json` | config | 715 |
| `.handoff/fleet/vox/capsule.json` | config | 708 |
| `.handoff/fleet/weave/capsule.json` | config | 2738 |
| `.handoff/hooks/hooks.toml` | config | 3843 |
| `.handoff/hooks/loop-entry.sh` | source | 2128 |
| `.handoff/hooks/session-end.sh` | source | 1518 |
| `.handoff/ledger.db` | unknown | 425984 |
| `.handoff/ledger.db.rvf` | unknown | 162 |
| `.handoff/ledger.events.jsonl` | unknown | 131775 |
| `.handoff/locks/handoff_claim_HFTASK-0016.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0017.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0019.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0034.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0037.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0038.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0052.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0053.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0054.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0062.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0064.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0065.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0066.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0067.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0068.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0069.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0070.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0071.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0072.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0075.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0076.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_HFTASK-0078.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_KBTASK-HFTASK-0058.lock` | unknown | 119 |
| `.handoff/locks/handoff_claim_KBTASK-HFTASK-0059.lock` | unknown | 119 |
| `.handoff/locks/handoff_claim_KBTASK-RUSTY-IDD-HANDOFF-SINGLE-REPO-ARCHITECTURE.lock` | unknown | 150 |
| `.handoff/locks/handoff_claim_TASK-0001.lock` | unknown | 110 |
| `.handoff/locks/handoff_claim_TASK-0002.lock` | unknown | 110 |
| `.handoff/locks/handoff_claim_TASK-0003.lock` | unknown | 110 |
| `.handoff/loop/evaluation.md` | documentation | 2975 |
| `.handoff/packets/latest.md` | test | 3343 |
| `.handoff/policies/rules.toml` | config | 2198 |
| `.handoff/policy.toml` | config | 2749 |
| `.handoff/skills/session-resume.skill.md` | documentation | 688 |
| `.handoff/tasks/HFTASK-0001.task.json` | config | 1101 |
| `.handoff/tasks/HFTASK-0002.task.json` | config | 1027 |
| `.handoff/tasks/HFTASK-0003.task.json` | config | 1687 |
| `.handoff/tasks/HFTASK-0004.task.json` | config | 1055 |
| `.handoff/tasks/HFTASK-0005.task.json` | config | 965 |
| `.handoff/tasks/HFTASK-0006.task.json` | config | 1046 |
| `.handoff/tasks/HFTASK-0007.task.json` | config | 1767 |
| `.handoff/tasks/HFTASK-0008.task.json` | config | 1176 |
| `.handoff/tasks/HFTASK-0009.task.json` | config | 1406 |
| `.handoff/tasks/HFTASK-0010.task.json` | config | 2856 |
| `.handoff/tasks/HFTASK-0011.task.json` | config | 1625 |
| `.handoff/tasks/HFTASK-0012.task.json` | config | 1935 |
| `.handoff/tasks/HFTASK-0013.task.json` | config | 1696 |
| `.handoff/tasks/HFTASK-0014.task.json` | config | 1656 |
| `.handoff/tasks/HFTASK-0015.task.json` | config | 1557 |
| `.handoff/tasks/HFTASK-0016.task.json` | config | 1846 |
| `.handoff/tasks/HFTASK-0017.task.json` | config | 1646 |
| `.handoff/tasks/HFTASK-0018.task.json` | config | 1393 |
| `.handoff/tasks/HFTASK-0019.task.json` | config | 1402 |
| `.handoff/tasks/HFTASK-0020.task.json` | config | 1573 |
| `.handoff/tasks/HFTASK-0021.task.json` | config | 1342 |
| `.handoff/tasks/HFTASK-0022.task.json` | config | 1549 |
| `.handoff/tasks/HFTASK-0026.task.json` | config | 1679 |
| `.handoff/tasks/HFTASK-0027.task.json` | config | 1448 |
| `.handoff/tasks/HFTASK-0028.task.json` | config | 1421 |
| `.handoff/tasks/HFTASK-0029.task.json` | config | 1621 |
| `.handoff/tasks/HFTASK-0030.task.json` | config | 1959 |
| `.handoff/tasks/HFTASK-0031.task.json` | config | 1508 |
| `.handoff/tasks/HFTASK-0032.task.json` | config | 1594 |
| `.handoff/tasks/HFTASK-0033.task.json` | config | 1447 |
| `.handoff/tasks/HFTASK-0034.task.json` | config | 1459 |
| `.handoff/tasks/HFTASK-0035.task.json` | config | 1246 |
| `.handoff/tasks/HFTASK-0036.task.json` | config | 1440 |
| `.handoff/tasks/HFTASK-0037.task.json` | config | 1483 |
| `.handoff/tasks/HFTASK-0038.task.json` | config | 1632 |
| `.handoff/tasks/HFTASK-0039.task.json` | config | 1331 |
| `.handoff/tasks/HFTASK-0040.task.json` | config | 1224 |
| `.handoff/tasks/HFTASK-0041.task.json` | config | 1428 |
| `.handoff/tasks/HFTASK-0042.task.json` | config | 1464 |
| `.handoff/tasks/HFTASK-0043.task.json` | config | 1346 |
| `.handoff/tasks/HFTASK-0044.task.json` | config | 1349 |
| `.handoff/tasks/HFTASK-0045.task.json` | config | 1415 |
| `.handoff/tasks/HFTASK-0046.task.json` | config | 1447 |
| `.handoff/tasks/HFTASK-0047.task.json` | config | 1440 |
| `.handoff/tasks/HFTASK-0048.task.json` | config | 1542 |
| `.handoff/tasks/HFTASK-0049.task.json` | config | 1353 |
| `.handoff/tasks/HFTASK-0050.task.json` | config | 1248 |
| `.handoff/tasks/HFTASK-0051.task.json` | config | 1229 |
| `.handoff/tasks/HFTASK-0052.task.json` | config | 1577 |
| `.handoff/tasks/HFTASK-0053.task.json` | config | 2008 |
| `.handoff/tasks/HFTASK-0054.task.json` | config | 1950 |
| `.handoff/tasks/HFTASK-0055.task.json` | config | 2108 |
| `.handoff/tasks/HFTASK-0056.task.json` | config | 1954 |
| `.handoff/tasks/HFTASK-0057.task.json` | config | 2005 |
| `.handoff/tasks/HFTASK-0058.task.json` | config | 1856 |
| `.handoff/tasks/HFTASK-0059.task.json` | config | 1874 |
| `.handoff/tasks/HFTASK-0060.task.json` | config | 1906 |
| `.handoff/tasks/HFTASK-0061.task.json` | config | 1947 |
| `.handoff/tasks/HFTASK-0062.task.json` | config | 2013 |
| `.handoff/tasks/HFTASK-0063.task.json` | config | 2001 |
| `.handoff/tasks/HFTASK-0064.task.json` | config | 2198 |
| `.handoff/tasks/HFTASK-0065.task.json` | config | 2596 |
| `.handoff/tasks/HFTASK-0066.task.json` | config | 2139 |
| `.handoff/tasks/HFTASK-0067.task.json` | config | 2245 |
| `.handoff/tasks/HFTASK-0068.task.json` | config | 1766 |
| `.handoff/tasks/HFTASK-0069.task.json` | config | 1693 |
| `.handoff/tasks/HFTASK-0070.task.json` | config | 1594 |
| `.handoff/tasks/HFTASK-0071.task.json` | config | 1501 |
| `.handoff/tasks/HFTASK-0072.task.json` | config | 1699 |
| `.handoff/tasks/HFTASK-0073.task.json` | config | 1613 |
| `.handoff/tasks/HFTASK-0074.task.json` | config | 1567 |
| `.handoff/tasks/HFTASK-0075.task.json` | config | 1527 |
| `.handoff/tasks/HFTASK-0076.task.json` | config | 1655 |
| `.handoff/tasks/HFTASK-0077.task.json` | config | 1556 |
| `.handoff/tasks/HFTASK-0078.task.json` | config | 3309 |
| `.handoff/tasks/KBTASK-FLEET-HANDOFF-ROLLOUT.task.json` | config | 918 |
| `.handoff/tasks/TASK-0001.task.json` | config | 1433 |
| `.handoff/tasks/TASK-0002.task.json` | config | 1724 |
| `.handoff/tasks/TASK-0003.task.json` | config | 1560 |
| `.kb/AGENTS.md` | agent-control | 29779 |
| `.kb/config.toml` | config | 540 |
| `.kb/store/commits/019eedca-a7b5-73e0-b195-e047a91a9690.json` | config | 556 |
| `.kb/store/commits/019eedcb-1c29-7233-8b33-3b3bdb860917.json` | config | 618 |
| `.kb/store/commits/019eedcc-681e-71d0-bc89-0ecfb3437c46.json` | config | 2376 |
| `.kb/store/commits/019eedd2-0b61-78d2-86b6-ee2aa9d67071.json` | config | 613 |
| `.kb/store/commits/019eedd2-2e57-7751-939d-aa112a24afd1.json` | config | 832 |
| `.kb/store/commits/019eedd2-4d57-7b91-9754-9742941995da.json` | config | 836 |
| `.kb/store/commits/019eedd2-4dbb-7ae0-9df8-0a1877886797.json` | config | 667 |
| `.kb/store/documents/context/extensible/product.md` | documentation | 980 |
| `.kb/store/documents/context/extensible/tech.md` | documentation | 1067 |
| `.kb/store/documents/context/immutable/architecture.md` | documentation | 1253 |
| `.kb/store/documents/context/immutable/patterns.md` | documentation | 1561 |
| `.kb/store/documents/context/immutable/project-brief.md` | documentation | 1412 |
| `.kb/store/documents/context/overridable/active.md` | documentation | 1139 |
| `.kb/store/documents/context/overridable/progress.md` | documentation | 652 |
| `.kb/store/manifest.json` | config | 162 |
| `.kb/store/refs/document-tips.json` | config | 1640 |
| `.kb/workspaces/main/context/extensible/product.md` | documentation | 980 |
| `.kb/workspaces/main/context/extensible/tech.md` | documentation | 1067 |
| `.kb/workspaces/main/context/immutable/architecture.md` | documentation | 1253 |
| `.kb/workspaces/main/context/immutable/patterns.md` | documentation | 1561 |
| `.kb/workspaces/main/context/immutable/project-brief.md` | documentation | 1412 |
| `.kb/workspaces/main/context/overridable/active.md` | documentation | 1139 |
| `.kb/workspaces/main/context/overridable/progress.md` | documentation | 652 |
| `.release-please-manifest.json` | config | 19 |
| `AGENTS.md` | agent-control | 5674 |
| `CLAUDE.md` | documentation | 33829 |
| `CONTRIBUTING.md` | documentation | 649 |
| `Cargo.lock` | lockfile | 99490 |
| `Cargo.toml` | build | 418 |
| `FLEET_GUIDE.md` | documentation | 14777 |
| `LESSONS.md` | documentation | 11433 |
| `Makefile` | build | 1714 |
| `NEEDS-HUMAN.md` | documentation | 1461 |
| `NORTH-STAR.md` | documentation | 5802 |
| `VERSION` | unknown | 6 |
| `_workspace_prev/01_navigator_truth.md` | documentation | 10650 |
| `_workspace_prev/02_research_HFTASK-0070.md` | documentation | 21284 |
| `_workspace_prev/03_impl_HFTASK-0070.md` | documentation | 3395 |
| `_workspace_prev/04_verify_HFTASK-0070.md` | documentation | 2523 |
| `_workspace_prev/05_verdict_HFTASK-0070.md` | documentation | 7405 |
| `commitlint.config.cjs` | source | 352 |
| `crates/cli/Cargo.toml` | build | 933 |
| `crates/cli/src/commands/core.rs` | source | 963 |
| `crates/cli/src/commands/mod.rs` | source | 195 |
| `crates/cli/src/commands/run.rs` | source | 3018 |
| `crates/cli/src/commands/spec.rs` | test | 13931 |
| `crates/cli/src/commands/spec_adr.rs` | test | 4182 |
| `crates/cli/src/commands/spec_archive.rs` | test | 9851 |
| `crates/cli/src/commands/spec_scaffold.rs` | test | 2321 |
| `crates/cli/src/commands/spec_status.rs` | test | 7158 |
| `crates/cli/src/commands/tui.rs` | source | 422 |
| `crates/cli/src/lib.rs` | source | 3293 |
| `crates/cli/src/main.rs` | source | 207 |
| `crates/cli/tests/archive_cli.rs` | test | 8615 |
| `crates/cli/tests/run_cli.rs` | test | 1254 |
| `crates/cli/tests/spec_adr_cli.rs` | test | 3006 |
| `crates/cli/tests/spec_cli.rs` | test | 7030 |
| `crates/cli/tests/spec_scaffold_cli.rs` | test | 3247 |
| `crates/cli/tests/spec_status_cli.rs` | test | 3484 |
| `crates/core/.gitignore` | unknown | 89 |
| `crates/core/Cargo.toml` | build | 644 |
| `crates/core/LICENSE` | unknown | 1095 |
| `crates/core/README.md` | documentation | 4772 |
| `crates/core/docs/AGENT_WORKFLOW.md` | documentation | 1266 |
| `crates/core/docs/ARCHITECTURE.md` | documentation | 1390 |
| `crates/core/docs/audits/GAP_AUDIT_v2.md` | documentation | 3850 |
| `crates/core/examples/README.md` | documentation | 1009 |
| `crates/core/rust-toolchain.toml` | config | 66 |
| `crates/core/scripts/package.sh` | source | 293 |
| `crates/core/src/cli.rs` | source | 11989 |
| `crates/core/src/env_contract.rs` | source | 10406 |
| `crates/core/src/fs_utils.rs` | source | 6355 |
| `crates/core/src/lib.rs` | source | 644 |
| `crates/core/src/manifest.rs` | source | 2388 |
| `crates/core/src/model.rs` | source | 5129 |
| `crates/core/src/planner.rs` | source | 26340 |
| `crates/core/src/scanner.rs` | source | 9644 |
| `crates/core/src/templates.rs` | source | 11193 |
| `crates/core/src/validation.rs` | source | 8208 |
| `crates/core/tests/smoke.rs` | test | 2387 |
| `crates/runner/Cargo.toml` | build | 758 |
| `crates/runner/src/config.rs` | source | 21543 |
| `crates/runner/src/data.rs` | source | 59681 |
| `crates/runner/src/lib.rs` | source | 502 |
| `crates/runner/src/runner.rs` | source | 74423 |
| `crates/spec/Cargo.toml` | build | 1008 |
| `crates/spec/src/adr/mod.rs` | test | 8138 |
| `crates/spec/src/archive/mod.rs` | test | 3819 |
| `crates/spec/src/lib.rs` | test | 1770 |
| `crates/spec/src/model/block.rs` | test | 828 |
| `crates/spec/src/model/delta.rs` | test | 1166 |
| `crates/spec/src/model/merge.rs` | test | 21903 |
| `crates/spec/src/model/mod.rs` | test | 1417 |
| `crates/spec/src/model/requirement.rs` | test | 1647 |
| `crates/spec/src/model/spec.rs` | test | 1220 |
| `crates/spec/src/parse/common.rs` | test | 2047 |
| `crates/spec/src/parse/delta_parser.rs` | test | 6773 |
| `crates/spec/src/parse/emit.rs` | test | 3044 |
| `crates/spec/src/parse/mod.rs` | test | 254 |
| `crates/spec/src/parse/spec_parser.rs` | test | 3039 |
| `crates/spec/src/scaffold/mod.rs` | test | 8588 |
| `crates/spec/src/schema/graph.rs` | test | 5539 |
| `crates/spec/src/schema/mod.rs` | test | 5026 |
| `crates/spec/src/validate/mod.rs` | test | 295 |
| `crates/spec/src/validate/report.rs` | test | 2152 |
| `crates/spec/src/validate/rules.rs` | test | 2964 |
| `crates/spec/tests/archive_direct.rs` | test | 2033 |
| `crates/spec/tests/archive_golden.rs` | test | 5954 |
| `crates/spec/tests/parse_emit_direct.rs` | test | 3823 |
| `crates/spec/tests/validate_golden.rs` | test | 3260 |
| `crates/tui/.claude/commands/opsx/apply.md` | documentation | 4546 |
| `crates/tui/.claude/commands/opsx/archive.md` | documentation | 5015 |
| `crates/tui/.claude/commands/opsx/bulk-archive.md` | documentation | 7552 |
| `crates/tui/.claude/commands/opsx/continue.md` | documentation | 4960 |
| `crates/tui/.claude/commands/opsx/explore.md` | documentation | 6592 |
| `crates/tui/.claude/commands/opsx/ff.md` | documentation | 4255 |
| `crates/tui/.claude/commands/opsx/new.md` | documentation | 2664 |
| `crates/tui/.claude/commands/opsx/onboard.md` | documentation | 13500 |
| `crates/tui/.claude/commands/opsx/propose.md` | documentation | 4418 |
| `crates/tui/.claude/commands/opsx/sync.md` | documentation | 4356 |
| `crates/tui/.claude/commands/opsx/verify.md` | documentation | 6431 |
| `crates/tui/.claude/skills/openspec-apply-change/SKILL.md` | test | 4724 |
| `crates/tui/.claude/skills/openspec-archive-change/SKILL.md` | test | 4157 |
| `crates/tui/.claude/skills/openspec-bulk-archive-change/SKILL.md` | test | 7661 |
| `crates/tui/.claude/skills/openspec-continue-change/SKILL.md` | test | 5054 |
| `crates/tui/.claude/skills/openspec-explore/SKILL.md` | test | 10569 |
| `crates/tui/.claude/skills/openspec-ff-change/SKILL.md` | test | 4438 |
| `crates/tui/.claude/skills/openspec-new-change/SKILL.md` | test | 2880 |
| `crates/tui/.claude/skills/openspec-onboard/SKILL.md` | test | 13581 |
| `crates/tui/.claude/skills/openspec-propose/SKILL.md` | test | 4646 |
| `crates/tui/.claude/skills/openspec-sync-specs/SKILL.md` | test | 4485 |
| `crates/tui/.claude/skills/openspec-verify-change/SKILL.md` | test | 6541 |
| `crates/tui/.gitignore` | unknown | 33 |
| `crates/tui/Cargo.toml` | build | 872 |
| `crates/tui/LICENSE` | unknown | 1069 |
| `crates/tui/README.md` | documentation | 4014 |
| `crates/tui/flake.lock` | unknown | 569 |
| `crates/tui/flake.nix` | build | 2704 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-nix-flake/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-nix-flake/design.md` | test | 2327 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-nix-flake/proposal.md` | test | 1038 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-nix-flake/specs/nix-dev-environment/spec.md` | test | 1652 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-nix-flake/tasks.md` | test | 635 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-readme/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-readme/design.md` | test | 1152 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-readme/proposal.md` | test | 722 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-readme/specs/project-readme/spec.md` | test | 1033 |
| `crates/tui/openspec/changes/archive/2026-03-03-add-readme/tasks.md` | test | 198 |
| `crates/tui/openspec/changes/archive/2026-03-03-tui-change-viewer/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-03-tui-change-viewer/design.md` | test | 2910 |
| `crates/tui/openspec/changes/archive/2026-03-03-tui-change-viewer/proposal.md` | test | 1404 |
| `crates/tui/openspec/changes/archive/2026-03-03-tui-change-viewer/specs/artifact-content-view/spec.md` | test | 2024 |
| `crates/tui/openspec/changes/archive/2026-03-03-tui-change-viewer/specs/artifact-menu-view/spec.md` | test | 2048 |
| `crates/tui/openspec/changes/archive/2026-03-03-tui-change-viewer/specs/change-list-view/spec.md` | test | 1982 |
| `crates/tui/openspec/changes/archive/2026-03-03-tui-change-viewer/tasks.md` | test | 1810 |
| `crates/tui/openspec/changes/archive/2026-03-05-md-word-wrap/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-05-md-word-wrap/design.md` | test | 2359 |
| `crates/tui/openspec/changes/archive/2026-03-05-md-word-wrap/proposal.md` | test | 1148 |
| `crates/tui/openspec/changes/archive/2026-03-05-md-word-wrap/specs/artifact-content-view/spec.md` | test | 659 |
| `crates/tui/openspec/changes/archive/2026-03-05-md-word-wrap/specs/md-word-wrap/spec.md` | test | 903 |
| `crates/tui/openspec/changes/archive/2026-03-05-md-word-wrap/tasks.md` | test | 430 |
| `crates/tui/openspec/changes/archive/2026-03-05-windows-openspec-path-fix/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-05-windows-openspec-path-fix/design.md` | test | 2310 |
| `crates/tui/openspec/changes/archive/2026-03-05-windows-openspec-path-fix/proposal.md` | test | 1129 |
| `crates/tui/openspec/changes/archive/2026-03-05-windows-openspec-path-fix/specs/cross-platform-command/spec.md` | test | 1231 |
| `crates/tui/openspec/changes/archive/2026-03-05-windows-openspec-path-fix/tasks.md` | test | 517 |
| `crates/tui/openspec/changes/archive/2026-03-06-add-markdown-highlighting/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-06-add-markdown-highlighting/design.md` | test | 2522 |
| `crates/tui/openspec/changes/archive/2026-03-06-add-markdown-highlighting/proposal.md` | test | 1241 |
| `crates/tui/openspec/changes/archive/2026-03-06-add-markdown-highlighting/specs/artifact-content-view/spec.md` | test | 1169 |
| `crates/tui/openspec/changes/archive/2026-03-06-add-markdown-highlighting/specs/markdown-rendering/spec.md` | test | 2714 |
| `crates/tui/openspec/changes/archive/2026-03-06-add-markdown-highlighting/tasks.md` | test | 696 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-implementation-runner/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-implementation-runner/design.md` | test | 5758 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-implementation-runner/proposal.md` | test | 1907 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-implementation-runner/specs/implementation-runner/spec.md` | test | 2857 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-implementation-runner/specs/implementation-status-bar/spec.md` | test | 2107 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-implementation-runner/tasks.md` | test | 2692 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/design.md` | test | 3735 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/implementation.log` | test | 7642 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/proposal.md` | test | 1706 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/specs/config-screen/spec.md` | test | 3461 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/specs/cross-platform-command/spec.md` | test | 1714 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/specs/tui-configuration/spec.md` | test | 3130 |
| `crates/tui/openspec/changes/archive/2026-03-08-add-tui-configuration/tasks.md` | test | 2730 |
| `crates/tui/openspec/changes/archive/2026-03-08-change-local-implementation-log/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-08-change-local-implementation-log/design.md` | test | 2899 |
| `crates/tui/openspec/changes/archive/2026-03-08-change-local-implementation-log/proposal.md` | test | 1394 |
| `crates/tui/openspec/changes/archive/2026-03-08-change-local-implementation-log/specs/artifact-menu-view/spec.md` | test | 1177 |
| `crates/tui/openspec/changes/archive/2026-03-08-change-local-implementation-log/specs/implementation-log/spec.md` | test | 2201 |
| `crates/tui/openspec/changes/archive/2026-03-08-change-local-implementation-log/tasks.md` | test | 1414 |
| `crates/tui/openspec/changes/archive/2026-03-08-enrich-runner-prompt/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-08-enrich-runner-prompt/design.md` | test | 2951 |
| `crates/tui/openspec/changes/archive/2026-03-08-enrich-runner-prompt/proposal.md` | test | 1197 |
| `crates/tui/openspec/changes/archive/2026-03-08-enrich-runner-prompt/specs/implementation-runner/spec.md` | test | 1676 |
| `crates/tui/openspec/changes/archive/2026-03-08-enrich-runner-prompt/tasks.md` | test | 482 |
| `crates/tui/openspec/changes/archive/2026-03-08-fix-config-input-modes/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-08-fix-config-input-modes/design.md` | test | 3003 |
| `crates/tui/openspec/changes/archive/2026-03-08-fix-config-input-modes/implementation.log` | test | 6502 |
| `crates/tui/openspec/changes/archive/2026-03-08-fix-config-input-modes/proposal.md` | test | 1466 |
| `crates/tui/openspec/changes/archive/2026-03-08-fix-config-input-modes/specs/config-screen/spec.md` | test | 4970 |
| `crates/tui/openspec/changes/archive/2026-03-08-fix-config-input-modes/tasks.md` | test | 1839 |
| `crates/tui/openspec/changes/archive/2026-03-08-improve-log-viewing/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-08-improve-log-viewing/design.md` | test | 3500 |
| `crates/tui/openspec/changes/archive/2026-03-08-improve-log-viewing/implementation.log` | test | 1904 |
| `crates/tui/openspec/changes/archive/2026-03-08-improve-log-viewing/proposal.md` | test | 1298 |
| `crates/tui/openspec/changes/archive/2026-03-08-improve-log-viewing/specs/artifact-content-view/spec.md` | test | 2443 |
| `crates/tui/openspec/changes/archive/2026-03-08-improve-log-viewing/specs/artifact-menu-view/spec.md` | test | 1543 |
| `crates/tui/openspec/changes/archive/2026-03-08-improve-log-viewing/tasks.md` | test | 1221 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/design.md` | test | 4249 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/implementation.log` | test | 3367 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/proposal.md` | test | 2092 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/specs/archived-change-browsing/spec.md` | test | 2831 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/specs/artifact-menu-view/spec.md` | test | 598 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/specs/change-list-view/spec.md` | test | 1866 |
| `crates/tui/openspec/changes/archive/2026-03-08-view-archived-changes/tasks.md` | test | 2007 |
| `crates/tui/openspec/changes/batch-implementation-runner/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/batch-implementation-runner/design.md` | test | 5866 |
| `crates/tui/openspec/changes/batch-implementation-runner/implementation.log` | test | 22316 |
| `crates/tui/openspec/changes/batch-implementation-runner/proposal.md` | test | 2820 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/artifact-menu-view/spec.md` | test | 1014 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/batch-runner/spec.md` | test | 2924 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/change-dependencies/spec.md` | test | 3663 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/change-list-view/spec.md` | test | 2566 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/dependency-management-view/spec.md` | test | 2564 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/implementation-runner/spec.md` | test | 964 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/implementation-status-bar/spec.md` | test | 1171 |
| `crates/tui/openspec/changes/batch-implementation-runner/specs/run-all-selection/spec.md` | test | 2943 |
| `crates/tui/openspec/changes/batch-implementation-runner/tasks.md` | test | 5344 |
| `crates/tui/openspec/changes/implementation-stall-detection/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/implementation-stall-detection/design.md` | test | 3256 |
| `crates/tui/openspec/changes/implementation-stall-detection/implementation.log` | test | 4497 |
| `crates/tui/openspec/changes/implementation-stall-detection/proposal.md` | test | 1530 |
| `crates/tui/openspec/changes/implementation-stall-detection/specs/implementation-runner/spec.md` | test | 2422 |
| `crates/tui/openspec/changes/implementation-stall-detection/specs/stall-detection/spec.md` | test | 1810 |
| `crates/tui/openspec/changes/implementation-stall-detection/tasks.md` | test | 1076 |
| `crates/tui/openspec/changes/injectable-config-path/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/injectable-config-path/design.md` | test | 2546 |
| `crates/tui/openspec/changes/injectable-config-path/implementation.log` | test | 1676 |
| `crates/tui/openspec/changes/injectable-config-path/proposal.md` | test | 1379 |
| `crates/tui/openspec/changes/injectable-config-path/specs/tui-configuration/spec.md` | test | 1815 |
| `crates/tui/openspec/changes/injectable-config-path/tasks.md` | test | 1039 |
| `crates/tui/openspec/changes/interactive-tool-launch/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/interactive-tool-launch/design.md` | test | 3605 |
| `crates/tui/openspec/changes/interactive-tool-launch/implementation.log` | test | 4442 |
| `crates/tui/openspec/changes/interactive-tool-launch/proposal.md` | test | 1407 |
| `crates/tui/openspec/changes/interactive-tool-launch/specs/config-screen/spec.md` | test | 2331 |
| `crates/tui/openspec/changes/interactive-tool-launch/specs/interactive-tool-launch/spec.md` | test | 2077 |
| `crates/tui/openspec/changes/interactive-tool-launch/specs/tui-configuration/spec.md` | test | 830 |
| `crates/tui/openspec/changes/interactive-tool-launch/tasks.md` | test | 1926 |
| `crates/tui/openspec/changes/post-implementation-hook/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/post-implementation-hook/dependencies.yaml` | test | 37 |
| `crates/tui/openspec/changes/post-implementation-hook/design.md` | test | 3877 |
| `crates/tui/openspec/changes/post-implementation-hook/implementation.log` | test | 2490 |
| `crates/tui/openspec/changes/post-implementation-hook/proposal.md` | test | 1780 |
| `crates/tui/openspec/changes/post-implementation-hook/specs/implementation-runner/spec.md` | test | 2488 |
| `crates/tui/openspec/changes/post-implementation-hook/specs/post-implementation-hook/spec.md` | test | 2948 |
| `crates/tui/openspec/changes/post-implementation-hook/specs/tui-configuration/spec.md` | test | 2090 |
| `crates/tui/openspec/changes/post-implementation-hook/tasks.md` | test | 1904 |
| `crates/tui/openspec/changes/run-finished-command/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/run-finished-command/design.md` | test | 4312 |
| `crates/tui/openspec/changes/run-finished-command/implementation.log` | test | 2004 |
| `crates/tui/openspec/changes/run-finished-command/proposal.md` | test | 1290 |
| `crates/tui/openspec/changes/run-finished-command/specs/run-finished-notification/spec.md` | test | 3114 |
| `crates/tui/openspec/changes/run-finished-command/specs/tui-configuration/spec.md` | test | 2889 |
| `crates/tui/openspec/changes/run-finished-command/tasks.md` | test | 1642 |
| `crates/tui/openspec/changes/selectable-run-mode/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/selectable-run-mode/design.md` | test | 4491 |
| `crates/tui/openspec/changes/selectable-run-mode/implementation.log` | test | 5385 |
| `crates/tui/openspec/changes/selectable-run-mode/proposal.md` | test | 1930 |
| `crates/tui/openspec/changes/selectable-run-mode/specs/implementation-runner/spec.md` | test | 1688 |
| `crates/tui/openspec/changes/selectable-run-mode/specs/per-change-config/spec.md` | test | 2361 |
| `crates/tui/openspec/changes/selectable-run-mode/tasks.md` | test | 2405 |
| `crates/tui/openspec/changes/tui-refresh-views/.openspec.yaml` | test | 40 |
| `crates/tui/openspec/changes/tui-refresh-views/design.md` | test | 3118 |
| `crates/tui/openspec/changes/tui-refresh-views/implementation.log` | test | 2812 |
| `crates/tui/openspec/changes/tui-refresh-views/proposal.md` | test | 1224 |
| `crates/tui/openspec/changes/tui-refresh-views/specs/view-refresh/spec.md` | test | 3231 |
| `crates/tui/openspec/changes/tui-refresh-views/tasks.md` | test | 1704 |
| `crates/tui/openspec/config.yaml` | test | 1602 |
| `crates/tui/openspec/specs/archived-change-browsing/spec.md` | test | 2831 |
| `crates/tui/openspec/specs/artifact-content-view/spec.md` | test | 2443 |
| `crates/tui/openspec/specs/artifact-menu-view/spec.md` | test | 1543 |
| `crates/tui/openspec/specs/change-list-view/spec.md` | test | 1866 |
| `crates/tui/openspec/specs/config-screen/spec.md` | test | 4970 |
| `crates/tui/openspec/specs/cross-platform-command/spec.md` | test | 1714 |
| `crates/tui/openspec/specs/implementation-log/spec.md` | test | 2201 |
| `crates/tui/openspec/specs/implementation-runner/spec.md` | test | 1676 |
| `crates/tui/openspec/specs/implementation-status-bar/spec.md` | test | 2107 |
| `crates/tui/openspec/specs/markdown-rendering/spec.md` | test | 2842 |
| `crates/tui/openspec/specs/md-word-wrap/spec.md` | test | 903 |
| `crates/tui/openspec/specs/nix-dev-environment/spec.md` | test | 1629 |
| `crates/tui/openspec/specs/tui-configuration/spec.md` | test | 3130 |
| `crates/tui/ralph-implement.sh` | source | 393 |
| `crates/tui/src/app.rs` | source | 195144 |
| `crates/tui/src/lib.rs` | source | 6981 |
| `crates/tui/src/ui.rs` | source | 84930 |
| `docs/ARCHITECTURE.md` | documentation | 6340 |
| `docs/Continuity_Ledger_Kernel_PRD.md` | documentation | 30657 |
| `docs/INTEGRATION-RUSTY-IDD.md` | documentation | 1576 |
| `docs/PHASE-8-HANDOFF.md` | documentation | 4777 |
| `docs/TEST_MATRIX.md` | test | 4524 |
| `docs/USER_STORY.md` | documentation | 2342 |
| `docs/adr-0001-flexnetos-autopilot-keystone.md` | documentation | 28552 |
| `docs/adr-0002-weave-a2a-conventions.md` | documentation | 3727 |
| `docs/adr-0003-kb-handoff-seam.md` | documentation | 4400 |
| `docs/adr-0004-fleet-handoff-rollout.md` | documentation | 10148 |
| `docs/adr-0005-needs-human-steward.md` | documentation | 6228 |
| `docs/adr-0006-meta-portability.md` | documentation | 12223 |
| `docs/adr-0007-flexnetos-secrets-retirement.md` | documentation | 6048 |
| `docs/adr-0008-flexnetos-app-runner.md` | documentation | 16315 |
| `docs/adr-0009-grit-parallel-coordination.md` | documentation | 6269 |
| `docs/adr-0010-grit-shared-backend-envctl.md` | documentation | 5420 |
| `docs/adr-0011-agentcontract-proof-at-handoff.md` | documentation | 7965 |
| `docs/adr-0012-domain-expansion-task-routing.md` | documentation | 4119 |
| `docs/adr-0013-flexnetos-meta-conventions.md` | documentation | 3345 |
| `docs/adr-0014-cognitum-gate-policy-engine.md` | documentation | 2281 |
| `docs/adr-0015-delivery-endpoint.md` | documentation | 2619 |
| `docs/adr-0016-handoff-durability-policy.md` | documentation | 5462 |
| `docs/adr-0017-pure-rust-ledger-store-redb.md` | documentation | 13953 |
| `docs/adr-0018-full-auto-agentic-operation.md` | documentation | 14222 |
| `docs/backlog.yaml` | config | 766 |
| `docs/branch-reconciliation-2026-06-21.md` | documentation | 3470 |
| `docs/repomix-research.md` | documentation | 1151 |
| `docs/rusty-idd-research.md` | documentation | 3250 |
| `docs/rusty-idd/oracle-fixtures/01-base-spec.md` | test | 775 |
| `docs/rusty-idd/oracle-fixtures/02-delta-spec.md` | test | 621 |
| `docs/rusty-idd/oracle-fixtures/03-archived-result.md` | documentation | 846 |
| `docs/rusty-idd/oracle-fixtures/04-validate-spec.json` | test | 537 |
| `docs/rusty-idd/oracle-fixtures/05-validate-no-scenario.json` | config | 842 |
| `docs/rusty-idd/oracle-fixtures/06-rename-modify-base.md` | documentation | 190 |
| `docs/rusty-idd/oracle-fixtures/07-rename-modify-delta.md` | documentation | 181 |
| `docs/rusty-idd/oracle-fixtures/08-rename-modify-result.md` | documentation | 202 |
| `docs/rusty-idd/oracle-fixtures/README.md` | documentation | 2869 |
| `docs/rusty-idd/spec-engine-design.md` | test | 16111 |
| `hf/Cargo.toml` | build | 3146 |
| `hf/src/bin/hf-mcp.rs` | source | 35037 |
| `hf/src/branch.rs` | source | 12014 |
| `hf/src/cognitum.rs` | source | 6440 |
| `hf/src/contract.rs` | source | 19346 |
| `hf/src/delivery.rs` | source | 12831 |
| `hf/src/durability.rs` | source | 21074 |
| `hf/src/fleet.rs` | source | 33930 |
| `hf/src/gatekeeper.rs` | source | 12749 |
| `hf/src/gates.rs` | source | 24670 |
| `hf/src/hooks.rs` | source | 16141 |
| `hf/src/intake.rs` | source | 11749 |
| `hf/src/kb.rs` | source | 19215 |
| `hf/src/lease.rs` | source | 10558 |
| `hf/src/main.rs` | source | 236436 |
| `hf/src/policy.rs` | source | 10016 |
| `hf/src/prompt_hub.rs` | source | 6023 |
| `hf/src/route.rs` | source | 8472 |
| `hf/src/routing.rs` | source | 10370 |
| `hf/src/schema.rs` | source | 7059 |
| `hf/src/secrets.rs` | source | 4907 |
| `hf/src/session.rs` | source | 32347 |
| `hf/src/sync.rs` | source | 19392 |
| `hf/src/test_support.rs` | test | 851 |
| `intent-driven-template/openspec/schemas/intent-driven/schema.yaml` | test | 10775 |
| `ledger/Cargo.toml` | build | 1990 |
| `ledger/src/export.rs` | source | 5874 |
| `ledger/src/lib.rs` | source | 1855 |
| `ledger/src/migrate.rs` | source | 8068 |
| `ledger/src/v1.rs` | source | 72101 |
| `ledger/src/v2.rs` | source | 27601 |
| `qodana.yaml` | config | 517 |
| `release-please-config.json` | config | 349 |
| `renovate.json` | config | 114 |
| `schemas/packet.schema.json` | config | 1400 |
| `schemas/session.schema.json` | config | 943 |
| `schemas/task.schema.json` | config | 4922 |
| `scripts/differential-drive.cases.sh` | source | 2033 |
| `scripts/differential-drive.sh` | source | 3766 |
| `scripts/fail-open-audit.sh` | source | 4688 |
| `scripts/fleet-rollout.sh` | source | 7090 |
| `scripts/grit-shared.sh` | source | 1756 |
| `scripts/handoff-lib.sh` | source | 4523 |
| `scripts/handoff-loop-init.sh` | source | 17100 |
| `spike/ruvocal-mcp-bridge/README.md` | documentation | 1759 |
| `spike/ruvocal-mcp-bridge/index.js` | source | 4072 |
| `spike/ruvocal-mcp-bridge/package-lock.json` | lockfile | 29383 |
| `spike/ruvocal-mcp-bridge/package.json` | build | 429 |
| `spike/ruvocal-mcp-bridge/servers.json.example` | unknown | 69 |
| `spike/ruvocal-mcp-bridge/test/smoke.js` | test | 1166 |
| `work-order/Cargo.toml` | build | 577 |
| `work-order/src/intake.rs` | source | 16933 |
| `work-order/src/lib.rs` | source | 25092 |
