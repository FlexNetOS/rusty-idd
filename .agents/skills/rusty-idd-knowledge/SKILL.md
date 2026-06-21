---
name: rusty-idd-knowledge
description: "Use when investigating Rusty IDD with generated knowledge artifacts, querying .idd/knowledge indexes and reports, or preparing bounded repo context packs for agents."
---

# rusty-idd Knowledge Skill

Use this skill when investigating this repository with the generated `rusty-idd knowledge`
artifacts or when preparing compact context for an agent.

## Workflow

1. Start with `.idd/knowledge/report.md` and `.idd/knowledge/architecture.md`.
   - Check the workspace fingerprint, file count, token count, top files, hotspots, and findings.
   - Use the architecture graph to map repo components, integration surfaces, OpenSpec
     stages, and evidence paths before deciding which files to edit.
   - If the report is missing or stale, run:
     `rusty-idd knowledge refresh --workspace .`

2. Query the graph index before rescanning source manually.
   - The local Rusty IDD index uses the vendored CodeGraph tree-sitter registry
     for supported Rust, TypeScript/JavaScript, Python, Go, Java, C/C++, Swift,
     Ruby, and PHP files. Treat missing local parser coverage as a Rusty IDD
     adapter gap, not proof that tree-sitter is absent from the system; the
     current system parser/runtime direction is Yazelix-backed.
   - Symbol lookup:
     `rusty-idd knowledge query --index .idd/knowledge/index.json --symbol NAME`
   - File lookup:
     `rusty-idd knowledge query --index .idd/knowledge/index.json --file PATH`
   - Impact lookup:
     `rusty-idd knowledge query --index .idd/knowledge/index.json --impact NODE_ID`

3. Use focused pack outputs for temporary context.
   - Prefer `/tmp` for exploratory packs:
     `rusty-idd knowledge pack --workspace . --out /tmp/rusty-idd-pack.xml --style xml --compress`
   - Use `--include` and `--ignore` to narrow the bundle.
   - Use `--remove-comments`, `--remove-empty-lines`, `--line-numbers`, `--truncate-base64`,
     `--git-diff`, and `--git-log` only when they directly help the task.
   - Do not commit ad hoc pack files.

4. Generate a fresh architecture graph when integration boundaries are unclear.
   - Markdown for reading:
     `rusty-idd knowledge architecture --workspace . --out /tmp/rusty-idd-architecture.md`
   - JSON for tooling:
     `rusty-idd knowledge architecture --workspace . --out /tmp/rusty-idd-architecture.json`
   - The graph combines CodeGraph-backed source structure with repomix-backed context
     package metrics and maps both to Rusty IDD automation stages.

5. Generate a system graph when the task crosses repo boundaries.
   - Use the parent meta workspace when working from this checkout:
     `rusty-idd knowledge system-architecture --workspace . --system-root .. --out /tmp/rusty-idd-system-architecture.md`
   - The system graph prefers `meta project list --json` and falls back to immediate
     child git repo discovery.
   - When a peer repo already publishes `.idd/knowledge/architecture.json`, the
     system graph ingests a bounded architecture summary: CodeGraph metrics,
     repomix package metrics, languages, top components, and integration surfaces.
     Invalid peer architecture artifacts are recorded as findings and do not block
     graph generation.
   - Use it to map Rusty IDD to handoff, weave, Obscura, Yazelix, envctl,
     prompt/meta producers, hubs, and agent environment repos before planning a
     cross-repo integration.
   - The command is read-only: it records peer repo state but does not start
     MCP servers, daemons, host services, or mutate peer repos.

6. Generate graph planning context before writing or changing OpenSpec artifacts.
   - Markdown for humans/agents:
     `rusty-idd knowledge plan-context --workspace . --out /tmp/rusty-idd-plan-context.md --goal "describe the task"`
   - JSON for automation:
     `rusty-idd knowledge plan-context --workspace . --out /tmp/rusty-idd-plan-context.json --goal "describe the task"`
   - The planning context consumes `.idd/knowledge/architecture.json` and, when
     present, `.idd/knowledge/system-architecture.json` and
     `.idd/knowledge/operating-model.json`; use it to choose
     proposal/spec/design/ADR/task scope, system capability owners, and
     integration boundaries before edits.
   - Selected system repos preserve their peer architecture summaries, so
     OpenSpec proposals and task plans can map cross-repo components without
     rescanning or mutating peer repos.

