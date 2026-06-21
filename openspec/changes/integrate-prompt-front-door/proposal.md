# integrate-prompt-front-door

## Why

Rusty IDD selected `work:integrate-prompt-front-door` from the integration automation plan to move `Integrate Prompt front door` from `partial` toward implemented system capability.

This change preserves the graph-backed owners, anchors, adopt-first inputs, validation gates, and rollback path before implementation begins.

## What Changes

- Implement integration work item `work:integrate-prompt-front-door`.
- Keep the implementation boundary: Adopt upstream repo surface first, run native diagnostics, then add thin Rusty IDD mapping.
- Use TDD consolidation: adopt current upstream/owner surfaces first, run native diagnostics, then cut only evidenced friction.

## Capabilities

### New Capabilities

- `prompt-front-door`: Integrate Prompt front door

### Modified Capabilities

- `integration-automation-plan`: this work item is now executing through OpenSpec artifacts.

## Impact

- Owner repos:
  - `repo:prompt-hub`
- Anchors:
  - `github.com/f/prompts.chat`
  - `github.com/f/ai-prompt`
  - `prompt_hub front door to handoff and rusty-idd`
- Adopt-first inputs:
  - `github.com/f/prompts.chat`
  - `github.com/f/ai-prompt`
