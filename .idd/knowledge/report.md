# Knowledge Report

- Workspace fingerprint: `fnv1a64:4963485cf98e16f9`
- Indexed source files: 135
- Graph nodes: 8440
- Graph edges: 34370
- Resolved call edges: 19487
- Functions with complexity: 2103
- Packed files: 122
- Packed tokens: 107682
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (3635) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 3056 | `contains` (5700) | `crates/spec/src/model/spec.rs` | 307 graph links, 304 call links |
| 2871 | `walk` (3487) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2601 | `new` (5985) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2375 | `walk` (3560) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3405) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (3611) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3429) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (3702) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2028 | `visit_dir` (2888) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 16 unresolved calls, cyclomatic complexity 4 |
| 2025 | `implementation_loop` (5263) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 1707 | `handle_config_input` (6000) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1701 | `process_path_event` (2890) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 13 unresolved calls, cyclomatic complexity 35 |
| 1456 | `walk` (3462) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (6702) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (5222) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (1572) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1342 | `integration_owner_surfaces_join_work_item_to_system_repos` (4700) | `crates/knowledge/src/lib.rs` | 195 lines, 112 graph links, 111 call links, 10 unresolved calls |
| 1337 | `integration_readiness_report` (4494) | `crates/knowledge/src/lib.rs` | 267 lines, 101 graph links, 95 call links, 6 unresolved calls, cyclomatic complexity 21 |
| 1329 | `try_run` (159) | `crates/cli/src/commands/knowledge.rs` | 256 lines, 97 graph links, 94 call links, 16 unresolved calls, cyclomatic complexity 19 |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 9464 | `crates/tui/src/app.rs` |
| 8390 | `crates/knowledge/src/lib.rs` |
| 7741 | `docs/rusty-idd/architecture-diagrams.md` |
| 5761 | `AI_MERGE/11_integration_research_audit_roadmap.md` |
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 2820 | `crates/core/src/templates.rs` |
| 2540 | `AI_MERGE/16_upstream_knowledge_revisit.md` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |
| 2421 | `AI_MERGE/14_upstream_full_adoption.md` |
| 2304 | `docs/rusty-idd/codex-environment.md` |
| 2097 | `crates/tui/src/ui.rs` |
| 1963 | `AI_MERGE/15_system_handoff_research.md` |
| 1905 | `Justfile` |
| 1712 | `crates/runner/src/data.rs` |
| 1633 | `Makefile` |
| 1603 | `crates/runner/src/runner.rs` |
| 1573 | `AI_MERGE/12_knowledge_deep_audit.md` |
| 1520 | `AI_MERGE/13_codex_environment.md` |
| 1452 | `docs/rusty-idd/design.md` |

## Findings

No knowledge findings.
