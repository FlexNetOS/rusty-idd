//! `rusty-idd deploy` — install the thin-adapter control-plane surface into a
//! *target* fleet repo (ADR-0017), so the whole fleet presents one minimal agent
//! harness backed by the one engine.
//!
//! `render` keeps the *home* repo's vendor dirs thin; `deploy` extends that to a
//! peer repo: it writes the byte-identical adapter (reusing
//! [`super::render::expected_adapter`] and the shared `VENDORS` set) plus a
//! SessionStart hook that calls the front door (`rusty-idd next`). It is strictly
//! additive — it never modifies or deletes the target repo's own forge loop,
//! runtime, build, source, or generated artifacts; existing hook entries are
//! preserved. `deploy --check`/`--dry-run` is an idempotent, fail-closed drift
//! gate that writes nothing.

use std::path::{Path, PathBuf};

use clap::Args;
use serde_json::{json, Value};

use super::render::{expected_adapter, lookup, ADAPTER_FILE, VENDORS};

/// The SessionStart hook command deployed into peer repos: it resolves the repo
/// root at runtime and calls the `rusty-idd` binary on PATH (a peer repo is not
/// the rusty-idd cargo workspace, so it cannot `cargo run` it). Move-safe and
/// target-agnostic. "Deploy the full package" implies `rusty-idd` is on PATH
/// across the fleet (the same model as `hf`).
const DEPLOY_HOOK_CMD: &str =
    "sh -lc 'root=\"$(git rev-parse --show-toplevel)\"; exec rusty-idd next --base \"$root\"'";

/// Substring shared by every Rusty IDD front-door SessionStart hook — both the
/// home repo's `cargo run ... -- next --base "$root"` and the deployed
/// `exec rusty-idd next --base "$root"`. Used for *semantic* hook-presence
/// detection so deploy is idempotent and never disturbs an already-wired repo
/// (e.g. the home repo's cargo-run hook), regardless of JSON key ordering.
const FRONT_DOOR_MARKER: &str = "next --base";

/// Args for `rusty-idd deploy`.
#[derive(Args)]
pub struct DeployArgs {
    /// Target repo root to deploy the thin-adapter surface into.
    #[arg(long, default_value = ".")]
    target: PathBuf,
    /// Deploy a single vendor by name (claude|codex|agents|devin).
    #[arg(long, conflicts_with = "all")]
    vendor: Option<String>,
    /// Deploy every known vendor that has a directory under the target (default).
    #[arg(long)]
    all: bool,
    /// Do not write; exit non-zero if the target is missing or drifted.
    #[arg(long)]
    check: bool,
    /// Alias for `--check`: report drift without writing.
    #[arg(long)]
    dry_run: bool,
}

/// The vendor config file (relative to the target root) that carries SessionStart
/// hooks, for hook-capable vendors. `agents`/`devin` have no defined hook runtime
/// and receive the adapter doc only.
fn hook_config_rel(vendor: &str) -> Option<&'static str> {
    match vendor {
        "codex" => Some(".codex/hooks.json"),
        "claude" => Some(".claude/settings.json"),
        _ => None,
    }
}

/// `rusty-idd deploy` — write or check the thin-adapter surface in a target repo.
pub fn run(args: DeployArgs) -> i32 {
    let check = args.check || args.dry_run;

    if !args.target.is_dir() {
        eprintln!(
            "rusty-idd deploy: target repo root not found: {}",
            args.target.display()
        );
        return 1;
    }

    let targets: Vec<(&str, &str)> = match &args.vendor {
        Some(v) => match lookup(v) {
            Some(t) => vec![t],
            None => {
                eprintln!(
                    "rusty-idd deploy: unknown vendor '{v}'; known: claude, codex, agents, devin"
                );
                return 2;
            }
        },
        // default / --all: every known vendor that already has a dir in the target
        // (never create unsolicited surfaces, mirroring `render --all`).
        None => VENDORS
            .iter()
            .copied()
            .filter(|(_, dir)| args.target.join(dir).is_dir())
            .collect(),
    };

    if targets.is_empty() {
        println!(
            "rusty-idd deploy: no known vendor directories under {}",
            args.target.display()
        );
        return 0;
    }

    if check {
        run_check(&args, &targets)
    } else {
        run_write(&args, &targets)
    }
}

