# fleet-deploy-control-plane

## Why

Rusty IDD is the single harness control plane (ADR-0015), and `rusty-idd render`
keeps the *home* repo's vendor directories thin by rendering an engine-owned
adapter (`render::expected_adapter`) and drift-checking it (`render --check`).
But there is no way to install that same thin control-plane surface into the
*other* repos of the meta fleet. Today each fleet member carries its own
divergent prose harness, so an agent moving between repos must re-learn a
different harness each time, and the per-session prose blob is the token black
hole ADR-0015 set out to remove.

The fleet needs a **front door for deployment**: one deterministic command that
installs the rusty-idd thin-adapter surface (the per-vendor
`rusty-idd-adapter.md` plus the SessionStart hook that calls `rusty-idd next`)
into a target repo, **without touching that repo's own forge loop or runtime**.
Rusty IDD consumes each repo's existing harness; it does not replace it. The
result is a uniform minimal agent harness across the whole fleet, so agents are
seamlessly swappable (CLI-over-MCP).

## What Changes

- A new `rusty-idd deploy` command that renders the **byte-identical**
  thin-adapter surface (reusing `render::expected_adapter` and the `VENDORS`
  gate) into a **target repo root** (`--target <path>`), writing each existing
  vendor dir's `rusty-idd-adapter.md` and a SessionStart hook that runs
  `rusty-idd next --base <target>`.
- An idempotent `--check` / `--dry-run` mode that reports per-target drift
  (missing or stale adapters/hooks) and exits non-zero on drift, without
  writing — so fleet state is verifiable and CI-gateable, exactly like
  `render --check` but across repos.
- A `--vendor <name>` selector (mirroring `render`) so a target can opt into a
  specific surface, and an `--all` default that deploys to every vendor dir that
  already exists in the target (never silently creating unsolicited surfaces).
- The deploy is strictly **additive**: it only writes adapter docs and the
  SessionStart hook entry; it MUST NOT modify the target's forge loop, runtime,
  build files, or any generated artifact, and MUST NOT delete anything.
- Documentation + ADR-0017 recording the fleet-deploy decision and the
  thin-adapter-only constraint for fleet members.

## Capabilities

### New Capabilities
- `fleet-deploy`: Deploy the rusty-idd thin-adapter control-plane surface
  (per-vendor adapter docs + SessionStart `rusty-idd next` hook) into target
  fleet repos additively, with an idempotent drift-checking dry-run mode, never
  mutating a target's own forge loop or runtime.

### Modified Capabilities
<!-- none: this slice adds a new capability and reuses existing render machinery without changing its spec-level behaviour. -->

## Impact

- New CLI command surface: `crates/cli/src/commands/` (a `deploy` module) wired
  into the command dispatch; reuses `render::expected_adapter` / `VENDORS` as the
  shared source of truth (no duplication of adapter content).
- New tests: `crates/cli/tests/` deploy integration tests (deploy into a temp
  target, idempotency, `--check` drift detection, additive-only guarantee).
- New spec capability `openspec/specs/fleet-deploy/` (via this change's delta).
- New `adr/0017-fleet-deploy-control-plane.md`.
- No change to existing gates, dependencies, or generated-artifact contracts;
  CI may later add a `deploy --check` fleet gate (out of scope for this slice).
