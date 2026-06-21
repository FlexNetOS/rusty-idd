# upgrade-codex-harness-rusty-idd-flow - Tasks

## 1. Rusty IDD Artifact Flow

- [x] 1.1 Refresh `.idd/knowledge/index.json`, `.idd/knowledge/report.md`, `.idd/knowledge/architecture.json`, and `.idd/knowledge/architecture.md`.
- [x] 1.2 Generate `.idd/knowledge/system-architecture.{json,md}`.
- [x] 1.3 Generate `.idd/knowledge/operating-model.{json,md}`.
- [x] 1.4 Generate `.idd/knowledge/integration-plan.{json,md}` and `.idd/knowledge/integration-status.{json,md}`.
- [x] 1.5 Generate `.idd/knowledge/plan-context.{json,md}` from the user goal.

## 2. OpenSpec Artifacts

- [x] 2.1 Create proposal for `upgrade-codex-harness-rusty-idd-flow`.
- [x] 2.2 Create `codex-harness-flow` spec delta.
- [x] 2.3 Create design document.
- [x] 2.4 Create the single active Codex harness ADR and summarize retired ADRs outside `adr/`.
- [x] 2.5 Validate the OpenSpec change status and spec structure.

## 3. Harness Upgrade

- [x] 3.1 Update `.codex/agents/*.toml` so agents follow Rusty IDD graph/OpenSpec flow and treat AI_MERGE as evidence only.
- [x] 3.2 Update `.codex/loops/rusty-idd-model-loop.toml` so the default loop is read-only and stops before implementation.
- [x] 3.3 Update `rusty-idd codex env-check` invariants to check Rusty IDD workflow surfaces instead of AI_MERGE-centered phrases.
- [x] 3.4 Update `AGENTS.md`, `.agents/skills/*`, and `docs/rusty-idd/codex-environment.md` to document the true flow.
- [x] 3.5 Update AI_MERGE documentation only where needed to label it as a tool/evidence surface.
- [x] 3.6 Add `crates/merge-tools` and `rusty-idd merge-tools` as the reusable merge-goal package.
- [x] 3.7 Retire active Claude/Gemini `idd-merge-idd` bridge surfaces and route bridge docs to Rusty IDD.

## 4. Validation

- [x] 4.1 Run `rusty-idd spec status openspec/changes/upgrade-codex-harness-rusty-idd-flow`.
- [x] 4.2 Run `rusty-idd spec validate --all`.
- [x] 4.3 Run `rusty-idd codex env-check --workspace .`.
- [x] 4.4 Run `rusty-idd codex model-loop`.
- [x] 4.5 Refresh `.idd/MANIFEST.tsv`.
- [x] 4.6 Run `rusty-idd validate --workspace .`.
- [x] 4.7 Run `rusty-idd merge-tools show`.
