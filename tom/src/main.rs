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

#[cfg(windows)]
extern "system" fn console_ctrl_handler(ctrl_type: u32) -> i32 {
    // 0 = CTRL_C_EVENT, 1 = CTRL_BREAK_EVENT
    if ctrl_type == 0 || ctrl_type == 1 {
        std::process::exit(130);
    }
    0
}

#[cfg(windows)]
fn init_ctrl_handler() {
    type HandlerRoutine = Option<unsafe extern "system" fn(u32) -> i32>;
    extern "system" {
        fn SetConsoleCtrlHandler(handler: HandlerRoutine, add: i32) -> i32;
    }
    unsafe {
        SetConsoleCtrlHandler(Some(console_ctrl_handler), 1);
    }
}

#[cfg(not(windows))]
fn init_ctrl_handler() {}

#[derive(Parser, Debug)]
#[command(
    name = "tom",
    version,
    about = "TOM — Personal Tool & Package Manager",
    long_about = "TOM (Tool Manager) discovers, fetches, installs, inspects, and manages your ecosystem of independent tools and Git repositories."
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

    /// Show detailed information, steps, and Git status for a specific tool
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

    /// Fetch / clone a tool's Git repository without building it
    #[command(alias = "get", alias = "clone")]
    Fetch {
        /// Name of the tool or Git URL to fetch
        tool: Option<String>,

        /// Fetch all tools defined in the registry
        #[arg(short = 'a', long = "all")]
        all: bool,
    },

    /// Run installation/build pipeline for a tool
    #[command(alias = "build", alias = "add")]
    Install {
        /// Name of the tool to install/build
        tool: Option<String>,

        /// Install all tools defined in the registry
        #[arg(short = 'a', long = "all")]
        all: bool,

        /// Automatically confirm prompts (e.g. auto-fetch)
        #[arg(short = 'y', long = "yes")]
        yes: bool,
    },

    /// Run uninstallation command for a tool (keeps repository files)
    Uninstall {
        /// Name of the tool to uninstall
        tool: String,
    },

    /// Remove a tool's repository and code files (preserves README.md)
    #[command(alias = "purge", alias = "remove", alias = "rm")]
    Unfetch {
        /// Name of the tool to unfetch / purge
        tool: String,

        /// Force removal even if there are uncommitted or unpushed changes
        #[arg(short, long)]
        force: bool,
    },

    /// Pull latest Git updates for tools
    #[command(alias = "up", alias = "pull")]
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
    init_ctrl_handler();

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
        Some(Commands::Fetch { tool, all }) => {
            commands::fetch::execute(tool.as_deref(), all, &tools_dir);
        }
        Some(Commands::Install { tool, all, yes }) => {
            commands::install::execute(tool.as_deref(), all, yes, &tools_dir);
        }
        Some(Commands::Uninstall { tool }) => {
            commands::uninstall::execute(&tool, &tools_dir);
        }
        Some(Commands::Unfetch { tool, force }) => {
            commands::unfetch::execute(&tool, force, &tools_dir);
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
