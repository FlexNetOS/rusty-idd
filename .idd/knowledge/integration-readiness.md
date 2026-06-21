# Integration Readiness

- Workspace root: `/home/drdave/Desktop/meta/rusty-idd`
- Source plan: `.idd/knowledge/integration-plan.json`
- Source system architecture: `.idd/knowledge/system-architecture.json`
- Change: `integrate-prompt-front-door`
- Capability: `capability:prompt-front-door`

## Owner States

| Owner | Found | Repo | Branch | Dirty | Required Tools |
|---|---|---|---|---:|---|
| `repo:prompt-hub` | true | `prompt_hub` | `main` | true | cargo, git |

## Tool Requirements

| Tool | Default | Provisioned By | Required By | Evidence |
|---|---:|---|---|---|
| `cargo` | true | parent meta/envctl Rust toolchain | repo:prompt-hub | owner repo exposes Rust package metadata or cargo diagnostics |
| `git` | true | parent meta/envctl managed PATH | integration-owner-state | native owner diagnostics include git state checks |

## Native Diagnostics

| Command | Owner | Mode | Mutates Repo | Tools |
|---|---|---|---:|---|
| `cd prompt_hub && cargo metadata --locked --format-version 1` | `repo:prompt-hub` | read-only | false | cargo |
| `cd prompt_hub && cargo test --workspace --all-features --locked` | `repo:prompt-hub` | native-build-or-test | false | cargo |
| `git -C prompt_hub rev-parse HEAD` | `repo:prompt-hub` | read-only | false | git |
| `git -C prompt_hub status --short --branch` | `repo:prompt-hub` | read-only | false | git |

## Runtime Assumptions

- No host runtime probing is required for the default readiness artifact

## Feature Gates

- Default Rusty IDD knowledge and planning commands remain read-only
- Host vault probing, secret relay minting, and long-running service control require an explicit feature or CLI opt-in
- Missing tools are provisioned through parent meta/envctl or tracked repo-local surfaces, not user-global installs
- Peer repo writes and branch changes stay outside default Rusty IDD readiness generation

## Validation

- `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps --locked`
- `affected CLI smoke tests`
- `cargo audit --deny warnings`
- `cargo fmt --all -- --check`
- `cargo run --bin rusty-idd -- spec validate --all`
- `cargo run --bin rusty-idd -- validate --workspace .`
- `cargo test --workspace --all-features --locked`
- `just ci`
- `make ci`
- `rusty-idd knowledge integration-readiness --workspace . --next --out .idd/knowledge/integration-readiness.json`

## Rollback

- Re-run focused owner-repo tests plus Rusty IDD gates
- Re-run rusty-idd knowledge refresh, system-architecture, operating-model, plan-context, and manifest
- Revert the OpenSpec change and generated artifacts for this integration slice
- remove .idd/knowledge/integration-readiness.* and regenerate integration status/owner artifacts

## Findings

- 1 resolved owner repos report dirty state
- all owner repos resolved in the system architecture graph
- joined 1 owner repos against .idd/knowledge/system-architecture.json
- readiness derived 2 tool requirements from 1 owner surfaces
- readiness generation is deterministic and does not execute native diagnostics
- selected integrate-prompt-front-door from .idd/knowledge/integration-plan.json
