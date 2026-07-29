// HFTASK-0080 (ADR-0019 D5 #3): the error-handling deny lints (unwrap_used/expect_used/panic)
// are allowed under test only — tests assert, which is idiomatic. Production code is hardened.
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]
//! `work-order` — the handoff.task.v1 work-order envelope (S1 spike).
//!
//! Validates the front-door seam: a prompt_hub `SwarmBundle` is converted into one or
//! more provable `WorkOrder`s (handoff.task.v1). The `workflow_id` is carried as
//! `correlation_id` — the single cross-reference handle that closes gap #1 (task-truth)
//! and gap #3 (integration contract). Intent/scope/acceptance are hashed (blake3) so a
//! downstream verifier (`ruvector-verified`) can treat the order as a provable contract.

pub mod intake;
pub use intake::{Intent, SynthSpec, synthesize_spec};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Backlog,
    Active,
    Claimed,
    Blocked,
    Checkpointed,
    Review,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum Priority {
    P0,
    P1,
    P2,
    P3,
}

/// HFTASK-0083: priority display (`Priority` → `"P0".."P3"`), lifted from hf so the peeled feature
/// crates (handoff-fleet, …) and the in-hf modules (sync) share one impl.
pub trait PrioStr {
    fn priority_str(&self) -> &'static str;
}
impl PrioStr for WorkOrder {
    fn priority_str(&self) -> &'static str {
        match self.priority {
            Priority::P0 => "P0",
            Priority::P1 => "P1",
            Priority::P2 => "P2",
            Priority::P3 => "P3",
        }
    }
}

/// The handoff.task.v1 envelope (mirrors `~/Downloads/tmp/handoff/handoff/schemas/task.schema.json`),
/// plus provenance fields that link it back to the front door and make it provable.
///
/// Deserialization is **fail-closed** (`try_from = "WorkOrderRaw"`): a card whose `schema`
/// discriminator is foreign, whose `id` violates the published pattern, or whose content no
/// longer matches its recorded blake3 `intent_lock` is rejected on load with a [`CardError`]
/// — the validating consumer the convergence suite (`tests/handoff_card_consumer.rs`)
/// demands. The kernel's schema-gated drift-review loader is the one sanctioned bypass
/// ([`WorkOrder::from_value_unvalidated`]).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "WorkOrderRaw")]
pub struct WorkOrder {
    /// const "handoff.task.v1" — the schema discriminator. A card carrying any other value is
    /// not a handoff.task.v1 envelope and is rejected by the validator.
    #[schemars(regex(pattern = r"^handoff\.task\.v1$"))]
    pub schema: String,
    /// `^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$` — the canonical id form. Accepts the numeric kernel/
    /// intake ids (`HFTASK-0058`, `PHTASK-0025`, `TASK-0001`) AND the slug-style kb-minted ids
    /// (`KBTASK-FLEET-HANDOFF-ROLLOUT`, `KBTASK-HFTASK-0058`); a free-form id or an empty slug
    /// is rejected. (Numeric-only `[0-9]{4,}` wrongly rejected every slug-id kb card on disk.)
    #[schemars(regex(pattern = r"^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$"))]
    pub id: String,
    pub title: String,
    pub status: Status,
    pub priority: Priority,
    pub objective: String,
    pub path_scope: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub test_commands: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub blocked_by: Vec<String>,
    #[serde(default)]
    pub allows_network: bool,
    #[serde(default)]
    pub allows_dependency_addition: bool,

    // --- provenance / contract extensions (S1) ---
    /// = SwarmBundle.workflow_id. The cross-ref handle weave Job.correlation_id syncs to.
    pub correlation_id: String,
    /// which role-prompt in the bundle minted this order (None = whole-bundle order).
    #[serde(default)]
    pub role: Option<String>,
    /// blake3 intent-lock (the drift sentinel anchor; ruvector-verified can prove against it).
    pub intent_lock: IntentLock,
}

/// blake3 hashes of the immutable contract surface — the .handoff drift-sentinel model.
///
/// PRD §12.2 specifies **five** lock fields. The first three (objective/path_scope/acceptance)
/// are the original surface; `constraint_hash` (§12.1 policy/permission surface) and
/// `northstar_revision` (the capsule doctrine the order was minted against) close the gap so
/// constraint drift and a North-Star revision become hash-detectable.
///
/// No-downgrade / backward-compat: the two new fields are `#[serde(default,
/// skip_serializing_if = "String::is_empty")]`, so a *legacy partial lock* (the three-hash
/// form minted before HFTASK-0047) round-trips to byte-identical JSON and existing cards keep
/// verifying unchanged. A populated 5-field lock is a strict superset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct IntentLock {
    pub objective_hash: String,
    pub path_scope_hash: String,
    pub acceptance_hash: String,
    /// blake3 of the constraint surface (permission flags + dependency edges). Empty on a
    /// legacy partial lock minted before the 5-field form existed.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub constraint_hash: String,
    /// blake3 of the North-Star doctrine this order was minted against (capsule `northstar`).
    /// Empty on a legacy partial lock; a change here means the order predates a doctrine
    /// revision and must be re-minted. Supplied externally (the capsule lives at the hf layer).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub northstar_revision: String,
}

