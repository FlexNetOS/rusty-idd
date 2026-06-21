//! Integration tests for `rusty-idd spec scaffold` / `spec new` — the scaffold
//! (minijinja) surface, exercised through the compiled binary.

use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

#[test]
fn scaffold_proposal_injects_change_and_has_no_jinja() {
    let out = Command::new(bin())
        .args(["spec", "scaffold", "proposal", "--change", "add-json"])
        .output()
        .expect("run rusty-idd");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.starts_with("# add-json\n"), "{text}");
    assert!(text.contains("## Why") && text.contains("## What Changes"));
    assert!(
        !text.contains("{{") && !text.contains("}}"),
        "no unrendered vars"
    );
}

#[test]
fn scaffold_adr_injects_number_title_date() {
    let out = Command::new(bin())
        .args([
            "spec",
            "scaffold",
            "adr",
            "--number",
            "0009",
            "--title",
            "Use queues",
            "--date",
            "2026-06-04",
        ])
        .output()
        .expect("run rusty-idd");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.starts_with("# 0009. Use queues\n"), "{text}");
    assert!(text.contains("- Date: 2026-06-04"), "{text}");
}

#[test]
fn scaffold_unknown_artifact_errors() {
    let out = Command::new(bin())
        .args(["spec", "scaffold", "nope"])
        .output()
        .expect("run rusty-idd");
    assert_eq!(out.status.code(), Some(1));
}

#[test]
fn new_creates_proposal_and_refuses_overwrite() {
    let base = tempfile::tempdir().unwrap();
    let out = Command::new(bin())
        .args(["spec", "new", "add-export", "--base"])
        .arg(base.path())
        .output()
        .expect("run rusty-idd");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    let proposal = base.path().join("openspec/changes/add-export/proposal.md");
    assert!(proposal.is_file());
    let body = std::fs::read_to_string(&proposal).unwrap();
    assert!(body.starts_with("# add-export\n"), "{body}");

    // Re-running refuses to overwrite.
    let again = Command::new(bin())
        .args(["spec", "new", "add-export", "--base"])
        .arg(base.path())
        .output()
        .expect("run rusty-idd");
    assert_eq!(again.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&again.stderr).contains("refusing to overwrite"));
}

/// The scaffolded `spec` delta stub must parse via the engine and `spec show`.
#[test]
fn scaffolded_spec_stub_is_show_able() {
    let dir = tempfile::tempdir().unwrap();
    let spec_path = dir.path().join("spec.md");
    let out = Command::new(bin())
        .args(["spec", "scaffold", "spec"])
        .output()
        .expect("run rusty-idd");
    assert!(out.status.success());
    std::fs::write(&spec_path, &out.stdout).unwrap();

    let show = Command::new(bin())
        .args(["spec", "show"])
        .arg(&spec_path)
        .output()
        .expect("run rusty-idd");
    assert!(
        show.status.success(),
        "spec show must handle a scaffolded stub"
    );
}

#[test]
fn plan_integration_creates_openspec_artifacts_from_integration_plan() {
    let base = tempfile::tempdir().unwrap();
    let plan_dir = base.path().join(".idd/knowledge");
    std::fs::create_dir_all(&plan_dir).unwrap();
    std::fs::write(
        plan_dir.join("integration-plan.json"),
        r#"{
  "schema_version": 1,
  "workspace_root": ".",
  "system_root": "..",
  "source_model": ".idd/knowledge/operating-model.json",
  "work_items": [
    {
      "id": "work:integrate-fleet-handoff",
      "title": "Integrate Central and fleet handoff",
      "capability": "capability:fleet-handoff",
      "layer": "layer:coordination-communication",
      "priority": 20,
      "status": "partial",
      "change_id": "integrate-fleet-handoff",
      "owner_repos": ["repo:handoff", "repo:weave"],
      "anchors": ["handoff central and fleet design"],
      "adopt_first_inputs": ["handoff central and fleet design"],
      "implementation_boundary": "Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input",
      "validation": ["cargo fmt --all -- --check", "just ci"],
      "rollback": ["Revert the integration slice", "Regenerate knowledge artifacts"]
    },
    {
      "id": "work:integrate-idd-spec-engine",
      "title": "Integrate IDD and spec engine",
      "capability": "capability:idd-spec-engine",
      "layer": "layer:executive-control-plane",
      "priority": 10,
      "status": "partial",
      "change_id": "integrate-idd-spec-engine",
      "owner_repos": ["repo:rusty-idd"],
      "anchors": [],
      "adopt_first_inputs": [],
      "implementation_boundary": "Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input",
      "validation": ["cargo fmt --all -- --check"],
      "rollback": ["Revert the integration slice"]
    }
  ],
  "gates": ["just ci"],
  "findings": []
}
"#,
    )
    .unwrap();

    let out = Command::new(bin())
        .args([
            "spec",
            "plan-integration",
            "--base",
            base.path().to_str().unwrap(),
            "--capability",
            "fleet-handoff",
        ])
        .output()
        .expect("run rusty-idd");
    assert!(
        out.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let change = base.path().join("openspec/changes/integrate-fleet-handoff");
    let proposal = std::fs::read_to_string(change.join("proposal.md")).unwrap();
    let design = std::fs::read_to_string(change.join("design.md")).unwrap();
    let tasks = std::fs::read_to_string(change.join("tasks.md")).unwrap();
    let spec = std::fs::read_to_string(change.join("specs/fleet-handoff/spec.md")).unwrap();

    assert!(proposal.contains("repo:handoff"), "{proposal}");
    assert!(
        proposal.contains("handoff central and fleet design"),
        "{proposal}"
    );
    assert!(design.contains("Implementation boundary"), "{design}");
    assert!(tasks.contains("cargo fmt --all -- --check"), "{tasks}");
    assert!(tasks.contains("Regenerate knowledge artifacts"), "{tasks}");
    assert!(spec.contains("### Requirement: Integrate Central and fleet handoff"));

    let show = Command::new(bin())
        .args(["spec", "show"])
        .arg(change.join("specs/fleet-handoff/spec.md"))
        .output()
        .expect("run rusty-idd");
    assert!(show.status.success(), "generated spec should be show-able");

    let again = Command::new(bin())
        .args([
            "spec",
            "plan-integration",
            "--base",
            base.path().to_str().unwrap(),
            "--capability",
            "fleet-handoff",
        ])
        .output()
        .expect("run rusty-idd");
    assert_eq!(again.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&again.stderr).contains("refusing to overwrite"));
}

