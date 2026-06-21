# add-system-architecture-peer-graph - Design

## Context

The current architecture graph maps one Rusty IDD checkout. That is not enough
for the FlexNetOS workflow because Rusty IDD is used per repo while the actual
goal often spans the parent meta workspace. The system includes peer repos for
handoff, environment/toolchain management, prompt/spec production, MCP/domain
coordination, and terminal/parser runtime support.

## Goals / Non-Goals

**Goals:**

- Generate a deterministic system graph from a parent workspace root.
- Prefer `meta project list --json` when the meta CLI is available.
- Fall back to filesystem and git discovery when meta metadata is unavailable.
- Include repo identity, tags, git state, markers, integration roles, and
  system edges.
- Keep the implementation inside `crates/knowledge` and CLI layers.

**Non-Goals:**

- Starting MCP servers, daemon processes, or host services.
- Mutating peer repos.
- Requiring every peer repo to have a local `.idd/knowledge` graph before it
  can be represented.
- Adding global tools outside parent-managed surfaces.

## Decisions

- Add a `system-architecture` command instead of overloading repo-local
  `architecture`.
- Keep system graph generation read-only.
- Use meta project metadata as a discovered input, not as a hard dependency.
- Classify known integration repos by deterministic repo name/tag rules:
  Rusty IDD, handoff, weave, Obscura, Yazelix, envctl, prompt/meta producers,
  hubs, Codex/agent environment, memory, and docs/knowledge surfaces.
- Store generated outputs under `.idd/knowledge/system-architecture.*` when
  produced for this workspace, but do not make CI require a parent meta checkout.

## Risks / Trade-offs

- Meta workspace state is local and may include dirty peer repos. The graph
  records that state as evidence instead of treating it as failure.
- The first system graph is metadata-oriented. Per-repo CodeGraph detail can be
  added later by recursively consuming peer `.idd/knowledge/architecture.json`
  files where they exist.

## Migration Plan

1. Add OpenSpec artifacts.
2. Add system graph DTOs and discovery.
3. Expose `rusty-idd knowledge system-architecture`.
4. Generate `.idd/knowledge/system-architecture.json` and Markdown for the
   current parent meta workspace.
5. Add focused tests and audit evidence.
