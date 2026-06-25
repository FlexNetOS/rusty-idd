# Repository Inventory: prompt_hub

- Root: `/home/drdave/Desktop/meta/prompt_hub`
- Files scanned: `420`

## Category Counts

| Category | Count |
|---|---:|
| source | 140 |
| config | 103 |
| workflow | 13 |
| documentation | 106 |
| test | 30 |
| build | 8 |
| lockfile | 1 |
| agent-control | 3 |
| security | 3 |
| unknown | 13 |

## Languages

| Language | Files |
|---|---:|
| Python | 3 |
| Rust | 142 |
| SQL | 9 |
| Shell | 9 |

## Package Managers / Toolchains

- `cargo`

## Entrypoints

- _none detected_

## Workflows

- `.github/workflows/ai-code-review.yml`
- `.github/workflows/ai-safety-deployment.yml`
- `.github/workflows/ai-test-doc-generation.yml`
- `.github/workflows/audit_sync.yml`
- `.github/workflows/ci.yml`
- `.github/workflows/external-ai-apis.yml`
- `.github/workflows/multi-model-evaluation.yml`
- `.github/workflows/mutation.yml`
- `.github/workflows/qodana_code_quality.yml`
- `.github/workflows/rust_native_guard.yml`
- `.github/workflows/security.yml`
- `.github/workflows/security_remediation.yml`

## Agent Control Files

- `.claude/skills/prompt-loop/handoff/templates/AGENTS.md`
- `.junie/AGENTS.md`
- `AGENTS.md`

## Security Files

- `.github/CODEOWNERS`
- `.github/dependabot.yml`
- `SECURITY.md`

## Environment Keys Found

- `ANTHROPIC_API_KEY`
- `DEVIN_API_KEY`
- `ENABLE_AI_WORKFLOWS`
- `GITHUB_TOKEN`
- `OUT_DIR`
- `PROMPTHUB_CONFIG`
- `PROMPTHUB_DB_PATH`
- `QODANA_TOKEN`
- `VAULT_OR_OPENBAO`

## Secret / Env References Found

| File | Key | Source |
|---|---|---|
| `.github/workflows/README.md` | `ANTHROPIC_API_KEY` | github-actions-secret |
| `.github/workflows/README.md` | `DEVIN_API_KEY` | github-actions-secret |
| `.github/workflows/README.md` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/ai-code-review.yml` | `ENABLE_AI_WORKFLOWS` | github-actions-variable |
| `.github/workflows/ai-code-review.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/ai-safety-deployment.yml` | `ENABLE_AI_WORKFLOWS` | github-actions-variable |
| `.github/workflows/ai-safety-deployment.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/ai-test-doc-generation.yml` | `ENABLE_AI_WORKFLOWS` | github-actions-variable |
| `.github/workflows/ai-test-doc-generation.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/external-ai-apis.yml` | `ANTHROPIC_API_KEY` | github-actions-secret |
| `.github/workflows/external-ai-apis.yml` | `DEVIN_API_KEY` | github-actions-secret |
| `.github/workflows/external-ai-apis.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/multi-model-evaluation.yml` | `ENABLE_AI_WORKFLOWS` | github-actions-variable |
| `.github/workflows/multi-model-evaluation.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/qodana_code_quality.yml` | `QODANA_TOKEN` | github-actions-secret |
| `.github/workflows/security_remediation.yml` | `ENABLE_AI_WORKFLOWS` | github-actions-variable |
| `.github/workflows/security_remediation.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `docker/docker-compose.yml` | `DOCKER_COMPOSE_ENV_FILE` | docker-compose-env-file |
| `prompt-hub/src/config.rs` | `PROMPTHUB_CONFIG` | std::env::var |
| `prompt-hub/templates/env_state_convergence.md` | `VAULT_OR_OPENBAO` | vault-or-openbao |
| `prompthub-server/build.rs` | `OUT_DIR` | std::env::var |
| `prompthub-server/src/main.rs` | `PROMPTHUB_DB_PATH` | std::env::var |
| `prompts/env-state-convergence.prompt.yml` | `VAULT_OR_OPENBAO` | vault-or-openbao |

## File Index

