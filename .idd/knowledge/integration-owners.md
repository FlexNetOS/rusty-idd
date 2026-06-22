# Integration Owner Surfaces

- Workspace root: `/home/drdave/Desktop/meta/rusty-idd/.worktrees/e2e-test-suite`
- Source plan: `.idd/knowledge/integration-plan.json`
- Source system architecture: `.idd/knowledge/system-architecture.json`
- Change: `integrate-prompt-front-door`
- Capability: `capability:prompt-front-door`
- Owner repos: 1

## Owners

| Owner | Found | Repo | Branch | Dirty | Roles | Markers | Architecture |
|---|---|---|---|---:|---|---|---|
| `repo:prompt-hub` | true | `prompt_hub` | `` | false | role:agent-environment, role:capability-hub, role:spec-producer |  | no |

## Evidence Paths

- `repo:prompt-hub`:
  - `prompt_hub`

## Native Diagnostics

- `git -C prompt_hub rev-parse HEAD`
- `git -C prompt_hub status --short --branch`

## Findings

- 0 resolved owner repos report dirty state
- all owner repos resolved in the system architecture graph
- joined 1 owner repos against .idd/knowledge/system-architecture.json
- selected integrate-prompt-front-door from .idd/knowledge/integration-plan.json
