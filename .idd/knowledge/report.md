# Knowledge Report

- Workspace fingerprint: `fnv1a64:895e52f8fb63db31`
- Indexed source files: 387
- Graph nodes: 25253
- Graph edges: 91112
- Resolved call edges: 50003
- Functions with complexity: 6535
- Packed files: 135
- Packed tokens: 95557
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 13684 | `clone` (14216) | `imports/prompt_hub/prompt-hub/src/chaos_auto.rs` | 1368 graph links, 1366 call links, 1 unresolved calls |
| 4876 | `collect` (14916) | `imports/prompt_hub/prompt-hub/src/garbage_collector.rs` | 43 lines, 484 graph links, 480 call links, 3 unresolved calls |
| 4185 | `main` (12149) | `imports/handoff/hf/src/main.rs` | 358 lines, 372 graph links, 371 call links, 10 unresolved calls, cyclomatic complexity 21 |
| 3547 | `walk` (4312) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 8 unresolved calls, cyclomatic complexity 31 |
| 2958 | `create_router` (21907) | `imports/prompt_hub/prompthub-server/src/server.rs` | 406 lines, 254 graph links, 250 call links, 12 unresolved calls |
| 2893 | `build_hf_args` (10877) | `imports/handoff/hf/src/bin/hf-mcp.rs` | 248 lines, 257 graph links, 248 call links, 2 unresolved calls, cyclomatic complexity 33 |
| 2874 | `walk` (4164) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 4 unresolved calls, cyclomatic complexity 41 |
| 2610 | `new` (6751) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 4 unresolved calls |
| 2610 | `new` (9850) | `imports/handoff/crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 4 unresolved calls |
| 2378 | `walk` (4237) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 4 unresolved calls, cyclomatic complexity 32 |
| 2267 | `into_response` (21131) | `imports/prompt_hub/prompthub-server/src/responses.rs` | 227 graph links, 225 call links, 1 unresolved calls |
| 2223 | `walk` (4082) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 6 unresolved calls, cyclomatic complexity 29 |
| 2196 | `walk` (4288) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 4 unresolved calls, cyclomatic complexity 33 |
| 2189 | `walk` (4106) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 4 unresolved calls, cyclomatic complexity 28 |
| 2144 | `walk` (4379) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 5 unresolved calls, cyclomatic complexity 30 |
| 2052 | `implementation_loop` (6029) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 11 unresolved calls, cyclomatic complexity 52 |
| 2042 | `implementation_loop` (9127) | `imports/handoff/crates/runner/src/runner.rs` | 342 lines, 155 graph links, 142 call links, 11 unresolved calls, cyclomatic complexity 52 |
| 2025 | `visit_dir` (3565) | `crates/external/codegraph-core/src/watch/mod.rs` | 198 graph links, 194 call links, 15 unresolved calls, cyclomatic complexity 4 |
| 1757 | `ok` (966) | `crates/cli/tests/deploy_cli.rs` | 176 graph links, 173 call links, 1 unresolved calls |
| 1703 | `handle_config_input` (6766) | `crates/tui/src/app.rs` | 133 lines, 150 graph links, 147 call links, 5 unresolved calls, cyclomatic complexity 19 |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 9464 | `crates/tui/src/app.rs` |
| 8632 | `crates/knowledge/src/lib.rs` |
| 4484 | `docs/rusty-idd/codex-environment.md` |
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 3081 | `crates/cli/src/commands/codex.rs` |
| 2595 | `crates/core/src/templates.rs` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |
| 2428 | `Justfile` |
| 2097 | `crates/tui/src/ui.rs` |
| 1994 | `Makefile` |
| 1779 | `docs/rusty-idd/architecture-diagrams.md` |
| 1712 | `crates/runner/src/data.rs` |
| 1603 | `crates/runner/src/runner.rs` |
| 1488 | `docs/rusty-idd/design.md` |
| 1364 | `crates/runner/src/config.rs` |
| 1360 | `crates/work-order/src/lib.rs` |
| 1356 | `docs/rusty-idd/slice-sequence.md` |
| 1306 | `crates/cli/src/commands/knowledge.rs` |
| 1282 | `adr/0004-handoff-outer-single-repo.md` |

## Findings

No knowledge findings.
