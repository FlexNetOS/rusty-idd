use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use codegraph_core::{
    CodeNode as UpstreamCodeNode, EdgeRelationship as UpstreamEdge, EdgeType as UpstreamEdgeType,
    Language as UpstreamLanguage, NodeType as UpstreamNodeType,
};
use codegraph_parser::language::LanguageRegistry;
use codegraph_parser::languages::extract_for_language;
use ignore::WalkBuilder;
use repomix_config::load::PartialConfig;
use repomix_config::schema::OutputStyle;
use repomix_core::packager::PackOptions as RepomixPackOptions;
use rusty_idd_core::manifest::workspace_fingerprint;
use serde::{Deserialize, Serialize};

const DEFAULT_MAX_PACK_TOKENS: usize = 120_000;
const GENERATED_ARTIFACT_MAX_PACK_TOKENS: usize = 800_000;
const DEFAULT_MAX_FILE_BYTES: u64 = 512 * 1024;

#[derive(Debug, Clone)]
pub struct IndexOptions {
    pub workspace: PathBuf,
    pub max_file_bytes: u64,
}

impl IndexOptions {
    pub fn new(workspace: impl Into<PathBuf>) -> Self {
        Self {
            workspace: workspace.into(),
            max_file_bytes: DEFAULT_MAX_FILE_BYTES,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PackWorkspaceOptions {
    pub workspace: PathBuf,
    pub out: PathBuf,
    pub style: PackStyle,
    pub max_tokens: usize,
    pub compress: bool,
    pub remove_comments: bool,
    pub remove_empty_lines: bool,
    pub show_line_numbers: bool,
    pub truncate_base64: bool,
    pub include_empty_directories: bool,
    pub top_files_length: Option<usize>,
    pub split_output: Option<u64>,
    pub header_text: Option<String>,
    pub instruction_file_path: Option<PathBuf>,
    pub include_diffs: bool,
    pub include_logs: bool,
    pub include_patterns: Vec<String>,
    pub ignore_patterns: Vec<String>,
}

impl PackWorkspaceOptions {
    pub fn new(workspace: impl Into<PathBuf>, out: impl Into<PathBuf>, style: PackStyle) -> Self {
        Self {
            workspace: workspace.into(),
            out: out.into(),
            style,
            max_tokens: DEFAULT_MAX_PACK_TOKENS,
            compress: false,
            remove_comments: false,
            remove_empty_lines: false,
            show_line_numbers: false,
            truncate_base64: false,
            include_empty_directories: false,
            top_files_length: None,
            split_output: None,
            header_text: None,
            instruction_file_path: None,
            include_diffs: false,
            include_logs: false,
            include_patterns: Vec::new(),
            ignore_patterns: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackStyle {
    Markdown,
    Xml,
    Json,
    Plain,
}

impl PackStyle {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "markdown" | "md" => Ok(Self::Markdown),
            "xml" => Ok(Self::Xml),
            "json" => Ok(Self::Json),
            "plain" | "txt" => Ok(Self::Plain),
            other => bail!("unsupported pack style `{other}`; use markdown, xml, json, or plain"),
        }
    }
}

impl From<PackStyle> for OutputStyle {
    fn from(value: PackStyle) -> Self {
        match value {
            PackStyle::Markdown => OutputStyle::Markdown,
            PackStyle::Xml => OutputStyle::Xml,
            PackStyle::Json => OutputStyle::Json,
            PackStyle::Plain => OutputStyle::Plain,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReportOptions {
    pub workspace: PathBuf,
    pub format: ReportFormat,
}

impl ReportOptions {
    pub fn new(workspace: impl Into<PathBuf>, format: ReportFormat) -> Self {
        Self {
            workspace: workspace.into(),
            format,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Markdown,
    Json,
}

#[derive(Debug, Clone)]
pub enum KnowledgeQuery {
    Symbol(String),
    File(String),
    Impact(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeIndex {
    pub schema_version: u32,
    pub workspace_fingerprint: String,
    pub workspace_root: String,
    pub files: Vec<FileSummary>,
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
    pub imports: Vec<ImportSummary>,
    pub hotspots: Vec<Hotspot>,
    pub failures: Vec<ParseFailure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSummary {
    pub path: String,
    pub language: String,
    pub line_count: usize,
    pub byte_count: usize,
    pub node_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: u64,
    pub kind: String,
    pub name: String,
    pub file: Option<String>,
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
    pub signature: Option<String>,
    pub visibility: Option<String>,
    pub unresolved_calls: Vec<String>,
    pub properties: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub id: u64,
    pub source: u64,
    pub target: u64,
    pub kind: String,
    pub properties: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSummary {
    pub file: String,
    pub module: String,
    pub external: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hotspot {
    pub node_id: u64,
    pub name: String,
    pub file: Option<String>,
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
    pub score: usize,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseFailure {
    pub path: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackSummary {
    pub output_paths: Vec<String>,
    pub total_files: usize,
    pub total_characters: usize,
    pub total_tokens: usize,
    pub git_diff_tokens: usize,
    pub git_log_tokens: usize,
    pub top_files_by_tokens: Vec<(String, usize)>,
    pub suspicious_files: Vec<SuspiciousFile>,
    pub skipped_files: Vec<SkippedFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousFile {
    pub path: String,
    pub line: usize,
    pub message: String,
    pub rule_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkippedFile {
    pub path: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeReport {
    pub workspace_fingerprint: String,
    pub files_indexed: usize,
    pub nodes: usize,
    pub edges: usize,
    pub call_edges: usize,
    pub functions_with_complexity: usize,
    pub parse_failures: Vec<ParseFailure>,
    pub pack: PackSummary,
    pub hotspots: Vec<Hotspot>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureGraph {
    pub schema_version: u32,
    pub workspace_fingerprint: String,
    pub workspace_root: String,
    pub source_graph: ArchitectureSourceGraph,
    pub context_package: ArchitectureContextPackage,
    pub components: Vec<ArchitectureComponent>,
    pub integration_surfaces: Vec<IntegrationSurface>,
    pub automation_stages: Vec<AutomationStage>,
    pub edges: Vec<ArchitectureEdge>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureSourceGraph {
    pub provider: String,
    pub files: usize,
    pub nodes: usize,
    pub edges: usize,
    pub languages: Vec<String>,
    pub hotspots: usize,
    pub parse_failures: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureContextPackage {
    pub provider: String,
    pub files: usize,
    pub tokens: usize,
    pub output_style: PackStyle,
    pub top_files_by_tokens: Vec<(String, usize)>,
    pub suspicious_files: usize,
    pub skipped_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureComponent {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub files: usize,
    pub nodes: usize,
    pub edges: usize,
    pub languages: Vec<String>,
    pub hotspots: Vec<Hotspot>,
    pub evidence_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSurface {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub provider: String,
    pub default_scope: String,
    pub capabilities: Vec<String>,
    pub evidence_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationStage {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub surfaces: Vec<String>,
    pub artifact_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureEdge {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ArchitectureOptions {
    pub workspace: PathBuf,
    pub format: ArchitectureFormat,
}

impl ArchitectureOptions {
    pub fn new(workspace: impl Into<PathBuf>, format: ArchitectureFormat) -> Self {
        Self {
            workspace: workspace.into(),
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArchitectureDiagramOptions {
    pub workspace: PathBuf,
}

impl ArchitectureDiagramOptions {
    pub fn new(workspace: impl Into<PathBuf>) -> Self {
        Self {
            workspace: workspace.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchitectureFormat {
    Markdown,
    Json,
}

#[derive(Debug, Clone)]
pub struct RefreshArtifacts {
    pub index: PathBuf,
    pub report: PathBuf,
    pub architecture_json: PathBuf,
    pub architecture_markdown: PathBuf,
}

#[derive(Debug, Clone)]
pub struct SystemArchitectureOptions {
    pub workspace: PathBuf,
    pub system_root: PathBuf,
    pub format: ArchitectureFormat,
}

impl SystemArchitectureOptions {
    pub fn new(
        workspace: impl Into<PathBuf>,
        system_root: impl Into<PathBuf>,
        format: ArchitectureFormat,
    ) -> Self {
        Self {
            workspace: workspace.into(),
            system_root: system_root.into(),
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OperatingModelOptions {
    pub workspace: PathBuf,
    pub system_architecture_path: Option<PathBuf>,
    pub format: PlanContextFormat,
}

impl OperatingModelOptions {
    pub fn new(workspace: impl Into<PathBuf>, format: PlanContextFormat) -> Self {
        Self {
            workspace: workspace.into(),
            system_architecture_path: None,
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationPlanOptions {
    pub workspace: PathBuf,
    pub operating_model_path: Option<PathBuf>,
    pub format: PlanContextFormat,
}

impl IntegrationPlanOptions {
    pub fn new(workspace: impl Into<PathBuf>, format: PlanContextFormat) -> Self {
        Self {
            workspace: workspace.into(),
            operating_model_path: None,
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationStatusOptions {
    pub workspace: PathBuf,
    pub integration_plan_path: Option<PathBuf>,
    pub format: PlanContextFormat,
}

impl IntegrationStatusOptions {
    pub fn new(workspace: impl Into<PathBuf>, format: PlanContextFormat) -> Self {
        Self {
            workspace: workspace.into(),
            integration_plan_path: None,
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationOwnersOptions {
    pub workspace: PathBuf,
    pub integration_plan_path: Option<PathBuf>,
    pub system_architecture_path: Option<PathBuf>,
    pub change: Option<String>,
    pub capability: Option<String>,
    pub work_item: Option<String>,
    pub next: bool,
    pub next_planned: bool,
    pub format: PlanContextFormat,
}

impl IntegrationOwnersOptions {
    pub fn new(workspace: impl Into<PathBuf>, format: PlanContextFormat) -> Self {
        Self {
            workspace: workspace.into(),
            integration_plan_path: None,
            system_architecture_path: None,
            change: None,
            capability: None,
            work_item: None,
            next: false,
            next_planned: false,
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationReadinessOptions {
    pub workspace: PathBuf,
    pub integration_plan_path: Option<PathBuf>,
    pub system_architecture_path: Option<PathBuf>,
    pub change: Option<String>,
    pub capability: Option<String>,
    pub work_item: Option<String>,
    pub next: bool,
    pub next_planned: bool,
    pub format: PlanContextFormat,
}

impl IntegrationReadinessOptions {
    pub fn new(workspace: impl Into<PathBuf>, format: PlanContextFormat) -> Self {
        Self {
            workspace: workspace.into(),
            integration_plan_path: None,
            system_architecture_path: None,
            change: None,
            capability: None,
            work_item: None,
            next: false,
            next_planned: false,
            format,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemArchitectureGraph {
    pub schema_version: u32,
    pub workspace_root: String,
    pub system_root: String,
    pub discovery_source: String,
    pub repos: Vec<SystemRepo>,
    pub roles: Vec<SystemRole>,
    pub edges: Vec<SystemArchitectureEdge>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRepo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub head: Option<String>,
    pub dirty: bool,
    pub tags: Vec<String>,
    pub markers: Vec<String>,
    pub roles: Vec<String>,
    pub has_local_architecture_graph: bool,
    #[serde(default)]
    pub local_architecture: Option<PeerArchitectureSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerArchitectureSummary {
    pub schema_version: u32,
    pub source_graph: ArchitectureSourceGraph,
    pub context_package: ArchitectureContextPackage,
    pub component_count: usize,
    pub integration_surface_count: usize,
    pub top_components: Vec<PeerArchitectureComponentSummary>,
    pub integration_surfaces: Vec<PeerIntegrationSurfaceSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerArchitectureComponentSummary {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub files: usize,
    pub nodes: usize,
    pub edges: usize,
    pub languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerIntegrationSurfaceSummary {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub provider: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRole {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub repos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemArchitectureEdge {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemOperatingModel {
    pub schema_version: u32,
    pub workspace_root: String,
    pub system_root: String,
    pub source_graph: String,
    pub layers: Vec<OperatingModelLayer>,
    pub capabilities: Vec<OperatingCapability>,
    pub edges: Vec<OperatingModelEdge>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingModelLayer {
    pub id: String,
    pub name: String,
    pub purpose: String,
    pub capabilities: Vec<String>,
    pub repos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingCapability {
    pub id: String,
    pub name: String,
    pub layer: String,
    pub purpose: String,
    pub status: String,
    pub repos: Vec<String>,
    pub anchors: Vec<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingModelEdge {
    pub source: String,
    pub target: String,
    pub kind: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAutomationPlan {
    pub schema_version: u32,
    pub workspace_root: String,
    pub system_root: String,
    pub source_model: String,
    pub work_items: Vec<IntegrationWorkItem>,
    pub gates: Vec<String>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationWorkItem {
    pub id: String,
    pub title: String,
    pub capability: String,
    pub layer: String,
    pub priority: u32,
    pub status: String,
    pub change_id: String,
    pub owner_repos: Vec<String>,
    pub anchors: Vec<String>,
    pub adopt_first_inputs: Vec<String>,
    pub implementation_boundary: String,
    pub validation: Vec<String>,
    pub rollback: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationStatusReport {
    pub schema_version: u32,
    pub workspace_root: String,
    pub source_plan: String,
    pub next_change_id: Option<String>,
    pub counts: IntegrationStatusCounts,
    pub work_items: Vec<IntegrationWorkStatus>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IntegrationStatusCounts {
    pub total: usize,
    pub planned: usize,
    pub incomplete_scaffold: usize,
    pub scaffolded: usize,
    pub ready_to_archive: usize,
    pub archived: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationWorkStatus {
    pub id: String,
    pub title: String,
    pub capability: String,
    pub priority: u32,
    pub change_id: String,
    pub status: String,
    pub openspec_path: Option<String>,
    pub missing_artifacts: Vec<String>,
    pub unchecked_tasks: usize,
    pub owner_repos: Vec<String>,
    pub adopt_first_inputs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationOwnersReport {
    pub schema_version: u32,
    pub workspace_root: String,
    pub source_plan: String,
    pub source_system_architecture: String,
    pub selector: IntegrationOwnerSelector,
    pub work_item: IntegrationWorkItem,
    pub owner_surfaces: Vec<IntegrationOwnerSurface>,
    pub missing_owner_repos: Vec<String>,
    pub diagnostics: Vec<String>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationOwnerSelector {
    pub change: Option<String>,
    pub capability: Option<String>,
    pub work_item: Option<String>,
    pub next: bool,
    pub next_planned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationOwnerSurface {
    pub owner_repo: String,
    pub repo_found: bool,
    pub repo_name: Option<String>,
    pub path: Option<String>,
    pub remote: Option<String>,
    pub branch: Option<String>,
    pub head: Option<String>,
    pub dirty: bool,
    pub tags: Vec<String>,
    pub markers: Vec<String>,
    pub roles: Vec<String>,
    pub has_local_architecture_graph: bool,
    pub local_architecture: Option<PeerArchitectureSummary>,
    pub evidence_paths: Vec<String>,
    pub native_diagnostic_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationReadinessReport {
    pub schema_version: u32,
    pub workspace_root: String,
    pub source_plan: String,
    pub source_system_architecture: String,
    pub selector: IntegrationOwnerSelector,
    pub work_item: IntegrationWorkItem,
    pub owner_states: Vec<IntegrationReadinessOwnerState>,
    #[serde(default)]
    pub upstream_inputs: Vec<IntegrationUpstreamInput>,
    pub tool_requirements: Vec<IntegrationToolRequirement>,
    pub native_diagnostics: Vec<IntegrationNativeDiagnostic>,
    pub runtime_assumptions: Vec<String>,
    pub feature_gates: Vec<String>,
    pub validation: Vec<String>,
    pub rollback: Vec<String>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationReadinessOwnerState {
    pub owner_repo: String,
    pub repo_found: bool,
    pub repo_name: Option<String>,
    pub path: Option<String>,
    pub branch: Option<String>,
    pub head: Option<String>,
    pub dirty: bool,
    pub required_tool_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationUpstreamInput {
    pub source: String,
    pub kind: String,
    pub mirror_path: String,
    pub required_tool_ids: Vec<String>,
    pub native_diagnostic_commands: Vec<String>,
    pub runtime_assumptions: Vec<String>,
    pub feature_flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationToolRequirement {
    pub id: String,
    pub name: String,
    pub required_by: Vec<String>,
    pub provisioned_by: String,
    pub default_path: bool,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationNativeDiagnostic {
    pub command: String,
    pub owner_repo: Option<String>,
    pub required_tool_ids: Vec<String>,
    pub mode: String,
    pub mutates_repo: bool,
}

#[derive(Debug, Clone)]
pub struct PlanContextOptions {
    pub workspace: PathBuf,
    pub out_format: PlanContextFormat,
    pub goal: Option<String>,
    pub change: Option<String>,
    pub architecture_path: Option<PathBuf>,
    pub system_architecture_path: Option<PathBuf>,
    pub operating_model_path: Option<PathBuf>,
    pub integration_plan_path: Option<PathBuf>,
}

impl PlanContextOptions {
    pub fn new(workspace: impl Into<PathBuf>, out_format: PlanContextFormat) -> Self {
        Self {
            workspace: workspace.into(),
            out_format,
            goal: None,
            change: None,
            architecture_path: None,
            system_architecture_path: None,
            operating_model_path: None,
            integration_plan_path: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanContextFormat {
    Markdown,
    Json,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPlanningContext {
    pub schema_version: u32,
    pub change: Option<String>,
    pub goal: Option<String>,
    pub workspace_root: String,
    pub source_graph: ArchitectureSourceGraph,
    pub context_package: ArchitectureContextPackage,
    pub automation_stages: Vec<AutomationStage>,
    pub integration_surfaces: Vec<IntegrationSurface>,
    pub focus_components: Vec<ArchitectureComponent>,
    pub system_roles: Vec<SystemRole>,
    pub system_repos: Vec<SystemRepo>,
    pub operating_layers: Vec<OperatingModelLayer>,
    pub operating_capabilities: Vec<OperatingCapability>,
    pub integration_work_items: Vec<IntegrationWorkItem>,
    pub guidance: Vec<String>,
    pub findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub title: String,
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
    pub notes: Vec<String>,
}

pub fn index_workspace(options: IndexOptions) -> Result<KnowledgeIndex> {
    let workspace = canonical_workspace(&options.workspace)?;
    let fingerprint = workspace_fingerprint(&workspace).map_err(anyhow::Error::msg)?;
    let registry = LanguageRegistry::new();
    let source_files =
        collect_supported_source_files(&workspace, options.max_file_bytes, &registry)?;
    let mut parsed_files = Vec::new();
    let mut failures = Vec::new();

    for source_file in source_files {
        match parse_source_file_with_codegraph(&source_file.path, source_file.language, &registry) {
            Ok(parsed) => parsed_files.push(parsed),
            Err(error) => failures.push(ParseFailure {
                path: display_path(&workspace, &source_file.path),
                error: error.to_string(),
            }),
        }
    }

    let mut next_node_id = 1;
    let mut files = Vec::new();
    let mut nodes = Vec::new();
    let mut upstream_to_local = BTreeMap::new();
    let mut file_node_by_path = BTreeMap::new();
    let mut upstream_edges = Vec::new();

    for parsed in parsed_files {
        let file_id = next_node_id;
        next_node_id += 1;
        let rel = display_path(&workspace, &parsed.path);
        let raw_path = parsed.path.display().to_string();
        files.push(FileSummary {
            path: rel.clone(),
            language: language_name(&parsed.language),
            line_count: parsed.line_count,
            byte_count: parsed.byte_count,
            node_id: file_id,
        });
        file_node_by_path.insert(raw_path.clone(), file_id);
        nodes.push(file_node_to_dto(
            file_id,
            &rel,
            &parsed.language,
            parsed.line_count,
            parsed.byte_count,
        ));

        for mut upstream_node in parsed.nodes {
            let id = next_node_id;
            next_node_id += 1;
            let upstream_run_id = upstream_node.id.to_string();
            upstream_node.set_deterministic_id(&workspace.display().to_string());
            upstream_to_local.insert(upstream_run_id, id);
            nodes.push(upstream_node_to_dto(id, &upstream_node, &workspace));
        }
        upstream_edges.extend(parsed.edges);
    }

    let mut edge_id = 1;
    let mut edges = Vec::new();
    add_containment_edges(
        &nodes,
        &file_node_by_path,
        &workspace,
        &mut edges,
        &mut edge_id,
    );
    add_upstream_edges(
        &mut nodes,
        &upstream_edges,
        &upstream_to_local,
        &mut edges,
        &mut edge_id,
        &mut next_node_id,
    );

    nodes.sort_by_key(|node| node.id);
    edges.sort_by_key(|edge| edge.id);

    let imports = imports_from_nodes(&nodes);
    let hotspots = derive_hotspots(&nodes, &edges);

    Ok(KnowledgeIndex {
        schema_version: 2,
        workspace_fingerprint: fingerprint,
        workspace_root: workspace.display().to_string(),
        files,
        nodes,
        edges,
        imports,
        hotspots,
        failures,
    })
}

pub fn pack_workspace(options: PackWorkspaceOptions) -> Result<PackSummary> {
    let workspace = canonical_workspace(&options.workspace)?;
    if let Some(parent) = options.out.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("create output directory {}", parent.display()))?;
    }

    let mut ignore_patterns = vec![
        ".idd/knowledge/**".to_string(),
        "target/**".to_string(),
        ".git/**".to_string(),
        "third_party/upstream/**".to_string(),
    ];
    ignore_patterns.extend(options.ignore_patterns);

    let partial_config = PartialConfig {
        include: optional_vec(options.include_patterns),
        ignore: Some(ignore_patterns),
        style: Some(options.style.into()),
        compress: optional_bool(options.compress),
        remove_comments: optional_bool(options.remove_comments),
        remove_empty_lines: optional_bool(options.remove_empty_lines),
        show_line_numbers: optional_bool(options.show_line_numbers),
        truncate_base64: optional_bool(options.truncate_base64),
        copy_to_clipboard: Some(false),
        output: Some(options.out.display().to_string()),
        include_empty_directories: optional_bool(options.include_empty_directories),
        top_files_length: options.top_files_length,
        split_output: options.split_output,
        header_text: options.header_text,
        instruction_file_path: options
            .instruction_file_path
            .map(|path| path.display().to_string()),
        include_diffs: optional_bool(options.include_diffs),
        include_logs: optional_bool(options.include_logs),
    };
    let mut config = repomix_config::schema::RepomixConfig::load(Some(partial_config), &workspace)
        .context("load repomix config")?;
    config.output.parsable_style = true;
    config.output.json.no_timestamp = true;

    let rt = tokio::runtime::Runtime::new().context("create repomix runtime")?;
    let result = rt
        .block_on(repomix_core::pack_with_options(
            RepomixPackOptions::new(workspace).with_config(config),
        ))
        .context("pack workspace with repomix-core")?;

    if result.total_tokens > options.max_tokens {
        for path in &result.output_paths {
            let _ = fs::remove_file(path);
        }
        bail!(
            "packed context is {} tokens, over budget {}; narrow includes or raise --max-tokens",
            result.total_tokens,
            options.max_tokens
        );
    }

    Ok(pack_result_to_summary(result))
}

pub fn build_knowledge_report(options: ReportOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let index = index_workspace(IndexOptions::new(&workspace))?;
    let tmp = tempfile::tempdir().context("create temporary report pack directory")?;
    let pack_out = tmp.path().join("report-pack.md");
    let mut pack_options = PackWorkspaceOptions::new(&workspace, &pack_out, PackStyle::Markdown);
    pack_options.max_tokens = GENERATED_ARTIFACT_MAX_PACK_TOKENS;
    pack_options.compress = true;
    pack_options.remove_comments = true;
    pack_options.remove_empty_lines = true;
    pack_options.top_files_length = Some(20);
    pack_options.include_patterns = default_report_include_patterns();
    pack_options
        .ignore_patterns
        .push("crates/external/**".to_string());
    pack_options
        .ignore_patterns
        .push("third_party/upstream/**".to_string());
    pack_options.ignore_patterns.push("AI_MERGE/**".to_string());
    let pack = pack_workspace(pack_options)?;
    let report = report_from_parts(&index, pack);

    match options.format {
        ReportFormat::Markdown => Ok(render_report_markdown(&report)),
        ReportFormat::Json => serde_json::to_string_pretty(&report).context("serialize report"),
    }
}

pub fn build_architecture_graph(options: ArchitectureOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let index = index_workspace(IndexOptions::new(&workspace))?;
    let pack = build_architecture_pack_summary(&workspace)?;
    let graph = architecture_graph_from_parts(&workspace, &index, pack);

    match options.format {
        ArchitectureFormat::Markdown => Ok(render_architecture_markdown(&graph)),
        ArchitectureFormat::Json => serde_json::to_string_pretty(&graph).context("serialize graph"),
    }
}

pub fn build_architecture_diagrams(options: ArchitectureDiagramOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let index = index_workspace(IndexOptions::new(&workspace))?;
    let pack = build_architecture_pack_summary(&workspace)?;
    let graph = architecture_graph_from_parts(&workspace, &index, pack);
    Ok(render_architecture_diagrams(&graph))
}

pub fn build_system_architecture_graph(options: SystemArchitectureOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let system_root = canonical_workspace(&options.system_root)?;
    let graph = system_architecture_graph(&workspace, &system_root)?;

    match options.format {
        ArchitectureFormat::Markdown => Ok(render_system_architecture_markdown(&graph)),
        ArchitectureFormat::Json => serde_json::to_string_pretty(&graph).context("serialize graph"),
    }
}

pub fn build_system_operating_model(options: OperatingModelOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let system_path = options
        .system_architecture_path
        .unwrap_or_else(|| workspace.join(".idd/knowledge/system-architecture.json"));
    let system = read_json_file::<SystemArchitectureGraph>(&system_path)?;
    let model = system_operating_model(&workspace, &system, &system_path);

    match options.format {
        PlanContextFormat::Markdown => Ok(render_system_operating_model_markdown(&model)),
        PlanContextFormat::Json => {
            serde_json::to_string_pretty(&model).context("serialize operating model")
        }
    }
}

pub fn build_integration_automation_plan(options: IntegrationPlanOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let operating_path = options
        .operating_model_path
        .unwrap_or_else(|| workspace.join(".idd/knowledge/operating-model.json"));
    let operating_model = read_json_file::<SystemOperatingModel>(&operating_path)?;
    let plan = integration_automation_plan(&workspace, &operating_model, &operating_path);

    match options.format {
        PlanContextFormat::Markdown => Ok(render_integration_automation_plan_markdown(&plan)),
        PlanContextFormat::Json => {
            serde_json::to_string_pretty(&plan).context("serialize integration plan")
        }
    }
}

pub fn build_integration_status_report(options: IntegrationStatusOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let plan_path = options
        .integration_plan_path
        .unwrap_or_else(|| workspace.join(".idd/knowledge/integration-plan.json"));
    let plan = read_json_file::<IntegrationAutomationPlan>(&plan_path)?;
    let report = integration_status_report(&workspace, &plan, &plan_path);

    match options.format {
        PlanContextFormat::Markdown => Ok(render_integration_status_markdown(&report)),
        PlanContextFormat::Json => {
            serde_json::to_string_pretty(&report).context("serialize integration status")
        }
    }
}

pub fn build_integration_owner_surfaces(options: IntegrationOwnersOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let format = options.format;
    let plan_path = options
        .integration_plan_path
        .clone()
        .unwrap_or_else(|| workspace.join(".idd/knowledge/integration-plan.json"));
    let system_path = options
        .system_architecture_path
        .clone()
        .unwrap_or_else(|| workspace.join(".idd/knowledge/system-architecture.json"));
    let plan = read_json_file::<IntegrationAutomationPlan>(&plan_path)?;
    let system = read_json_file::<SystemArchitectureGraph>(&system_path)?;
    let report =
        integration_owner_surfaces(&workspace, &plan_path, &system_path, plan, system, options)?;

    match format {
        PlanContextFormat::Markdown => Ok(render_integration_owner_surfaces_markdown(&report)),
        PlanContextFormat::Json => {
            serde_json::to_string_pretty(&report).context("serialize integration owner surfaces")
        }
    }
}

pub fn build_integration_readiness_report(options: IntegrationReadinessOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let format = options.format;
    let plan_path = options
        .integration_plan_path
        .clone()
        .unwrap_or_else(|| workspace.join(".idd/knowledge/integration-plan.json"));
    let system_path = options
        .system_architecture_path
        .clone()
        .unwrap_or_else(|| workspace.join(".idd/knowledge/system-architecture.json"));
    let plan = read_json_file::<IntegrationAutomationPlan>(&plan_path)?;
    let system = read_json_file::<SystemArchitectureGraph>(&system_path)?;
    let owner_options = IntegrationOwnersOptions {
        workspace: options.workspace,
        integration_plan_path: options.integration_plan_path,
        system_architecture_path: options.system_architecture_path,
        change: options.change,
        capability: options.capability,
        work_item: options.work_item,
        next: options.next,
        next_planned: options.next_planned,
        format,
    };
    let owners = integration_owner_surfaces(
        &workspace,
        &plan_path,
        &system_path,
        plan,
        system,
        owner_options,
    )?;
    let report = integration_readiness_report(&workspace, &plan_path, &system_path, owners);

    match format {
        PlanContextFormat::Markdown => Ok(render_integration_readiness_markdown(&report)),
        PlanContextFormat::Json => {
            serde_json::to_string_pretty(&report).context("serialize integration readiness")
        }
    }
}

pub fn build_graph_planning_context(options: PlanContextOptions) -> Result<String> {
    let workspace = canonical_workspace(&options.workspace)?;
    let format = options.out_format;
    let context = graph_planning_context(&workspace, options)?;
    match format {
        PlanContextFormat::Markdown => Ok(render_graph_planning_context_markdown(&context)),
        PlanContextFormat::Json => {
            serde_json::to_string_pretty(&context).context("serialize planning context")
        }
    }
}

fn default_report_include_patterns() -> Vec<String> {
    [
        "AGENTS.md",
        "Cargo.toml",
        "Makefile",
        "Justfile",
        "crates/**/*.rs",
        "crates/**/Cargo.toml",
        "AI_MERGE/*.md",
        "adr/*.md",
        "docs/rusty-idd/*.md",
        ".codex/**/*.toml",
        ".codex/**/*.json",
        ".agents/skills/**/SKILL.md",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .collect()
}

pub fn query_knowledge_index(index: &KnowledgeIndex, query: KnowledgeQuery) -> QueryResult {
    match query {
        KnowledgeQuery::Symbol(symbol) => query_symbol(index, &symbol),
        KnowledgeQuery::File(file) => query_file(index, &file),
        KnowledgeQuery::Impact(node_id) => query_impact(index, node_id),
    }
}

pub fn refresh_workspace(workspace: impl AsRef<Path>) -> Result<RefreshArtifacts> {
    let workspace = canonical_workspace(workspace.as_ref())?;
    let out_dir = workspace.join(".idd/knowledge");
    fs::create_dir_all(&out_dir).context("create .idd/knowledge")?;

    let index = index_workspace(IndexOptions::new(&workspace))?;
    let index_path = out_dir.join("index.json");
    write_json(&index_path, &index)?;

    let report = build_knowledge_report(ReportOptions::new(&workspace, ReportFormat::Markdown))?;
    let report_path = out_dir.join("report.md");
    write_text(&report_path, &report)?;

    let architecture_json = build_architecture_graph(ArchitectureOptions::new(
        &workspace,
        ArchitectureFormat::Json,
    ))?;
    let architecture_json_path = out_dir.join("architecture.json");
    write_text(&architecture_json_path, &(architecture_json + "\n"))?;

    let architecture_markdown = build_architecture_graph(ArchitectureOptions::new(
        &workspace,
        ArchitectureFormat::Markdown,
    ))?;
    let architecture_markdown_path = out_dir.join("architecture.md");
    write_text(&architecture_markdown_path, &architecture_markdown)?;

    Ok(RefreshArtifacts {
        index: index_path,
        report: report_path,
        architecture_json: architecture_json_path,
        architecture_markdown: architecture_markdown_path,
    })
}

pub fn load_index(path: impl AsRef<Path>) -> Result<KnowledgeIndex> {
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .with_context(|| format!("read knowledge index {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("parse knowledge index {}", path.display()))
}

#[derive(Debug)]
struct SourceFileCandidate {
    path: PathBuf,
    language: UpstreamLanguage,
}

#[derive(Debug)]
struct ParsedSourceFile {
    path: PathBuf,
    language: UpstreamLanguage,
    line_count: usize,
    byte_count: usize,
    nodes: Vec<UpstreamCodeNode>,
    edges: Vec<UpstreamEdge>,
}

fn parse_source_file_with_codegraph(
    path: &Path,
    language: UpstreamLanguage,
    registry: &LanguageRegistry,
) -> Result<ParsedSourceFile> {
    let source = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let mut parser = registry
        .create_parser(&language)
        .with_context(|| format!("initialize {:?} parser", language))?;
    let tree = parser
        .parse(&source, None)
        .ok_or_else(|| anyhow::anyhow!("parse {}", path.display()))?;
    let extraction = extract_for_language(&language, &tree, &source, &path.display().to_string())
        .with_context(|| format!("extract {:?} symbols", language))?;

    Ok(ParsedSourceFile {
        path: path.to_path_buf(),
        language,
        line_count: source.lines().count(),
        byte_count: source.len(),
        nodes: extraction.nodes,
        edges: extraction.edges,
    })
}

fn file_node_to_dto(
    id: u64,
    rel_path: &str,
    language: &UpstreamLanguage,
    line_count: usize,
    byte_count: usize,
) -> KnowledgeNode {
    KnowledgeNode {
        id,
        kind: "CodeFile".to_string(),
        name: rel_path.to_string(),
        file: Some(rel_path.to_string()),
        line_start: Some(1),
        line_end: Some(line_count.max(1)),
        signature: None,
        visibility: None,
        unresolved_calls: Vec::new(),
        properties: BTreeMap::from([
            (
                "language".to_string(),
                serde_json::json!(language_name(language)),
            ),
            ("line_count".to_string(), serde_json::json!(line_count)),
            ("byte_count".to_string(), serde_json::json!(byte_count)),
        ]),
    }
}

fn upstream_node_to_dto(id: u64, node: &UpstreamCodeNode, workspace: &Path) -> KnowledgeNode {
    let mut properties = upstream_metadata_to_json(node);
    properties.insert(
        "upstream_id".to_string(),
        serde_json::json!(node.id.to_string()),
    );
    if let Some(language) = &node.language {
        properties.insert(
            "language".to_string(),
            serde_json::json!(format!("{language:?}")),
        );
    }
    if let Some(complexity) = node.complexity {
        properties.insert(
            "cyclomatic_complexity".to_string(),
            json_number_from_f32(complexity),
        );
    }
    if let Some(span) = &node.span {
        properties.insert("start_byte".to_string(), serde_json::json!(span.start_byte));
        properties.insert("end_byte".to_string(), serde_json::json!(span.end_byte));
    }

    let content = node.content.as_deref();
    KnowledgeNode {
        id,
        kind: upstream_node_kind(node.node_type.as_ref()),
        name: node.name.to_string(),
        file: Some(display_path(workspace, Path::new(&node.location.file_path))),
        line_start: Some(node.location.line as usize),
        line_end: node.location.end_line.map(|line| line as usize),
        signature: content.and_then(first_signature_line),
        visibility: properties
            .get("visibility")
            .and_then(|value| value.as_str())
            .map(ToOwned::to_owned),
        unresolved_calls: Vec::new(),
        properties,
    }
}

fn upstream_metadata_to_json(node: &UpstreamCodeNode) -> BTreeMap<String, serde_json::Value> {
    node.metadata
        .attributes
        .iter()
        .map(|(key, value)| (key.clone(), parse_metadata_value(value)))
        .collect()
}

fn parse_metadata_value(value: &str) -> serde_json::Value {
    serde_json::from_str(value).unwrap_or_else(|_| serde_json::Value::String(value.to_string()))
}

fn json_number_from_f32(value: f32) -> serde_json::Value {
    if value.is_finite() && value.fract() == 0.0 {
        serde_json::json!(value as u64)
    } else {
        serde_json::Number::from_f64(value.into())
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null)
    }
}

fn first_signature_line(content: &str) -> Option<String> {
    content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(|line| line.chars().take(240).collect())
}

fn upstream_node_kind(kind: Option<&UpstreamNodeType>) -> String {
    match kind {
        Some(UpstreamNodeType::Function) => "Function",
        Some(UpstreamNodeType::Struct) => "Struct",
        Some(UpstreamNodeType::Enum) => "Enum",
        Some(UpstreamNodeType::Trait) => "Trait",
        Some(UpstreamNodeType::Module) => "Module",
        Some(UpstreamNodeType::Directory) => "Directory",
        Some(UpstreamNodeType::Variable) => "Variable",
        Some(UpstreamNodeType::Import) => "Import",
        Some(UpstreamNodeType::Class) => "Class",
        Some(UpstreamNodeType::Interface) => "Interface",
        Some(UpstreamNodeType::Type) => "Type",
        Some(UpstreamNodeType::Other(value)) => value.as_str(),
        None => "Unknown",
    }
    .to_string()
}

fn add_containment_edges(
    nodes: &[KnowledgeNode],
    file_node_by_path: &BTreeMap<String, u64>,
    workspace: &Path,
    edges: &mut Vec<KnowledgeEdge>,
    edge_id: &mut u64,
) {
    let file_node_ids = file_node_by_path.values().copied().collect::<BTreeSet<_>>();
    for node in nodes {
        if file_node_ids.contains(&node.id) {
            continue;
        }
        let Some(file) = &node.file else {
            continue;
        };
        let raw_path = workspace.join(file).display().to_string();
        let Some(source) = file_node_by_path.get(&raw_path).copied() else {
            continue;
        };
        edges.push(KnowledgeEdge {
            id: *edge_id,
            source,
            target: node.id,
            kind: "Contains".to_string(),
            properties: BTreeMap::new(),
        });
        *edge_id += 1;
    }
}

fn add_upstream_edges(
    nodes: &mut Vec<KnowledgeNode>,
    upstream_edges: &[UpstreamEdge],
    upstream_to_local: &BTreeMap<String, u64>,
    edges: &mut Vec<KnowledgeEdge>,
    edge_id: &mut u64,
    next_node_id: &mut u64,
) {
    let mut resolver = EdgeResolver::new(nodes);
    let mut external_nodes = BTreeMap::<(String, String), u64>::new();
    let mut unresolved_calls = BTreeMap::<u64, BTreeSet<String>>::new();

    for upstream_edge in upstream_edges {
        let Some(source) = upstream_to_local
            .get(&upstream_edge.from.to_string())
            .copied()
        else {
            continue;
        };
        let kind = upstream_edge_kind(&upstream_edge.edge_type);
        let (target, resolution) = resolver
            .resolve(source, &upstream_edge.to)
            .map(|target| (target, "local".to_string()))
            .unwrap_or_else(|| {
                let key = (kind.clone(), upstream_edge.to.clone());
                let target = *external_nodes.entry(key).or_insert_with(|| {
                    let id = *next_node_id;
                    *next_node_id += 1;
                    nodes.push(unresolved_symbol_node(id, &kind, &upstream_edge.to));
                    resolver.add_node(nodes.last().expect("just pushed unresolved symbol"));
                    id
                });
                if kind == "Calls" {
                    unresolved_calls
                        .entry(source)
                        .or_default()
                        .insert(upstream_edge.to.clone());
                }
                (target, "unresolved".to_string())
            });

        let mut properties = upstream_edge_metadata_to_json(upstream_edge);
        properties.insert("target".to_string(), serde_json::json!(upstream_edge.to));
        properties.insert("resolution".to_string(), serde_json::json!(resolution));
        edges.push(KnowledgeEdge {
            id: *edge_id,
            source,
            target,
            kind,
            properties,
        });
        *edge_id += 1;
    }

    let node_offsets = nodes
        .iter()
        .enumerate()
        .map(|(offset, node)| (node.id, offset))
        .collect::<BTreeMap<_, _>>();
    for (id, calls) in unresolved_calls {
        if let Some(offset) = node_offsets.get(&id).copied() {
            let calls = calls.iter().cloned().collect::<Vec<_>>();
            nodes[offset].unresolved_calls = calls.clone();
            nodes[offset]
                .properties
                .insert("unresolved_calls".to_string(), serde_json::json!(calls));
        }
    }
}

fn unresolved_symbol_node(id: u64, edge_kind: &str, target: &str) -> KnowledgeNode {
    KnowledgeNode {
        id,
        kind: if edge_kind == "Imports" {
            "ImportedSymbol".to_string()
        } else {
            "UnresolvedSymbol".to_string()
        },
        name: target.to_string(),
        file: None,
        line_start: None,
        line_end: None,
        signature: None,
        visibility: None,
        unresolved_calls: Vec::new(),
        properties: BTreeMap::from([
            ("unresolved".to_string(), serde_json::json!(true)),
            ("target".to_string(), serde_json::json!(target)),
        ]),
    }
}

fn upstream_edge_metadata_to_json(edge: &UpstreamEdge) -> BTreeMap<String, serde_json::Value> {
    let mut properties = edge
        .metadata
        .iter()
        .map(|(key, value)| (key.clone(), parse_metadata_value(value)))
        .collect::<BTreeMap<_, _>>();
    if let Some(span) = &edge.span {
        properties.insert("start_byte".to_string(), serde_json::json!(span.start_byte));
        properties.insert("end_byte".to_string(), serde_json::json!(span.end_byte));
    }
    properties
}

fn upstream_edge_kind(kind: &UpstreamEdgeType) -> String {
    match kind {
        UpstreamEdgeType::Calls => "Calls",
        UpstreamEdgeType::Defines => "Defines",
        UpstreamEdgeType::Uses => "Uses",
        UpstreamEdgeType::Imports => "Imports",
        UpstreamEdgeType::Extends => "Extends",
        UpstreamEdgeType::Implements => "Implements",
        UpstreamEdgeType::Contains => "Contains",
        UpstreamEdgeType::References => "References",
        UpstreamEdgeType::Other(value) => value.as_str(),
    }
    .to_string()
}

struct EdgeResolver {
    by_id: BTreeMap<u64, KnowledgeNode>,
    by_file_name: BTreeMap<(String, String), Vec<u64>>,
    by_name: BTreeMap<String, Vec<u64>>,
    by_qualified_name: BTreeMap<String, u64>,
}

impl EdgeResolver {
    fn new(nodes: &[KnowledgeNode]) -> Self {
        let mut resolver = Self {
            by_id: BTreeMap::new(),
            by_file_name: BTreeMap::new(),
            by_name: BTreeMap::new(),
            by_qualified_name: BTreeMap::new(),
        };
        for node in nodes {
            resolver.add_node(node);
        }
        resolver
    }

    fn add_node(&mut self, node: &KnowledgeNode) {
        self.by_id.insert(node.id, node.clone());
        self.by_name
            .entry(node.name.clone())
            .or_default()
            .push(node.id);
        if let Some(file) = &node.file {
            self.by_file_name
                .entry((file.clone(), node.name.clone()))
                .or_default()
                .push(node.id);
        }
        if let Some(qualified_name) = node
            .properties
            .get("qualified_name")
            .and_then(|value| value.as_str())
        {
            self.by_qualified_name
                .insert(qualified_name.to_string(), node.id);
        }
    }

    fn resolve(&self, source: u64, target: &str) -> Option<u64> {
        if let Some(id) = self.by_qualified_name.get(target).copied() {
            return Some(id);
        }

        let basename = symbol_basename(target);
        let source_file = self.by_id.get(&source).and_then(|node| node.file.as_ref());
        if let Some(file) = source_file
            && let Some(ids) = self.by_file_name.get(&(file.clone(), basename.clone()))
            && ids.len() == 1
        {
            return ids.first().copied();
        }

        if let Some(ids) = self.by_name.get(&basename)
            && ids.len() == 1
        {
            return ids.first().copied();
        }
        None
    }
}

fn symbol_basename(name: &str) -> String {
    name.rsplit("::")
        .next()
        .unwrap_or(name)
        .rsplit('.')
        .next()
        .unwrap_or(name)
        .trim_matches('!')
        .to_string()
}

fn canonical_workspace(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("workspace path does not exist: {}", path.display()))
}

fn optional_bool(value: bool) -> Option<bool> {
    value.then_some(true)
}

fn optional_vec(values: Vec<String>) -> Option<Vec<String>> {
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn collect_supported_source_files(
    workspace: &Path,
    max_file_bytes: u64,
    registry: &LanguageRegistry,
) -> Result<Vec<SourceFileCandidate>> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(workspace)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .build();

    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let path = entry.path();
        if should_skip_path(workspace, path) {
            continue;
        }
        if !entry
            .file_type()
            .map(|kind| kind.is_file())
            .unwrap_or(false)
        {
            continue;
        }
        let Some(language) = registry.detect_language(&path.display().to_string()) else {
            continue;
        };
        if fs::metadata(path)
            .map(|meta| meta.len())
            .unwrap_or(u64::MAX)
            > max_file_bytes
        {
            continue;
        }
        files.push(SourceFileCandidate {
            path: path.to_path_buf(),
            language,
        });
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(files)
}

fn language_name(language: &UpstreamLanguage) -> String {
    match language {
        UpstreamLanguage::Rust => "rust",
        UpstreamLanguage::TypeScript => "typescript",
        UpstreamLanguage::JavaScript => "javascript",
        UpstreamLanguage::Python => "python",
        UpstreamLanguage::Go => "go",
        UpstreamLanguage::Java => "java",
        UpstreamLanguage::Cpp => "cpp",
        UpstreamLanguage::Swift => "swift",
        UpstreamLanguage::Kotlin => "kotlin",
        UpstreamLanguage::CSharp => "csharp",
        UpstreamLanguage::Ruby => "ruby",
        UpstreamLanguage::Php => "php",
        UpstreamLanguage::Dart => "dart",
        UpstreamLanguage::Other(value) => value.as_str(),
    }
    .to_string()
}

fn should_skip_path(workspace: &Path, path: &Path) -> bool {
    let rel = path.strip_prefix(workspace).unwrap_or(path);
    rel.components().any(|component| {
        let name = component.as_os_str().to_string_lossy();
        matches!(
            name.as_ref(),
            ".git"
                | "target"
                | "node_modules"
                | ".idd-bak"
                | ".worktrees"
                | "_workspace"
                | "third_party"
        )
    }) || rel.starts_with(".idd/knowledge")
}

fn imports_from_nodes(nodes: &[KnowledgeNode]) -> Vec<ImportSummary> {
    let mut imports = Vec::new();
    for node in nodes.iter().filter(|node| node.kind == "Import") {
        if let Some(items) = node
            .properties
            .get("imports")
            .and_then(|value| value.as_array())
        {
            for item in items {
                if let Some(module) = item.get("full_path").and_then(|value| value.as_str()) {
                    imports.push(ImportSummary {
                        file: node.file.clone().unwrap_or_default(),
                        module: module.to_string(),
                        external: is_external_import(module),
                    });
                }
            }
        } else {
            imports.push(ImportSummary {
                file: node.file.clone().unwrap_or_default(),
                module: node.name.clone(),
                external: is_external_import(&node.name),
            });
        }
    }
    imports.sort_by(|a, b| a.module.cmp(&b.module).then(a.file.cmp(&b.file)));
    imports.dedup_by(|a, b| a.file == b.file && a.module == b.module);
    imports
}

fn is_external_import(module: &str) -> bool {
    !matches!(
        module.split("::").next().unwrap_or(module),
        "crate" | "self" | "super"
    )
}

fn derive_hotspots(nodes: &[KnowledgeNode], edges: &[KnowledgeEdge]) -> Vec<Hotspot> {
    let mut degree: BTreeMap<u64, usize> = BTreeMap::new();
    let mut call_degree: BTreeMap<u64, usize> = BTreeMap::new();
    for edge in edges {
        *degree.entry(edge.source).or_default() += 1;
        *degree.entry(edge.target).or_default() += 1;
        if edge.kind == "Calls" {
            *call_degree.entry(edge.source).or_default() += 1;
            *call_degree.entry(edge.target).or_default() += 1;
        }
    }

    let mut hotspots = nodes
        .iter()
        .filter(|node| {
            matches!(
                node.kind.as_str(),
                "Function" | "Struct" | "Enum" | "Trait" | "Class" | "Interface"
            )
        })
        .filter_map(|node| {
            let span = node
                .line_start
                .zip(node.line_end)
                .map(|(start, end)| end.saturating_sub(start) + 1)
                .unwrap_or(1);
            let links = degree.get(&node.id).copied().unwrap_or(0);
            let call_links = call_degree.get(&node.id).copied().unwrap_or(0);
            let unresolved = node.unresolved_calls.len();
            let complexity = node
                .properties
                .get("cyclomatic_complexity")
                .and_then(|value| value.as_u64())
                .unwrap_or(1) as usize;
            let score = span + links * 3 + call_links * 7 + unresolved * 3 + complexity * 4;
            if score < 25 {
                return None;
            }
            let mut reasons = Vec::new();
            if span >= 25 {
                reasons.push(format!("{span} lines"));
            }
            if links > 0 {
                reasons.push(format!("{links} graph links"));
            }
            if call_links > 0 {
                reasons.push(format!("{call_links} call links"));
            }
            if unresolved > 0 {
                reasons.push(format!("{unresolved} unresolved calls"));
            }
            if complexity >= 4 {
                reasons.push(format!("cyclomatic complexity {complexity}"));
            }
            Some(Hotspot {
                node_id: node.id,
                name: node.name.clone(),
                file: node.file.clone(),
                line_start: node.line_start,
                line_end: node.line_end,
                score,
                reasons,
            })
        })
        .collect::<Vec<_>>();

    hotspots.sort_by(|a, b| b.score.cmp(&a.score).then(a.name.cmp(&b.name)));
    hotspots.truncate(20);
    hotspots
}

fn pack_result_to_summary(result: repomix_core::PackResult) -> PackSummary {
    PackSummary {
        output_paths: result.output_paths,
        total_files: result.total_files,
        total_characters: result.total_characters,
        total_tokens: result.total_tokens,
        git_diff_tokens: result.git_diff_token_count,
        git_log_tokens: result.git_log_token_count,
        top_files_by_tokens: result.top_files_by_tokens,
        suspicious_files: result
            .suspicious_files
            .into_iter()
            .map(|file| SuspiciousFile {
                path: file.path.display().to_string(),
                line: file.line,
                message: file.message,
                rule_id: file.rule_id,
            })
            .collect(),
        skipped_files: result
            .skipped_files
            .into_iter()
            .map(|file| SkippedFile {
                path: file.path.display().to_string(),
                reason: file.reason,
            })
            .collect(),
    }
}

fn report_from_parts(index: &KnowledgeIndex, pack: PackSummary) -> KnowledgeReport {
    let mut findings = Vec::new();
    if !index.failures.is_empty() {
        findings.push(format!(
            "{} source files failed to parse",
            index.failures.len()
        ));
    }
    if !pack.suspicious_files.is_empty() {
        findings.push(format!(
            "{} suspicious files were excluded from packed context",
            pack.suspicious_files.len()
        ));
    }
    if pack.total_tokens > DEFAULT_MAX_PACK_TOKENS {
        findings.push(format!(
            "packed context is {} tokens, above default budget {}",
            pack.total_tokens, DEFAULT_MAX_PACK_TOKENS
        ));
    }

    KnowledgeReport {
        workspace_fingerprint: index.workspace_fingerprint.clone(),
        files_indexed: index.files.len(),
        nodes: index.nodes.len(),
        edges: index.edges.len(),
        call_edges: index
            .edges
            .iter()
            .filter(|edge| edge.kind == "Calls")
            .count(),
        functions_with_complexity: index
            .nodes
            .iter()
            .filter(|node| {
                node.kind == "Function" && node.properties.contains_key("cyclomatic_complexity")
            })
            .count(),
        parse_failures: index.failures.clone(),
        pack,
        hotspots: index.hotspots.clone(),
        findings,
    }
}

fn build_architecture_pack_summary(workspace: &Path) -> Result<PackSummary> {
    let tmp = tempfile::tempdir().context("create temporary architecture pack directory")?;
    let pack_out = tmp.path().join("architecture-pack.md");
    let mut pack_options = PackWorkspaceOptions::new(workspace, &pack_out, PackStyle::Markdown);
    pack_options.max_tokens = GENERATED_ARTIFACT_MAX_PACK_TOKENS;
    pack_options.compress = true;
    pack_options.remove_comments = true;
    pack_options.remove_empty_lines = true;
    pack_options.top_files_length = Some(25);
    pack_options.include_patterns = architecture_include_patterns();
    pack_options
        .ignore_patterns
        .push("third_party/upstream/**".to_string());
    pack_options
        .ignore_patterns
        .push("crates/external/**".to_string());
    pack_options
        .ignore_patterns
        .push("crates/tui/openspec/changes/archive/**".to_string());
    pack_options.ignore_patterns.push("AI_MERGE/**".to_string());
    pack_options
        .ignore_patterns
        .push("docs/rusty-idd/architecture-diagrams.md".to_string());
    pack_workspace(pack_options)
}

fn architecture_include_patterns() -> Vec<String> {
    [
        "AGENTS.md",
        "Cargo.toml",
        "Justfile",
        "Makefile",
        "crates/**/*.rs",
        "crates/**/Cargo.toml",
        "openspec/**/*.md",
        "adr/*.md",
        "AI_MERGE/*.md",
        "docs/**/*.md",
        ".agents/skills/**/SKILL.md",
        ".idd/MANIFEST.tsv",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .collect()
}

fn architecture_graph_from_parts(
    workspace: &Path,
    index: &KnowledgeIndex,
    pack: PackSummary,
) -> ArchitectureGraph {
    let components = architecture_components(index);
    let integration_surfaces = integration_surfaces(workspace);
    let automation_stages = automation_stages();
    let mut edges = architecture_component_edges(index);
    edges.extend(automation_stage_edges(&automation_stages));
    edges.extend(integration_component_edges(
        &integration_surfaces,
        &components,
    ));
    edges.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then(a.target.cmp(&b.target))
            .then(a.kind.cmp(&b.kind))
    });
    edges.dedup_by(|a, b| a.source == b.source && a.target == b.target && a.kind == b.kind);

    let mut findings = Vec::new();
    if index.failures.is_empty() {
        findings.push("CodeGraph-backed parsing completed without source failures".to_string());
    } else {
        findings.push(format!(
            "CodeGraph-backed parsing recorded {} source failures",
            index.failures.len()
        ));
    }
    findings.push(format!(
        "repomix context package measured {} files and {} tokens",
        pack.total_files, pack.total_tokens
    ));
    if !pack.suspicious_files.is_empty() {
        findings.push(format!(
            "repomix security scan reported {} suspicious files",
            pack.suspicious_files.len()
        ));
    }

    ArchitectureGraph {
        schema_version: 1,
        workspace_fingerprint: index.workspace_fingerprint.clone(),
        workspace_root: index.workspace_root.clone(),
        source_graph: ArchitectureSourceGraph {
            provider: "codegraph-rust".to_string(),
            files: index.files.len(),
            nodes: index.nodes.len(),
            edges: index.edges.len(),
            languages: indexed_languages(index),
            hotspots: index.hotspots.len(),
            parse_failures: index.failures.len(),
        },
        context_package: ArchitectureContextPackage {
            provider: "repomix-rs".to_string(),
            files: pack.total_files,
            tokens: pack.total_tokens,
            output_style: PackStyle::Markdown,
            top_files_by_tokens: pack.top_files_by_tokens,
            suspicious_files: pack.suspicious_files.len(),
            skipped_files: pack.skipped_files.len(),
        },
        components,
        integration_surfaces,
        automation_stages,
        edges,
        findings,
    }
}

fn indexed_languages(index: &KnowledgeIndex) -> Vec<String> {
    index
        .files
        .iter()
        .map(|file| file.language.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[derive(Default)]
struct ComponentAccumulator {
    id: String,
    name: String,
    kind: String,
    files: BTreeSet<String>,
    nodes: BTreeSet<u64>,
    edges: BTreeSet<u64>,
    languages: BTreeSet<String>,
    hotspots: Vec<Hotspot>,
    evidence_paths: BTreeSet<String>,
}

fn architecture_components(index: &KnowledgeIndex) -> Vec<ArchitectureComponent> {
    let mut components = BTreeMap::<String, ComponentAccumulator>::new();
    let mut node_component = BTreeMap::<u64, String>::new();

    for file in &index.files {
        let info = component_info_for_path(&file.path);
        let component = components
            .entry(info.id.clone())
            .or_insert_with(|| ComponentAccumulator {
                id: info.id.clone(),
                name: info.name.clone(),
                kind: info.kind.clone(),
                ..ComponentAccumulator::default()
            });
        component.files.insert(file.path.clone());
        component.nodes.insert(file.node_id);
        component.languages.insert(file.language.clone());
        component.evidence_paths.insert(file.path.clone());
        node_component.insert(file.node_id, info.id);
    }

    for node in &index.nodes {
        let Some(file) = &node.file else {
            continue;
        };
        let info = component_info_for_path(file);
        let component = components
            .entry(info.id.clone())
            .or_insert_with(|| ComponentAccumulator {
                id: info.id.clone(),
                name: info.name.clone(),
                kind: info.kind.clone(),
                ..ComponentAccumulator::default()
            });
        component.nodes.insert(node.id);
        component.evidence_paths.insert(file.clone());
        if let Some(language) = node
            .properties
            .get("language")
            .and_then(|value| value.as_str())
        {
            component.languages.insert(language.to_string());
        }
        node_component.insert(node.id, info.id);
    }

    for edge in &index.edges {
        if let Some(component_id) = node_component.get(&edge.source)
            && let Some(component) = components.get_mut(component_id)
        {
            component.edges.insert(edge.id);
        }
        if let Some(component_id) = node_component.get(&edge.target)
            && let Some(component) = components.get_mut(component_id)
        {
            component.edges.insert(edge.id);
        }
    }

    for hotspot in &index.hotspots {
        if let Some(file) = &hotspot.file {
            let info = component_info_for_path(file);
            if let Some(component) = components.get_mut(&info.id) {
                component.hotspots.push(hotspot.clone());
            }
        }
    }

    components
        .into_values()
        .map(|component| ArchitectureComponent {
            id: component.id,
            name: component.name,
            kind: component.kind,
            files: component.files.len(),
            nodes: component.nodes.len(),
            edges: component.edges.len(),
            languages: component.languages.into_iter().collect(),
            hotspots: component.hotspots.into_iter().take(10).collect(),
            evidence_paths: component.evidence_paths.into_iter().take(12).collect(),
        })
        .collect()
}

struct ComponentInfo {
    id: String,
    name: String,
    kind: String,
}

fn component_info_for_path(path: &str) -> ComponentInfo {
    let parts = path.split('/').collect::<Vec<_>>();
    match parts.as_slice() {
        ["crates", "external", name, ..] => ComponentInfo {
            id: format!("external:{name}"),
            name: (*name).to_string(),
            kind: "external_crate".to_string(),
        },
        ["crates", name, ..] => ComponentInfo {
            id: format!("crate:{name}"),
            name: (*name).to_string(),
            kind: "crate".to_string(),
        },
        ["openspec", ..] => ComponentInfo {
            id: "control:openspec".to_string(),
            name: "OpenSpec lifecycle".to_string(),
            kind: "control_plane".to_string(),
        },
        ["adr", ..] => ComponentInfo {
            id: "control:adr".to_string(),
            name: "Architecture decisions".to_string(),
            kind: "control_plane".to_string(),
        },
        ["AI_MERGE", ..] => ComponentInfo {
            id: "control:ai_merge".to_string(),
            name: "AI merge evidence".to_string(),
            kind: "control_plane".to_string(),
        },
        [".agents", ..] => ComponentInfo {
            id: "control:agents".to_string(),
            name: "Agent skills".to_string(),
            kind: "control_plane".to_string(),
        },
        ["docs", ..] => ComponentInfo {
            id: "control:docs".to_string(),
            name: "Documentation".to_string(),
            kind: "control_plane".to_string(),
        },
        [first, ..] => ComponentInfo {
            id: format!("repo:{first}"),
            name: (*first).to_string(),
            kind: "repo_surface".to_string(),
        },
        [] => ComponentInfo {
            id: "repo:root".to_string(),
            name: "Repository root".to_string(),
            kind: "repo_surface".to_string(),
        },
    }
}

fn architecture_component_edges(index: &KnowledgeIndex) -> Vec<ArchitectureEdge> {
    let node_components = index
        .nodes
        .iter()
        .filter_map(|node| {
            node.file
                .as_deref()
                .map(component_info_for_path)
                .map(|info| (node.id, info.id))
        })
        .collect::<BTreeMap<_, _>>();
    let mut edges = Vec::new();
    for edge in &index.edges {
        let Some(source) = node_components.get(&edge.source) else {
            continue;
        };
        let Some(target) = node_components.get(&edge.target) else {
            continue;
        };
        if source == target {
            continue;
        }
        edges.push(ArchitectureEdge {
            source: source.clone(),
            target: target.clone(),
            kind: format!("codegraph:{}", edge.kind),
            evidence: vec![format!("knowledge edge {}", edge.id)],
        });
    }
    edges
}

fn integration_surfaces(workspace: &Path) -> Vec<IntegrationSurface> {
    vec![
        IntegrationSurface {
            id: "surface:codegraph-rust".to_string(),
            name: "CodeGraph Rust".to_string(),
            kind: "architecture_graph".to_string(),
            provider: "codegraph-rust".to_string(),
            default_scope: "in-process knowledge indexing".to_string(),
            capabilities: vec![
                "multi-language tree-sitter registry".to_string(),
                "symbol/import/call/type graph extraction".to_string(),
                "impact and hotspot evidence".to_string(),
            ],
            evidence_paths: existing_paths(
                workspace,
                &[
                    "crates/external/codegraph-core",
                    "crates/external/codegraph-parser",
                    "third_party/upstream/codegraph-rust",
                    "docs/rusty-idd/merge-tools-package.md",
                    "AI_MERGE/16_upstream_knowledge_revisit.md",
                ],
            ),
        },
        IntegrationSurface {
            id: "surface:repomix-rs".to_string(),
            name: "repomix-rs".to_string(),
            kind: "context_package".to_string(),
            provider: "repomix-rs".to_string(),
            default_scope: "bounded context packing and token policy".to_string(),
            capabilities: vec![
                "compressed context packs".to_string(),
                "token accounting and top-file metrics".to_string(),
                "security and suspicious-file signals".to_string(),
                "git-aware context options".to_string(),
            ],
            evidence_paths: existing_paths(
                workspace,
                &[
                    "third_party/upstream/repomix-rs",
                    "Cargo.toml",
                    "docs/rusty-idd/merge-tools-package.md",
                    "AI_MERGE/16_upstream_knowledge_revisit.md",
                ],
            ),
        },
        IntegrationSurface {
            id: "surface:openspec".to_string(),
            name: "Rusty IDD OpenSpec lifecycle".to_string(),
            kind: "lifecycle_control_plane".to_string(),
            provider: "rusty-idd".to_string(),
            default_scope: "proposal, spec, design, ADR, task, validation, archive".to_string(),
            capabilities: vec![
                "goal intake".to_string(),
                "spec delta tracking".to_string(),
                "ordered implementation tasks".to_string(),
                "validation and merge evidence".to_string(),
            ],
            evidence_paths: existing_paths(
                workspace,
                &[
                    "openspec",
                    "crates/spec",
                    "crates/runner",
                    "docs/rusty-idd/proposal.md",
                ],
            ),
        },
        IntegrationSurface {
            id: "surface:audit-manifest".to_string(),
            name: "Audit and manifest evidence".to_string(),
            kind: "evidence_control_plane".to_string(),
            provider: "rusty-idd".to_string(),
            default_scope: "deterministic generated control-plane artifacts".to_string(),
            capabilities: vec![
                "AI_MERGE audit records".to_string(),
                "ADR traceability".to_string(),
                ".idd manifest baseline".to_string(),
                "knowledge artifact freshness".to_string(),
            ],
            evidence_paths: existing_paths(workspace, &["AI_MERGE", "adr", ".idd/MANIFEST.tsv"]),
        },
    ]
}

fn existing_paths(workspace: &Path, paths: &[&str]) -> Vec<String> {
    paths
        .iter()
        .filter(|path| workspace.join(path).exists())
        .map(|path| (*path).to_string())
        .collect()
}

fn automation_stages() -> Vec<AutomationStage> {
    vec![
        AutomationStage {
            id: "stage:intake".to_string(),
            name: "Goal intake and bounded context".to_string(),
            purpose: "turn repo state and user goal into bounded agent context".to_string(),
            surfaces: vec![
                "surface:repomix-rs".to_string(),
                "surface:openspec".to_string(),
            ],
            artifact_paths: vec!["openspec/changes/*/proposal.md".to_string()],
        },
        AutomationStage {
            id: "stage:architecture-map".to_string(),
            name: "Architecture mapping".to_string(),
            purpose: "map source structure, integrations, and impact before implementation"
                .to_string(),
            surfaces: vec![
                "surface:codegraph-rust".to_string(),
                "surface:repomix-rs".to_string(),
            ],
            artifact_paths: vec![
                ".idd/knowledge/index.json".to_string(),
                ".idd/knowledge/architecture.json".to_string(),
            ],
        },
        AutomationStage {
            id: "stage:specification".to_string(),
            name: "Specification and decisions".to_string(),
            purpose: "convert architecture map into spec deltas, design, ADRs, and tasks"
                .to_string(),
            surfaces: vec![
                "surface:openspec".to_string(),
                "surface:audit-manifest".to_string(),
            ],
            artifact_paths: vec![
                "openspec/changes/*/specs/**/*.md".to_string(),
                "adr/*.md".to_string(),
                "openspec/changes/*/tasks.md".to_string(),
            ],
        },
        AutomationStage {
            id: "stage:implementation".to_string(),
            name: "Implementation".to_string(),
            purpose: "apply graph-informed, spec-backed code changes".to_string(),
            surfaces: vec![
                "surface:codegraph-rust".to_string(),
                "surface:openspec".to_string(),
            ],
            artifact_paths: vec![
                "crates/**".to_string(),
                "openspec/changes/*/tasks.md".to_string(),
            ],
        },
        AutomationStage {
            id: "stage:validation".to_string(),
            name: "Validation and regeneration".to_string(),
            purpose: "run gates and refresh deterministic control-plane artifacts".to_string(),
            surfaces: vec![
                "surface:codegraph-rust".to_string(),
                "surface:repomix-rs".to_string(),
                "surface:audit-manifest".to_string(),
            ],
            artifact_paths: vec![
                ".idd/knowledge/report.md".to_string(),
                ".idd/MANIFEST.tsv".to_string(),
                "AI_MERGE/*.md".to_string(),
            ],
        },
        AutomationStage {
            id: "stage:handoff".to_string(),
            name: "Handoff and merge evidence".to_string(),
            purpose: "record evidence, rollback, and merge-ready traceability".to_string(),
            surfaces: vec![
                "surface:audit-manifest".to_string(),
                "surface:openspec".to_string(),
            ],
            artifact_paths: vec![
                "AI_MERGE/*.md".to_string(),
                "openspec/changes/archive/**".to_string(),
            ],
        },
    ]
}

fn automation_stage_edges(stages: &[AutomationStage]) -> Vec<ArchitectureEdge> {
    let mut edges = Vec::new();
    for stage in stages {
        for surface in &stage.surfaces {
            edges.push(ArchitectureEdge {
                source: stage.id.clone(),
                target: surface.clone(),
                kind: "uses".to_string(),
                evidence: stage.artifact_paths.clone(),
            });
        }
    }
    for window in stages.windows(2) {
        edges.push(ArchitectureEdge {
            source: window[0].id.clone(),
            target: window[1].id.clone(),
            kind: "precedes".to_string(),
            evidence: Vec::new(),
        });
    }
    edges
}

fn integration_component_edges(
    surfaces: &[IntegrationSurface],
    components: &[ArchitectureComponent],
) -> Vec<ArchitectureEdge> {
    let component_ids = components
        .iter()
        .map(|component| component.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut edges = Vec::new();
    for surface in surfaces {
        let target = match surface.id.as_str() {
            "surface:codegraph-rust" => "external:codegraph-parser",
            "surface:openspec" => "crate:spec",
            "surface:audit-manifest" => "control:ai_merge",
            _ => "",
        };
        if !target.is_empty() && component_ids.contains(target) {
            edges.push(ArchitectureEdge {
                source: surface.id.clone(),
                target: target.to_string(),
                kind: "implemented_by".to_string(),
                evidence: surface.evidence_paths.clone(),
            });
        }
    }
    edges
}

fn render_architecture_markdown(graph: &ArchitectureGraph) -> String {
    let mut out = String::new();
    out.push_str("# Architecture Graph\n\n");
    out.push_str(&format!(
        "- Workspace fingerprint: `{}`\n",
        graph.workspace_fingerprint
    ));
    out.push_str(&format!(
        "- Source graph provider: `{}`\n",
        graph.source_graph.provider
    ));
    out.push_str(&format!(
        "- Source graph: {} files, {} nodes, {} edges\n",
        graph.source_graph.files, graph.source_graph.nodes, graph.source_graph.edges
    ));
    out.push_str(&format!(
        "- Source languages: {}\n",
        graph.source_graph.languages.join(", ")
    ));
    out.push_str(&format!(
        "- Context provider: `{}`\n",
        graph.context_package.provider
    ));
    out.push_str(&format!(
        "- Context package: {} files, {} tokens\n\n",
        graph.context_package.files, graph.context_package.tokens
    ));

    out.push_str("## Automation Stages\n\n");
    out.push_str("| Stage | Purpose | Surfaces |\n|---|---|---|\n");
    for stage in &graph.automation_stages {
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            stage.name,
            stage.purpose,
            stage.surfaces.join(", ")
        ));
    }
    out.push('\n');

    out.push_str("## Integration Surfaces\n\n");
    out.push_str("| Surface | Kind | Scope | Capabilities |\n|---|---|---|---|\n");
    for surface in &graph.integration_surfaces {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            surface.name,
            surface.kind,
            surface.default_scope,
            surface.capabilities.join(", ")
        ));
    }
    out.push('\n');

    out.push_str("## Components\n\n");
    out.push_str(
        "| Component | Kind | Files | Nodes | Edges | Languages |\n|---|---|---:|---:|---:|---|\n",
    );
    for component in &graph.components {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} |\n",
            component.name,
            component.kind,
            component.files,
            component.nodes,
            component.edges,
            component.languages.join(", ")
        ));
    }
    out.push('\n');

    out.push_str("## Edges\n\n");
    out.push_str("| Source | Kind | Target |\n|---|---|---|\n");
    for edge in &graph.edges {
        out.push_str(&format!(
            "| `{}` | {} | `{}` |\n",
            edge.source, edge.kind, edge.target
        ));
    }
    out.push('\n');

    out.push_str("## Findings\n\n");
    for finding in &graph.findings {
        out.push_str(&format!("- {finding}\n"));
    }
    out
}

fn render_architecture_diagrams(graph: &ArchitectureGraph) -> String {
    let mut out = String::new();
    out.push_str("# rusty-idd Architecture Diagrams\n\n");
    out.push_str("> Generated by `rusty-idd knowledge diagrams`. Do not hand-edit; regenerate with `just diagrams`.\n\n");
    out.push_str(&format!(
        "- Source graph: {} files, {} nodes, {} edges via `{}`\n",
        graph.source_graph.files,
        graph.source_graph.nodes,
        graph.source_graph.edges,
        graph.source_graph.provider
    ));
    out.push_str(&format!(
        "- Context package: {} files, {} tokens via `{}`\n",
        graph.context_package.files, graph.context_package.tokens, graph.context_package.provider
    ));
    out.push_str(&format!(
        "- Components: {}; integration surfaces: {}; automation stages: {}\n\n",
        graph.components.len(),
        graph.integration_surfaces.len(),
        graph.automation_stages.len()
    ));

    out.push_str("## Autonomous Workflow\n\n");
    out.push_str("```mermaid\nflowchart LR\n");
    for stage in &graph.automation_stages {
        out.push_str(&format!(
            "    {}[\"{}\"]\n",
            mermaid_id(&stage.id),
            mermaid_label(&stage.name)
        ));
    }
    for surface in &graph.integration_surfaces {
        out.push_str(&format!(
            "    {}((\"{}\"))\n",
            mermaid_id(&surface.id),
            mermaid_label(&surface.name)
        ));
    }
    let mut workflow_edges = BTreeSet::new();
    for edge in &graph.edges {
        if edge.kind == "precedes" && edge.source.starts_with("stage:") {
            workflow_edges.insert((edge.source.clone(), edge.target.clone(), String::new()));
        }
        if edge.kind == "uses" && edge.source.starts_with("stage:") {
            workflow_edges.insert((edge.source.clone(), edge.target.clone(), "uses".to_string()));
        }
    }
    for (source, target, label) in workflow_edges {
        if label.is_empty() {
            out.push_str(&format!(
                "    {} --> {}\n",
                mermaid_id(&source),
                mermaid_id(&target)
            ));
        } else {
            out.push_str(&format!(
                "    {} -- {} --> {}\n",
                mermaid_id(&source),
                mermaid_label(&label),
                mermaid_id(&target)
            ));
        }
    }
    out.push_str("```\n\n");

    out.push_str("## Crate And Integration Boundaries\n\n");
    out.push_str("```mermaid\nflowchart LR\n");
    let component_ids: BTreeSet<&str> = graph
        .components
        .iter()
        .map(|component| component.id.as_str())
        .collect();
    for component in &graph.components {
        let id = mermaid_id(&component.id);
        let label = mermaid_label(&component.name);
        if component.kind == "external_crate" {
            out.push_str(&format!("    {id}[[\"{label}\"]]\n"));
        } else {
            out.push_str(&format!("    {id}[\"{label}\"]\n"));
        }
    }
    let mut component_edges = BTreeSet::new();
    for edge in &graph.edges {
        if component_ids.contains(edge.source.as_str())
            && component_ids.contains(edge.target.as_str())
        {
            component_edges.insert((edge.source.clone(), edge.target.clone(), edge.kind.clone()));
        }
    }
    for (source, target, kind) in component_edges {
        out.push_str(&format!(
            "    {} -- {} --> {}\n",
            mermaid_id(&source),
            mermaid_label(&kind),
            mermaid_id(&target)
        ));
    }
    out.push_str("```\n\n");

    out.push_str("## Generated Artifact Flow\n\n");
    out.push_str("```mermaid\nflowchart TD\n");
    out.push_str("    source[\"Source files\"]\n");
    out.push_str("    codegraph[\"CodeGraph index\"]\n");
    out.push_str("    repomix[\"repomix context package\"]\n");
    out.push_str("    architecture[\".idd/knowledge/architecture.{json,md}\"]\n");
    out.push_str("    diagrams[\"docs/rusty-idd/architecture-diagrams.md\"]\n");
    out.push_str("    system[\".idd/knowledge/system-architecture.{json,md}\"]\n");
    out.push_str("    operating[\".idd/knowledge/operating-model.{json,md}\"]\n");
    out.push_str("    integration[\"integration plan/status/owners/readiness\"]\n");
    out.push_str("    plan[\".idd/knowledge/plan-context.{json,md}\"]\n");
    out.push_str("    manifest[\".idd/MANIFEST.tsv\"]\n");
    out.push_str("    validation[\"AI_MERGE/validation_report.md\"]\n");
    out.push_str("    source --> codegraph\n");
    out.push_str("    source --> repomix\n");
    out.push_str("    codegraph --> architecture\n");
    out.push_str("    repomix --> architecture\n");
    out.push_str("    architecture --> diagrams\n");
    out.push_str("    architecture --> system\n");
    out.push_str("    system --> operating\n");
    out.push_str("    operating --> integration\n");
    out.push_str("    architecture --> plan\n");
    out.push_str("    system --> plan\n");
    out.push_str("    integration --> plan\n");
    out.push_str("    diagrams --> manifest\n");
    out.push_str("    plan --> manifest\n");
    out.push_str("    manifest --> validation\n");
    out.push_str("```\n\n");

    out.push_str("## Diagram Inputs\n\n");
    out.push_str("| Input | Provider | Count |\n|---|---|---:|\n");
    out.push_str(&format!(
        "| Source files | `{}` | {} |\n",
        graph.source_graph.provider, graph.source_graph.files
    ));
    out.push_str(&format!(
        "| Graph nodes | `{}` | {} |\n",
        graph.source_graph.provider, graph.source_graph.nodes
    ));
    out.push_str(&format!(
        "| Graph edges | `{}` | {} |\n",
        graph.source_graph.provider, graph.source_graph.edges
    ));
    out.push_str(&format!(
        "| Packed files | `{}` | {} |\n",
        graph.context_package.provider, graph.context_package.files
    ));
    out.push_str(&format!(
        "| Packed tokens | `{}` | {} |\n",
        graph.context_package.provider, graph.context_package.tokens
    ));
    out.push('\n');

    out.push_str("## Findings\n\n");
    if graph.findings.is_empty() {
        out.push_str("- No architecture graph findings.\n");
    } else {
        for finding in &graph.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }

    out
}

fn mermaid_id(value: &str) -> String {
    let mut out = String::from("n_");
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    out
}

fn mermaid_label(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('[', "(")
        .replace(']', ")")
}

#[derive(Debug, Deserialize)]
struct MetaProjectList {
    projects: Vec<MetaProject>,
}

#[derive(Debug, Deserialize)]
struct MetaProject {
    name: String,
    path: String,
    repo: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    is_meta: bool,
}

fn system_architecture_graph(
    workspace: &Path,
    system_root: &Path,
) -> Result<SystemArchitectureGraph> {
    let (mut repos, discovery_source) = discover_system_repos(system_root)?;
    let mut enrichment_findings = Vec::new();
    for repo in &mut repos {
        enrichment_findings.extend(enrich_system_repo(workspace, system_root, repo));
    }
    repos.sort_by(|a, b| a.name.cmp(&b.name).then(a.path.cmp(&b.path)));

    let roles = system_roles(&repos);
    let mut edges = system_edges(workspace, system_root, &repos, &roles);
    edges.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then(a.target.cmp(&b.target))
            .then(a.kind.cmp(&b.kind))
    });
    edges.dedup_by(|a, b| a.source == b.source && a.target == b.target && a.kind == b.kind);

    let dirty_count = repos.iter().filter(|repo| repo.dirty).count();
    let local_graph_count = repos
        .iter()
        .filter(|repo| repo.has_local_architecture_graph)
        .count();
    let parsed_local_graph_count = repos
        .iter()
        .filter(|repo| repo.local_architecture.is_some())
        .count();
    let mut findings = vec![
        format!(
            "discovered {} peer repos from {discovery_source}",
            repos.len()
        ),
        format!("{dirty_count} repos have local dirty state recorded as evidence"),
        format!("{local_graph_count} repos expose .idd/knowledge/architecture.json"),
        format!("{parsed_local_graph_count} repos expose parsed architecture summaries"),
    ];
    findings.extend(enrichment_findings);

    Ok(SystemArchitectureGraph {
        schema_version: 1,
        workspace_root: workspace.display().to_string(),
        system_root: system_root.display().to_string(),
        discovery_source,
        repos,
        roles,
        edges,
        findings,
    })
}

fn discover_system_repos(system_root: &Path) -> Result<(Vec<SystemRepo>, String)> {
    if let Ok(projects) = discover_meta_projects(system_root)
        && !projects.is_empty()
    {
        return Ok((projects, "meta project list --json".to_string()));
    }
    discover_git_child_repos(system_root)
        .map(|repos| (repos, "filesystem git discovery".to_string()))
}

fn discover_meta_projects(system_root: &Path) -> Result<Vec<SystemRepo>> {
    let output = Command::new("meta")
        .args(["project", "list", "--json"])
        .current_dir(system_root)
        .output()
        .context("run meta project list --json")?;
    if !output.status.success() {
        bail!("meta project list --json failed");
    }
    let list: MetaProjectList =
        serde_json::from_slice(&output.stdout).context("parse meta project list JSON")?;
    let repos = list
        .projects
        .into_iter()
        .map(|project| {
            let mut tags = project.tags;
            if project.is_meta && !tags.iter().any(|tag| tag == "meta") {
                tags.push("meta".to_string());
            }
            tags.sort();
            tags.dedup();
            SystemRepo {
                id: format!("repo:{}", slug(&project.name)),
                name: project.name,
                path: normalize_relative_path(&project.path),
                repo: project.repo,
                branch: None,
                head: None,
                dirty: false,
                tags,
                markers: Vec::new(),
                roles: Vec::new(),
                has_local_architecture_graph: false,
                local_architecture: None,
            }
        })
        .collect();
    Ok(repos)
}

fn discover_git_child_repos(system_root: &Path) -> Result<Vec<SystemRepo>> {
    let mut repos = Vec::new();
    for entry in fs::read_dir(system_root)
        .with_context(|| format!("read system root {}", system_root.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_dir() || !path.join(".git").exists() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if matches!(
            name,
            ".git" | ".worktrees" | "_workspace" | "_workspace_prev"
        ) {
            continue;
        }
        repos.push(SystemRepo {
            id: format!("repo:{}", slug(name)),
            name: name.to_string(),
            path: normalize_relative_path(name),
            repo: git_output(&path, &["remote", "get-url", "origin"]),
            branch: None,
            head: None,
            dirty: false,
            tags: Vec::new(),
            markers: Vec::new(),
            roles: Vec::new(),
            has_local_architecture_graph: false,
            local_architecture: None,
        });
    }
    Ok(repos)
}

fn enrich_system_repo(workspace: &Path, system_root: &Path, repo: &mut SystemRepo) -> Vec<String> {
    let mut findings = Vec::new();
    let repo_root = system_root.join(&repo.path);
    let is_current_workspace = repo_root == workspace;
    if is_current_workspace {
        repo.branch = None;
        repo.head = None;
        repo.dirty = false;
    } else {
        repo.branch = git_output(&repo_root, &["rev-parse", "--abbrev-ref", "HEAD"]);
        repo.head = git_output(&repo_root, &["rev-parse", "HEAD"]);
        repo.dirty = git_dirty(&repo_root);
    }
    repo.markers = repo_markers(&repo_root);
    let architecture_path = repo_root.join(".idd/knowledge/architecture.json");
    repo.has_local_architecture_graph = architecture_path.exists();
    repo.local_architecture = if repo.has_local_architecture_graph {
        match read_json_file::<ArchitectureGraph>(&architecture_path) {
            Ok(architecture) => Some(peer_architecture_summary(&architecture)),
            Err(error) => {
                findings.push(format!(
                    "repo {} exposes unreadable architecture graph at {}: {error:#}",
                    repo.name,
                    display_path(system_root, &architecture_path)
                ));
                None
            }
        }
    } else {
        None
    };
    repo.roles = classify_repo_roles(repo);
    findings
}

fn peer_architecture_summary(architecture: &ArchitectureGraph) -> PeerArchitectureSummary {
    let mut components = architecture.components.clone();
    components.sort_by(|a, b| {
        let a_score = a.nodes + a.edges + a.files * 10;
        let b_score = b.nodes + b.edges + b.files * 10;
        b_score.cmp(&a_score).then(a.id.cmp(&b.id))
    });
    let top_components = components
        .into_iter()
        .take(8)
        .map(|component| PeerArchitectureComponentSummary {
            id: component.id,
            name: component.name,
            kind: component.kind,
            files: component.files,
            nodes: component.nodes,
            edges: component.edges,
            languages: component.languages,
        })
        .collect();

    let integration_surfaces = architecture
        .integration_surfaces
        .iter()
        .map(|surface| PeerIntegrationSurfaceSummary {
            id: surface.id.clone(),
            name: surface.name.clone(),
            kind: surface.kind.clone(),
            provider: surface.provider.clone(),
            capabilities: surface.capabilities.clone(),
        })
        .collect();

    PeerArchitectureSummary {
        schema_version: architecture.schema_version,
        source_graph: architecture.source_graph.clone(),
        context_package: architecture.context_package.clone(),
        component_count: architecture.components.len(),
        integration_surface_count: architecture.integration_surfaces.len(),
        top_components,
        integration_surfaces,
    }
}

fn git_output(repo_root: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|value| !value.is_empty())
}

fn git_dirty(repo_root: &Path) -> bool {
    let Some(status) = git_output(repo_root, &["status", "--porcelain"]) else {
        return false;
    };
    !status.trim().is_empty()
}

fn repo_markers(repo_root: &Path) -> Vec<String> {
    let checks = [
        ("rust", "Cargo.toml"),
        ("node", "package.json"),
        ("openspec", "openspec"),
        ("idd-knowledge", ".idd/knowledge"),
        ("handoff", ".handoff"),
        ("agents", ".agents"),
        ("claude", ".claude"),
        ("github-actions", ".github/workflows"),
        ("make", "Makefile"),
        ("just", "Justfile"),
    ];
    checks
        .into_iter()
        .filter(|(_, path)| repo_root.join(path).exists())
        .map(|(marker, _)| marker.to_string())
        .collect()
}

fn classify_repo_roles(repo: &SystemRepo) -> Vec<String> {
    let name = repo.name.to_ascii_lowercase();
    let tags = repo
        .tags
        .iter()
        .map(|tag| tag.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let markers = repo
        .markers
        .iter()
        .map(|marker| marker.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let mut roles = BTreeSet::new();

    if name == "rusty-idd" {
        roles.insert("role:idd-control-plane".to_string());
    }
    if name == "handoff" || tags.contains("handoff") || markers.contains("handoff") {
        roles.insert("role:fleet-handoff".to_string());
    }
    if name == "weave" || tags.contains("mcp") || tags.contains("orchestration") {
        roles.insert("role:coordination-domain-surface".to_string());
    }
    if name == "obscura" {
        roles.insert("role:domain-upgrade-surface".to_string());
    }
    if name == "yazelix" {
        roles.insert("role:parser-runtime-surface".to_string());
    }
    if name == "envctl" || tags.contains("env") {
        roles.insert("role:toolchain-provider".to_string());
    }
    if name.contains("prompt") || tags.contains("prompts") {
        roles.insert("role:spec-producer".to_string());
    }
    if name.starts_with("meta_") || name == "meta_cli" || tags.contains("canon") {
        roles.insert("role:meta-control-plane".to_string());
    }
    if name.ends_with("_hub") || tags.contains("hub") {
        roles.insert("role:capability-hub".to_string());
    }
    if tags.contains("ai") || tags.contains("agent-env") || markers.contains("agents") {
        roles.insert("role:agent-environment".to_string());
    }
    if tags.contains("memory") || tags.contains("knowledge") {
        roles.insert("role:knowledge-memory".to_string());
    }
    if tags.contains("docs") || tags.contains("wiki") {
        roles.insert("role:documentation-knowledge".to_string());
    }
    if markers.contains("rust") {
        roles.insert("role:rust-code-surface".to_string());
    }

    roles.into_iter().collect()
}

fn system_roles(repos: &[SystemRepo]) -> Vec<SystemRole> {
    let mut role_repos = BTreeMap::<String, Vec<String>>::new();
    for repo in repos {
        for role in &repo.roles {
            role_repos
                .entry(role.clone())
                .or_default()
                .push(repo.id.clone());
        }
    }
    role_repos
        .into_iter()
        .map(|(id, mut repos)| {
            repos.sort();
            SystemRole {
                name: system_role_name(&id).to_string(),
                purpose: system_role_purpose(&id).to_string(),
                id,
                repos,
            }
        })
        .collect()
}

fn system_role_name(id: &str) -> &str {
    match id {
        "role:idd-control-plane" => "Rusty IDD control plane",
        "role:fleet-handoff" => "Fleet handoff",
        "role:coordination-domain-surface" => "Coordination and domain surface",
        "role:domain-upgrade-surface" => "Domain upgrade surface",
        "role:parser-runtime-surface" => "Parser/runtime surface",
        "role:toolchain-provider" => "Toolchain provider",
        "role:spec-producer" => "Spec producer",
        "role:meta-control-plane" => "Meta control plane",
        "role:capability-hub" => "Capability hub",
        "role:agent-environment" => "Agent environment",
        "role:knowledge-memory" => "Knowledge and memory",
        "role:documentation-knowledge" => "Documentation and knowledge",
        "role:rust-code-surface" => "Rust code surface",
        _ => "System role",
    }
}

fn system_role_purpose(id: &str) -> &str {
    match id {
        "role:idd-control-plane" => {
            "Owns OpenSpec, ADR, task, validation, manifest, and graph-driven implementation workflow"
        }
        "role:fleet-handoff" => {
            "Carries central and fleet handoff state for cross-repo agent continuity"
        }
        "role:coordination-domain-surface" => {
            "Provides orchestration, MCP, and domain-adjacent system coordination surfaces"
        }
        "role:domain-upgrade-surface" => {
            "Contributes domain behavior through weave plus Obscura upgrade paths"
        }
        "role:parser-runtime-surface" => {
            "Carries parser/runtime support such as tree-sitter through Yazelix"
        }
        "role:toolchain-provider" => {
            "Provides parent-managed tools instead of user-global installs"
        }
        "role:spec-producer" => {
            "Produces intent or prompt artifacts that Rusty IDD can turn into OpenSpec"
        }
        "role:meta-control-plane" => {
            "Provides parent meta workspace inventory and execution surfaces"
        }
        "role:capability-hub" => "Groups domain capability repos used by the wider system",
        "role:agent-environment" => {
            "Supports agent runtime, skills, prompts, or execution environment"
        }
        "role:knowledge-memory" => "Stores memory or knowledge surfaces used by agents",
        "role:documentation-knowledge" => "Stores documentation and wiki surfaces",
        "role:rust-code-surface" => {
            "Contains Rust source that can be indexed by CodeGraph-backed Rusty IDD knowledge"
        }
        _ => "System role discovered from repo metadata",
    }
}

fn system_edges(
    workspace: &Path,
    system_root: &Path,
    repos: &[SystemRepo],
    roles: &[SystemRole],
) -> Vec<SystemArchitectureEdge> {
    let current_repo = repos
        .iter()
        .find(|repo| system_root.join(&repo.path) == workspace)
        .map(|repo| repo.id.clone())
        .unwrap_or_else(|| "repo:rusty-idd".to_string());
    let mut edges = Vec::new();

    for repo in repos {
        edges.push(SystemArchitectureEdge {
            source: "system:meta-workspace".to_string(),
            target: repo.id.clone(),
            kind: "contains".to_string(),
            evidence: vec![repo.path.clone()],
        });
        for role in &repo.roles {
            edges.push(SystemArchitectureEdge {
                source: repo.id.clone(),
                target: role.clone(),
                kind: "provides".to_string(),
                evidence: repo.tags.clone(),
            });
        }
        if repo.has_local_architecture_graph {
            edges.push(SystemArchitectureEdge {
                source: repo.id.clone(),
                target: "artifact:.idd/knowledge/architecture.json".to_string(),
                kind: "publishes".to_string(),
                evidence: vec![format!("{}/.idd/knowledge/architecture.json", repo.path)],
            });
        }
    }

    for role in roles {
        if role.id != "role:idd-control-plane" {
            edges.push(SystemArchitectureEdge {
                source: current_repo.clone(),
                target: role.id.clone(),
                kind: "maps_for_automation".to_string(),
                evidence: vec![
                    ".idd/knowledge/system-architecture.json".to_string(),
                    "openspec/changes/add-system-architecture-peer-graph".to_string(),
                ],
            });
        }
    }

    for (target, kind) in [
        ("role:fleet-handoff", "uses_for_continuity"),
        (
            "role:coordination-domain-surface",
            "scopes_as_feature_gated_surface",
        ),
        (
            "role:domain-upgrade-surface",
            "scopes_as_feature_gated_surface",
        ),
        (
            "role:parser-runtime-surface",
            "uses_as_parser_runtime_evidence",
        ),
        ("role:toolchain-provider", "uses_for_parent_managed_tools"),
        ("role:spec-producer", "consumes_spec_intent_from"),
        ("role:meta-control-plane", "uses_for_workspace_inventory"),
    ] {
        if roles.iter().any(|role| role.id == target) {
            edges.push(SystemArchitectureEdge {
                source: current_repo.clone(),
                target: target.to_string(),
                kind: kind.to_string(),
                evidence: vec!["AI_MERGE/17_architecture_graph_workflow.md".to_string()],
            });
        }
    }

    edges
}

fn render_system_architecture_markdown(graph: &SystemArchitectureGraph) -> String {
    let mut out = String::new();
    out.push_str("# System Architecture Graph\n\n");
    out.push_str(&format!("- System root: `{}`\n", graph.system_root));
    out.push_str(&format!("- Workspace root: `{}`\n", graph.workspace_root));
    out.push_str(&format!(
        "- Discovery source: `{}`\n",
        graph.discovery_source
    ));
    out.push_str(&format!("- Repos: {}\n", graph.repos.len()));
    out.push_str(&format!("- Roles: {}\n", graph.roles.len()));
    out.push_str(&format!("- Edges: {}\n\n", graph.edges.len()));

    out.push_str("## Roles\n\n");
    out.push_str("| Role | Purpose | Repos |\n|---|---|---|\n");
    for role in &graph.roles {
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            role.name,
            role.purpose,
            role.repos.join(", ")
        ));
    }
    out.push('\n');

    out.push_str("## Repos\n\n");
    out.push_str("| Repo | Branch | Dirty | Tags | Roles | Markers |\n|---|---|---|---|---|---|\n");
    for repo in &graph.repos {
        out.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | {} |\n",
            repo.name,
            repo.branch.as_deref().unwrap_or(""),
            repo.dirty,
            repo.tags.join(", "),
            repo.roles.join(", "),
            repo.markers.join(", ")
        ));
    }
    out.push('\n');

    out.push_str("## Peer Architecture Summaries\n\n");
    let architecture_repos = graph
        .repos
        .iter()
        .filter(|repo| repo.local_architecture.is_some())
        .collect::<Vec<_>>();
    if architecture_repos.is_empty() {
        out.push_str("No parsed peer architecture summaries.\n\n");
    } else {
        out.push_str("| Repo | Source Graph | Context Package | Surfaces | Top Components |\n|---|---|---|---:|---|\n");
        for repo in architecture_repos {
            let architecture = repo.local_architecture.as_ref().expect("filtered summary");
            let top_components = architecture
                .top_components
                .iter()
                .take(5)
                .map(|component| component.name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "| `{}` | {} files, {} nodes, {} edges via `{}` | {} files, {} tokens via `{}` | {} | {} |\n",
                repo.name,
                architecture.source_graph.files,
                architecture.source_graph.nodes,
                architecture.source_graph.edges,
                architecture.source_graph.provider,
                architecture.context_package.files,
                architecture.context_package.tokens,
                architecture.context_package.provider,
                architecture.integration_surface_count,
                top_components
            ));
        }
        out.push('\n');
    }

    out.push_str("## Edges\n\n");
    out.push_str("| Source | Kind | Target |\n|---|---|---|\n");
    for edge in &graph.edges {
        out.push_str(&format!(
            "| `{}` | {} | `{}` |\n",
            edge.source, edge.kind, edge.target
        ));
    }
    out.push('\n');

    out.push_str("## Findings\n\n");
    for finding in &graph.findings {
        out.push_str(&format!("- {finding}\n"));
    }
    out
}

struct OperatingLayerDefinition {
    id: &'static str,
    name: &'static str,
    purpose: &'static str,
}

struct OperatingCapabilityDefinition {
    id: &'static str,
    name: &'static str,
    layer: &'static str,
    purpose: &'static str,
    repo_names: &'static [&'static str],
    role_ids: &'static [&'static str],
    anchors: &'static [&'static str],
}

fn system_operating_model(
    workspace: &Path,
    system: &SystemArchitectureGraph,
    source_path: &Path,
) -> SystemOperatingModel {
    let layer_definitions = operating_layer_definitions();
    let capability_definitions = operating_capability_definitions();
    let mut findings = vec![format!(
        "operating model derived from {} repos and {} roles in {}",
        system.repos.len(),
        system.roles.len(),
        display_path(workspace, source_path)
    )];
    let mut capabilities = Vec::new();
    let mut edges = Vec::new();

    edges.push(OperatingModelEdge {
        source: "system:agentic-company".to_string(),
        target: "repo:rusty-idd".to_string(),
        kind: "planned_by".to_string(),
        evidence: vec!["Rusty IDD graph/spec workflow".to_string()],
    });

    for definition in &capability_definitions {
        let repos = matching_operating_repos(system, definition);
        let status = if repos.is_empty() && definition.anchors.is_empty() {
            "missing"
        } else if repos.is_empty() {
            "external"
        } else if definition.anchors.is_empty() {
            "mapped"
        } else {
            "partial"
        }
        .to_string();
        if repos.is_empty() {
            findings.push(format!(
                "{} has no discovered repo owner in the system graph",
                definition.name
            ));
        }
        for anchor in definition.anchors {
            findings.push(format!(
                "{} records external or future anchor: {}",
                definition.name, anchor
            ));
        }

        edges.push(OperatingModelEdge {
            source: definition.layer.to_string(),
            target: definition.id.to_string(),
            kind: "contains_capability".to_string(),
            evidence: vec![definition.purpose.to_string()],
        });
        for repo in &repos {
            edges.push(OperatingModelEdge {
                source: definition.id.to_string(),
                target: repo.clone(),
                kind: "mapped_to_repo".to_string(),
                evidence: vec![definition.name.to_string()],
            });
        }
        for anchor in definition.anchors {
            edges.push(OperatingModelEdge {
                source: definition.id.to_string(),
                target: format!("anchor:{}", slug(anchor)),
                kind: "records_anchor".to_string(),
                evidence: vec![anchor.to_string()],
            });
        }

        capabilities.push(OperatingCapability {
            id: definition.id.to_string(),
            name: definition.name.to_string(),
            layer: definition.layer.to_string(),
            purpose: definition.purpose.to_string(),
            status,
            repos,
            anchors: definition
                .anchors
                .iter()
                .map(|anchor| anchor.to_string())
                .collect(),
            evidence: operating_capability_evidence(definition),
        });
    }

    let mut layers = layer_definitions
        .iter()
        .map(|definition| {
            let capability_ids = capabilities
                .iter()
                .filter(|capability| capability.layer == definition.id)
                .map(|capability| capability.id.clone())
                .collect::<Vec<_>>();
            let mut repos = capabilities
                .iter()
                .filter(|capability| capability.layer == definition.id)
                .flat_map(|capability| capability.repos.clone())
                .collect::<Vec<_>>();
            repos.sort();
            repos.dedup();
            edges.push(OperatingModelEdge {
                source: "system:agentic-company".to_string(),
                target: definition.id.to_string(),
                kind: "contains_layer".to_string(),
                evidence: vec![definition.purpose.to_string()],
            });
            OperatingModelLayer {
                id: definition.id.to_string(),
                name: definition.name.to_string(),
                purpose: definition.purpose.to_string(),
                capabilities: capability_ids,
                repos,
            }
        })
        .collect::<Vec<_>>();

    capabilities.sort_by(|a, b| a.id.cmp(&b.id));
    layers.sort_by(|a, b| a.id.cmp(&b.id));
    edges.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then(a.target.cmp(&b.target))
            .then(a.kind.cmp(&b.kind))
    });
    findings.sort();
    findings.dedup();

    SystemOperatingModel {
        schema_version: 1,
        workspace_root: workspace.display().to_string(),
        system_root: system.system_root.clone(),
        source_graph: display_path(workspace, source_path),
        layers,
        capabilities,
        edges,
        findings,
    }
}

fn operating_layer_definitions() -> Vec<OperatingLayerDefinition> {
    vec![
        OperatingLayerDefinition {
            id: "layer:governance-reasoning",
            name: "Governance and reasoning",
            purpose: "Board-style reasoning, strategy, and policy without direct execution",
        },
        OperatingLayerDefinition {
            id: "layer:executive-control-plane",
            name: "Executive control plane",
            purpose: "Company-level command, OpenSpec, handoff, and repo governance",
        },
        OperatingLayerDefinition {
            id: "layer:coordination-communication",
            name: "Coordination and communication",
            purpose: "Agent communication, orchestration, and cross-agent continuity",
        },
        OperatingLayerDefinition {
            id: "layer:environment-security",
            name: "Environment and security",
            purpose: "Vault, key relay, certificates, and parent-managed toolchains",
        },
        OperatingLayerDefinition {
            id: "layer:knowledge-runtime",
            name: "Knowledge and runtime",
            purpose: "Memory, vector/progress databases, inference, training, and runtime state",
        },
        OperatingLayerDefinition {
            id: "layer:front-door-experience",
            name: "Front door experience",
            purpose: "Prompt, chat, LifeOS, and operator-facing user experience surfaces",
        },
        OperatingLayerDefinition {
            id: "layer:agent-runtime",
            name: "Agent runtime",
            purpose: "Agent harnesses, execution workers, and automation runtimes",
        },
        OperatingLayerDefinition {
            id: "layer:simulation-validation",
            name: "Simulation and validation",
            purpose: "Digital twin simulation and high-fidelity failure space for agents",
        },
        OperatingLayerDefinition {
            id: "layer:infrastructure-device-fabric",
            name: "Infrastructure and device fabric",
            purpose: "Network control plus distributed device compute, storage, inference, and memory",
        },
        OperatingLayerDefinition {
            id: "layer:toolchain-parser-runtime",
            name: "Toolchain and parser runtime",
            purpose: "Tree-sitter, Lua, terminal/runtime, parser, and toolchain surfaces",
        },
        OperatingLayerDefinition {
            id: "layer:interface-automation",
            name: "Interface automation",
            purpose: "AR-glasses workflow, local automation, media, and home interfaces",
        },
    ]
}

fn operating_capability_definitions() -> Vec<OperatingCapabilityDefinition> {
    vec![
        OperatingCapabilityDefinition {
            id: "capability:board-reasoning",
            name: "Board reasoning layer",
            layer: "layer:governance-reasoning",
            purpose: "Non-executing strategic reasoning layer for company direction",
            repo_names: &[
                "flexnetos_brain",
                "flexnetos_wiki",
                "my_wiki",
                "obsidian_mind",
            ],
            role_ids: &["role:documentation-knowledge", "role:knowledge-memory"],
            anchors: &["company hierarchy board layer"],
        },
        OperatingCapabilityDefinition {
            id: "capability:idd-spec-engine",
            name: "IDD and spec engine",
            layer: "layer:executive-control-plane",
            purpose: "Turns goals into OpenSpec, ADR, tasks, implementation, validation, and merge evidence",
            repo_names: &["rusty_idd", "handoff"],
            role_ids: &["role:idd-control-plane"],
            anchors: &["Rusty IDD built into handoff"],
        },
        OperatingCapabilityDefinition {
            id: "capability:meta-peer-control",
            name: "Meta peer repo control",
            layer: "layer:executive-control-plane",
            purpose: "Controls the peer-repo environment and hosts full-system execution context",
            repo_names: &[
                "meta_cli",
                "meta_core",
                "meta_git_cli",
                "meta_git_lib",
                "meta_project_cli",
                "meta_rust_cli",
            ],
            role_ids: &["role:meta-control-plane"],
            anchors: &["meta peer repo system"],
        },
        OperatingCapabilityDefinition {
            id: "capability:fleet-handoff",
            name: "Central and fleet handoff",
            layer: "layer:coordination-communication",
            purpose: "Maintains central and fleet handoff state for cross-repo agents",
            repo_names: &["handoff", "rusty_idd", "weave"],
            role_ids: &["role:fleet-handoff"],
            anchors: &["handoff central and fleet design"],
        },
        OperatingCapabilityDefinition {
            id: "capability:agent-communication",
            name: "Agent communication layer",
            layer: "layer:coordination-communication",
            purpose: "Provides agent communication and orchestration paths",
            repo_names: &["weave", "atc", "mcp_hub"],
            role_ids: &["role:coordination-domain-surface"],
            anchors: &["weave agent communication layer"],
        },
        OperatingCapabilityDefinition {
            id: "capability:domain-upgrade",
            name: "Domain upgrade path",
            layer: "layer:coordination-communication",
            purpose: "Routes domain behavior through weave plus Obscura upgrades",
            repo_names: &["weave", "obscura"],
            role_ids: &["role:domain-upgrade-surface"],
            anchors: &["weave plus Obscura domain upgrades"],
        },
        OperatingCapabilityDefinition {
            id: "capability:env-vault-relay",
            name: "Environment and vault relay",
            layer: "layer:environment-security",
            purpose: "Mints relay credentials from long-running vault material through parent-managed env tooling",
            repo_names: &["envctl", "vault_hub"],
            role_ids: &["role:toolchain-provider"],
            anchors: &["/run/media/drdave/COGNITUM", "Cognitum vault on Pi Zero"],
        },
        OperatingCapabilityDefinition {
            id: "capability:prompt-front-door",
            name: "Prompt front door",
            layer: "layer:front-door-experience",
            purpose: "Routes prompts into handoff and Rusty IDD lifecycle automation",
            repo_names: &["prompt_hub"],
            role_ids: &["role:spec-producer"],
            anchors: &[
                "github.com/f/prompts.chat",
                "github.com/f/ai-prompt",
                "prompt_hub front door to handoff and rusty-idd",
            ],
        },
        OperatingCapabilityDefinition {
            id: "capability:user-front-door",
            name: "User front door",
            layer: "layer:front-door-experience",
            purpose: "Operator chat, LifeOS, and UI entrypoint for the agentic system",
            repo_names: &["lifeos", "ruvector", "prompt_hub"],
            role_ids: &[],
            anchors: &["goose-like chat integration", "LifeOS front door"],
        },
        OperatingCapabilityDefinition {
            id: "capability:vector-runtime",
            name: "Vector and agentic runtime",
            layer: "layer:knowledge-runtime",
            purpose: "Provides vector DB, progress DB, agentic runtime, inference, and training surfaces",
            repo_names: &["ruvector", "database_hub", "icm"],
            role_ids: &["role:knowledge-memory"],
            anchors: &["meta-ruvector full agentic system"],
        },
        OperatingCapabilityDefinition {
            id: "capability:agent-harness",
            name: "Agent harness runtime",
            layer: "layer:agent-runtime",
            purpose: "Builds and runs agent harnesses and automation workers",
            repo_names: &[
                "harness_hub",
                "flexnetos_runner",
                "agent",
                "hermes_agent",
                "n8n",
                "ruflo",
            ],
            role_ids: &["role:agent-environment"],
            anchors: &["harness-agent-rs rust port"],
        },
        OperatingCapabilityDefinition {
            id: "capability:github-agent-run-upgrades",
            name: "GitHub agent-run upgrades",
            layer: "layer:agent-runtime",
            purpose: "Provides GRIT and Beads foundations for GitHub-centered agent contribution runs",
            repo_names: &["grit", "yazelix"],
            role_ids: &[],
            anchors: &[
                "GRIT from rtk-ai",
                "Beads mandatory for code contributors through Yazelix",
                "github.com/Dicklesworthstone/beads_rust@2d824a8deaa203d64326849d86f8e6d4a9c24eca",
                "github.com/delightful-ai/beads-rs@d98da231d068acbadcdcd2262971c561de86132b",
            ],
        },
        OperatingCapabilityDefinition {
            id: "capability:digital-twin-simulation",
            name: "Digital twin simulation",
            layer: "layer:simulation-validation",
            purpose: "Simulates target environments so agents can test implementation behavior before real-world execution",
            repo_names: &["teri"],
            role_ids: &[],
            anchors: &["Teri digital twin simulator"],
        },
        OperatingCapabilityDefinition {
            id: "capability:network-engineering",
            name: "Network engineering and control",
            layer: "layer:infrastructure-device-fabric",
            purpose: "Owns network engineering, control, and lane-to-network-manager upgrade path",
            repo_names: &["lane", "network_control", "network_hub"],
            role_ids: &[],
            anchors: &["lane merges into network-manager"],
        },
        OperatingCapabilityDefinition {
            id: "capability:distributed-device-fabric",
            name: "Distributed device fabric",
            layer: "layer:infrastructure-device-fabric",
            purpose: "Uses user devices for distributed compute, storage, inference, and memory",
            repo_names: &["oh_my_pi", "network_control", "envctl"],
            role_ids: &[],
            anchors: &["user devices for distributed compute storage inference memory"],
        },
        OperatingCapabilityDefinition {
            id: "capability:parser-runtime",
            name: "Parser and terminal runtime",
            layer: "layer:toolchain-parser-runtime",
            purpose: "Carries tree-sitter, Yazelix terminal, parser, and runtime support",
            repo_names: &["yazelix", "rusty_idd", "tool_hub"],
            role_ids: &["role:parser-runtime-surface"],
            anchors: &[
                "tree-sitter via Yazelix",
                "Yazelix default terminal",
                "nushell",
                "Lua",
                "Ghostty",
                "Zellij",
            ],
        },
        OperatingCapabilityDefinition {
            id: "capability:rtk-ai-foundation",
            name: "RTK AI foundation",
            layer: "layer:toolchain-parser-runtime",
            purpose: "Provides foundational RTK, ICM, VOX, and GRIT surfaces from rtk-ai",
            repo_names: &["rtk_tokenkill", "icm", "vox", "grit"],
            role_ids: &[],
            anchors: &[
                "RTK from rtk-ai",
                "ICM from rtk-ai",
                "VOX from rtk-ai",
                "GRIT from rtk-ai",
            ],
        },
        OperatingCapabilityDefinition {
            id: "capability:lua-ar-interface",
            name: "Lua and AR interface automation",
            layer: "layer:interface-automation",
            purpose: "Supports AR-glasses coding and local automation with Rust-native Lua surfaces",
            repo_names: &["lifeos", "oh_my_pi", "yazelix"],
            role_ids: &[],
            anchors: &[
                "Lua required for AR glasses workflow",
                "Brilliant Labs Noa style Rust-native agent UX",
            ],
        },
        OperatingCapabilityDefinition {
            id: "capability:personal-automation",
            name: "Personal media and home automation",
            layer: "layer:interface-automation",
            purpose: "Adds local personal life, media, TV, and home automation surfaces",
            repo_names: &["lifeos", "oh_my_pi"],
            role_ids: &[],
            anchors: &["personal life media TV home automation"],
        },
    ]
}

fn matching_operating_repos(
    system: &SystemArchitectureGraph,
    definition: &OperatingCapabilityDefinition,
) -> Vec<String> {
    let expected_names = definition
        .repo_names
        .iter()
        .map(|name| canonical_repo_name(name))
        .collect::<BTreeSet<_>>();
    let expected_roles = definition.role_ids.iter().copied().collect::<BTreeSet<_>>();
    let mut repos = system
        .repos
        .iter()
        .filter(|repo| {
            expected_names.contains(&canonical_repo_name(&repo.name))
                || repo
                    .roles
                    .iter()
                    .any(|role| expected_roles.contains(role.as_str()))
        })
        .map(|repo| repo.id.clone())
        .collect::<Vec<_>>();
    repos.sort();
    repos.dedup();
    repos
}

fn operating_capability_evidence(definition: &OperatingCapabilityDefinition) -> Vec<String> {
    let mut evidence = Vec::new();
    evidence.extend(
        definition
            .repo_names
            .iter()
            .map(|name| format!("repo-name:{name}")),
    );
    evidence.extend(
        definition
            .role_ids
            .iter()
            .map(|role| format!("system-role:{role}")),
    );
    evidence.extend(
        definition
            .anchors
            .iter()
            .map(|anchor| format!("anchor:{anchor}")),
    );
    evidence
}

fn canonical_repo_name(name: &str) -> String {
    name.to_ascii_lowercase().replace(['-', ' '], "_")
}

fn render_system_operating_model_markdown(model: &SystemOperatingModel) -> String {
    let mut out = String::new();
    out.push_str("# System Operating Model\n\n");
    out.push_str(&format!("- System root: `{}`\n", model.system_root));
    out.push_str(&format!("- Workspace root: `{}`\n", model.workspace_root));
    out.push_str(&format!("- Source graph: `{}`\n", model.source_graph));
    out.push_str(&format!("- Layers: {}\n", model.layers.len()));
    out.push_str(&format!("- Capabilities: {}\n", model.capabilities.len()));
    out.push_str(&format!("- Edges: {}\n\n", model.edges.len()));

    out.push_str("## Layers\n\n");
    out.push_str("| Layer | Purpose | Capabilities | Repos |\n|---|---|---:|---:|\n");
    for layer in &model.layers {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            layer.name,
            layer.purpose,
            layer.capabilities.len(),
            layer.repos.len()
        ));
    }

    out.push_str("\n## Capabilities\n\n");
    out.push_str("| Capability | Layer | Status | Repos | Anchors |\n|---|---|---|---|---|\n");
    for capability in &model.capabilities {
        out.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} |\n",
            capability.name,
            capability.layer,
            capability.status,
            capability.repos.join(", "),
            capability.anchors.join(", ")
        ));
    }

    out.push_str("\n## Edges\n\n");
    out.push_str("| Source | Kind | Target |\n|---|---|---|\n");
    for edge in &model.edges {
        out.push_str(&format!(
            "| `{}` | {} | `{}` |\n",
            edge.source, edge.kind, edge.target
        ));
    }

    out.push_str("\n## Findings\n\n");
    if model.findings.is_empty() {
        out.push_str("No findings.\n");
    } else {
        for finding in &model.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }
    out
}

fn integration_automation_plan(
    workspace: &Path,
    operating_model: &SystemOperatingModel,
    source_path: &Path,
) -> IntegrationAutomationPlan {
    let mut work_items = operating_model
        .capabilities
        .iter()
        .filter(|capability| capability.status != "mapped")
        .map(integration_work_item)
        .collect::<Vec<_>>();
    work_items.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.id.cmp(&b.id)));

    let gates = default_integration_gates();
    let mut findings = vec![format!(
        "integration plan derived {} work items from {} operating capabilities",
        work_items.len(),
        operating_model.capabilities.len()
    )];
    let external_anchor_count = work_items
        .iter()
        .map(|item| item.adopt_first_inputs.len())
        .sum::<usize>();
    findings.push(format!(
        "{external_anchor_count} adopt-first inputs preserved from operating-model anchors"
    ));
    findings.sort();

    IntegrationAutomationPlan {
        schema_version: 1,
        workspace_root: workspace.display().to_string(),
        system_root: operating_model.system_root.clone(),
        source_model: display_path(workspace, source_path),
        work_items,
        gates,
        findings,
    }
}

fn integration_work_item(capability: &OperatingCapability) -> IntegrationWorkItem {
    let priority = integration_priority(capability);
    let change_id = format!(
        "integrate-{}",
        capability
            .id
            .strip_prefix("capability:")
            .unwrap_or(&capability.id)
    );
    let title = format!("Integrate {}", capability.name);
    let adopt_first_inputs = capability
        .anchors
        .iter()
        .filter(|anchor| is_adopt_first_anchor(anchor))
        .cloned()
        .collect::<Vec<_>>();
    let implementation_boundary = if capability
        .anchors
        .iter()
        .any(|anchor| anchor.contains("vault") || anchor.contains("COGNITUM"))
    {
        "Feature-gate host/vault behavior; keep default Rusty IDD generation read-only".to_string()
    } else if capability
        .anchors
        .iter()
        .any(|anchor| anchor.contains("Beads") || anchor.contains("github.com/"))
    {
        "Adopt upstream repo surface first, run native diagnostics, then add thin Rusty IDD mapping"
            .to_string()
    } else if capability.repos.is_empty() {
        "Add repo ownership evidence before implementation".to_string()
    } else {
        "Use OpenSpec change in owning repos with Rusty IDD graph artifacts as planning input"
            .to_string()
    };

    IntegrationWorkItem {
        id: format!("work:{}", slug(&change_id)),
        title,
        capability: capability.id.clone(),
        layer: capability.layer.clone(),
        priority,
        status: capability.status.clone(),
        change_id,
        owner_repos: capability.repos.clone(),
        anchors: capability.anchors.clone(),
        adopt_first_inputs,
        implementation_boundary,
        validation: default_integration_gates(),
        rollback: vec![
            "Revert the OpenSpec change and generated artifacts for this integration slice"
                .to_string(),
            "Re-run rusty-idd knowledge refresh, system-architecture, operating-model, plan-context, and manifest"
                .to_string(),
            "Re-run focused owner-repo tests plus Rusty IDD gates".to_string(),
        ],
    }
}

fn integration_priority(capability: &OperatingCapability) -> u32 {
    match capability.id.as_str() {
        "capability:idd-spec-engine" => 10,
        "capability:fleet-handoff" => 20,
        "capability:agent-communication" => 30,
        "capability:env-vault-relay" => 40,
        "capability:prompt-front-door" => 50,
        "capability:rtk-ai-foundation" => 60,
        "capability:github-agent-run-upgrades" => 70,
        "capability:parser-runtime" => 80,
        "capability:vector-runtime" => 90,
        "capability:user-front-door" => 100,
        "capability:digital-twin-simulation" => 110,
        "capability:network-engineering" => 120,
        "capability:distributed-device-fabric" => 130,
        "capability:lua-ar-interface" => 140,
        "capability:personal-automation" => 150,
        _ => 500,
    }
}

fn is_adopt_first_anchor(anchor: &str) -> bool {
    anchor.contains("github.com/")
        || anchor.contains("upstream")
        || anchor.contains("Beads")
        || anchor.contains("goose")
        || anchor.contains("COGNITUM")
        || anchor.contains("Cognitum")
}

fn default_integration_gates() -> Vec<String> {
    vec![
        "cargo fmt --all -- --check".to_string(),
        "cargo test --workspace --all-features --locked".to_string(),
        "RUSTDOCFLAGS=\"-D warnings\" cargo doc --workspace --all-features --no-deps --locked"
            .to_string(),
        "cargo audit --deny warnings".to_string(),
        "cargo run --bin rusty-idd -- validate --workspace .".to_string(),
        "cargo run --bin rusty-idd -- spec validate --all".to_string(),
        "just ci".to_string(),
        "make ci".to_string(),
        "affected CLI smoke tests".to_string(),
    ]
}

fn render_integration_automation_plan_markdown(plan: &IntegrationAutomationPlan) -> String {
    let mut out = String::new();
    out.push_str("# Integration Automation Plan\n\n");
    out.push_str(&format!("- System root: `{}`\n", plan.system_root));
    out.push_str(&format!("- Workspace root: `{}`\n", plan.workspace_root));
    out.push_str(&format!("- Source model: `{}`\n", plan.source_model));
    out.push_str(&format!("- Work items: {}\n\n", plan.work_items.len()));

    out.push_str("## Work Items\n\n");
    out.push_str("| Priority | Work Item | Capability | Status | Owners | Adopt First |\n|---:|---|---|---|---|---|\n");
    for item in &plan.work_items {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {} | {} | {} |\n",
            item.priority,
            item.title,
            item.capability,
            item.status,
            item.owner_repos.join(", "),
            item.adopt_first_inputs.join(", ")
        ));
    }

    out.push_str("\n## Gates\n\n");
    for gate in &plan.gates {
        out.push_str(&format!("- `{gate}`\n"));
    }

    out.push_str("\n## Rollback Pattern\n\n");
    out.push_str("- Revert the integration slice commit or PR.\n");
    out.push_str("- Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.\n");
    out.push_str("- Re-run focused tests plus full Rusty IDD gates.\n");

    out.push_str("\n## Findings\n\n");
    if plan.findings.is_empty() {
        out.push_str("No findings.\n");
    } else {
        for finding in &plan.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }
    out
}

fn integration_status_report(
    workspace: &Path,
    plan: &IntegrationAutomationPlan,
    source_path: &Path,
) -> IntegrationStatusReport {
    let mut work_items = plan
        .work_items
        .iter()
        .map(|item| integration_work_status(workspace, item))
        .collect::<Vec<_>>();
    work_items.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then(a.change_id.cmp(&b.change_id))
    });

    let mut counts = IntegrationStatusCounts {
        total: work_items.len(),
        ..Default::default()
    };
    for item in &work_items {
        match item.status.as_str() {
            "planned" => counts.planned += 1,
            "incomplete-scaffold" => counts.incomplete_scaffold += 1,
            "scaffolded" => counts.scaffolded += 1,
            "ready-to-archive" => counts.ready_to_archive += 1,
            "archived" => counts.archived += 1,
            _ => {}
        }
    }

    let next_change_id = work_items
        .iter()
        .find(|item| item.status == "planned")
        .map(|item| item.change_id.clone());
    let mut findings = vec![format!(
        "integration status classified {} work items from {}",
        work_items.len(),
        display_path(workspace, source_path)
    )];
    if let Some(next) = &next_change_id {
        findings.push(format!("next planned integration work item is {next}"));
    } else {
        findings.push("no planned integration work items remain".to_string());
    }
    findings.sort();

    IntegrationStatusReport {
        schema_version: 1,
        workspace_root: workspace.display().to_string(),
        source_plan: display_path(workspace, source_path),
        next_change_id,
        counts,
        work_items,
        findings,
    }
}

fn integration_work_status(workspace: &Path, item: &IntegrationWorkItem) -> IntegrationWorkStatus {
    let changes_root = workspace.join("openspec/changes");
    let active_dir = changes_root.join(&item.change_id);
    let archive_dir = changes_root.join("archive").join(&item.change_id);

    let (status, openspec_path, missing_artifacts, unchecked_tasks) = if archive_dir.is_dir() {
        (
            "archived".to_string(),
            Some(display_path(workspace, &archive_dir)),
            Vec::new(),
            0,
        )
    } else if active_dir.is_dir() {
        let missing = missing_integration_artifacts(&active_dir);
        let unchecked = unchecked_task_count(&active_dir.join("tasks.md"));
        let status = if !missing.is_empty() {
            "incomplete-scaffold"
        } else if unchecked == 0 {
            "ready-to-archive"
        } else {
            "scaffolded"
        }
        .to_string();
        (
            status,
            Some(display_path(workspace, &active_dir)),
            missing,
            unchecked,
        )
    } else {
        ("planned".to_string(), None, Vec::new(), 0)
    };

    IntegrationWorkStatus {
        id: item.id.clone(),
        title: item.title.clone(),
        capability: item.capability.clone(),
        priority: item.priority,
        change_id: item.change_id.clone(),
        status,
        openspec_path,
        missing_artifacts,
        unchecked_tasks,
        owner_repos: item.owner_repos.clone(),
        adopt_first_inputs: item.adopt_first_inputs.clone(),
    }
}

fn missing_integration_artifacts(change_dir: &Path) -> Vec<String> {
    let mut missing = Vec::new();
    for artifact in ["proposal.md", "design.md", "tasks.md"] {
        if !change_dir.join(artifact).is_file() {
            missing.push(artifact.to_string());
        }
    }
    if !has_spec_delta(change_dir) {
        missing.push("specs/**/spec.md".to_string());
    }
    missing
}

fn has_spec_delta(change_dir: &Path) -> bool {
    has_file_named(&change_dir.join("specs"), "spec.md")
}

fn has_file_named(dir: &Path, file_name: &str) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if has_file_named(&path, file_name) {
                return true;
            }
        } else if path.file_name().and_then(|name| name.to_str()) == Some(file_name) {
            return true;
        }
    }
    false
}

fn unchecked_task_count(path: &Path) -> usize {
    fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .filter(|line| line.contains("- [ ]"))
                .count()
        })
        .unwrap_or(0)
}

fn render_integration_status_markdown(report: &IntegrationStatusReport) -> String {
    let mut out = String::new();
    out.push_str("# Integration Status Queue\n\n");
    out.push_str(&format!("- Workspace root: `{}`\n", report.workspace_root));
    out.push_str(&format!("- Source plan: `{}`\n", report.source_plan));
    match &report.next_change_id {
        Some(next) => out.push_str(&format!("- Next planned work: `{next}`\n\n")),
        None => out.push_str("- Next planned work: none\n\n"),
    }

    out.push_str("## Counts\n\n");
    out.push_str(
        "| Total | Planned | Incomplete Scaffold | Scaffolded | Ready To Archive | Archived |\n",
    );
    out.push_str("|---:|---:|---:|---:|---:|---:|\n");
    out.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} |\n\n",
        report.counts.total,
        report.counts.planned,
        report.counts.incomplete_scaffold,
        report.counts.scaffolded,
        report.counts.ready_to_archive,
        report.counts.archived
    ));

    out.push_str("## Work Items\n\n");
    out.push_str(
        "| Priority | Change | Status | Capability | OpenSpec | Missing | Unchecked Tasks |\n",
    );
    out.push_str("|---:|---|---|---|---|---|---:|\n");
    for item in &report.work_items {
        out.push_str(&format!(
            "| {} | `{}` | {} | `{}` | {} | {} | {} |\n",
            item.priority,
            item.change_id,
            item.status,
            item.capability,
            item.openspec_path
                .as_ref()
                .map(|path| format!("`{path}`"))
                .unwrap_or_else(String::new),
            item.missing_artifacts.join(", "),
            item.unchecked_tasks
        ));
    }

    out.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        out.push_str("No findings.\n");
    } else {
        for finding in &report.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }
    out
}

fn integration_owner_surfaces(
    workspace: &Path,
    plan_path: &Path,
    system_path: &Path,
    plan: IntegrationAutomationPlan,
    system: SystemArchitectureGraph,
    options: IntegrationOwnersOptions,
) -> Result<IntegrationOwnersReport> {
    let selector = IntegrationOwnerSelector {
        change: options.change,
        capability: options.capability,
        work_item: options.work_item,
        next: options.next,
        next_planned: options.next_planned,
    };
    let selected = select_owner_work_item(workspace, &plan, &selector)?;
    let repo_by_id = system
        .repos
        .iter()
        .map(|repo| (repo.id.as_str(), repo))
        .collect::<BTreeMap<_, _>>();
    let mut owner_surfaces = Vec::new();
    let mut missing_owner_repos = Vec::new();

    for owner_repo in &selected.owner_repos {
        if let Some(repo) = repo_by_id.get(owner_repo.as_str()) {
            owner_surfaces.push(owner_surface_from_repo(owner_repo, repo));
        } else {
            missing_owner_repos.push(owner_repo.clone());
            owner_surfaces.push(missing_owner_surface(owner_repo));
        }
    }

    let mut diagnostics = owner_surfaces
        .iter()
        .flat_map(|surface| surface.native_diagnostic_commands.clone())
        .collect::<Vec<_>>();
    diagnostics.sort();
    diagnostics.dedup();

    let mut findings = vec![
        format!(
            "selected {} from {}",
            selected.change_id,
            display_path(workspace, plan_path)
        ),
        format!(
            "joined {} owner repos against {}",
            selected.owner_repos.len(),
            display_path(workspace, system_path)
        ),
    ];
    if missing_owner_repos.is_empty() {
        findings.push("all owner repos resolved in the system architecture graph".to_string());
    } else {
        findings.push(format!(
            "{} owner repos are missing from the system architecture graph: {}",
            missing_owner_repos.len(),
            missing_owner_repos.join(", ")
        ));
    }
    let dirty_owners = owner_surfaces
        .iter()
        .filter(|surface| surface.repo_found && surface.dirty)
        .count();
    findings.push(format!(
        "{dirty_owners} resolved owner repos report dirty state"
    ));
    findings.sort();

    Ok(IntegrationOwnersReport {
        schema_version: 1,
        workspace_root: workspace.display().to_string(),
        source_plan: display_path(workspace, plan_path),
        source_system_architecture: display_path(workspace, system_path),
        selector,
        work_item: selected,
        owner_surfaces,
        missing_owner_repos,
        diagnostics,
        findings,
    })
}

fn select_owner_work_item(
    workspace: &Path,
    plan: &IntegrationAutomationPlan,
    selector: &IntegrationOwnerSelector,
) -> Result<IntegrationWorkItem> {
    let selected_count = [
        selector.change.as_ref().map(|_| ()),
        selector.capability.as_ref().map(|_| ()),
        selector.work_item.as_ref().map(|_| ()),
        selector.next.then_some(()),
        selector.next_planned.then_some(()),
    ]
    .into_iter()
    .flatten()
    .count();
    if selected_count != 1 {
        bail!(
            "select exactly one of --change, --capability, --work-item, --next, or --next-planned"
        );
    }

    if selector.next {
        return plan
            .work_items
            .iter()
            .map(|item| integration_work_status(workspace, item))
            .filter(|status| status.status != "archived")
            .min_by(|a, b| {
                a.priority
                    .cmp(&b.priority)
                    .then(a.change_id.cmp(&b.change_id))
            })
            .and_then(|status| {
                plan.work_items
                    .iter()
                    .find(|item| item.change_id == status.change_id)
                    .cloned()
            })
            .ok_or_else(|| anyhow::anyhow!("no non-archived integration work item remains"));
    }

    if selector.next_planned {
        return plan
            .work_items
            .iter()
            .map(|item| integration_work_status(workspace, item))
            .filter(|status| status.status == "planned")
            .min_by(|a, b| {
                a.priority
                    .cmp(&b.priority)
                    .then(a.change_id.cmp(&b.change_id))
            })
            .and_then(|status| {
                plan.work_items
                    .iter()
                    .find(|item| item.change_id == status.change_id)
                    .cloned()
            })
            .ok_or_else(|| anyhow::anyhow!("no planned integration work item remains"));
    }

    let matches = plan
        .work_items
        .iter()
        .filter(|item| {
            selector
                .change
                .as_ref()
                .is_some_and(|change| item.change_id == *change)
                || selector
                    .capability
                    .as_ref()
                    .is_some_and(|capability| item.capability == *capability)
                || selector
                    .work_item
                    .as_ref()
                    .is_some_and(|work_item| item.id == *work_item)
        })
        .collect::<Vec<_>>();

    match matches.as_slice() {
        [item] => Ok((*item).clone()),
        [] => bail!("no integration work item matched the selected owner-surface selector"),
        _ => bail!("owner-surface selector matched multiple integration work items"),
    }
}

fn owner_surface_from_repo(owner_repo: &str, repo: &SystemRepo) -> IntegrationOwnerSurface {
    IntegrationOwnerSurface {
        owner_repo: owner_repo.to_string(),
        repo_found: true,
        repo_name: Some(repo.name.clone()),
        path: Some(repo.path.clone()),
        remote: repo.repo.clone(),
        branch: repo.branch.clone(),
        head: repo.head.clone(),
        dirty: repo.dirty,
        tags: repo.tags.clone(),
        markers: repo.markers.clone(),
        roles: repo.roles.clone(),
        has_local_architecture_graph: repo.has_local_architecture_graph,
        local_architecture: repo.local_architecture.clone(),
        evidence_paths: owner_evidence_paths(repo),
        native_diagnostic_commands: native_owner_diagnostic_commands(repo),
    }
}

fn missing_owner_surface(owner_repo: &str) -> IntegrationOwnerSurface {
    IntegrationOwnerSurface {
        owner_repo: owner_repo.to_string(),
        repo_found: false,
        repo_name: None,
        path: None,
        remote: None,
        branch: None,
        head: None,
        dirty: false,
        tags: Vec::new(),
        markers: Vec::new(),
        roles: Vec::new(),
        has_local_architecture_graph: false,
        local_architecture: None,
        evidence_paths: Vec::new(),
        native_diagnostic_commands: Vec::new(),
    }
}

fn owner_evidence_paths(repo: &SystemRepo) -> Vec<String> {
    let mut paths = vec![repo.path.clone()];
    for (marker, suffix) in [
        ("rust", "Cargo.toml"),
        ("node", "package.json"),
        ("openspec", "openspec"),
        ("idd-knowledge", ".idd/knowledge"),
        ("handoff", ".handoff"),
        ("agents", ".agents"),
        ("claude", ".claude"),
        ("github-actions", ".github/workflows"),
        ("make", "Makefile"),
        ("just", "Justfile"),
    ] {
        if repo.markers.iter().any(|value| value == marker) {
            paths.push(format!("{}/{}", repo.path, suffix));
        }
    }
    if repo.has_local_architecture_graph {
        paths.push(format!("{}/.idd/knowledge/architecture.json", repo.path));
    }
    paths.sort();
    paths.dedup();
    paths
}

fn native_owner_diagnostic_commands(repo: &SystemRepo) -> Vec<String> {
    let mut commands = vec![
        format!("git -C {} status --short --branch", repo.path),
        format!("git -C {} rev-parse HEAD", repo.path),
    ];
    if repo.markers.iter().any(|marker| marker == "rust") {
        commands.push(format!(
            "cd {} && cargo metadata --locked --format-version 1",
            repo.path
        ));
        commands.push(format!(
            "cd {} && cargo test --workspace --all-features --locked",
            repo.path
        ));
    }
    if repo.markers.iter().any(|marker| marker == "just") {
        commands.push(format!("cd {} && just --list", repo.path));
        commands.push(format!("cd {} && just ci", repo.path));
    }
    if repo.markers.iter().any(|marker| marker == "make") {
        commands.push(format!("cd {} && make -n ci", repo.path));
        commands.push(format!("cd {} && make ci", repo.path));
    }
    if repo.markers.iter().any(|marker| marker == "node") {
        commands.push(format!("cd {} && npm run", repo.path));
        commands.push(format!("cd {} && npm test", repo.path));
    }
    if repo.has_local_architecture_graph {
        commands.push(format!(
            "test -f {}/.idd/knowledge/architecture.json",
            repo.path
        ));
    }
    commands.sort();
    commands.dedup();
    commands
}

fn integration_readiness_report(
    workspace: &Path,
    plan_path: &Path,
    system_path: &Path,
    owners: IntegrationOwnersReport,
) -> IntegrationReadinessReport {
    let mut tools = BTreeMap::<String, IntegrationToolRequirement>::new();
    let mut owner_states = Vec::new();
    let mut native_diagnostics = Vec::new();
    let mut runtime_assumptions = Vec::new();
    let mut feature_gates = vec![
        "Default Rusty IDD knowledge and planning commands remain read-only".to_string(),
        "Host vault probing, secret relay minting, and long-running service control require an explicit feature or CLI opt-in".to_string(),
        "Missing tools are provisioned through parent meta/envctl or tracked repo-local surfaces, not user-global installs".to_string(),
        "Peer repo writes and branch changes stay outside default Rusty IDD readiness generation".to_string(),
    ];

    add_tool_requirement(
        &mut tools,
        "git",
        "Git",
        "integration-owner-state",
        "parent meta/envctl managed PATH",
        true,
        "native owner diagnostics include git state checks",
    );

    for surface in &owners.owner_surfaces {
        let mut required_tool_ids = BTreeSet::<String>::from(["git".to_string()]);
        if surface.markers.iter().any(|marker| marker == "rust")
            || surface
                .native_diagnostic_commands
                .iter()
                .any(|command| command.contains("cargo "))
        {
            required_tool_ids.insert("cargo".to_string());
            add_tool_requirement(
                &mut tools,
                "cargo",
                "Rust Cargo",
                &surface.owner_repo,
                "parent meta/envctl Rust toolchain",
                true,
                "owner repo exposes Rust package metadata or cargo diagnostics",
            );
        }
        for (id, name, marker, evidence) in [
            (
                "just",
                "Just",
                "just",
                "owner repo exposes Justfile diagnostics",
            ),
            (
                "make",
                "Make",
                "make",
                "owner repo exposes Makefile diagnostics",
            ),
            (
                "node",
                "Node/npm",
                "node",
                "owner repo exposes package.json diagnostics",
            ),
        ] {
            if surface.markers.iter().any(|value| value == marker)
                || surface
                    .native_diagnostic_commands
                    .iter()
                    .any(|command| command.contains(id) || command.contains("npm "))
            {
                required_tool_ids.insert(id.to_string());
                add_tool_requirement(
                    &mut tools,
                    id,
                    name,
                    &surface.owner_repo,
                    "parent meta/envctl managed toolchain",
                    true,
                    evidence,
                );
            }
        }
        if surface.owner_repo == "repo:envctl"
            || surface
                .roles
                .iter()
                .any(|role| role == "role:toolchain-provider")
        {
            required_tool_ids.insert("envctl".to_string());
            add_tool_requirement(
                &mut tools,
                "envctl",
                "envctl",
                &surface.owner_repo,
                "parent meta/envctl",
                true,
                "owner repo is the tracked toolchain provider for this capability",
            );
        }
        if surface.owner_repo == "repo:vault-hub" {
            required_tool_ids.insert("kasetto".to_string());
            add_tool_requirement(
                &mut tools,
                "kasetto",
                "Kasetto",
                &surface.owner_repo,
                "vault_hub/kasetto through parent meta/envctl",
                false,
                "vault_hub README identifies kasetto as its Rust capability manager",
            );
        }
        if surface.owner_repo == "repo:yazelix" {
            for (id, name, evidence) in [
                (
                    "nix",
                    "Nix",
                    "Yazelix publishes flake and Nix package/runtime surfaces",
                ),
                (
                    "nushell",
                    "Nushell",
                    "Yazelix owns Nushell runtime configuration",
                ),
                ("lua", "Lua", "Yazelix ships Lua plugin/runtime surfaces"),
                (
                    "ghostty",
                    "Ghostty",
                    "Yazelix default terminal runtime includes Ghostty",
                ),
                (
                    "zellij",
                    "Zellij",
                    "Yazelix workspace orchestration is Zellij-backed",
                ),
                (
                    "beads",
                    "Beads Rust",
                    "Yazelix agent workflow makes Beads mandatory for contributors",
                ),
            ] {
                required_tool_ids.insert(id.to_string());
                add_tool_requirement(
                    &mut tools,
                    id,
                    name,
                    &surface.owner_repo,
                    "Yazelix packaged runtime through parent meta/envctl",
                    false,
                    evidence,
                );
            }
        }

        owner_states.push(IntegrationReadinessOwnerState {
            owner_repo: surface.owner_repo.clone(),
            repo_found: surface.repo_found,
            repo_name: surface.repo_name.clone(),
            path: surface.path.clone(),
            branch: surface.branch.clone(),
            head: surface.head.clone(),
            dirty: surface.dirty,
            required_tool_ids: required_tool_ids.into_iter().collect(),
        });

        for command in &surface.native_diagnostic_commands {
            native_diagnostics.push(IntegrationNativeDiagnostic {
                command: command.clone(),
                owner_repo: Some(surface.owner_repo.clone()),
                required_tool_ids: diagnostic_tool_ids(command),
                mode: if diagnostic_command_is_read_only(command) {
                    "read-only".to_string()
                } else {
                    "native-build-or-test".to_string()
                },
                mutates_repo: diagnostic_command_mutates_repo(command),
            });
        }
    }

    let upstream_inputs = owners
        .work_item
        .adopt_first_inputs
        .iter()
        .map(|input| integration_upstream_input(input))
        .collect::<Vec<_>>();
    for upstream in &upstream_inputs {
        for tool_id in &upstream.required_tool_ids {
            let (name, provisioned_by, default_path, evidence) = match tool_id.as_str() {
                "git" => (
                    "Git",
                    "parent meta/envctl managed PATH",
                    true,
                    "upstream adoption pins exact git revisions before consolidation",
                ),
                "node" => (
                    "Node/npm",
                    "parent meta/envctl managed Node/npm toolchain",
                    true,
                    "upstream package metadata exposes npm native diagnostics",
                ),
                "postgres" => (
                    "PostgreSQL-compatible DATABASE_URL",
                    "parent meta/envctl managed runtime or explicit external service",
                    false,
                    "upstream postinstall/build commands require DATABASE_URL for Prisma generation",
                ),
                "wordpress" => (
                    "WordPress/Gutenberg toolchain",
                    "parent meta/envctl managed frontend/tooling surface",
                    false,
                    "upstream WordPress plugin scripts are native diagnostic surfaces",
                ),
                _ => (
                    tool_id.as_str(),
                    "parent meta/envctl managed toolchain",
                    false,
                    "upstream adoption records this tool as a native requirement",
                ),
            };
            add_tool_requirement(
                &mut tools,
                tool_id,
                name,
                &upstream.source,
                provisioned_by,
                default_path,
                evidence,
            );
        }
        runtime_assumptions.extend(upstream.runtime_assumptions.iter().cloned());
        feature_gates.extend(upstream.feature_flags.iter().cloned());
        for command in &upstream.native_diagnostic_commands {
            native_diagnostics.push(IntegrationNativeDiagnostic {
                command: command.clone(),
                owner_repo: Some(upstream.source.clone()),
                required_tool_ids: diagnostic_tool_ids(command),
                mode: if diagnostic_command_is_read_only(command) {
                    "read-only".to_string()
                } else {
                    "native-build-or-test".to_string()
                },
                mutates_repo: diagnostic_command_mutates_repo(command),
            });
        }
    }

    for anchor in &owners.work_item.anchors {
        if anchor.contains("COGNITUM") {
            runtime_assumptions.push(
                "Cognitum path `/run/media/drdave/COGNITUM` is an external host/vault anchor and is recorded without probing by default".to_string(),
            );
            add_tool_requirement(
                &mut tools,
                "cognitum-vault",
                "Cognitum vault",
                "work-item-anchor",
                "external vault surfaced through envctl relay only",
                false,
                "integration work item records /run/media/drdave/COGNITUM",
            );
        }
        if anchor.to_ascii_lowercase().contains("pi zero") {
            runtime_assumptions.push(
                "Cognitum on Pi Zero is a remote/edge runtime assumption; Rusty IDD readiness does not start or manage that host".to_string(),
            );
        }
    }

    if runtime_assumptions.is_empty() {
        runtime_assumptions.push(
            "No host runtime probing is required for the default readiness artifact".to_string(),
        );
    }
    feature_gates.sort();
    feature_gates.dedup();
    runtime_assumptions.sort();
    runtime_assumptions.dedup();
    native_diagnostics.sort_by(|a, b| a.command.cmp(&b.command));
    native_diagnostics.dedup_by(|a, b| a.command == b.command && a.owner_repo == b.owner_repo);

    let mut validation = owners.work_item.validation.clone();
    validation.push(
        "rusty-idd knowledge integration-readiness --workspace . --next --out .idd/knowledge/integration-readiness.json".to_string(),
    );
    validation.sort();
    validation.dedup();

    let mut rollback = owners.work_item.rollback.clone();
    rollback.push(
        "remove .idd/knowledge/integration-readiness.* and regenerate integration status/owner artifacts".to_string(),
    );
    rollback.sort();
    rollback.dedup();

    let mut findings = owners.findings.clone();
    findings.push(format!(
        "readiness derived {} tool requirements from {} owner surfaces",
        tools.len(),
        owners.owner_surfaces.len()
    ));
    findings.push(
        "readiness generation is deterministic and does not execute native diagnostics".to_string(),
    );
    if owners
        .work_item
        .implementation_boundary
        .contains("Feature-gate")
    {
        findings.push(
            "selected work item requires feature-gated host/vault behavior outside default workflows"
                .to_string(),
        );
    }
    findings.sort();
    findings.dedup();

    IntegrationReadinessReport {
        schema_version: 1,
        workspace_root: workspace.display().to_string(),
        source_plan: display_path(workspace, plan_path),
        source_system_architecture: display_path(workspace, system_path),
        selector: owners.selector,
        work_item: owners.work_item,
        owner_states,
        upstream_inputs,
        tool_requirements: tools.into_values().collect(),
        native_diagnostics,
        runtime_assumptions,
        feature_gates,
        validation,
        rollback,
        findings,
    }
}

fn integration_upstream_input(source: &str) -> IntegrationUpstreamInput {
    let repo_name = source
        .rsplit('/')
        .next()
        .filter(|value| !value.is_empty())
        .unwrap_or(source)
        .trim_end_matches(".git");
    let mut required_tool_ids = BTreeSet::from(["git".to_string()]);
    let mut native_diagnostic_commands = vec![
        format!("git ls-remote {} HEAD", github_remote_url(source)),
        format!("test -f third_party/upstream/{repo_name}/package.json"),
    ];
    let mut runtime_assumptions = vec![
        "External upstream mirrors are tracked as source snapshots and are not workspace members by default".to_string(),
    ];
    let mut feature_flags = vec![
        "External upstream servers, MCP transports, and host services stay out of default Rusty IDD workflows unless a later spec explicitly gates them".to_string(),
    ];

    if source.contains("github.com/f/prompts.chat") {
        required_tool_ids.insert("node".to_string());
        required_tool_ids.insert("postgres".to_string());
        native_diagnostic_commands.extend([
            "cd third_party/upstream/prompts.chat && DATABASE_URL=\"postgresql://test:test@localhost:5432/test\" npm ci".to_string(),
            "cd third_party/upstream/prompts.chat && DATABASE_URL=\"postgresql://test:test@localhost:5432/test\" npm run lint".to_string(),
            "cd third_party/upstream/prompts.chat && DATABASE_URL=\"postgresql://test:test@localhost:5432/test\" npm test".to_string(),
        ]);
        runtime_assumptions.extend([
            "prompts.chat package metadata requires Node 24.x".to_string(),
            "prompts.chat Prisma generation requires DATABASE_URL; diagnostics may use a non-secret temporary PostgreSQL URL".to_string(),
        ]);
        feature_flags.push(
            "prompts.chat MCP/server/web runtime surfaces are adoption evidence only until a prompt-front-door feature boundary enables them".to_string(),
        );
    } else if source.contains("github.com/f/ai-prompt") {
        required_tool_ids.insert("node".to_string());
        required_tool_ids.insert("wordpress".to_string());
        native_diagnostic_commands.extend([
            "cd third_party/upstream/ai-prompt && npm ci".to_string(),
            "cd third_party/upstream/ai-prompt && npm run build".to_string(),
            "cd third_party/upstream/ai-prompt && npm run lint:js".to_string(),
            "cd third_party/upstream/ai-prompt && npm run lint:css".to_string(),
        ]);
        runtime_assumptions.push(
            "ai-prompt CI uses Node 20 for its WordPress/Gutenberg plugin diagnostics".to_string(),
        );
        feature_flags.push(
            "ai-prompt WordPress plugin UI remains an upstream prompt rendering surface until mapped through prompt_hub and Rusty IDD DTOs".to_string(),
        );
    }

    IntegrationUpstreamInput {
        source: source.to_string(),
        kind: if source.contains("github.com/") {
            "github-repository".to_string()
        } else {
            "external-anchor".to_string()
        },
        mirror_path: format!("third_party/upstream/{repo_name}"),
        required_tool_ids: required_tool_ids.into_iter().collect(),
        native_diagnostic_commands,
        runtime_assumptions,
        feature_flags,
    }
}

fn github_remote_url(source: &str) -> String {
    if source.starts_with("http://") || source.starts_with("https://") || source.ends_with(".git") {
        source.to_string()
    } else if let Some(path) = source.strip_prefix("github.com/") {
        format!("https://github.com/{path}.git")
    } else {
        source.to_string()
    }
}

fn add_tool_requirement(
    tools: &mut BTreeMap<String, IntegrationToolRequirement>,
    id: &str,
    name: &str,
    required_by: &str,
    provisioned_by: &str,
    default_path: bool,
    evidence: &str,
) {
    let requirement = tools
        .entry(id.to_string())
        .or_insert_with(|| IntegrationToolRequirement {
            id: id.to_string(),
            name: name.to_string(),
            required_by: Vec::new(),
            provisioned_by: provisioned_by.to_string(),
            default_path,
            evidence: Vec::new(),
        });
    if !requirement
        .required_by
        .iter()
        .any(|value| value == required_by)
    {
        requirement.required_by.push(required_by.to_string());
    }
    if !requirement.evidence.iter().any(|value| value == evidence) {
        requirement.evidence.push(evidence.to_string());
    }
    requirement.required_by.sort();
    requirement.evidence.sort();
    requirement.default_path &= default_path;
}

fn diagnostic_tool_ids(command: &str) -> Vec<String> {
    let mut ids = BTreeSet::new();
    if command.contains("git ") || command.starts_with("git") {
        ids.insert("git".to_string());
    }
    if command.contains("cargo ") {
        ids.insert("cargo".to_string());
    }
    if command.contains("just ") || command.ends_with(" just") {
        ids.insert("just".to_string());
    }
    if command.contains("make ") || command.ends_with(" make") {
        ids.insert("make".to_string());
    }
    if command.contains("npm ") {
        ids.insert("node".to_string());
    }
    ids.into_iter().collect()
}

fn diagnostic_command_is_read_only(command: &str) -> bool {
    command.contains(" rev-parse ")
        || command.contains(" status ")
        || command.contains(" metadata ")
        || command.contains(" ls-remote ")
        || command.contains(" --list")
        || command.starts_with("test -f ")
        || command.contains("make -n ")
}

fn diagnostic_command_mutates_repo(command: &str) -> bool {
    command.contains(" npm install")
        || command.contains(" npm ci")
        || command.contains(" pnpm install")
        || command.contains(" cargo update")
        || command.contains(" cargo install")
}

fn render_integration_owner_surfaces_markdown(report: &IntegrationOwnersReport) -> String {
    let mut out = String::new();
    out.push_str("# Integration Owner Surfaces\n\n");
    out.push_str(&format!("- Workspace root: `{}`\n", report.workspace_root));
    out.push_str(&format!("- Source plan: `{}`\n", report.source_plan));
    out.push_str(&format!(
        "- Source system architecture: `{}`\n",
        report.source_system_architecture
    ));
    out.push_str(&format!("- Change: `{}`\n", report.work_item.change_id));
    out.push_str(&format!(
        "- Capability: `{}`\n",
        report.work_item.capability
    ));
    out.push_str(&format!(
        "- Owner repos: {}\n\n",
        report.owner_surfaces.len()
    ));

    out.push_str("## Owners\n\n");
    out.push_str(
        "| Owner | Found | Repo | Branch | Dirty | Roles | Markers | Architecture |\n|---|---|---|---|---:|---|---|---|\n",
    );
    for surface in &report.owner_surfaces {
        out.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | {} | {} | {} | {} |\n",
            surface.owner_repo,
            surface.repo_found,
            surface.repo_name.as_deref().unwrap_or(""),
            surface.branch.as_deref().unwrap_or(""),
            surface.dirty,
            surface.roles.join(", "),
            surface.markers.join(", "),
            if surface.has_local_architecture_graph {
                "yes"
            } else {
                "no"
            }
        ));
    }

    out.push_str("\n## Evidence Paths\n\n");
    for surface in &report.owner_surfaces {
        out.push_str(&format!("- `{}`:", surface.owner_repo));
        if surface.evidence_paths.is_empty() {
            out.push_str(" none\n");
        } else {
            out.push('\n');
            for path in &surface.evidence_paths {
                out.push_str(&format!("  - `{path}`\n"));
            }
        }
    }

    out.push_str("\n## Native Diagnostics\n\n");
    if report.diagnostics.is_empty() {
        out.push_str("No native diagnostic command candidates discovered.\n");
    } else {
        for command in &report.diagnostics {
            out.push_str(&format!("- `{command}`\n"));
        }
    }

    out.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        out.push_str("No findings.\n");
    } else {
        for finding in &report.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }
    out
}

fn render_integration_readiness_markdown(report: &IntegrationReadinessReport) -> String {
    let mut out = String::new();
    out.push_str("# Integration Readiness\n\n");
    out.push_str(&format!("- Workspace root: `{}`\n", report.workspace_root));
    out.push_str(&format!("- Source plan: `{}`\n", report.source_plan));
    out.push_str(&format!(
        "- Source system architecture: `{}`\n",
        report.source_system_architecture
    ));
    out.push_str(&format!("- Change: `{}`\n", report.work_item.change_id));
    out.push_str(&format!(
        "- Capability: `{}`\n\n",
        report.work_item.capability
    ));

    out.push_str("## Owner States\n\n");
    out.push_str(
        "| Owner | Found | Repo | Branch | Dirty | Required Tools |\n|---|---|---|---|---:|---|\n",
    );
    for owner in &report.owner_states {
        out.push_str(&format!(
            "| `{}` | {} | `{}` | `{}` | {} | {} |\n",
            owner.owner_repo,
            owner.repo_found,
            owner.repo_name.as_deref().unwrap_or(""),
            owner.branch.as_deref().unwrap_or(""),
            owner.dirty,
            owner.required_tool_ids.join(", ")
        ));
    }

    out.push_str("\n## Upstream Inputs\n\n");
    if report.upstream_inputs.is_empty() {
        out.push_str("No adopt-first upstream inputs recorded for this work item.\n");
    } else {
        out.push_str(
            "| Source | Kind | Mirror | Required Tools | Runtime Assumptions |\n|---|---|---|---|---|\n",
        );
        for upstream in &report.upstream_inputs {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | {} | {} |\n",
                upstream.source,
                upstream.kind,
                upstream.mirror_path,
                upstream.required_tool_ids.join(", "),
                upstream.runtime_assumptions.join("; ")
            ));
        }
    }

    out.push_str("\n## Tool Requirements\n\n");
    out.push_str(
        "| Tool | Default | Provisioned By | Required By | Evidence |\n|---|---:|---|---|---|\n",
    );
    for tool in &report.tool_requirements {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} |\n",
            tool.id,
            tool.default_path,
            tool.provisioned_by,
            tool.required_by.join(", "),
            tool.evidence.join("; ")
        ));
    }

    out.push_str("\n## Native Diagnostics\n\n");
    out.push_str("| Command | Owner | Mode | Mutates Repo | Tools |\n|---|---|---|---:|---|\n");
    for diagnostic in &report.native_diagnostics {
        out.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} |\n",
            diagnostic.command,
            diagnostic.owner_repo.as_deref().unwrap_or(""),
            diagnostic.mode,
            diagnostic.mutates_repo,
            diagnostic.required_tool_ids.join(", ")
        ));
    }

    out.push_str("\n## Runtime Assumptions\n\n");
    for assumption in &report.runtime_assumptions {
        out.push_str(&format!("- {assumption}\n"));
    }

    out.push_str("\n## Feature Gates\n\n");
    for gate in &report.feature_gates {
        out.push_str(&format!("- {gate}\n"));
    }

    out.push_str("\n## Validation\n\n");
    for gate in &report.validation {
        out.push_str(&format!("- `{gate}`\n"));
    }

    out.push_str("\n## Rollback\n\n");
    for rollback in &report.rollback {
        out.push_str(&format!("- {rollback}\n"));
    }

    out.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        out.push_str("No findings.\n");
    } else {
        for finding in &report.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }
    out
}

fn graph_planning_context(
    workspace: &Path,
    options: PlanContextOptions,
) -> Result<GraphPlanningContext> {
    let architecture_path = options
        .architecture_path
        .unwrap_or_else(|| workspace.join(".idd/knowledge/architecture.json"));
    let architecture: ArchitectureGraph = read_json_file(&architecture_path)?;

    let system_path = options
        .system_architecture_path
        .unwrap_or_else(|| workspace.join(".idd/knowledge/system-architecture.json"));
    let system = if system_path.exists() {
        Some(read_json_file::<SystemArchitectureGraph>(&system_path)?)
    } else {
        None
    };
    let operating_path = options
        .operating_model_path
        .unwrap_or_else(|| workspace.join(".idd/knowledge/operating-model.json"));
    let operating_model = if operating_path.exists() {
        Some(read_json_file::<SystemOperatingModel>(&operating_path)?)
    } else {
        None
    };
    let integration_path = options
        .integration_plan_path
        .unwrap_or_else(|| workspace.join(".idd/knowledge/integration-plan.json"));
    let integration_plan = if integration_path.exists() {
        Some(read_json_file::<IntegrationAutomationPlan>(
            &integration_path,
        )?)
    } else {
        None
    };

    let focus_components = select_focus_components(&architecture, options.goal.as_deref());
    let (system_roles, system_repos, mut findings) =
        select_system_context(system.as_ref(), options.goal.as_deref());
    let (operating_layers, operating_capabilities, operating_findings) =
        select_operating_context(operating_model.as_ref(), options.goal.as_deref());
    let (integration_work_items, integration_findings) =
        select_integration_work(integration_plan.as_ref(), options.goal.as_deref());
    findings.extend(operating_findings);
    findings.extend(integration_findings);
    if system.is_none() {
        findings.push(format!(
            "system architecture graph unavailable at {}",
            system_path.display()
        ));
    }
    if operating_model.is_none() {
        findings.push(format!(
            "operating model graph unavailable at {}",
            operating_path.display()
        ));
    }
    if integration_plan.is_none() {
        findings.push(format!(
            "integration automation plan unavailable at {}",
            integration_path.display()
        ));
    }

    let mut guidance = vec![
        "Use proposal.md to bind the goal to graph-backed scope before implementation".to_string(),
        "Use specs/*/spec.md to express externally visible behavior and integration contracts"
            .to_string(),
        "Use design.md to map repo components, system roles, and feature-gated surfaces".to_string(),
        "Use ADRs for durable boundary decisions such as default workflow versus system capability"
            .to_string(),
        "Use tasks.md to make every consolidation or integration cut a test-backed step".to_string(),
        "Regenerate .idd/knowledge artifacts and .idd/MANIFEST.tsv after source or control-plane edits"
            .to_string(),
    ];
    if !system_repos.is_empty() {
        guidance.push(
            "For cross-repo work, treat peer repo state as evidence and avoid mutating peers from this command"
                .to_string(),
        );
    }

    Ok(GraphPlanningContext {
        schema_version: 1,
        change: options.change,
        goal: options.goal,
        workspace_root: architecture.workspace_root,
        source_graph: architecture.source_graph,
        context_package: architecture.context_package,
        automation_stages: architecture.automation_stages,
        integration_surfaces: architecture.integration_surfaces,
        focus_components,
        system_roles,
        system_repos,
        operating_layers,
        operating_capabilities,
        integration_work_items,
        guidance,
        findings,
    })
}

fn read_json_file<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let content = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("parse {}", path.display()))
}

fn select_focus_components(
    architecture: &ArchitectureGraph,
    goal: Option<&str>,
) -> Vec<ArchitectureComponent> {
    let goal_terms = goal_terms(goal);
    let mut scored = architecture
        .components
        .iter()
        .cloned()
        .map(|component| {
            let mut score = component.nodes + component.edges + component.files * 10;
            let haystack = [
                component.name.as_str(),
                component.kind.as_str(),
                &component.languages.join(" "),
                &component.evidence_paths.join(" "),
            ]
            .join(" ")
            .to_ascii_lowercase();
            for term in &goal_terms {
                if haystack.contains(term) {
                    score += 10_000;
                }
            }
            (score, component)
        })
        .collect::<Vec<_>>();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.id.cmp(&b.1.id)));
    scored
        .into_iter()
        .take(12)
        .map(|(_, component)| component)
        .collect()
}

fn select_system_context(
    system: Option<&SystemArchitectureGraph>,
    goal: Option<&str>,
) -> (Vec<SystemRole>, Vec<SystemRepo>, Vec<String>) {
    let Some(system) = system else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    let goal_terms = goal_terms(goal);
    let mut selected_role_ids = BTreeSet::new();
    for role in &system.roles {
        let text = format!("{} {} {}", role.id, role.name, role.purpose).to_ascii_lowercase();
        if goal_terms.is_empty()
            || goal_terms.iter().any(|term| text.contains(term))
            || matches!(
                role.id.as_str(),
                "role:idd-control-plane"
                    | "role:fleet-handoff"
                    | "role:coordination-domain-surface"
                    | "role:domain-upgrade-surface"
                    | "role:parser-runtime-surface"
                    | "role:toolchain-provider"
                    | "role:spec-producer"
                    | "role:meta-control-plane"
            )
        {
            selected_role_ids.insert(role.id.clone());
        }
    }

    let mut system_roles = system
        .roles
        .iter()
        .filter(|role| selected_role_ids.contains(&role.id))
        .cloned()
        .collect::<Vec<_>>();
    system_roles.sort_by(|a, b| a.id.cmp(&b.id));

    let mut scored_repos = Vec::new();
    for repo in &system.repos {
        let mut score = repo
            .roles
            .iter()
            .filter(|role| selected_role_ids.contains(*role))
            .count()
            * 100;
        let text = format!(
            "{} {} {} {} {}",
            repo.name,
            repo.path,
            repo.tags.join(" "),
            repo.markers.join(" "),
            repo.roles.join(" ")
        )
        .to_ascii_lowercase();
        for term in &goal_terms {
            if text.contains(term) {
                score += 1_000;
            }
        }
        if score > 0 || repo.name == "rusty-idd" {
            scored_repos.push((score, repo.clone()));
        }
    }
    scored_repos.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.cmp(&b.1.name)));
    let system_repos = scored_repos
        .into_iter()
        .take(20)
        .map(|(_, repo)| repo)
        .collect::<Vec<_>>();

    let findings = vec![format!(
        "system context selected {} roles and {} repos from {} discovered repos",
        system_roles.len(),
        system_repos.len(),
        system.repos.len()
    )];
    (system_roles, system_repos, findings)
}

fn select_operating_context(
    operating_model: Option<&SystemOperatingModel>,
    goal: Option<&str>,
) -> (
    Vec<OperatingModelLayer>,
    Vec<OperatingCapability>,
    Vec<String>,
) {
    let Some(operating_model) = operating_model else {
        return (Vec::new(), Vec::new(), Vec::new());
    };
    let goal_terms = goal_terms(goal);
    let mut scored_capabilities = operating_model
        .capabilities
        .iter()
        .cloned()
        .map(|capability| {
            let mut score = capability.repos.len() * 100 + capability.anchors.len() * 25;
            let haystack = [
                capability.id.as_str(),
                capability.name.as_str(),
                capability.layer.as_str(),
                capability.purpose.as_str(),
                &capability.repos.join(" "),
                &capability.anchors.join(" "),
            ]
            .join(" ")
            .to_ascii_lowercase();
            for term in &goal_terms {
                if haystack.contains(term) {
                    score += 1_000;
                }
            }
            if matches!(
                capability.id.as_str(),
                "capability:idd-spec-engine"
                    | "capability:fleet-handoff"
                    | "capability:agent-communication"
                    | "capability:env-vault-relay"
                    | "capability:prompt-front-door"
                    | "capability:vector-runtime"
                    | "capability:parser-runtime"
                    | "capability:rtk-ai-foundation"
                    | "capability:github-agent-run-upgrades"
            ) {
                score += 500;
            }
            (score, capability)
        })
        .collect::<Vec<_>>();
    scored_capabilities.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.id.cmp(&b.1.id)));
    let operating_capabilities = scored_capabilities
        .into_iter()
        .take(18)
        .map(|(_, capability)| capability)
        .collect::<Vec<_>>();
    let layer_ids = operating_capabilities
        .iter()
        .map(|capability| capability.layer.clone())
        .collect::<BTreeSet<_>>();
    let operating_layers = operating_model
        .layers
        .iter()
        .filter(|layer| layer_ids.contains(&layer.id))
        .cloned()
        .collect::<Vec<_>>();

    let findings = vec![format!(
        "operating context selected {} layers and {} capabilities from {} generated capabilities",
        operating_layers.len(),
        operating_capabilities.len(),
        operating_model.capabilities.len()
    )];
    (operating_layers, operating_capabilities, findings)
}

fn select_integration_work(
    integration_plan: Option<&IntegrationAutomationPlan>,
    goal: Option<&str>,
) -> (Vec<IntegrationWorkItem>, Vec<String>) {
    let Some(integration_plan) = integration_plan else {
        return (Vec::new(), Vec::new());
    };
    let goal_terms = goal_terms(goal);
    let mut scored = integration_plan
        .work_items
        .iter()
        .cloned()
        .map(|item| {
            let mut score = 10_000usize.saturating_sub(item.priority as usize);
            let haystack = [
                item.id.as_str(),
                item.title.as_str(),
                item.capability.as_str(),
                item.layer.as_str(),
                item.change_id.as_str(),
                item.implementation_boundary.as_str(),
                &item.owner_repos.join(" "),
                &item.anchors.join(" "),
            ]
            .join(" ")
            .to_ascii_lowercase();
            for term in &goal_terms {
                if haystack.contains(term) {
                    score += 1_000;
                }
            }
            (score, item)
        })
        .collect::<Vec<_>>();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.priority.cmp(&b.1.priority)));
    let mut work_items = scored
        .into_iter()
        .take(12)
        .map(|(_, item)| item)
        .collect::<Vec<_>>();
    work_items.sort_by(|a, b| a.priority.cmp(&b.priority).then(a.id.cmp(&b.id)));
    let findings = vec![format!(
        "integration context selected {} work items from {} generated work items",
        work_items.len(),
        integration_plan.work_items.len()
    )];
    (work_items, findings)
}

fn goal_terms(goal: Option<&str>) -> BTreeSet<String> {
    goal.unwrap_or_default()
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .filter(|term| term.len() >= 3)
        .collect()
}

fn render_graph_planning_context_markdown(context: &GraphPlanningContext) -> String {
    let mut out = String::new();
    out.push_str("# Graph Planning Context\n\n");
    if let Some(change) = &context.change {
        out.push_str(&format!("- Change: `{change}`\n"));
    }
    if let Some(goal) = &context.goal {
        out.push_str(&format!("- Goal: {goal}\n"));
    }
    out.push_str(&format!("- Workspace root: `{}`\n", context.workspace_root));
    out.push_str(&format!(
        "- Source graph: {} files, {} nodes, {} edges via `{}`\n",
        context.source_graph.files,
        context.source_graph.nodes,
        context.source_graph.edges,
        context.source_graph.provider
    ));
    out.push_str(&format!(
        "- Context package: {} files, {} tokens via `{}`\n\n",
        context.context_package.files,
        context.context_package.tokens,
        context.context_package.provider
    ));

    out.push_str("## Automation Order\n\n");
    for stage in &context.automation_stages {
        out.push_str(&format!(
            "- `{}`: {} ({})\n",
            stage.name,
            stage.purpose,
            stage.surfaces.join(", ")
        ));
    }

    out.push_str("\n## Integration Surfaces\n\n");
    for surface in &context.integration_surfaces {
        out.push_str(&format!(
            "- `{}` [{}]: {}. Capabilities: {}\n",
            surface.name,
            surface.kind,
            surface.default_scope,
            surface.capabilities.join(", ")
        ));
    }

    out.push_str("\n## Focus Components\n\n");
    out.push_str(
        "| Component | Kind | Files | Nodes | Edges | Evidence |\n|---|---|---:|---:|---:|---|\n",
    );
    for component in &context.focus_components {
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} |\n",
            component.name,
            component.kind,
            component.files,
            component.nodes,
            component.edges,
            component.evidence_paths.join(", ")
        ));
    }

    out.push_str("\n## System Roles\n\n");
    if context.system_roles.is_empty() {
        out.push_str("No system roles included.\n");
    } else {
        for role in &context.system_roles {
            out.push_str(&format!("- `{}`: {}\n", role.name, role.purpose));
        }
    }

    out.push_str("\n## System Repos\n\n");
    if context.system_repos.is_empty() {
        out.push_str("No system repos included.\n");
    } else {
        out.push_str("| Repo | Branch | Dirty | Roles | Architecture |\n|---|---|---|---|---|\n");
        for repo in &context.system_repos {
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} |\n",
                repo.name,
                repo.branch.as_deref().unwrap_or(""),
                repo.dirty,
                repo.roles.join(", "),
                peer_architecture_summary_cell(repo.local_architecture.as_ref())
            ));
        }
    }

    out.push_str("\n## Operating Layers\n\n");
    if context.operating_layers.is_empty() {
        out.push_str("No operating layers included.\n");
    } else {
        for layer in &context.operating_layers {
            out.push_str(&format!(
                "- `{}`: {} ({} capabilities, {} repos)\n",
                layer.name,
                layer.purpose,
                layer.capabilities.len(),
                layer.repos.len()
            ));
        }
    }

    out.push_str("\n## Operating Capabilities\n\n");
    if context.operating_capabilities.is_empty() {
        out.push_str("No operating capabilities included.\n");
    } else {
        out.push_str("| Capability | Layer | Status | Repos | Anchors |\n|---|---|---|---|---|\n");
        for capability in &context.operating_capabilities {
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} |\n",
                capability.name,
                capability.layer,
                capability.status,
                capability.repos.join(", "),
                capability.anchors.join(", ")
            ));
        }
    }

    out.push_str("\n## Integration Work\n\n");
    if context.integration_work_items.is_empty() {
        out.push_str("No integration work items included.\n");
    } else {
        out.push_str(
            "| Priority | Work Item | Change | Owners | Adopt First |\n|---:|---|---|---|---|\n",
        );
        for item in &context.integration_work_items {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | {} | {} |\n",
                item.priority,
                item.title,
                item.change_id,
                item.owner_repos.join(", "),
                item.adopt_first_inputs.join(", ")
            ));
        }
    }

    out.push_str("\n## Planning Guidance\n\n");
    for item in &context.guidance {
        out.push_str(&format!("- {item}\n"));
    }

    out.push_str("\n## Findings\n\n");
    if context.findings.is_empty() {
        out.push_str("No findings.\n");
    } else {
        for finding in &context.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }
    out
}

fn peer_architecture_summary_cell(summary: Option<&PeerArchitectureSummary>) -> String {
    let Some(summary) = summary else {
        return String::new();
    };
    let top_components = summary
        .top_components
        .iter()
        .take(3)
        .map(|component| component.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{} files, {} nodes, {} edges; {} tokens; surfaces {}; top: {}",
        summary.source_graph.files,
        summary.source_graph.nodes,
        summary.source_graph.edges,
        summary.context_package.tokens,
        summary.integration_surface_count,
        top_components
    )
}

fn slug(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn normalize_relative_path(path: &str) -> String {
    path.trim_start_matches("./").replace('\\', "/")
}

fn render_report_markdown(report: &KnowledgeReport) -> String {
    let mut out = String::new();
    out.push_str("# Knowledge Report\n\n");
    out.push_str(&format!(
        "- Workspace fingerprint: `{}`\n",
        report.workspace_fingerprint
    ));
    out.push_str(&format!(
        "- Indexed source files: {}\n",
        report.files_indexed
    ));
    out.push_str(&format!("- Graph nodes: {}\n", report.nodes));
    out.push_str(&format!("- Graph edges: {}\n", report.edges));
    out.push_str(&format!("- Resolved call edges: {}\n", report.call_edges));
    out.push_str(&format!(
        "- Functions with complexity: {}\n",
        report.functions_with_complexity
    ));
    out.push_str(&format!("- Packed files: {}\n", report.pack.total_files));
    out.push_str(&format!("- Packed tokens: {}\n", report.pack.total_tokens));
    out.push_str(&format!(
        "- Suspicious files: {}\n\n",
        report.pack.suspicious_files.len()
    ));

    out.push_str("## Hotspots\n\n");
    if report.hotspots.is_empty() {
        out.push_str("No hotspots crossed the reporting threshold.\n\n");
    } else {
        out.push_str("| Score | Node | File | Reasons |\n|---:|---|---|---|\n");
        for hotspot in &report.hotspots {
            out.push_str(&format!(
                "| {} | `{}` ({}) | `{}` | {} |\n",
                hotspot.score,
                hotspot.name,
                hotspot.node_id,
                hotspot.file.as_deref().unwrap_or(""),
                hotspot.reasons.join(", ")
            ));
        }
        out.push('\n');
    }

    out.push_str("## Top Files By Tokens\n\n");
    if report.pack.top_files_by_tokens.is_empty() {
        out.push_str("No token metrics available.\n\n");
    } else {
        out.push_str("| Tokens | File |\n|---:|---|\n");
        for (file, tokens) in &report.pack.top_files_by_tokens {
            out.push_str(&format!("| {tokens} | `{file}` |\n"));
        }
        out.push('\n');
    }

    out.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        out.push_str("No knowledge findings.\n");
    } else {
        for finding in &report.findings {
            out.push_str(&format!("- {finding}\n"));
        }
    }
    out
}

fn query_symbol(index: &KnowledgeIndex, symbol: &str) -> QueryResult {
    let needle = symbol.to_ascii_lowercase();
    let nodes = index
        .nodes
        .iter()
        .filter(|node| node.name.to_ascii_lowercase().contains(&needle))
        .cloned()
        .collect::<Vec<_>>();
    let node_ids = nodes.iter().map(|node| node.id).collect::<BTreeSet<_>>();
    let edges = index
        .edges
        .iter()
        .filter(|edge| node_ids.contains(&edge.source) || node_ids.contains(&edge.target))
        .cloned()
        .collect();
    QueryResult {
        title: format!("symbol `{symbol}`"),
        nodes,
        edges,
        notes: Vec::new(),
    }
}

fn query_file(index: &KnowledgeIndex, file: &str) -> QueryResult {
    let nodes = index
        .nodes
        .iter()
        .filter(|node| node.file.as_deref() == Some(file) || node.name == file)
        .cloned()
        .collect::<Vec<_>>();
    let node_ids = nodes.iter().map(|node| node.id).collect::<BTreeSet<_>>();
    let edges = index
        .edges
        .iter()
        .filter(|edge| node_ids.contains(&edge.source) || node_ids.contains(&edge.target))
        .cloned()
        .collect();
    QueryResult {
        title: format!("file `{file}`"),
        nodes,
        edges,
        notes: Vec::new(),
    }
}

fn query_impact(index: &KnowledgeIndex, node_id: u64) -> QueryResult {
    let mut impacted = BTreeSet::from([node_id]);
    let mut frontier = BTreeSet::from([node_id]);
    for _ in 0..2 {
        let mut next = BTreeSet::new();
        for edge in index.edges.iter().filter(|edge| is_impact_edge(edge)) {
            if frontier.contains(&edge.source) && impacted.insert(edge.target) {
                next.insert(edge.target);
            }
            if frontier.contains(&edge.target) && impacted.insert(edge.source) {
                next.insert(edge.source);
            }
        }
        frontier = next;
        if frontier.is_empty() {
            break;
        }
    }
    let nodes = index
        .nodes
        .iter()
        .filter(|node| impacted.contains(&node.id))
        .cloned()
        .collect::<Vec<_>>();
    let edges = index
        .edges
        .iter()
        .filter(|edge| {
            impacted.contains(&edge.source)
                && impacted.contains(&edge.target)
                && (is_impact_edge(edge) || edge.kind == "Contains")
        })
        .cloned()
        .collect();
    QueryResult {
        title: format!("impact `{node_id}`"),
        nodes,
        edges,
        notes: vec![
            "impact traverses semantic edges up to depth 2".to_string(),
            "containment edges are included only for context, not traversal".to_string(),
        ],
    }
}

fn is_impact_edge(edge: &KnowledgeEdge) -> bool {
    matches!(
        edge.kind.as_str(),
        "Calls" | "Invokes" | "Instantiates" | "Extends" | "Implements" | "Uses" | "References"
    )
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let content = serde_json::to_string_pretty(value).context("serialize JSON")?;
    write_text(path, &(content + "\n"))
}

fn write_text(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }
    fs::write(path, content).with_context(|| format!("write {}", path.display()))
}

fn display_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_rust_symbols_and_queries_without_rescan() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(
            tmp.path().join("src/lib.rs"),
            "use std::fmt;\npub struct Person;\nimpl Person { pub fn new() -> Self { Person } }\npub fn greet() { let _ = Person::new(); }\n",
        )
        .unwrap();

        let index = index_workspace(IndexOptions::new(tmp.path())).unwrap();
        assert!(index.nodes.iter().any(|node| node.name == "Person"));
        assert!(index.nodes.iter().any(|node| node.name == "greet"));
        assert!(!index.imports.is_empty());
        assert!(index.edges.iter().any(|edge| edge.kind == "Calls"));
        assert!(index.nodes.iter().any(|node| {
            node.name == "greet"
                && node
                    .properties
                    .get("cyclomatic_complexity")
                    .and_then(|value| value.as_u64())
                    == Some(1)
        }));

        let result = query_knowledge_index(&index, KnowledgeQuery::Symbol("greet".to_string()));
        assert_eq!(result.nodes.len(), 1);
        let greet_id = result.nodes[0].id;
        let impact = query_knowledge_index(&index, KnowledgeQuery::Impact(greet_id));
        assert!(impact.nodes.iter().any(|node| node.name == "new"));
        assert!(
            !impact
                .nodes
                .iter()
                .any(|node| node.name == "Person" && node.kind == "Class")
        );
    }

    #[test]
    fn index_output_is_deterministic_across_runs() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(
            tmp.path().join("src/lib.rs"),
            "pub struct Person;\npub fn greet() -> Person { Person }\n",
        )
        .unwrap();

        let first = index_workspace(IndexOptions::new(tmp.path())).unwrap();
        let second = index_workspace(IndexOptions::new(tmp.path())).unwrap();

        assert_eq!(
            serde_json::to_string_pretty(&first).unwrap(),
            serde_json::to_string_pretty(&second).unwrap()
        );
    }

    #[test]
    fn indexes_multiple_tree_sitter_languages_through_codegraph_dispatch() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(
            tmp.path().join("src/lib.rs"),
            "pub fn rust_greet(name: &str) -> String { name.to_string() }\n",
        )
        .unwrap();
        fs::write(
            tmp.path().join("src/app.ts"),
            "export function tsGreet(name: string): string { return name; }\n",
        )
        .unwrap();
        fs::write(
            tmp.path().join("src/script.py"),
            "def py_greet(name):\n    return name\n",
        )
        .unwrap();

        let index = index_workspace(IndexOptions::new(tmp.path())).unwrap();
        let languages = index
            .files
            .iter()
            .map(|file| file.language.as_str())
            .collect::<BTreeSet<_>>();

        assert!(languages.contains("rust"));
        assert!(languages.contains("typescript"));
        assert!(languages.contains("python"));
        assert!(index.failures.is_empty());
        assert!(index.nodes.iter().any(|node| node.name == "rust_greet"));
        assert!(index.nodes.iter().any(|node| node.name == "tsGreet"));
        assert!(index.nodes.iter().any(|node| node.name == "py_greet"));
    }

    #[test]
    fn packs_typescript_python_ignored_binary_and_secret_like_files() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join(".gitignore"), "ignored.ts\n").unwrap();
        fs::write(tmp.path().join("main.ts"), "export const x = 1;\n").unwrap();
        fs::write(tmp.path().join("script.py"), "print('ok')\n").unwrap();
        fs::write(
            tmp.path().join("ignored.ts"),
            "export const ignored = true;\n",
        )
        .unwrap();
        fs::write(tmp.path().join("blob.bin"), [0, 159, 146, 150]).unwrap();
        let secretish = ["API", "_KEY=sk", "-1234567890abcdef1234567890abcdef\n"].concat();
        fs::write(tmp.path().join("secret.txt"), secretish).unwrap();

        let out = tmp.path().join("pack.md");
        let mut options = PackWorkspaceOptions::new(tmp.path(), &out, PackStyle::Markdown);
        options.remove_empty_lines = true;
        options.truncate_base64 = true;
        options.top_files_length = Some(5);
        options.ignore_patterns.push("ignored.ts".to_string());
        let summary = pack_workspace(options).unwrap();

        let content = fs::read_to_string(out).unwrap();
        assert!(content.contains("main.ts"));
        assert!(content.contains("script.py"));
        assert!(!content.contains("export const ignored"));
        assert!(!summary.suspicious_files.is_empty());
        assert!(summary.total_tokens > 0);
    }

    #[test]
    fn pack_honors_repomix_project_config() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(
            tmp.path().join("repomix.config.json"),
            serde_json::json!({
                "ignore": {
                    "custom_ignore": ["configured-ignore.ts"]
                },
                "output": {
                    "remove_empty_lines": true,
                    "top_files_length": 3
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(tmp.path().join("kept.ts"), "export const kept = true;\n\n").unwrap();
        fs::write(
            tmp.path().join("configured-ignore.ts"),
            "export const ignored = true;\n",
        )
        .unwrap();

        let out = tmp.path().join("pack.md");
        let summary = pack_workspace(PackWorkspaceOptions::new(
            tmp.path(),
            &out,
            PackStyle::Markdown,
        ))
        .unwrap();
        let content = fs::read_to_string(out).unwrap();

        assert!(summary.top_files_by_tokens.len() <= 3);
        assert!(content.contains("kept.ts"));
        assert!(!content.contains("export const ignored"));
        assert!(!content.contains("\n\n\n"));
    }

    #[test]
    fn knowledge_defaults_skip_full_upstream_mirrors() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::create_dir_all(tmp.path().join("third_party/upstream/codegraph-rust/src")).unwrap();
        fs::write(tmp.path().join("src/lib.rs"), "pub fn local() {}\n").unwrap();
        fs::write(
            tmp.path()
                .join("third_party/upstream/codegraph-rust/src/lib.rs"),
            "pub fn upstream_mirror() {}\n",
        )
        .unwrap();

        let index = index_workspace(IndexOptions::new(tmp.path())).unwrap();

        assert!(index.files.iter().any(|file| file.path == "src/lib.rs"));
        assert!(
            !index
                .files
                .iter()
                .any(|file| file.path.contains("third_party/upstream"))
        );
    }

    #[test]
    fn renders_report_with_fingerprint() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/lib.rs"), "pub fn demo() {}\n").unwrap();

        let report =
            build_knowledge_report(ReportOptions::new(tmp.path(), ReportFormat::Markdown)).unwrap();

        assert!(report.contains("# Knowledge Report"));
        assert!(report.contains("Workspace fingerprint"));
    }

    #[test]
    fn architecture_graph_maps_tools_to_automation_stages() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("crates/knowledge/src")).unwrap();
        fs::create_dir_all(tmp.path().join("openspec/changes/demo/specs/demo")).unwrap();
        fs::create_dir_all(tmp.path().join("AI_MERGE")).unwrap();
        fs::create_dir_all(tmp.path().join("adr")).unwrap();
        fs::write(
            tmp.path().join("crates/knowledge/src/lib.rs"),
            "pub fn build_architecture() {}\n",
        )
        .unwrap();
        fs::write(
            tmp.path().join("openspec/changes/demo/proposal.md"),
            "# demo\n",
        )
        .unwrap();
        fs::write(tmp.path().join("AI_MERGE/01.md"), "# Evidence\n").unwrap();
        fs::write(tmp.path().join("adr/0001.md"), "# Decision\n").unwrap();

        let graph_json = build_architecture_graph(ArchitectureOptions::new(
            tmp.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        let graph: ArchitectureGraph = serde_json::from_str(&graph_json).unwrap();

        assert_eq!(graph.source_graph.provider, "codegraph-rust");
        assert_eq!(graph.context_package.provider, "repomix-rs");
        assert!(
            graph
                .automation_stages
                .iter()
                .any(|stage| stage.id == "stage:architecture-map")
        );
        assert!(
            graph
                .integration_surfaces
                .iter()
                .any(|surface| surface.id == "surface:codegraph-rust")
        );
        assert!(
            graph
                .integration_surfaces
                .iter()
                .any(|surface| surface.id == "surface:repomix-rs")
        );
        assert!(graph.components.iter().any(|component| {
            component.id == "crate:knowledge" && component.languages.contains(&"rust".to_string())
        }));

        let graph_markdown = build_architecture_graph(ArchitectureOptions::new(
            tmp.path(),
            ArchitectureFormat::Markdown,
        ))
        .unwrap();
        assert!(graph_markdown.contains("# Architecture Graph"));
        assert!(graph_markdown.contains("Architecture mapping"));
    }

    #[test]
    fn system_architecture_graph_maps_peer_repo_roles() {
        let system = tempfile::tempdir().unwrap();
        let rusty = system.path().join("rusty-idd");
        let weave = system.path().join("weave");
        let envctl = system.path().join("envctl");
        fs::create_dir_all(&rusty).unwrap();
        fs::create_dir_all(&weave).unwrap();
        fs::create_dir_all(&envctl).unwrap();
        init_git(&rusty);
        init_git(&weave);
        init_git(&envctl);
        fs::write(
            rusty.join("Cargo.toml"),
            "[package]\nname = \"rusty-idd\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            weave.join("Cargo.toml"),
            "[package]\nname = \"weave\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(envctl.join("Makefile"), "ci:\n\ttrue\n").unwrap();

        let graph_json = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        let graph: SystemArchitectureGraph = serde_json::from_str(&graph_json).unwrap();

        assert_eq!(graph.discovery_source, "filesystem git discovery");
        assert!(graph.repos.iter().any(|repo| repo.name == "rusty-idd"
            && repo.roles.contains(&"role:idd-control-plane".to_string())));
        assert!(graph.repos.iter().any(|repo| {
            repo.name == "weave"
                && repo
                    .roles
                    .contains(&"role:coordination-domain-surface".to_string())
        }));
        assert!(graph.repos.iter().any(|repo| repo.name == "envctl"
            && repo.roles.contains(&"role:toolchain-provider".to_string())));
        assert!(
            graph
                .edges
                .iter()
                .any(|edge| edge.kind == "maps_for_automation")
        );

        let graph_markdown = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Markdown,
        ))
        .unwrap();
        assert!(graph_markdown.contains("# System Architecture Graph"));
        assert!(graph_markdown.contains("Rusty IDD control plane"));
    }

    #[test]
    fn system_architecture_graph_ingests_peer_architecture_summary() {
        let system = tempfile::tempdir().unwrap();
        let rusty = system.path().join("rusty-idd");
        let weave = system.path().join("weave");
        fs::create_dir_all(rusty.join("src")).unwrap();
        fs::create_dir_all(weave.join("src")).unwrap();
        fs::create_dir_all(weave.join(".idd/knowledge")).unwrap();
        init_git(&rusty);
        init_git(&weave);
        fs::write(
            rusty.join("Cargo.toml"),
            "[package]\nname = \"rusty-idd\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            weave.join("Cargo.toml"),
            "[package]\nname = \"weave\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            weave.join("src/lib.rs"),
            "pub struct Handoff;\npub fn coordinate() -> Handoff { Handoff }\n",
        )
        .unwrap();
        let peer_architecture =
            build_architecture_graph(ArchitectureOptions::new(&weave, ArchitectureFormat::Json))
                .unwrap();
        fs::write(
            weave.join(".idd/knowledge/architecture.json"),
            peer_architecture,
        )
        .unwrap();

        let graph_json = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        let graph: SystemArchitectureGraph = serde_json::from_str(&graph_json).unwrap();
        let peer = graph
            .repos
            .iter()
            .find(|repo| repo.name == "weave")
            .expect("weave repo");
        let architecture = peer
            .local_architecture
            .as_ref()
            .expect("peer architecture summary");
        assert!(peer.has_local_architecture_graph);
        assert_eq!(architecture.source_graph.provider, "codegraph-rust");
        assert_eq!(architecture.context_package.provider, "repomix-rs");
        assert!(architecture.component_count > 0);
        assert!(!architecture.top_components.is_empty());
        assert!(
            graph
                .findings
                .iter()
                .any(|finding| { finding.contains("repos expose parsed architecture summaries") })
        );

        fs::write(
            weave.join(".idd/knowledge/architecture.json"),
            "{not valid json",
        )
        .unwrap();
        let graph_json = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        let graph: SystemArchitectureGraph = serde_json::from_str(&graph_json).unwrap();
        let peer = graph
            .repos
            .iter()
            .find(|repo| repo.name == "weave")
            .expect("weave repo");
        assert!(peer.has_local_architecture_graph);
        assert!(peer.local_architecture.is_none());
        assert!(graph.findings.iter().any(|finding| {
            finding.contains("repo weave exposes unreadable architecture graph")
        }));
    }

    #[test]
    fn graph_planning_context_consumes_repo_graph_without_system_graph() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("crates/knowledge/src")).unwrap();
        fs::create_dir_all(tmp.path().join(".idd/knowledge")).unwrap();
        fs::write(
            tmp.path().join("crates/knowledge/src/lib.rs"),
            "pub fn plan_context() {}\n",
        )
        .unwrap();

