# Handoff Source State

This records source checkout state at import time. The mirror imports the
tracked commit only and excludes Git metadata.

## Status

```text
## develop...origin/develop
```

## Source Commit

```text
7be85fcea3c2454fc3470fc929860afb7ea9864b
```

## Diff Stat

```text
no uncommitted diff
```

## Import Policy

The mirror imports the tracked source commit with `git archive HEAD`. The source
checkout was clean at import time, so the mirror includes the complete tracked
handoff repository, including tracked dot-directory material, and excludes only
Git metadata.