/// Why a card was rejected by the fail-closed load path (the validating consumer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CardError {
    /// `schema` != `handoff.task.v1` (task.schema.json discriminator, `^handoff\.task\.v1$`).
    ForeignSchema(String),
    /// `id` violates the published `^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$` pattern.
    InvalidId(String),
    /// Recomputed objective/path_scope/acceptance hashes no longer match the recorded
    /// `intent_lock` — the card was tampered with after minting.
    IntentLockDrift { id: String },
}

impl std::fmt::Display for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardError::ForeignSchema(s) => write!(
                f,
                "card rejected: schema discriminator {s:?} is not \"handoff.task.v1\""
            ),
            CardError::InvalidId(id) => write!(
                f,
                "card rejected: id {id:?} violates the published ^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$ pattern"
            ),
            CardError::IntentLockDrift { id } => write!(
                f,
                "card rejected: {id} content no longer matches its recorded intent_lock (tampered after minting)"
            ),
        }
    }
}

impl std::error::Error for CardError {}

/// `^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$` without a regex dependency: some occurrence of `TASK-`
/// must be preceded only by ASCII uppercase letters and followed by a non-empty
/// `[A-Z0-9][A-Z0-9-]*` suffix.
fn id_matches_published_pattern(id: &str) -> bool {
    id.match_indices("TASK-").any(|(i, _)| {
        let prefix_ok = id[..i].chars().all(|c| c.is_ascii_uppercase());
        let suffix = &id[i + "TASK-".len()..];
        let mut chars = suffix.chars();
        let head_ok =
            matches!(chars.next(), Some(c) if c.is_ascii_uppercase() || c.is_ascii_digit());
        prefix_ok
            && head_ok
            && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
    })
}

/// Unvalidated wire shape backing [`WorkOrder`]'s fail-closed `Deserialize` (`try_from`).
/// Field-for-field identical to [`WorkOrder`]; validation lives in the `TryFrom` impl.
#[derive(Deserialize)]
struct WorkOrderRaw {
    schema: String,
    id: String,
    title: String,
    status: Status,
    priority: Priority,
    objective: String,
    path_scope: Vec<String>,
    acceptance_criteria: Vec<String>,
    test_commands: Vec<String>,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    blocked_by: Vec<String>,
    #[serde(default)]
    allows_network: bool,
    #[serde(default)]
    allows_dependency_addition: bool,
    correlation_id: String,
    #[serde(default)]
    role: Option<String>,
    intent_lock: IntentLock,
}

impl WorkOrderRaw {
    /// Field-for-field move into the validated type (no checks — callers decide).
    fn into_order(self) -> WorkOrder {
        WorkOrder {
            schema: self.schema,
            id: self.id,
            title: self.title,
            status: self.status,
            priority: self.priority,
            objective: self.objective,
            path_scope: self.path_scope,
            acceptance_criteria: self.acceptance_criteria,
            test_commands: self.test_commands,
            dependencies: self.dependencies,
            blocked_by: self.blocked_by,
            allows_network: self.allows_network,
            allows_dependency_addition: self.allows_dependency_addition,
            correlation_id: self.correlation_id,
            role: self.role,
            intent_lock: self.intent_lock,
        }
    }
}

impl TryFrom<WorkOrderRaw> for WorkOrder {
    type Error = CardError;

    fn try_from(raw: WorkOrderRaw) -> Result<Self, Self::Error> {
        if raw.schema != "handoff.task.v1" {
            return Err(CardError::ForeignSchema(raw.schema));
        }
        if !id_matches_published_pattern(&raw.id) {
            return Err(CardError::InvalidId(raw.id));
        }
        let order = raw.into_order();
        if !order.intent_unchanged() {
            return Err(CardError::IntentLockDrift { id: order.id });
        }
        Ok(order)
    }
}

/// Per-surface drift verdict (`true` = unchanged) across all five IntentLock surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntentComponents {
    pub objective: bool,
    pub path_scope: bool,
    pub acceptance: bool,
    pub constraint: bool,
    pub northstar: bool,
}

