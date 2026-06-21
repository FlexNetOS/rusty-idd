# Integration Owner Surfaces

- Workspace root: `/home/drdave/Desktop/meta/rusty-idd`
- Source plan: `.idd/knowledge/integration-plan.json`
- Source system architecture: `.idd/knowledge/system-architecture.json`
- Change: `integrate-fleet-handoff`
- Capability: `capability:fleet-handoff`
- Owner repos: 13

## Owners

| Owner | Found | Repo | Branch | Dirty | Roles | Markers | Architecture |
|---|---|---|---|---:|---|---|---|
| `repo:agent` | true | `agent` | `main` | false | role:agent-environment, role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions | no |
| `repo:ecc` | true | `ECC` | `main` | false | role:agent-environment, role:fleet-handoff | node, handoff, agents, claude, github-actions | no |
| `repo:envctl` | true | `envctl` | `master` | true | role:agent-environment, role:fleet-handoff, role:rust-code-surface, role:toolchain-provider | rust, handoff, agents, claude, github-actions | no |
| `repo:flexnetos-runner` | true | `flexnetos_runner` | `chore/handoff-tier-a-pilot` | false | role:fleet-handoff, role:rust-code-surface | rust, handoff, github-actions | no |
| `repo:github-org` | true | `github_org` | `fix/wrap-up-base-develop` | false | role:agent-environment, role:fleet-handoff | handoff, agents, claude, github-actions, make | no |
| `repo:handoff` | true | `handoff` | `fix/windows-ledger-path-and-promote-checkout` | true | role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions, make | no |
| `repo:harness-hub` | true | `harness_hub` | `master` | false | role:capability-hub, role:fleet-handoff | handoff, github-actions | no |
| `repo:lane` | true | `lane` | `main` | false | role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions | no |
| `repo:lifeos` | true | `lifeos` | `main` | false | role:fleet-handoff, role:rust-code-surface | rust, node, openspec, handoff, claude | no |
| `repo:network-control` | true | `network-control` | `fix/handoff-remove-hand-rolled-cards` | false | role:fleet-handoff, role:rust-code-surface | rust, handoff, claude, github-actions | no |
| `repo:prompt-hub` | true | `prompt_hub` | `main` | true | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer | rust, handoff, claude, github-actions | no |
| `repo:rusty-idd` | true | `rusty-idd` | `` | false | role:agent-environment, role:fleet-handoff, role:idd-control-plane, role:rust-code-surface | rust, openspec, idd-knowledge, handoff, agents, claude, github-actions, make, just | yes |
| `repo:weave` | true | `weave` | `wl056-xmachine-push` | true | role:agent-environment, role:coordination-domain-surface, role:fleet-handoff, role:rust-code-surface | rust, handoff, agents, claude, github-actions | no |

## Evidence Paths

- `repo:agent`:
  - `agent`
  - `agent/.claude`
  - `agent/.github/workflows`
  - `agent/.handoff`
  - `agent/Cargo.toml`
- `repo:ecc`:
  - `ECC`
  - `ECC/.agents`
  - `ECC/.claude`
  - `ECC/.github/workflows`
  - `ECC/.handoff`
  - `ECC/package.json`
- `repo:envctl`:
  - `envctl`
  - `envctl/.agents`
  - `envctl/.claude`
  - `envctl/.github/workflows`
  - `envctl/.handoff`
  - `envctl/Cargo.toml`
- `repo:flexnetos-runner`:
  - `flexnetos_runner`
  - `flexnetos_runner/.github/workflows`
  - `flexnetos_runner/.handoff`
  - `flexnetos_runner/Cargo.toml`
- `repo:github-org`:
  - `.github_org`
  - `.github_org/.agents`
  - `.github_org/.claude`
  - `.github_org/.github/workflows`
  - `.github_org/.handoff`
  - `.github_org/Makefile`
- `repo:handoff`:
  - `handoff`
  - `handoff/.claude`
  - `handoff/.github/workflows`
  - `handoff/.handoff`
  - `handoff/Cargo.toml`
  - `handoff/Makefile`
- `repo:harness-hub`:
  - `harness_hub`
  - `harness_hub/.github/workflows`
  - `harness_hub/.handoff`
- `repo:lane`:
  - `lane`
  - `lane/.claude`
  - `lane/.github/workflows`
  - `lane/.handoff`
  - `lane/Cargo.toml`
- `repo:lifeos`:
  - `lifeos`
  - `lifeos/.claude`
  - `lifeos/.handoff`
  - `lifeos/Cargo.toml`
  - `lifeos/openspec`
  - `lifeos/package.json`
