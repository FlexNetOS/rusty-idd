# Upstream Full Mirrors

This directory stores exact tracked-file snapshots of upstream repositories used
for Rusty IDD knowledge integration. The mirrors are intentionally not Cargo
workspace members. They are the adoption baseline for audits, diffs, rollback,
and future consolidation work.

## Pins

| Upstream | URL | Ref | Tracked files | Local mirror |
| --- | --- | --- | ---: | --- |
| CodeGraph Rust | https://github.com/Jakedismo/codegraph-rust | `ce5bf27a2978983a9089d177447f296e4c6521bb` | 369 | `third_party/upstream/codegraph-rust` |
| repomix-rs | https://github.com/sopaco/repomix-rs | `946df10d48c669ca3a99f757ffd2c6fa35844e62` | 133 | `third_party/upstream/repomix-rs` |

## Import Method

Both mirrors were imported from clean detached checkouts with `git archive`.
Generated build artifacts and `.git` directories are excluded; tracked dotfiles,
CI, scripts, docs, tests, fixtures, configs, examples, workspace manifests, and
package metadata are included.

## Local Boundary

Rusty IDD default builds keep using the consolidated crates in
`crates/external/` plus the public Repomix crates where compatible. The full
mirrors are preserved as the canonical upstream reference before any cut is
accepted. Every local cut must cite the native upstream diagnostic that forced
it and the rollback path back to this mirror.
