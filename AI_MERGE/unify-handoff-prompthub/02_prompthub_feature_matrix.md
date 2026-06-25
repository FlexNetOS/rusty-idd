# Feature Matrix

This matrix is generated from structural signals. Treat it as a starting point, then refine it with explicit product intent.

| Capability | Repo A Signal | Repo B Signal | Default Decision | Migration Action |
|---|---|---|---|---|
| Rust native core | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Node/TypeScript UI or tooling | yes | no | Keep Repo A implementation unless tests fail | Wrap behind stable interface |
| Python tooling | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| GitHub Actions CI | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Environment contract | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Secret references | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Nix, mise, or direnv toolchain | no | no | Create only if required by product intent | No action yet |
| Agent control files | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |
| Security policy files | yes | yes | Compare and select canonical implementation | Write parity tests, then deduplicate |

## Shared Paths

| Path | Repo A | Repo B | Risk |
|---|---|---|---|
| `.githooks/pre-commit` | yes | yes | naming/API collision |
| `.github/CODEOWNERS` | yes | yes | naming/API collision |
| `.github/dependabot.yml` | yes | yes | naming/API collision |
| `.github/pull_request_template.md` | yes | yes | naming/API collision |
| `.github/workflows/ci.yml` | yes | yes | naming/API collision |
| `.gitignore` | yes | yes | naming/API collision |
| `.handoff/README.md` | yes | yes | naming/API collision |
| `.handoff/context/capsule.json` | yes | yes | naming/API collision |
| `.handoff/packets/.gitkeep` | yes | yes | naming/API collision |
| `.handoff/tasks/.gitkeep` | yes | yes | naming/API collision |
| `AGENTS.md` | yes | yes | naming/API collision |
| `CLAUDE.md` | yes | yes | naming/API collision |
| `CONTRIBUTING.md` | yes | yes | naming/API collision |
| `Cargo.lock` | yes | yes | naming/API collision |
| `Cargo.toml` | yes | yes | naming/API collision |
| `GEMINI.md` | yes | yes | naming/API collision |
| `README.md` | yes | yes | naming/API collision |
| `SECURITY.md` | yes | yes | naming/API collision |
