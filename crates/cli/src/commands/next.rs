//! `rusty-idd next` — the harness control-plane FRONT DOOR (ADR-0015).
//!
//! One deterministic imperative for "what do I do right now", computed from the
//! active change's artifact DAG — not a static prose harness re-read every
//! session. Vendor surfaces (`.claude`, `.codex`, `.agents`, …) are thin
//! adapters that call this instead of carrying their own always-loaded workflow
//! (ADR-0001 flow + ADR-0002 thin front door + ADR-0010 stage packages).
//!
//! State precedence: the active change is read from `.idd/workflow/active-change`
//! (a single change name, e.g. `harness-control-plane`), resolved against
//! `openspec/changes/<name>`. The artifact DAG / `next ready` computation is
//! reused verbatim from [`crate::commands::spec_status`] so there is exactly one
//! oracle, not two.

use std::path::{Path, PathBuf};

use clap::Args;
use serde::Serialize;

/// Args for `rusty-idd next`.
#[derive(Args)]
pub struct NextArgs {
    /// Base directory containing `.idd/` and `openspec/` (defaults to the
    /// current directory).
    #[arg(long, default_value = ".")]
    base: PathBuf,
    /// Emit a deterministic JSON object for non-interactive adapters instead of
    /// the human-readable imperative.
    #[arg(long)]
    json: bool,
}

/// Machine-readable front-door view (`rusty-idd next --json`). Embeds the shared
/// `spec_status` snapshot so it cannot disagree with `spec status --json`.
#[derive(Serialize)]
struct FrontDoorView {
    active_change: Option<String>,
    change: Option<crate::commands::spec_status::StatusSnapshot>,
    next_command: Option<String>,
}

/// The one scoped next command for a change, given its next ready artifact.
fn next_command_for(active: &str, next_id: Option<&str>) -> String {
    match next_id {
        Some(id) => format!("rusty-idd spec scaffold {id} --change {active}"),
        None => format!("rusty-idd spec archive openspec/changes/{active} --yes"),
    }
}

/// Read the active change name from `<base>/.idd/workflow/active-change`.
/// Returns `None` when the pointer is absent or empty (whitespace-only).
pub(crate) fn resolve_active_change(base: &Path) -> Option<String> {
    let pointer = base.join(".idd/workflow/active-change");
    std::fs::read_to_string(pointer)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// `rusty-idd next` — print the single next imperative for the active change.
pub fn run(args: NextArgs) -> i32 {
    if args.json {
        return run_json(&args.base);
    }
    run_text(&args.base)
}

/// Machine-readable mode: one deterministic JSON object for adapters. Fails
/// closed (non-zero, no stdout JSON) on a dangling active-change pointer.
fn run_json(base: &Path) -> i32 {
    let Some(active) = resolve_active_change(base) else {
        let view = FrontDoorView {
            active_change: None,
            change: None,
            next_command: None,
        };
        println!("{}", serde_json::to_string_pretty(&view).unwrap());
        return 0;
    };

    let change_dir = base.join("openspec/changes").join(&active);
    if !change_dir.is_dir() {
        eprintln!(
            "rusty-idd next: active change '{active}' has no directory at {}",
            change_dir.display()
        );
        return 1;
    }

    let snapshot = match crate::commands::spec_status::snapshot_for(&change_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("rusty-idd next: {e}");
            return 1;
        }
    };
    let next_id = crate::commands::spec_status::next_artifact_id(&change_dir);
    let view = FrontDoorView {
        active_change: Some(active.clone()),
        change: Some(snapshot),
        next_command: Some(next_command_for(&active, next_id.as_deref())),
    };
    match serde_json::to_string_pretty(&view) {
        Ok(text) => {
            println!("{text}");
            0
        }
        Err(e) => {
            eprintln!("rusty-idd next: failed to serialize JSON: {e}");
            1
        }
    }
}

/// Human-readable mode: the single next imperative for the active change.
fn run_text(base: &Path) -> i32 {
    let Some(active) = resolve_active_change(base) else {
        println!("rusty-idd next — no active change.");
        println!("  set one:   echo <change> > .idd/workflow/active-change");
        println!("  or create: rusty-idd spec new <change>");
        return 0;
    };

    let change_dir = base.join("openspec/changes").join(&active);
    if !change_dir.is_dir() {
        eprintln!(
            "rusty-idd next: active change '{active}' has no directory at {}",
            change_dir.display()
        );
        eprintln!("  fix the pointer at .idd/workflow/active-change, or run `rusty-idd spec new {active}`.");
        return 1;
    }

    println!("rusty-idd — harness control-plane front door (ADR-0015)");
    println!("Active change: {active}");
    println!();

    // One oracle: delegate the full artifact-DAG status (prints `Next: <id>`).
    let code = crate::commands::spec_status::run_status(&change_dir, false);
    if code != 0 {
        return code;
    }

    // Token-cheap, step-scoped next action — one command, not a prose dump.
    println!();
    match crate::commands::spec_status::next_artifact_id(&change_dir) {
        Some(id) => {
            println!("Do this now:");
            println!("  rusty-idd spec scaffold {id} --change {active}   # stub the next artifact, then author it");
            println!("  rusty-idd next                                   # re-run to advance");
        }
        None => {
            println!("All artifacts present — review, then:");
            println!(
                "  rusty-idd spec archive openspec/changes/{active} --yes   # merge delta into base specs"
            );
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(path: &Path, contents: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, contents).unwrap();
    }

    #[test]
    fn resolve_active_change_trims_and_filters_empty() {
        let root = tempfile::tempdir().unwrap();
        // absent pointer -> None
        assert_eq!(resolve_active_change(root.path()), None);
        // whitespace-only -> None
        write(&root.path().join(".idd/workflow/active-change"), "  \n");
        assert_eq!(resolve_active_change(root.path()), None);
        // real value -> trimmed Some
        write(
            &root.path().join(".idd/workflow/active-change"),
            "harness-control-plane\n",
        );
        assert_eq!(
            resolve_active_change(root.path()).as_deref(),
            Some("harness-control-plane")
        );
    }
}