impl IntentComponents {
    /// True iff every surface is unchanged.
    pub fn all_match(&self) -> bool {
        self.objective && self.path_scope && self.acceptance && self.constraint && self.northstar
    }
}

fn b3(s: &str) -> String {
    format!("blake3:{}", blake3::hash(s.as_bytes()).to_hex())
}

/// blake3 of the North-Star doctrine, in the same `blake3:<hex>` form the lock stores. Empty
/// doctrine → empty revision (legacy/uninitialized capsule raises no northstar obligation).
/// Single hashing impl shared by the lock and the hf layer (no second blake3 dep edge).
pub fn northstar_revision(doctrine: &str) -> String {
    if doctrine.is_empty() {
        String::new()
    } else {
        b3(doctrine)
    }
}

/// Canonicalize the constraint/permission surface (§12.1) into one deterministic string so it
/// can be hashed. Order-stable: dependency lists are joined as-stored (intake order is itself
/// deterministic). Kept as a free fn so both the struct and call sites can hash a surface.
pub fn constraint_surface(
    allows_network: bool,
    allows_dependency_addition: bool,
    dependencies: &[String],
    blocked_by: &[String],
) -> String {
    format!(
        "network={};dep_add={};dependencies={};blocked_by={}",
        allows_network,
        allows_dependency_addition,
        dependencies.join(","),
        blocked_by.join(","),
    )
}

impl WorkOrder {
    /// Legacy three-hash constructor (objective/path_scope/acceptance). Retained verbatim so
    /// every existing call site is unchanged and mints a partial lock with empty constraint /
    /// northstar fields (no-downgrade). Use [`WorkOrder::full_intent_lock`] for the 5-field form.
    pub fn compute_intent_lock(
        objective: &str,
        path_scope: &[String],
        acceptance: &[String],
    ) -> IntentLock {
        IntentLock {
            objective_hash: b3(objective),
            path_scope_hash: b3(&path_scope.join("\n")),
            acceptance_hash: b3(&acceptance.join("\n")),
            constraint_hash: String::new(),
            northstar_revision: String::new(),
        }
    }

    /// blake3 of this order's current constraint surface (§12.1).
    pub fn constraint_hash(&self) -> String {
        b3(&constraint_surface(
            self.allows_network,
            self.allows_dependency_addition,
            &self.dependencies,
            &self.blocked_by,
        ))
    }

    /// Compute the full 5-field lock from this order's live fields plus the externally-supplied
    /// `northstar_revision` (the hash of the capsule doctrine; the capsule lives at the hf layer).
    pub fn full_intent_lock(&self, northstar_revision: &str) -> IntentLock {
        let mut lock =
            Self::compute_intent_lock(&self.objective, &self.path_scope, &self.acceptance_criteria);
        lock.constraint_hash = self.constraint_hash();
        lock.northstar_revision = northstar_revision.to_string();
        lock
    }

    /// Deserialize a card WITHOUT the fail-closed envelope checks (discriminator, id pattern,
    /// intent-lock match). The one sanctioned caller is the kernel's schema-gated card loader
    /// (`handoff-core::try_parse_card`): the jsonschema gate upstream already enforces the
    /// discriminator + id pattern, and a card with a DRIFTED intent_lock must still LOAD there
    /// so the drift sentinel can report it and prescribe `hf relock` (loading is not trusting —
    /// PRD §12.3). Every other consumer gets the fail-closed `Deserialize` path.
    pub fn from_value_unvalidated(value: serde_json::Value) -> Result<Self, String> {
        let raw: WorkOrderRaw = serde_json::from_value(value).map_err(|e| e.to_string())?;
        Ok(raw.into_order())
    }

    /// Recompute the intent-lock from current fields and report whether it still matches
    /// (the core drift check: did objective/scope/acceptance mutate without a new order?).
    /// Degrade-aware: only the three base hashes are compared (constraint/northstar need the
    /// capsule and are checked by [`WorkOrder::intent_components`] / the drift gate).
    pub fn intent_unchanged(&self) -> bool {
        let r =
            Self::compute_intent_lock(&self.objective, &self.path_scope, &self.acceptance_criteria);
        r.objective_hash == self.intent_lock.objective_hash
            && r.path_scope_hash == self.intent_lock.path_scope_hash
            && r.acceptance_hash == self.intent_lock.acceptance_hash
    }

