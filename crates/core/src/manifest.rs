use crate::fs_utils::{relative_path, stable_walk, write_string_preserving_existing};
use crate::model::ManifestEntry;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub fn generate_manifest(root: impl AsRef<Path>) -> Result<Vec<ManifestEntry>, String> {
    let root = root.as_ref();
    if !root.exists() || !root.is_dir() {
        return Err(format!(
            "manifest root is not a directory: {}",
            root.display()
        ));
    }

    let mut entries = Vec::new();
    for abs in stable_walk(root).map_err(|e| format!("walk failed: {e}"))? {
        let rel = relative_path(root, &abs);
        if manifest_should_skip(&rel) {
            continue;
        }
        let metadata = match fs::metadata(&abs) {
            Ok(value) => value,
            Err(_) => continue,
        };
        if !metadata.is_file() {
            continue;
        }
        let digest =
            fnv1a64_file(&abs).map_err(|e| format!("hash failed for {}: {e}", abs.display()))?;
        entries.push(ManifestEntry {
            path: rel,
            size_bytes: metadata.len(),
            fnv1a64: format!("{digest:016x}"),
        });
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

pub fn manifest_tsv(entries: &[ManifestEntry]) -> String {
    let mut out = String::from("path\tsize_bytes\tfnv1a64\n");
    for entry in entries {
        out.push_str(&format!(
            "{}\t{}\t{}\n",
            entry.path, entry.size_bytes, entry.fnv1a64
        ));
    }
    out
}

pub fn write_manifest(
    root: impl AsRef<Path>,
    out: impl AsRef<Path>,
) -> Result<Vec<ManifestEntry>, String> {
    let entries = generate_manifest(root)?;
    write_string_preserving_existing(out.as_ref(), &manifest_tsv(&entries))
        .map_err(|e| format!("failed to write manifest: {e}"))?;
    Ok(entries)
}

pub fn workspace_fingerprint(root: impl AsRef<Path>) -> Result<String, String> {
    let root = root.as_ref();
    if !root.exists() || !root.is_dir() {
        return Err(format!(
            "fingerprint root is not a directory: {}",
            root.display()
        ));
    }

    let mut hash: u64 = 0xcbf29ce484222325;
    for abs in stable_walk(root).map_err(|e| format!("walk failed: {e}"))? {
        let rel = relative_path(root, &abs);
        if fingerprint_should_skip(&rel) {
            continue;
        }
        for byte in rel.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
        let file_hash =
            fnv1a64_file(&abs).map_err(|e| format!("hash failed for {}: {e}", abs.display()))?;
        for byte in file_hash.to_be_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

/// Runtime state (gitignored, machine-local) must never enter EITHER generated
/// surface — the fingerprint OR the manifest. A dev tree carrying these mints
/// artifacts no clean CI checkout can reproduce: phantom "knowledge is stale"
/// criticals and manifest-refresh diffs (both hit on the fork-unification
/// PR #143 — locks/ledgers in one round, then again in the committed manifest).
fn runtime_state_should_skip(rel: &str) -> bool {
    rel.starts_with(".handoff/ledger.db")
        || rel.starts_with(".handoff/locks/")
        || rel.starts_with(".idea/")
        || rel.starts_with(".kb/.cache/")
        || rel.starts_with(".kb/workspaces/")
}

fn fingerprint_should_skip(rel: &str) -> bool {
    rel.starts_with(".idd/knowledge/")
        || rel.starts_with(".idd/runs/")
        || rel == ".idd/MANIFEST.tsv"
        || rel == "AI_MERGE/validation_report.md"
        || rel == "docs/rusty-idd/architecture-diagrams.md"
        || runtime_state_should_skip(rel)
        || is_upstream_generated_local_artifact(rel)
}

fn manifest_should_skip(rel: &str) -> bool {
    rel.starts_with(".idd/runs/")
        || runtime_state_should_skip(rel)
        || is_upstream_generated_local_artifact(rel)
}

fn is_upstream_generated_local_artifact(rel: &str) -> bool {
    rel.starts_with("third_party/upstream/codegraph-rust/docs/specifications/")
        || rel == "third_party/upstream/repomix-rs/.mind-mesh/agent/repomix.md"
        // The vendored repomix test suite writes repomix-output.* fixtures into
        // its own src dir when `cargo test --workspace` runs before validation.
        || rel.starts_with("third_party/upstream/repomix-rs/crates/core/src/repomix-output.")
}

fn fnv1a64_file(path: &Path) -> io::Result<u64> {
    let mut file = fs::File::open(path)?;
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut buffer = [0_u8; 8192];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        for byte in &buffer[..read] {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
    }
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_tsv_has_header() {
        let out = manifest_tsv(&[]);
        assert_eq!(out, "path\tsize_bytes\tfnv1a64\n");
    }

    #[test]
    fn manifest_excludes_local_execution_artifacts() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        fs::create_dir(root.join(".idd")).unwrap();
        fs::write(root.join(".idd/MANIFEST.tsv"), "old self hash\n").unwrap();
        fs::create_dir_all(root.join(".idd/runs/rusty-idd-codex-loop")).unwrap();
        fs::write(
            root.join(".idd/runs/rusty-idd-codex-loop/run-manifest.json"),
            "{}\n",
        )
        .unwrap();
        fs::write(root.join("Cargo.toml.idd-bak-1"), "stale\n").unwrap();
        fs::create_dir(root.join("_workspace")).unwrap();
        fs::write(root.join("_workspace/HANDOFF.md"), "local\n").unwrap();
        fs::create_dir(root.join(".devin")).unwrap();
        fs::write(root.join(".devin/config.local.json"), "{}\n").unwrap();
        fs::create_dir(root.join(".worktrees")).unwrap();
        fs::write(root.join(".worktrees/branch.txt"), "local\n").unwrap();

        let paths = generate_manifest(root)
            .unwrap()
            .into_iter()
            .map(|entry| entry.path)
            .collect::<Vec<_>>();

        assert_eq!(paths, vec!["Cargo.toml"]);
    }

    #[test]
    fn manifest_excludes_ignored_upstream_generated_artifacts() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        fs::create_dir_all(root.join("third_party/upstream/codegraph-rust/docs/specifications"))
            .unwrap();
        fs::create_dir_all(root.join("third_party/upstream/repomix-rs/.mind-mesh/agent")).unwrap();
        fs::write(
            root.join("third_party/upstream/codegraph-rust/docs/specifications/local.spec.md"),
            "generated\n",
        )
        .unwrap();
        fs::write(
            root.join("third_party/upstream/repomix-rs/.mind-mesh/agent/repomix.md"),
            "generated\n",
        )
        .unwrap();
        fs::write(
            root.join("third_party/upstream/repomix-rs/Cargo.toml"),
            "[workspace]\n",
        )
        .unwrap();

        let paths = generate_manifest(root)
            .unwrap()
            .into_iter()
            .map(|entry| entry.path)
            .collect::<Vec<_>>();

        assert_eq!(
            paths,
            vec!["Cargo.toml", "third_party/upstream/repomix-rs/Cargo.toml"]
        );
    }

    #[test]
    fn write_manifest_is_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir(root.join(".idd")).unwrap();
        fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        let manifest = root.join(".idd/MANIFEST.tsv");

        write_manifest(root, &manifest).unwrap();
        let first = fs::read_to_string(&manifest).unwrap();
        write_manifest(root, &manifest).unwrap();
        let second = fs::read_to_string(&manifest).unwrap();

        assert_eq!(first, second);
        assert!(!first.contains(".idd/MANIFEST.tsv"));
    }

    #[test]
    fn workspace_fingerprint_ignores_generated_knowledge() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir(root.join(".idd")).unwrap();
        fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
        let first = workspace_fingerprint(root).unwrap();

        fs::create_dir(root.join(".idd/knowledge")).unwrap();
        fs::write(root.join(".idd/knowledge/index.json"), "{}\n").unwrap();
        fs::write(root.join(".idd/knowledge/report.md"), "# Report\n").unwrap();
        fs::create_dir_all(root.join(".idd/runs/rusty-idd-codex-loop")).unwrap();
        fs::write(
            root.join(".idd/runs/rusty-idd-codex-loop/run-manifest.json"),
            "{}\n",
        )
        .unwrap();
        fs::create_dir(root.join("AI_MERGE")).unwrap();
        fs::write(root.join("AI_MERGE/validation_report.md"), "# Validation\n").unwrap();
        fs::create_dir_all(root.join("third_party/upstream/codegraph-rust/docs/specifications"))
            .unwrap();
        fs::write(
            root.join("third_party/upstream/codegraph-rust/docs/specifications/local.spec.md"),
            "generated\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("docs/rusty-idd")).unwrap();
        fs::write(
            root.join("docs/rusty-idd/architecture-diagrams.md"),
            "generated diagrams\n",
        )
        .unwrap();
        let second = workspace_fingerprint(root).unwrap();

        assert_eq!(first, second);
    }
}
