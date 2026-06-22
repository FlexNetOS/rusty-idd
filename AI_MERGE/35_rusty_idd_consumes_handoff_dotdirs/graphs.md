# Architecture Graphs

These graphs are planning evidence for Rusty IDD consuming `meta/handoff` whole
while governing the growing dot-directory surface.

## Dot-Directory Ownership

```mermaid
flowchart TB
    intent["User intent"] --> idd[".idd canonical control plane"]
    idd --> openspec["OpenSpec change"]
    openspec --> adr["adr accepted decisions"]
    idd --> knowledge[".idd/knowledge + plan-context"]
    idd --> manifest[".idd/MANIFEST.tsv + validation"]

    kb[".kb workspace knowledge"] -. source input .-> idd
    idea[".idea ideas/editor state"] -. promote before work .-> idd
    handoff[".handoff adopted runtime evidence"] -. adapter input .-> idd
    claude[".claude + harness_hub traces"] -. compatibility source .-> idd

    codex[".codex hooks/rules"] --> validation["workflow checks"]
    agents[".agents skills"] --> validation
    github[".github CI/PR gates"] --> validation
    validation --> manifest

    cache["local caches, locks, binary ledgers"] -. no authority .-> quarantine["ignored or regenerated"]
```

## Intent To Evidence Lifecycle

```mermaid
sequenceDiagram
    participant Owner as Owner intent
    participant IDD as Rusty IDD .idd
    participant Spec as OpenSpec
    participant ADR as ADR
    participant HF as Handoff adapter
    participant Val as Validation
    participant Git as PR/merge

    Owner->>IDD: rusty-idd --goal-file
    IDD->>IDD: refresh knowledge and plan-context
    IDD->>Spec: create proposal, design, spec, tasks
    Spec->>ADR: record accepted/superseding decision
    ADR->>HF: bind adopted handoff runtime semantics
    HF->>Val: produce task, ledger, delivery, fleet evidence
    Spec->>Val: validate readiness and manifest
    Val->>Git: publish PR to develop with evidence
```

## Handoff Adoption And Migration

```mermaid
flowchart LR
    scan["Deep scan meta/handoff"] --> mirror["Adopt full tracked handoff surface"]
    mirror --> contracts["Map contracts: hf, ledger, work-order, .handoff"]
    contracts --> adapters["Rusty IDD typed adapters"]
    adapters --> parity["Parity tests and evidence"]
    parity --> normalize["Dot-directory normalization validators"]
    normalize --> retire["Retire legacy duplicates after proof"]

    mirror -. excludes .-> excluded[".git, local locks, untracked runtime files, binary cache state"]
    parity -. blocks .-> cuts["behavior cuts or deletions"]
```

## Compatibility And Retirement

```mermaid
stateDiagram-v2
    [*] --> LegacyObserved
    LegacyObserved: .claude, harness_hub, .handoff/loop traces observed
    LegacyObserved --> CompatibilityMapped: behavior and risk mapped
    CompatibilityMapped --> AdapterOwned: Rusty IDD adapter owns behavior
    AdapterOwned --> ParityProven: tests and evidence pass
    ParityProven --> Frozen: legacy trace frozen as historical evidence
    ParityProven --> Retired: duplicate surface retired
    CompatibilityMapped --> LegacyObserved: missing evidence or conflict
    Retired --> [*]
    Frozen --> [*]
```

## State Precedence

```mermaid
flowchart TD
    p1["1 Git-tracked Rusty IDD source + canonical planning artifacts"]
    p2["2 .idd goals, knowledge, plan-context, manifest, validation"]
    p3["3 OpenSpec + ADR"]
    p4["4 adopted handoff task/ledger/packet/delivery/fleet evidence"]
    p5["5 .kb planning and backlog input"]
    p6["6 .idea concepts and editor input"]
    p7["7 .claude, harness_hub, .handoff/loop compatibility traces"]
    p8["8 caches, binary state, locks, relay prose"]

    p1 --> p2 --> p3 --> p4 --> p5 --> p6 --> p7 --> p8
```

## Target Repository Layout

```mermaid
flowchart TB
    repo["meta/rusty-idd"]
    repo --> crates["crates/"]
    repo --> idd[".idd/"]
    repo --> openspec["openspec/"]
    repo --> adr["adr/"]
    repo --> ai["AI_MERGE/"]
    repo --> docs["docs/rusty-idd/"]
    repo --> codex[".codex/"]
    repo --> agents[".agents/"]
    repo --> github[".github/"]
    repo --> upstream["third_party/upstream/handoff or equivalent adoption source"]
    repo --> adapter["future crates/handoff-adapter boundary"]

    upstream --> hf["hf"]
    upstream --> ledger["ledger"]
    upstream --> work_order["work-order"]
    upstream --> handoff_dot[".handoff durable text evidence"]
    adapter --> task["task-card adapter"]
    adapter --> ledger_adapter["ledger JSONL/event adapter"]
    adapter --> fleet["fleet/delivery/policy adapters"]
```

## Validation Flow

```mermaid
flowchart LR
    artifacts["ADR + OpenSpec + docs + AI_MERGE graphs"] --> knowledge["rusty-idd knowledge refresh"]
    knowledge --> plan["rusty-idd knowledge plan-context"]
    plan --> manifest["rusty-idd manifest"]
    manifest --> spec["rusty-idd spec validate/status"]
    spec --> gates["workflow checks + local CI"]
    gates --> pr["PR to develop"]
    pr --> sync["develop to main sync"]
```
