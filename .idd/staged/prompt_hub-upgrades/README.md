# Staged: prompt_hub pending upgrades

Curated, actionable staging surface for prompt_hub's **pending** (`backlog`)
upgrade cards, preserved inside rusty-idd's control plane so they can be resumed
**after** the fleet production deployment of the rusty-idd full package.

This is the *resume list* — distinct from the faithful byte-for-byte archive at
`imports/prompt_hub/` (ADR-0018 adopt). The cards here are copied **verbatim**
(intent_lock hashes intact) from the source repo's continuity kernel; they are
intentionally **not started** in rusty-idd yet.

## Provenance

- Source repo: `meta/prompt_hub` (branch `main`, clean).
- Source path: `prompt_hub/.handoff/tasks/PHTASK-*.task.json` (schema
  `handoff.task.v1`).
- Selection: every card with `status: "backlog"` at staging time (2026-06-23) —
  21 of 71 cards. The other 50 are `done` and already reflected in the adopted
  `imports/prompt_hub/` tree.
- These are the deep-audit gap-inventory cards (filed 2026-06-18): completing
  stubbed capabilities, expanding HTTP route coverage, and hygiene/retire
  decisions for hollow feature flags.

## Resume protocol (after fleet deploy)

1. The canonical state stays in prompt_hub's `.handoff/` kernel (and its faithful
   copy under `imports/prompt_hub/.handoff/`). These staged copies are pointers.
2. When resuming, reconcile each card against the then-current adopted code under
   rusty-idd (post-Phase-5 reorganization may relocate `prompthub-server/**`).
3. Verify each card's `intent_lock` before acting; honor its `test_commands`.
4. Cards depending on default-features CI (PHTASK-0064) gate the others.

## Cards (21 backlog)

| Card | Pri | Role | Title |
|------|-----|------|-------|
| PHTASK-0047 | P1 | implementer | Complete garbage_collector real DB purge (reconcile with auto_purge) |
| PHTASK-0049 | P1 | implementer | Complete audit right_to_erasure real GDPR erasure |
| PHTASK-0050 | P2 | implementer | Complete vibe SelfHealer::heal |
| PHTASK-0051 | P2 | implementer | Complete chaos-automation scheduler + real webhook POST |
| PHTASK-0052 | P2 | implementer | Complete offline conflict-resolution (LastWriteWins + Merge) |
| PHTASK-0053 | P2 | implementer | Complete mobile push persistence + real sizing |
| PHTASK-0054 | P2 | implementer | Complete analytics cost-trends real time-series |
| PHTASK-0055 | P2 | implementer | Complete voice STT/TTS backends behind the FSM |
| PHTASK-0056 | P1 | implementer | EPIC — expose remaining ~125 hub capabilities over HTTP |
| PHTASK-0060 | P2 | implementer | Routes: cost-limits + beta + quota + moderation (14) |
| PHTASK-0061 | P2 | implementer | Routes: audit/SOC2/diff + retention/GC + auto-purge (23) |
| PHTASK-0062 | P3 | implementer | Routes: voice + local-llm + analytics + accessibility (16) |
| PHTASK-0063 | P3 | implementer | Routes: sandbox + malware + offline/sync/mobile + chaos + multimodal/i18n + preview + pollination + quality-gate (remaining ~32) |
| PHTASK-0064 | P1 | implementer | CI: add Default-Features Build + Test-Compile to branch-protection required checks |
| PHTASK-0065 | P3 | implementer | Hygiene: retire Python drift (generate_cards.py) + resolve unused libloading dep |
| PHTASK-0066 | P1 | implementer | Complete LockManager TTL sweep (expired locks never deleted) |
| PHTASK-0067 | P2 | implementer | Complete CLI JunieCommand::Task execution (print-only stub) |
| PHTASK-0068 | P3 | implementer | Complete or formally retire the TUI (run_tui is a stub) |
| PHTASK-0069 | P2 | implementer | Wire TLS/mTLS server path or retire the hollow `tls` feature |
| PHTASK-0070 | P3 | implementer | Wire HF tokenizers counting backend or retire the hollow `tokenizers` feature |
| PHTASK-0071 | P3 | implementer | Reconcile SPEC-promised but absent feature flags: sqlcipher + ffi |

Raw cards: `tasks/PHTASK-*.task.json` (verbatim, intent_lock preserved).
