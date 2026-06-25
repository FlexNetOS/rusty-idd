## ADDED Requirements

### Requirement: Verify stage package delegates post-task verification
Rusty IDD SHALL provide a `verify` harness package stage for exhaustive
post-task verification after implementation work is complete.

#### Scenario: Model adapter invokes verify package
- **GIVEN** a task has completed implementation work
- **WHEN** a user invokes `/verify` or an equivalent model adapter command
- **THEN** the adapter SHALL invoke Rusty IDD verify package generation instead
  of embedding the verification workflow as always-loaded prompt text.

#### Scenario: Verify package receives task context
- **GIVEN** a completed task has an original request, goal, task artifact, and
  plan artifact
- **WHEN** Rusty IDD creates the verify package
- **THEN** the package SHALL bind the target path, goal file, task file, plan
  file, implementation diff, and evidence output schema.

### Requirement: Verify package performs exhaustive cross-verification
The verify package SHALL require cross-verification against the original user
request, goal, task artifacts, plans, diffs, tests, generated artifacts, graph
context, ICM memory, and rollback evidence.

#### Scenario: Verification compares against original intent
- **GIVEN** the original request and task plan are available
- **WHEN** verification runs
- **THEN** the verifier SHALL compare completed work against the original
  request, goal, OpenSpec tasks, and implementation plan before declaring pass.

#### Scenario: Verification inspects implementation evidence
- **GIVEN** a repository has changed files after a task
- **WHEN** verification runs
- **THEN** the verifier SHALL review `git status`, `git diff`, changed-file
  classification, test results, generated-artifact freshness, and validation
  output.

#### Scenario: Verification compares graph and memory context
- **GIVEN** Rusty IDD knowledge artifacts and ICM are available
- **WHEN** verification runs
- **THEN** the verifier SHALL compare relevant graph/context artifacts and ICM
  recall results against the implementation and report any mismatch or stale
  assumption.

### Requirement: Verify package emits typed evidence
The verify package SHALL declare a typed verification evidence schema with
findings, commands, diff summary, tests, graph comparison, ICM comparison,
unresolved questions, pass/fail verdict, and rollback risk.

#### Scenario: Verification reports findings first
- **GIVEN** verification finds blockers, risks, or missing evidence
- **WHEN** the verification report is produced
- **THEN** findings SHALL be listed before summary prose, with concrete file,
  command, or artifact references where possible.

#### Scenario: Verification passes with complete evidence
- **GIVEN** goal comparison, diff review, tests, graph checks, ICM checks, and
  generated-artifact checks are complete
- **WHEN** no blockers remain
- **THEN** the verification report SHALL include a pass verdict, commands run,
  evidence locations, unresolved non-blocking questions, and rollback path.