fn run_check(args: &DeployArgs, targets: &[(&str, &str)]) -> i32 {
    let mut drift: Vec<String> = Vec::new();
    for (name, dir) in targets {
        // Adapter: missing or byte-different from the engine output.
        let adapter_path = args.target.join(dir).join(ADAPTER_FILE);
        let expected = expected_adapter(name);
        match std::fs::read_to_string(&adapter_path) {
            Ok(actual) if actual == expected => {}
            Ok(_) => drift.push(format!("drifted adapter: {}", adapter_path.display())),
            Err(_) => drift.push(format!("missing adapter: {}", adapter_path.display())),
        }
        // Hook (hook-capable vendors only): the front-door SessionStart entry must
        // be present (semantic check).
        if let Some(rel) = hook_config_rel(name) {
            let cfg_path = args.target.join(rel);
            match read_json(&cfg_path) {
                Ok(Some(value)) if front_door_hook_present(&value) => {}
                Ok(Some(_)) => {
                    drift.push(format!("missing front-door hook: {}", cfg_path.display()))
                }
                Ok(None) => drift.push(format!("missing hook config: {}", cfg_path.display())),
                Err(e) => drift.push(format!(
                    "unreadable hook config {}: {e}",
                    cfg_path.display()
                )),
            }
        }
    }
    if drift.is_empty() {
        println!(
            "rusty-idd deploy --check: target {} in sync ({} vendor(s))",
            args.target.display(),
            targets.len()
        );
        0
    } else {
        eprintln!(
            "rusty-idd deploy --check: target {} out of sync:",
            args.target.display()
        );
        for d in &drift {
            eprintln!("  {d}");
        }
        eprintln!("  fix: rusty-idd deploy --target {}", args.target.display());
        1
    }
}

fn run_write(args: &DeployArgs, targets: &[(&str, &str)]) -> i32 {
    for (name, dir) in targets {
        let vdir = args.target.join(dir);
        // An explicitly named vendor may create its dir; --all only fills dirs
        // that already exist (filtered above).
        if args.vendor.is_some() {
            if let Err(e) = std::fs::create_dir_all(&vdir) {
                eprintln!("rusty-idd deploy: failed to create {}: {e}", vdir.display());
                return 1;
            }
        }
        // 1. Adapter doc (idempotent, byte-identical to `render`).
        let adapter_path = vdir.join(ADAPTER_FILE);
        let expected = expected_adapter(name);
        if std::fs::read_to_string(&adapter_path).ok().as_deref() != Some(expected.as_str()) {
            if let Err(e) = std::fs::write(&adapter_path, &expected) {
                eprintln!(
                    "rusty-idd deploy: failed to write {}: {e}",
                    adapter_path.display()
                );
                return 1;
            }
        }
        // 2. SessionStart hook (hook-capable vendors only), merged additively.
        let mut hook_note = "";
        if let Some(rel) = hook_config_rel(name) {
            let cfg_path = args.target.join(rel);
            match ensure_front_door_hook_file(&cfg_path) {
                Ok(true) => hook_note = " + hook",
                Ok(false) => hook_note = " (hook present)",
                Err(e) => {
                    eprintln!(
                        "rusty-idd deploy: failed to update hook config {}: {e}",
                        cfg_path.display()
                    );
                    return 1;
                }
            }
        }
        println!("deployed: {}{}", adapter_path.display(), hook_note);
    }
    0
}

