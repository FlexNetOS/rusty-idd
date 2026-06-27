//! Additive RED suite — convergence acceptance criteria for the `work-order` crate.
//!
//! Context (plan/rusty-idd-red-tests, test-coverage dimension):
//! `work-order` IS the `handoff.task.v1` envelope (`crates/work-order/src/lib.rs:37-73`)
//! but it is an UNCONSUMED S1 spike (codemap: 24 dead symbols, zero product callers). The
//! convergence seam to the fleet is a **filesystem + JSON-schema contract**: rusty-idd reads
//! cards from `.handoff/tasks/*.json` (`crates/cli/src/commands/codex.rs:592` →
//! `contains_task_card` @ `codex.rs:886`), and the published downstream contract a real
//! consumer (the `hf` kernel) reads lives at `imports/handoff/schemas/task.schema.json`
//! (and `third_party/upstream/handoff/schemas/task.schema.json`).
//!
//! The UNBUILT capability these tests encode: a **fail-closed validating card consumer**.
//! The published schema declares `pattern`/`const` constraints
//! (`task.schema.json:55` id pattern, `task.schema.json:88` schema discriminator) and the
//! crate doc promises orders are "provable contracts" via the blake3 `intent_lock`
//! (`lib.rs:1-7`, `lib.rs:71`) — but loading a card through the crate today
//! (`serde_json::from_str::<WorkOrder>`, the only deserialize path) IGNORES every one of
//! those constraints, so a card that the published schema would reject is silently accepted.
//! There is no validating load entry point at all (`to_json` exists @ `lib.rs:222`; no
//! `from_card`/`load`/`validate` counterpart).
//!
//! Each test asserts the END-STATE the convergence requires (a malformed/tampered card is
//! REJECTED on load). All three are RED today because the default deserialize path accepts
//! the bad card. They flip GREEN when Feature Forge makes `WorkOrder` deserialization
//! fail-closed (see `## FF test-build spec`). A well-formed card MUST keep loading — the
//! baseline assertion guards against an over-broad fix.

use work_order::{work_orders_from_bundle, SwarmBundle, WorkOrder};

/// A real, well-formed `handoff.task.v1` card produced by the crate's own seam.
fn valid_card_value() -> serde_json::Value {
    let bundle = SwarmBundle {
        workflow_id: "wf-redsuite-0001".to_string(),
        role_prompts: vec![(
            "architect".to_string(),
            "Design the storefront schema in rust".to_string(),
        )],
        handoff_template: String::new(),
        consistency_report: vec![],
        evolution_suggestions: vec![],
    };
    let order = work_orders_from_bundle(&bundle).remove(0);
    serde_json::from_str(&order.to_json()).expect("a freshly minted card is valid JSON")
}

fn load_card(value: &serde_json::Value) -> Result<WorkOrder, serde_json::Error> {
    serde_json::from_str(&value.to_string())
}

/// Baseline: a well-formed card MUST load. This passes today and MUST keep passing after the
/// fail-closed fix — it stops Feature Forge from "fixing" the RED tests by rejecting everything.
#[test]
fn baseline_well_formed_card_loads() {
    let card = valid_card_value();
    assert!(
        load_card(&card).is_ok(),
        "a well-formed handoff.task.v1 card must deserialize"
    );
}

/// Criterion A — discriminator enforcement.
/// `task.schema.json:88` declares `schema` as `const`/`pattern ^handoff\.task\.v1$`. A card
/// carrying any other discriminator is, by the schema, NOT a handoff.task.v1 envelope and a
/// fail-closed consumer must reject it. Today the deserialize path ignores the pattern.
#[test]
fn consumer_rejects_foreign_schema_discriminator() {
    let mut card = valid_card_value();
    card["schema"] = serde_json::json!("openai.task.v1");
    assert!(
        load_card(&card).is_err(),
        "RED: a card whose `schema` != handoff.task.v1 must be rejected on load, but the \
         deserialize path accepted a foreign discriminator (no validating consumer exists)"
    );
}

/// Criterion B — id pattern enforcement.
/// `task.schema.json:55` declares `id` `pattern ^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$`. A lowercase
/// / free-form id violates the published contract and a fail-closed consumer must reject it.
/// Today the deserialize path ignores the pattern.
#[test]
fn consumer_rejects_id_violating_published_pattern() {
    let mut card = valid_card_value();
    card["id"] = serde_json::json!("task-lowercase-not-canonical");
    assert!(
        load_card(&card).is_err(),
        "RED: a card whose `id` violates the published ^[A-Z]*TASK- pattern must be rejected \
         on load, but the deserialize path accepted it"
    );
}

/// Criterion C — intent_lock provability on load.
/// The crate promises each order is a "provable contract": the blake3 `intent_lock`
/// (`lib.rs:71`) anchors objective/path_scope/acceptance. A tampered card whose `objective`
/// was edited after minting (so the recorded lock no longer matches the content) must be
/// rejected by a provable load — otherwise the drift-sentinel guarantee is hollow. Today the
/// deserialize path accepts the tampered card (the existing `intent_unchanged()` check is
/// never run on load — there is no validating loader).
#[test]
fn consumer_rejects_card_with_drifted_intent_lock() {
    let mut card = valid_card_value();
    card["objective"] = serde_json::json!("Totally different objective injected after minting");
    // Sanity: the lock is now stale relative to the content (proves the card IS tampered).
    let tampered: WorkOrder =
        serde_json::from_str(&card.to_string()).expect("tampered card is still valid JSON");
    assert!(
        !tampered.intent_unchanged(),
        "precondition: the mutated objective must make the recorded intent_lock stale"
    );
    assert!(
        load_card(&card).is_err(),
        "RED: a card whose content no longer matches its recorded intent_lock must be rejected \
         by a provable load, but the deserialize path accepted the tampered card"
    );
}
