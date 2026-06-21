use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
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
    pack_options
        .ignore_patterns
        .push("AI_MERGE/validation_report.md".to_string());
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
        if let Some(file) = source_file {
            if let Some(ids) = self.by_file_name.get(&(file.clone(), basename.clone())) {
                if ids.len() == 1 {
                    return ids.first().copied();
                }
            }
        }

        if let Some(ids) = self.by_name.get(&basename) {
            if ids.len() == 1 {
                return ids.first().copied();
            }
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
    pack_options
        .ignore_patterns
        .push("AI_MERGE/validation_report.md".to_string());
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
        if let Some(component_id) = node_component.get(&edge.source) {
            if let Some(component) = components.get_mut(component_id) {
                component.edges.insert(edge.id);
            }
        }
        if let Some(component_id) = node_component.get(&edge.target) {
            if let Some(component) = components.get_mut(component_id) {
                component.edges.insert(edge.id);
            }
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
                    "adr/0005-full-feature-upstream-knowledge-integration.md",
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
                    "adr/0005-full-feature-upstream-knowledge-integration.md",
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
        assert!(index.nodes.iter().any(|node| node.name == "greet"
            && node
                .properties
                .get("cyclomatic_complexity")
                .and_then(|value| value.as_u64())
                == Some(1)));

        let result = query_knowledge_index(&index, KnowledgeQuery::Symbol("greet".to_string()));
        assert_eq!(result.nodes.len(), 1);
        let greet_id = result.nodes[0].id;
        let impact = query_knowledge_index(&index, KnowledgeQuery::Impact(greet_id));
        assert!(impact.nodes.iter().any(|node| node.name == "new"));
        assert!(!impact
            .nodes
            .iter()
            .any(|node| node.name == "Person" && node.kind == "Class"));
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
        assert!(!index
            .files
            .iter()
            .any(|file| file.path.contains("third_party/upstream")));
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
        assert!(graph
            .automation_stages
            .iter()
            .any(|stage| stage.id == "stage:architecture-map"));
        assert!(graph
            .integration_surfaces
            .iter()
            .any(|surface| surface.id == "surface:codegraph-rust"));
        assert!(graph
            .integration_surfaces
            .iter()
            .any(|surface| surface.id == "surface:repomix-rs"));
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
}
