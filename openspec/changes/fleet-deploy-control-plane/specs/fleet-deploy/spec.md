## ADDED Requirements

### Requirement: Deploy the thin-adapter surface into a target repo

Rusty IDD SHALL provide a `rusty-idd deploy` command that installs the
engine-owned thin-adapter control-plane surface into a target repository root
given by `--target <path>`. The deployed surface SHALL be byte-identical to what
`rusty-idd render` produces for the home repo: for each targeted vendor it writes
that vendor's `rusty-idd-adapter.md` (from `render`'s single source of truth) and
a SessionStart hook that runs `rusty-idd next --base <target>`. The command SHALL
reuse the same adapter content and `VENDORS` set as `render` so the two can never
diverge.

#### Scenario: Deploy writes adapters and a SessionStart hook into the target
- **GIVEN** a target repo root containing a `.claude` directory
- **WHEN** the operator runs `rusty-idd deploy --target <repo> --vendor claude`
- **THEN** `<repo>/.claude/rusty-idd-adapter.md` is written with content identical to `rusty-idd render`'s output for that vendor
- **AND** a SessionStart hook is installed in that vendor's config that runs `rusty-idd next --base <repo>`

#### Scenario: Deploy all existing vendor surfaces by default
- **GIVEN** a target repo with `.claude` and `.codex` directories but no `.agents` directory
- **WHEN** the operator runs `rusty-idd deploy --target <repo>`
- **THEN** adapters and hooks are deployed into `.claude` and `.codex`
- **AND** no `.agents` directory is created (only existing vendor surfaces are targeted)

### Requirement: Deploy is additive and never mutates the target runtime

The deploy SHALL be strictly additive with respect to the target repo: it writes
only the vendor adapter documents and the SessionStart hook entry. It SHALL NOT
modify, downgrade, or delete the target repo's forge loop, runtime, build files,
source, or any generated artifact. Existing unrelated hook entries in a vendor's
config SHALL be preserved (the SessionStart entry is merged in, not overwritten
wholesale).

#### Scenario: Existing hooks are preserved
- **GIVEN** a target vendor config that already defines PreToolUse and Stop hooks
- **WHEN** the operator runs `rusty-idd deploy --target <repo>`
- **THEN** the SessionStart entry calling `rusty-idd next` is added
- **AND** the pre-existing PreToolUse and Stop hooks remain unchanged

#### Scenario: Forge loop and runtime are untouched
- **GIVEN** a target repo with its own forge-loop harness and runtime files
- **WHEN** the operator runs `rusty-idd deploy --target <repo>`
- **THEN** only vendor adapter docs and the SessionStart hook entry are created or updated
- **AND** no forge-loop, runtime, build, or generated-artifact file in the target is modified or deleted

### Requirement: Idempotent fail-closed deploy drift gate

`rusty-idd deploy --check` (equivalently `--dry-run`) SHALL compute the expected
deployed surface in memory and compare it to what is on disk in the target,
WITHOUT writing anything. If any targeted adapter or SessionStart hook is missing
or differs from the engine output, the command SHALL report the per-target drift
and exit non-zero. When the target is already in sync it SHALL exit zero and
write nothing. Re-running `deploy` over an already-deployed, unchanged target
SHALL produce byte-identical results (idempotent).

#### Scenario: In-sync target passes the check
- **GIVEN** a target previously deployed with `rusty-idd deploy`
- **WHEN** the operator runs `rusty-idd deploy --target <repo> --check`
- **THEN** the command exits zero and modifies no files

#### Scenario: Missing or drifted surface fails the check
- **GIVEN** a target whose `rusty-idd-adapter.md` was removed or hand-edited
- **WHEN** the operator runs `rusty-idd deploy --target <repo> --check`
- **THEN** the command names the missing or drifted file and exits non-zero

#### Scenario: Deploy is idempotent
- **GIVEN** a target already in sync
- **WHEN** the operator runs `rusty-idd deploy --target <repo>` again
- **THEN** the on-disk adapters and hook are byte-identical to before and the run reports no changes