    /// Per-component drift report against the recorded lock, including the two new surfaces.
    /// `northstar_revision` is the current capsule doctrine hash. A component is reported
    /// `false` (drifted) only when the recorded lock actually carries that surface — a legacy
    /// partial lock (empty constraint/northstar) is never spuriously flagged (no-downgrade).
    pub fn intent_components(&self, northstar_revision: &str) -> IntentComponents {
        let rec = &self.intent_lock;
        let red =
            Self::compute_intent_lock(&self.objective, &self.path_scope, &self.acceptance_criteria);
        IntentComponents {
            objective: rec.objective_hash == red.objective_hash,
            path_scope: rec.path_scope_hash == red.path_scope_hash,
            acceptance: rec.acceptance_hash == red.acceptance_hash,
            // legacy partial lock (empty) → treat as matching, never a false drift
            constraint: rec.constraint_hash.is_empty()
                || rec.constraint_hash == self.constraint_hash(),
            northstar: rec.northstar_revision.is_empty()
                || rec.northstar_revision == northstar_revision,
        }
    }

    pub fn to_json(&self) -> String {
        // INFALLIBLE: `WorkOrder` derives `Serialize` over owned String/Vec/enum fields only —
        // no non-string map keys and no custom serializer that can fail, so `to_string_pretty`
        // cannot error here. Justified per-site (HFTASK-0080) rather than rippling a `Result`
        // return through every packet/render call site.
        #[allow(clippy::expect_used)]
        serde_json::to_string_pretty(self).expect("serialize WorkOrder")
    }
}

/// HFTASK-0057 (PRD §7.3/§23): the canonical JSON Schema for the handoff.task.v1 envelope,
/// generated from the live `WorkOrder` types via schemars (single source of truth — the schema
/// can never drift from the Rust shape because it is *derived* from it). Pretty-printed so a
/// committed `schemas/task.schema.json` diffs cleanly. The `hf schema` verb and the fail-closed
/// card-load validator both compile *this* schema, so a card that violates the Rust contract is
/// rejected loudly instead of being silently dropped.
pub fn task_schema_json() -> String {
    let schema = schemars::schema_for!(WorkOrder);
    // INFALLIBLE: a schemars `RootSchema` is a plain serializable JSON structure (no failing
    // serializer), so pretty-printing it cannot error. Justified per-site (HFTASK-0080).
    #[allow(clippy::expect_used)]
    serde_json::to_string_pretty(&schema).expect("serialize task schema")
}

// --- integration contract: mirror of prompt_hub's `SwarmBundle` ---
//
// Field-for-field against `prompt_hub/prompt-hub/src/models.rs:528`:
//   pub struct SwarmBundle {
//       pub workflow_id: Uuid,                      // 530
//       pub role_prompts: HashMap<Role, String>,    // 532
//       pub handoff_template: String,               // 534
//       pub consistency_report: Vec<Conflict>,      // 536
//       pub evolution_suggestions: Vec<String>,     // 538
//   }
//
// This is a *contract mirror*, not a path-dependency: prompt-hub's Cargo uses
// `version.workspace`/`edition.workspace` inheritance, so a path-dep from this workspace
// risks a workspace-inheritance build break (HFTASK-0003 research §A.1/§B.3). Mirroring the
// shape keeps the dependency a documented contract with no cross-repo build edge.
//
// Representation notes (decouple from upstream wire churn, stay deterministic):
//   - `workflow_id: Uuid` is carried as a `String` (the `correlation_id` handle).
//   - `role_prompts: HashMap<Role,String>` is carried as an ordered `Vec<(String,String)>`
//     (role token, prompt). A Vec — not a map — so intake order, and therefore minted ids,
//     are deterministic. `Role` is a string token (matches the enum's serde repr + `Custom`).
//   - `consistency_report: Vec<Conflict>` is reduced to `Vec<String>` (human-readable
//     conflict summaries) — the conflict detail is not needed to synthesize a WorkOrder.
//   - `#[serde(default)]` on the trailing three fields so older 3-field bundle JSON
//     (the S1 spike shape) still deserializes — backward compatible.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmBundle {
    /// = prompt_hub `workflow_id: Uuid`, as string. Becomes each order's `correlation_id`.
    pub workflow_id: String,
    /// = prompt_hub `role_prompts: HashMap<Role,String>`, ordered for determinism.
    #[serde(default)]
    pub role_prompts: Vec<(String, String)>,
    /// = prompt_hub `handoff_template: String` (Handlebars skeleton).
    #[serde(default)]
    pub handoff_template: String,
    /// = prompt_hub `consistency_report: Vec<Conflict>`, reduced to summary strings.
    #[serde(default)]
    pub consistency_report: Vec<String>,
    /// = prompt_hub `evolution_suggestions: Vec<String>`.
    #[serde(default)]
    pub evolution_suggestions: Vec<String>,
}

