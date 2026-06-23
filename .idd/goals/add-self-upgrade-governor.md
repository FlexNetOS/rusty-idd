# add-self-upgrade-governor

## Step 1: Run This Goal Through Rusty IDD

```bash
rusty-idd --goal-file .idd/goals/add-self-upgrade-governor.md
```

If the active CLI surface requires a subcommand for goal binding, use the same
goal file as the input to the Rusty IDD planning command:

```bash
rusty-idd knowledge plan-context \
  --workspace . \
  --out .idd/knowledge/plan-context.md \
  --goal-file .idd/goals/add-self-upgrade-governor.md
```

## Goal

Create a first-class Rusty IDD self-upgrade governor workflow from the approved
brainstorm below. The always-on harness must remain small. Rusty IDD must own
the task-scoped packages, candidate-goal generation, lifecycle gates,
verification, publishing, and learning loop.

## Approved Brainstorm To Execute

Yes. The path is not "one infinite agent with every tool loaded." It is a
bounded renewable Rusty IDD loop: each cycle discovers work, writes one narrow
goal, generates the right task-scoped package, runs it through OpenSpec,
executes one PR, verifies hard, merges, then starts the next cycle from the new
repo truth.

The repo already has several pieces:

- `rusty-idd knowledge` gives graph/context artifacts.
- `rusty-idd spec status/next` gives lifecycle gates.
- `rusty-idd run` can drive OpenSpec tasks.
- `.codex/loops/rusty-idd-model-loop.toml` already defines read-only
  explore/gap/verify passes.
- `.codex/agents/*` already splits explorer, gap-hunter, verifier,
  implementer roles.
- `codex workflow-check` already enforces active-change, validation, and PR
  evidence.
- `merge-tools` is already a Rusty IDD-owned package model for one workflow
  family.

What is missing is the self-upgrade governor.

## Core Idea

Rusty IDD should own a first-class command family something like:

```bash
rusty-idd self-upgrade scan
rusty-idd self-upgrade propose
rusty-idd self-upgrade goal
rusty-idd self-upgrade package
rusty-idd self-upgrade run
rusty-idd self-upgrade verify
rusty-idd self-upgrade publish
rusty-idd self-upgrade next
```

The always-on harness stays tiny. It only says: "Ask Rusty IDD for the next
scoped package." Rusty IDD does the real routing.

The loop should look like this:

```text
repo truth
  -> scan
  -> opportunity graph
  -> candidate goals
  -> architecture/design reasoning
  -> goal file
  -> OpenSpec change
  -> task-scoped package
  -> implementation
  -> exhaustive verify
  -> PR/merge
  -> ICM + knowledge refresh
  -> next goal
```

## How Rusty IDD Writes Its Own Goals

It should not let a model free-write arbitrary goals directly into execution.
Instead, use a typed pipeline:

```text
Finding
  -> Opportunity
  -> Hypothesis
  -> CandidateGoal
  -> GoalReview
  -> ApprovedGoal
  -> OpenSpecChange
  -> Package
```

Example:

```text
Finding:
  "The verify package exists as docs/goal artifacts but has no first-class CLI package command."

Opportunity:
  "Promote verify package from documented workflow to executable Rusty IDD package."

Hypothesis:
  "A first-class verify package reduces copy/paste prompts and improves post-task quality."

CandidateGoal:
  "Add `rusty-idd harness package --stage verify` that emits contracts, agents, tools, evidence schema, and gates."

GoalReview:
  risk: medium
  blast_radius: cli + docs + tests
  package: verify
  requires_human: false if docs/CLI only; true if changing merge policy

ApprovedGoal:
  saved under `.idd/goals/...`

OpenSpecChange:
  proposal/design/spec/ADR/tasks generated in order.
```

That gives self-authored goals without letting the loop become sloppy.

## The Endless Loop Should Be Two Loops

Split it:

