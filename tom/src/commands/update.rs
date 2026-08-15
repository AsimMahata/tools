use colored::*;
use std::path::Path;

use crate::git::GitStatus;
use crate::installer::build_tool;
use crate::tool::{discover_tools, find_tool, Tool};

pub fn execute(target_tool: Option<&str>, all: bool, tools_dir: &Path) {
    if all {
        println!("{}", "Updating all tools...".bold().cyan());
        println!("{}", "=====================".dimmed());

        let tools = discover_tools(tools_dir);
        for tool in &tools {
            update_single_tool(tool);
            println!();
        }
        return;
    }

    let tool_name = match target_tool {
        Some(t) => t,
        None => {
            eprintln!("{} Please specify a tool name to update or use --all.", "Error:".red().bold());
            return;
        }
    };

    let tool = match find_tool(tools_dir, tool_name) {
        Some(t) => t,
        None => {
            eprintln!(
                "{} Tool '{}' not found in {}",
                "Error:".red().bold(),
                tool_name,
                tools_dir.display()
            );
            return;
        }
    };

    update_single_tool(&tool);
}

fn update_single_tool(tool: &Tool) {
    println!("Updating {}...", tool.name.cyan().bold());

    if !tool.git.is_repo {
        eprintln!("  {} Not a Git repository.", "✗".red());
        return;
    }

    match GitStatus::pull_update(&tool.path, &tool.name) {
        Ok(msg) => {
            println!("  {} {}", "✓".green().bold(), msg.green());

            // Rebuild if needed
            if msg.contains("Updating") || msg.contains("Fast-forward") {
                println!("  {} Rebuilding {}...", "→".cyan(), tool.name);
                match build_tool(&tool.path, None, None, None) {
                    Ok(Some(b_msg)) => println!("  {} {}", "✓".green(), b_msg),
                    Ok(None) => {}
                    Err(err) => eprintln!("  {} Build error: {}", "⚠".yellow(), err),
                }
            }
        }
        Err(err) => {
            eprintln!("  {} {}", "✗".red().bold(), err);
        }
    }
}