7. Generate the operating model when the task touches the wider agentic company system.
   - Markdown for humans/agents:
     `rusty-idd knowledge operating-model --workspace . --out /tmp/rusty-idd-operating-model.md`
   - JSON for automation:
     `rusty-idd knowledge operating-model --workspace . --out /tmp/rusty-idd-operating-model.json`
   - The operating model consumes `.idd/knowledge/system-architecture.json` and
     maps discovered repos to agentic-company layers and capabilities: board
     reasoning, Rusty IDD/handoff control plane, weave communication, envctl and
     vault relay, prompt front door, ruvector runtime, LifeOS/front-door UX, Teri
     simulation, network control, Yazelix terminal/parser/runtime, RTK AI
     foundation, GRIT/Beads agent-run upgrades, Lua/AR, and distributed device
     fabric.
   - External or future anchors such as Cognitum, upstream prompt repos, Lua/AR,
     Beads upstreams, and distributed device fabric are recorded as findings
     unless repo evidence exists.

8. Generate the integration plan when the task needs ordered cross-repo execution.
   - Markdown for humans/agents:
     `rusty-idd knowledge integration-plan --workspace . --out /tmp/rusty-idd-integration-plan.md`
   - JSON for automation:
     `rusty-idd knowledge integration-plan --workspace . --out /tmp/rusty-idd-integration-plan.json`
   - The integration plan consumes `.idd/knowledge/operating-model.json` and
     turns partial, external, or missing capabilities into OpenSpec-ready work
     items with owner repos, unresolved anchors, adopt-first inputs, validation
     gates, and rollback guidance.
   - Use this before cross-repo implementation to choose the next vertical slice
     and to keep upstream/adopt-first anchors such as Beads, Cognitum, prompt
     sources, goose-like chat, or vault/toolchain surfaces explicit.

9. Generate integration status before scaffolding more work.
   - Markdown for humans/agents:
     `rusty-idd knowledge integration-status --workspace . --out /tmp/rusty-idd-integration-status.md`
   - JSON for automation:
     `rusty-idd knowledge integration-status --workspace . --out /tmp/rusty-idd-integration-status.json`
   - The status queue consumes `.idd/knowledge/integration-plan.json` and
     joins work items to OpenSpec change state. Use it to see planned,
     incomplete-scaffold, scaffolded, ready-to-archive, and archived items and
     to identify the next planned work item.

10. Generate owner surfaces before implementing cross-repo work.
   - Markdown for humans/agents:
     `rusty-idd knowledge integration-owners --workspace . --next --out /tmp/rusty-idd-integration-owners.md`
   - JSON for automation:
     `rusty-idd knowledge integration-owners --workspace . --next --out /tmp/rusty-idd-integration-owners.json`
   - Select exactly one work item with `--next`, `--next-planned`, `--change`,
     `--capability`, or `--work-item`.
   - Prefer `--next` for durable `.idd/knowledge/integration-owners.*`
     artifacts so the report stays on the highest-priority non-archived work
     item while a change is active, then advances after archive. Use
     `--next-planned` only when active/scaffolded changes should be skipped.
   - The owner-surface report consumes `.idd/knowledge/integration-plan.json` and
     `.idd/knowledge/system-architecture.json`, joins owner repo ids to peer repo
     state, and emits branches, dirty flags, markers, roles, local architecture
     summaries, evidence paths, and native diagnostic command candidates.
   - Use it before native diagnostics and before any consolidation cut. The
     command is read-only and does not mutate peer repos or start services.

11. Generate integration readiness before crossing toolchain, host, or vault boundaries.
   - Markdown for humans/agents:
     `rusty-idd knowledge integration-readiness --workspace . --next --out /tmp/rusty-idd-integration-readiness.md`
   - JSON for automation:
     `rusty-idd knowledge integration-readiness --workspace . --next --out /tmp/rusty-idd-integration-readiness.json`
   - The readiness report consumes the same selector and source artifacts as
     `integration-owners`, then emits owner state, required tools, parent
     `meta`/`envctl` provisioning boundaries, runtime assumptions, feature
     gates, native diagnostic command expectations, validation, and rollback.
   - The command is deterministic and read-only. It does not install missing
     binaries, probe vault paths such as Cognitum, mint relay credentials,
     mutate peer repos, or start services.
   - Use it to decide whether a missing tool belongs in parent-managed
     `meta`/`envctl`, a repo-local tracked surface, or a feature-gated runtime
     path before implementation begins.

