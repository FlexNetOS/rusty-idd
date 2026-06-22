# decide-prompt-hub-boundary

## Why

The local `prompt_hub` repo appears to be the user-facing front door for prompt
and intent capture, while Rusty IDD owns graph-backed OpenSpec planning,
implementation gating, validation, generated artifacts, and merge evidence.

The boundary must be explicit before code integration. Without a recorded
decision, future work could either make Rusty IDD import too much prompt
product surface or make prompt_hub own Rusty IDD lifecycle state.

## What Changes

- Add a Rusty IDD goal file for the PromptHub boundary decision.
- Record local research against `/home/drdave/Desktop/meta/prompt_hub`.
- Define the artifact contract: prompt_hub produces durable goal artifacts;
  Rusty IDD consumes them through goal-file planning and OpenSpec workflow.
- Add ADR and AI_MERGE evidence for the ownership decision.
- Refresh generated Rusty IDD knowledge, diagrams, and manifest artifacts.

## Capabilities

### New Capabilities

- `prompt-front-door-boundary`: define PromptHub as a front-door/spec-producer
  consumed by Rusty IDD through goal artifacts.

### Modified Capabilities

- `prompt-front-door`: clarify that front-door integration means artifact
  production for Rusty IDD, not crate-level ownership inversion.
- `graph-context-planning`: bind plan context to a researched cross-repo goal.

## Impact

- Owner repos:
  - `repo:rusty-idd`
  - `repo:prompt-hub`
- New artifacts:
  - `.idd/goals/prompt-hub-boundary-decision.md`
  - `.handoff/tasks/rusty-idd-prompt-hub-boundary-decision.task.json`
  - `adr/0007-prompt-hub-front-door-boundary.md`
  - `AI_MERGE/37_prompt_hub_boundary_research/README.md`
- Validation:
  - PromptHub native diagnostic: `rtk cargo check --workspace`
  - Rusty IDD generated artifacts and full gates.
