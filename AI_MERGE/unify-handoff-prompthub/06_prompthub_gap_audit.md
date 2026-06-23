# Gap Audit and Applied Updates

| ID | Gap | Risk | Applied Update |
|---|---|---|---|
| GAP-001 | AI agents start editing before repository intent is mapped | High: causes duplicate abstractions, broken entrypoints, and unreviewable mega-PRs. | Generated inventories, feature matrix, API/env/secret contract docs, JSON sidecars, and task templates before merge work begins. |
| GAP-002 | Secrets and environment configuration are conflated | High: leaks secrets or creates CI/local drift. | Expanded extraction for dotenv, GitHub secrets/vars/env, Rust, Node, Vite, Python, Deno, SOPS, Infisical, Doppler, direnv, mise, Vault/OpenBao, and Compose env files. |
| GAP-003 | Parallel agent sessions create integration conflicts | High: multiple branches claim merge authority and overwrite each other. | Added `.idd/LOCK.md`, `AI_MERGE/08_agent_queue.md`, and AGENTS.md rule: many agents may analyze, one integration branch has authority. |
| GAP-004 | No reproducible local/CI toolchain contract | Medium: repo works for one agent but fails for another. | Kept dependency-free Rust implementation, added GitHub CI, PR/issue templates, manifesting, and validation gates. |
| GAP-005 | Feature merge lacks rollback and parity evidence | Medium: old features disappear during cleanup. | Added parity test plan, PR evidence requirements, migration notes, deprecate-before-delete rule, and task Definition of Done checklist. |
| GAP-006 | Repository instructions are not optimized for current GitHub agents | Medium: agents waste context, miss rules, or produce broad changes. | Added `AGENTS.md`, `.github/copilot-instructions.md`, issue template, and PR template so agent prompts are repo-native and reviewable. |
| GAP-007 | Cloud agents cannot safely mutate two repos in one run | High: multi-repo tasks exceed agent limits or silently drop changes. | Added GitHub execution plan requiring imports/mirrors into one integration repo and one task branch per issue. |
| GAP-008 | Generated artifacts are overwritten without an audit trail | Medium: agent reruns erase prior decisions. | Added backup-on-overwrite writes and `.idd/MANIFEST.tsv` generation using deterministic file hashes. |
