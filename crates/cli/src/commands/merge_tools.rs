use clap::Subcommand;
use rusty_idd_merge_tools::{package, render_markdown, verify_workspace};
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum MergeToolsCommand {
    /// Print the canonical Rusty IDD merge-goal package.
    Show,
    /// Print only the legacy surface disposition table.
    Legacy,
    /// Verify merge-tool safety gates that replaced the retired bridge drift scripts.
    Verify {
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
    },
}

pub fn run(command: MergeToolsCommand) -> i32 {
    let package = package();
    match command {
        MergeToolsCommand::Show => {
            print!("{}", render_markdown(&package));
        }
        MergeToolsCommand::Legacy => {
            println!("Legacy surface disposition");
            for surface in package.legacy_surfaces {
                println!(
                    "- {}: {} -> {}",
                    surface.path, surface.disposition, surface.replacement
                );
            }
        }
        MergeToolsCommand::Verify { workspace } => match verify_workspace(&workspace) {
            Ok(report) if report.is_clean() => {
                println!(
                    "Merge tools verification passed: {} crate manifests, {} src trees checked.",
                    report.checked_crates, report.checked_src_trees
                );
            }
            Ok(report) => {
                eprintln!("Merge tools verification failed:");
                for finding in report.findings {
                    eprintln!("- {finding}");
                }
                return 1;
            }
            Err(error) => {
                eprintln!("Merge tools verification failed: {error}");
                return 1;
            }
        },
    }
    0
}
