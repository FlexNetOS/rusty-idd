# fleet-deploy-control-plane — Design

## Context

ADR-0015 makes Rusty IDD the single harness control plane; `rusty-idd render` +
`render --check` keep the *home* repo's vendor dirs thin by generating an
engine-owned adapter (`render::expected_adapter`) and failing closed on drift.
There is no equivalent for *peer* repos: the fleet cannot be brought under the
same thin control plane. This slice adds `rusty-idd deploy`, which installs the
identical thin-adapter surface into a *target* repo root, consuming (never
replacing) that repo's own forge loop and runtime.

## Goals / Non-Goals

**Goals:**
- One deterministic command installs the rusty-idd thin-adapter surface into a
  target repo: per-vendor `rusty-idd-adapter.md` (reused from `render`, the
  single source of truth) + a SessionStart hook calling `rusty-idd next`.
- Strictly additive: no mutation/deletion of the target's forge loop, runtime,
  build, source, or generated artifacts; existing hook entries preserved.
- Idempotent `--check`/`--dry-run` drift gate, per target, fail-closed.

**Non-Goals:**
- No adoption of handoff/prompt_hub runtimes into rusty-idd (sequenced follow-on).
- No retirement / `.meta.yaml` unregistration.
- No new vendor surfaces beyond the known `render` set.
- No CI fleet gate wiring in this slice (a `deploy --check` gate can follow).

## Decisions

1. **Reuse, never duplicate, the adapter source of truth.** `deploy` calls the
   same `render::expected_adapter(vendor)` and iterates the same `VENDORS` set as
   `render`. The adapter written into `<target>/<vendor-dir>/rusty-idd-adapter.md`
   is byte-identical to `rusty-idd render`'s output. `expected_adapter` / the
   vendor table become `pub(crate)` so `deploy` shares them.

2. **Target root is explicit.** `--target <path>` is the target repo root
   (default `.`). All writes are under `<target>/`. The command never walks
   above the target.

3. **Deployed hook uses the installed binary, not `cargo run`.** The home repo's
   SessionStart hook runs `cargo run --manifest-path "$root/Cargo.toml" --bin
   rusty-idd -- next` because it IS the rusty-idd workspace. A peer fleet repo is
   not, so the deployed hook resolves the repo root at runtime and calls the
   `rusty-idd` binary on PATH:

   ```sh
   sh -lc 'root="$(git rev-parse --show-toplevel)"; exec rusty-idd next --base "$root"'
   ```

   This is the canonical deploy hook command (one `const`), target-agnostic and
   move-safe. ("Deploy the full package" implies `rusty-idd` is installed on PATH
   across the fleet, the same model as `hf`.)

4. **Hook-capable vendors.** Only vendors with a known hook-config format get a
   SessionStart hook:
   - `codex` → `<target>/.codex/hooks.json` (`hooks.SessionStart[]`)
   - `claude` → `<target>/.claude/settings.json` (`hooks.SessionStart[]`)
   `agents` / `devin` receive the adapter doc only (no defined hook runtime).

5. **Hook merge preserves everything.** The config JSON is parsed; if
   `hooks.SessionStart` does not already contain an entry whose inner command
   equals the canonical deploy hook command, that entry is appended. All other
   keys (e.g. `$comment`), other hook phases (PreToolUse/Stop/…), and other
   SessionStart entries are preserved. Missing file → create the minimal
   `{"hooks":{"SessionStart":[<entry>]}}`. This is idempotent.

6. **`deploy` (write) vs `deploy --check` (compare) share one planner.** A pure
   function computes the desired surface (adapter bytes per vendor + whether the
   canonical hook entry is present). `--check`/`--dry-run` reports drift and exits
   non-zero without writing; a clean target exits zero, writes nothing.

7. **Drift semantics.** Adapter drift = file missing OR bytes differ from
   `expected_adapter` (byte comparison, since it is generated). Hook drift =
   the canonical SessionStart entry is absent from the parsed config (semantic
   comparison, so unrelated key ordering/whitespace never causes false drift).
   `--check` aggregates all drift across vendors, prints each, exits 1.

8. **`--all` vs `--vendor`.** Default `--all` targets only vendor dirs that
   already exist under `<target>` (never creates unsolicited surfaces, mirroring
   `render --all`). An explicit `--vendor <name>` targets that one and may create
   its dir (mirroring `render --vendor`).

## Risks / Trade-offs

- **JSON reformatting.** Writing back parsed JSON may reorder keys vs the
  original. Mitigation: hook `--check` is semantic (entry-presence), not a byte
  diff, so reformatting never trips the gate; and the merge preserves all keys/
  values, only appending. Adapter `.md` stays a byte gate (it is fully generated).
- **PATH assumption.** The deployed hook needs `rusty-idd` on PATH in the target
  environment. This is the intended fleet model (install the package), and it
  fails loud (command-not-found) rather than silently mis-running, so a missing
  install is observable. Documented as a deploy precondition.
- **Partial multi-vendor write.** If a later vendor write fails mid-run, earlier
  vendors are already written. Mitigation: writes are idempotent and additive, so
  a re-run completes the deploy; `--check` then confirms convergence.

## Migration / Rollout

- Land `deploy` behind tests; deploy first to the home repo (no-op vs `render`
  for adapters) and to one peer (handoff) to prove the path.
- A later slice may add `rusty-idd deploy --all-fleet --check` reading
  `.meta.yaml` to gate the whole fleet in CI (out of scope here).
