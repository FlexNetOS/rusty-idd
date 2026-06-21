# Knowledge Report

- Workspace fingerprint: `fnv1a64:3a0e9b5a0e9b348b`
- Indexed source files: 138
- Graph nodes: 8532
- Graph edges: 34813
- Resolved call edges: 19755
- Functions with complexity: 2127
- Packed files: 124
- Packed tokens: 109384
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (3665) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 3116 | `contains` (5791) | `crates/spec/src/model/spec.rs` | 313 graph links, 310 call links |
| 2871 | `walk` (3517) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2601 | `new` (6076) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2375 | `walk` (3590) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3435) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (3641) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3459) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (3732) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2028 | `visit_dir` (2918) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 16 unresolved calls, cyclomatic complexity 4 |
| 2025 | `implementation_loop` (5354) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 1707 | `handle_config_input` (6091) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1701 | `process_path_event` (2920) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 13 unresolved calls, cyclomatic complexity 35 |
| 1649 | `integration_readiness_report` (4525) | `crates/knowledge/src/lib.rs` | 336 lines, 123 graph links, 117 call links, 7 unresolved calls, cyclomatic complexity 26 |
| 1488 | `integration_owner_surfaces_join_work_item_to_system_repos` (4739) | `crates/knowledge/src/lib.rs` | 251 lines, 121 graph links, 120 call links, 10 unresolved calls |
| 1456 | `walk` (3492) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (6793) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (5313) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (1602) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1329 | `try_run` (159) | `crates/cli/src/commands/knowledge.rs` | 256 lines, 97 graph links, 94 call links, 16 unresolved calls, cyclomatic complexity 19 |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 9464 | `crates/tui/src/app.rs` |
| 8507 | `crates/knowledge/src/lib.rs` |
| 7741 | `docs/rusty-idd/architecture-diagrams.md` |
| 5761 | `AI_MERGE/11_integration_research_audit_roadmap.md` |
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 2766 | `docs/rusty-idd/codex-environment.md` |
| 2540 | `AI_MERGE/16_upstream_knowledge_revisit.md` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |
| 2421 | `AI_MERGE/14_upstream_full_adoption.md` |
| 2301 | `crates/core/src/templates.rs` |
| 2097 | `crates/tui/src/ui.rs` |
| 1994 | `Makefile` |
| 1963 | `AI_MERGE/15_system_handoff_research.md` |
| 1941 | `Justfile` |
| 1712 | `crates/runner/src/data.rs` |
| 1679 | `AI_MERGE/31_prompt_front_door_upstream_adoption.md` |
| 1603 | `crates/runner/src/runner.rs` |
| 1594 | `AI_MERGE/13_codex_environment.md` |
| 1573 | `AI_MERGE/12_knowledge_deep_audit.md` |

## Findings

No knowledge findings.
