# Env/Vault Relay Readiness

## Scope

This slice implements the next integration queue item,
`integrate-env-vault-relay`, without crossing into host/vault execution. The
local Rusty IDD boundary is a deterministic readiness artifact:

- `rusty-idd knowledge integration-readiness --workspace . --next`
- `.idd/knowledge/integration-readiness.json`
- `.idd/knowledge/integration-readiness.md`

The command consumes the generated integration plan and owner surfaces, then
records owner repo state, tool requirements, native diagnostic expectations,
feature gates, runtime assumptions, validation, and rollback.

## Owner Surfaces Verified

Live peer state observed before implementation:

| Owner | Head | Branch | State |
| --- | --- | --- | --- |
| `repo:envctl` | `4a3b8ec71f97f2be56449fd7b489d07f57d0162a` | `master...origin/develop [behind 1]` | clean |
| `repo:vault-hub` | `a358e1b39a65c761ade76508f57c056e480a722c` | `main...origin/main` | clean |
| `repo:yazelix` | `03352e0687ad275459dc261d5355234c46998f0a` | `main...origin/main [behind 197]` | clean |

The first generated readiness draft used stale `.idd/knowledge/system-architecture.*`
state for `envctl`; the final artifact must be generated after the normal
system graph refresh so live peer state is reflected.

## Native Diagnostics

| Repo | Command | Result | Evidence |
| --- | --- | --- | --- |
| `envctl` | `cargo metadata --locked --format-version 1` | pass | Metadata resolved under the owner repo. |
| `envctl` | `cargo test --workspace --all-features --locked` | fail | `crates/secrets-store-libsql/src/lib.rs` emits `compile_error!("select only one of `remote` or `embedded`")`; the owner repo intentionally makes `remote` and `embedded` mutually exclusive. |
| `envctl` | `cargo test --workspace --locked` | pass | 841 passed, 9 ignored, 504.25s. |
| `vault_hub/kasetto` | `just test` | fail | Cargo reports `current package believes it's in a workspace when it's not` because `/home/drdave/Desktop/meta/Cargo.toml` captures `vault_hub/kasetto`; repair belongs in parent workspace or repo-local workspace config, not user-global installs. |
| `yazelix/rust_core` | `cargo metadata --locked --format-version 1` | pass | Metadata resolved after fetching pinned Git inputs. |
| `yazelix/rust_core` | `cargo test --workspace --locked` | fail | One test, `config_set_and_unset_edit_settings_jsonc`, expects line `6` but current behavior reports line `9`; 397 core tests and the other suites shown in output passed. |

## Toolchain / Runtime Requirements

The readiness artifact records:

- default-path tools: `git`, `cargo`, `envctl`
- parent-managed or feature-gated runtime tools: `kasetto`, `nix`, `nushell`,
  `lua`, `ghostty`, `zellij`, `beads`, and `cognitum-vault`
- Cognitum anchors as assumptions only:
  `/run/media/drdave/COGNITUM` and `Cognitum vault on Pi Zero`

No tool was installed into user-global state. Missing tools or broken native
commands are routed to parent `meta`/`envctl` or owner-repo repairs.

## Consolidation / Cuts

No upstream feature was downgraded or removed. The local cut is a boundary cut:
Rusty IDD records readiness and toolchain policy, but does not probe Cognitum,
mint relay credentials, mutate peer repos, start services, or manage daemon
state from default workflows.

Rollback:

1. Remove `integration-readiness` source, CLI, Just/Make targets, and generated
   `.idd/knowledge/integration-readiness.*`.
2. Re-run `just knowledge`, `just system-architecture`, `just operating-model`,
   `just integration-plan`, `just integration-status`, `just integration-owners`,
   `just plan-context`, and `just manifest`.
3. Re-run focused CLI tests plus full Rusty IDD gates.

## Verification

Focused checks completed before full artifact refresh:

```bash
cargo test -p rusty-idd-knowledge integration_owner_surfaces_join_work_item_to_system_repos --locked
cargo test -p rusty-idd-cli knowledge_commands_cover_index_pack_report_query_and_refresh --locked
cargo run --bin rusty-idd -- knowledge integration-readiness --workspace . --next --out /tmp/rusty-idd-integration-readiness.json
cargo run --bin rusty-idd -- knowledge integration-readiness --workspace . --next --out /tmp/rusty-idd-integration-readiness.md
```
