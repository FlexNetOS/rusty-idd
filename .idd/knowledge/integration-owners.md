# Integration Owner Surfaces

- Workspace root: `/home/drdave/Desktop/meta/rusty-idd`
- Source plan: `.idd/knowledge/integration-plan.json`
- Source system architecture: `.idd/knowledge/system-architecture.json`
- Change: `integrate-env-vault-relay`
- Capability: `capability:env-vault-relay`
- Owner repos: 3

## Owners

| Owner | Found | Repo | Branch | Dirty | Roles | Markers | Architecture |
|---|---|---|---|---:|---|---|---|
| `repo:envctl` | true | `envctl` | `feat/bun-toolchain-path-and-tool-registrations` | false | role:fleet-handoff, role:rust-code-surface, role:toolchain-provider | rust, handoff, claude, github-actions | no |
| `repo:vault-hub` | true | `vault_hub` | `main` | false | role:capability-hub |  | no |
| `repo:yazelix` | true | `yazelix` | `main` | false | role:parser-runtime-surface, role:toolchain-provider | claude, github-actions | no |

## Evidence Paths

- `repo:envctl`:
  - `envctl`
  - `envctl/.claude`
  - `envctl/.github/workflows`
  - `envctl/.handoff`
  - `envctl/Cargo.toml`
- `repo:vault-hub`:
  - `vault_hub`
- `repo:yazelix`:
  - `yazelix`
  - `yazelix/.claude`
  - `yazelix/.github/workflows`

## Native Diagnostics

- `cd envctl && cargo metadata --locked --format-version 1`
- `cd envctl && cargo test --workspace --all-features --locked`
- `git -C envctl rev-parse HEAD`
- `git -C envctl status --short --branch`
- `git -C vault_hub rev-parse HEAD`
- `git -C vault_hub status --short --branch`
- `git -C yazelix rev-parse HEAD`
- `git -C yazelix status --short --branch`

## Findings

- 0 resolved owner repos report dirty state
- all owner repos resolved in the system architecture graph
- joined 3 owner repos against .idd/knowledge/system-architecture.json
- selected integrate-env-vault-relay from .idd/knowledge/integration-plan.json
