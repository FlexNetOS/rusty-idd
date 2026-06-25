# 02 — Research Dossier: HFTASK-0070

**Task:** ADR-0018 D5 — session-relay-resume/-wrap-up canonical format defined IN handoff,
rendered from the witnessed ledger/packet, deployed + byte-enforced fleet-wide via the
/handoff-loop-init family (HFTASK-0065). Cross-repo (harness_hub) — gatekeeper-gated.
**path_scope:** `handoff/**`, `spike/**`. **acceptance:** implemented + cargo test green + checkpointed.
**Cycle:** Phase 3 (research). **Date:** 2026-06-21.
**Confidence:** HIGH on (1)-(5). Web: not needed (entirely internal architecture/codebase; no
external version/protocol facts are load-bearing). Code intelligence: read AST-relevant sources directly.

---

## 0. VERDICT FIRST — the architecture-tension decision (load-bearing)

**VERDICT: "handoff-central relay format" is CONSISTENT with "handoff is an adapter under
rusty-idd / harness_hub is not the foundation." HFTASK-0070 PROCEEDS AS SPECIFIED.** This is a
decide-myself call (owner rule: research + decide, do not defer). It could be wrong only in the
narrow way noted in §0.4 — which this task does not trigger.

### 0.1 The two inputs, quoted

- **Card / ADR-0018 D5** (`docs/adr-0018-full-auto-agentic-operation.md:30-31`, `:93-97`):
  > "`session-relay-resume`/`-wrap-up` formatting is owned per-repo (harness_hub), **not centrally
  > by handoff**, so cross-fleet handoffs are inconsistent." … "**handoff owns the canonical
  > format/templates for both relay skills**; a deployment mechanism (the `/handoff-loop-init`
  > family, HFTASK-0065) pushes the canonical relay skills + format to every fleet member … The
  > relay skills render from the witnessed ledger/packet, never hand-authored prose."

- **Owner clarification** (ICM `decisions-rusty-idd`, 2026-06-21, quoted verbatim):
  > "handoff repo has two parts, central management and a `.handoff` directory created by
  > `/harness:handoff-loop-init` tracing back to the `.claude` harness at `meta/harness_hub`. **The
  > `.handoff` harness trace is not the desired foundation.** Revised architecture answer: handoff
  > should belong in `meta/rusty-idd` as a protocol/runtime/evidence adapter, not Rusty IDD embedded
  > under `meta/handoff` as the outer canonical repo."

### 0.2 Why they are CONSISTENT (the reconciliation)

The owner clarification is about **two distinct things, neither of which HFTASK-0070 contradicts**:

1. **Repo LOCATION / outer-canonical question** — "handoff should belong *in* meta/rusty-idd as an
   adapter, not Rusty IDD embedded *under* meta/handoff." This is a statement about which repo is the
   *outer* container and where handoff *sits in the tree*. HFTASK-0070 changes **no repo location, no
   `.meta.yaml` membership, no outer-vs-inner relationship.** It only moves *one continuity-render
   format* from harness_hub into handoff. Orthogonal axis — untouched.

