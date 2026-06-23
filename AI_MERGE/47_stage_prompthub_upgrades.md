# 47 — Phase 3: stage prompt_hub pending upgrades + preserve stashes

Evidence note for `feat/stage-prompthub-upgrades`. Phase 3 of the fleet-deploy
goal: complete pending changes by **preserving** prompt_hub's not-yet-done work
so it can be resumed after the fleet production deployment — nothing lost, nothing
prematurely implemented.

## What this slice does

1. **Stages prompt_hub's 21 backlog upgrade cards** into rusty-idd's control
   plane at `.idd/staged/prompt_hub-upgrades/` — copied **verbatim** (intent_lock
   hashes intact) from `meta/prompt_hub/.handoff/tasks/PHTASK-*.task.json`
   (schema `handoff.task.v1`). Selection = every card with `status: "backlog"`
   at staging time (21 of 71; the other 50 are `done`). A `README.md` records
   provenance, the resume protocol, and the full card table.

   These are the deep-audit gap-inventory cards (filed 2026-06-18): finish
   stubbed capabilities (GC purge, GDPR erasure, self-healer, voice STT/TTS, …),
   expand HTTP route coverage (~125 hub capabilities), and hygiene/retire
   decisions for hollow feature flags (`tls`, `tokenizers`, `sqlcipher`/`ffi`).

   This is the *resume list* — distinct from the faithful byte-for-byte archive
   already under `imports/prompt_hub/.handoff/` (ADR-0018). It is intentionally
   not started in rusty-idd; PHTASK-0064 (default-features CI) gates the rest.

2. **Preserves prompt_hub's 2 stashes as remote branches** (never-delete):
   - `preserve/stash-0-unused-path-var` → `1834bb8` (WIP on
     `work/PHTASK-0048-onnx-real-inference`: "fix: prefix unused path variable").
   - `preserve/stash-1-wip-main-rs` → `5e5c8f7` (on `fix/handlebars-no-html-escape`:
     "wip-main-rs-unrelated").

   Created with `git branch <name> stash@{n}` (a stash is a commit, so this
   captures the full WIP tree — base + index + untracked) and pushed with
   `--no-verify` (WIP states are deliberately incomplete; preservation, not a
   mergeable contribution). The original stash entries remain intact in the repo.

## Why now (preservation before retirement)

Phase 6 retires the standalone prompt_hub repo (archive + unregister + compress).
Before any retirement, every piece of in-flight work must be durably preserved:
the backlog cards become a discoverable resume list inside the surviving
rusty-idd control plane, and the loose stashes become named, pushed refs. This
satisfies the never-delete-backups rule and the owner directive to "complete any
pending changes / stage prompt_hub upgrades to resume after fleet deployment."

## Verification evidence

- `rusty-idd validate --workspace .` — 0 critical, 0 warning (refresh-last).
- `rusty-idd manifest` — STABLE; `merge-tools verify` — passed (59 manifests,
  53 src trees).
- knowledge refreshed; 0 `.worktrees` contamination.
- prompt_hub preserve branches pushed; `git stash list` still shows both stashes.

## Not in this slice

Implementing any PHTASK card (deferred to post-deploy resume), Phase 4 fleet
deploy, Phase 5 reorganization, Phase 6 retirement.
