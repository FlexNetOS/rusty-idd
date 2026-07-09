# rusty-idd — security advisory ledger

Tracks `cargo audit` findings for the workspace and their disposition. Doctrine: **STRICT
UPGRADE-ONLY — no unmaintained dependency is left in the tree, and nothing is suppressed.**
Every finding below is FIXED (the crate is gone from the tree), not ignored. The machine-enforced
policy lives in `.cargo/audit.toml`, which has an **empty `ignore` list**.

Last reviewed: **2026-06-26**.

## Policy

<!-- drift-guard id="no-unmaintained-bincode-yamlrust" forbid="bincode =,yaml-rust =" path="**/*.toml" reason="bincode (RUSTSEC-2025-0141) and yaml-rust 0.4 (RUSTSEC-2024-0320) were eliminated workspace-wide (vendored syntect+postcard); reintroducing either as a direct dependency is a regression" -->

This decision is **machine-enforced** by the `hf drift` §12.3 #8 sentinel: the `drift-guard`
marker above forbids `bincode`/`yaml-rust` from reappearing as a direct dependency in any
non-vendored manifest. If one does, `hf drift` reports a decision contradiction naming this record.

- **Vulnerabilities** (`RUSTSEC` with a non-empty `patched`): upgrade or replace. Never ignored.
- **Unmaintained** (informational): if a maintained successor exists, switch to it; if the dead
  crate is buried in an upstream dependency, **host that upstream locally and upgrade the dead
  dep ourselves** (the vendored-fork pattern below). Do **not** add an `ignore` to paper over it.

## The vendored syntect fork — `vendor/syntect`

