# add-autonomous-workflow-hooks - Design

## Context

Rusty IDD's project rules already require a full autonomous workflow: ICM
recall, graph-backed context, OpenSpec artifacts, implementation from tasks,
validation, manifest/evidence refresh, and PR handoff. The current Codex hook
only runs `codex env-check` at Stop, which verifies static environment
invariants but cannot stop an agent from starting code changes before the
change-specific Rusty IDD artifacts exist.

The hook enforcement must stay Rust-native and repo-scoped. It must also avoid
host service management, global tool installation, or a new scripting runtime.

## Goals / Non-Goals

**Goals:**

- Add a `rusty-idd codex workflow-check` command for lifecycle hooks.
- Register pre/post Codex hooks around tool use and turn/subagent completion.
- Enforce the worktree-from-`develop` rule before implementation.
- Enforce plan-context, OpenSpec readiness, task-card evidence, validation
  evidence, and PR/automerge evidence as the workflow advances.
- Keep checks deterministic, file/git based, and suitable for tests.

**Non-Goals:**

- Replacing the OpenSpec lifecycle engine.
- Managing GitHub repository settings or host services from hooks.
- Installing missing tools into user-global state.
- Blocking read-only exploration before the change has implementation intent.
- Rewriting the wider handoff/ledger system.

## Decisions

- Add a new `codex workflow-check` subcommand with a `--phase` argument. The
  registered phases are `pre-tool`, `post-tool`, and `stop`.
- Use Rust CLI checks instead of shell scripts so runtime audit remains
  Rust-native and tests can exercise behavior directly.
- The pre-tool phase verifies branch/worktree shape, `develop` ancestry,
  plan-context freshness by presence, OpenSpec change readiness by file
  contract, and task-card evidence.
- The post-tool and stop phases additionally verify dirty-work handoff signals:
  validation evidence and PR/automerge evidence files when tracked files have
  changed.
- PR/automerge evidence is represented as repository evidence text so hooks can
  verify the required final handoff without needing live network access.

## Risks / Trade-offs

- Hooks can only prove local evidence unless the final PR step queries GitHub.
  Mitigation: final delivery still runs `gh pr create`, `gh pr merge --auto`,
  and PR checks outside the hook; the hook validates the recorded evidence.
- A strict pre-tool hook can annoy read-only exploration. Mitigation: the check
  is intended for tool-use lifecycle enforcement and reports concrete missing
  artifacts instead of mutating state.
- Existing agents may need to trust the changed hook hash. Mitigation: document
  the change and keep the command path stable.

## Migration Plan

1. Add this OpenSpec change and ADR before implementation.
2. Implement the Rust-native workflow check command and tests.
3. Register pre/post/stop hooks in `.codex/hooks.json`.
4. Update Codex environment docs and skill guidance.
5. Refresh knowledge/manifest artifacts.
6. Validate locally, checkpoint the task, push the feature branch, open a PR
   into `develop`, enable auto-merge, and record evidence.

## Open Questions

- Should future work add a first-class `rusty-idd handoff evidence` command so
  hooks can consume structured JSON instead of text evidence files?
