# plan-handoff-single-repo-architecture - Design

## Context

Rusty IDD and handoff are complementary parts of the same autonomous agent
workflow. Rusty IDD produces intent-bound planning, graph context, OpenSpec
readiness, validation, and evidence. Handoff records witnessed backlog/task
state, claims, checkpoints, done gates, delivery correlation, and fleet pickup.

Keeping them separate preserves independent ownership, but it also forces agents
to reason across two repos whenever a task needs both planning and witnessed
execution. The architecture plan must choose the repository shape that minimizes
that split without flattening boundaries blindly.

## Goals / Non-Goals

**Goals:**

- Use generated Rusty IDD artifacts and direct source scans to compare both repo
  surfaces.
- Decide the target repository shape with evidence.
- Preserve working behavior, task ledger compatibility, release hygiene, and
  existing validation gates.
- Define migration phases and rollback before any code movement.
- Keep the first implementation slice narrow and reviewable.

**Non-Goals:**

- Move `handoff` source code in this planning change.
- Delete, flatten, or rewrite either repository before inventories and contract
  maps exist.
- Start services, daemons, MCP servers, or host process managers.
- Replace the handoff ledger/task-card semantics with Rusty IDD guesses.

## Decision Inputs

- Rusty IDD generated knowledge artifacts and OpenSpec status.
- Handoff crate/workspace structure, CLI/task surfaces, persistence files,
  tests, CI, and dirty-state boundaries.
- Existing `fleet-handoff` and autonomous workflow evidence.
- Current branch topology and release/CI constraints for both repos.

## Candidate Shapes

### Option A: Rusty IDD embedded in handoff

Handoff becomes the outer repository and Rusty IDD becomes an embedded package or
subtree under the handoff workspace.

### Option B: Handoff embedded in Rusty IDD

Rusty IDD becomes the outer repository and handoff becomes an embedded crate or
package under the Rusty IDD workspace.

### Option C: Peer crates/packages in one combined repository

A single repository preserves Rusty IDD and handoff as explicit peers under one
workspace, with one root CI/release/handoff policy and clear internal ownership
boundaries.

## Evaluation Criteria

- Which repo owns the authoritative workflow intent and validation lifecycle?
- Which repo owns task-card persistence, leases, and delivery?
- Which shape reduces cross-repo context rot without hiding boundaries?
- Which shape preserves existing CLI users and CI gates with the least churn?
- Which shape supports a staged migration and rollback?
- Which shape avoids putting heavy or volatile dependencies into Rusty IDD core?

## Expected Output

- ADR selecting the target repository shape.
- AI_MERGE evidence note with inventories, contract maps, risk register,
  migration phases, and rollback.
- Refreshed `.idd/knowledge/*`, `.idd/MANIFEST.tsv`, and plan-context artifacts.
- A first-slice implementation plan that can become a later PR.

## Selected Architecture

Use handoff as the outer canonical repository and embed current Rusty IDD as
explicit peer workspace packages inside handoff.

The selected shape is Option A with a boundary-preserving constraint: Rusty IDD
must remain a modular package family and planning/validation control plane. It
must not be flattened into `hf`, and `hf` must not replace Rusty IDD OpenSpec,
knowledge, ADR, validation, and manifest semantics.

The migration starts by adopting current Rusty IDD into handoff intact enough to
build and diagnose, then cuts only duplicate or stale surfaces after parity
tests prove the combined workspace.
