# Repository Inventory: grit

- Root: `/home/drdave/Desktop/meta/grit`
- Files scanned: `100`

## Category Counts

| Category | Count |
|---|---:|
| source | 22 |
| config | 4 |
| workflow | 5 |
| documentation | 17 |
| test | 38 |
| build | 6 |
| lockfile | 1 |
| agent-control | 1 |
| unknown | 6 |

## Languages

| Language | Files |
|---|---:|
| Python | 6 |
| Rust | 21 |
| Shell | 16 |
| TypeScript | 14 |

## Package Managers / Toolchains

- `cargo`

## Entrypoints

- `src/main.rs`

## Workflows

- `.github/workflows/ci.yml`
- `.github/workflows/next-release.yml`
- `.github/workflows/pr-target-check.yml`
- `.github/workflows/release-please.yml`
- `.github/workflows/release.yml`

## Agent Control Files

- `AGENTS.md`

## Security Files

- _none detected_

## Environment Keys Found

- `GITHUB_TOKEN`
- `HOMEBREW_TAP_TOKEN`

## Secret / Env References Found

| File | Key | Source |
|---|---|---|
| `.github/workflows/next-release.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/release-please.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/release.yml` | `GITHUB_TOKEN` | github-actions-secret |
| `.github/workflows/release.yml` | `HOMEBREW_TAP_TOKEN` | github-actions-secret |
| `docs/RELEASE_FLOW.md` | `GITHUB_TOKEN` | github-actions-secret |

## File Index