2. **The undesired FOUNDATION trace** — "the `.handoff` harness trace **back to** the `.claude`
   harness at `meta/harness_hub` is not the desired foundation." Read precisely: the thing the owner
   does **not** want is `.handoff` (and, by D5's own framing, the relay skills) **depending on
   harness_hub as their source of truth.** HFTASK-0070 **removes exactly that trace** — it pulls the
   canonical relay format *out of* harness_hub and makes *handoff* the source. **This task is the
   owner's stated direction, executed.** Making handoff the canonical relay-format owner *eliminates*
   the harness_hub-as-foundation dependency the owner objected to.

3. **"Adapter" ≠ "not the owner of continuity rendering."** "handoff is a protocol/runtime/evidence
   adapter under rusty-idd" describes handoff's role *relative to rusty-idd's planning/intent plane*
   (rusty-idd owns intent/.idd/OpenSpec/ADR; handoff owns *witnessed execution truth* — see ICM
   `decisions-rusty-idd`: "keep hf/ledger/work-order as witnessed execution truth"). The session-relay
   artifacts **render from the witnessed ledger/packet** (D5: "render from the witnessed
   ledger/packet, never hand-authored prose") — that is **precisely handoff's evidence/runtime
   adapter job.** Continuity-render format is *exactly the kind of thing the adapter owns*, regardless
   of where the repo physically sits. Location is about the tree; format-ownership is about which
   component renders continuity. The card asks handoff to own the latter — fully in character for an
   evidence/runtime adapter.

**Net:** the owner wants the foundation trace to go *handoff→(its own ledger)*, not
*handoff→harness_hub*. HFTASK-0070 is the move that severs handoff's relay artifacts from harness_hub.
**It is aligned, not in tension.** The navigator's flag was correct to *raise* it; on inspection it
*resolves in favor of proceeding.*

### 0.3 Corroborating ICM evidence
- `decisions-rusty-idd`: "keep hf/ledger/work-order as witnessed execution truth and Rusty IDD
  .idd/OpenSpec/ADR/knowledge as planning and validation truth." → continuity rendering (ledger→packet→relay)
  is handoff's lane by the owner's own split.
- `decisions-rusty-idd`: "Supersede the prior handoff-outer ADR with a **new ADR** rather than editing
  accepted ADR history." → the location/outer-canonical realignment is a *separate, future ADR in
  rusty-idd*, not something HFTASK-0070 touches or pre-empts. The two efforts do not collide.

### 0.4 How the verdict could be wrong (the honest caveat — does NOT fire here)
The verdict would be wrong **only if** D5's mechanism required handoff to *deepen* its dependency on
harness_hub (e.g. importing harness_hub as the runtime source for the templates). It does the
opposite: §3/§4 below show the implementation is **self-contained in handoff** (templates authored in
`handoff/`, deploy writes member copies at install-time) and **never reads from nor edits
harness_hub**. So the one failure mode is structurally excluded. **Verdict stands: PROCEED.**

> Gatekeeper note: surface this reconciliation explicitly in the verdict. The card's *prose* still
> says "today harness_hub-owned" (a true statement of the **current** state being corrected); the
> implementer must **not** edit that locked objective (byte-exact, objective_hash
> `blake3:ca93233…`) — it correctly describes the starting condition. Any "this is now handoff-owned"
> reconciliation goes in the **ADR/comments**, never the seed objective (HFTASK-0075 precedent).

---

## 1. Findings — the canonical format to render (D5: "from the witnessed ledger/packet, never prose")

### 1.1 Current harness_hub templates (the content handoff must make canonical)
Read in full: `../harness_hub/harness/skills/session-relay-resume/SKILL.md` (3.7K),
`../harness_hub/harness/skills/session-relay-wrap-up/SKILL.md` (5.2K),
`../harness_hub/harness/skills/session-relay-resume/scripts/verify-on-resume.template.sh` (939B).

**session-relay-resume** is a 7-step cold-start sequence: (1) ICM recall, (2) weave inbox scan,
(3) locate+read committed `HANDOFF.md` — *"If `hf` is reachable, prefer `hf resume` to render the
packet from the witnessed ledger"* (`SKILL.md:36-37`), (4) verify-on-resume fail-closed baseline,
(5) broadcast `relay:resumed`, (6) reset session counter, (7) hand back to loop at `next_item`.

**session-relay-wrap-up** is an 8-step end-of-session sequence: (1) stop-checks, (2) Phase-E retro,
(3) ICM store, (4) write checkpoint — *"If the meta handoff kernel (`hf`) is reachable, prefer
`hf checkpoint` / `hf handoff` to render the packet from the witnessed ledger; the file-based form is
the fallback"* (`SKILL.md:44-45`), (5) commit, (6) weave heartbeat, (7) best-effort successor cron,
(8) stop. Its `HANDOFF.md` schema (`SKILL.md:64-80`): `next_item`, `last_item`, `cycles_total`,
`cycles_this_session`, `gate_status`, `landed_this_session`, `icm_stored`, `verify_on_resume`,
`resume_command`.

**Crucial pre-existing alignment:** *both* harness_hub skills **already say** "prefer the `hf`
ledger/packet render when reachable." So D5 is not inventing a new render path — it is **promoting
the `hf`-rendered form from optional to canonical** and making handoff the owner of the template.

### 1.2 What `hf` already renders — the ledger/packet source (the anti-prose substrate)
`render_packet_md` (`hf/src/main.rs:2362-2416`) emits `handoff.packet.v2` with these sections, **each
derived from the witnessed ledger replay** (`replay`, `witness` count, `next_safe`, `summary_json`):

| Packet section (main.rs) | Source | Maps to relay field |
|---|---|---|
| `## 1. North Star` (`:2384`) | `capsule_field("northstar")` (ADR-0006, not hardcoded) | resume orientation header |
| `## 2. State Precedence` (`:2385`) | constant | resume "what's authoritative" |
| `## 3. Progress` Done N/M + witnessed events (`:2386-2391`) | `done.len()/tasks.len()`, `witness` | wrap-up cycle summary; resume progress |
| `## 4. Remaining (next safe first)` (`:2392-2400`) | `remaining` filtered by replay | wrap-up "what's left" |
| `## 5. Next Best Task` id/title/objective (`:2401-2407`) | `next_safe(tasks, replay)` | resume `next_item`; wrap-up `next_item` |
| `## 6. Resume Commands` `hf resume` + next_command (`:2408-2411`) | `summary["next_command"]` (`:2342`, `hf claim <id>`) | resume `resume_command`; wrap-up `resume_command` |
| `## 7. Machine Summary` JSON (`:2412-2414`) | `summary_json` (`:2318`) | machine-readable handoff payload |

`summary_json` (`hf/src/main.rs:2318-2345`) is the *single source* for both the packet and
`hf resume --json` (`:2317,2347`) — `next_command` = `format!("hf claim {}", t.id)` when a next-safe
task exists. `cmd_handoff` (`:2448`) and `cmd_resume` (`:2522`) both render through `render_packet_md`.
`active_task` (`:2420`) + `checkpoint_count` (`:2431`) give the wrap-up its claimed-task + checkpoint
state. **Conclusion: every relay template field has a witnessed-ledger source already emitted by `hf`.**
The canonical relay templates therefore **instruct the agent to invoke `hf resume`/`hf handoff` and
fill the HANDOFF.md schema from that rendered packet** — satisfying "never hand-authored prose."

### 1.3 Field→source map the implementer encodes into the canonical templates
- resume: `next_item` ← packet §5 / `summary.next_command`; progress ← packet §3; `verify_on_resume`
  ← the seeded `verify-on-resume.template.sh`; orientation ← packet §1/§2.
- wrap-up: cycle summary ← packet §3 (Done N/M, witnessed count) + `checkpoint_count`; `next_item` ←
  packet §5; `gate_status` ← (loop-provided); `resume_command` ← `hf resume`; checkpoint body ←
  `hf checkpoint`/`hf handoff` render (NOT prose).

---

## 2. Cross-references / mismatches

- **MATCH:** harness_hub skills already prefer the `hf` render (§1.1) → D5 only formalizes ownership.
  No behavioral contradiction.
- **MATCH:** the navigator's surface map (deployed copies in `network-control`, `harness-agent-rs`,
  `envctl`, `harness_hub/handoff-loop`) is the set of byte-consistency targets. Confirmed they are
  *deployed* copies, so overwriting them at install-time is the intended enforcement, not a source edit.
- **MISMATCH (benign, do NOT "fix"):** card prose says "today harness_hub-owned" — *true of the start
  state*; locked objective stays byte-exact (§0.4). The post-change reality is recorded in ADR/comments.
- **GAP (the actual work):** `grep -i session-relay scripts/handoff-loop-init.sh scripts/handoff-lib.sh`
  → nothing. handoff deploys ZERO session-relay artifacts today. New `deploy_session_relay()` + a
  canonical template source under `handoff/` is the increment.
- **NO drift** between ledger/packet and the templates — the packet IS the render source.

---

## 3. The deploy mechanism to mirror (HFTASK-0078 `deploy_diff_drive` is the exact precedent)

`scripts/handoff-loop-init.sh`. Reusable scaffolding (cite):
- `SCRIPT_DIR` (`:28`), `KERNEL_HOME` (`:46-55`, robust dev-vs-ejected detection), `META_ROOT`
  (`:50`), `fleet_members()` (`:124-130`), `say()`/`run()`/`$DRY` (`:77-78`).
- The per-dir main loop (`:213-281`) iterates `${TARGETS[@]}`; `--fleet` (`:65,131-133`) expands to
  every `.meta.yaml` member with a `.git`.
- **`deploy_diff_drive()` (`:191-211`) — copy this shape verbatim:** resolves a canonical source
  (`$KERNEL_HOME/...` first, then `$SCRIPT_DIR/..` vendored fallback for the ejected case), fails
  closed with a `say` + `return 1` when no source is reachable, honors `$DRY`, `mkdir -p` the target
  subdir, plain `cp` (idempotent overwrite), returns 0 on deploy.
- Wiring points: a counter like `DIFFDRIVE=0` (`:139`), the call site in the loop
  (`deploy_diff_drive "$dir" && { DIFFDRIVE=... }`, `:262-263`), and the commit `git add` allowlist
  (`:273`).

### 3.1 Shape of `deploy_session_relay()` (to ADD, in handoff scope)
```
deploy_session_relay() {
  local dir="$1"
  # canonical source = handoff's own committed relay templates (dev checkout or ejected)
  local src="$KERNEL_HOME/.claude/skills"; [ -d "$src/session-relay-resume" ] || src="$SCRIPT_DIR/../.claude/skills"
  # … vendored fallback path under the plugin if neither exists → say + return 1 (fail-closed)
  for skill in session-relay-resume session-relay-wrap-up; do
    mkdir -p "$dir/.claude/skills/$skill"
    # byte-consistency ENFORCEMENT: overwrite (cp) any drifted member copy; detect+report drift first
    if [ -f "$dir/.claude/skills/$skill/SKILL.md" ] && ! cmp -s "$src/$skill/SKILL.md" "$dir/.claude/skills/$skill/SKILL.md"; then
      say "  session-relay drift in $skill — re-deploying canonical (byte-consistency)"
    fi
    cp "$src/$skill/SKILL.md" "$dir/.claude/skills/$skill/SKILL.md"
    [ -d "$src/$skill/scripts" ] && { mkdir -p "$dir/.claude/skills/$skill/scripts"; cp -r "$src/$skill/scripts/." "$dir/.claude/skills/$skill/scripts/"; }
  done
}
```
- **Drift = re-deploy overwrites** (HFTASK-0067 byte-consistency model), with a `say` report when an
  existing copy differs (`cmp -s`). Idempotent: a matching copy re-deploys harmlessly.
- Call site mirrors `:262-263`; counter `RELAY=0`; add `.claude/skills/session-relay-resume
  .claude/skills/session-relay-wrap-up` to the commit allowlist (`:273`).
- `--fleet` already drives it across members; `--dry-run` already gated via `$DRY`.

---

## 4. Scope-law guard (path_scope = `handoff/**` + `spike/**`, NOT `harness_hub/**`)

**The whole task is achievable inside `handoff/**` — NO harness_hub source edit required. Confirmed.**

1. **Canonical templates:** author them as **handoff-owned files** under
   `handoff/.claude/skills/session-relay-{resume,wrap-up}/SKILL.md` (+ the
   `scripts/verify-on-resume.template.sh`). handoff currently has only a single
   `handoff/.claude/skills/session-relay/SKILL.md` (4.3K) — adding the split resume/wrap-up skills is
   net-new in-scope. (Content seeded from the harness_hub versions, re-pointed to canonical `hf`
   render per §1.2; harness_hub is *read* as a reference, never *written*.)
2. **Deploy:** `deploy_session_relay()` lives in `handoff/scripts/handoff-loop-init.sh` (in-scope) and
   **writes member copies at install-time** — a runtime action against `$dir/.claude/...`, NOT a
   tracked source edit of harness_hub. Writing into a *member's working tree when the operator runs
   the initializer* is the established pattern (`deploy_hooks`/`deploy_diff_drive` already write into
   `$dir`), and is outside this PR's committed diff.
3. **Walls flagged for the gatekeeper (none block this PR):**
   - Editing `harness_hub/harness/skills/session-relay-*/SKILL.md` to *remove* its now-superseded
     ownership, or marking harness_hub's copies as "deployed-from-handoff," **WOULD be out of
     path_scope** → must be a **separate, gatekeeper-authorized cross-repo PR** (or deferred). The
     navigator's reconciliation note in `decisions-rusty-idd` ("supersede via a new ADR, don't edit
     accepted ADR history") supports deferring the harness_hub-side cleanup.
   - This PR's committed diff must be **exactly** files under `handoff/` (+ `spike/` if used). Cargo.lock
     only if a dep is genuinely added (this task needs none → expect no Cargo.lock change).

---

## 5. Minimal viable implementation plan (one witnessed cycle, byte-exact, in-scope)

**Recommended approach:** template-as-handoff-asset + mirror `deploy_diff_drive` + a `cargo test`
guard via the seeded tight `test_commands` (the HFTASK-0078 pattern). Lowest blast radius; zero new
crate code paths; reuses the proven deploy scaffold.

**Files to ADD/EDIT (all under `handoff/**`):**
1. **ADD** `handoff/.claude/skills/session-relay-resume/SKILL.md` — canonical resume template,
   re-pointed so step 3/4 invoke `hf resume` as the **authoritative** render (not "if reachable").
2. **ADD** `handoff/.claude/skills/session-relay-resume/scripts/verify-on-resume.template.sh` — copy
   of the harness_hub template (939B).
3. **ADD** `handoff/.claude/skills/session-relay-wrap-up/SKILL.md` — canonical wrap-up template,
   step-4 checkpoint render via `hf checkpoint`/`hf handoff` as authoritative; HANDOFF.md schema kept.
4. **EDIT** `handoff/scripts/handoff-loop-init.sh` — add `deploy_session_relay()` (mirror
   `deploy_diff_drive`, §3.1), the `RELAY=0` counter (`:139`), the loop call site (after `:263`), and
   the commit allowlist entries (`:273`). Keep `$DRY`/fail-closed/vendored-fallback parity.
5. **EDIT** `hf/src/main.rs` `cmd_seed` tight-`test_commands` block (the `match wo.id { … }` at
   `:3016`) — add an `"HFTASK-0070" =>` arm with **executable, fast** checks so `cargo test`/`hf test`
   has real evidence, e.g.:
   ```
   "HFTASK-0070" => &[
     "bash -n scripts/handoff-loop-init.sh",
     "test -f .claude/skills/session-relay-resume/SKILL.md",
     "test -f .claude/skills/session-relay-wrap-up/SKILL.md",
     "grep -q 'hf resume' .claude/skills/session-relay-resume/SKILL.md",
   ],
   ```
6. **ADD** a Rust unit test (in `hf/src/main.rs` `#[cfg(test)]`) asserting the seed for HFTASK-0070
   carries the session-relay tight test_commands (parity with how 0078's seeding is covered) — this is
   what makes `cargo test` (the card's `test_commands`) green and gives the verifier a deterministic
   assertion. **Run `cargo clippy --workspace --all-targets -- -D warnings` + `cargo fmt --all --check`
   before push** (the PR #30 / handoff CI lesson — `--all-targets` lints the new test code).

**Acceptance mapping:**
- "implemented" → files 1-4 (canonical templates + deploy_session_relay).
- "cargo test green" → files 5-6 (seeded tight tests + the Rust unit test); run `cargo test --workspace`.
- "checkpointed" → ≥1 witnessed `hf checkpoint HFTASK-0070` during the cycle.

**Blast radius (code intelligence):** `render_packet_md`, `summary_json`, `next_safe`, `cmd_resume`,
`cmd_handoff` are **READ ONLY** here — the templates *consume* their output; no signature change, so
zero caller breakage. `cmd_seed` edit is additive (a new match arm + a new test) — the seed is
idempotent/additive (`:3025-3037`, only writes MISSING cards), so re-seed won't clobber the live
HFTASK-0070 status. `deploy_session_relay` is a new function, zero existing callers affected.

---

## 6. ADR-ready Research section (for the gatekeeper/doc-updater)

> **Decision:** handoff becomes the canonical owner of the `session-relay-resume`/`-wrap-up` format,
> rendered from the witnessed `hf` ledger/packet, and deploys + byte-enforces them fleet-wide via
> `handoff-loop-init.sh::deploy_session_relay()` (mirroring HFTASK-0078 `deploy_diff_drive`).
> **Consistency with the rusty-idd realignment:** this *removes* the harness_hub-as-foundation trace
> the owner flagged (ICM `decisions-rusty-idd`, 2026-06-21) — it does not entrench it. Repo
> location/outer-canonical (handoff-under-rusty-idd) is a separate axis handled by a future rusty-idd
> ADR; continuity-render-format ownership is handoff's evidence/runtime-adapter lane regardless of
> tree location. **Scope:** implemented entirely within `handoff/**`; the harness_hub-side cleanup of
> the now-superseded copies is a separate gatekeeper-authorized cross-repo step (NOT in this PR).
> **Evidence:** `hf/src/main.rs:2362-2416` (packet v2 render), `:2318-2345` (summary_json source),
> harness_hub `SKILL.md` already prefers `hf` render (`session-relay-resume/SKILL.md:36-37`,
> `session-relay-wrap-up/SKILL.md:44-45`), `handoff-loop-init.sh:191-211` (deploy precedent).

---

## Handoff to next agents
- → **kernel-implementer:** approach = §5 (template-as-asset + mirror `deploy_diff_drive` + seeded
  tight tests + 1 Rust unit test). Stay byte-exact on the locked objective; do NOT edit harness_hub.
  Run `cargo test --workspace` + clippy `--all-targets` + `cargo fmt --check` before push. Blast
  radius = near-zero (read-only on render fns; additive `cmd_seed`/deploy).
- → **code-omniscient-gatekeeper:** the load-bearing audit is (a) the §0 architecture-tension verdict
  (CONSISTENT — surface the reconciliation in the verdict), (b) §4 scope-law (committed diff must be
  exactly `handoff/**`; ANY harness_hub source edit = out-of-scope wall needing a separate authorized
  PR), (c) the locked objective stays byte-untouched (HFTASK-0075 precedent), (d) `cargo test` shows
  tests-ran>0 (U1 fail-closed; `executed 0 tests` = FAIL).
