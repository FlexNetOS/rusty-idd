# Integration Readiness

- Workspace root: `/home/drdave/Desktop/meta/.worktrees/rusty-idd-prompt-front-door`
- Source plan: `.idd/knowledge/integration-plan.json`
- Source system architecture: `.idd/knowledge/system-architecture.json`
- Change: `integrate-prompt-front-door`
- Capability: `capability:prompt-front-door`

## Owner States

| Owner | Found | Repo | Branch | Dirty | Required Tools |
|---|---|---|---|---:|---|
| `repo:prompt-hub` | true | `prompt_hub` | `main` | true | cargo, git |

## Upstream Inputs

| Source | Kind | Mirror | Required Tools | Runtime Assumptions |
|---|---|---|---|---|
| `github.com/f/prompts.chat` | github-repository | `third_party/upstream/prompts.chat` | git, node, postgres | External upstream mirrors are tracked as source snapshots and are not workspace members by default; prompts.chat package metadata requires Node 24.x; prompts.chat Prisma generation requires DATABASE_URL; diagnostics may use a non-secret temporary PostgreSQL URL |
| `github.com/f/ai-prompt` | github-repository | `third_party/upstream/ai-prompt` | git, node, wordpress | External upstream mirrors are tracked as source snapshots and are not workspace members by default; ai-prompt CI uses Node 20 for its WordPress/Gutenberg plugin diagnostics |

## Tool Requirements

| Tool | Default | Provisioned By | Required By | Evidence |
|---|---:|---|---|---|
| `cargo` | true | parent meta/envctl Rust toolchain | repo:prompt-hub | owner repo exposes Rust package metadata or cargo diagnostics |
| `git` | true | parent meta/envctl managed PATH | github.com/f/ai-prompt, github.com/f/prompts.chat, integration-owner-state | native owner diagnostics include git state checks; upstream adoption pins exact git revisions before consolidation |
| `node` | true | parent meta/envctl managed Node/npm toolchain | github.com/f/ai-prompt, github.com/f/prompts.chat | upstream package metadata exposes npm native diagnostics |
| `postgres` | false | parent meta/envctl managed runtime or explicit external service | github.com/f/prompts.chat | upstream postinstall/build commands require DATABASE_URL for Prisma generation |
| `wordpress` | false | parent meta/envctl managed frontend/tooling surface | github.com/f/ai-prompt | upstream WordPress plugin scripts are native diagnostic surfaces |

## Native Diagnostics

| Command | Owner | Mode | Mutates Repo | Tools |
|---|---|---|---:|---|
| `cd prompt_hub && cargo metadata --locked --format-version 1` | `repo:prompt-hub` | read-only | false | cargo |
| `cd prompt_hub && cargo test --workspace --all-features --locked` | `repo:prompt-hub` | native-build-or-test | false | cargo |
| `cd third_party/upstream/ai-prompt && npm ci` | `github.com/f/ai-prompt` | native-build-or-test | true | node |
| `cd third_party/upstream/ai-prompt && npm run build` | `github.com/f/ai-prompt` | native-build-or-test | false | node |
| `cd third_party/upstream/ai-prompt && npm run lint:css` | `github.com/f/ai-prompt` | native-build-or-test | false | node |
| `cd third_party/upstream/ai-prompt && npm run lint:js` | `github.com/f/ai-prompt` | native-build-or-test | false | node |
| `cd third_party/upstream/prompts.chat && DATABASE_URL="postgresql://test:test@localhost:5432/test" npm ci` | `github.com/f/prompts.chat` | native-build-or-test | true | node |
| `cd third_party/upstream/prompts.chat && DATABASE_URL="postgresql://test:test@localhost:5432/test" npm run lint` | `github.com/f/prompts.chat` | native-build-or-test | false | node |
| `cd third_party/upstream/prompts.chat && DATABASE_URL="postgresql://test:test@localhost:5432/test" npm test` | `github.com/f/prompts.chat` | native-build-or-test | false | node |
| `git -C prompt_hub rev-parse HEAD` | `repo:prompt-hub` | read-only | false | git |
| `git -C prompt_hub status --short --branch` | `repo:prompt-hub` | read-only | false | git |
| `git ls-remote https://github.com/f/ai-prompt.git HEAD` | `github.com/f/ai-prompt` | read-only | false | git |
| `git ls-remote https://github.com/f/prompts.chat.git HEAD` | `github.com/f/prompts.chat` | read-only | false | git |
| `test -f third_party/upstream/ai-prompt/package.json` | `github.com/f/ai-prompt` | read-only | false |  |
| `test -f third_party/upstream/prompts.chat/package.json` | `github.com/f/prompts.chat` | read-only | false |  |

## Runtime Assumptions

- External upstream mirrors are tracked as source snapshots and are not workspace members by default
- ai-prompt CI uses Node 20 for its WordPress/Gutenberg plugin diagnostics
- prompts.chat Prisma generation requires DATABASE_URL; diagnostics may use a non-secret temporary PostgreSQL URL
- prompts.chat package metadata requires Node 24.x

## Feature Gates

- Default Rusty IDD knowledge and planning commands remain read-only
- External upstream servers, MCP transports, and host services stay out of default Rusty IDD workflows unless a later spec explicitly gates them
- Host vault probing, secret relay minting, and long-running service control require an explicit feature or CLI opt-in
- Missing tools are provisioned through parent meta/envctl or tracked repo-local surfaces, not user-global installs
- Peer repo writes and branch changes stay outside default Rusty IDD readiness generation
- ai-prompt WordPress plugin UI remains an upstream prompt rendering surface until mapped through prompt_hub and Rusty IDD DTOs
- prompts.chat MCP/server/web runtime surfaces are adoption evidence only until a prompt-front-door feature boundary enables them

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
- readiness derived 5 tool requirements from 1 owner surfaces
- readiness generation is deterministic and does not execute native diagnostics
- selected integrate-prompt-front-door from .idd/knowledge/integration-plan.json
