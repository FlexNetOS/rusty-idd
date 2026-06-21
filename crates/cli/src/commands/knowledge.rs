use std::fs;
use std::path::PathBuf;

use clap::{Args, Subcommand};
use rusty_idd_knowledge::{
    build_architecture_graph, build_graph_planning_context, build_knowledge_report,
    build_system_architecture_graph, index_workspace, load_index, pack_workspace,
    query_knowledge_index, refresh_workspace, ArchitectureFormat, ArchitectureOptions,
    IndexOptions, KnowledgeQuery, PackStyle, PackWorkspaceOptions, PlanContextFormat,
    PlanContextOptions, ReportFormat, ReportOptions, SystemArchitectureOptions,
};

#[derive(Subcommand)]
pub enum KnowledgeCommand {
    /// Build a compact graph/symbol index.
    Index(IndexArgs),
    /// Create a bounded AI context bundle.
    Pack(PackArgs),
    /// Combine inventory, graph, pack metrics, hotspots, and findings.
    Report(ReportArgs),
    /// Generate the system architecture graph from CodeGraph and repomix surfaces.
    Architecture(ArchitectureArgs),
    /// Generate a cross-repo system graph from a parent meta workspace.
    SystemArchitecture(SystemArchitectureArgs),
    /// Generate a graph-backed planning packet for OpenSpec work.
    PlanContext(PlanContextArgs),
    /// Answer local graph questions from an existing index.
    Query(QueryArgs),
    /// Regenerate .idd/knowledge/index.json and report.md.
    Refresh(RefreshArgs),
}

