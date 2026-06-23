# Knowledge Report

- Workspace fingerprint: `fnv1a64:af60691671d6d197`
- Indexed source files: 143
- Graph nodes: 8880
- Graph edges: 36560
- Resolved call edges: 20861
- Functions with complexity: 2232
- Packed files: 210
- Packed tokens: 678973
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (3983) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 3356 | `contains` (6132) | `crates/spec/src/model/spec.rs` | 337 graph links, 334 call links |
| 2871 | `walk` (3835) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2601 | `new` (6417) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2375 | `walk` (3908) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3753) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (3959) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3777) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (4050) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2025 | `implementation_loop` (5695) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 2022 | `visit_dir` (3236) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 14 unresolved calls, cyclomatic complexity 4 |
| 1707 | `handle_config_input` (6432) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1695 | `process_path_event` (3238) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 11 unresolved calls, cyclomatic complexity 35 |
| 1649 | `integration_readiness_report` (4866) | `crates/knowledge/src/lib.rs` | 336 lines, 123 graph links, 117 call links, 7 unresolved calls, cyclomatic complexity 26 |
| 1488 | `integration_owner_surfaces_join_work_item_to_system_repos` (5080) | `crates/knowledge/src/lib.rs` | 251 lines, 121 graph links, 120 call links, 10 unresolved calls |
| 1456 | `walk` (3810) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (7134) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (5654) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (1920) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1356 | `try_run` (291) | `crates/cli/src/commands/knowledge.rs` | 262 lines, 100 graph links, 97 call links, 13 unresolved calls, cyclomatic complexity 19 |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 102353 | `AI_MERGE/35_e2e_test_suite/plan-workspace/AI_MERGE/00_repo_a_inventory.md` |
| 102353 | `AI_MERGE/35_e2e_test_suite/plan-workspace/AI_MERGE/01_repo_b_inventory.md` |
| 101899 | `AI_MERGE/35_e2e_test_suite/00_rusty_idd_inventory_before.md` |
| 95914 | `AI_MERGE/34_grit_full_integration/plan-workspace/AI_MERGE/00_repo_a_inventory.md` |
| 95446 | `AI_MERGE/34_grit_full_integration/01_rusty_idd_inventory_before_adoption.md` |
| 9464 | `crates/tui/src/app.rs` |
| 8632 | `crates/knowledge/src/lib.rs` |
| 7982 | `AI_MERGE/38_handoff_kb_refresh/handoff-tracked-files.md` |
| 7659 | `AI_MERGE/36_handoff_full_adoption/handoff-tracked-files.md` |
| 7088 | `AI_MERGE/34_grit_full_integration/plan-workspace/AI_MERGE/03_env_and_secret_contracts.md` |
| 7088 | `AI_MERGE/35_e2e_test_suite/plan-workspace/AI_MERGE/03_env_and_secret_contracts.md` |
| 5761 | `AI_MERGE/11_integration_research_audit_roadmap.md` |
| 4484 | `docs/rusty-idd/codex-environment.md` |
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 3140 | `AI_MERGE/34_handoff_single_repo_architecture.md` |
| 3081 | `crates/cli/src/commands/codex.rs` |
| 2595 | `crates/core/src/templates.rs` |
| 2540 | `AI_MERGE/16_upstream_knowledge_revisit.md` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |

## Findings

- packed context is 678973 tokens, above default budget 120000