- `repo:network-control`:
  - `network-control`
  - `network-control/.claude`
  - `network-control/.github/workflows`
  - `network-control/.handoff`
  - `network-control/Cargo.toml`
- `repo:prompt-hub`:
  - `prompt_hub`
  - `prompt_hub/.claude`
  - `prompt_hub/.github/workflows`
  - `prompt_hub/.handoff`
  - `prompt_hub/Cargo.toml`
- `repo:rusty-idd`:
  - `rusty-idd`
  - `rusty-idd/.agents`
  - `rusty-idd/.claude`
  - `rusty-idd/.github/workflows`
  - `rusty-idd/.handoff`
  - `rusty-idd/.idd/knowledge`
  - `rusty-idd/.idd/knowledge/architecture.json`
  - `rusty-idd/Cargo.toml`
  - `rusty-idd/Justfile`
  - `rusty-idd/Makefile`
  - `rusty-idd/openspec`
- `repo:weave`:
  - `weave`
  - `weave/.agents`
  - `weave/.claude`
  - `weave/.github/workflows`
  - `weave/.handoff`
  - `weave/Cargo.toml`

## Native Diagnostics

- `cd .github_org && make -n ci`
- `cd .github_org && make ci`
- `cd ECC && npm run`
- `cd ECC && npm test`
- `cd agent && cargo metadata --locked --format-version 1`
- `cd agent && cargo test --workspace --all-features --locked`
- `cd envctl && cargo metadata --locked --format-version 1`
- `cd envctl && cargo test --workspace --all-features --locked`
- `cd flexnetos_runner && cargo metadata --locked --format-version 1`
- `cd flexnetos_runner && cargo test --workspace --all-features --locked`
- `cd handoff && cargo metadata --locked --format-version 1`
- `cd handoff && cargo test --workspace --all-features --locked`
- `cd handoff && make -n ci`
- `cd handoff && make ci`
- `cd lane && cargo metadata --locked --format-version 1`
- `cd lane && cargo test --workspace --all-features --locked`
- `cd lifeos && cargo metadata --locked --format-version 1`
- `cd lifeos && cargo test --workspace --all-features --locked`
- `cd lifeos && npm run`
- `cd lifeos && npm test`
- `cd network-control && cargo metadata --locked --format-version 1`
- `cd network-control && cargo test --workspace --all-features --locked`
- `cd prompt_hub && cargo metadata --locked --format-version 1`
- `cd prompt_hub && cargo test --workspace --all-features --locked`
- `cd rusty-idd && cargo metadata --locked --format-version 1`
- `cd rusty-idd && cargo test --workspace --all-features --locked`
- `cd rusty-idd && just --list`
- `cd rusty-idd && just ci`
- `cd rusty-idd && make -n ci`
- `cd rusty-idd && make ci`
- `cd weave && cargo metadata --locked --format-version 1`
- `cd weave && cargo test --workspace --all-features --locked`
- `git -C .github_org rev-parse HEAD`
- `git -C .github_org status --short --branch`
- `git -C ECC rev-parse HEAD`
- `git -C ECC status --short --branch`
- `git -C agent rev-parse HEAD`
- `git -C agent status --short --branch`
- `git -C envctl rev-parse HEAD`
- `git -C envctl status --short --branch`
- `git -C flexnetos_runner rev-parse HEAD`
- `git -C flexnetos_runner status --short --branch`
- `git -C handoff rev-parse HEAD`
- `git -C handoff status --short --branch`
- `git -C harness_hub rev-parse HEAD`
- `git -C harness_hub status --short --branch`
- `git -C lane rev-parse HEAD`
- `git -C lane status --short --branch`
- `git -C lifeos rev-parse HEAD`
- `git -C lifeos status --short --branch`
- `git -C network-control rev-parse HEAD`
- `git -C network-control status --short --branch`
- `git -C prompt_hub rev-parse HEAD`
- `git -C prompt_hub status --short --branch`
- `git -C rusty-idd rev-parse HEAD`
- `git -C rusty-idd status --short --branch`
- `git -C weave rev-parse HEAD`
- `git -C weave status --short --branch`
- `test -f rusty-idd/.idd/knowledge/architecture.json`

## Findings

- 4 resolved owner repos report dirty state
- all owner repos resolved in the system architecture graph
- joined 13 owner repos against .idd/knowledge/system-architecture.json
- selected integrate-fleet-handoff from .idd/knowledge/integration-plan.json