| Path | Category | Size |
|---|---|---:|
| `.github/workflows/ci.yml` | workflow | 1143 |
| `.github/workflows/next-release.yml` | workflow | 4138 |
| `.github/workflows/pr-target-check.yml` | workflow | 1568 |
| `.github/workflows/release-please.yml` | workflow | 3203 |
| `.github/workflows/release.yml` | workflow | 7546 |
| `.gitignore` | unknown | 477 |
| `.release-please-manifest.json` | config | 19 |
| `.rtk/filters.toml` | config | 477 |
| `AGENTS.md` | agent-control | 906 |
| `CHANGELOG.md` | documentation | 3571 |
| `CLAUDE.md` | documentation | 5664 |
| `Cargo.lock` | lockfile | 92635 |
| `Cargo.toml` | build | 1483 |
| `LICENSE` | unknown | 10763 |
| `README.md` | documentation | 14188 |
| `assets/banner.png` | unknown | 32456 |
| `assets/bench_data.json` | config | 4465 |
| `assets/benchmark.pdf` | unknown | 48604 |
| `assets/benchmark.png` | unknown | 205844 |
| `docs/README.ar.md` | documentation | 8410 |
| `docs/README.de.md` | documentation | 7590 |
| `docs/README.es.md` | documentation | 7583 |
| `docs/README.fr.md` | documentation | 7637 |
| `docs/README.hi.md` | documentation | 10169 |
| `docs/README.it.md` | documentation | 7518 |
| `docs/README.ja.md` | documentation | 8308 |
| `docs/README.ko.md` | documentation | 7771 |
| `docs/README.nl.md` | documentation | 7677 |
| `docs/README.pt.md` | documentation | 7517 |
| `docs/README.ru.md` | documentation | 9297 |
| `docs/README.zh.md` | documentation | 7312 |
| `docs/RELEASE_FLOW.md` | documentation | 3264 |
| `examples/01-basic-workflow.sh` | source | 2237 |
| `examples/02-parallel-agents.sh` | source | 2455 |
| `examples/03-session-pr.sh` | source | 1845 |
| `examples/04-s3-backend.sh` | source | 1871 |
| `examples/05-claude-code-integration.sh` | source | 2151 |
| `examples/06-monitoring.sh` | source | 1971 |
| `release-please-config.json` | config | 182 |
| `scripts/.gitignore` | unknown | 11 |
| `scripts/README.md` | documentation | 1877 |
| `scripts/ai-agents/bench.sh` | source | 9055 |
| `scripts/lib/common.sh` | source | 3228 |
| `scripts/sweep/bench.sh` | source | 6700 |
| `scripts/synthetic/bench.sh` | source | 8282 |
| `scripts/throughput/bench.sh` | source | 15218 |
| `src/cli/mod.rs` | source | 52762 |
| `src/config.rs` | source | 3915 |
| `src/db/azure_store.rs` | source | 17205 |
| `src/db/lock_store.rs` | source | 1260 |
| `src/db/mod.rs` | source | 29533 |
| `src/db/s3_store.rs` | source | 24367 |
| `src/db/sqlite_store.rs` | source | 21370 |
| `src/git/mod.rs` | source | 23935 |
| `src/main.rs` | source | 188 |
| `src/parser/mod.rs` | source | 50622 |
| `src/room/mod.rs` | source | 5002 |
| `test-projects/pi-calc/backend/Cargo.toml` | build | 382 |
| `test-projects/pi-calc/backend/src/algorithms.rs` | test | 3985 |
| `test-projects/pi-calc/backend/src/handlers.rs` | test | 7433 |
| `test-projects/pi-calc/backend/src/main.rs` | test | 1408 |
| `test-projects/pi-calc/backend/src/precision.rs` | test | 2077 |
| `test-projects/pi-calc/backend/src/state.rs` | test | 2019 |
| `test-projects/pi-calc/frontend/index.html` | test | 433 |
| `test-projects/pi-calc/frontend/package.json` | build | 386 |
| `test-projects/pi-calc/frontend/src/App.tsx` | test | 2203 |
| `test-projects/pi-calc/frontend/src/api/client.ts` | test | 3649 |
| `test-projects/pi-calc/frontend/src/components/AlgorithmPicker.tsx` | test | 3794 |
| `test-projects/pi-calc/frontend/src/components/Comparison.tsx` | test | 4929 |
| `test-projects/pi-calc/frontend/src/components/Convergence.tsx` | test | 5119 |
| `test-projects/pi-calc/frontend/src/components/PiDisplay.tsx` | test | 4233 |
| `test-projects/pi-calc/frontend/src/index.tsx` | test | 324 |
| `test-projects/pi-calc/frontend/src/utils/format.ts` | test | 2014 |
| `test-projects/pi-calc/frontend/src/utils/math.ts` | test | 3439 |
| `test-projects/pi-calc/frontend/tsconfig.json` | test | 485 |
| `test-projects/py-ml/pyproject.toml` | build | 448 |
| `test-projects/py-ml/src/__init__.py` | test | 0 |
| `test-projects/py-ml/src/features.py` | test | 9660 |
| `test-projects/py-ml/src/model.py` | test | 8247 |
| `test-projects/py-ml/src/pipeline.py` | test | 8389 |
| `test-projects/py-ml/src/utils.py` | test | 5777 |
| `test-projects/rust-service/Cargo.toml` | build | 559 |
| `test-projects/rust-service/src/auth.rs` | test | 4640 |
| `test-projects/rust-service/src/config.rs` | test | 5816 |
| `test-projects/rust-service/src/handler.rs` | test | 7156 |
| `test-projects/rust-service/src/main.rs` | test | 1943 |
| `test-projects/rust-service/src/storage.rs` | test | 4418 |
| `test-projects/ts-api/package.json` | build | 549 |
| `test-projects/ts-api/src/api.ts` | test | 6039 |
| `test-projects/ts-api/src/auth.ts` | test | 4992 |
| `test-projects/ts-api/src/db.ts` | test | 4183 |
| `test-projects/ts-api/src/middleware.ts` | test | 5976 |
| `test-projects/ts-api/src/utils.ts` | test | 2953 |
| `test-projects/ts-api/tsconfig.json` | test | 445 |
| `tests/bench_parallel.sh` | test | 6025 |
| `tests/bench_sweep.sh` | test | 6729 |
| `tests/benchmark.sh` | test | 12070 |
| `tests/claude_agents_test.sh` | test | 7055 |
| `tests/gen_graph.py` | test | 18247 |
| `tests/harness.sh` | test | 6248 |