/// THE SEAM: prompt_hub SwarmBundle -> Vec<WorkOrder> (one provable handoff.task.v1 per role).
///
/// Each order's verifiable fields (`path_scope`, `acceptance_criteria`, `test_commands`) are
/// **synthesized deterministically** from a vibe `Intent` via [`synthesize_spec`] — closing
/// the HFTASK-0003 gap where the spike emitted `path_scope: ["."]` + `test_commands: []`
/// (unverifiable by the drift gate). The per-role Intent is, in precedence:
///   1. `intent_override` (the `--vibe`/`--intent` whole-bundle intent), else
///   2. `Intent::classify(role_prompt)` (deterministic, mirrors prompt_hub's classifier).
///
/// `objective = "<TaskType>: <prompt>"` (≥10 chars, schema minLength), `correlation_id =
/// workflow_id` (the cross-ref handle), and `intent_lock` is computed over the synthesized
/// triple. Pure: same `(bundle, intent_override, scope_override)` → byte-identical orders.
pub fn work_orders_from_bundle_with(
    bundle: &SwarmBundle,
    intent_override: Option<&Intent>,
    scope_override: Option<&[String]>,
) -> Vec<WorkOrder> {
    bundle
        .role_prompts
        .iter()
        .enumerate()
        .map(|(i, (role, prompt))| {
            // HFTASK-0084: scope the id to the bundle's workflow_id so re-runs of a DIFFERENT
            // bundle never clobber an existing TASK-NNNN card on disk (save_task overwrites by id).
            let id = synthesized_task_id(&bundle.workflow_id, i + 1);
            let classified;
            let intent = match intent_override {
                Some(it) => it,
                None => {
                    classified = Intent::classify(prompt);
                    &classified
                }
            };
            let spec = synthesize_spec(intent, Some(role), scope_override);
            let objective = compose_objective(&intent.task_type, prompt);
            let intent_lock = WorkOrder::compute_intent_lock(
                &objective,
                &spec.path_scope,
                &spec.acceptance_criteria,
            );
            WorkOrder {
                schema: "handoff.task.v1".to_string(),
                id,
                title: format!("[{role}] {}", first_line(prompt)),
                status: Status::Backlog,
                priority: Priority::P1,
                objective,
                path_scope: spec.path_scope,
                acceptance_criteria: spec.acceptance_criteria,
                test_commands: spec.test_commands,
                dependencies: vec![],
                blocked_by: vec![],
                allows_network: false,
                allows_dependency_addition: false,
                correlation_id: bundle.workflow_id.clone(),
                role: Some(role.clone()),
                intent_lock,
            }
        })
        .collect()
}

/// Back-compat convenience: synthesize with a per-role classified Intent and default scope.
pub fn work_orders_from_bundle(bundle: &SwarmBundle) -> Vec<WorkOrder> {
    work_orders_from_bundle_with(bundle, None, None)
}

/// Deterministic 64-bit FNV-1a hash (mirrors the prompt_hub vibe handle).
fn stable_hash(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// HFTASK-0084: the collision-free, schema-valid task id for a synthesized order.
///
/// The old sites minted `TASK-0001`, `TASK-0002`, … counting from a FIXED base, so a second
/// intake/prompt-hub run silently clobbered any existing `TASK-NNNN.task.json` on disk
/// (`save_task` overwrites by id) — durable continuity-state loss (FAIL-OPEN). The fix derives a
/// stable 24-bit prefix from the unique `workflow_id` (which already carries a hash + nanos), so
/// distinct bundles get disjoint id sets while the same bundle re-mints the SAME ids (idempotent,
/// not data loss). The intake determinism property holds: same `workflow_id` → byte-identical id.
///
/// Form `TASK-{:06X}-{:04}` is valid under the live schema id pattern
/// `^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$` (uppercase hex `0-9A-F` + a hyphen separator).
pub fn synthesized_task_id(workflow_id: &str, seq: usize) -> String {
    format!(
        "TASK-{:06X}-{:04}",
        stable_hash(workflow_id) & 0x00FF_FFFF,
        seq
    )
}

/// Compose a schema-valid objective (`minLength: 10`) from the task_type + prompt. When the
/// prompt is empty (prod `role_prompts` can be empty) a descriptive fallback is used.
fn compose_objective(task_type: &str, prompt: &str) -> String {
    let p = prompt.trim();
    let composed = if p.is_empty() {
        format!("{task_type}: work order synthesized from SwarmBundle (no role prompt)")
    } else {
        let verb = task_type
            .chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>() + &task_type[1..])
            .unwrap_or_else(|| task_type.to_string());
        format!("{verb}: {p}")
    };
    if composed.len() < 10 {
        format!("{composed} (handoff work order)")
    } else {
        composed
    }
}

