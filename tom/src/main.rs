use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;
mod config;
mod git;
mod installer;
mod metadata;
mod registry;
mod tool;

use config::Config;

#[derive(Parser, Debug)]
#[command(
    name = "tom",
    version,
    about = "TOM — Personal Tool & Package Manager",
    long_about = "TOM (Tool Manager) discovers, installs, inspects, and manages your ecosystem of independent tools and Git repositories."
)]
struct Cli {
    /// Override the root tools directory
    #[arg(short = 'd', long = "dir", global = true)]
    dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all discovered tools in the tools directory
    #[command(alias = "ls")]
    List,

    /// Show detailed information and Git status for a specific tool
    Info {
        /// Name of the tool to inspect
        tool: String,
    },

    /// Show Git status across all tools or for a specific tool
    #[command(alias = "st")]
    Status {
        /// Specific tool name (optional; shows all tools if omitted)
        tool: Option<String>,
    },

    /// Install a tool from registry or URL into the tools directory
    #[command(alias = "add")]
    Install {
        /// Name of the tool or Git URL to install
        tool: Option<String>,

        /// Install all tools defined in the registry
        #[arg(short = 'a', long = "all")]
        all: bool,
    },

    /// Uninstall / remove an installed tool
    #[command(alias = "rm")]
    Uninstall {
        /// Name of the tool to uninstall
        tool: String,

        /// Force removal even if there are uncommitted or unpushed changes
        #[arg(short, long)]
        force: bool,
    },

    /// Pull latest updates and rebuild a tool safely
    #[command(alias = "up")]
    Update {
        /// Name of the tool to update (or use --all)
        tool: Option<String>,

        /// Update all installed tools
        #[arg(short = 'a', long = "all")]
        all: bool,
    },

    /// Open a tool's directory in your editor or file explorer
    Open {
        /// Name of the tool to open
        tool: String,

        /// Custom editor command override (e.g. code, nvim, explorer)
        #[arg(short, long)]
        editor: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let (config, _) = Config::load_and_persist();
    let tools_dir = config.resolve_tools_directory(cli.dir.as_deref());

    match cli.command {
        Some(Commands::List) | None => {
            commands::list::execute(&tools_dir);
        }
        Some(Commands::Info { tool }) => {
            commands::info::execute(&tool, &tools_dir);
        }
        Some(Commands::Status { tool }) => {
            commands::status::execute(tool.as_deref(), &tools_dir);
        }
        Some(Commands::Install { tool, all }) => {
            commands::install::execute(tool.as_deref(), all, &tools_dir);
        }
        Some(Commands::Uninstall { tool, force }) => {
            commands::uninstall::execute(&tool, force, &tools_dir);
        }
        Some(Commands::Update { tool, all }) => {
            commands::update::execute(tool.as_deref(), all, &tools_dir);
        }
        Some(Commands::Open { tool, editor }) => {
            let editor_choice = editor.as_deref().or(config.editor.as_deref());
            commands::open::execute(&tool, &tools_dir, editor_choice);
        }
    }
}
