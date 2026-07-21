pub const AGENTS_MD: &str = r#"# AGENTS.md — rusty-idd (Intent Driven Development)

## North Star

Unify repositories by preserving working behavior, making contracts explicit, and merging only through reviewable, test-backed increments.

## Operating Rules

1. Do not flatten two repos before generating inventories, manifest, feature matrix, and contract maps.
2. Do not introduce a new secret provider unless the env/secrets contract says why.
3. Do not delete source code during migration; deprecate first, remove after parity tests pass.
4. Keep PRs narrow: one vertical slice, one migration intent, one validation path.
5. Every generated PR must update the relevant OpenSpec, `.idd`, ADR, and evidence records; use `/AI_MERGE` only when audit, migration, rollback, or merge evidence is required.
6. Never commit real secrets. Use `.env.example`, `.env.schema.example.json`, GitHub Actions secrets, OIDC, or provider references.
7. If two agents conflict, stop and update `/AI_MERGE/05_conflict_risk_register.md` before continuing.
8. Treat `.idd/MANIFEST.tsv` as the audit baseline for generated control-plane artifacts.
9. Break work into sub-59-minute tasks when using cloud coding agents with hard session limits.
10. One integration branch has authority. Other branches are research, staging, or disposable.

## Rusty IDD Workflow Rules

