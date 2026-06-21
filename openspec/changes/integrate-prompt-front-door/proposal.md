# integrate-prompt-front-door

## Why

Rusty IDD selected `work:integrate-prompt-front-door` from the integration automation plan to move `Integrate Prompt front door` from `partial` toward implemented system capability.

This change preserves the graph-backed owners, anchors, adopt-first inputs, validation gates, and rollback path before implementation begins.

## What Changes

- Implement integration work item `work:integrate-prompt-front-door`.
- Keep the implementation boundary: Adopt upstream repo surface first, run native diagnostics, then add thin Rusty IDD mapping.
- Use TDD consolidation: adopt current upstream/owner surfaces first, run native diagnostics, then cut only evidenced friction.
- Preserve the parent automation assumptions: prompt_hub feeds Rusty IDD's
  OpenSpec lifecycle, RTK wraps repo commands, Yazelix owns the current
  terminal/parser/runtime direction, and Beads/GRIT remain contributor-run
  upgrade inputs for future slices.

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
  - `RTK/ICM/VOX/GRIT foundation surfaces`
  - `Yazelix terminal/parser/runtime and Beads contributor workflow`
- Adopt-first inputs:
  - `github.com/f/prompts.chat`
  - `github.com/f/ai-prompt`
