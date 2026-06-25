## ADDED Requirements

### Requirement: Inventory before flatten

A repo unification SHALL produce the merge-tools inventory artifacts —
RepoInventory, feature matrix, env/secret contract, conflict-risk register, and a
vertical-slice task plan — for each source repo BEFORE any source files are moved
or flattened into the destination tree.

#### Scenario: Inventory precedes any code move
- **GIVEN** a goal to unify a source repo into rusty-idd
- **WHEN** the unification begins
- **THEN** `rusty-idd scan` and `rusty-idd plan` outputs (inventory, feature matrix, env/secret contract, conflict register, task plan) exist as evidence
- **AND** no source file has yet been merged into rusty-idd's `crates/`

### Requirement: handoff is the canonical base

Where rusty-idd and handoff share a divergent file (the 293 shared paths from a
prior poor merge), handoff's implementation SHALL be the canonical base. rusty-idd's
genuine forward additions SHALL be reconciled onto the handoff base upgrade-only —
no capability from either side may be lost, and no working surface may be
downgraded.

#### Scenario: Divergent shared file resolves to handoff plus forward additions
- **GIVEN** a file present in both handoff and rusty-idd with diverged content
- **WHEN** the unification reconciles it
- **THEN** handoff's implementation is the base
- **AND** rusty-idd's forward additions on that surface are merged in, losing neither side's capability

### Requirement: prompt_hub is integrated additively

prompt_hub (independent; only meta-file path overlaps) SHALL be integrated
additively — its crates absorbed without displacing rusty-idd or handoff code,
reconciling only the shared meta files.

#### Scenario: prompt_hub crates land without crate-name collisions
- **GIVEN** prompt_hub's distinct crates
- **WHEN** they are absorbed
- **THEN** they are added without renaming rusty-idd or handoff crates
- **AND** only shared meta files (CI, .gitignore, AGENTS.md) are reconciled

### Requirement: Faithful adoption preserves complete current state

Adoption SHALL bring each source repo's complete current state forward intact,
including its `.kb` knowledge base and `.handoff` witnessed ledgers
(`ledger.db`/`.rvf`). Binaries, databases, and knowledge/ledger state SHALL NOT
be stripped during adoption; any later removal SHALL be a separate, evidence-
backed cut, never part of the import.

#### Scenario: Knowledge base and witnessed ledger survive import
- **GIVEN** a source repo with a `.kb` index and a `.handoff/ledger.db` witnessed ledger
- **WHEN** it is imported
- **THEN** the imported tree contains the `.kb` knowledge base and the `.handoff` ledger files unchanged

### Requirement: Absorbed code is first-class in the code graph

Code absorbed by a unification SHALL be indexed in rusty-idd's code graph / `.kb`
code intelligence as first-class code. The unification SHALL NOT hide absorbed
code from the code graph.

#### Scenario: Imported code appears in the code graph
- **GIVEN** source code imported by a unification
- **WHEN** `rusty-idd knowledge refresh` runs
- **THEN** the imported code is represented in the code graph index, not excluded from it

### Requirement: Behavior preserved through parity-tested vertical slices

A unification SHALL migrate one narrow vertical slice per change, and SHALL prove
old==new behavior with parity tests before deprecating or removing any source
path. Duplicate code is removed only after parity passes.

#### Scenario: A slice cannot dedup before parity passes
- **GIVEN** a vertical slice that reconciles a shared subsystem
- **WHEN** the slice is implemented
- **THEN** parity tests comparing the old and new behavior pass
- **AND** only then are the now-duplicate source paths deprecated and removed