#[derive(Args)]
pub struct IndexArgs {
    #[arg(long)]
    pub workspace: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

#[derive(Args)]
pub struct PackArgs {
    #[arg(long)]
    pub workspace: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
    #[arg(long, default_value = "markdown")]
    pub style: String,
    #[arg(long, default_value_t = 120_000)]
    pub max_tokens: usize,
    #[arg(long)]
    pub compress: bool,
    #[arg(long)]
    pub remove_comments: bool,
    #[arg(long)]
    pub remove_empty_lines: bool,
    #[arg(long)]
    pub line_numbers: bool,
    #[arg(long)]
    pub truncate_base64: bool,
    #[arg(long)]
    pub include_empty_directories: bool,
    #[arg(long)]
    pub top_files_length: Option<usize>,
    #[arg(long)]
    pub split_output: Option<u64>,
    #[arg(long)]
    pub header_text: Option<String>,
    #[arg(long)]
    pub instruction_file: Option<PathBuf>,
    #[arg(long)]
    pub git_diff: bool,
    #[arg(long)]
    pub git_log: bool,
    #[arg(long = "include")]
    pub include_patterns: Vec<String>,
    #[arg(long = "ignore")]
    pub ignore_patterns: Vec<String>,
}

#[derive(Args)]
pub struct ReportArgs {
    #[arg(long)]
    pub workspace: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

#[derive(Args)]
pub struct ArchitectureArgs {
    #[arg(long)]
    pub workspace: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

#[derive(Args)]
pub struct SystemArchitectureArgs {
    #[arg(long)]
    pub workspace: PathBuf,
    #[arg(long)]
    pub system_root: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

#[derive(Args)]
pub struct PlanContextArgs {
    #[arg(long)]
    pub workspace: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
    #[arg(long)]
    pub change: Option<String>,
    #[arg(long)]
    pub goal: Option<String>,
    #[arg(long)]
    pub goal_file: Option<PathBuf>,
    #[arg(long)]
    pub architecture: Option<PathBuf>,
    #[arg(long)]
    pub system_architecture: Option<PathBuf>,
}

#[derive(Args)]
pub struct QueryArgs {
    #[arg(long)]
    pub index: PathBuf,
    #[arg(long)]
    pub symbol: Option<String>,
    #[arg(long)]
    pub file: Option<String>,
    #[arg(long)]
    pub impact: Option<u64>,
}

#[derive(Args)]
pub struct RefreshArgs {
    #[arg(long)]
    pub workspace: PathBuf,
}

pub fn run(command: KnowledgeCommand) -> i32 {
    match try_run(command) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("rusty-idd knowledge: {error:#}");
            1
        }
    }
}

fn try_run(command: KnowledgeCommand) -> anyhow::Result<()> {
    match command {
        KnowledgeCommand::Index(args) => {
            let index = index_workspace(IndexOptions::new(args.workspace))?;
            write_json(&args.out, &index)?;
            println!("wrote knowledge index to {}", args.out.display());
        }
        KnowledgeCommand::Pack(args) => {
            let style = PackStyle::parse(&args.style)?;
            let mut options = PackWorkspaceOptions::new(args.workspace, &args.out, style);
            options.max_tokens = args.max_tokens;
            options.compress = args.compress;
            options.remove_comments = args.remove_comments;
            options.remove_empty_lines = args.remove_empty_lines;
            options.show_line_numbers = args.line_numbers;
            options.truncate_base64 = args.truncate_base64;
            options.include_empty_directories = args.include_empty_directories;
            options.top_files_length = args.top_files_length;
            options.split_output = args.split_output;
            options.header_text = args.header_text;
            options.instruction_file_path = args.instruction_file;
            options.include_diffs = args.git_diff;
            options.include_logs = args.git_log;
            options.include_patterns = args.include_patterns;
            options.ignore_patterns = args.ignore_patterns;
            let summary = pack_workspace(options)?;
            println!(
                "wrote pack to {} ({} files, {} tokens)",
                args.out.display(),
                summary.total_files,
                summary.total_tokens
            );
        }
        KnowledgeCommand::Report(args) => {
            let format = if args
                .out
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
            {
                ReportFormat::Json
            } else {
                ReportFormat::Markdown
            };
            let report = build_knowledge_report(ReportOptions::new(args.workspace, format))?;
            write_text(&args.out, &report)?;
            println!("wrote knowledge report to {}", args.out.display());
        }
        KnowledgeCommand::Architecture(args) => {
            let format = if args
                .out
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
            {
                ArchitectureFormat::Json
            } else {
                ArchitectureFormat::Markdown
            };
            let graph = build_architecture_graph(ArchitectureOptions::new(args.workspace, format))?;
            if matches!(format, ArchitectureFormat::Json) {
                write_text(&args.out, &(graph + "\n"))?;
            } else {
                write_text(&args.out, &graph)?;
            }
            println!("wrote architecture graph to {}", args.out.display());
        }
        KnowledgeCommand::SystemArchitecture(args) => {
            let format = if args
                .out
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
            {
                ArchitectureFormat::Json
            } else {
                ArchitectureFormat::Markdown
            };
            let graph = build_system_architecture_graph(SystemArchitectureOptions::new(
                args.workspace,
                args.system_root,
                format,
            ))?;
            if matches!(format, ArchitectureFormat::Json) {
                write_text(&args.out, &(graph + "\n"))?;
            } else {
                write_text(&args.out, &graph)?;
            }
            println!("wrote system architecture graph to {}", args.out.display());
        }
        KnowledgeCommand::PlanContext(args) => {
            let format = if args
                .out
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
            {
                PlanContextFormat::Json
            } else {
                PlanContextFormat::Markdown
            };
            let goal = selected_goal(args.goal, args.goal_file)?;
            let mut options = PlanContextOptions::new(args.workspace, format);
            options.goal = goal;
            options.change = args.change;
            options.architecture_path = args.architecture;
            options.system_architecture_path = args.system_architecture;
            let context = build_graph_planning_context(options)?;
            if matches!(format, PlanContextFormat::Json) {
                write_text(&args.out, &(context + "\n"))?;
            } else {
                write_text(&args.out, &context)?;
            }
            println!("wrote graph planning context to {}", args.out.display());
        }
        KnowledgeCommand::Query(args) => {
            let index = load_index(&args.index)?;
            let query = selected_query(&args)?;
            let result = query_knowledge_index(&index, query);
            print_query_result(&result);
        }
        KnowledgeCommand::Refresh(args) => {
            let artifacts = refresh_workspace(args.workspace)?;
            println!(
                "refreshed knowledge artifacts: {}, {}, {}, {}",
                artifacts.index.display(),
                artifacts.report.display(),
                artifacts.architecture_json.display(),
                artifacts.architecture_markdown.display()
            );
        }
    }
    Ok(())
}

fn selected_goal(
    goal: Option<String>,
    goal_file: Option<PathBuf>,
) -> anyhow::Result<Option<String>> {
    match (goal, goal_file) {
        (Some(_), Some(_)) => anyhow::bail!("use only one of --goal or --goal-file"),
        (Some(goal), None) => Ok(Some(goal)),
        (None, Some(path)) => fs::read_to_string(&path)
            .map(|content| Some(content.trim().to_string()))
            .map_err(anyhow::Error::from),
        (None, None) => Ok(None),
    }
}

fn selected_query(args: &QueryArgs) -> anyhow::Result<KnowledgeQuery> {
    let selected = [
        args.symbol
            .as_ref()
            .map(|value| KnowledgeQuery::Symbol(value.clone())),
        args.file
            .as_ref()
            .map(|value| KnowledgeQuery::File(value.clone())),
        args.impact.map(KnowledgeQuery::Impact),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    match selected.as_slice() {
        [query] => Ok(query.clone()),
        [] => anyhow::bail!("select one query flag: --symbol, --file, or --impact"),
        _ => anyhow::bail!("select only one query flag: --symbol, --file, or --impact"),
    }
}

fn print_query_result(result: &rusty_idd_knowledge::QueryResult) {
    println!("# Knowledge Query: {}", result.title);
    for note in &result.notes {
        println!("- {note}");
    }
    println!("\n## Nodes");
    if result.nodes.is_empty() {
        println!("No matching nodes.");
    } else {
        for node in &result.nodes {
            let file = node.file.as_deref().unwrap_or("");
            let mut details = Vec::new();
            if let Some(complexity) = node
                .properties
                .get("cyclomatic_complexity")
                .and_then(|value| value.as_u64())
            {
                details.push(format!("complexity={complexity}"));
            }
            if !node.unresolved_calls.is_empty() {
                details.push(format!("unresolved_calls={}", node.unresolved_calls.len()));
            }
            let suffix = if details.is_empty() {
                String::new()
            } else {
                format!(" {}", details.join(" "))
            };
            println!(
                "- {} `{}` id={} file=`{}`{}",
                node.kind, node.name, node.id, file, suffix
            );
        }
    }
    println!("\n## Edges");
    if result.edges.is_empty() {
        println!("No matching edges.");
    } else {
        for edge in &result.edges {
            let target = edge
                .properties
                .get("target")
                .and_then(|value| value.as_str());
            let resolution = edge
                .properties
                .get("resolution")
                .and_then(|value| value.as_str());
            match (target, resolution) {
                (Some(target), Some(resolution)) => println!(
                    "- {}: {} -> {} target=`{target}` resolution=`{resolution}`",
                    edge.kind, edge.source, edge.target
                ),
                _ => println!("- {}: {} -> {}", edge.kind, edge.source, edge.target),
            }
        }
    }
}

fn write_json(path: &PathBuf, value: &impl serde::Serialize) -> anyhow::Result<()> {
    let content = serde_json::to_string_pretty(value)?;
    write_text(path, &(content + "\n"))
}

fn write_text(path: &PathBuf, content: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}
