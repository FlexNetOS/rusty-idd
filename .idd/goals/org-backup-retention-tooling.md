# Goal: route safe-write backups into the never-delete store (tooling layer)

Policy: `meta/META-BACKUP-POLICY.md` (META-ORG-POLICY.md P5.25). The sweeper
(`meta/scripts/idd-backup-sweep.sh`) already covers the gap by relocating loose
`*.idd-bak-*` into the compressed, append-only, out-of-tree store. This goal is the
**durable tool-layer fix** so backups never land loose in the first place.

## Intent

Patch the safe-write path in `crates/core/src/fs_utils.rs` (`next_backup_path` and its
caller) so that, instead of writing an unbounded `<file>.idd-bak-N` next to the live file,
a pre-overwrite snapshot is appended directly into the repo's backup store
(`meta/.backups/<repo>/idd-backups.tar.zst` + `index.tsv`), content-addressed by SHA-256.

The **identical** `next_backup_path` exists in `handoff/crates/core/src/fs_utils.rs`
(the `hf` kernel); both must be fixed, each as its own per-repo PR (CI-gated shared crates).

## Required method

- Keep behavior fail-closed: if the store is unreachable, fall back to the current loose
  `.idd-bak-N` write (never lose a backup), then let the sweeper roll it in later.
- No content ever deleted; never weaken the existing safe-write guarantee.
- Resolve the store root the same way the sweeper does (walk up to `.meta.yaml`;
  honor `STORE_ROOT`); when no meta root is found, retain loose-file behavior.
- Parity test: byte-identical snapshot is recoverable from the store after overwrite.
- Per-repo branch + PR; green fmt/clippy/test/audit; update `META-BACKUP-POLICY.md`
  "Automation → Tool" status when landed.

## Non-goals

- Do not change the sweeper contract.
- Do not commit the store or any backup into any repo's git history.
- No new runtime deps beyond what `zstd`/hashing already require (prefer reusing the
  sweeper, or a thin shell-out, over a heavy crate dependency in `core`).
