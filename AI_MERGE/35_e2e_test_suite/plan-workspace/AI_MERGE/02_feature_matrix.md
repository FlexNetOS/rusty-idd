# Feature Matrix

This matrix is generated from structural signals. Treat it as a starting point, then refine it with explicit product intent.

| Capability | Repo A Signal | Repo B Signal | Default Decision | Migration Action |
|---|---|---|---|---|
| Rust native core | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Node/TypeScript UI or tooling | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Python tooling | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| GitHub Actions CI | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Environment contract | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Secret references | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Nix, mise, or direnv toolchain | no | no | Create only if required by product intent | No action yet |
| Agent control files | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Security policy files | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |

## Shared Paths

| Path | Repo A | Repo B | Risk |
|---|---|---|---|
| `.agents/skills/rusty-idd-adopt-first/SKILL.md` | yes | yes | naming/API collision |
| `.agents/skills/rusty-idd-codex-rust-env/SKILL.md` | yes | yes | naming/API collision |
| `.agents/skills/rusty-idd-knowledge/SKILL.md` | yes | yes | naming/API collision |
| `.cargo/audit.toml` | yes | yes | naming/API collision |
| `.claude/agent-guard.toml` | yes | yes | naming/API collision |
| `.claude/rules/meta-destructive-commands.md` | yes | yes | naming/API collision |
| `.codex/agents/rusty-idd-explorer.toml` | yes | yes | naming/API collision |
| `.codex/agents/rusty-idd-gap-hunter.toml` | yes | yes | naming/API collision |
| `.codex/agents/rusty-idd-implementer.toml` | yes | yes | naming/API collision |
| `.codex/agents/rusty-idd-verifier.toml` | yes | yes | naming/API collision |
| `.codex/config.toml` | yes | yes | naming/API collision |
| `.codex/hooks.json` | yes | yes | naming/API collision |
| `.codex/loops/rusty-idd-model-loop.toml` | yes | yes | naming/API collision |
| `.codex/rules/default.rules` | yes | yes | naming/API collision |
| `.env.contract.yaml` | yes | yes | naming/API collision |
| `.env.schema.example.json` | yes | yes | naming/API collision |
| `.gitattributes` | yes | yes | naming/API collision |
| `.githooks/commit-msg` | yes | yes | naming/API collision |
| `.githooks/pre-commit` | yes | yes | naming/API collision |
| `.githooks/pre-push` | yes | yes | naming/API collision |
| `.github/CODEOWNERS` | yes | yes | naming/API collision |
| `.github/ISSUE_TEMPLATE/idd-task.yml` | yes | yes | naming/API collision |
| `.github/codeql/codeql-config.yml` | yes | yes | naming/API collision |
| `.github/copilot-instructions.md` | yes | yes | naming/API collision |
| `.github/dependabot.yml` | yes | yes | naming/API collision |
| `.github/pull_request_template.md` | yes | yes | naming/API collision |
| `.github/workflows/ci.yml` | yes | yes | naming/API collision |
| `.github/workflows/codeql.yml` | yes | yes | naming/API collision |
| `.github/workflows/on-push-main.yml` | yes | yes | naming/API collision |
| `.github/workflows/promote-verify.yml` | yes | yes | naming/API collision |
| `.github/workflows/release.yml` | yes | yes | naming/API collision |
| `.github/workflows/semantic-pr-title.yml` | yes | yes | naming/API collision |
| `.gitignore` | yes | yes | naming/API collision |
| `.handoff/README.md` | yes | yes | naming/API collision |
| `.handoff/context/capsule.json` | yes | yes | naming/API collision |
| `.handoff/packets/.gitkeep` | yes | yes | naming/API collision |
| `.handoff/tasks/.gitkeep` | yes | yes | naming/API collision |
| `.idd/LOCK.md` | yes | yes | naming/API collision |
| `.idd/evidence/autonomous-workflow/pr.md` | yes | yes | naming/API collision |
| `.idd/evidence/autonomous-workflow/task.md` | yes | yes | naming/API collision |
| `.idd/evidence/autonomous-workflow/validation.md` | yes | yes | naming/API collision |
| `.idd/goals/comprehensive-e2e-test-suite.md` | yes | yes | naming/API collision |
| `.idd/goals/grit-full-integration.md` | yes | yes | naming/API collision |
| `.idd/goals/rusty-idd-handoff-single-repo.md` | yes | yes | naming/API collision |
| `.idd/knowledge/architecture.json` | yes | yes | naming/API collision |
| `.idd/knowledge/architecture.md` | yes | yes | naming/API collision |
| `.idd/knowledge/index.json` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-owners.json` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-owners.md` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-plan.json` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-plan.md` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-readiness.json` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-readiness.md` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-status.json` | yes | yes | naming/API collision |
| `.idd/knowledge/integration-status.md` | yes | yes | naming/API collision |
| `.idd/knowledge/operating-model.json` | yes | yes | naming/API collision |
| `.idd/knowledge/operating-model.md` | yes | yes | naming/API collision |
| `.idd/knowledge/plan-context.json` | yes | yes | naming/API collision |
| `.idd/knowledge/plan-context.md` | yes | yes | naming/API collision |
| `.idd/knowledge/report.md` | yes | yes | naming/API collision |
| `.idd/knowledge/system-architecture.json` | yes | yes | naming/API collision |
| `.idd/knowledge/system-architecture.md` | yes | yes | naming/API collision |
| `.idd/workflow/active-change` | yes | yes | naming/API collision |
| `.release-please-manifest.json` | yes | yes | naming/API collision |
| `AGENTS.md` | yes | yes | naming/API collision |
| `AI_MERGE/03_env_and_secret_contracts.md` | yes | yes | naming/API collision |
| `AI_MERGE/04_merge_plan.md` | yes | yes | naming/API collision |
| `AI_MERGE/08_agent_queue.md` | yes | yes | naming/API collision |
| `AI_MERGE/10_parity_test_plan.md` | yes | yes | naming/API collision |
| `AI_MERGE/11_integration_research_audit_roadmap.md` | yes | yes | naming/API collision |
| `AI_MERGE/12_knowledge_deep_audit.md` | yes | yes | naming/API collision |
| `AI_MERGE/13_codex_environment.md` | yes | yes | naming/API collision |
| `AI_MERGE/14_upstream_full_adoption.md` | yes | yes | naming/API collision |
| `AI_MERGE/15_system_handoff_research.md` | yes | yes | naming/API collision |
| `AI_MERGE/16_upstream_knowledge_revisit.md` | yes | yes | naming/API collision |
| `AI_MERGE/17_architecture_graph_workflow.md` | yes | yes | naming/API collision |
| `AI_MERGE/18_system_architecture_peer_graph.md` | yes | yes | naming/API collision |
| `AI_MERGE/19_graph_context_planning.md` | yes | yes | naming/API collision |
| `AI_MERGE/20_peer_architecture_detail_ingestion.md` | yes | yes | naming/API collision |
| `AI_MERGE/21_system_operating_model_graph.md` | yes | yes | naming/API collision |
| `AI_MERGE/22_integration_automation_plan.md` | yes | yes | naming/API collision |
| `AI_MERGE/23_integration_work_scaffold.md` | yes | yes | naming/API collision |
| `AI_MERGE/24_integration_status_queue.md` | yes | yes | naming/API collision |
| `AI_MERGE/25_queue_aware_plan_integration.md` | yes | yes | naming/API collision |
| `AI_MERGE/26_idd_spec_engine_automation.md` | yes | yes | naming/API collision |
| `AI_MERGE/27_archive_idd_spec_engine.md` | yes | yes | naming/API collision |
| `AI_MERGE/28_fleet_handoff_owner_surfaces.md` | yes | yes | naming/API collision |
| `AI_MERGE/29_agent_communication_queue_owner_surfaces.md` | yes | yes | naming/API collision |
| `AI_MERGE/30_env_vault_relay_readiness.md` | yes | yes | naming/API collision |
| `AI_MERGE/31_prompt_front_door_upstream_adoption.md` | yes | yes | naming/API collision |
| `AI_MERGE/32_autonomous_workflow_hooks.md` | yes | yes | naming/API collision |
| `AI_MERGE/33_architecture_diagram_artifacts.md` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/00_grit_inventory.json` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/00_grit_inventory.md` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/01_rusty_idd_inventory_before_adoption.json` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/01_rusty_idd_inventory_before_adoption.md` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/README.md` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/adoption-evidence.md` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/plan-workspace/.env.contract.yaml` | yes | yes | naming/API collision |
| `AI_MERGE/34_grit_full_integration/plan-workspace/.env.schema.example.json` | yes | yes | naming/API collision |
