# Knowledge Report

- Workspace fingerprint: `fnv1a64:c82e502e245c8c3e`
- Indexed Rust files: 134
- Graph nodes: 7817
- Graph edges: 30378
- Resolved call edges: 17039
- Functions with complexity: 1973
- Packed files: 104
- Packed tokens: 76839
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 3544 | `walk` (3475) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 7 unresolved calls, cyclomatic complexity 31 |
| 2871 | `walk` (3327) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 3 unresolved calls, cyclomatic complexity 41 |
| 2601 | `new` (5394) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 1 unresolved calls |
| 2476 | `contains` (5109) | `crates/spec/src/model/spec.rs` | 249 graph links, 246 call links |
| 2375 | `walk` (3400) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 3 unresolved calls, cyclomatic complexity 32 |
| 2220 | `walk` (3245) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 5 unresolved calls, cyclomatic complexity 29 |
| 2193 | `walk` (3451) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 33 |
| 2186 | `walk` (3269) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 3 unresolved calls, cyclomatic complexity 28 |
| 2141 | `walk` (3542) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 4 unresolved calls, cyclomatic complexity 30 |
| 2028 | `visit_dir` (2728) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 16 unresolved calls, cyclomatic complexity 4 |
| 2025 | `implementation_loop` (4672) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 2 unresolved calls, cyclomatic complexity 52 |
| 1707 | `handle_config_input` (5409) | `crates/tui/src/app.rs` | 133 lines, 151 graph links, 148 call links, 3 unresolved calls, cyclomatic complexity 19 |
| 1701 | `process_path_event` (2730) | `crates/external/codegraph-core/src/watch/mod.rs` | 194 lines, 137 graph links, 131 call links, 13 unresolved calls, cyclomatic complexity 35 |
| 1456 | `walk` (3302) | `crates/external/codegraph-parser/src/languages/go.rs` | 193 lines, 118 graph links, 115 call links, 4 unresolved calls, cyclomatic complexity 23 |
| 1429 | `draw_config_screen` (6111) | `crates/tui/src/ui.rs` | 214 lines, 112 graph links, 108 call links, 13 unresolved calls, cyclomatic complexity 21 |
| 1408 | `new` (4631) | `crates/runner/src/runner.rs` | 145 graph links, 137 call links |
| 1383 | `apply_env_overrides` (1412) | `crates/external/codegraph-core/src/config_manager.rs` | 222 lines, 91 graph links, 88 call links, 4 unresolved calls, cyclomatic complexity 65 |
| 1272 | `extract_symbols` (2752) | `crates/external/codegraph-core/src/watch/mod.rs` | 134 lines, 103 graph links, 97 call links, 6 unresolved calls, cyclomatic complexity 33 |
| 1179 | `parse_content_with_recovery` (3625) | `crates/external/codegraph-parser/src/parser.rs` | 194 lines, 92 graph links, 85 call links, 14 unresolved calls, cyclomatic complexity 18 |
| 1075 | `generate_workspace` (802) | `crates/core/src/planner.rs` | 131 lines, 100 graph links, 91 call links, 1 unresolved calls |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 9464 | `crates/tui/src/app.rs` |
| 7741 | `docs/rusty-idd/architecture-diagrams.md` |
| 5626 | `AI_MERGE/11_integration_research_audit_roadmap.md` |
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 2834 | `crates/knowledge/src/lib.rs` |
| 2820 | `crates/core/src/templates.rs` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |
| 2243 | `docs/rusty-idd/codex-environment.md` |
| 2097 | `crates/tui/src/ui.rs` |
| 1712 | `crates/runner/src/data.rs` |
| 1603 | `crates/runner/src/runner.rs` |
| 1520 | `AI_MERGE/13_codex_environment.md` |
| 1452 | `docs/rusty-idd/design.md` |
| 1387 | `AI_MERGE/12_knowledge_deep_audit.md` |
| 1364 | `crates/runner/src/config.rs` |
| 1356 | `docs/rusty-idd/slice-sequence.md` |
| 1294 | `crates/cli/src/commands/codex.rs` |
| 1150 | `crates/core/src/model.rs` |
| 1095 | `docs/rusty-idd/dependency-duplication.md` |

## Findings

No knowledge findings.