/// Read a JSON file into a `Value`. `Ok(None)` = file absent; `Err` = present but
/// unreadable/unparsable (never silently clobbered).
fn read_json(path: &Path) -> Result<Option<Value>, String> {
    match std::fs::read_to_string(path) {
        Ok(src) => serde_json::from_str(&src)
            .map(Some)
            .map_err(|e| e.to_string()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

/// Whether `hooks.SessionStart` already contains a Rusty IDD front-door hook
/// (matched by the shared `next --base` marker, so the home repo's cargo-run hook
/// and the deployed PATH-binary hook both count).
fn front_door_hook_present(value: &Value) -> bool {
    value
        .get("hooks")
        .and_then(|h| h.get("SessionStart"))
        .and_then(|s| s.as_array())
        .map(|entries| entries.iter().any(entry_has_front_door))
        .unwrap_or(false)
}

fn entry_has_front_door(entry: &Value) -> bool {
    entry
        .get("hooks")
        .and_then(|h| h.as_array())
        .map(|hooks| {
            hooks.iter().any(|h| {
                h.get("command")
                    .and_then(|c| c.as_str())
                    .map(|c| c.contains(FRONT_DOOR_MARKER))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// The canonical SessionStart entry deployed into peer repos.
fn canonical_hook_entry() -> Value {
    json!({
        "hooks": [
            {
                "type": "command",
                "command": DEPLOY_HOOK_CMD,
                "timeout": 180,
                "statusMessage": "Rusty IDD front door: computing the next step"
            }
        ]
    })
}

/// Ensure the target config file at `path` carries the front-door SessionStart
/// hook. Returns `Ok(true)` if the file was created or modified, `Ok(false)` if
/// it already had a front-door hook (no write). Preserves every other key and
/// hook phase; appends only.
fn ensure_front_door_hook_file(path: &Path) -> Result<bool, String> {
    let mut value = read_json(path)?.unwrap_or_else(|| json!({}));
    if ensure_front_door_hook(&mut value)? {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut out = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
        out.push('\n');
        std::fs::write(path, out).map_err(|e| e.to_string())?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Append the front-door SessionStart entry to `value` iff absent. Returns
/// `Ok(true)` if it mutated `value`. Errors if the existing `hooks`/`SessionStart`
/// shape is malformed (so a hand-broken config is never silently clobbered).
fn ensure_front_door_hook(value: &mut Value) -> Result<bool, String> {
    if front_door_hook_present(value) {
        return Ok(false);
    }
    let obj = value
        .as_object_mut()
        .ok_or("config root is not a JSON object")?;
    let hooks = obj.entry("hooks").or_insert_with(|| json!({}));
    let hooks_obj = hooks
        .as_object_mut()
        .ok_or("`hooks` is not a JSON object")?;
    let session_start = hooks_obj.entry("SessionStart").or_insert_with(|| json!([]));
    let arr = session_start
        .as_array_mut()
        .ok_or("`hooks.SessionStart` is not a JSON array")?;
    arr.push(canonical_hook_entry());
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deploy_uses_the_render_vendor_set_and_adapter() {
        // Deploy shares render's VENDORS + expected_adapter (same source of
        // truth): every known vendor resolves and its adapter points at the
        // front door.
        for (name, dir) in VENDORS {
            assert_eq!(lookup(name), Some((*name, *dir)));
            let adapter = expected_adapter(name);
            assert!(adapter.contains("rusty-idd next"));
            assert!(adapter.contains("THIN ADAPTER"));
        }
    }

    #[test]
    fn hook_config_rel_only_for_hook_capable_vendors() {
        assert_eq!(hook_config_rel("codex"), Some(".codex/hooks.json"));
        assert_eq!(hook_config_rel("claude"), Some(".claude/settings.json"));
        assert_eq!(hook_config_rel("agents"), None);
        assert_eq!(hook_config_rel("devin"), None);
    }

    #[test]
    fn front_door_present_matches_both_cargo_run_and_path_forms() {
        let cargo_run = json!({
            "hooks": { "SessionStart": [ { "hooks": [ {
                "type": "command",
                "command": "sh -lc 'root=\"$(git rev-parse --show-toplevel)\"; exec cargo run --quiet --manifest-path \"$root/Cargo.toml\" --bin rusty-idd -- next --base \"$root\"'"
            } ] } ] }
        });
        let path_form = json!({
            "hooks": { "SessionStart": [ canonical_hook_entry() ] }
        });
        assert!(front_door_hook_present(&cargo_run));
        assert!(front_door_hook_present(&path_form));
    }

    #[test]
    fn front_door_absent_when_only_other_phases() {
        let other = json!({
            "hooks": { "PreToolUse": [ { "hooks": [ {
                "type": "command", "command": "rusty-idd codex workflow-check --phase pre-tool"
            } ] } ] }
        });
        assert!(!front_door_hook_present(&other));
    }

    #[test]
    fn ensure_hook_appends_then_is_idempotent_and_preserves_keys() {
        let mut value = json!({
            "$comment": "keep me",
            "hooks": {
                "PreToolUse": [ { "hooks": [ { "type": "command", "command": "x" } ] } ],
                "SessionStart": []
            }
        });
        // First call appends.
        assert!(ensure_front_door_hook(&mut value).unwrap());
        assert!(front_door_hook_present(&value));
        // Other keys/phases preserved.
        assert_eq!(value["$comment"], "keep me");
        assert!(value["hooks"]["PreToolUse"].as_array().unwrap().len() == 1);
        // Second call is a no-op.
        assert!(!ensure_front_door_hook(&mut value).unwrap());
        assert_eq!(
            value["hooks"]["SessionStart"].as_array().unwrap().len(),
            1,
            "no duplicate SessionStart entry"
        );
    }

    #[test]
    fn ensure_hook_creates_structure_from_empty() {
        let mut value = json!({});
        assert!(ensure_front_door_hook(&mut value).unwrap());
        assert!(front_door_hook_present(&value));
    }

    #[test]
    fn ensure_hook_errors_on_malformed_shape() {
        let mut value = json!({ "hooks": { "SessionStart": "not-an-array" } });
        assert!(ensure_front_door_hook(&mut value).is_err());
    }
}