```text
1. Discovery Loop: endless, read-only, cheap
2. Delivery Loop: finite, write-capable, one goal/PR at a time
```

The discovery loop can run forever because it only produces ranked candidate
goals. The delivery loop must always terminate:

- one active goal
- one worktree
- one OpenSpec change
- one PR
- one merge or one blocked handoff
- no hidden background mutation

That prevents agent soup.

## Package Types To Add First

Create Rusty IDD-owned packages in this order:

1. `scan` package

   Finds stale artifacts, missing specs, workflow gaps, CI drift, code
   hotspots, orphaned work, toolchain risk.

2. `goal` package

   Converts findings into candidate goals with risk score, blast radius, owner
   boundary, evidence, and suggested OpenSpec slug.

3. `design` package

   Forces architecture reasoning before implementation. Reads ADRs, OpenSpec,
   `.idd/knowledge`, current code, and prior ICM.

4. `implement` package

   The only write-capable package. Must require ready OpenSpec status.

5. `verify` package

   Exhaustive post-task verifier: original request vs goal vs plan vs diff vs
   tests vs graph vs ICM vs PR evidence.

6. `publish` package

   Commit, push, PR, CI wait with useful parallel work, merge, sync, cleanup.

7. `learn` package

   Stores durable ICM lessons, updates knowledge artifacts, feeds the next
   discovery cycle.

## The Self-Upgrade Governor

This is the missing component. Name it something like `crates/self-upgrade` or
`crates/governor`.

It owns:

```text
Queue:
  candidate goals, approved goals, blocked goals, completed goals

Policy:
  what can run automatically
  what requires user approval
  max risk per cycle
  max file blast radius
  max session duration
  max parallel agents

Scoring:
  correctness impact
  workflow friction removed
  compile/test speed impact
  verification quality
  token/context savings
  user-stated priority

State:
  last scan
  last completed PR
  active worktree
  active change
  current package
  verification result
```

## Important Safety Rule

Do not build a true unbounded write loop. Build an endless read/recommend loop
plus a bounded approve/run/publish loop.

Auto-run can be allowed for low-risk categories:

```text
Allowed auto goals:
  stale generated artifact refresh
  docs/spec consistency repair
  missing validation evidence
  narrow CLI package emission
  test fixture repair
  workflow prompt/package scaffolding

Require approval:
  dependency upgrades
  architecture boundary changes
  toolchain changes
  auth/secrets/env behavior
  deletion/removal
  cross-repo mutation
  CI policy changes
```

## Preferred Path

Start with one vertical slice:

```text
Goal: Add Rusty IDD self-upgrade discovery package.

It should:
  1. scan repo truth
  2. produce candidate goals
  3. rank them
  4. write no code by default
  5. emit a `rusty-idd --goal-file ...` ready artifact
  6. route the chosen candidate into the existing OpenSpec flow
```

Then the next goal can be generated by the new system itself: "Promote candidate
goal into OpenSpec scaffolding."

That is the bootstrap moment. After that, Rusty IDD starts feeding itself
clean, scoped goals instead of relying on giant always-loaded harness prompts.

The north star:

```text
Codex asks: "What package do I need for this goal?"
Rusty IDD answers with a scoped package.
The package produces evidence.
The evidence produces the next goal.
The loop continues, but every write is still reviewable, typed, gated, and PR-shaped.
```

That is how Rusty IDD gets full-auto self-upgrade without turning the harness
into a token furnace.

## First Downstream Test Target

After this goal-artifact pass is complete, the first test target must be Rusty
IDD feature integrations and automations:

- What is the real integration between Rusty IDD, handoff, and prompt_hub?
- Where is the autonomous flow?
- How are the handoff kernel, handoff CLI, and prompt_hub CLI integrated?
- What is the directory structure?
- How do other repos initiate, build the proper directory structure, and sync?

This goal file records that target for the next cycle. Do not research or
implement that target until the self-upgrade governor goal artifacts are
created and validated.