        let architecture = build_architecture_graph(ArchitectureOptions::new(
            tmp.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        fs::write(
            tmp.path().join(".idd/knowledge/architecture.json"),
            architecture,
        )
        .unwrap();

        let mut options = PlanContextOptions::new(tmp.path(), PlanContextFormat::Json);
        options.goal = Some("Use CodeGraph and repomix for planning context".to_string());
        options.change = Some("demo-graph-context".to_string());
        let context_json = build_graph_planning_context(options).unwrap();
        let context: GraphPlanningContext = serde_json::from_str(&context_json).unwrap();

        assert_eq!(context.change.as_deref(), Some("demo-graph-context"));
        assert_eq!(context.source_graph.provider, "codegraph-rust");
        assert_eq!(context.context_package.provider, "repomix-rs");
        assert!(
            context
                .focus_components
                .iter()
                .any(|component| component.id == "crate:knowledge")
        );
        assert!(
            context
                .findings
                .iter()
                .any(|finding| finding.contains("system architecture graph unavailable"))
        );

        let mut options = PlanContextOptions::new(tmp.path(), PlanContextFormat::Markdown);
        options.goal = Some("Use CodeGraph and repomix for planning context".to_string());
        let markdown = build_graph_planning_context(options).unwrap();
        assert!(markdown.contains("# Graph Planning Context"));
        assert!(markdown.contains("## Planning Guidance"));
    }

    #[test]
    fn system_operating_model_maps_agentic_company_capabilities() {
        let system = tempfile::tempdir().unwrap();
        let repo_names = [
            "rusty-idd",
            "handoff",
            "weave",
            "envctl",
            "prompt_hub",
            "ruvector",
            "lifeos",
            "teri",
            "lane",
            "network-control",
            "vault_hub",
            "yazelix",
        ];
        for name in repo_names {
            let repo = system.path().join(name);
            fs::create_dir_all(&repo).unwrap();
            init_git(&repo);
            fs::write(
                repo.join("Cargo.toml"),
                format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
            )
            .unwrap();
        }
        let rusty = system.path().join("rusty-idd");
        fs::create_dir_all(rusty.join(".idd/knowledge")).unwrap();
        let system_architecture = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/system-architecture.json"),
            system_architecture,
        )
        .unwrap();

        let model_json = build_system_operating_model(OperatingModelOptions::new(
            &rusty,
            PlanContextFormat::Json,
        ))
        .unwrap();
        let model: SystemOperatingModel = serde_json::from_str(&model_json).unwrap();

        let idd = model
            .capabilities
            .iter()
            .find(|capability| capability.id == "capability:idd-spec-engine")
            .expect("idd capability");
        assert!(idd.repos.contains(&"repo:rusty-idd".to_string()));
        assert!(idd.repos.contains(&"repo:handoff".to_string()));

        let communication = model
            .capabilities
            .iter()
            .find(|capability| capability.id == "capability:agent-communication")
            .expect("communication capability");
        assert!(communication.repos.contains(&"repo:weave".to_string()));

        let vault = model
            .capabilities
            .iter()
            .find(|capability| capability.id == "capability:env-vault-relay")
            .expect("vault relay capability");
        assert!(
            vault
                .anchors
                .iter()
                .any(|anchor| anchor.contains("COGNITUM"))
        );

        let simulation = model
            .capabilities
            .iter()
            .find(|capability| capability.id == "capability:digital-twin-simulation")
            .expect("simulation capability");
        assert!(simulation.repos.contains(&"repo:teri".to_string()));

        let interface = model
            .capabilities
            .iter()
            .find(|capability| capability.id == "capability:lua-ar-interface")
            .expect("lua interface capability");
        assert!(
            interface
                .anchors
                .iter()
                .any(|anchor| anchor.contains("Lua"))
        );

        let rtk = model
            .capabilities
            .iter()
            .find(|capability| capability.id == "capability:rtk-ai-foundation")
            .expect("rtk-ai capability");
        assert!(rtk.anchors.iter().any(|anchor| anchor.contains("ICM")));

        let beads = model
            .capabilities
            .iter()
            .find(|capability| capability.id == "capability:github-agent-run-upgrades")
            .expect("github agent-run capability");
        assert!(
            beads
                .anchors
                .iter()
                .any(|anchor| anchor.contains("beads-rs@d98da231"))
        );

        let markdown = build_system_operating_model(OperatingModelOptions::new(
            &rusty,
            PlanContextFormat::Markdown,
        ))
        .unwrap();
        assert!(markdown.contains("# System Operating Model"));
        assert!(markdown.contains("Digital twin simulation"));
        assert!(markdown.contains("Lua and AR interface automation"));
    }

