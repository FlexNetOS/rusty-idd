### Requirement: Self-upgrade governor routes automation through bounded goals
Rusty IDD SHALL provide a self-upgrade governor workflow that turns repository
evidence into bounded candidate goals before any write-capable execution.

#### Scenario: Discovery loop proposes candidate goals
- **GIVEN** Rusty IDD has current repository knowledge and workflow evidence
- **WHEN** the self-upgrade discovery loop scans for upgrade opportunities
- **THEN** it SHALL produce candidate goals with evidence, risk, blast radius,
  owner boundary, and suggested OpenSpec change identity.

#### Scenario: Delivery loop handles one approved goal
- **GIVEN** a candidate goal has been approved for execution
- **WHEN** Rusty IDD starts the delivery loop
- **THEN** the delivery loop SHALL bind one goal file, one worktree, one active
  OpenSpec change, one package selection, and one PR-shaped completion path.

### Requirement: Self-authored goals use a typed review pipeline
Rusty IDD SHALL prevent arbitrary model-authored work from entering execution
until it passes through a typed goal review pipeline.

#### Scenario: Finding becomes approved goal
- **GIVEN** a repository finding suggests a possible upgrade
- **WHEN** Rusty IDD prepares self-authored work
- **THEN** it SHALL represent the work as Finding, Opportunity, Hypothesis,
  CandidateGoal, GoalReview, ApprovedGoal, OpenSpecChange, and Package before
  implementation.

#### Scenario: High-risk candidate requires owner approval
- **GIVEN** a candidate goal changes dependencies, architecture boundaries,
  toolchains, auth or secrets behavior, deletion policy, cross-repo mutation, or
  CI policy
- **WHEN** Rusty IDD reviews the candidate goal
- **THEN** it SHALL require explicit owner approval before write-capable
  execution.

### Requirement: Harness adapters stay minimal
Rusty IDD SHALL keep always-on model harness surfaces minimal by selecting
task-scoped packages for the current stage and target.

#### Scenario: Model asks for task capability
- **GIVEN** a model adapter needs tools, skills, helpers, hooks, validation
  gates, or evidence schema for a workflow stage
- **WHEN** the adapter enters that stage
- **THEN** it SHALL ask Rusty IDD for the scoped package instead of loading a
  broad always-on harness surface.

#### Scenario: Package produces next evidence
- **GIVEN** a selected package completes its stage
- **WHEN** Rusty IDD records the stage result
- **THEN** the package SHALL emit typed evidence that can feed verification,
  publishing, learning, or the next candidate goal.
