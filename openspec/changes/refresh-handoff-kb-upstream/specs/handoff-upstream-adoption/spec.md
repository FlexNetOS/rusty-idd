### Requirement: Refresh handoff upstream mirror when committed upstream advances

Rusty IDD SHALL refresh the handoff upstream mirror when the adopted committed
handoff source advances and the user goal requires the complete current handoff
repository.

#### Scenario: Handoff adds tracked KB state

- **GIVEN** `meta/handoff` committed `HEAD` tracks `.kb/*` files
- **WHEN** Rusty IDD refreshes the handoff upstream mirror
- **THEN** `third_party/upstream/handoff` SHALL include the tracked `.kb` files
- **AND** the upstream registry SHALL record the refreshed commit and tracked
  file count
- **AND** source-local dirty working-tree edits SHALL remain excluded.