| Path | Category | Size |
|---|---|---:|
| `.agent.md` | documentation | 3279 |
| `.cargo/config.toml` | config | 353 |
| `.cargo/mutants.toml` | config | 326 |
| `.claude/agents/backlog-curator.md` | documentation | 5209 |
| `.claude/agents/continuity-steward.md` | documentation | 3707 |
| `.claude/agents/docs-scribe.md` | documentation | 3763 |
| `.claude/agents/evolution-steward.md` | documentation | 9083 |
| `.claude/agents/feature-architect.md` | documentation | 4506 |
| `.claude/agents/rust-implementer.md` | documentation | 4525 |
| `.claude/agents/verification-gate.md` | documentation | 5247 |
| `.claude/settings.local.json` | config | 481 |
| `.claude/skills/feature-build/SKILL.md` | documentation | 5914 |
| `.claude/skills/feature-build/references/boundary-checks.md` | documentation | 3940 |
| `.claude/skills/feature-build/references/rust-native-checklist.md` | documentation | 2911 |
| `.claude/skills/harness-evolution/SKILL.md` | documentation | 5834 |
| `.claude/skills/lane-loop/SKILL.md` | documentation | 4324 |
| `.claude/skills/prompt-loop/SKILL.md` | documentation | 19083 |
| `.claude/skills/prompt-loop/handoff/docs/sessions-handoff.md` | documentation | 29184 |
| `.claude/skills/prompt-loop/handoff/examples/hf-resume-output.json` | config | 705 |
| `.claude/skills/prompt-loop/handoff/manifest.txt` | documentation | 386 |
| `.claude/skills/prompt-loop/handoff/roadmap/backlog.yaml` | config | 766 |
| `.claude/skills/prompt-loop/handoff/schemas/packet.schema.json` | config | 1410 |
| `.claude/skills/prompt-loop/handoff/schemas/session.schema.json` | config | 943 |
| `.claude/skills/prompt-loop/handoff/schemas/task.schema.json` | config | 1265 |
| `.claude/skills/prompt-loop/handoff/templates/.handoff/hooks/hooks.toml` | config | 737 |
| `.claude/skills/prompt-loop/handoff/templates/.handoff/policies/rules.toml` | config | 795 |
| `.claude/skills/prompt-loop/handoff/templates/.handoff/skills/session-resume.skill.md` | documentation | 447 |
| `.claude/skills/prompt-loop/handoff/templates/.handoff/tasks/TASK-0001.task.yaml` | config | 725 |
| `.claude/skills/prompt-loop/handoff/templates/AGENTS.md` | agent-control | 896 |
| `.claude/skills/prompt-loop/scripts/ralph-prompt.sh` | source | 5470 |
| `.claude/skills/session-relay/SKILL.md` | documentation | 8058 |
| `.cliff.toml` | config | 2083 |
| `.deny.toml` | config | 978 |
| `.githooks/pre-commit` | unknown | 699 |
| `.github/CODEOWNERS` | security | 1615 |
| `.github/ISSUE_TEMPLATE/bug_report.md` | documentation | 893 |
| `.github/ISSUE_TEMPLATE/feature_request.md` | documentation | 1089 |
| `.github/dependabot.yml` | security | 902 |
| `.github/pull_request_template.md` | documentation | 2210 |
| `.github/workflows/README.md` | workflow | 11980 |
| `.github/workflows/ai-code-review.yml` | workflow | 4625 |
| `.github/workflows/ai-safety-deployment.yml` | workflow | 10288 |
| `.github/workflows/ai-test-doc-generation.yml` | workflow | 9850 |
| `.github/workflows/audit_sync.yml` | workflow | 3020 |
| `.github/workflows/ci.yml` | workflow | 5356 |
| `.github/workflows/external-ai-apis.yml` | workflow | 7238 |
| `.github/workflows/multi-model-evaluation.yml` | workflow | 7341 |
| `.github/workflows/mutation.yml` | workflow | 786 |
| `.github/workflows/qodana_code_quality.yml` | workflow | 1872 |
| `.github/workflows/rust_native_guard.yml` | workflow | 1525 |
| `.github/workflows/security.yml` | workflow | 775 |
| `.github/workflows/security_remediation.yml` | workflow | 3322 |
| `.gitignore` | unknown | 437 |
| `.handoff/.gitignore` | unknown | 584 |
| `.handoff/README.md` | documentation | 3597 |
| `.handoff/active.md` | documentation | 455 |
| `.handoff/context/capsule.json` | config | 1140 |
| `.handoff/decisions/ADR-0001-adopt-handoff-kernel.md` | documentation | 3930 |
| `.handoff/deliveries/prompt-hub-construction.delivery.json` | config | 647 |
| `.handoff/history/LESSONS.md` | documentation | 1527 |
| `.handoff/history/SESSION-2026-06-13.md` | documentation | 3288 |
| `.handoff/history/SESSION-2026-06-18.md` | documentation | 6247 |
| `.handoff/history/_workspace-archive/77_architect_plan.md` | documentation | 10580 |
| `.handoff/history/_workspace-archive/HANDOFF.md` | documentation | 7265 |
| `.handoff/history/_workspace-archive/README.md` | documentation | 1647 |
| `.handoff/history/_workspace-archive/audit/gap_analysis.md` | documentation | 12298 |
| `.handoff/history/_workspace-archive/backlog.md` | documentation | 19710 |
| `.handoff/history/_workspace-archive/c1_architect_plan.md` | documentation | 8601 |
| `.handoff/history/_workspace-archive/c1_verification_report.md` | documentation | 3679 |
| `.handoff/history/_workspace-archive/c2_architect_plan.md` | documentation | 11584 |
| `.handoff/history/_workspace-archive/c2_verification_report.md` | documentation | 5456 |
| `.handoff/history/_workspace-archive/c3_architect_plan.md` | documentation | 15047 |
| `.handoff/history/_workspace-archive/c3_verification_report.md` | documentation | 4740 |
| `.handoff/history/_workspace-archive/c4_architect_plan.md` | documentation | 11929 |
| `.handoff/history/_workspace-archive/cycle_64_architect_plan.md` | documentation | 25923 |
| `.handoff/history/_workspace-archive/cycle_64_implementer_notes.md` | documentation | 2252 |
| `.handoff/history/_workspace-archive/cycle_65_architect_plan.md` | documentation | 16496 |
| `.handoff/history/_workspace-archive/cycle_65_implementer_notes.md` | documentation | 5637 |
| `.handoff/history/_workspace-archive/cycle_66_architect_plan.md` | documentation | 15783 |
| `.handoff/history/_workspace-archive/cycle_66_implementer_notes.md` | documentation | 5903 |
| `.handoff/history/_workspace-archive/cycle_67_architect_plan.md` | documentation | 18054 |
| `.handoff/history/_workspace-archive/cycle_67_implementer_notes.md` | documentation | 4081 |
| `.handoff/history/_workspace-archive/cycle_82_implementer_notes.md` | documentation | 4512 |
| `.handoff/history/_workspace-archive/cycle_83_implementer_notes.md` | documentation | 4990 |
| `.handoff/history/_workspace-archive/designs/accessibility-output-formatting.md` | documentation | 18780 |
| `.handoff/history/_workspace-archive/loop_state.md` | documentation | 6078 |
| `.handoff/history/_workspace-archive/loop_state.md.bak` | unknown | 911 |
| `.handoff/history/_workspace-archive/s11c1_architect_plan.md` | documentation | 11038 |
| `.handoff/history/_workspace-archive/s11c3_architect_plan.md` | documentation | 9149 |
| `.handoff/history/_workspace-archive/s3c3_architect_plan.md` | documentation | 3694 |
| `.handoff/history/generate_cards.py` | source | 16480 |
| `.handoff/hooks/hooks.toml` | config | 2388 |
| `.handoff/hooks/loop-entry.sh` | source | 2721 |
| `.handoff/hooks/session-end.sh` | source | 1044 |
| `.handoff/ledger.db` | unknown | 36864 |
| `.handoff/locks/handoff_claim_PHTASK-0048.lock` | unknown | 112 |
| `.handoff/locks/handoff_claim_PHTASK-0055.lock` | unknown | 112 |
| `.handoff/packets/.gitkeep` | unknown | 0 |
| `.handoff/packets/latest.md` | test | 1935 |
| `.handoff/policies/rules.toml` | config | 2388 |
| `.handoff/policy.toml` | config | 1591 |
| `.handoff/skills/session-resume.skill.md` | documentation | 861 |
| `.handoff/tasks/.gitkeep` | unknown | 0 |
| `.handoff/tasks/PHTASK-0001.task.json` | config | 1252 |
| `.handoff/tasks/PHTASK-0002.task.json` | config | 1332 |
| `.handoff/tasks/PHTASK-0003.task.json` | config | 1275 |
| `.handoff/tasks/PHTASK-0004.task.json` | config | 1297 |
| `.handoff/tasks/PHTASK-0005.task.json` | config | 1301 |
| `.handoff/tasks/PHTASK-0006.task.json` | config | 1348 |
| `.handoff/tasks/PHTASK-0007.task.json` | config | 1357 |
| `.handoff/tasks/PHTASK-0008.task.json` | config | 1338 |
| `.handoff/tasks/PHTASK-0009.task.json` | config | 1316 |
| `.handoff/tasks/PHTASK-0010.task.json` | config | 1339 |
| `.handoff/tasks/PHTASK-0011.task.json` | config | 1360 |
| `.handoff/tasks/PHTASK-0012.task.json` | config | 1340 |
| `.handoff/tasks/PHTASK-0013.task.json` | config | 1351 |
| `.handoff/tasks/PHTASK-0014.task.json` | config | 1333 |
| `.handoff/tasks/PHTASK-0015.task.json` | config | 1358 |
| `.handoff/tasks/PHTASK-0016.task.json` | config | 1352 |
| `.handoff/tasks/PHTASK-0017.task.json` | config | 1321 |
| `.handoff/tasks/PHTASK-0018.task.json` | config | 1321 |
| `.handoff/tasks/PHTASK-0019.task.json` | config | 1338 |
| `.handoff/tasks/PHTASK-0020.task.json` | config | 1360 |
| `.handoff/tasks/PHTASK-0021.task.json` | config | 1373 |
| `.handoff/tasks/PHTASK-0022.task.json` | config | 1210 |
| `.handoff/tasks/PHTASK-0023.task.json` | config | 1224 |
| `.handoff/tasks/PHTASK-0024.task.json` | config | 1302 |
| `.handoff/tasks/PHTASK-0025.task.json` | config | 1180 |
| `.handoff/tasks/PHTASK-0026.task.json` | config | 1258 |
| `.handoff/tasks/PHTASK-0027.task.json` | config | 1329 |
| `.handoff/tasks/PHTASK-0028.task.json` | config | 1859 |
| `.handoff/tasks/PHTASK-0029.task.json` | config | 3367 |
| `.handoff/tasks/PHTASK-0030.task.json` | config | 3815 |
| `.handoff/tasks/PHTASK-0031.task.json` | config | 2406 |
| `.handoff/tasks/PHTASK-0032.task.json` | config | 3898 |
| `.handoff/tasks/PHTASK-0033.task.json` | config | 2742 |
| `.handoff/tasks/PHTASK-0034.task.json` | config | 2937 |
| `.handoff/tasks/PHTASK-0035.task.json` | config | 2125 |
| `.handoff/tasks/PHTASK-0036.task.json` | config | 3025 |
| `.handoff/tasks/PHTASK-0037.task.json` | config | 3375 |
| `.handoff/tasks/PHTASK-0038.task.json` | config | 2246 |
| `.handoff/tasks/PHTASK-0039.task.json` | config | 2251 |
| `.handoff/tasks/PHTASK-0040.task.json` | config | 2629 |
| `.handoff/tasks/PHTASK-0041.task.json` | config | 3998 |
| `.handoff/tasks/PHTASK-0042.task.json` | config | 2923 |
| `.handoff/tasks/PHTASK-0043.task.json` | config | 3006 |
| `.handoff/tasks/PHTASK-0044.task.json` | config | 2754 |
| `.handoff/tasks/PHTASK-0045.task.json` | config | 2389 |
| `.handoff/tasks/PHTASK-0046.task.json` | config | 2312 |
| `.handoff/tasks/PHTASK-0047.task.json` | config | 2271 |
| `.handoff/tasks/PHTASK-0048.task.json` | config | 2107 |
| `.handoff/tasks/PHTASK-0049.task.json` | config | 1871 |
| `.handoff/tasks/PHTASK-0050.task.json` | config | 1792 |
| `.handoff/tasks/PHTASK-0051.task.json` | config | 1992 |
| `.handoff/tasks/PHTASK-0052.task.json` | config | 1979 |
| `.handoff/tasks/PHTASK-0053.task.json` | config | 1908 |
| `.handoff/tasks/PHTASK-0054.task.json` | config | 1788 |
| `.handoff/tasks/PHTASK-0055.task.json` | config | 2143 |
| `.handoff/tasks/PHTASK-0056.task.json` | config | 2069 |
| `.handoff/tasks/PHTASK-0057.task.json` | config | 1564 |
| `.handoff/tasks/PHTASK-0058.task.json` | config | 1496 |
| `.handoff/tasks/PHTASK-0059.task.json` | config | 1638 |
| `.handoff/tasks/PHTASK-0060.task.json` | config | 1529 |
| `.handoff/tasks/PHTASK-0061.task.json` | config | 1893 |
| `.handoff/tasks/PHTASK-0062.task.json` | config | 1799 |
| `.handoff/tasks/PHTASK-0063.task.json` | config | 2361 |
| `.handoff/tasks/PHTASK-0064.task.json` | config | 1570 |
| `.handoff/tasks/PHTASK-0065.task.json` | config | 2240 |
| `.handoff/tasks/PHTASK-0066.task.json` | config | 2258 |
| `.handoff/tasks/PHTASK-0067.task.json` | config | 2022 |
| `.handoff/tasks/PHTASK-0068.task.json` | config | 2151 |
| `.handoff/tasks/PHTASK-0069.task.json` | config | 2446 |
| `.handoff/tasks/PHTASK-0070.task.json` | config | 2226 |
| `.handoff/tasks/PHTASK-0071.task.json` | config | 2396 |
| `.instructions.md` | documentation | 6989 |
| `.junie/AGENTS.md` | agent-control | 6164 |
| `.kb/store/commits/019ea7b9-50da-7540-af8c-8d74c70fa354.json` | config | 734 |
| `.kb/store/documents/tasks/lane-loop-handoff.md` | documentation | 1324 |
| `.kb/store/manifest.json` | config | 162 |
| `.kb/store/refs/document-tips.json` | config | 214 |
| `.kb/workspaces/main/tasks/lane-loop-handoff.md` | documentation | 1324 |
| `.prompt.md` | documentation | 10078 |
| `AGENTS.md` | agent-control | 10141 |
| `AGENT_GUIDE.md` | documentation | 5971 |
| `AI_MODELS_QUICK_START.md` | documentation | 5218 |
| `CHANGELOG.md` | documentation | 2334 |
| `CLAUDE.md` | documentation | 12802 |
| `CONTRIBUTING.md` | documentation | 5823 |
| `Cargo.lock` | lockfile | 143693 |
| `Cargo.toml` | build | 4562 |
| `GEMINI.md` | documentation | 2494 |
| `LESSONS.md` | documentation | 911 |
| `LICENSE-APACHE` | unknown | 11356 |
| `LICENSE-MIT` | unknown | 1079 |
| `README.md` | documentation | 5802 |
| `SECURITY.md` | security | 629 |
| `SESSION.md` | documentation | 10871 |
| `SPEC.md` | test | 7529 |
| `TODO.md` | documentation | 7935 |
| `VERIFICATION_REPORT.md` | documentation | 7194 |
| `benches/search_latency.rs` | source | 3810 |
| `docker/Dockerfile` | build | 2232 |
| `docker/docker-compose.yml` | config | 1087 |
| `docs/adr/0001-why-sqlite.md` | documentation | 872 |
| `docs/adr/0002-why-rust-not-python.md` | documentation | 719 |
| `docs/adr/0003-embedding-strategy.md` | documentation | 374 |
| `docs/adr/0004-async-architecture.md` | documentation | 334 |
| `docs/adr/0005-security-model.md` | documentation | 327 |
| `docs/adr/0006-data-flow.md` | documentation | 340 |
| `docs/adr/0007-plugin-system.md` | documentation | 257 |
| `docs/adr/0008-vibe-coding-architecture.md` | documentation | 409 |
| `docs/architecture.md` | documentation | 2969 |
| `docs/audits/Test cli.run.xml` | test | 12502 |
| `docs/audits/qodana.sarif.json` | config | 1418155 |
| `docs/deployment.md` | documentation | 401 |
| `docs/runbooks/incident_response.md` | documentation | 413 |
| `docs/runbooks/onboarding.md` | documentation | 237 |
| `examples/auto_context_demo.rs` | source | 371 |
| `examples/basic_usage.rs` | source | 1364 |
| `examples/confidence_scoring_demo.rs` | source | 437 |
| `examples/evolution_demo.rs` | source | 1319 |
| `examples/multimodal_input_demo.rs` | source | 495 |
| `examples/plugin_example.rs` | source | 678 |
| `examples/safe_deploy_demo.rs` | source | 448 |
| `examples/server_client.rs` | source | 472 |
| `examples/swarm_demo.rs` | source | 420 |
| `examples/vibe_coding_demo.rs` | source | 6356 |
| `justfile` | build | 1609 |
| `plan.md` | documentation | 1810 |
| `plan_wave2.md` | documentation | 851 |
| `plugins/example_sanitizer/Cargo.toml` | build | 131 |
| `plugins/example_sanitizer/src/lib.rs` | source | 467 |
| `plugins/example_search_backend/Cargo.toml` | build | 136 |
| `plugins/example_search_backend/src/lib.rs` | source | 485 |
| `prompt-hub/Cargo.toml` | build | 4391 |
| `prompt-hub/GEMINI.md` | documentation | 1665 |
| `prompt-hub/README.md` | documentation | 4159 |
| `prompt-hub/benches/db_write_throughput.rs` | source | 6178 |
| `prompt-hub/benches/embedding_generation.rs` | source | 3265 |
| `prompt-hub/benches/search_latency.rs` | source | 3912 |
| `prompt-hub/build.rs` | source | 559 |
| `prompt-hub/migrations/0001_initial.sql` | source | 4001 |
| `prompt-hub/migrations/0002_audit.sql` | source | 621 |
| `prompt-hub/migrations/0003_locks.sql` | source | 550 |
| `prompt-hub/migrations/0004_swarm_state.sql` | source | 516 |
| `prompt-hub/migrations/0005_backup_meta.sql` | source | 370 |
| `prompt-hub/migrations/0006_plugins.sql` | source | 506 |
| `prompt-hub/migrations/0007_soft_delete.sql` | source | 379 |
| `prompt-hub/migrations/0008_generation_params.sql` | source | 2077 |
| `prompt-hub/migrations/0009_config.sql` | source | 282 |
| `prompt-hub/models.json` | config | 241 |
| `prompt-hub/prompthub.db` | unknown | 192512 |
| `prompt-hub/src/accessibility.rs` | source | 39718 |
| `prompt-hub/src/analytics.rs` | source | 17230 |
| `prompt-hub/src/audit.rs` | source | 21729 |
| `prompt-hub/src/auth.rs` | source | 17486 |
| `prompt-hub/src/auto_purge.rs` | source | 24480 |
| `prompt-hub/src/beta_program.rs` | source | 11392 |
| `prompt-hub/src/budget.rs` | source | 7709 |
| `prompt-hub/src/chaos.rs` | source | 27912 |
| `prompt-hub/src/chaos_auto.rs` | source | 34515 |
| `prompt-hub/src/circuit_breaker.rs` | source | 8064 |
| `prompt-hub/src/confidence.rs` | source | 10273 |
| `prompt-hub/src/config.rs` | source | 5348 |
| `prompt-hub/src/context_gatherer.rs` | source | 19878 |
| `prompt-hub/src/cost.rs` | source | 9487 |
| `prompt-hub/src/cost_limits.rs` | source | 14307 |
| `prompt-hub/src/defaults.rs` | source | 9287 |
| `prompt-hub/src/diff.rs` | source | 10538 |
| `prompt-hub/src/error.rs` | source | 4388 |
| `prompt-hub/src/evolution.rs` | source | 12687 |
| `prompt-hub/src/fallback.rs` | source | 15653 |
| `prompt-hub/src/garbage_collector.rs` | source | 17350 |
| `prompt-hub/src/gather.rs` | source | 32605 |
| `prompt-hub/src/gradual_rollout.rs` | source | 8253 |
| `prompt-hub/src/health.rs` | source | 3644 |
| `prompt-hub/src/hooks.rs` | source | 10104 |
| `prompt-hub/src/hub.rs` | source | 185316 |
| `prompt-hub/src/i18n.rs` | source | 10737 |
| `prompt-hub/src/junie.rs` | source | 1429 |
| `prompt-hub/src/learn.rs` | source | 10121 |
| `prompt-hub/src/lib.rs` | source | 4481 |
| `prompt-hub/src/lineage.rs` | source | 12898 |
| `prompt-hub/src/load_balancer.rs` | source | 10460 |
| `prompt-hub/src/local_llm/engine.rs` | source | 9063 |
| `prompt-hub/src/local_llm/inference.rs` | source | 6728 |
| `prompt-hub/src/local_llm/mod.rs` | source | 522 |
| `prompt-hub/src/lock.rs` | source | 18550 |
| `prompt-hub/src/malware_scan.rs` | source | 21080 |
| `prompt-hub/src/metrics.rs` | source | 23709 |
| `prompt-hub/src/mobile.rs` | source | 17554 |
| `prompt-hub/src/models.rs` | source | 46001 |
| `prompt-hub/src/moderation.rs` | source | 9426 |
| `prompt-hub/src/multi_provider.rs` | source | 14400 |
| `prompt-hub/src/multimodal.rs` | source | 9921 |
| `prompt-hub/src/multimodal_input.rs` | source | 13423 |
| `prompt-hub/src/offline.rs` | source | 44404 |
| `prompt-hub/src/plugins.rs` | source | 16520 |
| `prompt-hub/src/pollination.rs` | source | 13199 |
| `prompt-hub/src/preview.rs` | source | 16275 |
| `prompt-hub/src/privacy.rs` | source | 12805 |
| `prompt-hub/src/provider_health.rs` | source | 11010 |
| `prompt-hub/src/qdrant.rs` | source | 31055 |
| `prompt-hub/src/quality_gate.rs` | source | 15990 |
| `prompt-hub/src/quota.rs` | source | 8824 |
| `prompt-hub/src/retention.rs` | source | 8628 |
| `prompt-hub/src/rollback.rs` | source | 11294 |
| `prompt-hub/src/sandbox.rs` | source | 16753 |
| `prompt-hub/src/sanitize.rs` | source | 35742 |
| `prompt-hub/src/satisfaction.rs` | source | 11306 |
| `prompt-hub/src/search.rs` | source | 73650 |
| `prompt-hub/src/shutdown.rs` | source | 8952 |
| `prompt-hub/src/storage.rs` | source | 79204 |
| `prompt-hub/src/summarizer.rs` | source | 9402 |
| `prompt-hub/src/swarm.rs` | source | 28089 |
| `prompt-hub/src/sync.rs` | source | 29170 |
| `prompt-hub/src/templates.rs` | source | 10331 |
| `prompt-hub/src/tokens.rs` | source | 8336 |
| `prompt-hub/src/touch.rs` | source | 17959 |
| `prompt-hub/src/vibe.rs` | source | 46698 |
| `prompt-hub/src/voice.rs` | source | 26002 |
| `prompt-hub/src/voice_anonymize.rs` | source | 19288 |
| `prompt-hub/templates/base_architect.md` | documentation | 756 |
| `prompt-hub/templates/base_critic.md` | documentation | 905 |
| `prompt-hub/templates/base_implementer.md` | documentation | 830 |
| `prompt-hub/templates/base_orchestrator.md` | documentation | 578 |
| `prompt-hub/templates/base_reviewer.md` | documentation | 764 |
| `prompt-hub/templates/env_state_convergence.md` | documentation | 2731 |
| `prompt-hub/templates/handoff_standard.md` | documentation | 248 |
| `prompt-hub/test.db` | test | 192512 |
| `prompt-hub/tests/fixtures/generate_embed_model.py` | test | 3735 |
| `prompt-hub/tests/fixtures/test_embedder.json` | test | 263 |
| `prompt-hub/tests/fixtures/test_embedder.onnx` | test | 8852 |
| `prompt-hub/tests/test_accessibility.rs` | test | 6308 |
| `prompt-hub/tests/test_auto_purge.rs` | test | 2216 |
| `prompt-hub/tests/test_chaos.rs` | test | 3836 |
| `prompt-hub/tests/test_chaos_auto.rs` | test | 4412 |
| `prompt-hub/tests/test_get_rbac.rs` | test | 4961 |
| `prompt-hub/tests/test_hub.rs` | test | 3925 |
| `prompt-hub/tests/test_malware_scan.rs` | test | 3474 |
| `prompt-hub/tests/test_models.rs` | test | 10420 |
| `prompt-hub/tests/test_offline.rs` | test | 2265 |
| `prompt-hub/tests/test_qdrant.rs` | test | 4787 |
| `prompt-hub/tests/test_search.rs` | test | 7159 |
| `prompt-hub/tests/test_security.rs` | test | 6431 |
| `prompt-hub/tests/test_touch.rs` | test | 6844 |
| `prompt-hub/tests/test_voice.rs` | test | 4053 |
| `prompt-hub/tests/test_voice_anonymize.rs` | test | 2829 |
| `prompthub-server/Cargo.toml` | build | 2324 |
| `prompthub-server/GEMINI.md` | documentation | 1286 |
| `prompthub-server/build.rs` | source | 1034 |
| `prompthub-server/src/main.rs` | source | 4435 |
| `prompthub-server/src/middleware.rs` | source | 6066 |
| `prompthub-server/src/openapi.rs` | source | 68188 |
| `prompthub-server/src/responses.rs` | source | 2501 |
| `prompthub-server/src/routes.rs` | source | 198552 |
| `prompthub-server/src/server.rs` | source | 15153 |
| `prompthub-server/src/state.rs` | source | 1164 |
| `prompthub.db` | unknown | 208896 |
| `prompthub/Cargo.toml` | build | 2585 |
| `prompthub/GEMINI.md` | documentation | 1084 |
| `prompthub/src/cli.rs` | source | 8827 |
| `prompthub/src/commands/add.rs` | source | 2637 |
| `prompthub/src/commands/budget.rs` | source | 3742 |
| `prompthub/src/commands/cache.rs` | source | 1376 |
| `prompthub/src/commands/cost.rs` | source | 883 |
| `prompthub/src/commands/deploy.rs` | source | 599 |
| `prompthub/src/commands/evolve.rs` | source | 610 |
| `prompthub/src/commands/export.rs` | source | 2964 |
| `prompthub/src/commands/feedback.rs` | source | 535 |
| `prompthub/src/commands/gather.rs` | source | 598 |
| `prompthub/src/commands/import.rs` | source | 3135 |
| `prompthub/src/commands/init.rs` | source | 917 |
| `prompthub/src/commands/junie.rs` | source | 1036 |
| `prompthub/src/commands/list.rs` | source | 1384 |
| `prompthub/src/commands/metrics.rs` | source | 1485 |
| `prompthub/src/commands/mod.rs` | source | 346 |
| `prompthub/src/commands/plugin.rs` | source | 3295 |
| `prompthub/src/commands/preview.rs` | source | 510 |
| `prompthub/src/commands/rollback.rs` | source | 546 |
| `prompthub/src/commands/search.rs` | source | 1140 |
| `prompthub/src/commands/vibe.rs` | source | 578 |
| `prompthub/src/fuzzy.rs` | source | 5660 |
| `prompthub/src/identity.rs` | source | 1199 |
| `prompthub/src/main.rs` | source | 17372 |
| `prompthub/src/tui.rs` | source | 323 |
| `prompthub/tests/cli_add_identity.rs` | test | 1289 |
| `prompthub/tests/cli_log_routing.rs` | test | 2174 |
| `prompts/README.md` | documentation | 7628 |
| `prompts/code-review-rust.prompt.yml` | config | 3640 |
| `prompts/debug-compilation.prompt.yml` | config | 4629 |
| `prompts/design-api-endpoint.prompt.yml` | config | 7798 |
| `prompts/design-migration.prompt.yml` | config | 5582 |
| `prompts/env-state-convergence.prompt.yml` | config | 6782 |
| `prompts/implement-feature.prompt.yml` | config | 3751 |
| `qodana.yaml` | config | 1778 |
| `rust-toolchain.toml` | config | 78 |
| `scripts/audit_watcher.sh` | source | 567 |
| `scripts/check_safety.sh` | source | 1263 |
| `scripts/code_review.sh` | source | 1476 |
| `scripts/drift_guard.sh` | source | 5027 |
| `scripts/setup.sh` | source | 4869 |
| `scripts/update_todo_from_audit.py` | source | 2905 |
| `skills/junie/SKILL.md` | documentation | 1677 |
| `skills/prompt-hub-dev/SKILL.md` | documentation | 1770 |
| `skills/prompt-hub-dev/references/architecture.md` | documentation | 894 |
| `skills/prompt-hub-dev/references/cli-command.md` | documentation | 361 |
| `skills/prompt-hub-dev/references/database.md` | documentation | 211 |
| `skills/prompt-hub-dev/references/hub-method.md` | documentation | 1012 |
| `skills/prompt-hub-dev/references/module-creation.md` | documentation | 318 |
| `skills/prompt-hub-dev/references/server-route.md` | documentation | 289 |
| `skills/prompt-hub-dev/references/testing.md` | test | 188 |
| `skills/prompt-hub-dev/scripts/check_safety.sh` | source | 788 |
| `skills/security-remediation/SKILL.md` | documentation | 7686 |
| `tests/test_end_to_end.rs` | test | 10449 |
| `tests/test_hub.rs` | test | 2479 |
| `tests/test_models.rs` | test | 9854 |
| `tests/test_search.rs` | test | 4668 |
| `tests/test_security.rs` | test | 5498 |
| `validation_log.txt` | documentation | 0 |
