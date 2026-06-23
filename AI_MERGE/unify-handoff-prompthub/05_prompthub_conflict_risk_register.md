# Conflict Risk Register

| Risk | Evidence | Mitigation |
|---|---|---|
| Path collisions | 18 identical relative paths | Keep imports isolated; migrate into canonical modules one PR at a time |
| Secret/config drift | Secret/env references found | Define one SecretProvider interface and one env resolution order |
| CI drift | Workflow files detected | Merge CI by job intent, not by copying both workflow files blindly |
| Agent conflict | Parallel agent branches possible | Use `.idd/LOCK.md` and `AI_MERGE/08_agent_queue.md`; only integration branch has merge authority |
