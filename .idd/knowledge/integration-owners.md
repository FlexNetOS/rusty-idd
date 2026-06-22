# Integration Owner Surfaces

- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/prompt_hub`
- Source plan: `.idd/knowledge/integration-plan.json`
- Source system architecture: `.idd/knowledge/system-architecture.json`
- Change: `integrate-prompt-front-door`
- Capability: `capability:prompt-front-door`
- Owner repos: 1

## Owners

| Owner | Found | Repo | Branch | Dirty | Roles | Markers | Architecture |
|---|---|---|---|---:|---|---|---|
| `repo:prompt-hub` | true | `prompt_hub` | `` | false | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer | rust, openspec, idd-knowledge, handoff, agents, claude, github-actions, make, just | yes |

## Evidence Paths

- `repo:prompt-hub`:
  - `prompt_hub`
  - `prompt_hub/.agents`
  - `prompt_hub/.claude`
  - `prompt_hub/.github/workflows`
  - `prompt_hub/.handoff`
  - `prompt_hub/.idd/knowledge`
  - `prompt_hub/.idd/knowledge/architecture.json`
  - `prompt_hub/Cargo.toml`
  - `prompt_hub/Justfile`
  - `prompt_hub/Makefile`
  - `prompt_hub/openspec`

## Native Diagnostics

- `cd prompt_hub && cargo metadata --locked --format-version 1`
- `cd prompt_hub && cargo test --workspace --all-features --locked`
- `cd prompt_hub && just --list`
- `cd prompt_hub && just ci`
- `cd prompt_hub && make -n ci`
- `cd prompt_hub && make ci`
- `git -C prompt_hub rev-parse HEAD`
- `git -C prompt_hub status --short --branch`
- `test -f prompt_hub/.idd/knowledge/architecture.json`

## Findings

- 0 resolved owner repos report dirty state
- all owner repos resolved in the system architecture graph
- joined 1 owner repos against .idd/knowledge/system-architecture.json
- selected integrate-prompt-front-door from .idd/knowledge/integration-plan.json
