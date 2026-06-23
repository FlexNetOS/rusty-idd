# Knowledge Report

- Workspace fingerprint: `fnv1a64:82e1b99cfd6c7238`
- Indexed source files: 147
- Graph nodes: 9069
- Graph edges: 37181
- Resolved call edges: 21184
- Functions with complexity: 2279
- Packed files: 218
- Packed tokens: 683044
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (4169) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 3406 | `contains` (6318) | `crates/spec/src/model/spec.rs` | 342 graph links, 339 call links |
| 2871 | `walk` (4021) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2601 | `new` (6603) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2375 | `walk` (4094) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3939) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (4145) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3963) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (4236) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2025 | `implementation_loop` (5881) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 2022 | `visit_dir` (3422) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 14 unresolved calls, cyclomatic complexity 4 |
| 1707 | `handle_config_input` (6618) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1695 | `process_path_event` (3424) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 11 unresolved calls, cyclomatic complexity 35 |
| 1649 | `integration_readiness_report` (5052) | `crates/knowledge/src/lib.rs` | 336 lines, 123 graph links, 117 call links, 7 unresolved calls, cyclomatic complexity 26 |
| 1488 | `integration_owner_surfaces_join_work_item_to_system_repos` (5266) | `crates/knowledge/src/lib.rs` | 251 lines, 121 graph links, 120 call links, 10 unresolved calls |
| 1456 | `walk` (3996) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (7320) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (5840) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (2106) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1356 | `try_run` (300) | `crates/cli/src/commands/knowledge.rs` | 262 lines, 100 graph links, 97 call links, 13 unresolved calls, cyclomatic complexity 19 |

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

- packed context is 683044 tokens, above default budget 120000
