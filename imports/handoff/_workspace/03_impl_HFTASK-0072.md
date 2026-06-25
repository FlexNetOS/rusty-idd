# HFTASK-0072 — Implementation (ADR-0018 D7: full `.kb` adoption + two-way seam)

Branch: `feat/hftask-0072-full-kb-adoption` · Scope: `handoff/**` only.

## What changed

### 1. Initialized handoff's OWN durable `.kb` (`git kb init`)
handoff was code-intelligence-only (`.git/gitkb/code.db`); it now has a full git-kb `.kb/`
with the **seven context documents** mandated by `meta/.kb/AGENTS.md`, seeded with handoff's
real identity (Continuity Ledger Kernel), concise — not padded:

- `context/immutable/project-brief` — vision, Integrity·Reversibility·Capability-Gain law, no-C / fail-closed / scope constraints
- `context/immutable/patterns` — witnessed ledger, two-plane seam (ADR-0003), two-ledger residency, text-vs-binary durability
- `context/immutable/architecture` — the `hf`/`ledger`/`work-order` workspace, the data-flow diagram, the `kb.rs` seam module
- `context/extensible/product` — problem (context resets → repo as memory), users (the loop / fleet / owner), principles
- `context/extensible/tech` — Rust + redb (no-C) + ruvector-verified + git-kb stack, CI gate, develop-base merge flow
- `context/overridable/active` — current focus = HFTASK-0072; authoritative state is `hf resume`, not this doc
- `context/overridable/progress` — status, blockers (none), remaining ADR-0018 work

### 2. Residency decision — text-vs-binary (the HFTASK-0067 precedent, EXACTLY)
`git kb init` writes durable text into `.kb/store/**` (markdown documents + JSON
commits/refs/manifest — verified all `ASCII/JSON text`) and a binary rebuild cache into
`.kb/.cache/gitkb.db*` (sqlite). This mirrors `.handoff/ledger.events.jsonl` (committed) vs
`.handoff/**/ledger.db` (gitignored).

**`git kb init` WRONGLY gitignores `.kb/store/` by default** (it would drop the durable
continuity text). I **overrode** that in `.gitignore`. Exact entries written
(replacing the wrong `# GitKB` block):

```gitignore
# === .kb durability policy (ADR-0003 / ADR-0018 D7 — HFTASK-0072) ===
# COMMIT the durable TEXT (.kb/store/** ...); ignore ONLY the binary DB cache + ephemeral
# editing surface + per-user config. git kb init wrongly ignores .kb/store/ — we OVERRIDE.
.kb/.cache/
.kb/workspace/
.kb/workspaces/
.kb/config.toml
```

- `.kb/store/**` → **committed** (durable planning text; analogous to `ledger.events.jsonl`)
- `.kb/.cache/` → ignored (binary `gitkb.db*` rebuild cache, re-derived via `git kb code index`)
- `.kb/workspaces/` → ignored (ephemeral checked-out editing surface, rebuilt from store)
- `.kb/config.toml` → ignored (per-user; carries the local `default_branch` — churny)

Verified: `git ls-files --others --exclude-standard .kb/` lists ONLY text (`.md`/`.json`/`AGENTS.md`);
NO `.db`, no `.cache`, no `workspaces/`, no `config.toml`. `git check-ignore .kb/.cache/gitkb.db` → ignored.

### 3. Loop discipline wired (create-first + board + traceability)
- `.claude/rules/knowledge-management.md` — new "handoff's local `.kb`" section: KB-state detection,
  create-first discipline, residency, and the two-way seam (with ADR-0003 one-way authority).
- `AGENTS.md` — new "Knowledge base — the planning plane (`.kb`)" section + navigation-order entry 6
  (`git kb checkout --path context/` · `git kb board`).

### 4. Seam reconciled BOTH ways against the LOCAL `.kb` — no downgrade (`hf/src/kb.rs`)
The pre-existing seam only saw the **meta-root** `.kb` (`<repo>/../.kb`); it could not see
handoff's new local `.kb`. Fixed fail-closed, additive:

- `kb_root` now resolves **local-first** (`<repo>/.kb`) → then meta-root `.kb` (the original
  FLEET fallback, unchanged) → `None`. Made `pub` for testability. (2 callers, both in `kb.rs`.)
- `mint_target` is now **plane-aware**: a slug minted from the LOCAL kb lands in the repo's own
  `.handoff/tasks/` (`[LOCAL]`); a META-kb slug still routes to FLEET (preserving the
  anti-contamination invariant). Signature `mint_target(local_kb: bool, meta_root)`.
- `cmd_mint_from_kb` threads `local_kb = (kb_root == repo_root)` and updates the no-kb message.
- The OUT write-back (`write_back`/`KbTransition`) needed no change — it already resolves via
  `kb_root`, so it now binds to the local `.kb` automatically. Still ONE-WAY (ADR-0003): kb is
  never read back as execution truth.

### 5. Tests + seed
- `hf/src/main.rs` `cmd_seed`: added the HFTASK-0072 `test_commands` arm (additive; the
  `mk("HFTASK-0072", …)` seed objective is byte-untouched) — asserts the `.kb` text store +
  context docs exist, the binary cache is gitignored, and the `task mint` seam verb is exposed.
- `hf/src/kb.rs`: updated the 3 existing `mint_target` callers to the 2-arg signature; added
  `mint_target_is_local_when_slug_came_from_repo_kb` and `kb_root_prefers_local_kb_then_meta_then_none`.

## Seam-both-ways live evidence (driven against handoff's local `.kb`)
```
# IN: hf task mint --from-kb tasks/seam-probe
  → KBTASK-SEAM-PROBE minted, wrote card to .handoff/tasks [LOCAL]   (local-kb routing correct)
# OUT: hf claim KBTASK-SEAM-PROBE
  → "hf claim: kb tasks/seam-probe → active (write-back)"            (kb draft → active)
# OUT: hf release KBTASK-SEAM-PROBE
  → "hf release: kb tasks/seam-probe → backlog (write-back)"         (kb active → backlog)
```
(The seam-probe card + kb task were removed afterward — verification artifacts only.)

## Files / dirs changed
- `.kb/` (NEW) — durable text store: `.kb/AGENTS.md`, `.kb/store/documents/context/**` (7 docs),
  `.kb/store/{commits,refs,manifest.json}`. Binary cache `.kb/.cache/` + `workspaces/` gitignored.
- `.gitignore` — `.kb` text-vs-binary residency block (overrides git-kb's wrong default).
- `AGENTS.md` — planning-plane section + nav entry.
- `.claude/rules/knowledge-management.md` — local `.kb` + seam discipline.
- `hf/src/kb.rs` — local-first `kb_root`, plane-aware `mint_target`, updated/added tests.
- `hf/src/main.rs` — HFTASK-0072 `test_commands` seed arm.

## Scope / no-downgrade
- Diff strictly within `handoff/**`. `meta/.kb` untouched (read-only reference).
- ADR-0003 one-way authority preserved (kb never overrides execution truth). The meta-root
  FLEET seam path is unchanged (the local path is additive). `Cargo.lock` not touched.
