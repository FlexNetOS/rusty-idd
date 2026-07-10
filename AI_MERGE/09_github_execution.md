# GitHub Execution Plan

## Current GitHub-native pattern

Use GitHub Issues/PRs as the auditable task ledger. Assign one narrow issue at a time to a coding agent. Let agents research, plan, create a branch, push commits, and produce a PR, but keep the integration branch serialized.

## Branch model

```text
main
└── idd/integration                # authoritative branch
    ├── idd/research/repo-map       # disposable/research only
    ├── idd/env-secrets             # narrow task branch
    ├── idd/interfaces              # narrow task branch
    └── idd/vertical-slice-001      # narrow task branch
```

## Agent assignment rules

1. One task per issue.
2. One branch per task.
3. One PR per task.
4. PRs must update OpenSpec, `.idd/knowledge`, manifest, and AI_MERGE evidence only as required by the Rusty IDD workflow.
5. If an agent hits a timeout, split the task instead of increasing scope.
6. If an agent needs a second repo, import or mirror required context into this repo first; do not assume one cloud-agent run can mutate two repos.

## Minimum required gates

- branch protection
- required status checks
- PR template
- secret scan
- CODEOWNERS or explicit reviewer rule
- `idd validate`
