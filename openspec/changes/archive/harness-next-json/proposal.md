# harness-next-json

## Why

The harness control-plane front door `rusty-idd next` (ADR-0015) prints
human-oriented text. Non-interactive vendor adapters — the thin `.claude`/`.codex`
shims that backlog 4.1/4.2 will generate — need to *act on* the next imperative,
not scrape prose. A `--json` mode is the prerequisite brick for the
vendor-adapter render/drift gate and hook wiring.

## What Changes

- Add `--json` to `rusty-idd next`: emit one deterministic JSON object with the
  active change, artifact-DAG status, the next ready artifact, archivability, and
  the scoped next command. Reuse the `spec_status` snapshot so `next --json` and
  `spec status --json` cannot disagree.
- Fail closed: a dangling active-change pointer exits non-zero with no stdout
  JSON, so adapters do not act on a half-state.
- Default human output of `rusty-idd next` is unchanged.

## Capabilities

### Modified Capabilities
- `harness-control-plane`: the front door gains a machine-readable `--json` mode.

## Impact

- `crates/cli/src/commands/next.rs` (+ a serializable snapshot reused from
  `spec_status`); `tests/next_cli.rs` (json cases). No removals; no new deps
  beyond existing `serde`/`serde_json`.
