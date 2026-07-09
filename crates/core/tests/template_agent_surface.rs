// HFTASK-0082 (ADR-0019 D5 #3): this whole crate is an integration test; unwrap/expect/panic
// are idiomatic here (tests assert), so the deny lints are allowed crate-wide.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use rusty_idd_core::model::FileCategory;
use rusty_idd_core::scanner::{classify_file, scan_repo};
use std::fs;
use std::path::Path;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
}

#[test]
fn portable_template_c4_command_loads_canonical_skill_and_templates() {
    let root = repo_root();
    let required = [
        "intent-driven-template/.agent/manifest.json",
        "intent-driven-template/.agent/README.md",
        "intent-driven-template/.agent/commands/create-c4-diagram.md",
        "intent-driven-template/.agent/skills/c4-diagrams/SKILL.md",
        "intent-driven-template/.agents/skills/c4-diagrams/SKILL.md",
        "intent-driven-template/.agents/skills/c4-diagrams/templates.md",
    ];

    for rel in required {
        assert!(root.join(rel).exists(), "missing {rel}");
    }

    let manifest =
        fs::read_to_string(root.join("intent-driven-template/.agent/manifest.json")).unwrap();
    assert!(manifest.contains("\"name\": \"c4-diagrams\""));
    assert!(manifest.contains("\"name\": \"create-c4-diagram\""));
    assert!(manifest.contains("../.agents/skills/c4-diagrams/SKILL.md"));
    assert!(manifest.contains("../.agents/skills/c4-diagrams/templates.md"));

    let command = fs::read_to_string(
        root.join("intent-driven-template/.agent/commands/create-c4-diagram.md"),
    )
    .unwrap();
    assert!(command.contains("intent-driven-template/.agents/skills/c4-diagrams/SKILL.md"));
    assert!(command.contains("intent-driven-template/.agents/skills/c4-diagrams/templates.md"));
    assert!(command.contains("Mermaid"));

    let wrapper =
        fs::read_to_string(root.join("intent-driven-template/.agent/skills/c4-diagrams/SKILL.md"))
            .unwrap();
    assert!(wrapper.contains("intent-driven-template/.agents/skills/c4-diagrams/SKILL.md"));
    assert!(wrapper.contains("intent-driven-template/.agents/skills/c4-diagrams/templates.md"));
}

#[test]
fn portable_template_wrappers_reference_existing_canonical_files() {
    let root = repo_root();
    let command_pairs = [
        ("opsx-apply.md", "../../.opencode/commands/opsx-apply.md"),
        (
            "opsx-archive.md",
            "../../.opencode/commands/opsx-archive.md",
        ),
        (
            "opsx-bulk-apply.md",
            "../../.opencode/commands/opsx-bulk-apply.md",
        ),
        (
            "opsx-continue.md",
            "../../.opencode/commands/opsx-continue.md",
        ),
        (
            "opsx-explore.md",
            "../../.opencode/commands/opsx-explore.md",
        ),
        ("opsx-new.md", "../../.opencode/commands/opsx-new.md"),
        (
            "opsx-propose.md",
            "../../.opencode/commands/opsx-propose.md",
        ),
        ("opsx-sync.md", "../../.opencode/commands/opsx-sync.md"),
        ("opsx-verify.md", "../../.opencode/commands/opsx-verify.md"),
    ];

    for (wrapper, source) in command_pairs {
        let wrapper_path = root
            .join("intent-driven-template/.agent/commands")
            .join(wrapper);
        let text = fs::read_to_string(&wrapper_path).unwrap();
        assert!(text.contains(source), "{wrapper} missing {source}");
        assert!(
            wrapper_path.parent().unwrap().join(source).exists(),
            "{wrapper} points at missing source {source}"
        );
    }

    let skill_pairs = [
        (
            "architectural-decision-records/SKILL.md",
            "../../../.agents/skills/architectural-decision-records/SKILL.md",
        ),
        (
            "c4-diagrams/SKILL.md",
            "../../../.agents/skills/c4-diagrams/SKILL.md",
        ),
        (
            "gherkin-authoring/SKILL.md",
            "../../../.agents/skills/gherkin-authoring/SKILL.md",
        ),
        (
            "grill-me/SKILL.md",
            "../../../.agents/skills/grill-me/SKILL.md",
        ),
        (
            "openspec-git-discipline/SKILL.md",
            "../../../.agents/skills/openspec-git-discipline/SKILL.md",
        ),
    ];

    for (wrapper, canonical) in skill_pairs {
        let wrapper_path = root
            .join("intent-driven-template/.agent/skills")
            .join(wrapper);
        let text = fs::read_to_string(&wrapper_path).unwrap();
        assert!(text.contains(canonical), "{wrapper} missing {canonical}");
        assert!(
            wrapper_path.parent().unwrap().join(canonical).exists(),
            "{wrapper} points at missing canonical skill {canonical}"
        );
    }
}

#[test]
fn scanner_classifies_portable_agent_surfaces_as_agent_control() {
    assert_eq!(
        classify_file(
            "intent-driven-template/.agent/commands/create-c4-diagram.md",
            Some("md"),
        ),
        FileCategory::AgentControl
    );
    assert_eq!(
        classify_file(
            "intent-driven-template/.agents/skills/c4-diagrams/SKILL.md",
            Some("md"),
        ),
        FileCategory::AgentControl
    );
    assert_eq!(
        classify_file(
            "intent-driven-template/.opencode/commands/opsx-propose.md",
            Some("md")
        ),
        FileCategory::AgentControl
    );

    let inv = scan_repo(repo_root()).unwrap();
    assert!(inv
        .agent_files
        .iter()
        .any(|path| { path == "intent-driven-template/.agent/commands/create-c4-diagram.md" }));
    assert!(inv
        .agent_files
        .iter()
        .any(|path| path == "intent-driven-template/.agents/skills/c4-diagrams/SKILL.md"));
}
