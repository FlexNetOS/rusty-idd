# 0017. Fleet deploy: install the thin control-plane surface into peer repos

- Status: accepted
- Date: 2026-06-23

## Context

ADR-0015 established Rusty IDD as the single harness control plane and the
`harness-vendor-render` slice made the *home* repo's vendor directories thin by
generating an engine-owned adapter (`render::expected_adapter`) and failing
closed on drift (`render --check`). That enforcement stops at the home repo: the
rest of the meta fleet still carries divergent, per-repo prose harnesses, so an
agent moving between repos re-learns a different harness each time and each
session re-loads a repo-specific prose blob — the token cost ADR-0015 set out to
remove.

The fleet model is: each repo keeps its own forge loop and runtime; Rusty IDD is
the thin control plane layered on top. To realize that across more than one repo
we need a deterministic way to *install* the thin-adapter surface into a peer
repo without touching its runtime. Rendering only ever wrote into the current
repo; there was no "deploy into target X" verb.

## Decision

1. **Add `rusty-idd deploy --target <repo>`** which installs the thin-adapter
   control-plane surface into a target repo root: for each targeted vendor it
   writes `<target>/<vendor-dir>/rusty-idd-adapter.md` and a SessionStart hook
   that calls the front door.
2. **The adapter content is the same engine source of truth as `render`.**
   `deploy` reuses `render::expected_adapter` and the `VENDORS` table; the
   adapter bytes are identical to `rusty-idd render`'s output. There is no second
   copy of adapter content to drift.
3. **The deployed SessionStart hook uses the installed `rusty-idd` binary on
   PATH**, resolving the repo root at runtime
   (`sh -lc 'root="$(git rev-parse --show-toplevel)"; exec rusty-idd next --base
   "$root"'`), because a peer repo is not the rusty-idd cargo workspace and
   cannot `cargo run` it. Deploying the full package therefore implies
   `rusty-idd` is installed on PATH across the fleet (the same model as `hf`).
4. **Deploy is strictly additive and never mutates the target runtime.** It only
   writes vendor adapter docs and merges the SessionStart hook entry, preserving
   all other config keys and hook phases. It never modifies or deletes the
   target's forge loop, runtime, build, source, or generated artifacts.
5. **An idempotent, fail-closed `deploy --check`/`--dry-run` drift gate**
   reports per-target drift (missing/stale adapter bytes; absent canonical hook
   entry) and exits non-zero without writing; a clean target exits zero. Adapter
   drift is a byte comparison (generated content); hook drift is a semantic
   entry-presence check (so JSON key ordering never causes false drift).

This builds on ADR-0010 (task-scoped packages) and ADR-0015 (single control
plane) and extends the `render` / `render --check` invariant from the home repo
to the fleet, without superseding either.

## Consequences

- The whole fleet can present one minimal agent harness, so agents are
  seamlessly swappable (CLI-over-MCP): the only harness an agent sees is the thin
  front door, identical everywhere.
- A fleet member's own forge loop and runtime remain authoritative and untouched;
  Rusty IDD consumes them, it does not replace them.
- A later slice may add a `deploy --all-fleet --check` mode that reads
  `.meta.yaml` to gate the entire fleet in CI; this ADR does not require it.
- Deployment depends on `rusty-idd` being installed on PATH in the target
  environment; a missing install fails loud (command-not-found), not silently.
