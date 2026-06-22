# Knowledge Report

- Workspace fingerprint: `fnv1a64:d40b2715e256a746`
- Indexed source files: 139
- Graph nodes: 8758
- Graph edges: 36130
- Resolved call edges: 20635
- Functions with complexity: 2206
- Packed files: 199
- Packed tokens: 672886
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (3860) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 3346 | `contains` (6009) | `crates/spec/src/model/spec.rs` | 336 graph links, 333 call links |
| 2871 | `walk` (3712) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2601 | `new` (6294) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2375 | `walk` (3785) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3630) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (3836) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3654) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (3927) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2025 | `implementation_loop` (5572) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 2022 | `visit_dir` (3113) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 14 unresolved calls, cyclomatic complexity 4 |
| 1707 | `handle_config_input` (6309) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1695 | `process_path_event` (3115) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 11 unresolved calls, cyclomatic complexity 35 |
| 1649 | `integration_readiness_report` (4743) | `crates/knowledge/src/lib.rs` | 336 lines, 123 graph links, 117 call links, 7 unresolved calls, cyclomatic complexity 26 |
| 1488 | `integration_owner_surfaces_join_work_item_to_system_repos` (4957) | `crates/knowledge/src/lib.rs` | 251 lines, 121 graph links, 120 call links, 10 unresolved calls |
| 1456 | `walk` (3687) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (7011) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (5531) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (1797) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1356 | `try_run` (257) | `crates/cli/src/commands/knowledge.rs` | 262 lines, 100 graph links, 97 call links, 13 unresolved calls, cyclomatic complexity 19 |

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
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3715 | `docs/rusty-idd/codex-environment.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 3140 | `AI_MERGE/34_handoff_single_repo_architecture.md` |
| 3081 | `crates/cli/src/commands/codex.rs` |
| 2540 | `AI_MERGE/16_upstream_knowledge_revisit.md` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |
| 2469 | `AI_MERGE/35_e2e_test_suite/plan-workspace/AI_MERGE/02_feature_matrix.md` |

## Findings

- packed context is 672886 tokens, above default budget 120000