#[test]
fn plan_integration_default_skips_existing_queue_slots() {
    let base = tempfile::tempdir().unwrap();
    write_two_item_integration_plan(base.path());
    std::fs::create_dir_all(
        base.path()
            .join("openspec/changes/integrate-idd-spec-engine"),
    )
    .unwrap();

    let out = Command::new(bin())
        .args([
            "spec",
            "plan-integration",
            "--base",
            base.path().to_str().unwrap(),
        ])
        .output()
        .expect("run rusty-idd");
    assert!(
        out.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(base
        .path()
        .join("openspec/changes/integrate-fleet-handoff/proposal.md")
        .is_file());
    assert!(!base
        .path()
        .join("openspec/changes/integrate-idd-spec-engine/specs")
        .exists());
}

#[test]
fn plan_integration_default_errors_when_no_planned_work_remains() {
    let base = tempfile::tempdir().unwrap();
    write_two_item_integration_plan(base.path());
    std::fs::create_dir_all(
        base.path()
            .join("openspec/changes/integrate-idd-spec-engine"),
    )
    .unwrap();
    std::fs::create_dir_all(
        base.path()
            .join("openspec/changes/archive/integrate-fleet-handoff"),
    )
    .unwrap();

    let out = Command::new(bin())
        .args([
            "spec",
            "plan-integration",
            "--base",
            base.path().to_str().unwrap(),
        ])
        .output()
        .expect("run rusty-idd");
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("no planned integration work items"));
}

fn write_two_item_integration_plan(base: &std::path::Path) {
    let plan_dir = base.join(".idd/knowledge");
    std::fs::create_dir_all(&plan_dir).unwrap();
    std::fs::write(
        plan_dir.join("integration-plan.json"),
        r#"{
  "schema_version": 1,
  "workspace_root": ".",
  "system_root": "..",
  "source_model": ".idd/knowledge/operating-model.json",
  "work_items": [
    {
      "id": "work:integrate-idd-spec-engine",
      "title": "Integrate IDD and spec engine",
      "capability": "capability:idd-spec-engine",
      "layer": "layer:executive-control-plane",
      "priority": 10,
      "status": "partial",
      "change_id": "integrate-idd-spec-engine",
      "owner_repos": ["repo:rusty-idd"],
      "anchors": [],
      "adopt_first_inputs": [],
      "implementation_boundary": "Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input",
      "validation": ["cargo fmt --all -- --check"],
      "rollback": ["Revert the integration slice"]
    },
    {
      "id": "work:integrate-fleet-handoff",
      "title": "Integrate Central and fleet handoff",
      "capability": "capability:fleet-handoff",
      "layer": "layer:coordination-communication",
      "priority": 20,
      "status": "partial",
      "change_id": "integrate-fleet-handoff",
      "owner_repos": ["repo:handoff", "repo:weave"],
      "anchors": ["handoff central and fleet design"],
      "adopt_first_inputs": [],
      "implementation_boundary": "Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input",
      "validation": ["just ci"],
      "rollback": ["Revert the integration slice"]
    }
  ],
  "gates": ["just ci"],
  "findings": []
}
"#,
    )
    .unwrap();
}
