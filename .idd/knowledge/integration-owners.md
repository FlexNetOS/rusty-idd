# Integration Owner Surfaces

- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/autonomous-workflow-hooks`
- Source plan: `.idd/knowledge/integration-plan.json`
- Source system architecture: `.idd/knowledge/system-architecture.json`
- Change: `integrate-prompt-front-door`
- Capability: `capability:prompt-front-door`
- Owner repos: 1

## Owners

| Owner | Found | Repo | Branch | Dirty | Roles | Markers | Architecture |
|---|---|---|---|---:|---|---|---|
| `repo:prompt-hub` | true | `prompt_hub` | `main` | true | role:agent-environment, role:capability-hub, role:fleet-handoff, role:rust-code-surface, role:spec-producer | rust, handoff, claude, github-actions | no |

## Evidence Paths

- `repo:prompt-hub`:
  - `prompt_hub`
  - `prompt_hub/.claude`
  - `prompt_hub/.github/workflows`
  - `prompt_hub/.handoff`
  - `prompt_hub/Cargo.toml`

## Native Diagnostics

- `cd prompt_hub && cargo metadata --locked --format-version 1`
- `cd prompt_hub && cargo test --workspace --all-features --locked`
- `git -C prompt_hub rev-parse HEAD`
- `git -C prompt_hub status --short --branch`

## Findings

- 1 resolved owner repos report dirty state
- all owner repos resolved in the system architecture graph
- joined 1 owner repos against .idd/knowledge/system-architecture.json
- selected integrate-prompt-front-door from .idd/knowledge/integration-plan.json
