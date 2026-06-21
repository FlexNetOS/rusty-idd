# Knowledge Report

- Workspace fingerprint: `fnv1a64:dd998e34d63688ab`
- Indexed source files: 135
- Graph nodes: 8287
- Graph edges: 33408
- Resolved call edges: 18923
- Functions with complexity: 2079
- Packed files: 116
- Packed tokens: 98762
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (3588) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 2871 | `walk` (3440) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2866 | `contains` (5555) | `crates/spec/src/model/spec.rs` | 288 graph links, 285 call links |
| 2601 | `new` (5840) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2375 | `walk` (3513) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3358) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (3564) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3382) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (3655) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2028 | `visit_dir` (2841) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 16 unresolved calls, cyclomatic complexity 4 |
| 2025 | `implementation_loop` (5118) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 1707 | `handle_config_input` (5855) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1701 | `process_path_event` (2843) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 13 unresolved calls, cyclomatic complexity 35 |
| 1456 | `walk` (3415) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (6557) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (5077) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (1525) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1272 | `extract_symbols` (2865) | `crates/external/codegraph-core/src/watch/mod.rs` | 134 lines, 103 graph links, 97 call links, 6 unresolved calls, cyclomatic complexity 33 |
| 1179 | `parse_content_with_recovery` (3738) | `crates/external/codegraph-parser/src/parser.rs` | 194 lines, 92 graph links, 85 call links, 14 unresolved calls, cyclomatic complexity 18 |
| 1075 | `generate_workspace` (911) | `crates/core/src/planner.rs` | 131 lines, 100 graph links, 91 call links, 1 unresolved calls |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 9464 | `crates/tui/src/app.rs` |
| 7741 | `docs/rusty-idd/architecture-diagrams.md` |
| 7026 | `crates/knowledge/src/lib.rs` |
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
| 1712 | `crates/runner/src/data.rs` |
| 1603 | `crates/runner/src/runner.rs` |
| 1573 | `AI_MERGE/12_knowledge_deep_audit.md` |
| 1520 | `AI_MERGE/13_codex_environment.md` |
| 1469 | `Justfile` |
| 1452 | `docs/rusty-idd/design.md` |
| 1366 | `adr/0004-knowledge-direct-crate-integration.md` |

## Findings

No knowledge findings.
