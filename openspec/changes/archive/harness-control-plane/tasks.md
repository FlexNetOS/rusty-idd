# harness-control-plane — Tasks

## 1. Front-door oracle (this slice)

- [x] 1.1 Add `next_artifact_id` reusable helper to `commands::spec_status`
- [x] 1.2 Add `rusty-idd next` front-door command (`commands::next`) reusing the spec DAG oracle
- [x] 1.3 Wire `next` into the CLI enum, dispatch, module tree, and lib docs
- [x] 1.4 Resolve active change from `.idd/workflow/active-change` (Git-tracked text)
- [x] 1.5 Integration tests: no-active-change, routes-to-imperative, dangling-pointer
- [x] 1.6 Unit test: `resolve_active_change` trims/filters empty

## 2. Design + decision record (this slice)

- [x] 2.1 Generate as-is architecture graphs via `knowledge architecture`/`diagrams` (committed under `assets/`)
- [x] 2.2 Author to-be graphs (inversion, determinism loop, control-plane layers)
- [x] 2.3 Author `design.md`, capability spec, and proposal
- [ ] 2.4 Author ADR-0015 unifying ADR-0001 / ADR-0002 / ADR-0010

## 3. Verification gates (this slice)

- [ ] 3.1 `rusty-idd spec validate --all` passes
- [ ] 3.2 `cargo test --workspace` passes (incl. `next_cli`)
- [ ] 3.3 `cargo fmt --all --check` + `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] 3.4 Refresh `.idd/knowledge/*` + `.idd/MANIFEST.tsv`
- [ ] 3.5 Live `rusty-idd next` drives the change to archivable

## 4. Follow-up slices (tracked, not in this change)

- [ ] 4.1 `rusty-idd render <vendor>` + `render --check` drift gate (enforce ADR-0010 thin adapters)
- [ ] 4.2 Wire vendor hooks (`.claude`/`.codex`) to call `rusty-idd next` as the session front door
- [ ] 4.3 Additional stage packages (implementation / validation / handoff swarms) per ADR-0010
- [ ] 4.4 `rusty-idd next --json` for non-interactive adapters
- [ ] 4.5 Reconcile the duplicate ADR-0002 numbering