12. Scaffold integration work before implementation.
   - To create the next OpenSpec change from the highest-priority planned
     integration work item:
     `rusty-idd spec plan-integration --base .`
   - To select a specific work item:
     `rusty-idd spec plan-integration --base . --change integrate-fleet-handoff`
     or
     `rusty-idd spec plan-integration --base . --capability fleet-handoff`
   - The command consumes `.idd/knowledge/integration-plan.json`, writes
     proposal, design, tasks, and spec delta artifacts, preserves owner repos,
     adopt-first inputs, validation gates, and rollback, and refuses to
     overwrite existing files unless `--force` is passed.
   - With no selector, it skips work items that already have active or archived
     OpenSpec changes, so repeated runs advance the queue.
   - This is the handoff from graph planning into the normal Rusty IDD
     OpenSpec lifecycle; implementation still follows adopt-first TDD.

13. Use machine-readable OpenSpec lifecycle status for automation.
   - Human status:
     `rusty-idd spec status openspec/changes/CHANGE_ID`
   - JSON status for handoff, runners, and automation:
     `rusty-idd spec status --json openspec/changes/CHANGE_ID`
   - The JSON output preserves schema identity, ordered artifact status, done
     count, total count, archivability, and next ready artifact. Prefer this
     over scraping human status text.

14. Read compactly.
   - Grep or slice generated packs and indexes instead of dumping the full file.
   - Treat `.idd/knowledge/index.json`, `.idd/knowledge/report.md`,
     `.idd/knowledge/architecture.json`, `.idd/knowledge/architecture.md`,
     `.idd/knowledge/system-architecture.json`, and
    `.idd/knowledge/system-architecture.md`, `.idd/knowledge/plan-context.json`,
    `.idd/knowledge/plan-context.md`, `.idd/knowledge/operating-model.json`,
    `.idd/knowledge/operating-model.md`, `.idd/knowledge/integration-plan.json`,
    `.idd/knowledge/integration-plan.md`, `.idd/knowledge/integration-owners.json`,
    `.idd/knowledge/integration-owners.md`,
    `.idd/knowledge/integration-readiness.json`, and
    `.idd/knowledge/integration-readiness.md`
     as durable control-plane artifacts; keep them deterministic and bounded.

15. Stay in-process.
   - Do not start MCP servers, daemons, or host services for this workflow.
   - MCP, daemon, domain, vector, SurrealDB, and cloud/provider integrations may
     exist in the wider meta system, but they are feature-gated or external
     surfaces rather than default knowledge behavior.

## Validation

After changing source or control-plane files, refresh artifacts and run:

```bash
cargo run --bin rusty-idd -- knowledge refresh --workspace .
cargo run --bin rusty-idd -- knowledge architecture --workspace . --out /tmp/rusty-idd-architecture.md
cargo run --bin rusty-idd -- knowledge system-architecture --workspace . --system-root .. --out /tmp/rusty-idd-system-architecture.md
cargo run --bin rusty-idd -- knowledge operating-model --workspace . --out /tmp/rusty-idd-operating-model.md
cargo run --bin rusty-idd -- knowledge integration-plan --workspace . --out /tmp/rusty-idd-integration-plan.md
cargo run --bin rusty-idd -- knowledge integration-status --workspace . --out /tmp/rusty-idd-integration-status.md
cargo run --bin rusty-idd -- knowledge integration-owners --workspace . --next --out /tmp/rusty-idd-integration-owners.md
cargo run --bin rusty-idd -- knowledge integration-readiness --workspace . --next --out /tmp/rusty-idd-integration-readiness.md
cargo run --bin rusty-idd -- knowledge plan-context --workspace . --out /tmp/rusty-idd-plan-context.md --goal "describe the task"
cargo run --bin rusty-idd -- validate --workspace .
```
