# scan-stage scoped Rust agent swarm package

- Stage: `scan`
- Target: `/home/drdave/Desktop/meta/rusty-idd`
- Purpose: Bound the scan stage to only the roles, contracts, tools, helpers, hooks, gates, and evidence needed to inventory a target and hand typed evidence to the next workflow stage.

## Agent Team

- `scan-orchestrator`: Owns stage routing, package scope, and evidence handoff for the scan target.
- `inventory-reader`: Collects repository files, manifests, agent surfaces, and toolchain signals without writing.
- `risk-classifier`: Classifies secrets, workflow drift, tool overflow, and adapter-boundary risks from scan outputs.

## Contracts

- `target-contract`: The scan package operates only on the declared target path.
- `inventory-contract`: Capture files, package managers, languages, agent directories, and workflow control-plane surfaces.
- `adapter-boundary-contract`: Treat .codex, .claude, .kimi, .agents, and peer agent directories as adapters or compatibility views, not source-of-truth toolboxes.
- `no-default-mcp-contract`: Do not include MCP servers in the default scan package unless a later feature gate declares a stage-specific reason.

## Tools

- `rusty-idd scan`: Generate deterministic inventory for the target.
- `rusty-idd knowledge plan-context`: Bind graph-backed context to the current goal before implementation.
- `rusty-idd manifest`: Refresh deterministic artifact inventory after scan-related control-plane changes.
- `rusty-idd validate`: Run Rusty IDD validation gates before handoff.
- `rusty-idd spec status`: Verify the active OpenSpec change before later workflow stages write code.

## Helpers

- `bounded-context-pack`: Use generated knowledge and context reports instead of broad manual rescans.
- `adapter-surface-map`: List agent directories as launch adapters and compatibility sources.
- `package-scope-check`: Ensure the selected package does not load unrelated stage tools.

## Hooks

- `pre-scan-package-check`: Verify goal, target, active change, and package stage before scan execution.
- `post-scan-evidence-check`: Require scan evidence and next-stage recommendation before handoff.

## Validation Gates

- `target-exists`: The declared package target must exist.
- `default-tool-scope`: The package tool list must stay scan-specific and omit default MCP sprawl.
- `adapter-minimality`: Adapter directories must not become the authoritative package catalog.
- `typed-evidence`: The package must declare evidence fields for the next workflow stage.

## Evidence Schema

- `inventory`: Repository inventory, package managers, languages, manifests, and agent adapter surfaces.
- `graph-context`: Knowledge graph and bounded context evidence for planning.
- `risk-register`: Tool overflow, secret/config, workflow drift, and adapter-boundary findings.
- `validation-summary`: Commands run, generated artifacts refreshed, and remaining gaps.
- `next-stage-recommendation`: The workflow stage and scoped package that should run after scan.

## Adapter Boundary

- `.codex`: Thin Codex launch adapter that calls Rusty IDD package generation.
- `.claude`: Compatibility/source-material view, not the active workflow package catalog.
- `.kimi`: Optional runtime adapter when present, not a source-of-truth toolbox.
- `.agents`: Reusable instructions remain available, but workflow packages decide what is loaded for a stage.

