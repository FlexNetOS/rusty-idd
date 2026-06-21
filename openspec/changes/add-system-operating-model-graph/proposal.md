# add-system-operating-model-graph

## Why

Rusty IDD can now generate repo-local architecture graphs, parent meta system
graphs, graph planning context, and bounded peer architecture summaries. The
remaining gap is that the system graph still speaks mostly in generic repo
roles. It does not encode the operating model for the wider agentic company
system: company hierarchy, handoff, weave, meta, envctl, vault, ruvector,
lifeos, prompt_hub, Teri, network control, parser/runtime, Lua/AR, and
distributed device capabilities.

Rusty IDD needs a deterministic generated artifact that maps the discovered
repo fleet onto that operating model before future cross-repo implementation
plans are produced.

## What Changes

- Add `rusty-idd knowledge operating-model`.
- Consume `.idd/knowledge/system-architecture.json`.
- Generate `.idd/knowledge/operating-model.json` and
  `.idd/knowledge/operating-model.md`.
- Map discovered repos into agentic-company layers and capabilities.
- Record external or not-yet-owned anchors such as Cognitum vault, upstream
  prompt sources, goose-like front door, Lua/AR, and distributed device fabric
  as findings rather than pretending they are implemented in Rusty IDD.
- Include Yazelix as the default terminal/runtime surface and RTK AI surfaces
  such as RTK, ICM, VOX, GRIT, and Beads as foundation agent-run capabilities.
- Add deterministic checks to keep the operating-model artifacts fresh.
- Preserve selected operating layers and capabilities inside graph planning
  context when the operating-model artifact is present.

## Capabilities

### New Capabilities

- `system-operating-model-graph`: turns the parent system architecture graph
  into a capability/layer map for full-system planning.

### Modified Capabilities

- `graph-context-planning`: planning artifacts can now refer to a generated
  operating-model graph for cross-repo responsibilities.

## Impact

- `crates/knowledge`
- `crates/cli`
- `.idd/knowledge`
- `.idd/MANIFEST.tsv`
- `.agents/skills/rusty-idd-knowledge`
- `/AI_MERGE`
