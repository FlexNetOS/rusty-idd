# add-self-upgrade-governor - Tasks

## 1. Artifact Flow

- [x] 1.1 Create the goal file from the approved brainstorm.
- [x] 1.2 Include the requested `rusty-idd --goal-file` step at the top of the
  goal file.
- [x] 1.3 Create the OpenSpec proposal.
- [x] 1.4 Create the design document.
- [x] 1.5 Create spec deltas for self-upgrade governor and harness workflow.
- [x] 1.6 Create the ADR.
- [x] 1.7 Record task and validation evidence.

## 2. Future Implementation

- [ ] 2.1 Add the first `rusty-idd self-upgrade` CLI surface.
- [ ] 2.2 Implement read-only discovery loop output for candidate goals.
- [ ] 2.3 Implement typed candidate-goal review data.
- [ ] 2.4 Implement package selection for scan, goal, design, implement,
  verify, publish, and learn stages.
- [ ] 2.5 Add tests proving discovery is read-only and delivery is finite.
- [ ] 2.6 Use the first downstream test target to evaluate Rusty IDD feature
  integrations and automations after the governor package exists.

## 3. Validation

- [x] 3.1 Refresh `.idd/knowledge/*`.
- [x] 3.2 Verify OpenSpec status for this change.
- [x] 3.3 Run Rusty IDD validation.
- [x] 3.4 Refresh `.idd/MANIFEST.tsv`.
- [x] 3.5 Commit, push, and open a PR against `develop`.