    #[test]
    fn integration_automation_plan_orders_operating_capability_work() {
        let system = tempfile::tempdir().unwrap();
        let repo_names = [
            "rusty-idd",
            "handoff",
            "weave",
            "envctl",
            "prompt_hub",
            "grit",
            "yazelix",
        ];
        for name in repo_names {
            let repo = system.path().join(name);
            fs::create_dir_all(&repo).unwrap();
            init_git(&repo);
            fs::write(
                repo.join("Cargo.toml"),
                format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n"),
            )
            .unwrap();
        }
        let rusty = system.path().join("rusty-idd");
        fs::create_dir_all(rusty.join(".idd/knowledge")).unwrap();
        let system_architecture = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/system-architecture.json"),
            system_architecture,
        )
        .unwrap();
        let operating_model = build_system_operating_model(OperatingModelOptions::new(
            &rusty,
            PlanContextFormat::Json,
        ))
        .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/operating-model.json"),
            operating_model,
        )
        .unwrap();

        let plan_json = build_integration_automation_plan(IntegrationPlanOptions::new(
            &rusty,
            PlanContextFormat::Json,
        ))
        .unwrap();
        let plan: IntegrationAutomationPlan = serde_json::from_str(&plan_json).unwrap();
        assert!(!plan.work_items.is_empty());
        assert_eq!(plan.work_items[0].capability, "capability:idd-spec-engine");
        assert!(plan.work_items.iter().any(|item| {
            item.capability == "capability:github-agent-run-upgrades"
                && item
                    .adopt_first_inputs
                    .iter()
                    .any(|anchor| anchor.contains("beads-rs"))
        }));
        assert!(
            plan.gates
                .iter()
                .any(|gate| gate == "cargo audit --deny warnings")
        );

        let markdown = build_integration_automation_plan(IntegrationPlanOptions::new(
            &rusty,
            PlanContextFormat::Markdown,
        ))
        .unwrap();
        assert!(markdown.contains("# Integration Automation Plan"));
        assert!(markdown.contains("Integrate IDD and spec engine"));
    }

    #[test]
    fn integration_status_reports_queue_state_from_openspec_changes() {
        let tmp = tempfile::tempdir().unwrap();
        let workspace = tmp.path();
        fs::create_dir_all(workspace.join(".idd/knowledge")).unwrap();
        fs::create_dir_all(
            workspace.join("openspec/changes/integrate-fleet-handoff/specs/fleet-handoff"),
        )
        .unwrap();
        fs::create_dir_all(
            workspace
                .join("openspec/changes/integrate-agent-communication/specs/agent-communication"),
        )
        .unwrap();
        fs::write(
            workspace.join("openspec/changes/integrate-fleet-handoff/proposal.md"),
            "# integrate-fleet-handoff\n",
        )
        .unwrap();
        fs::write(
            workspace.join("openspec/changes/integrate-fleet-handoff/design.md"),
            "# design\n",
        )
        .unwrap();
        fs::write(
            workspace.join("openspec/changes/integrate-fleet-handoff/tasks.md"),
            "- [ ] run diagnostics\n",
        )
        .unwrap();
        fs::write(
            workspace.join("openspec/changes/integrate-fleet-handoff/specs/fleet-handoff/spec.md"),
            "## ADDED Requirements\n",
        )
        .unwrap();
        fs::write(
            workspace.join("openspec/changes/integrate-agent-communication/proposal.md"),
            "# integrate-agent-communication\n",
        )
        .unwrap();
        fs::write(
            workspace.join("openspec/changes/integrate-agent-communication/design.md"),
            "# design\n",
        )
        .unwrap();
        fs::write(
            workspace.join("openspec/changes/integrate-agent-communication/tasks.md"),
            "- [x] run diagnostics\n",
        )
        .unwrap();
        fs::write(
            workspace.join(
                "openspec/changes/integrate-agent-communication/specs/agent-communication/spec.md",
            ),
            "## ADDED Requirements\n",
        )
        .unwrap();

        let plan = IntegrationAutomationPlan {
            schema_version: 1,
            workspace_root: workspace.display().to_string(),
            system_root: tmp.path().display().to_string(),
            source_model: ".idd/knowledge/operating-model.json".to_string(),
            work_items: vec![
                test_work_item(
                    "integrate-idd-spec-engine",
                    "capability:idd-spec-engine",
                    10,
                ),
                test_work_item("integrate-fleet-handoff", "capability:fleet-handoff", 20),
                test_work_item(
                    "integrate-agent-communication",
                    "capability:agent-communication",
                    30,
                ),
            ],
            gates: vec!["just ci".to_string()],
            findings: Vec::new(),
        };
        fs::write(
            workspace.join(".idd/knowledge/integration-plan.json"),
            serde_json::to_string_pretty(&plan).unwrap(),
        )
        .unwrap();

        let report_json = build_integration_status_report(IntegrationStatusOptions::new(
            workspace,
            PlanContextFormat::Json,
        ))
        .unwrap();
        let report: IntegrationStatusReport = serde_json::from_str(&report_json).unwrap();
        assert_eq!(
            report.next_change_id.as_deref(),
            Some("integrate-idd-spec-engine")
        );
        assert_eq!(report.counts.planned, 1);
        assert_eq!(report.counts.scaffolded, 1);
        assert_eq!(report.counts.ready_to_archive, 1);
        assert!(
            report
                .work_items
                .iter()
                .any(|item| item.change_id == "integrate-fleet-handoff"
                    && item.status == "scaffolded"
                    && item.unchecked_tasks == 1)
        );

        let markdown = build_integration_status_report(IntegrationStatusOptions::new(
            workspace,
            PlanContextFormat::Markdown,
        ))
        .unwrap();
        assert!(markdown.contains("# Integration Status Queue"));
        assert!(markdown.contains("integrate-idd-spec-engine"));
        assert!(markdown.contains("ready-to-archive"));
    }

    #[test]
    fn integration_owner_surfaces_join_work_item_to_system_repos() {
        let system = tempfile::tempdir().unwrap();
        let rusty = system.path().join("rusty-idd");
        let handoff = system.path().join("handoff");
        fs::create_dir_all(rusty.join(".idd/knowledge")).unwrap();
        fs::create_dir_all(rusty.join("src")).unwrap();
        fs::create_dir_all(handoff.join(".idd/knowledge")).unwrap();
        fs::create_dir_all(handoff.join(".handoff")).unwrap();
        fs::create_dir_all(handoff.join("src")).unwrap();
        init_git(&rusty);
        init_git(&handoff);
        fs::write(
            rusty.join("Cargo.toml"),
            "[package]\nname = \"rusty-idd\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(rusty.join("src/lib.rs"), "pub fn idd() {}\n").unwrap();
        fs::write(
            handoff.join("Cargo.toml"),
            "[package]\nname = \"handoff\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            handoff.join("src/lib.rs"),
            "pub struct FleetHandoff;\npub fn sync() -> FleetHandoff { FleetHandoff }\n",
        )
        .unwrap();
        let handoff_architecture =
            build_architecture_graph(ArchitectureOptions::new(&handoff, ArchitectureFormat::Json))
                .unwrap();
        fs::write(
            handoff.join(".idd/knowledge/architecture.json"),
            handoff_architecture,
        )
        .unwrap();
        let system_architecture = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/system-architecture.json"),
            system_architecture,
        )
        .unwrap();

        let plan = IntegrationAutomationPlan {
            schema_version: 1,
            workspace_root: rusty.display().to_string(),
            system_root: system.path().display().to_string(),
            source_model: ".idd/knowledge/operating-model.json".to_string(),
            work_items: vec![
                IntegrationWorkItem {
                    owner_repos: vec![
                        "repo:handoff".to_string(),
                        "repo:rusty-idd".to_string(),
                        "repo:missing".to_string(),
                    ],
                    ..test_work_item("integrate-fleet-handoff", "capability:fleet-handoff", 20)
                },
                IntegrationWorkItem {
                    owner_repos: vec!["repo:handoff".to_string()],
                    ..test_work_item(
                        "integrate-agent-communication",
                        "capability:agent-communication",
                        30,
                    )
                },
                IntegrationWorkItem {
                    owner_repos: vec!["repo:handoff".to_string()],
                    ..test_work_item(
                        "integrate-env-vault-relay",
                        "capability:env-vault-relay",
                        40,
                    )
                },
                IntegrationWorkItem {
                    owner_repos: vec!["repo:handoff".to_string()],
                    anchors: vec![
                        "github.com/f/prompts.chat".to_string(),
                        "github.com/f/ai-prompt".to_string(),
                    ],
                    adopt_first_inputs: vec![
                        "github.com/f/prompts.chat".to_string(),
                        "github.com/f/ai-prompt".to_string(),
                    ],
                    implementation_boundary:
                        "Adopt upstream repo surface first, run native diagnostics, then add thin Rusty IDD mapping"
                            .to_string(),
                    ..test_work_item(
                        "integrate-prompt-front-door",
                        "capability:prompt-front-door",
                        50,
                    )
                },
            ],
            gates: vec!["just ci".to_string()],
            findings: Vec::new(),
        };
        fs::write(
            rusty.join(".idd/knowledge/integration-plan.json"),
            serde_json::to_string_pretty(&plan).unwrap(),
        )
        .unwrap();

        let mut options = IntegrationOwnersOptions::new(&rusty, PlanContextFormat::Json);
        options.change = Some("integrate-fleet-handoff".to_string());
        let report_json = build_integration_owner_surfaces(options).unwrap();
        let report: IntegrationOwnersReport = serde_json::from_str(&report_json).unwrap();
        assert_eq!(report.work_item.change_id, "integrate-fleet-handoff");
        assert_eq!(report.owner_surfaces.len(), 3);
        assert_eq!(report.missing_owner_repos, vec!["repo:missing"]);
        let handoff_surface = report
            .owner_surfaces
            .iter()
            .find(|surface| surface.owner_repo == "repo:handoff")
            .expect("handoff owner surface");
        assert!(handoff_surface.repo_found);
        assert!(handoff_surface.has_local_architecture_graph);
        assert!(
            handoff_surface
                .roles
                .contains(&"role:fleet-handoff".to_string())
        );
        assert!(
            handoff_surface
                .native_diagnostic_commands
                .iter()
                .any(|command| command.contains("cargo test --workspace"))
        );

        let mut markdown_options =
            IntegrationOwnersOptions::new(&rusty, PlanContextFormat::Markdown);
        markdown_options.capability = Some("capability:fleet-handoff".to_string());
        let markdown = build_integration_owner_surfaces(markdown_options).unwrap();
        assert!(markdown.contains("# Integration Owner Surfaces"));
        assert!(markdown.contains("repo:handoff"));
        assert!(markdown.contains("repo:missing"));

        fs::create_dir_all(
            rusty.join("openspec/changes/archive/integrate-fleet-handoff/specs/fleet-handoff"),
        )
        .unwrap();
        fs::create_dir_all(
            rusty.join("openspec/changes/integrate-agent-communication/specs/agent-communication"),
        )
        .unwrap();
        fs::write(
            rusty.join("openspec/changes/integrate-agent-communication/proposal.md"),
            "# integrate-agent-communication\n",
        )
        .unwrap();
        fs::write(
            rusty.join("openspec/changes/integrate-agent-communication/design.md"),
            "# design\n",
        )
        .unwrap();
        fs::write(
            rusty.join("openspec/changes/integrate-agent-communication/tasks.md"),
            "- [ ] record diagnostics\n",
        )
        .unwrap();
        fs::write(
            rusty.join(
                "openspec/changes/integrate-agent-communication/specs/agent-communication/spec.md",
            ),
            "## ADDED Requirements\n",
        )
        .unwrap();
        let mut next_options = IntegrationOwnersOptions::new(&rusty, PlanContextFormat::Json);
        next_options.next = true;
        let next_report_json = build_integration_owner_surfaces(next_options).unwrap();
        let next_report: IntegrationOwnersReport = serde_json::from_str(&next_report_json).unwrap();
        assert_eq!(
            next_report.work_item.change_id,
            "integrate-agent-communication"
        );
        assert!(next_report.selector.next);

        let mut planned_options = IntegrationOwnersOptions::new(&rusty, PlanContextFormat::Json);
        planned_options.next_planned = true;
        let planned_report_json = build_integration_owner_surfaces(planned_options).unwrap();
        let planned_report: IntegrationOwnersReport =
            serde_json::from_str(&planned_report_json).unwrap();
        assert_eq!(
            planned_report.work_item.change_id,
            "integrate-env-vault-relay"
        );
        assert!(planned_report.selector.next_planned);

        let mut readiness_options =
            IntegrationReadinessOptions::new(&rusty, PlanContextFormat::Json);
        readiness_options.next_planned = true;
        let readiness_json = build_integration_readiness_report(readiness_options).unwrap();
        let readiness: IntegrationReadinessReport = serde_json::from_str(&readiness_json).unwrap();
        assert_eq!(readiness.work_item.change_id, "integrate-env-vault-relay");
        assert!(
            readiness
                .tool_requirements
                .iter()
                .any(|tool| tool.id == "cargo" && tool.default_path)
        );
        assert!(
            readiness
                .feature_gates
                .iter()
                .any(|gate| gate.contains("read-only"))
        );
        assert!(
            readiness
                .native_diagnostics
                .iter()
                .any(|diagnostic| diagnostic.command.contains("cargo test"))
        );

        let mut prompt_readiness_options =
            IntegrationReadinessOptions::new(&rusty, PlanContextFormat::Json);
        prompt_readiness_options.change = Some("integrate-prompt-front-door".to_string());
        let prompt_readiness_json =
            build_integration_readiness_report(prompt_readiness_options).unwrap();
        let prompt_readiness: IntegrationReadinessReport =
            serde_json::from_str(&prompt_readiness_json).unwrap();
        assert_eq!(
            prompt_readiness.work_item.change_id,
            "integrate-prompt-front-door"
        );
        assert!(
            prompt_readiness
                .upstream_inputs
                .iter()
                .any(|upstream| upstream.source == "github.com/f/prompts.chat"
                    && upstream.required_tool_ids.contains(&"postgres".to_string()))
        );
        assert!(prompt_readiness.upstream_inputs.iter().any(|upstream| {
            upstream.source == "github.com/f/ai-prompt"
                && upstream
                    .required_tool_ids
                    .contains(&"wordpress".to_string())
        }));
        assert!(
            prompt_readiness
                .native_diagnostics
                .iter()
                .any(|diagnostic| diagnostic.command.contains(
                    "DATABASE_URL=\"postgresql://test:test@localhost:5432/test\" npm ci"
                ) && diagnostic.mutates_repo)
        );
        assert!(
            prompt_readiness
                .tool_requirements
                .iter()
                .any(|tool| tool.id == "node" && tool.required_by.len() >= 2)
        );

        let mut readiness_markdown_options =
            IntegrationReadinessOptions::new(&rusty, PlanContextFormat::Markdown);
        readiness_markdown_options.change = Some("integrate-prompt-front-door".to_string());
        let readiness_markdown =
            build_integration_readiness_report(readiness_markdown_options).unwrap();
        assert!(readiness_markdown.contains("# Integration Readiness"));
        assert!(readiness_markdown.contains("## Upstream Inputs"));
        assert!(readiness_markdown.contains("github.com/f/prompts.chat"));
        assert!(readiness_markdown.contains("Tool Requirements"));
    }

    #[test]
    fn graph_planning_context_preserves_peer_architecture_summary() {
        let system = tempfile::tempdir().unwrap();
        let rusty = system.path().join("rusty-idd");
        let weave = system.path().join("weave");
        fs::create_dir_all(rusty.join(".idd/knowledge")).unwrap();
        fs::create_dir_all(rusty.join("src")).unwrap();
        fs::create_dir_all(weave.join(".idd/knowledge")).unwrap();
        fs::create_dir_all(weave.join("src")).unwrap();
        init_git(&rusty);
        init_git(&weave);
        fs::write(
            rusty.join("Cargo.toml"),
            "[package]\nname = \"rusty-idd\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(rusty.join("src/lib.rs"), "pub fn plan_context() {}\n").unwrap();
        fs::write(
            weave.join("Cargo.toml"),
            "[package]\nname = \"weave\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(
            weave.join("src/lib.rs"),
            "pub struct Handoff;\npub fn coordinate() -> Handoff { Handoff }\n",
        )
        .unwrap();
        let rusty_architecture =
            build_architecture_graph(ArchitectureOptions::new(&rusty, ArchitectureFormat::Json))
                .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/architecture.json"),
            rusty_architecture,
        )
        .unwrap();
        let weave_architecture =
            build_architecture_graph(ArchitectureOptions::new(&weave, ArchitectureFormat::Json))
                .unwrap();
        fs::write(
            weave.join(".idd/knowledge/architecture.json"),
            weave_architecture,
        )
        .unwrap();
        let system_architecture = build_system_architecture_graph(SystemArchitectureOptions::new(
            &rusty,
            system.path(),
            ArchitectureFormat::Json,
        ))
        .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/system-architecture.json"),
            system_architecture,
        )
        .unwrap();
        let operating_model = build_system_operating_model(OperatingModelOptions::new(
            &rusty,
            PlanContextFormat::Json,
        ))
        .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/operating-model.json"),
            operating_model,
        )
        .unwrap();
        let integration_plan = build_integration_automation_plan(IntegrationPlanOptions::new(
            &rusty,
            PlanContextFormat::Json,
        ))
        .unwrap();
        fs::write(
            rusty.join(".idd/knowledge/integration-plan.json"),
            integration_plan,
        )
        .unwrap();

        let mut options = PlanContextOptions::new(&rusty, PlanContextFormat::Json);
        options.goal = Some("weave handoff architecture integration".to_string());
        let context_json = build_graph_planning_context(options).unwrap();
        let context: GraphPlanningContext = serde_json::from_str(&context_json).unwrap();
        let peer = context
            .system_repos
            .iter()
            .find(|repo| repo.name == "weave")
            .expect("weave repo");
        assert!(peer.local_architecture.is_some());
        assert!(
            context
                .operating_capabilities
                .iter()
                .any(|capability| capability.id == "capability:fleet-handoff")
        );
        assert!(
            context
                .integration_work_items
                .iter()
                .any(|item| item.capability == "capability:fleet-handoff")
        );

        let mut options = PlanContextOptions::new(&rusty, PlanContextFormat::Markdown);
        options.goal = Some("weave handoff architecture integration".to_string());
        let markdown = build_graph_planning_context(options).unwrap();
        assert!(markdown.contains("Architecture"));
        assert!(markdown.contains("top:"));
        assert!(markdown.contains("## Operating Capabilities"));
        assert!(markdown.contains("## Integration Work"));
    }

    fn init_git(path: &Path) {
        let out = Command::new("git")
            .arg("init")
            .current_dir(path)
            .output()
            .expect("run git init");
        assert!(
            out.status.success(),
            "git init should succeed: stdout={} stderr={}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }

    fn test_work_item(change_id: &str, capability: &str, priority: u32) -> IntegrationWorkItem {
        IntegrationWorkItem {
            id: format!("work:{change_id}"),
            title: format!("Integrate {change_id}"),
            capability: capability.to_string(),
            layer: "layer:test".to_string(),
            priority,
            status: "partial".to_string(),
            change_id: change_id.to_string(),
            owner_repos: vec!["repo:rusty-idd".to_string()],
            anchors: Vec::new(),
            adopt_first_inputs: Vec::new(),
            implementation_boundary: "test boundary".to_string(),
            validation: vec!["just ci".to_string()],
            rollback: vec!["revert".to_string()],
        }
    }
}
