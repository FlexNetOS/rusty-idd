# refresh-handoff-kb-upstream - Design

## Approach

Use `git archive` from `/home/drdave/Desktop/meta/handoff` at committed `HEAD`
to replace the Rusty IDD upstream mirror. This preserves committed tracked
source exactly and excludes `.git` plus dirty working-tree edits.

## Verification

Compare:

```bash
git -C /home/drdave/Desktop/meta/handoff ls-tree -r --name-only HEAD
find third_party/upstream/handoff \( -type f -o -type l \) -printf "%P\n"
```

Both sorted lists must contain 550 paths and have an empty diff.

## Non-Goals

- Do not import handoff's uncommitted `.handoff` working-tree edits.
- Do not add handoff crates to the Rusty IDD Cargo workspace.
- Do not refactor handoff behavior in this refresh slice.
