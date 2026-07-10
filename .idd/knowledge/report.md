# Knowledge Report

- Workspace fingerprint: `fnv1a64:a3160ec9e969f63a`
- Indexed source files: 457
- Graph nodes: 30876
- Graph edges: 112917
- Resolved call edges: 62244
- Functions with complexity: 7946
- Packed files: 135
- Packed tokens: 101751
- Suspicious files: 0

## Hotspots

| Score | Node | File | Reasons |
|---:|---|---|---|
| 33054 | `to_string` (26506) | `vendor/syntect/src/parsing/scope.rs` | 3304 graph links, 3301 call links, 1 unresolved calls, cyclomatic complexity 4 |
| 6406 | `collect` (17619) | `imports/prompt_hub/prompt-hub/src/garbage_collector.rs` | 43 lines, 637 graph links, 633 call links, 3 unresolved calls |
| 4978 | `main` (9893) | `hf/src/main.rs` | 467 lines, 433 graph links, 432 call links, 32 unresolved calls, cyclomatic complexity 23 |
| 4242 | `main` (14852) | `imports/handoff/hf/src/main.rs` | 358 lines, 372 graph links, 371 call links, 29 unresolved calls, cyclomatic complexity 21 |
| 4057 | `build_hf_args` (9018) | `hf/src/bin/hf-mcp.rs` | 352 lines, 357 graph links, 348 call links, 2 unresolved calls, cyclomatic complexity 48 |
| 3553 | `walk` (4316) | `crates/external/codegraph-parser/src/languages/rust.rs` | 400 lines, 302 graph links, 299 call links, 10 unresolved calls, cyclomatic complexity 31 |
| 2958 | `create_router` (24610) | `imports/prompt_hub/prompthub-server/src/server.rs` | 406 lines, 254 graph links, 250 call links, 12 unresolved calls |
| 2893 | `build_hf_args` (13580) | `imports/handoff/hf/src/bin/hf-mcp.rs` | 248 lines, 257 graph links, 248 call links, 2 unresolved calls, cyclomatic complexity 33 |
| 2877 | `walk` (4168) | `crates/external/codegraph-parser/src/languages/java.rs` | 349 lines, 237 graph links, 234 call links, 5 unresolved calls, cyclomatic complexity 41 |
| 2610 | `new` (6770) | `crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 4 unresolved calls |
| 2610 | `new` (12553) | `imports/handoff/crates/tui/src/app.rs` | 35 lines, 259 graph links, 254 call links, 4 unresolved calls |
| 2381 | `walk` (4241) | `crates/external/codegraph-parser/src/languages/php.rs` | 299 lines, 196 graph links, 193 call links, 5 unresolved calls, cyclomatic complexity 32 |
| 2267 | `into_response` (23834) | `imports/prompt_hub/prompthub-server/src/responses.rs` | 227 graph links, 225 call links, 1 unresolved calls |
| 2226 | `walk` (4086) | `crates/external/codegraph-parser/src/languages/cpp.rs` | 290 lines, 182 graph links, 179 call links, 7 unresolved calls, cyclomatic complexity 29 |
| 2199 | `walk` (4292) | `crates/external/codegraph-parser/src/languages/ruby.rs` | 273 lines, 180 graph links, 177 call links, 5 unresolved calls, cyclomatic complexity 33 |
| 2192 | `walk` (4110) | `crates/external/codegraph-parser/src/languages/csharp.rs` | 286 lines, 180 graph links, 177 call links, 5 unresolved calls, cyclomatic complexity 28 |
| 2147 | `walk` (4383) | `crates/external/codegraph-parser/src/languages/swift.rs` | 280 lines, 175 graph links, 172 call links, 6 unresolved calls, cyclomatic complexity 30 |
| 2055 | `implementation_loop` (6044) | `crates/runner/src/runner.rs` | 342 lines, 156 graph links, 143 call links, 12 unresolved calls, cyclomatic complexity 52 |
| 2051 | `find` (23592) | `imports/prompt_hub/prompthub/src/fuzzy.rs` | 204 graph links, 199 call links, 3 unresolved calls, cyclomatic complexity 5 |
| 2045 | `implementation_loop` (11830) | `imports/handoff/crates/runner/src/runner.rs` | 342 lines, 155 graph links, 142 call links, 12 unresolved calls, cyclomatic complexity 52 |

## Top Files By Tokens

| Tokens | File |
|---:|---|
| 9464 | `crates/tui/src/app.rs` |
| 8628 | `crates/knowledge/src/lib.rs` |
| 4484 | `docs/rusty-idd/codex-environment.md` |
| 4132 | `docs/rusty-idd/spec-engine-design.md` |
| 3666 | `docs/rusty-idd/lifecycle-contract.md` |
| 3198 | `docs/rusty-idd/dot-directory-architecture.md` |
| 3081 | `crates/cli/src/commands/codex.rs` |
| 2688 | `AGENTS.md` |
| 2672 | `docs/rusty-idd/security-advisories.md` |
| 2595 | `crates/core/src/templates.rs` |
| 2527 | `Makefile` |
| 2502 | `docs/rusty-idd/production-readiness-audit.md` |
| 2428 | `Justfile` |
| 2097 | `crates/tui/src/ui.rs` |
| 1779 | `docs/rusty-idd/architecture-diagrams.md` |
| 1750 | `crates/runner/src/data.rs` |
| 1635 | `crates/runner/src/runner.rs` |
| 1488 | `docs/rusty-idd/design.md` |
| 1419 | `crates/runner/src/config.rs` |
| 1360 | `crates/work-order/src/lib.rs` |

## Findings

No knowledge findings.
