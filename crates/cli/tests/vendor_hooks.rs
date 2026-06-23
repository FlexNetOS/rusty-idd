//! Integration tests for the vendor session-start wiring (ADR-0015 front door,
//! harness-session-frontdoor change). These assert the *real* repo config files
//! parse as JSON and that their `SessionStart` hook invokes `rusty-idd next` —
//! so the front door is actually called at session start, not just pointed at.

use std::path::PathBuf;

use serde_json::Value;

/// Repository root (two levels up from this crate: `<root>/crates/cli`).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .to_path_buf()
}

fn load_json(rel: &str) -> Value {
    let path = repo_root().join(rel);
    let src =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    serde_json::from_str(&src).unwrap_or_else(|e| panic!("parse {}: {e}", path.display()))
}

/// Collect every `command` string under a hooks-config `SessionStart` array.
fn session_start_commands(cfg: &Value) -> Vec<String> {
    let mut cmds = Vec::new();
    let Some(groups) = cfg
        .get("hooks")
        .and_then(|h| h.get("SessionStart"))
        .and_then(|s| s.as_array())
    else {
        return cmds;
    };
    for group in groups {
        let Some(inner) = group.get("hooks").and_then(|h| h.as_array()) else {
            continue;
        };
        for hook in inner {
            if let Some(cmd) = hook.get("command").and_then(|c| c.as_str()) {
                cmds.push(cmd.to_string());
            }
        }
    }
    cmds
}

fn invokes_next(cmds: &[String]) -> bool {
    cmds.iter()
        .any(|c| c.contains("rusty-idd -- next") || c.contains("rusty-idd next"))
}

#[test]
fn codex_session_start_calls_front_door() {
    let cfg = load_json(".codex/hooks.json");
    let cmds = session_start_commands(&cfg);
    assert!(
        !cmds.is_empty(),
        ".codex/hooks.json has no SessionStart hook"
    );
    assert!(
        invokes_next(&cmds),
        ".codex SessionStart does not invoke `rusty-idd next`: {cmds:?}"
    );
}

#[test]
fn codex_keeps_existing_hooks() {
    // The front-door wiring must be additive — the workflow-check gates stay.
    let cfg = load_json(".codex/hooks.json");
    let hooks = cfg.get("hooks").expect("hooks");
    for key in ["PreToolUse", "PostToolUse", "Stop", "SubagentStop"] {
        assert!(hooks.get(key).is_some(), ".codex/hooks.json lost `{key}`");
    }
}

#[test]
fn claude_session_start_calls_front_door() {
    let cfg = load_json(".claude/settings.json");
    let cmds = session_start_commands(&cfg);
    assert!(
        !cmds.is_empty(),
        ".claude/settings.json has no SessionStart hook"
    );
    assert!(
        invokes_next(&cmds),
        ".claude SessionStart does not invoke `rusty-idd next`: {cmds:?}"
    );
}
