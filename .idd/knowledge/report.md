# Knowledge Report

- Workspace fingerprint: `fnv1a64:65c6c3315a0df997`
- Indexed source files: 139
- Graph nodes: 8649
- Graph edges: 35408
- Resolved call edges: 20150
- Functions with complexity: 2160
- Packed files: 155
- Packed tokens: 320795
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (3755) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 3296 | `contains` (5904) | `crates/spec/src/model/spec.rs` | 331 graph links, 328 call links |
| 2871 | `walk` (3607) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2601 | `new` (6189) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2375 | `walk` (3680) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3525) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (3731) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3549) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (3822) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2025 | `implementation_loop` (5467) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 2025 | `visit_dir` (3008) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 15 unresolved calls, cyclomatic complexity 4 |
| 1707 | `handle_config_input` (6204) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1698 | `process_path_event` (3010) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 12 unresolved calls, cyclomatic complexity 35 |
| 1649 | `integration_readiness_report` (4638) | `crates/knowledge/src/lib.rs` | 336 lines, 123 graph links, 117 call links, 7 unresolved calls, cyclomatic complexity 26 |
| 1488 | `integration_owner_surfaces_join_work_item_to_system_repos` (4852) | `crates/knowledge/src/lib.rs` | 251 lines, 121 graph links, 120 call links, 10 unresolved calls |
| 1456 | `walk` (3582) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (6906) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (5426) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (1692) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1359 | `try_run` (199) | `crates/cli/src/commands/knowledge.rs` | 262 lines, 100 graph links, 97 call links, 14 unresolved calls, cyclomatic complexity 19 |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 95914 | `AI_MERGE/34_grit_full_integration/plan-workspace/AI_MERGE/00_repo_a_inventory.md` |
| 95446 | `AI_MERGE/34_grit_full_integration/01_rusty_idd_inventory_before_adoption.md` |
| 9464 | `crates/tui/src/app.rs` |
| 8632 | `crates/knowledge/src/lib.rs` |
| 7088 | `AI_MERGE/34_grit_full_integration/plan-workspace/AI_MERGE/03_env_and_secret_contracts.md` |
| 5761 | `AI_MERGE/11_integration_research_audit_roadmap.md` |
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 3140 | `AI_MERGE/34_handoff_single_repo_architecture.md` |
| 3096 | `docs/rusty-idd/codex-environment.md` |
| 2540 | `AI_MERGE/16_upstream_knowledge_revisit.md` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |
| 2421 | `AI_MERGE/14_upstream_full_adoption.md` |
| 2319 | `Justfile` |
| 2301 | `crates/core/src/templates.rs` |
| 2098 | `AI_MERGE/34_grit_full_integration/00_grit_inventory.md` |
| 2098 | `AI_MERGE/34_grit_full_integration/plan-workspace/AI_MERGE/01_repo_b_inventory.md` |
| 2097 | `crates/tui/src/ui.rs` |
| 1994 | `Makefile` |
| 1963 | `AI_MERGE/15_system_handoff_research.md` |

## Findings

- packed context is 320795 tokens, above default budget 120000
