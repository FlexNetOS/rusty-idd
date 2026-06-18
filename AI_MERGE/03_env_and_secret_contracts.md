# 03. Environment and Secret Contracts

This document maps expected environment variables and secrets for the
`rusty-idd` workspace. Core runtime functionality must remain usable without
external secrets.

## Runtime Contract

| Name | Required | Secret | Owner | Purpose |
|---|---:|---:|---|---|
| `RUST_LOG` | no | no | local operator | Enables Rust logging for debugging when supported by downstream crates. |
| `NO_COLOR` | no | no | local operator | Requests plain terminal output from compatible tools. |

No runtime API token, database URL, model key, or cloud credential is required
for `rusty-idd scan`, `plan`, `task`, `validate`, `manifest`, `spec`, `run`, or
`tui`.

## CI And Release Contract

| Name | Required | Secret | Source | Purpose |
|---|---:|---:|---|---|
| `GITHUB_TOKEN` | yes | yes | GitHub Actions automatic token | Runs semantic PR title checks, release-please fallback, and release asset upload. |
| `PARENT_REPO_PAT` | no | yes | GitHub Actions repository/org secret | Optional release-please token. When absent, the release-please job is skipped because this repo does not permit `GITHUB_TOKEN` to create pull requests. |
| `ENABLE_RELEASE_PLEASE` | no | no | GitHub Actions repository/org variable | Set to `true` only after `PARENT_REPO_PAT` is configured and allowed to create release PRs. |

`PARENT_REPO_PAT` is intentionally optional. If it is absent, release-please
does not run; release PR creation must be performed manually or by adding a
least-privilege repository/org secret and setting `ENABLE_RELEASE_PLEASE=true`.

## Provider Rules

- Do not add a new secret provider without updating this file and
  `.env.contract.yaml`.
- Do not commit real secret values.
- Use `.env.schema.example.json` for local non-secret examples.
- Use GitHub Actions secrets or OIDC for CI credentials.
- Keep release credentials repository-scoped and least-privilege.
