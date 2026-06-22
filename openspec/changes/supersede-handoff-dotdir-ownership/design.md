# supersede-handoff-dotdir-ownership - Design

## Context

Rusty IDD already owns goal intake, graph-backed context, OpenSpec, ADRs,
validation, manifest, and workflow gates. Handoff owns useful task-card, claim,
checkpoint, done, delivery, fleet, and ledger mechanics, but its repository also
carries a `.handoff` runtime directory that was created by a harness loop and
inherits from `.claude`/`meta/harness_hub` history.

The architectural question is therefore not merely "which repo is outer?" It is
"which control plane owns intent, state, evidence, and compatibility when dot
directories multiply?" The answer must avoid allowing `.idd`, `.handoff`, `.kb`,
`.idea`, `.claude`, `.codex`, `.agents`, and `.github` to become peer sources of
truth for the same workflow event.

## Goals

- Make Rusty IDD the canonical product and control plane for the combined
  workflow.
- Consume `meta/handoff` whole through adopt-first migration, preserving its
  useful semantics before cutting duplicates.
- Classify dot directories by role, authority, mutability, retention, and
  migration path.
- Preserve compatibility with legacy `.handoff` and harness traces without
  letting them own current intent.
- Produce visual graphs that agents can use before implementation.

## Non-Goals

- No code movement from `meta/handoff` in this planning slice.
- No deletion of `.handoff`, `.kb`, `.idea`, `.claude`, `.codex`, `.agents`, or
  `.github`.
- No replacement of handoff ledger semantics with handwritten notes.
- No host daemon, service, or user-global tool installation.

## Dot-Directory Role Model

| Directory | Role | Authority | Target Treatment |
|---|---|---|---|
| `.idd/` | canonical Rusty IDD control plane | goal, knowledge, plan context, manifest, validation evidence | keep canonical in Rusty IDD |
| `.handoff/` | adopted witnessed runtime/evidence surface | task cards, claims, checkpoints, delivery, ledger compatibility | consume as Rusty IDD runtime adapter input, then normalize under Rusty IDD policy |
| `.kb/` | workspace knowledge/backlog source | task/spec/note source documents and KB sync | keep as upstream workspace source, reference from Rusty IDD tasks |
| `.idea/` | idea/design workspace | pre-implementation concept capture | preserve as non-authoritative idea intake; promote through `.idd` before implementation |
| `.claude/` | legacy harness and Claude-specific behavior | compatibility/source material only | mirror or import useful behavior; do not make canonical |
| `.codex/` | Codex agent policy and hooks | Rusty IDD workflow enforcement for Codex | keep repo-local enforcement surface generated/validated by Rusty IDD |
| `.agents/` | reusable skills and agent workflows | skill source material | keep as reusable workflow library referenced by Rusty IDD |
| `.github/` | CI and PR policy | remote validation and branch protection | keep as delivery gate, not planning source |
| editor/cache dot dirs | local tool state | none | ignore or keep tool-owned; never use as workflow truth |

## Architecture Decisions

1. Rusty IDD owns canonical workflow state through `.idd`, OpenSpec, ADR, and
   generated knowledge artifacts.
2. Handoff is adopted into Rusty IDD as source and runtime capability, not as
   the outer repository.
3. `.handoff` is treated as witnessed compatibility/evidence input during
   adoption. Its durable semantics must be represented by typed Rusty IDD
   adapters before any cleanup.
4. `.kb` remains a parent/workspace knowledge source. Rusty IDD can mint or bind
   tasks from it, but `.kb` does not replace `.idd` goal/OpenSpec readiness.
5. `.idea` remains a low-friction concept intake surface. Ideas must graduate to
   `.idd/goals`, OpenSpec, and ADR before implementation.
6. `.claude`, `meta/harness_hub`, and harness-loop traces are source material
   for compatibility only. Useful behavior is adopted, tested, and owned by
   Rusty IDD before old harness surfaces are retired.

## Visual Graph Set

The planning evidence package must include at least these graphs:

- Dot-directory ownership graph.
- Intent-to-evidence lifecycle graph.
- Handoff adoption and migration graph.
- Compatibility and retirement graph.
- Repository layout graph for Rusty IDD consuming handoff whole.

## Migration Plan

1. Planning package: this change.
2. Adopt-first inventory: import or mirror `meta/handoff` source and `.handoff`
   semantics into Rusty IDD evidence without behavior cuts.
3. Adapter slice: create typed Rusty IDD crates/modules for handoff task cards,
   claims, checkpoints, done, delivery, fleet state, and ledger compatibility.
4. Dot-directory normalization: define generated manifests and validators that
   prove which dot directory owns each state class.
5. Compatibility retirement: only after parity, freeze or retire legacy harness
   traces that are replaced by Rusty IDD-owned behavior.

## Rollback

Revert this planning package. No handoff code is moved by this change, so
rollback does not require data migration. The prior ADR remains historically
present but superseded by the new ADR.
