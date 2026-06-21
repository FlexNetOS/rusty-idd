# add-system-architecture-peer-graph

## Why

PR #55 added a repo-local architecture graph, but the Rusty IDD vision is
system-scoped: a task may target one repo while depending on peer repos,
handoff state, parent toolchains, domain/runtime upgrades, and fleet
coordination.

Rusty IDD needs a system architecture graph that discovers the parent meta
workspace and maps peer repositories into the same automation model as local
CodeGraph and repomix artifacts.

## What Changes

- Add a system architecture graph command under `rusty-idd knowledge`.
- Discover peer repositories from a system root, preferring parent meta project
  metadata when available.
- Map repo tags, git state, language/tool markers, and known integration roles
  into deterministic system graph nodes and edges.
- Generate JSON and Markdown outputs suitable for handoff and future automated
  integration planning.

## Capabilities

### New Capabilities

- `system-architecture-peer-graph`: generate a cross-repo system graph that
  relates Rusty IDD to handoff, weave, Obscura, Yazelix, envctl, prompt/meta
  producers, hubs, and other peer repos.

### Modified Capabilities

- `knowledge`: can now emit repo-local architecture graphs and system-wide
  peer architecture graphs.

## Impact

- `crates/knowledge`
- `crates/cli`
- `.agents/skills/rusty-idd-knowledge`
- `.idd/knowledge`
- `/AI_MERGE`
