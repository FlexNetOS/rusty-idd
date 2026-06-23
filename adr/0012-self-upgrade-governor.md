# ADR 0012: Rusty IDD Self-Upgrade Governor

- **Status:** Accepted
- **Date:** 2026-06-22
- **Change:** `add-self-upgrade-governor`

## Context

Rusty IDD has knowledge artifacts, OpenSpec lifecycle gates, ADRs, task
execution, validation, package concepts, and model harness adapters. The system
still needs a Rusty IDD-owned way to discover the next upgrade, write a bounded
goal, select the right task-scoped package, and feed the next cycle without
turning `.codex`, `.claude`, `.kimi`, or similar directories into giant
always-loaded harnesses.

## Decision

Rusty IDD will add a self-upgrade governor workflow. The governor owns candidate
goal generation, risk review, package routing, bounded delivery, verification,
publishing, and learning loop state. Harness adapters remain thin and delegate
stage capability selection to Rusty IDD packages.

Self-upgrade automation is split into:

- an endless read-only discovery loop that produces ranked candidate goals;
- a finite write-capable delivery loop that handles one approved goal through
  OpenSpec, implementation, verification, PR, merge, and cleanup.

Self-authored work must pass through this typed pipeline before execution:

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

## Consequences

- Rusty IDD can start feeding itself clean scoped goals.
- The always-on harness remains small.
- High-risk changes still require owner approval.
- Every write-capable cycle remains reviewable, typed, gated, and PR-shaped.
- Future implementation should start with read-only discovery and candidate-goal
  output before adding write-capable execution.

## Rollback

Remove the self-upgrade governor goal, OpenSpec change, and future package
implementation. Existing Rusty IDD knowledge, OpenSpec, runner, harness package,
and validation flows continue to work without this governor.
