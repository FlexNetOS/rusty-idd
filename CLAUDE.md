# Claude Bridge

Claude-specific files in this repository are compatibility notes only. The
authoritative workflow is Rusty IDD:

1. read `AGENTS.md`;
2. read or refresh `.idd/knowledge/*`;
3. bind the goal with OpenSpec artifacts;
4. use `rusty-idd spec status` before implementation;
5. use `rusty-idd merge-tools show` for merge, migration, or repository
   unification goals;
6. validate with the repo gates and record evidence only where Rusty IDD calls
   for it.

Do not resurrect the retired `idd-merge-idd` Claude loop as the control plane.
`AI_MERGE/` is evidence/history, not the source of intent.
