# harness-vendor-render

## Why

ADR-0010 and ADR-0015 declare that vendor directories (`.claude`, `.codex`,
`.agents`, `.devin`) are thin adapters and that Rusty IDD owns the workflow. But
nothing *enforces* it — a vendor dir can silently grow back into an always-loaded
prose harness (the per-session token black hole). The front door (`rusty-idd
next`) and its `--json` mode now exist; this slice adds the generation + drift
gate that makes "thin adapters" a checked invariant.

## What Changes

- Add `rusty-idd render`: generate a deterministic, minimal adapter file into
  each vendor dir from one engine-owned template (the source of truth). The
  adapter points agents at `rusty-idd next`; it carries no workflow logic.
- Add `rusty-idd render --check`: regenerate in memory, compare to disk, and
  exit non-zero on any missing or drifted adapter (the drift gate).
- Render the adapters into the existing vendor dirs and wire `render --check`
  into CI and the Justfile so drift fails the build.

## Capabilities

### New Capabilities
- `harness-vendor-render`: engine-owned generation of minimal vendor adapters
  plus a fail-closed drift gate.

## Impact

- New `crates/cli/src/commands/render.rs`; CLI enum/dispatch/module wiring;
  `tests/render_cli.rs`. Generated `*/rusty-idd-adapter.md` in each vendor dir.
  CI (`.github/workflows/ci.yml`) + `Justfile` gain a `render --check` step. No
  removals; no new dependencies.
