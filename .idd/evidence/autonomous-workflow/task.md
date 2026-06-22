# PromptHub Boundary Task Evidence

- Task: `rusty-idd-prompt-hub-boundary-decision`
- Claim: repo-local task card
  `.handoff/tasks/rusty-idd-prompt-hub-boundary-decision.task.json`
- Change: `decide-prompt-hub-boundary`
- Goal file: `.idd/goals/prompt-hub-boundary-decision.md`
- Worktree:
  `/home/drdave/Desktop/meta/rusty-idd/.worktrees/prompt_hub`
- Branch: `feature/prompt_hub`
- Peer repo researched:
  `/home/drdave/Desktop/meta/prompt_hub`

The task was claimed before writing the decision artifacts. Research found
PromptHub is a front-door/spec-producer surface with three Rust crates, prompt
templates, RBAC/audit/search, swarm handoff helpers, and vibe intent flow.

The accepted decision is that Rusty IDD consumes PromptHub-produced goal
artifacts through goal-file planning. PromptHub does not own Rusty IDD `.idd`,
OpenSpec, ADR, validation, manifest, or merge evidence state.

Completion evidence: goal file, OpenSpec proposal/design/spec/tasks, ADR 0007,
AI_MERGE research note, generated knowledge artifacts, architecture diagrams,
manifest, and validation evidence were all produced for the active change.