fn first_line(s: &str) -> String {
    s.lines().next().unwrap_or("").chars().take(60).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bundle() -> SwarmBundle {
        SwarmBundle {
            workflow_id: "wf-0001".to_string(),
            role_prompts: vec![
                (
                    "architect".to_string(),
                    "Design the storefront schema in rust".to_string(),
                ),
                (
                    "coder".to_string(),
                    "Implement the checkout flow in rust".to_string(),
                ),
            ],
            handoff_template: "standard".to_string(),
            consistency_report: vec![],
            evolution_suggestions: vec![],
        }
    }

    #[test]
    fn seam_bundle_to_workorders() {
        let orders = work_orders_from_bundle(&sample_bundle());
        assert_eq!(orders.len(), 2);
        // every order carries the workflow_id as correlation_id (the cross-ref handle)
        assert!(orders.iter().all(|o| o.correlation_id == "wf-0001"));
        // HFTASK-0084: ids are bundle-scoped (TASK-<6hex>-<seq>), no longer the fixed TASK-0001.
        assert_eq!(orders[0].id, synthesized_task_id("wf-0001", 1));
        assert!(orders[0].id.ends_with("-0001") && orders[1].id.ends_with("-0002"));
        assert_eq!(orders[0].role.as_deref(), Some("architect"));
        assert_eq!(orders[0].schema, "handoff.task.v1");
    }

    #[test]
    fn synthesized_task_id_is_deterministic_and_schema_valid() {
        // same workflow_id → byte-identical id (intake determinism property)
        assert_eq!(
            synthesized_task_id("vibe-abc-123", 1),
            synthesized_task_id("vibe-abc-123", 1)
        );
        // valid under the live schema pattern ^[A-Z]*TASK-[A-Z0-9][A-Z0-9-]*$
        let id = synthesized_task_id("vibe-abc-123", 7);
        let body = id.strip_prefix("TASK-").expect("starts with TASK-");
        assert!(
            body.starts_with(|c: char| c.is_ascii_uppercase() || c.is_ascii_digit())
                && body
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-'),
            "id {id} must match the schema id pattern"
        );
        assert!(id.ends_with("-0007"));
    }

    #[test]
    fn synthesized_task_ids_do_not_collide_across_bundles() {
        // Two DISTINCT bundles must produce DISJOINT id sets — the clobber bug (HFTASK-0084).
        let a: Vec<_> = (1..=3).map(|i| synthesized_task_id("wf-A", i)).collect();
        let b: Vec<_> = (1..=3).map(|i| synthesized_task_id("wf-B", i)).collect();
        assert!(
            a.iter().all(|x| !b.contains(x)),
            "distinct workflow_ids must yield disjoint ids: {a:?} vs {b:?}"
        );
    }

    #[test]
    fn synthesized_orders_are_verifiable_no_junk_defaults() {
        // HFTASK-0003 acceptance #1: never path_scope ["."], never empty test_commands.
        for o in work_orders_from_bundle(&sample_bundle()) {
            assert!(!o.path_scope.is_empty());
            assert!(
                !o.path_scope.iter().any(|s| s == "." || s == "./"),
                "{}: path_scope must be narrower than repo root, got {:?}",
                o.id,
                o.path_scope
            );
            assert!(
                !o.test_commands.is_empty(),
                "{}: test_commands must be non-empty",
                o.id
            );
            // rust prompts → cargo test present
            assert!(o.test_commands.iter().any(|c| c == "cargo test"));
            // objective satisfies schema minLength 10
            assert!(o.objective.len() >= 10);
            // acceptance is non-empty and intent_lock is fresh
            assert!(!o.acceptance_criteria.is_empty());
            assert!(o.intent_unchanged());
        }
    }

    #[test]
    fn intake_is_deterministic_same_ids_same_locks() {
        // HFTASK-0003 acceptance #3: re-running yields identical ids + intent_lock hashes.
        let a = work_orders_from_bundle(&sample_bundle());
        let b = work_orders_from_bundle(&sample_bundle());
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(b.iter()) {
            assert_eq!(x.id, y.id);
            assert_eq!(x.intent_lock, y.intent_lock);
            assert_eq!(x.correlation_id, y.correlation_id);
            assert_eq!(x.objective, y.objective);
        }
    }

    #[test]
    fn intent_lock_detects_drift() {
        let mut o = work_orders_from_bundle(&sample_bundle()).remove(0);
        assert!(o.intent_unchanged(), "fresh order must match its lock");
        o.objective = "Redesign the entire architecture".to_string(); // goal drift
        assert!(!o.intent_unchanged(), "objective drift must be detected");
    }

    #[test]
    fn roundtrips_through_json() {
        let o = work_orders_from_bundle(&sample_bundle()).remove(0);
        let j = o.to_json();
        let back: WorkOrder = serde_json::from_str(&j).unwrap();
        assert_eq!(back.id, o.id);
        assert_eq!(back.intent_lock, o.intent_lock);
    }

    #[test]
    fn legacy_partial_lock_serializes_to_three_fields() {
        // HFTASK-0047 no-downgrade: a lock minted by the 3-arg constructor must round-trip to
        // byte-identical 3-field JSON (the new fields are skipped when empty), so existing
        // committed cards are unaffected.
        let lock = WorkOrder::compute_intent_lock("obj", &["a/".into()], &["does x".into()]);
        let j = serde_json::to_string(&lock).unwrap();
        assert!(
            !j.contains("constraint_hash"),
            "empty constraint must be skipped: {j}"
        );
        assert!(
            !j.contains("northstar_revision"),
            "empty northstar must be skipped: {j}"
        );
        // and a legacy 3-field card still deserializes
        let back: IntentLock = serde_json::from_str(
            r#"{"objective_hash":"a","path_scope_hash":"b","acceptance_hash":"c"}"#,
        )
        .unwrap();
        assert_eq!(back.constraint_hash, "");
        assert_eq!(back.northstar_revision, "");
    }

    #[test]
    fn full_lock_carries_all_five_surfaces() {
        let mut o = work_orders_from_bundle(&sample_bundle()).remove(0);
        o.intent_lock = o.full_intent_lock("blake3:northstar-rev-1");
        assert!(!o.intent_lock.constraint_hash.is_empty());
        assert_eq!(o.intent_lock.northstar_revision, "blake3:northstar-rev-1");
        // a full lock survives a JSON round-trip with all five fields
        let back: WorkOrder = serde_json::from_str(&o.to_json()).unwrap();
        assert_eq!(back.intent_lock, o.intent_lock);
    }

    #[test]
    fn constraint_drift_is_detected_only_on_a_full_lock() {
        let mut o = work_orders_from_bundle(&sample_bundle()).remove(0);
        // legacy partial lock: constraint surface is never spuriously flagged
        assert!(
            o.intent_components("ns").constraint,
            "legacy lock must not flag constraint"
        );
        // promote to a full lock, then mutate the permission surface
        o.intent_lock = o.full_intent_lock("ns");
        assert!(o.intent_components("ns").all_match());
        o.allows_network = !o.allows_network; // policy/constraint drift
        let c = o.intent_components("ns");
        assert!(
            !c.constraint,
            "constraint drift must be detected on a full lock"
        );
        assert!(
            c.objective && c.path_scope && c.acceptance,
            "base surfaces unchanged"
        );
    }

    #[test]
    fn northstar_revision_drift_is_detected() {
        let mut o = work_orders_from_bundle(&sample_bundle()).remove(0);
        o.intent_lock = o.full_intent_lock("blake3:rev-A");
        assert!(o.intent_components("blake3:rev-A").northstar);
        assert!(
            !o.intent_components("blake3:rev-B").northstar,
            "a doctrine revision must mark the order's northstar surface drifted"
        );
    }

    #[test]
    fn id_pattern_accepts_canonical_and_slug_forms_rejects_freeform() {
        for ok in [
            "TASK-0001",
            "HFTASK-0058",
            "PHTASK-0025",
            "KBTASK-FLEET-HANDOFF-ROLLOUT",
            "KBTASK-HFTASK-0058",
            synthesized_task_id("wf-0001", 1).as_str(), // HFTASK-0084 scoped form
        ] {
            assert!(id_matches_published_pattern(ok), "{ok} must match");
        }
        for bad in [
            "task-lowercase-not-canonical",
            "TASK-",        // empty suffix
            "TASK-x",       // lowercase suffix head
            "0058-HFTASK",  // digits before TASK-
            "NOTHING",      // no TASK- at all
            "TASK-A_B",     // underscore not in class
            "hfTASK-0001x", // lowercase prefix + tail
        ] {
            assert!(!id_matches_published_pattern(bad), "{bad} must be rejected");
        }
    }

    #[test]
    fn fail_closed_load_reports_typed_card_errors() {
        let order = work_orders_from_bundle(&sample_bundle()).remove(0);

        let mut v: serde_json::Value = serde_json::from_str(&order.to_json()).unwrap();
        v["schema"] = serde_json::json!("openai.task.v1");
        let e = serde_json::from_str::<WorkOrder>(&v.to_string()).unwrap_err();
        assert!(e.to_string().contains("schema discriminator"), "{e}");

        let mut v: serde_json::Value = serde_json::from_str(&order.to_json()).unwrap();
        v["id"] = serde_json::json!("free-form");
        let e = serde_json::from_str::<WorkOrder>(&v.to_string()).unwrap_err();
        assert!(e.to_string().contains("violates the published"), "{e}");

        let mut v: serde_json::Value = serde_json::from_str(&order.to_json()).unwrap();
        v["objective"] = serde_json::json!("tampered objective after minting");
        let e = serde_json::from_str::<WorkOrder>(&v.to_string()).unwrap_err();
        assert!(e.to_string().contains("intent_lock"), "{e}");
    }

    #[test]
    fn unvalidated_loader_still_loads_a_drifted_card_for_drift_review() {
        // The kernel's relock flow depends on this: a tampered card must be LOADABLE through
        // the explicit bypass (then reported by the drift sentinel), while the default
        // Deserialize path rejects it.
        let order = work_orders_from_bundle(&sample_bundle()).remove(0);
        let mut v: serde_json::Value = serde_json::from_str(&order.to_json()).unwrap();
        v["objective"] = serde_json::json!("tampered objective after minting");

        assert!(serde_json::from_str::<WorkOrder>(&v.to_string()).is_err());
        let loaded = WorkOrder::from_value_unvalidated(v).expect("bypass loads the drifted card");
        assert!(
            !loaded.intent_unchanged(),
            "the loaded card must still be reportable as drifted"
        );
    }

    #[test]
    fn committed_v1_card_corpus_still_loads_fail_closed() {
        // Live-data regression: every handoff.task.v1 card committed under .handoff/tasks
        // must survive the fail-closed loader (no over-tightening against real cards).
        // Pre-envelope legacy cards (no `schema` field) were never loadable as WorkOrder
        // and stay out of contract.
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../.handoff/tasks");
        let mut v1_seen = 0usize;
        for entry in std::fs::read_dir(&dir).expect(".handoff/tasks exists in the repo") {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let value: serde_json::Value = serde_json::from_str(&text).unwrap();
            if value.get("schema").and_then(|s| s.as_str()) != Some("handoff.task.v1") {
                continue; // pre-envelope legacy card: out of contract
            }
            v1_seen += 1;
            if let Err(e) = serde_json::from_str::<WorkOrder>(&text) {
                panic!("committed card {} no longer loads: {e}", path.display());
            }
        }
        assert!(
            v1_seen > 0,
            "corpus check must actually exercise cards (fail-closed: zero cards = FAIL)"
        );
    }

    #[test]
    fn task_schema_is_nonempty_and_requires_contract_fields() {
        // HFTASK-0057 (PRD §7.3): the generated schema is non-empty JSON whose `required`
        // set includes the load-bearing contract fields — the very ones whose absence let a
        // bad card (missing intent_lock) get silently dropped before this task.
        let schema = task_schema_json();
        assert!(!schema.trim().is_empty(), "schema must be non-empty");
        let v: serde_json::Value = serde_json::from_str(&schema).expect("schema is valid JSON");
        let required = v
            .get("required")
            .and_then(|r| r.as_array())
            .expect("WorkOrder schema must declare a `required` array");
        let names: Vec<&str> = required.iter().filter_map(|x| x.as_str()).collect();
        for field in [
            "intent_lock",
            "objective",
            "path_scope",
            "acceptance_criteria",
        ] {
            assert!(
                names.contains(&field),
                "schema `required` must include `{field}`, got {names:?}"
            );
        }
    }

    #[test]
    fn schemars_derive_does_not_change_serialization() {
        // No serde regression: deriving JsonSchema must not alter the wire form. A real
        // WorkOrder still round-trips byte-identically, and a 3-field legacy lock still skips
        // the two empty extension fields (the `skip_serializing_if` contract is preserved).
        let o = work_orders_from_bundle(&sample_bundle()).remove(0);
        let before = o.to_json();
        let back: WorkOrder = serde_json::from_str(&before).expect("round-trip");
        assert_eq!(
            before,
            back.to_json(),
            "serialization must be byte-identical"
        );
        let lock = WorkOrder::compute_intent_lock("obj", &["a/".into()], &["does x".into()]);
        let j = serde_json::to_string(&lock).unwrap();
        assert!(
            !j.contains("constraint_hash"),
            "empty constraint skipped: {j}"
        );
        assert!(
            !j.contains("northstar_revision"),
            "empty northstar skipped: {j}"
        );
    }
}