1. Rusty IDD is the intent-driven workflow engine: user intent, graph/context artifacts, OpenSpec proposal/spec/design/ADR/tasks, implementation gating, validation, archive, and handoff evidence.
2. `AI_MERGE/` is a Rusty IDD tool and evidence surface. Use it for audit notes, migration history, rollback, and merge evidence when the workflow calls for it; do not treat it as the main intent source or authoritative control plane.
3. Before implementation, create or refresh the relevant `.idd/knowledge/*` graph artifacts and bind the goal with `rusty-idd knowledge plan-context`.
4. Before writes, create or select an OpenSpec change and verify readiness with `rusty-idd spec status` or `rusty-idd spec next`.
5. ADR decisions live in repo-level `adr/`; accepted ADRs are immutable. Supersede with a new ADR instead of editing prior accepted decisions.
6. Implementation follows tasks only after the OpenSpec artifacts are ready.
7. Validation must refresh deterministic artifacts: `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, OpenSpec status, and Rusty IDD validation.

## Codex Environment Rules

1. Use `.idd/knowledge/report.md`, `.idd/knowledge/architecture.md`, `.idd/knowledge/plan-context.md`, and OpenSpec status before broad manual rescans or edits.
2. Use repo-local Codex surfaces intentionally:
   - `AGENTS.md` for durable repo rules.
   - `.agents/skills` for reusable workflows.
   - `.codex/agents` for project-scoped subagents.
   - `.codex/rules` for project-scoped exec policy.
   - `.codex/hooks.json` and `.codex/hooks` for lifecycle checks.
3. The default Codex harness is read-only and design-first. A write-capable implementation pass requires explicit authorization and ready OpenSpec artifacts.
4. Adopt first, cut after evidence. For crate, tool, agent, skill, hook, or repo integrations, first bring the upstream/current surface forward intactly enough to diagnose it, then cut only evidenced compile, audit, security, or scope friction.
5. Do not replace an upstream capability with local guesses before proving the upstream surface cannot be used. If a cut is required, record the reason in an ADR or evidence note and keep the adapter thin.
6. Upgrade only. Never downgrade a working surface, dependency, action, model, agent, skill, or generated artifact to simplify a task.
7. Treat stale or orphaned work as unfinished by default unless evidence proves it is intentionally local and ignored.
8. Tooling required to run this repo must be tracked and provisioned through the parent `meta` / `envctl` path first. Do not install missing binaries into user-global state to make a gate pass.
9. Parallel subagents are for read-heavy exploration, verification, and gap hunting unless a single integration branch/worktree owner coordinates writes.
10. Host service and process management is out of scope for this repo. Do not use raw `systemctl`, daemon kills, or binary installation as a way to make repository work pass.

## Required PR Evidence

- Build command result
- Test command result
- Lint/typecheck result
- Secret scan result
- Migration note explaining old path -> new path
- Rollback path
- Updated manifest or note explaining why unchanged

## Merge Authority

Parallel agents may analyze and propose branches, but only the integration branch is authoritative.
"#;

pub const LOCK_TEMPLATE: &str = r#"# IDD Integration Lock

This file exists to prevent uncontrolled parallel merge authority.

- Owner: unassigned
- Active branch: unassigned
- Active intent: unassigned
- Active task file: unassigned
- Last update: unassigned

Rule: only one branch may hold integration authority at a time.
"#;

pub const ENV_SCHEMA_EXAMPLE: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "title": "Intent Driven Development Environment Contract",
  "type": "object",
  "required": [],
  "properties": {
    "EXAMPLE_API_URL": {
      "type": "string",
      "description": "Example non-secret endpoint. Replace with project-specific key."
    }
  },
  "additionalProperties": true
}
"#;

pub const ENV_CONTRACT_EXAMPLE: &str = r#"runtime:
  secrets_required: false
  optional_env: []

ci_release:
  secrets:
    - name: GITHUB_TOKEN
      required: true
      provider: github-actions
      purpose: Automatic token for pull request checks and repository automation.
"#;

pub const GITHUB_ACTIONS_CI: &str = r#"name: ci

on:
  workflow_call:
  pull_request:
  push:
    branches: [main, develop]

permissions:
  contents: read

jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Resolve meta-owned Rust root
        id: rust-meta
        run: |
          meta_root="$(cd "$GITHUB_WORKSPACE/.." && pwd -P)/meta"
          mkdir -p "$meta_root/.env/rust" "$meta_root/.cache/rust"
          echo "meta_root=$meta_root" >> "$GITHUB_OUTPUT"
      - name: Restore meta-owned Rust tool cache
        uses: actions/cache@v4
        with:
          path: |
            ${{ steps.rust-meta.outputs.meta_root }}/.env/rust
            ${{ steps.rust-meta.outputs.meta_root }}/.cache/rust
          key: rusty-idd-envctl-rust-tools-nightly-${{ runner.os }}-${{ hashFiles('scripts/ci/envctl-rust-env.sh', 'scripts/ci/envctl-rust-audit.sh') }}
          restore-keys: |
            rusty-idd-envctl-rust-tools-nightly-${{ runner.os }}-
      - name: Restore workspace target cache
        uses: actions/cache@v4
        with:
          path: |
            target
          key: rusty-idd-target-nightly-${{ runner.os }}-${{ hashFiles('Cargo.lock', 'scripts/ci/envctl-rust-env.sh', 'scripts/ci/envctl-rust-audit.sh') }}
          restore-keys: |
            rusty-idd-target-nightly-${{ runner.os }}-
      - name: Envctl Rust toolchain and cache
        env:
          RUSTY_IDD_META_ROOT: ${{ steps.rust-meta.outputs.meta_root }}
        run: scripts/ci/envctl-rust-env.sh ci
      - name: Strict envctl Rust toolchain audit
        run: scripts/ci/envctl-rust-audit.sh
      - name: Merge-tools verification
        run: cargo run --bin rusty-idd -- merge-tools verify --workspace .
      - name: Format
        run: cargo fmt --all -- --check
      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      - name: Test
        run: cargo test --all --locked
      - name: IDD validation
        run: cargo run --bin rusty-idd -- validate --workspace .
      - name: IDD manifest refresh check
        run: |
          cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv
          git diff --exit-code -- .idd/MANIFEST.tsv
      - name: Security audit
        run: cargo audit --deny warnings
"#;

pub const TASK_TEMPLATE: &str = r#"# IDD Task: {{TITLE}}

## Intent

{{TITLE}}

## Scope

- Kind: {{KIND}}
- Repo area: TBD
- Files expected to change: TBD
- Files forbidden to change: TBD
- Branch authority: non-authoritative unless `.idd/LOCK.md` assigns this task

## Inputs

- `/AI_MERGE/00_repo_a_inventory.md`
- `/AI_MERGE/01_repo_b_inventory.md`
- `/AI_MERGE/02_feature_matrix.md`
- `/AI_MERGE/03_env_and_secret_contracts.md`
- `/AI_MERGE/04_merge_plan.md`
- `/AI_MERGE/08_agent_queue.md`
- `/AI_MERGE/10_parity_test_plan.md`

## Required Output

- Small PR
- Updated migration notes
- Updated tests
- Updated docs if user-facing behavior changes
- Updated `.idd/MANIFEST.tsv` if generated control-plane files change

## Definition of Done

- [ ] Build passes
- [ ] Tests pass
- [ ] Lint/typecheck passes
- [ ] Secret scan has no critical findings
- [ ] Rollback path documented
- [ ] Contract map updated
- [ ] No source deletion without parity evidence

## Agent Guardrails

Do not invent provider-specific behavior. Do not remove old implementation until parity has been proven by tests or an explicit migration note. Never print secret values into logs, PR descriptions, or generated documents.
"#;

pub const ISSUE_TEMPLATE: &str = r#"name: IDD Agent Task
description: Small, testable repo-unification task for AI or human execution
title: "[IDD] "
labels: ["idd", "agent-task"]
body:
  - type: textarea
    id: intent
    attributes:
      label: Intent
      description: One narrow outcome. Do not paste a mega-merge request here.
    validations:
      required: true
  - type: textarea
    id: source-of-truth
    attributes:
      label: Rusty IDD artifacts
      value: |
        Use AGENTS.md, .idd/knowledge, and OpenSpec artifacts as the source of truth.
        Treat AI_MERGE as an audit/evidence surface when the workflow calls for it.
    validations:
      required: true
  - type: textarea
    id: constraints
    attributes:
      label: Constraints
      value: |
        - Preserve existing behavior.
        - Do not commit secrets.
        - Do not delete old implementation until parity tests pass.
        - Keep the PR small enough to review.
    validations:
      required: true
  - type: checkboxes
    id: done
    attributes:
      label: Definition of Done
      options:
        - label: Build passes
        - label: Tests pass
        - label: Lint/typecheck passes
        - label: Secret scan passes
        - label: Rollback path is documented
"#;

pub const PR_TEMPLATE: &str = r#"## IDD PR Evidence

### Intent

-

### Changed surfaces

-

### Old path -> new path migration note

-

### Validation evidence

- Build:
- Tests:
- Lint/typecheck:
- `rusty-idd validate`:
- Secret scan:

### Rollback path

-

### Rusty IDD artifacts

- [ ] OpenSpec proposal/spec/design/ADR/tasks updated or intentionally unchanged
- [ ] `.idd/knowledge/*` refreshed or intentionally unchanged
- [ ] Manifest updated or intentionally unchanged
- [ ] AI_MERGE evidence updated only if audit, migration, rollback, or merge records are required
  - [ ] Feature matrix updated if capability changed
  - [ ] Env/secrets contract updated if config changed
  - [ ] Conflict register updated if collision found
  - [ ] Agent queue updated
"#;

pub const COPILOT_INSTRUCTIONS: &str = r#"# Repository Instructions for AI Coding Agents

Follow `AGENTS.md` first. Treat Rusty IDD as the intent-driven workflow engine.
`AI_MERGE/` is an audit and evidence tool that Rusty IDD may use, not the main
intent source.

Preferred workflow:

1. Read the user goal and active OpenSpec change completely.
2. Inspect `.idd/knowledge/report.md`, `.idd/knowledge/architecture.md`, and `.idd/knowledge/plan-context.md`.
3. Verify `rusty-idd spec status <change>` before editing.
4. Make the smallest behavior-preserving change authorized by `tasks.md`.
5. Refresh `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and validation artifacts.
6. Update `AI_MERGE/` only when audit, migration, rollback, or merge evidence is required.
7. Never commit secret values.

Do not perform broad cleanup, style-only rewrites, dependency swaps, or folder flattening unless the task explicitly says so.
"#;

pub const SECURITY_MD: &str = r#"# Security Policy

## Secret handling

- Do not commit real secrets.
- Prefer OIDC for cloud credentials in CI instead of long-lived access keys.
- Use GitHub Actions secrets, repository/environment variables, SOPS, Infisical, OpenBao/Vault, or a local secure store by explicit contract only.
- Mask non-GitHub secret values in logs.

## Agent safety

- Do not give broad write authority to parallel agents.
- Use one authoritative integration branch.
- Require review before merging agent-generated code.
- Preserve rollback paths for each migration PR.
"#;

pub const AGENT_QUEUE: &str = r#"# Agent Queue

This queue serializes work for GitHub cloud agents, local agents, and human contributors.

| Order | Task | Branch | Agent | Status | Blocking files | Notes |
|---:|---|---|---|---|---|---|
| 1 | Import repositories under `/imports` | `idd/imports` | TBD | queued | `/imports` | No flattening |
| 2 | Normalize env/secrets contract | `idd/env-secrets` | TBD | queued | config, CI | No secret values |
| 3 | Create canonical interfaces | `idd/interfaces` | TBD | queued | crates/apps | Preserve old behavior |
| 4 | Add parity tests | `idd/parity-tests` | TBD | queued | tests | Required before deletion |
| 5 | Migrate first vertical slice | `idd/vertical-slice-001` | TBD | queued | TBD | Small PR |
| 6 | Final dedupe cleanup | `idd/final-cleanup` | TBD | blocked | TBD | Only after parity passes |

Only one row may be marked `active` at a time when it touches overlapping files.
"#;

pub const GITHUB_EXECUTION: &str = r#"# GitHub Execution Plan

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
"#;

pub const PARITY_TEST_PLAN: &str = r#"# Parity Test Plan

## Goal

Prove that migrated behavior matches or intentionally improves old behavior before deleting old code.

## Required test classes

| Class | Purpose | Required before deletion? |
|---|---|---|
| Golden input/output tests | Compare old and new behavior on fixed fixtures | yes |
| Contract tests | Verify canonical interface compatibility | yes |
| Env resolution tests | Verify config/secret precedence | yes |
| CI workflow dry-read | Verify workflow references expected secrets/vars | yes |
| Rollback smoke test | Prove old path can be restored or feature flag disabled | yes |

## Deletion rule

No old implementation is deleted until parity tests pass and the migration note names the replacement path.
"#;

pub const PROVIDER_MATRIX: &str = r#"# Provider Matrix

| Provider / Tool | Intended role | May store secrets? | Notes |
|---|---|---:|---|
| GitHub Actions secrets | CI secret injection | yes | Prefer environment-scoped secrets for deployment |
| GitHub Actions variables | Non-secret CI config | no | Do not store credentials here |
| OIDC | Short-lived cloud auth | no long-lived secret | Preferred over static cloud keys where supported |
| SOPS | Encrypted secrets in Git | encrypted only | Requires key-management decision |
| Infisical | Central secret management | yes | Use explicit project/env mapping |
| OpenBao/Vault | Central secret management | yes | Use policies and short TTL credentials |
| direnv/mise | Local developer env loading | no by default | Keep local-only values out of Git |
| Docker/Compose env_file | Local/runtime config | maybe | Treat `.env` as local-only unless encrypted |

Decision rule: do not add a provider until the env/secrets contract lists the key namespace, source of truth, rotation model, and CI/local resolution path.
"#;