All three problem crates entered via `syntect 5.3.0` (pulled by `comrak`/`tui-markdown`, used by
`crates/{spec,tui,cli}`; the TUI genuinely uses `tui-markdown`'s `highlight-code`). syntect could
not be dropped, and downstream crates can't subtract a transitive's features, so we took syntect
under our control: `vendor/syntect` is a FlexNetOS fork of syntect `master` (rev `4aa78031`),
wired in via `[patch.crates-io] syntect = { path = "vendor/syntect" }` in the workspace `Cargo.toml`.

### RUSTSEC-2024-0320 — `yaml-rust 0.4.5` unmaintained — ✅ FIXED
syntect `master` had already migrated to the maintained **`yaml-rust2 0.10.4`**; the fork inherits
that. `cargo tree -i yaml-rust` → not present.

### RUSTSEC-2025-0141 — `bincode 1.3.3` unmaintained — ✅ FIXED
This advisory flags **all** bincode versions (the crate was permanently frozen, `patched = []`),
so a version bump could not clear it. bincode was syntect's dump (`.packdump`/`.themedump`) format.
The fork **replaces bincode with the maintained `postcard`** in `vendor/syntect/src/dumps.rs`
(postcard is, like bincode, a compact non-self-describing serde format, so syntect's existing
`Serialize`/`Deserialize` types port unchanged) and the bundled `assets/*` were **regenerated in
postcard format** via the `gendata` example. `cargo tree -i bincode` → not present.

### RUSTSEC-2023-0089 — `atomic-polyfill` unmaintained — ✅ FIXED (avoided)
postcard's *default* features pull `heapless 0.7`, which pulls the unmaintained `atomic-polyfill`.
The fork depends on postcard with `default-features = false, features = ["use-std"]` — we only need
`to_allocvec`/`from_bytes` on std `Vec`/slices, never heapless — so neither `heapless` nor
`atomic-polyfill` enters the tree. `cargo tree -i atomic-polyfill --target all` → not present.

## Verification (2026-06-26)

- `cargo tree -i` for `yaml-rust` / `bincode` / `atomic-polyfill` / `heapless` → all "did not match".
- `cargo audit --deny warnings` → **exit 0 with an empty ignore list** (nothing suppressed).
- Highlight parity preserved: `crates/tui` `test_code_blocks_rendered_with_highlighting` passes;
  the `gendata`-regenerated assets load and highlight (verified via syntect's `syncat` example).
- Workspace: 695 tests pass; build green default / `--no-default-features` / `--all-features`;
  clippy `--workspace --all-targets -- -D warnings` clean.

## Maintenance / exit condition

The fork tracks syntect `master` rev `4aa78031`. When upstream ships **syntect 6.0.0** with its own
bincode replacement (`trishume/syntect#623`/`#694`) and `tui-markdown` revs to it, reconcile:
re-evaluate whether the fork is still needed; if upstream's serializer is maintained, drop
`vendor/syntect` and the patch and depend on the released crate. Until then we carry the fork
(rebase onto new syntect master as needed; regenerate `assets/*` with the `gendata` example after
any serializer change).

### Regenerating the assets (after a serializer/source change)

```
# from a checkout that has syntect's testdata submodules populated:
cd vendor/syntect && ln -s <syntect-testdata> testdata
cargo run --features=metadata --example gendata -- synpack testdata/Packages \
  assets/default_newlines.packdump assets/default_nonewlines.packdump \
  assets/default_metadata.packdump testdata/DefaultPackage
cargo run --features=metadata --example gendata -- themepack testdata assets/default.themedump
rm testdata   # regen-only; never committed
```

## C-dependency removal — `onig` / `onig-sys` — ✅ DONE (no-C highlighting)

Not a `cargo audit` finding (oniguruma is maintained), but a **C** dependency in a no-C-preferring
tree. Removed by flipping the highlighting backend to pure-Rust `fancy-regex`, via two coordinated
changes (one alone is insufficient — the two syntect entry points request features differently):

1. **`vendor/syntect/Cargo.toml`:** the fork's `default` flipped from `default-onig` to
   `default-fancy`. `tui-markdown` requests syntect's *default* features, so owning the fork's
   default flips that path to `fancy-regex`. The regex *source* strings live in the `.packdump`
   assets and compile under either backend, so **no asset regeneration** was needed.
2. **`crates/spec/Cargo.toml`:** `comrak = { version = "0.52", default-features = false }`. comrak
   sets `default-features = false` on syntect and explicitly selects an onig backend, so the fork
   default flip does **not** reach the comrak path. But `crates/spec` uses comrak only for
   parse/emit (`parse_document`/`format_commonmark`/`Arena`/`nodes`, no `SyntectAdapter`), so
   dropping comrak's default features removes its syntect (and onig) entirely.

Verified: `cargo tree -i onig` / `onig-sys` → both "did not match"; `fancy-regex 0.18` is the
active backend; highlight parity holds (the `crates/tui` test passes and `syncat --features
default-fancy` produces identical colors to the onig backend).
# Security advisories — status & accepted-risk register

This file records every advisory `cargo audit` reports against the rusty-idd
workspace, the disposition of each, and the re-evaluation trigger. It is the
human-readable companion to the machine-readable baseline `.cargo/audit.toml`
(auto-loaded by every `cargo audit`) and the CI gate in
`.github/workflows/ci.yml`.

Policy (per the loop's UPGRADE-ONLY / NO-DOWNGRADE invariant): advisories are
remediated by moving **forward** (newer dependency) — never by downgrading or by
removing a capability. When no forward path exists and the advisory is a
non-vulnerability *unmaintained* warning, it may be **accepted-risk** here with an
explicit rationale and a re-evaluation trigger, rather than walling all work.

## Remediated

| Advisory | Crate | Disposition | Slice |
|----------|-------|-------------|-------|
| RUSTSEC-2026-0009 | `time` 0.3.41 → **0.3.47** | Fixed by forward upgrade (`cargo update -p time --precise 0.3.47`). DoS via stack exhaustion. No longer reported. | A2 |
| RUSTSEC-2026-0186 | `memmap2` 0.9.10 → **0.9.11** | Fixed by forward upgrade (`cargo update -p memmap2 --precise 0.9.11`). Unchecked pointer offset warning. No longer reported. | envctl-owned-rust-toolchain-cache |

## Accepted risk (no forward upgrade path)

Both entries below are **unmaintained warnings, not vulnerabilities** (no CVSS
severity; `cargo audit` classifies them `Warning: unmaintained`). They are pulled
in **transitively** and there is **no forward upgrade path** as of 2026-06-06:

- They both come exclusively from **`syntect` 5.3.0**, which is the **latest**
  published version (crates.io) and still depends on `bincode 1.x` + `yaml-rust 0.4`
  (used to load/serialize its syntax-definition sets).
- `syntect` reaches the tree via two latest-version consumers:
  - `comrak` 0.52.0 (latest) → `crates/spec` (CommonMark parse/emit), and
  - `tui-markdown` 0.3.7 (latest), pulled with **`features = ["highlight-code"]`** →
    `crates/tui`. That feature **is** the TUI's markdown code-block syntax-highlighting
    capability. Dropping `syntect` would remove that capability — a **downgrade**,
    which the invariant forbids.

Therefore these cannot be removed without either (a) `syntect` upstream dropping
them, or (b) deleting a shipped capability. Neither is acceptable today, so they are
accepted-risk and ignored in the shared baseline (warnings-only; the fail-closed
gate still fails on any **new** advisory or any **vulnerability**).

| Advisory | Crate | Why ignored | Re-evaluate when |
|----------|-------|-------------|------------------|
| RUSTSEC-2025-0141 | `bincode` 1.3.3 (via `syntect`) | unmaintained-only; required transitively by `syntect` (latest), which backs the TUI `highlight-code` capability and `comrak` | a `syntect` release drops `bincode 1.x`, or `comrak`/`tui-markdown` move off `syntect` |
| RUSTSEC-2024-0320 | `yaml-rust` 0.4.5 (via `syntect`) | unmaintained-only; same `syntect` dependency as above | a `syntect` release moves to `yaml-rust2`/`saphyr`, or the markdown stack drops `syntect` |

**Periodic check:** when bumping `syntect`/`comrak`/`tui-markdown`, re-run
`cargo audit` and, if either advisory clears, remove its line from
`.cargo/audit.toml` and move the row here to *Remediated*.
